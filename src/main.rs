#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

use actix::Actor;
use actix_cors::Cors;
use actix_rt;
use dotenv::dotenv;
use env_logger;
use std::env;

pub mod schema;
pub mod models;
pub mod routes;
pub mod websocket;
mod errors;
mod vars;

use actix_web::{
    HttpServer,
    App,
    middleware::{Compress, Logger},
    http,
};
use actix_redis::RedisSession;
use actix_files::Files;
use crate::routes::routes;

#[macro_use]
mod utils;
#[macro_use]
mod views;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));
    let server = websocket::Server::new().start();

    HttpServer::new(|| {
        let _files = Files::new("/static", "static/").show_files_listing();
        let _files2 = Files::new("/media", "media/").show_files_listing();
        let cors = Cors::default()
            .allowed_origin(&env::var("CLIENT_HOST").unwrap())
            .allow_any_method()
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(Compress::default())
            .wrap(RedisSession::new("127.0.0.1:6379", &[0; 32]))
            .data(server.clone())
            .service(_files)
            .service(_files2)
            .configure(routes)
    })

    .bind("194.58.90.123:8084")?       // порт для разработки
    //.bind("194.58.90.123:8082")?     // порт для автоматической доставки
    .run()
    .await
}
