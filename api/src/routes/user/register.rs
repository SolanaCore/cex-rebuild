use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use validator::Validate;
use crate::routes::{MAX_CONST, MIN_CONST};
#[derive(Deserialize, Validate)]
pub(crate) struct RegisterRequest {
    #[validate(length(min = "MIN_CONST", max = "MAX_CONST"))]
    username: String,
    #[validate(length(min = "MIN_CONST", max = "MAX_CONST"))]
    password: String,
    #[validate(email)]
    email: String,
}


//todo: add database connection in the api 
#[post("/signup")]
pub async fn register(
    form: web::Json<RegisterRequest>,
) -> impl Responder {
     let data = form.into_inner();

    if let Err(errors) = data.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    // proceed with valid data
    HttpResponse::Ok().body("Registration successful")
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
}