#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

use actix::Actor;
use actix_cors::Cors;
use dotenv::dotenv;
use env_logger;

pub mod schema;
pub mod models;
pub mod routes;
pub mod websocket;
mod errors;
mod vars;

use actix_web::{
    HttpServer,
    App,
    middleware::{
        Compress, 
        Logger, 
        NormalizePath,
    },
    web,
    http,
};
use actix_redis::RedisSession;
use actix_files::Files;
use crate::routes::routes;
use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

#[macro_use]
mod utils;
#[macro_use]
mod views;

use crate::utils::AppState;
use crate::views::not_found;

static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));
    let server = websocket::Server::new().start();

    HttpServer::new(move || {
        let _files = Files::new("/static", "static/").show_files_listing();
        let _files2 = Files::new("/media", "media/").show_files_listing();
        let messages = Arc::new(Mutex::new(vec![]));
        //let cors = Cors::default()
        //    .allowed_origin("194.58.90.123:8084")
        //    .allowed_origin("194.58.90.123:8082")
        //    .allowed_origin("127.0.0.1:6379")
        //    .allowed_methods(vec!["GET", "POST"])
        //    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        //    .allowed_header(http::header::CONTENT_TYPE)
        //    .max_age(3600);

        App::new() 
            .data(AppState {
                server_id: SERVER_COUNTER.fetch_add(1, Ordering::SeqCst),
                request_count: Cell::new(0),
                messages: messages.clone(),
            }) 
            .wrap(Logger::default())
            .wrap(Compress::default())
            .wrap(NormalizePath::trim())
            //.wrap(cors)
            .wrap(RedisSession::new("127.0.0.1:6379", &[0; 32]))
            .data(server.clone())
            .default_service(web::route().to(not_found))
            .service(_files)
            .service(_files2)

            .configure(routes)
    })

    .bind("194.58.90.123:8084")?       // порт для разработки
    //.bind("194.58.90.123:8082")?     // порт для автоматической доставки
    .workers(4)
    .run()
    .await
}
