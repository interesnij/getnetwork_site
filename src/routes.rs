use actix_web::web;

use crate::views::{
    work_progs,
    blog_progs,
    service_progs,
    store_progs,
    wiki_progs,
    tag_progs,
    serve_progs,
    //search_progs,
    pages,
    auth,
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    .configure(pages::pages_routes)
    .configure(blog_progs::blog_routes)
    .configure(service_progs::service_routes)
    .configure(store_progs::store_routes)
    .configure(wiki_progs::wiki_routes)
    .configure(work_progs::work_routes)
    //.configure(search_progs::search_routes)
    .configure(serve_progs::serve_routes)
    .configure(tag_progs::tag_routes)
    .configure(auth::auth_routes)
    ;
}
