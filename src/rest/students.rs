use actix_identity::Identity;
use actix_web::{web, HttpResponse, Responder};
use log::{debug, info};

use crate::db::models::Student;

pub async fn students_index(user: Option<Identity>) -> impl Responder {
    match Student::get_all_students().await {
        Err(_) => HttpResponse::Ok().body("Error"),
        Ok(app_users) => HttpResponse::Ok().json(app_users),
    }
}

pub async fn students_create(
    //user: Option<Identity>,
    imut_new_app_user: web::Json<Student>,
) -> impl Responder {
    info!("Post students");
    let mut new_student = imut_new_app_user.into_inner();
    debug!("New User  : {:?}", new_student);
    match Student::add_student(&mut new_student).await {
        Err(_) => HttpResponse::Ok().body("Error"),
        Ok(app_users) => HttpResponse::Ok().json(app_users),
    }
}

pub async fn students_by_id(user: Option<Identity>, id: web::Path<uuid::Uuid>) -> impl Responder {
    match Student::get_students_by_id(id.into_inner()).await {
        Err(_) => HttpResponse::NotFound().body("not found"),
        Ok(student) => HttpResponse::Ok().json(student),
    }
}
