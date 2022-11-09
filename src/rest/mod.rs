use actix_web::web;

mod app_users;
mod students;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/login").route(web::post().to(crate::security::login::user_login)));
    cfg.service(
        web::resource("/appUsers")
            .route(web::get().to(app_users::app_users_index))
            .route(web::post().to(app_users::app_users_create)),
    );

    cfg.service(
        web::resource("/students")
            .route(web::get().to(students::students_index))
            .route(web::post().to(students::students_create)),
    );
    cfg.service(
        web::resource("/students/{id}").route(web::get().to(students::students_by_id)), //.route(web::post().to(students::students_create)),
    );
}
