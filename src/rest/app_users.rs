use actix_identity::Identity;
use actix_web::{HttpResponse, Responder};
use diesel::expression::is_aggregate::No;
use log::info;

use crate::db::app_users::*;

pub async fn app_users_index(user: Option<Identity>) -> impl Responder {
    match user {
        None => info!("get AppUsers, no user"),
        Some(_) => info!("get AppUsers  user: {:?}", user.unwrap().id()),
    }

    let query_result = AppUser::get_all_app_users().await;
    match query_result {
        Err(_) => HttpResponse::Ok().body("Error"),
        Ok(app_users) => HttpResponse::Ok().json(app_users),
    }
}
