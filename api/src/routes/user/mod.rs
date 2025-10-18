pub mod balance;
pub mod login;
pub mod register;
pub mod logout;

pub use balance::*;
pub use login::*;
pub use register::*;
pub use logout::*;

pub fn user_config(cfg: &mut actix_web::web::ServiceConfig) {
    balance::config(cfg);
    login::config(cfg);
    register::config(cfg);
    logout::config(cfg);
}

pub const MIN_CONST: u64 = 2;
pub const MAX_CONST: u64 = 50;