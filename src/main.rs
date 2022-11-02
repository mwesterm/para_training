use actix_web::{cookie::Key, web, web::Data, App, HttpServer};

use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{storage::RedisActorSessionStore, Session, SessionMiddleware};

use cfg_block::*;
use env_logger::Env;
use rustls::{Certificate, PrivateKey, ServerConfig};
#[cfg(feature = "TSL")]
use rustls_pemfile::{certs, pkcs8_private_keys};

use std::{env, fs::File, io::BufReader};

use log::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // get env vars
    dotenv::dotenv().ok();

    let db_url =
        env::var("DATABASE_URL").unwrap_or_else(|_| panic!("{}", fl!("Database_url_missing")));
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let tls_key_file = env::var("KEYFILE").expect("KEYFILE is not set in .env file");
    let tls_cert_file = env::var("CERTFILE").expect("CERTFILE is not set in .env file");
    let server_url = format!("{}:{}", host, port);

    let log_level = env::var("RUST_LOG").expect("RUST_LOG is not set in .env file");
    let env = Env::default().default_filter_or(log_level);
    env_logger::init_from_env(env);
    info!("Logging started");
    // establish connection to database
    let server_config = match load_certs(&tls_cert_file, &tls_key_file) {
        Err(e) => {
            error!("error open cert or  key file: {}", e);
            panic!("")
        }
        Ok(server_config) => server_config,
    };
    info!("Key and Cert loaded");

    let secret_key = Key::generate();
    let redis_connection_string = "127.0.0.1:6379";

    // create server and try to serve over socket if possible
    let server = HttpServer::new(move || {
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
    /*     if cfg!(RUN_TLS) {
        let server = server.bind_rustls(&server_url, server_config)?;
    } else {
        */
    let server = server.bind(&server_url)?;

    server.run().await?;
    Ok(())
}
