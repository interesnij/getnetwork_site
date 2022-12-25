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
    },
    web,
    http,
};
//use actix_redis::RedisSession;
use actix_files::Files;
use crate::routes::routes;
use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;

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
    let secret_key = Key::generate();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));
    let server = websocket::Server::new().start();

    HttpServer::new(move || {
        let _files = Files::new("/static", "static/").show_files_listing();
        let _files2 = Files::new("/media", "media/").show_files_listing();
        let messages = Arc::new(Mutex::new(vec![]));

        App::new()  
            .wrap(Logger::default())
            .wrap(Compress::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            //.wrap(RedisSession::new("127.0.0.1:6379", &[0; 32]))
            .app_data(AppState {
                server_id: SERVER_COUNTER.fetch_add(1, Ordering::SeqCst),
                request_count: Cell::new(0),
                messages: messages.clone(),
            })
            .app_data(server.clone())
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
