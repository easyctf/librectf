use rocket::{form::Form, serde::json::Json};

#[get("/register", format = "json", data = "<form>")]
pub fn register(form: Json<RegisterForm<'_>>) -> Json<RegisterResponse> {
    Json(RegisterResponse {})
}

#[derive(Deserialize)]
pub struct RegisterForm<'f> {
    name: &'f str,
}

#[derive(Serialize)]
pub struct RegisterResponse {
}
