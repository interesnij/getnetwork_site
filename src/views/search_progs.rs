use actix_web::{
    HttpRequest,
    Responder,
    HttpResponse,
    web
};
use tera::{Tera, Context};
use serde::Deserialize;
use crate::utils::{get_template_2, establish_connection};
use crate::schema;
use diesel::prelude::*;


#[derive(Debug, Deserialize)]
pub struct SearchParams {
    pub q: String,
}

pub async fn search_page(req: HttpRequest, tera: web::Data<Tera>) -> impl Responder {
    use crate::models::{Work, Blog, Service, Store, Wiki};

    let _connection = establish_connection();

    let params = web::Query::<SearchParams>::from_query(&req.query_string());
    let _q = params.q.clone();
    let _q_standalone = "%".to_owned() + &_q + "%";

    let _blogs = schema::blogs::table
        .filter(schema::blogs::title.eq(&_q_standalone))
        .or_filter(schema::blogs::description.eq(&_q_standalone))
        .or_filter(schema::blogs::content.eq(&_q_standalone))
        .order(schema::blogs::blog_created.desc())
        .limit(3)
        .load::<Blog>(&_connection)
        .expect("e");
    let _services = schema::services::table
        .filter(schema::services::title.ilike(&_q_standalone))
        .or_filter(schema::services::description.ilike(&_q_standalone))
        .or_filter(schema::services::content.ilike(&_q_standalone))
        .order(schema::services::service_created.desc())
        .limit(3)
        .load::<Service>(&_connection)
        .expect("e");
    let _stores = schema::stores::table
        .filter(schema::stores::title.eq(&_q))
        .or_filter(schema::stores::description.eq(&_q_standalone))
        .or_filter(schema::stores::content.eq(&_q_standalone))
        .order(schema::stores::store_created.desc())
        .limit(3)
        .load::<Store>(&_connection)
        .expect("e");
    let _wikis = schema::wikis::table
        .filter(schema::wikis::title.eq(&_q))
        .or_filter(schema::wikis::description.eq(&_q_standalone))
        .or_filter(schema::wikis::content.eq(&_q_standalone))
        .order(schema::wikis::wiki_created.desc())
        .limit(3)
        .load::<Wiki>(&_connection)
        .expect("e");
    let _works = schema::works::table
        .filter(schema::works::title.eq(&_q))
        .or_filter(schema::works::description.eq(&_q_standalone))
        .or_filter(schema::works::content.eq(&_q_standalone))
        .order(schema::works::work_created.desc())
        .limit(3)
        .load::<Work>(&_connection)
        .expect("e");

    let mut data = Context::new();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("blogs", &_blogs);
    data.insert("services", &_services);
    data.insert("stores", &_stores);
    data.insert("wikis", &_wikis);
    data.insert("works", &_works);
    data.insert("blogs_count", &_blogs.len());
    data.insert("services_count", &_services.len());
    data.insert("stores_count", &_stores.len());
    data.insert("wikis_count", &_wikis.len());
    data.insert("works_count", &_works.len());
    data.insert("is_admin", &_is_admin);
    data.insert("q", &_q);

    let _template = _type + &"search/all.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn search_blogs_page(req: HttpRequest, tera: web::Data<Tera>) -> impl Responder {
    use crate::schema::blogs::dsl::blogs;
    use crate::models::Blog;

    let _connection = establish_connection();
    let params = web::Query::<SearchParams>::from_query(&req.query_string()).unwrap();
    let _q = params.q.clone();
    let _q_standalone = "%".to_owned() + &_q + "%";

    let page_size = 20;
    let mut offset = 0;
    let mut data = Context::new();

    loop {
        let _blogs = blogs
            .filter(schema::blogs::title.ilike(&_q_standalone))
            .or_filter(schema::blogs::description.ilike(&_q_standalone))
            .or_filter(schema::blogs::content.ilike(&_q_standalone))
            .limit(page_size)
            .offset(offset)
            .order(schema::blogs::blog_created.desc())
            .load::<Blog>(&_connection)
            .expect("e");
        if _blogs.len() > 0 {
            data.insert("blogs", &_blogs);
            data.insert("blogs_count", &blogs
                .filter(schema::blogs::title.ilike(&_q_standalone))
                .or_filter(schema::blogs::description.ilike(&_q_standalone))
                .or_filter(schema::blogs::content.ilike(&_q_standalone))
                .load::<Blog>(&_connection)
                .expect("E")
                .len());
            offset += page_size;
        }
        else {break;}
    }

    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);
    data.insert("q", &_q);

    let _template = _type + &"search/blogs.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn search_services_page(req: HttpRequest, tera: web::Data<Tera>) -> impl Responder {
    use crate::schema::services::dsl::services;
    use crate::models::Service;

    let _connection = establish_connection();
    let params = web::Query::<SearchParams>::from_query(&req.query_string()).unwrap();
    let _q = params.q.clone();
    let _q_standalone = "%".to_owned() + &_q + "%";

    let page_size = 20;
    let mut offset = 0;
    let mut data = Context::new();

    loop {
        let _services = services
            .filter(schema::services::title.ilike(&_q_standalone))
            .or_filter(schema::services::description.ilike(&_q_standalone))
            .or_filter(schema::services::content.ilike(&_q_standalone))
            .limit(page_size)
            .offset(offset)
            .order(schema::services::service_created.desc())
            .load::<Service>(&_connection)
            .expect("e");
        if _services.len() > 0 {
            data.insert("services", &_services);
            data.insert("services_count", &services
                .filter(schema::services::title.ilike(&_q_standalone))
                .or_filter(schema::services::description.ilike(&_q_standalone))
                .or_filter(schema::services::content.ilike(&_q_standalone))
                .load::<Service>(&_connection)
                .expect("E")
                .len());
            offset += page_size;
        }
        else {break;}
    }

    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);
    data.insert("q", &_q);

    let _template = _type + &"search/services.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn search_stores_page(req: HttpRequest, tera: web::Data<Tera>) -> impl Responder {
    use crate::schema::stores::dsl::stores;
    use crate::models::Store;

    let _connection = establish_connection();
    let params = web::Query::<SearchParams>::from_query(&req.query_string()).unwrap();
    let _q = params.q.clone();
    let _q_standalone = "%".to_owned() + &_q + "%";

    let page_size = 20;
    let mut offset = 0;
    let mut data = Context::new();

    loop {
        let _stores = stores
            .filter(schema::stores::title.ilike(&_q_standalone))
            .or_filter(schema::stores::description.ilike(&_q_standalone))
            .or_filter(schema::stores::content.ilike(&_q_standalone))
            .limit(page_size)
            .offset(offset)
            .order(schema::stores::store_created.desc())
            .load::<Store>(&_connection)
            .expect("e");
        if _stores.len() > 0 {
            data.insert("stores", &_stores);
            data.insert("stores_count", &stores
                .filter(schema::stores::title.ilike(&_q_standalone))
                .or_filter(schema::stores::description.ilike(&_q_standalone))
                .or_filter(schema::stores::content.ilike(&_q_standalone))
                .load::<Store>(&_connection)
                .expect("E")
                .len());
            offset += page_size;
        }
        else {break;}
    }

    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);
    data.insert("q", &_q);

    let _template = _type + &"search/stores.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn search_wikis_page(req: HttpRequest, tera: web::Data<Tera>) -> impl Responder {
    use crate::schema::wikis::dsl::wikis;
    use crate::models::Wiki;

    let _connection = establish_connection();
    let params = web::Query::<SearchParams>::from_query(&req.query_string()).unwrap();
    let _q = params.q.clone();
    let _q_standalone = "%".to_owned() + &_q + "%";

    let page_size = 20;
    let mut offset = 0;
    let mut data = Context::new();

    loop {
        let _wikis = wikis
            .filter(schema::wikis::title.ilike(&_q_standalone))
            .or_filter(schema::wikis::description.ilike(&_q_standalone))
            .or_filter(schema::wikis::content.ilike(&_q_standalone))
            .limit(page_size)
            .offset(offset)
            .order(schema::wikis::wiki_created.desc())
            .load::<Wiki>(&_connection)
            .expect("e");
        if _wikis.len() > 0 {
            data.insert("wikis", &_wikis);
            data.insert("wikis_count", &wikis
                .filter(schema::wikis::title.ilike(&_q_standalone))
                .or_filter(schema::wikis::description.ilike(&_q_standalone))
                .or_filter(schema::wikis::content.ilike(&_q_standalone))
                .load::<Wiki>(&_connection)
                .expect("E")
                .len());
            offset += page_size;
        }
        else {break;}
    }

    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);
    data.insert("q", &_q);

    let _template = _type + &"search/wikis.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn search_works_page(req: HttpRequest, tera: web::Data<Tera>) -> impl Responder {
    use crate::schema::works::dsl::works;
    use crate::models::Work;

    let _connection = establish_connection();
    let params = web::Query::<SearchParams>::from_query(&req.query_string()).unwrap();
    let _q = params.q.clone();
    let _q_standalone = "%".to_owned() + &_q + "%";

    let page_size = 20;
    let mut offset = 0;
    let mut data = Context::new();

    loop {
        let _works = works
            .filter(schema::works::title.ilike(&_q_standalone))
            .or_filter(schema::works::description.ilike(&_q_standalone))
            .or_filter(schema::works::content.ilike(&_q_standalone))
            .limit(page_size)
            .offset(offset)
            .order(schema::works::work_created.desc())
            .load::<Work>(&_connection)
            .expect("e");
        if _works.len() > 0 {
            data.insert("works", &_works);
            data.insert("works_count", &works
                .filter(schema::works::title.ilike(&_q_standalone))
                .or_filter(schema::works::description.ilike(&_q_standalone))
                .or_filter(schema::works::content.ilike(&_q_standalone))
                .load::<Work>(&_connection)
                .expect("E")
                .len());
            offset += page_size;
        }
        else {break;}
    }

    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);
    data.insert("q", &_q);

    let _template = _type + &"search/works.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}
