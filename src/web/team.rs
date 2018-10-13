use actix_web::{HttpResponse, Json};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTeamForm {
    name: String,
}

pub fn create(form: Json<CreateTeamForm>) -> HttpResponse {
    HttpResponse::Ok().json("lol")
}
