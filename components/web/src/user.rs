use actix_web::{
    self,
    middleware::{Middleware, Started},
    App, HttpRequest, HttpResponse, Json,
};
use bcrypt;
use chrono::{self, serde::ts_milliseconds, DateTime, Utc};
use diesel::{self, prelude::*};
use jsonwebtoken::{self, Header, Validation};
use core::models::{NewUser, User};

use super::{errors::WebError, APIMiddleware, DbConn, State};

pub fn app(state: State) -> App<State> {
    App::with_state(state)
        .middleware(APIMiddleware)
        .prefix("/user")
        .resource("/login", |r| r.post().with(login))
        .resource("/register", |r| r.post().with(register))
}

pub struct LoginRequired;

impl Middleware<State> for LoginRequired {
    fn start(&self, req: &HttpRequest<State>) -> actix_web::Result<Started> {
        let state = req.state();

        let headers = req.headers();
        let token = match headers.get("Authorization").and_then(|t| t.to_str().ok()) {
            Some(token) => if token.starts_with("Token ") {
                token.trim_left_matches("Token ")
            } else {
                token
            },
            None => {
                return Ok(Started::Response(
                    HttpResponse::Forbidden().json("access denied 1"),
                ))
            }
        };

        // TODO: don't unwrap here
        let validation = Validation {
            leeway: 60,
            ..Default::default()
        };
        let decoded =
            match jsonwebtoken::decode::<LoginClaim>(token, &state.get_secret_key(), &validation) {
                Ok(claims) => claims,
                err => {
                    error!("Error decoding JWT from user: {:?}", err);
                    return Ok(Started::Response(
                        HttpResponse::Forbidden().json("access denied 1"),
                    ));
                }
            };

        let mut ext = req.extensions_mut();
        ext.insert(decoded.claims);

        Ok(Started::Done)
    }
}

#[derive(Deserialize)]
struct LoginForm {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginClaim {
    #[serde(with = "ts_milliseconds")]
    exp: DateTime<Utc>,
    pub id: i32,
    username: String,
    admin: bool,
}

fn login((req, form, db): (HttpRequest<State>, Json<LoginForm>, DbConn)) -> HttpResponse {
    use core::schema::users::dsl::*;
    let state = req.state();
    let form = form.into_inner();
    users
        .filter(email.eq(&form.email))
        .first::<User>(&*db)
        .map_err(|err| {
            error!("Diesel error on user/login: {}", err);
            HttpResponse::Unauthorized().json("Check your credentials.".to_owned())
        }).and_then(|user| {
            bcrypt::verify(&form.password, &user.password)
                .map(|_| user)
                .map_err(|_| {
                    HttpResponse::Unauthorized().json("Check your credentials.".to_owned())
                })
        }).map(|user| {
            let claim = LoginClaim {
                exp: Utc::now() + chrono::Duration::weeks(6),
                id: user.id,
                username: user.name.clone(),
                admin: user.admin,
            };

            // generate jwt
            // TODO don't unwrap() this
            let token =
                jsonwebtoken::encode(&Header::default(), &claim, state.get_secret_key().as_ref())
                    .unwrap();
            // let cookie = Cookie::build("user", token)
            //     .same_site(SameSite::Strict)
            //     .finish();

            HttpResponse::Ok()
                // .cookie(cookie)
                .json(token)
        }).unwrap_or_else(|err| err)
}

#[derive(Deserialize)]
struct RegisterForm {
    email: String,
    username: String,
    password: String,
}

fn register((form, db): (Json<RegisterForm>, DbConn)) -> HttpResponse {
    use core::schema::users;

    let new_user: NewUser = match form.into_inner().into() {
        Ok(user) => user,
        Err(err) => {
            error!("Failed to register: {:?}", err);
            return HttpResponse::BadRequest().json(err.to_string());
        }
    };

    diesel::insert_into(users::table)
        .values(new_user)
        .execute(&*db)
        .map(|_| HttpResponse::Ok().json("Registration successful."))
        .unwrap_or_else(|err| {
            error!("Diesel error on user/register: {}", err);
            HttpResponse::BadRequest().json("Failed to complete registration.")
        })
}

// TODO: this should be a ValidationError instead of WebError
impl Into<Result<NewUser, WebError>> for RegisterForm {
    fn into(self) -> Result<NewUser, WebError> {
        Ok(NewUser {
            email: self.email,
            name: self.username,
            password: bcrypt::hash(&self.password, bcrypt::DEFAULT_COST)?,
        })
    }
}
