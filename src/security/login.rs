use actix_web::{web, HttpResponse, Responder};
use log::debug;

use super::LoginUserPayload;

pub async fn user_login(login_user: web::Json<LoginUserPayload>) -> impl Responder {
    let login_user = login_user.into_inner();
    debug!("Try login user : {}", login_user);
    HttpResponse::Ok().body("Ok")
}
