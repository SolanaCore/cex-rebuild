//TODO!
use actix_web::{post, web, HttpResponse};
#[post("/logout")]
async fn logout() -> HttpResponse {
    // Implementation for logout
    HttpResponse::Ok().body("User logged out")

}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(logout);
}