use actix_web::{App, HttpResponse};

use api::APIMiddleware;
use State;

pub fn router(state: State) -> App<State> {
    App::with_state(state)
        .middleware(APIMiddleware)
        .resource("/", |r| r.f(|_| HttpResponse::Ok().json("hello there")))
        .resource("/scoreboard", |r| r.with(self::base::scoreboard))
        .scope("/user", |scope| {
            use user::LoginRequired;
            scope
                .middleware(APIMiddleware)
                .resource("/login", |r| r.post().with(self::user::login))
                .resource("/register", |r| r.post().with(self::user::register))
            // .nested("/settings", |scope| {
            //     scope.middleware(LoginRequired).resource("/", |r| {
            //         r.get().with(self::user::get_settings);
            //         r.post().with(self::user::post_settings);
            //     })
            // })
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
                HttpResponse::Ok().finish()
            }).unwrap_or_else(|err| {
                error!("Error while fetching scoreboard: {}", err);
                HttpResponse::InternalServerError().finish()
            })
    }
}

mod user {
    use actix_web::{HttpRequest, HttpResponse, Json};
    use user::auth::{login_user, register_user, LoginForm, RegisterForm, UserError};
    use {DbConn, State};

    pub fn login((req, form, db): (HttpRequest<State>, Json<LoginForm>, DbConn)) -> HttpResponse {
        let state = req.state();

        login_user(db, state.get_secret_key(), form.into_inner())
            .map(|token| HttpResponse::Ok().json(token))
            .unwrap_or_else(|err| match err {
                UserError::AlreadyRegistered => HttpResponse::BadRequest().finish(),
                UserError::BadUsernameOrPassword => HttpResponse::Unauthorized().finish(),
                UserError::ServerError(err) => {
                    error!("Error logging in: {}", err);
                    HttpResponse::InternalServerError().finish()
                }
            })
    }

    pub fn register((form, db): (Json<RegisterForm>, DbConn)) -> HttpResponse {
        let form = form.into_inner();
        register_user(db, form)
            .map(|_| HttpResponse::Ok().finish())
            .unwrap_or_else(|err| {
                error!("Error registering: {}", err);
                HttpResponse::InternalServerError().finish()
            })
    }
}
