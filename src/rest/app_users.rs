use actix_identity::Identity;
use actix_web::{web, App, HttpResponse, Responder};
use log::{debug, info};

use crate::db::{app_users::*, models::AppUser};

pub async fn app_users_index(user: Option<Identity>) -> impl Responder {
    match user {
        None => info!("GET AppUsers, no user,"),
        Some(_) => info!("GET AppUsers  user: {:?}", user.unwrap().id()),
    }

    let query_result = AppUser::get_all_app_users().await;
    match query_result {
        Err(_) => HttpResponse::Ok().body("Error"),
        Ok(app_users) => HttpResponse::Ok().json(app_users),
    }
}

pub async fn app_users_create(
    user: Option<Identity>,
    imut_new_app_user: web::Json<AppUser>,
) -> impl Responder {
    info!("Post AppUser");
    debug!("New User  : {:?}", imut_new_app_user);
    let mut new_app_user = imut_new_app_user.into_inner();
    AppUser::add_app_user(&mut new_app_user).await;
    HttpResponse::Ok().body("Ok")
}
