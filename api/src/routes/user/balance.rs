//TODO!
use actix_web::{post, web, HttpResponse};
#[post("/balance")]
async fn balance() -> HttpResponse {
    // Implementation for logout
    return HttpResponse::Ok().body("User balance");
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(balance);
}