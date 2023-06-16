use std::collections::HashMap;

use actix_web::web;
use serde::{Serialize, Deserialize};

pub mod auth;
pub mod user;

#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultResponse {
    data: String,
    method: String,
    headers: HashMap<String, String>
}


pub fn config_api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(auth::config_auth)
            .configure(user::config_user)
    );
}
