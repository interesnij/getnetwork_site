use actix_web::web;

use crate::views::{
    work_progs,
    blog_progs,
    service_progs,
    store_progs,
    wiki_progs,
    tag_progs,
};
use crate::views::{process_signup,signup,about,index};


pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    // pages urls
    .route("/", web::get().to(index))
    .route("/about/", web::get().to(about))
    .route("/signup", web::get().to(signup))
    .route("/signup", web::post().to(process_signup))

    // tags urls
    .route("/tags/", web::get().to(tag_progs::tags_page))
    .route("/tag/{id}/", web::get().to(tag_progs::tag_page))
    .service(web::resource("/create_tag/")
        .route(web::get().to(tag_progs::create_tag_page))
        .route(web::post().to(tag_progs::create_tag))
    )
    .service(web::resource("/edit_tag/{id}/")
        .route(web::get().to(tag_progs::edit_tag_page))
        .route(web::post().to(tag_progs::edit_tag))
    )
    .route("/delete_tag/{id}/", web::get().to(tag_progs::delete_tag))

    // portfolio urls
    .route("/work_categories/", web::get().to(work_progs::work_categories_page))
    .service(web::resource("/create_work_categories/")
        .route(web::get().to(work_progs::create_work_categories_page))
        .route(web::post().to(work_progs::create_work_categories))
    )
    .service(web::resource("/edit_work_category/{id}/")
        .route(web::get().to(work_progs::edit_work_category_page))
        .route(web::post().to(work_progs::edit_work_category))
    )
    .service(web::resource("/create_work/")
        .route(web::get().to(work_progs::create_work_page))
        .route(web::post().to(work_progs::create_work))
    )
    .service(web::resource("/edit_work/{id}/")
        .route(web::get().to(work_progs::edit_work_page))
        .route(web::post().to(work_progs::edit_work))
    )
    .route("/delete_work/{id}/", web::get().to(work_progs::delete_work))
    .route("/delete_work_category/{id}/", web::get().to(work_progs::delete_work_category))
    .service(web::resource("/work/{cat_id}/{work_id}/").route(web::get().to(work_progs::get_work_page)))
    .service(web::resource("/work/{id}/").route(web::get().to(work_progs::work_category_page)))

    // blogs urls
    .route("/blog_categories/", web::get().to(blog_progs::blog_categories_page))
    .service(web::resource("/create_blog_categories/")
        .route(web::get().to(blog_progs::create_blog_categories_page))
        .route(web::post().to(blog_progs::create_blog_categories))
    )
    .service(web::resource("/edit_blog_category/{id}/")
        .route(web::get().to(blog_progs::edit_blog_category_page))
        .route(web::post().to(blog_progs::edit_blog_category))
    )
    .service(web::resource("/create_blog/")
        .route(web::get().to(blog_progs::create_blog_page))
        .route(web::post().to(blog_progs::create_blog))
    )
    .service(web::resource("/edit_blog/{id}/")
        .route(web::get().to(blog_progs::edit_blog_page))
        .route(web::post().to(blog_progs::edit_blog))
    )
    .service(web::resource("/edit_content_blog/{id}/")
        .route(web::get().to(blog_progs::edit_content_blog_page))
        .route(web::post().to(blog_progs::edit_content_blog))
    )
    .route("/delete_blog/{id}/", web::get().to(blog_progs::delete_blog))
    .route("/delete_blog_category/{id}/", web::get().to(blog_progs::delete_blog_category))
    .service(web::resource("/blog/{cat_id}/{blog_id}/").route(web::get().to(blog_progs::get_blog_page)))
    .service(web::resource("/blog/{id}/").route(web::get().to(blog_progs::blog_category_page)))

    // store urls
    .route("/store_categories/", web::get().to(store_progs::store_categories_page))
    .service(web::resource("/create_store_categories/")
        .route(web::get().to(store_progs::create_store_categories_page))
        .route(web::post().to(store_progs::create_store_categories))
    )
    .service(web::resource("/edit_store_category/{id}/")
        .route(web::get().to(store_progs::edit_store_category_page))
        .route(web::post().to(store_progs::edit_store_category))
    )
    .service(web::resource("/create_store/")
        .route(web::get().to(store_progs::create_store_page))
        .route(web::post().to(store_progs::create_store))
    )
    .service(web::resource("/edit_store/{id}/")
        .route(web::get().to(store_progs::edit_store_page))
        .route(web::post().to(store_progs::edit_store))
    )
    .route("/delete_store/{id}/", web::get().to(store_progs::delete_store))
    .route("/delete_store_category/{id}/", web::get().to(store_progs::delete_store_category))
    .service(web::resource("/store/{cat_id}/{store_id}/").route(web::get().to(store_progs::get_store_page)))
    .service(web::resource("/store/{id}/").route(web::get().to(store_progs::store_category_page)))

    // service urls
    .route("/service_categories/", web::get().to(service_progs::service_categories_page))
    .service(web::resource("/create_service_categories/")
        .route(web::get().to(service_progs::create_service_categories_page))
        .route(web::post().to(service_progs::create_service_categories))
    )
    .service(web::resource("/edit_service_category/{id}/")
        .route(web::get().to(service_progs::edit_service_category_page))
        .route(web::post().to(service_progs::edit_service_category))
    )
    .service(web::resource("/create_service/")
        .route(web::get().to(service_progs::create_service_page))
        .route(web::post().to(service_progs::create_service))
    )
    .service(web::resource("/edit_service/{id}/")
        .route(web::get().to(service_progs::edit_service_page))
        .route(web::post().to(service_progs::edit_service))
    )
    .route("/delete_service/{id}/", web::get().to(service_progs::delete_service))
    .route("/delete_service_category/{id}/", web::get().to(service_progs::delete_service_category))
    .service(web::resource("/service/{cat_id}/{service_id}/").route(web::get().to(service_progs::get_service_page)))
    .service(web::resource("/service/{id}/").route(web::get().to(service_progs::service_category_page)))

    // wiki urls
    .route("/wiki_categories/", web::get().to(wiki_progs::wiki_categories_page))
    .service(web::resource("/create_wiki_categories/")
        .route(web::get().to(wiki_progs::create_wiki_categories_page))
        .route(web::post().to(wiki_progs::create_wiki_categories))
    )
    .service(web::resource("/edit_wiki_category/{id}/")
        .route(web::get().to(wiki_progs::edit_wiki_category_page))
        .route(web::post().to(wiki_progs::edit_wiki_category))
    )
    .service(web::resource("/create_wiki/")
        .route(web::get().to(wiki_progs::create_wiki_page))
        .route(web::post().to(wiki_progs::create_wiki))
    )
    .service(web::resource("/edit_wiki/{id}/")
        .route(web::get().to(wiki_progs::edit_wiki_page))
        .route(web::post().to(wiki_progs::edit_wiki))
    )
    .route("/delete_wiki/{id}/", web::get().to(wiki_progs::delete_wiki))
    .route("/delete_wiki_category/{id}/", web::get().to(wiki_progs::delete_wiki_category))
    .service(web::resource("/wiki/{cat_id}/{wiki_id}/").route(web::get().to(wiki_progs::get_wiki_page)))
    .service(web::resource("/wiki/{id}/").route(web::get().to(wiki_progs::wiki_category_page)))
    ;
}
