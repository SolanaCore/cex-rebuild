use actix_web::{
    cookie::Cookie,
    http::StatusCode,
    post, web, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use std::env;
use validator::Validate;
use crate::routes::{MAX_CONST, MIN_CONST};

#[derive(Deserialize, Validate)]
pub(crate) struct LoginRequest {
    #[validate(length(min = "MIN_CONST", max = "MAX_CONST"))]
    username: String,
    #[validate(length(min = "MIN_CONST", max = "MAX_CONST"))]
    password: String,
    #[validate(email)]
    email: String,
}


#[post("/login")]
async fn login(form: web::Json<LoginRequest>) -> impl Responder {
    let signin_data = form.into_inner();

    //return that only 
    if signin_data.username == "admin" && signin_data.password == "password" {
        let mut response = HttpResponse::Ok();

        let cookie = Cookie::build("session_id", "some_session_token")
            .path("/")
            .http_only(true)
            .finish();  
        response.cookie(cookie);  
        response.body("Login successful")
    } else {
        HttpResponse::Unauthorized().body("Invalid username or password")   
    }
        
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
}