use actix_web::web;

mod app_users;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/appUsers")
            .route(web::get().to(app_users::app_users_index))
            .route(web::post().to(app_users::app_users_create)),
    );
}
