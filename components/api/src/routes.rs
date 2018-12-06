use actix_web::{App, HttpResponse};

use api::APIMiddleware;
use State;

pub fn router(state: State) -> App<State> {
    use team::{middleware::Boolean::*, TeamRequired};
    use user::LoginRequired;
    App::with_state(state)
        .middleware(APIMiddleware)
        .resource("/", |r| r.f(|_| HttpResponse::Ok().json("hello there")))
        .resource("/scoreboard", |r| r.with(self::base::scoreboard))
        .scope("/chal", |scope| {
            scope
                .middleware(TeamRequired(False))
                .resource("/list", |r| r.get().with(self::chal::list))
                .resource("/submit", |r| r.post().with(self::chal::submit))
        }).scope("/team", |scope| {
            scope
                .middleware(LoginRequired)
                .resource("/create", |r| r.post().with(self::team::create))
                .resource("/me", |r| r.get().with(self::team::me))
                .resource("/accept", |r| r.post().with(self::team::accept))
                .resource("/invites", |r| r.get().with(self::team::invites))
                .nested("/manage", |scope| {
                    scope
                        .middleware(TeamRequired(True))
                        .resource("/invite", |r| r.post().with(self::team::manage::invite))
                        .resource("/kick", |r| r.post().with(self::team::manage::kick))
                })
        }).scope("/user", |scope| {
            scope
                .resource("/login", |r| r.post().with(self::user::login))
                .resource("/register", |r| r.post().with(self::user::register))
        })
}

mod base {
    use actix_web::{HttpResponse, Query};
    use scoreboard::{get_scoreboard, ScoreboardOptions};
    use DbConn;

    pub fn scoreboard((query, db): (Query<ScoreboardOptions>, DbConn)) -> HttpResponse {
        get_scoreboard(db, &query.into_inner())
            .map(|entries| {
                info!("Scoreboard: {:?}", entries);
                HttpResponse::Ok().json(entries)
            }).unwrap_or_else(|err| {
                error!("Error while fetching scoreboard: {}", err);
                HttpResponse::InternalServerError().finish()
            })
    }
}

mod chal {
    use actix_web::{HttpRequest, HttpResponse, Json};
    use chal::{list_all, submit_flag, Submission, SubmitForm};
    use core::models::{Team, User};
    use {DbConn, State};

    pub fn list(db: DbConn) -> HttpResponse {
        list_all(db)
            .map(|chals| {
                let chals = chals
                    .iter()
                    .map(|chal| {
                        json!({
                            "title": chal.title,
                            "value": chal.value,
                            "description": chal.description,
                        })
                    }).collect::<Vec<_>>();
                HttpResponse::Ok().json(chals)
            }).unwrap_or_else(|err| {
                error!("Error while listing chals: {}", err);
                HttpResponse::InternalServerError().finish()
            })
    }

    pub fn submit((req, form, db): (HttpRequest<State>, Json<SubmitForm>, DbConn)) -> HttpResponse {
        let form = form.into_inner();
        let ext = req.extensions();

        let user = ext.get::<User>().unwrap();
        let team = ext.get::<Team>().unwrap();

        let submission = Submission {
            user_id: user.id,
            team_id: team.id,
            form,
        };
        submit_flag(db, submission)
            .map(|result| HttpResponse::Ok().json(result))
            .unwrap_or_else(|err| {
                error!("Error during submission: {}", err);
                HttpResponse::InternalServerError().finish()
            })
    }
}

mod team {
    use actix_web::{HttpRequest, HttpResponse, Json};
    use core::models::User;
    use team::{accept_invite, create_team, get_invites, my_profile, AcceptForm, CreateTeamForm};
    use user::auth::LoginClaims;
    use {DbConn, State};

    pub fn create(
        (req, form, db): (HttpRequest<State>, Json<CreateTeamForm>, DbConn),
    ) -> HttpResponse {
        let ext = req.extensions();
        let claims = ext.get::<LoginClaims>().unwrap();
        let form = form.into_inner();
        create_team(db, claims.id, form)
            .map(|_| HttpResponse::Ok().finish())
            .unwrap_or_else(|err| {
                error!("Error during team creation: {}", err);
                HttpResponse::InternalServerError().finish()
            })
    }

    pub fn me((req, db): (HttpRequest<State>, DbConn)) -> HttpResponse {
        let ext = req.extensions();
        let claims = ext.get::<LoginClaims>().unwrap();

        my_profile(db, claims.id)
            .map(|profile| HttpResponse::Ok().json(profile))
            .unwrap_or_else(|err| {
                error!("Error fetching profile: {}", err);
                HttpResponse::InternalServerError().finish()
            })
    }

    pub fn accept((req, form, db): (HttpRequest<State>, Json<AcceptForm>, DbConn)) -> HttpResponse {
        // TODO: finish this
        let form = form.into_inner();
        let ext = req.extensions();
        let user = ext.get::<User>().unwrap();
        let team_id = form.team_id;

        accept_invite(db, user.id, form)
            .map(|_| HttpResponse::Ok().finish())
            .unwrap_or_else(|err| {
                error!(
                    "Error accepting invite: uid={} tid={} err={}",
                    user.id, team_id, err
                );
                HttpResponse::InternalServerError().finish()
            })
    }

    pub fn invites((req, db): (HttpRequest<State>, DbConn)) -> HttpResponse {
        let ext = req.extensions();
        let user = ext.get::<User>().unwrap();

        get_invites(db, user.id)
            .map(|invites| HttpResponse::Ok().json(json!(invites)))
            .unwrap_or_else(|err| {
                error!("Error fetching invites: {}", err);
                HttpResponse::InternalServerError().finish()
            })
    }

    pub mod manage {
        use actix_web::{HttpRequest, HttpResponse, Json};
        use core::models::Team;
        use team::manage::{invite_user, InviteUserForm};
        use {DbConn, State};

        pub fn invite(
            (req, form, db): (HttpRequest<State>, Json<InviteUserForm>, DbConn),
        ) -> HttpResponse {
            let form = form.into_inner();
            let ext = req.extensions();

            let team = ext.get::<Team>().unwrap();
            invite_user(db, team.id, form)
                .map(|_| HttpResponse::Ok().finish())
                .unwrap_or_else(|err| {
                    error!("Error inviting user: {}", err);
                    HttpResponse::InternalServerError().finish()
                })
        }

        pub fn kick(_db: DbConn) -> HttpResponse {
            // TODO: finish this
            HttpResponse::Ok().finish()
        }
    }
}

mod user {
    use actix_web::{HttpRequest, HttpResponse, Json};
    use user::auth::{login_user, register_user, LoginForm, RegisterForm, UserError};
    use {DbConn, State};

    pub fn login((req, form, db): (HttpRequest<State>, Json<LoginForm>, DbConn)) -> HttpResponse {
        let state = req.state();
        let form = form.into_inner();

        info!("Login request: user={:?}", form.user);
        let cfg = state.get_web_config().unwrap();
        login_user(db, cfg.secret_key.as_ref(), form)
            .map(|(user, token)| {
                info!(
                    "Successfully logged in: id={:?}, email={:?}",
                    user.id, user.email
                );
                HttpResponse::Ok().json(token)
            }).unwrap_or_else(|err| match err {
                UserError::AlreadyRegistered => HttpResponse::BadRequest().finish(),
                UserError::BadUsernameOrPassword => HttpResponse::Unauthorized().finish(),
                UserError::ServerError(err) => {
                    error!("Error logging in: {}", err);
                    HttpResponse::InternalServerError().finish()
                }
            })
    }

    pub fn register(
        (req, form, db): (HttpRequest<State>, Json<RegisterForm>, DbConn),
    ) -> HttpResponse {
        let state = req.state();
        let form = form.into_inner();
        info!(
            "Register request: username={:?}, email={:?}",
            form.username, form.email
        );
        let cfg = state.get_web_config().unwrap();
        register_user(db, cfg.secret_key.as_ref(), form)
            .map(|(user, token)| {
                info!(
                    "Successfully registered: id={:?}, username={:?}",
                    user.id, user.name
                );
                HttpResponse::Ok().json(token)
            }).unwrap_or_else(|err| {
                error!("Error registering: {}", err);
                HttpResponse::InternalServerError().finish()
            })
    }
}
