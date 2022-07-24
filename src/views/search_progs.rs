use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
};
use crate::diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
    PgTextExpressionMethods,
};
use actix_session::Session;
use serde::Deserialize;
use crate::utils::{
    establish_connection,
    is_signed_in,
    get_request_user_data,
};
use crate::schema;
use sailfish::TemplateOnce;
use crate::models::User;


pub fn search_routes(config: &mut web::ServiceConfig) {
    config.route("/search/{q}/", web::get().to(search_page));
    config.route("/search_blogs/{q}/", web::get().to(search_blogs_page));
    config.route("/search_services/{q}/", web::get().to(search_services_page));
    config.route("/search_stores/{q}/", web::get().to(search_stores_page));
    config.route("/search_wikis/{q}/", web::get().to(search_wikis_page));
    config.route("/search_works/{q}/", web::get().to(search_works_page));
}

pub async fn search_page(session: Session, req: HttpRequest, q: web::Path<String>) -> actix_web::Result<HttpResponse> {
    use crate::models::{Work, Blog, Service, Store, Wiki};

    let _connection = establish_connection();
    let _q_standalone = "%".to_owned() + &_q.clone() + "%";

    let _blogs = schema::blogs::table
        .filter(schema::blogs::title.eq(&_q_standalone))
        .or_filter(schema::blogs::description.eq(&_q_standalone))
        .or_filter(schema::blogs::content.eq(&_q_standalone))
        .order(schema::blogs::created.desc())
        .load::<Blog>(&_connection)
        .expect("e");
    let blogs_count = _blogs.len();
    let blog_list = &_blogs[..3];

    let _services = schema::services::table
        .filter(schema::services::title.ilike(&_q_standalone))
        .or_filter(schema::services::description.ilike(&_q_standalone))
        .or_filter(schema::services::content.ilike(&_q_standalone))
        .order(schema::services::created.desc())
        .load::<Service>(&_connection)
        .expect("e");
    let services_count = _services.len();
    let service_list = &_services[..3];

    let _stores = schema::stores::table
        .filter(schema::stores::title.eq(&_q))
        .or_filter(schema::stores::description.eq(&_q_standalone))
        .or_filter(schema::stores::content.eq(&_q_standalone))
        .order(schema::stores::created.desc())
        .load::<Store>(&_connection)
        .expect("e");
    let stores_count = _stores.len();
    let store_list = &_stores[..3];

    let _wikis = schema::wikis::table
        .filter(schema::wikis::title.eq(&_q))
        .or_filter(schema::wikis::description.eq(&_q_standalone))
        .or_filter(schema::wikis::content.eq(&_q_standalone))
        .order(schema::wikis::created.desc())
        .load::<Wiki>(&_connection)
        .expect("e");
    let wiki_count = _wikis.len();
    let wiki_list = &_wikis[..3];

    let _works = schema::works::table
        .filter(schema::works::title.eq(&_q))
        .or_filter(schema::works::description.eq(&_q_standalone))
        .or_filter(schema::works::content.eq(&_q_standalone))
        .order(schema::works::created.desc())
        .load::<Work>(&_connection)
        .expect("e");
    let work_count = _works.len();
    let work_list = &_works[..3];

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/search/all.stpl")]
            struct Template {
                request_user:   User,
                works_list:     Vec<Work>,
                services_list:  Vec<Service>,
                wikis_list:     Vec<Wiki>,
                blogs_list:     Vec<Blog>,
                stores_list:    Vec<Store>,

                works_count:    usize,
                services_count: usize,
                wikis_count:    usize,
                blogs_count:    usize,
                stores_count:   usize,
                is_ajax:        bool,
                q:              String,
            }
            let body = Template {
                request_user:   _request_user,
                works_list:     work_list.to_vec(),
                services_list:  service_list.to_vec(),
                wikis_list:     wiki_list.to_vec(),
                blogs_list:     blog_list.to_vec(),
                stores_list:    store_list.to_vec(),

                works_count:    works_count,
                services_count: services_count,
                wikis_counts:   wikis_count,
                blogs_count:    blogs_count,
                stores_count:   stores_count,
                is_ajax:        is_ajax,
                q:              q.to_string(),
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/search/all.stpl")]
            struct Template {
                request_user:   User,
                works_list:     Vec<Work>,
                services_list:  Vec<Service>,
                wikis_list:     Vec<Wiki>,
                blogs_list:     Vec<Blog>,
                stores_list:    Vec<Store>,

                works_count:    usize,
                services_count: usize,
                wikis_count:    usize,
                blogs_count:    usize,
                stores_count:   usize,
                is_ajax:        bool,
                q:              String,
            }
            let body = Template {
                request_user:   _request_user,
                works_list:     work_list.to_vec(),
                services_list:  service_list.to_vec(),
                wikis_list:     wiki_list.to_vec(),
                blogs_list:     blog_list.to_vec(),
                stores_list:    store_list.to_vec(),

                works_count:    works_count,
                services_count: services_count,
                wikis_counts:   wikis_count,
                blogs_count:    blogs_count,
                stores_count:   stores_count,
                is_ajax:        is_ajax,
                q:              q.to_string(),
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/search/anon_all.stpl")]
            struct Template {
                works_list:     Vec<Work>,
                services_list:  Vec<Service>,
                wikis_list:     Vec<Wiki>,
                blogs_list:     Vec<Blog>,
                stores_list:    Vec<Store>,

                works_count:    usize,
                services_count: usize,
                wikis_count:    usize,
                blogs_count:    usize,
                stores_count:   usize,
                is_ajax:        bool,
                q:              String,
            }
            let body = Template {
                works_list:     work_list.to_vec(),
                services_list:  service_list.to_vec(),
                wikis_list:     wiki_list.to_vec(),
                blogs_list:     blog_list.to_vec(),
                stores_list:    store_list.to_vec(),

                works_count:    works_count,
                services_count: services_count,
                wikis_counts:   wikis_count,
                blogs_count:    blogs_count,
                stores_count:   stores_count,
                is_ajax:        is_ajax,
                q:              q.to_string(),
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/search/anon_all.stpl")]
            struct Template {
                works_list:     Vec<Work>,
                services_list:  Vec<Service>,
                wikis_list:     Vec<Wiki>,
                blogs_list:     Vec<Blog>,
                stores_list:    Vec<Store>,

                works_count:    usize,
                services_count: usize,
                wikis_count:    usize,
                blogs_count:    usize,
                stores_count:   usize,
                is_ajax:        bool,
                q:              String,
            }
            let body = Template {
                works_list:     work_list.to_vec(),
                services_list:  service_list.to_vec(),
                wikis_list:     wiki_list.to_vec(),
                blogs_list:     blog_list.to_vec(),
                stores_list:    store_list.to_vec(),

                works_count:    works_count,
                services_count: services_count,
                wikis_count:    wikis_count,
                blogs_count:    blogs_count,
                stores_count:   stores_count,
                is_ajax:        is_ajax,
                q:              q.to_string(),
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn search_blogs_page(session: Session, req: HttpRequest, q: web::Path<String>) -> actix_web::Result<HttpResponse> {
    use crate::schema::blogs::dsl::blogs;
    use crate::models::Blog;
    use crate::utils::{get_device_and_ajax, get_page};

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let page = get_page(&req);

    let _connection = establish_connection();
    let _q_standalone = "%".to_owned() + &_q.clone() + "%";

    let mut next_page_number = 0;
    let offset: i32;
    let next_item: i32;
    if page > 1 {
        offset = (page - 1) * 20;
        next_item = page * 20 + 1;
    }
    else {
        offset = 0;
        next_item = 21;
    }

    let _blogs = blogs
        .filter(schema::blogs::title.ilike(&_q_standalone))
        .or_filter(schema::blogs::description.ilike(&_q_standalone))
        .or_filter(schema::blogs::content.ilike(&_q_standalone))
        .limit(20)
        .offset(offset.into())
        .order(schema::blogs::created.desc())
        .load::<Blog>(&_connection)
        .expect("e");

    let blogs_count = _blogs.len();

    if blogs
        .filter(schema::blogs::title.ilike(&_q_standalone))
        .or_filter(schema::blogs::description.ilike(&_q_standalone))
        .or_filter(schema::blogs::content.ilike(&_q_standalone))
        .limit(1)
        .offset(next_item.into())
        .select(schema::blogs::id)
        .load::<i32>(&_connection)
        .expect("e")
        .len() > 0 {
            next_page_number = page + 1;
        }

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/search/blogs.stpl")]
            struct Template {
                request_user:     User,
                blogs_list:       Vec<Blog>,
                blogs_count:      usize,
                is_ajax:          bool,
                q:                String,
                next_page_number: i32,
            }
            let body = Template {
                request_user:     _request_user,
                blogs_list:       _blogs,
                blogs_count:      blogs_count,
                is_ajax:          is_ajax,
                q:                q.to_string(),
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/search/blogs.stpl")]
            struct Template {
                request_user:     User,
                blogs_list:       Vec<Blog>,
                blogs_count:      usize,
                is_ajax:          bool,
                q:                String,
                next_page_number: i32,
            }
            let body = Template {
                request_user:     _request_user,
                blogs_list:       _blogs,
                blogs_count:      blogs_count,
                is_ajax:          is_ajax,
                q:                q.to_string(),
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/search/anon_blogs.stpl")]
            struct Template {
                blogs_list:       Vec<Blog>,
                blogs_count:      usize,
                is_ajax:          bool,
                q:                String,
                next_page_number: i32,
            }
            let body = Template {
                blogs_list:       _blogs,
                blogs_count:      blogs_count,
                is_ajax:          is_ajax,
                q:                q.to_string(),
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/search/anon_blogs.stpl")]
            struct Template {
                blogs_list:       Vec<Blog>,
                blogs_count:      usize,
                is_ajax:          bool,
                q:                String,
                next_page_number: i32,
            }
            let body = Template {
                blogs_list:       _blogs,
                blogs_count:      blogs_count,
                is_ajax:          is_ajax,
                q:                q.to_string(),
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }

}

pub async fn search_services_page(session: Session, req: HttpRequest, q: web::Path<String>) -> actix_web::Result<HttpResponse> {
    use crate::schema::services::dsl::services;
    use crate::models::Service;
    use crate::utils::{get_device_and_ajax, get_page};

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let page = get_page(&req);

    let _connection = establish_connection();
    let _q_standalone = "%".to_owned() + &_q.clone() + "%";

    let mut next_page_number = 0;
    let offset: i32;
    let next_item: i32;
    if page > 1 {
        offset = (page - 1) * 20;
        next_item = page * 20 + 1;
    }
    else {
        offset = 0;
        next_item = 21;
    }

    let _services = services
        .filter(schema::services::title.ilike(&_q_standalone))
        .or_filter(schema::services::description.ilike(&_q_standalone))
        .or_filter(schema::services::content.ilike(&_q_standalone))
        .limit(20)
        .offset(offset.into())
        .order(schema::services::created.desc())
        .load::<Service>(&_connection)
        .expect("e");

    let services_count = _services.len();

    if services
        .filter(schema::services::title.ilike(&_q_standalone))
        .or_filter(schema::services::description.ilike(&_q_standalone))
        .or_filter(schema::services::content.ilike(&_q_standalone))
        .limit(1)
        .offset(next_item.into())
        .select(schema::services::id)
        .load::<i32>(&_connection)
        .expect("e")
        .len() > 0 {
            next_page_number = page + 1;
        }

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/search/services.stpl")]
            struct Template {
                request_user:     User,
                services_list:    Vec<Service>,
                services_count:   usize,
                is_ajax:          bool,
                q:                String,
                next_page_number: i32,
            }
            let body = Template {
                request_user:     _request_user,
                services_list:    _services,
                services_count:   services_count,
                is_ajax:          is_ajax,
                q:                q.to_string(),
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/search/services.stpl")]
            struct Template {
                request_user:     User,
                services_list:    Vec<Service>,
                services_count:   usize,
                is_ajax:          bool,
                q:                String,
                next_page_number: i32,
            }
            let body = Template {
                request_user:     _request_user,
                services_list:    _services,
                services_count:   services_count,
                is_ajax:          is_ajax,
                q:                q.to_string(),
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/search/anon_services.stpl")]
            struct Template {
                services_list:    Vec<Service>,
                services_count:   usize,
                is_ajax:          bool,
                q:                String,
                next_page_number: i32,
            }
            let body = Template {
                services_list:    _services,
                services_count:   services_count,
                is_ajax:          is_ajax,
                q:                q.to_string(),
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/search/anon_services.stpl")]
            struct Template {
                services_list:    Vec<Service>,
                services_count:   usize,
                is_ajax:          bool,
                q:                String,
                next_page_number: i32,
            }
            let body = Template {
                services_list:    _services,
                services_count:   services_count,
                is_ajax:          is_ajax,
                q:                q.to_string(),
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }

}

pub async fn search_stores_page(session: Session, req: HttpRequest, q: web::Path<String>) -> actix_web::Result<HttpResponse> {
    use crate::schema::stores::dsl::stores;
    use crate::models::Store;
    use crate::utils::{get_device_and_ajax, get_page};

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let page = get_page(&req);

    let _connection = establish_connection();
    let _q_standalone = "%".to_owned() + &_q.clone() + "%";

    let mut next_page_number = 0;
    let offset: i32;
    let next_item: i32;
    if page > 1 {
        offset = (page - 1) * 20;
        next_item = page * 20 + 1;
    }
    else {
        offset = 0;
        next_item = 21;
    }

    let _stores = stores
        .filter(schema::stores::title.ilike(&_q_standalone))
        .or_filter(schema::stores::description.ilike(&_q_standalone))
        .or_filter(schema::stores::content.ilike(&_q_standalone))
        .limit(20)
        .offset(offset.into())
        .order(schema::stores::created.desc())
        .load::<Store>(&_connection)
        .expect("e");

    let stores_count = _stores.len();

    if stores
        .filter(schema::stores::title.ilike(&_q_standalone))
        .or_filter(schema::stores::description.ilike(&_q_standalone))
        .or_filter(schema::stores::content.ilike(&_q_standalone))
        .limit(1)
        .offset(next_item.into())
        .select(schema::stores::id)
        .load::<i32>(&_connection)
        .expect("e")
        .len() > 0 {
            next_page_number = page + 1;
        }

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/search/stores.stpl")]
            struct Template {
                request_user:     User,
                stores_list:       Vec<Store>,
                stores_count:      usize,
                is_ajax:          bool,
                q:                String,
                next_page_number: i32,
            }
            let body = Template {
                request_user:     _request_user,
                stores_list:       _stores,
                stores_count:      stores_count,
                is_ajax:          is_ajax,
                q:                q.to_string(),
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/search/stores.stpl")]
            struct Template {
                request_user:     User,
                stores_list:       Vec<Store>,
                stores_count:      usize,
                is_ajax:          bool,
                q:                String,
                next_page_number: i32,
            }
            let body = Template {
                request_user:     _request_user,
                stores_list:       _stores,
                stores_count:      stores_count,
                is_ajax:          is_ajax,
                q:                q.to_string(),
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/search/anon_stores.stpl")]
            struct Template {
                stores_list:       Vec<Store>,
                stores_count:      usize,
                is_ajax:          bool,
                q:                String,
                next_page_number: i32,
            }
            let body = Template {
                stores_list:       _stores,
                stores_count:      stores_count,
                is_ajax:          is_ajax,
                q:                q.to_string(),
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/search/anon_stores.stpl")]
            struct Template {
                stores_list:       Vec<Store>,
                stores_count:      usize,
                is_ajax:          bool,
                q:                String,
                next_page_number: i32,
            }
            let body = Template {
                stores_list:       _stores,
                stores_count:      stores_count,
                is_ajax:          is_ajax,
                q:                q.to_string(),
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }

}

pub async fn search_wikis_page(session: Session, req: HttpRequest, q: web::Path<String>) -> actix_web::Result<HttpResponse> {
    use crate::schema::wikis::dsl::wikis;
    use crate::models::Wiki;
    use crate::utils::{get_device_and_ajax, get_page};

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let page = get_page(&req);

    let _connection = establish_connection();
    let _q_standalone = "%".to_owned() + &_q.clone() + "%";

    let mut next_page_number = 0;
    let offset: i32;
    let next_item: i32;
    if page > 1 {
        offset = (page - 1) * 20;
        next_item = page * 20 + 1;
    }
    else {
        offset = 0;
        next_item = 21;
    }

    let _wikis = wikis
        .filter(schema::wikis::title.ilike(&_q_standalone))
        .or_filter(schema::wikis::description.ilike(&_q_standalone))
        .or_filter(schema::wikis::content.ilike(&_q_standalone))
        .limit(20)
        .offset(offset.into())
        .order(schema::wikis::created.desc())
        .load::<Wiki>(&_connection)
        .expect("e");

    let wikis_count = _wikis.len();

    if wikis
        .filter(schema::wikis::title.ilike(&_q_standalone))
        .or_filter(schema::wikis::description.ilike(&_q_standalone))
        .or_filter(schema::wikis::content.ilike(&_q_standalone))
        .limit(1)
        .offset(next_item.into())
        .select(schema::wikis::id)
        .load::<i32>(&_connection)
        .expect("e")
        .len() > 0 {
            next_page_number = page + 1;
        }

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/search/wikis.stpl")]
            struct Template {
                request_user:     User,
                wikis_list:       Vec<Wiki>,
                wikis_count:      usize,
                is_ajax:          bool,
                q:                String,
                next_page_number: i32,
            }
            let body = Template {
                request_user:     _request_user,
                wikis_list:       _wikis,
                wikis_count:      wikis_count,
                is_ajax:          is_ajax,
                q:                q.to_string(),
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/search/wikis.stpl")]
            struct Template {
                request_user:     User,
                wikis_list:       Vec<Wiki>,
                wikis_count:      usize,
                is_ajax:          bool,
                q:                String,
                next_page_number: i32,
            }
            let body = Template {
                request_user:     _request_user,
                wikis_list:       _wikis,
                wikis_count:      wikis_count,
                is_ajax:          is_ajax,
                q:                q.to_string(),
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/search/anon_wikis.stpl")]
            struct Template {
                wikis_list:       Vec<Wiki>,
                wikis_count:      usize,
                is_ajax:          bool,
                q:                String,
                next_page_number: i32,
            }
            let body = Template {
                wikis_list:       _wikis,
                wikis_count:      wikis_count,
                is_ajax:          is_ajax,
                q:                q.to_string(),
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/search/anon_wikis.stpl")]
            struct Template {
                wikis_list:       Vec<Wiki>,
                wikis_count:      usize,
                is_ajax:          bool,
                q:                String,
                next_page_number: i32,
            }
            let body = Template {
                wikis_list:       _wikis,
                wikis_count:      wikis_count,
                is_ajax:          is_ajax,
                q:                q.to_string(),
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }

}

pub async fn search_works_page(session: Session, req: HttpRequest, q: web::Path<String>) -> actix_web::Result<HttpResponse> {
    use crate::schema::works::dsl::works;
    use crate::models::Work;
    use crate::utils::{get_device_and_ajax, get_page};

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let page = get_page(&req);

    let _connection = establish_connection();
    let _q_standalone = "%".to_owned() + &_q.clone() + "%";

    let mut next_page_number = 0;
    let offset: i32;
    let next_item: i32;
    if page > 1 {
        offset = (page - 1) * 20;
        next_item = page * 20 + 1;
    }
    else {
        offset = 0;
        next_item = 21;
    }

    let _works = works
        .filter(schema::works::title.ilike(&_q_standalone))
        .or_filter(schema::works::description.ilike(&_q_standalone))
        .or_filter(schema::works::content.ilike(&_q_standalone))
        .limit(20)
        .offset(offset.into())
        .order(schema::works::created.desc())
        .load::<Work>(&_connection)
        .expect("e");

    let works_count = _works.len();

    if works
        .filter(schema::works::title.ilike(&_q_standalone))
        .or_filter(schema::works::description.ilike(&_q_standalone))
        .or_filter(schema::works::content.ilike(&_q_standalone))
        .limit(1)
        .offset(next_item.into())
        .select(schema::works::id)
        .load::<i32>(&_connection)
        .expect("e")
        .len() > 0 {
            next_page_number = page + 1;
        }

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/search/works.stpl")]
            struct Template {
                request_user:     User,
                works_list:       Vec<Work>,
                works_count:      usize,
                is_ajax:          bool,
                q:                String,
                next_page_number: i32,
            }
            let body = Template {
                request_user:     _request_user,
                works_list:       _works,
                works_count:      works_count,
                is_ajax:          is_ajax,
                q:                q.to_string(),
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/search/works.stpl")]
            struct Template {
                request_user:     User,
                works_list:       Vec<Work>,
                works_count:      usize,
                is_ajax:          bool,
                q:                String,
                next_page_number: i32,
            }
            let body = Template {
                request_user:     _request_user,
                works_list:       _works,
                works_count:      works_count,
                is_ajax:          is_ajax,
                q:                q.to_string(),
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/search/anon_works.stpl")]
            struct Template {
                works_list:       Vec<Work>,
                works_count:      usize,
                is_ajax:          bool,
                q:                String,
                next_page_number: i32,
            }
            let body = Template {
                works_list:       _works,
                works_count:      works_count,
                is_ajax:          is_ajax,
                q:                q.to_string(),
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/search/anon_works.stpl")]
            struct Template {
                works_list:       Vec<Work>,
                works_count:      usize,
                is_ajax:          bool,
                q:                String,
                next_page_number: i32,
            }
            let body = Template {
                works_list:       _works,
                works_count:      works_count,
                is_ajax:          is_ajax,
                q:                q.to_string(),
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }

}
