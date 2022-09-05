#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;
pub mod routes;
mod errors;
mod vars;

use actix_web::{
    HttpServer,
    App,
    middleware,
};
use actix_files::Files;
use crate::routes::routes;

#[macro_use]
mod utils;
#[macro_use]
mod views;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_redis::RedisSession;
    use actix_cors::Cors;

    HttpServer::new(|| {
        let _files = Files::new("/static", "static/").show_files_listing();
        let _files2 = Files::new("/media", "media/").show_files_listing();
        let cors = Cors::default()
            .allowed_origin("http://вебсервисы.рф")
            .allowed_origin("https://вебсервисы.рф")
            //.allowed_origin("194.58.90.123:8084")
            .allowed_methods(vec!["GET", "POST"])
            .max_age(3600);
        App::new()
            .wrap(cors)
            .wrap(middleware::Compress::default())
            .wrap(RedisSession::new("127.0.0.1:6379", &[0; 32]))
            .service(_files)
            .service(_files2)
            .configure(routes)
    })

    .bind("194.58.90.123:8084")?
    //.bind("194.58.90.123:8082")?
    .run()
    .await
}
