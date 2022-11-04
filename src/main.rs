use actix_web::{cookie::Key, web, App, HttpServer};

use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{storage::RedisActorSessionStore, Session, SessionMiddleware};

use dotenvy::*;
use env_logger::Env;
use std::env;

mod db;
mod error_handler;
mod security;
use db::connection::*;

use log::*;

use crate::security::tls::load_certs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // get env vars
    dotenv().ok();

    env::var("DATABASE_URL").unwrap_or_else(|_| panic!("{}", "Database_url_missing")); //nur zum checlen ob env ok ist
    let host = env::var("HOST").expect("HOST is not set in .env");
    let port = env::var("PORT").expect("PORT is not set in .env");
    let tls_key_file = env::var("KEYFILE").expect("KEYFILE is not set in .env file");
    let tls_cert_file = env::var("CERTFILE").expect("CERTFILE is not set in .env file");
    let server_url = format!("{}:{}", host, port);

    let log_level = env::var("RUST_LOG").expect("RUST_LOG is not set in .env file");
    let env = Env::default().default_filter_or(log_level);
    env_logger::init_from_env(env);
    info!("Logging started");
    // establish connection to database
    init_db_connectons();
    info!("DB Connections established");

    let server_config = match load_certs(&tls_cert_file, &tls_key_file) {
        Err(e) => {
            error!("error open cert or  key file: {}", e);
            panic!("")
        }
        Ok(server_config) => server_config,
    };
    info!("Key and Cert loaded");

    init_db_connectons();

    let secret_key = Key::generate();
    let redis_connection_string = "127.0.0.1:6379";

    // create server and try to serve over socket if possible
    let mut server = HttpServer::new(move || {
        App::new()
            //            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                RedisActorSessionStore::new(redis_connection_string),
                secret_key.clone(),
            ))
            .route("/", web::get().to(|| async { "Actix REST API" }))
        // .service(student_index)
        // .service(student_show)
        //.service(login)
        /*              .service(student_create)
        .service(student_update)
        .service(student_destroy)*/
    });
    //
    let mut listenfd = listenfd::ListenFd::from_env();

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => {
            info!("Restating Web- Server");
            server.listen(listener)?
        }
        None => {
            if cfg!(USE_TLS) {
                info!("Web-Server starting (SSL)");
                server.bind_rustls(&server_url, server_config)?
            } else {
                info!("Web-Server starting (Non-SSL)");
                server.bind(format!("{}:{}", host, port))?
            }
        }
    };
    server.run().await?;
    Ok(())
}
