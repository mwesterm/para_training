use actix_web::web;

mod app_users;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/appUsers").route(web::get().to(app_users::appUsers_index)));
}
