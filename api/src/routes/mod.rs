pub mod user;
pub use user::*;

pub fn routes_config(cfg: &mut actix_web::web::ServiceConfig) {
    user::user_config(cfg);
}
