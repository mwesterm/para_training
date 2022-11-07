use actix_identity::Identity;
use actix_web::{web, App, HttpResponse, Responder};
use log::info;

use crate::{db::app_users::*, error_handler::CustomError};

pub async fn appUsers_index(user: Option<Identity>) -> impl Responder {
    info!("get AppUsers  user: {:?}", user.unwrap().id());

    let query_result = AppUser::get_all_app_users().await;
    match query_result {
        CustomError => HttpResponse::Ok().body("Error"),
        Ok(appUsers) => HttpResponse::Ok().json(appUsers),
    }
}
