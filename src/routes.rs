use actix_web::web;

use crate::handlers::users::{delete_user, get_user, list_users};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(list_users)
        .service(get_user)
        .service(delete_user);
}
