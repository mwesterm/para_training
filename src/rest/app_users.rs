use actix_identity::Identity;
use actix_web::{web, HttpResponse, Responder};
use log::{debug, error, info};

use crate::db::{app_users, models::AppUser};

pub async fn app_users_index(user: Option<Identity>) -> impl Responder {
    match user {
        None => info!("GET AppUsers, no user,"),
        Some(_) => info!("GET AppUsers  user: {:?}", user.unwrap().id()),
    }

    let query_result = AppUser::get_all_app_users().await;
    match query_result {
        Err(_) => HttpResponse::InternalServerError().body("error"),
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
    match AppUser::add_app_user(&mut new_app_user).await {
        Err(e) => {
            error!("error:{:?}", e);
            HttpResponse::InternalServerError().body("error")
        }
        Ok(()) => HttpResponse::Ok().body("Ok"),
    }
}
