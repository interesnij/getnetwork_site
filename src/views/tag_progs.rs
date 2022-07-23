use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
    Responder,
};
use crate::models::User;
use actix_multipart::Multipart;
use std::borrow::BorrowMut;
use crate::diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use actix_session::Session;
use crate::utils::{
    category_form,
    establish_connection,
    is_signed_in,
    get_request_user_data,
};
use crate::schema;
use crate::models::{
    Tag,
    NewTag,
    TagItems,
};
use sailfish::TemplateOnce;


pub fn tag_routes(config: &mut web::ServiceConfig) {
    config.route("/tags/", web::get().to(tags_page));
    config.route("/tag/{id}/", web::get().to(tag_page));
    config.route("/tag_blogs/{id}/", web::get().to(tag_blogs_page));
    config.route("/tag_services/{id}/", web::get().to(tag_services_page));
    config.route("/tag_stores/{id}/", web::get().to(tag_stores_page));
    config.route("/tag_wikis/{id}/", web::get().to(tag_wikis_page));
    config.route("/tag_works/{id}/", web::get().to(tag_works_page));
    config.service(web::resource("/create_tag/")
        .route(web::get().to(create_tag_page))
        .route(web::post().to(create_tag))
    );
    config.service(web::resource("/edit_tag/{id}/")
        .route(web::get().to(edit_tag_page))
        .route(web::post().to(edit_tag))
    );
    config.route("/delete_tag/{id}/", web::get().to(delete_tag));
}

pub async fn create_tag_page(session: Session, req: HttpRequest) -> impl Responder {
    use schema::tags::dsl::tags;
    use crate::utils::{get_request_user_data, get_device_and_ajax};

    let _connection = establish_connection();
    let all_tags: Vec<Tag> = tags
        .load(&_connection)
        .expect("Error.");

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/tags/create_tag.stpl")]
            struct Template {
                request_user: User,
                all_tags:     Vec<Tag>,
                is_ajax:      bool,
            }
            let body = Template {
                request_user: _request_user,
                all_tags:     all_tags,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/tags/create_tag.stpl")]
            struct Template {
                request_user: User,
                all_tags:     Vec<Tag>,
                is_ajax:      bool,
            }
            let body = Template {
                request_user: _request_user,
                all_tags:     all_tags,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied."))
    }
}

pub async fn create_tag(session: Session, mut payload: Multipart) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let form = category_form(payload.borrow_mut()).await;
            let new_tag = NewTag {
                name:          form.name.clone(),
                position:      form.position,
                count:         0,
                blog_count:    0,
                service_count: 0,
                store_count:   0,
                wiki_count:    0,
                work_count:    0,
                user_id:       _request_user.id,
            };
            let _new_tag = diesel::insert_into(schema::tags::table)
                .values(&new_tag)
                .get_result::<Tag>(&_connection)
                .expect("E.");
        }
    }
    return HttpResponse::Ok();
}

pub async fn tag_page(req: HttpRequest, session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use schema::tags::dsl::tags;
    use crate::utils::get_request_user_data;
    use crate::models::{Work, Blog, Service, Store, Wiki};

    let _connection = establish_connection();
    let _tag_id: i32 = *_id;
    let _tag = tags
        .filter(schema::tags::id.eq(_tag_id))
        .load::<Tag>(&_connection)
        .expect("E");

    let _tag_items = tags_items
        .filter(schema::tags_items::tag_id.eq(&_tag_id))
        .load::<TagItems>(&_connection)
        .expect("E");
    let mut blog_stack = Vec::new();
    let mut service_stack = Vec::new();
    let mut store_stack = Vec::new();
    let mut wiki_stack = Vec::new();
    let mut work_stack = Vec::new();
    for _tag_item in _tag_items.iter() {
        if _tag_item.blog_id > 0 {
            blog_stack.push(_tag_item.blog_id);
        } else if _tag_item.service_id > 0 {
            service_stack.push(_tag_item.service_id);
        } else if _tag_item.store_id > 0 {
            store_stack.push(_tag_item.store_id);
        } else if _tag_item.wiki_id > 0 {
            wiki_stack.push(_tag_item.wiki_id);
        } else if _tag_item.work_id > 0 {
            work_stack.push(_tag_item.work_id);
        }
    };

    let _blogs = Blog::get_blogs_list_for_ids(0, 3, &blog_stack).0;
    let _services = Service::get_services_list_for_ids(0, 3, &service_stack).0;
    let _stores = Store::get_stores_list_for_ids(0, 3, &store_stack).0;
    let _wikis = Wiki::get_wikis_list_for_ids(0, 3, &wiki_stack).0;
    let _works = Work::get_works_list_for_ids(0, 3, &work_stack).0;

    let blogs_count = _blogs.len();
    let services_count = _services.len();
    let stores_count = _stores.len();
    let wikis_count = _wikis.len();
    let works_count = _works.len();
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/tags/tag.stpl")]
            struct Template {
                tag:           Tag,
                request_user:  User,
                works_list:    Vec<Work>,
                services_list: Vec<Service>,
                wikis_list:    Vec<Wiki>,
                blogs_list:    Vec<Blog>,
                stores_list:   Vec<Store>,

                works_count:   usize,
                services_count:usize,
                wikis_count:   usize,
                blogs_count:   usize,
                stores_count:  usize,
                is_ajax:       bool,
            }
            let body = Template {
                tag:           _tag.into_iter().nth(0).unwrap(),
                request_user:  _request_user,
                works_list:    _works,
                services_list: _services,
                wikis_list:    _wikis,
                blogs_list:    _blogs,
                stores_list:   _stores,

                works_count:   works_count,
                services_count:services_count,
                wikis_count:   wikis_count,
                blogs_count:   blogs_count,
                stores_count:  stores_count,
                is_ajax:       is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/tags/tag.stpl")]
            struct Template {
                tag:           Tag,
                request_user:  User,
                works_list:    Vec<Work>,
                services_list: Vec<Service>,
                wikis_list:    Vec<Wiki>,
                blogs_list:    Vec<Blog>,
                stores_list:   Vec<Store>,

                works_count:   usize,
                services_count:usize,
                wikis_count:   usize,
                blogs_count:   usize,
                stores_count:  usize,
                is_ajax:       bool,
            }
            let body = Template {
                tag:           _tag.into_iter().nth(0).unwrap(),
                request_user:  _request_user,
                works_list:    _works,
                services_list: _services,
                wikis_list:    _wikis,
                blogs_list:    _blogs,
                stores_list:   _stores,

                works_count:   works_count,
                services_count:services_count,
                wikis_count:   wikis_count,
                blogs_count:   blogs_count,
                stores_count:  stores_count,
                is_ajax:       is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/tags/anon_tag.stpl")]
            struct Template {
                tag:           Tag,
                works_list:    Vec<Work>,
                services_list: Vec<Service>,
                wikis_list:    Vec<Wiki>,
                blogs_list:    Vec<Blog>,
                stores_list:   Vec<Store>,

                works_count:   usize,
                services_count:usize,
                wikis_count:   usize,
                blogs_count:   usize,
                stores_count:  usize,
                is_ajax:       bool,
            }
            let body = Template {
                tag:           _tag.into_iter().nth(0).unwrap(),
                works_list:    _works,
                services_list: _services,
                wikis_list:    _wikis,
                blogs_list:    _blogs,
                stores_list:   _stores,

                works_count:   works_count,
                services_count:services_count,
                wikis_count:   wikis_count,
                blogs_count:   blogs_count,
                stores_count:  stores_count,
                is_ajax:       is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/tags/anon_tag.stpl")]
            struct Template {
                tag:           Tag,
                works_list:    Vec<Work>,
                services_list: Vec<Service>,
                wikis_list:    Vec<Wiki>,
                blogs_list:    Vec<Blog>,
                stores_list:   Vec<Store>,

                works_count:   usize,
                services_count:usize,
                wikis_count:   usize,
                blogs_count:   usize,
                stores_count:  usize,
                is_ajax:       bool,
            }
            let body = Template {
                tag:           _tag.into_iter().nth(0).unwrap(),
                works_list:    _works,
                services_list: _services,
                wikis_list:    _wikis,
                blogs_list:    _blogs,
                stores_list:   _stores,

                works_count:   works_count,
                services_count:services_count,
                wikis_count:   wikis_count,
                blogs_count:   blogs_count,
                stores_count:  stores_count,
                is_ajax:       is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn tag_blogs_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use schema::tags::dsl::tags;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::models::Blog;
    use crate::utils::get_device_and_page_and_ajax;

    let (is_desctop, page, is_ajax) = get_device_and_page_and_ajax(&req);
    let _connection = establish_connection();
    let _tag_id: i32 = *_id;
    let _tag = tags
        .filter(schema::tags::id.eq(_tag_id))
        .load::<Tag>(&_connection)
        .expect("E");

    let _tag_items = tags_items
        .filter(schema::tags_items::tag_id.eq(&_tag_id))
        .select(schema::tags_items::blog_id)
        .load::<i32>(&_connection)
        .expect("E");

    let (_blogs, next_page_number) = Blog::get_blogs_list_for_ids(page, 20, &_tag_items);
    let blog_count = _blogs.len();
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/tags/tag_blogs.stpl")]
            struct Template {
                request_user:     User,
                tag:              Tag,
                blogs_list:       Vec<Blog>,
                blogs_count:      usize,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                request_user:     _request_user,
                tag:              _tag.into_iter().nth(0).unwrap(),
                blogs_list:       _blogs,
                blogs_count:      blog_count,
                next_page_number: next_page_number,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/tags/tag_blogs.stpl")]
            struct Template {
                request_user:     User,
                tag:              Tag,
                blogs_list:       Vec<Blog>,
                blogs_count:      usize,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                request_user:     _request_user,
                tag:              _tag.into_iter().nth(0).unwrap(),
                blogs_list:       _blogs,
                blogs_count:      blog_count,
                next_page_number: next_page_number,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/tags/anon_tag_blogs.stpl")]
            struct Template {
                tag:              Tag,
                blogs_list:       Vec<Blog>,
                blogs_count:      usize,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                tag:              _tag.into_iter().nth(0).unwrap(),
                blogs_list:       _blogs,
                blogs_count:      blog_count,
                next_page_number: next_page_number,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/tags/anon_tag_blogs.stpl")]
            struct Template {
                tag:              Tag,
                blogs_list:       Vec<Blog>,
                blogs_count:      usize,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                tag:              _tag.into_iter().nth(0).unwrap(),
                blogs_list:       _blogs,
                blogs_count:      blog_count,
                next_page_number: next_page_number,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn tag_services_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use schema::tags::dsl::tags;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::models::Service;
    use crate::utils::get_device_and_page_and_ajax;

    let (is_desctop, page, is_ajax) = get_device_and_page_and_ajax(&req);
    let _connection = establish_connection();
    let _tag_id: i32 = *_id;
    let _tag = tags
        .filter(schema::tags::id.eq(_tag_id))
        .load::<Tag>(&_connection)
        .expect("E");

    let _tag_items = tags_items
        .filter(schema::tags_items::tag_id.eq(&_tag_id))
        .select(schema::tags_items::service_id)
        .load::<i32>(&_connection)
        .expect("E");

    let (_services, next_page_number) = Service::get_services_list_for_ids(page, 20, &_tag_items);
    let service_count = _services.len();
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/tags/tag_services.stpl")]
            struct Template {
                request_user:     User,
                tag:              Tag,
                services_list:    Vec<Service>,
                services_count:   usize,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                request_user:     _request_user,
                tag:              _tag.into_iter().nth(0).unwrap(),
                services_list:    _services,
                services_count:   service_count,
                next_page_number: next_page_number,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/tags/tag_services.stpl")]
            struct Template {
                request_user:     User,
                tag:              Tag,
                services_list:    Vec<Service>,
                services_count:   usize,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                request_user:     _request_user,
                tag:              _tag.into_iter().nth(0).unwrap(),
                services_list:    _services,
                services_count:   service_count,
                next_page_number: next_page_number,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/tags/anon_tag_services.stpl")]
            struct Template {
                tag:              Tag,
                services_list:    Vec<Service>,
                services_count:   usize,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                tag:              _tag.into_iter().nth(0).unwrap(),
                services_list:    _services,
                services_count:   service_count,
                next_page_number: next_page_number,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/tags/anon_tag_services.stpl")]
            struct Template {
                tag:              Tag,
                services_list:    Vec<Service>,
                services_count:   usize,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                tag:              _tag.into_iter().nth(0).unwrap(),
                services_list:    _services,
                services_count:   service_count,
                next_page_number: next_page_number,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn tag_stores_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use schema::tags::dsl::tags;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::stores::dsl::stores;
    use crate::models::Store;
    use crate::utils::get_device_and_page_and_ajax;

    let (is_desctop, page, is_ajax) = get_device_and_page_and_ajax(&req);
    let _connection = establish_connection();
    let _tag_id: i32 = *_id;
    let _tag = tags
        .filter(schema::tags::id.eq(_tag_id))
        .load::<Tag>(&_connection)
        .expect("E");

    let _tag_items = tags_items
        .filter(schema::tags_items::tag_id.eq(&_tag_id))
        .select(schema::tags_items::store_id)
        .load::<i32>(&_connection)
        .expect("E");

    let (_stores, next_page_number) = Store::get_stores_list_for_ids(page, 20, &_tag_items);
    let stores_count = _stores.len();
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/tags/tag_stores.stpl")]
            struct Template {
                request_user:     User,
                tag:              Tag,
                stores_list:      Vec<Store>,
                stores_count:     usize,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                request_user:     _request_user,
                tag:              _tag.into_iter().nth(0).unwrap(),
                stores_list:      _stores,
                stores_count:     stores_count,
                next_page_number: next_page_number,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/tags/tag_stores.stpl")]
            struct Template {
                request_user:     User,
                tag:              Tag,
                stores_list:      Vec<Store>,
                stores_count:     usize,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                request_user:     _request_user,
                tag:              _tag.into_iter().nth(0).unwrap(),
                stores_list:      _stores,
                stores_count:     stores_count,
                next_page_number: next_page_number,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/tags/anon_tag_stores.stpl")]
            struct Template {
                tag:              Tag,
                stores_list:      Vec<Store>,
                stores_count:     usize,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                tag:              _tag.into_iter().nth(0).unwrap(),
                stores_list:      _stores,
                stores_count:     stores_count,
                next_page_number: next_page_number,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/tags/anon_tag_stores.stpl")]
            struct Template {
                tag:              Tag,
                stores_list:      Vec<Store>,
                stores_count:     usize,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                tag:              _tag.into_iter().nth(0).unwrap(),
                stores_list:      _stores,
                stores_count:     stores_count,
                next_page_number: next_page_number,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn tag_wikis_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use schema::tags::dsl::tags;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::models::Wiki;
    use crate::utils::get_device_and_page_and_ajax;

    let (is_desctop, page, is_ajax) = get_device_and_page_and_ajax(&req);
    let _connection = establish_connection();
    let _tag_id: i32 = *_id;
    let _tag = tags
        .filter(schema::tags::id.eq(_tag_id))
        .load::<Tag>(&_connection)
        .expect("E");

    let _tag_items = tags_items
        .filter(schema::tags_items::tag_id.eq(&_tag_id))
        .select(schema::tags_items::wiki_id)
        .load::<i32>(&_connection)
        .expect("E");

    let (_wikis, next_page_number) = Wiki::get_wikis_list_for_ids(page, 20, &_tag_items);
    let wikis_count = _wikis.len();
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/tags/tag_wikis.stpl")]
            struct Template {
                request_user:     User,
                tag:              Tag,
                wikis_list:       Vec<Wiki>,
                wikis_count:      usize,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                request_user:     _request_user,
                tag:              _tag.into_iter().nth(0).unwrap(),
                wikis_list:       _wikis,
                wikis_count:      wikis_count,
                next_page_number: next_page_number,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/tags/tag_wikis.stpl")]
            struct Template {
                tag:              Tag,
                wikis_list:       Vec<Wiki>,
                wikis_count:      usize,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                tag:              _tag.into_iter().nth(0).unwrap(),
                wikis_list:       _wikis,
                wikis_count:      wikis_count,
                next_page_number: next_page_number,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/tags/anon_tag_wikis.stpl")]
            struct Template {
                tag:              Tag,
                wikis_list:       Vec<Wiki>,
                wikis_count:      usize,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                tag:              _tag.into_iter().nth(0).unwrap(),
                wikis_list:       _wikis,
                wikis_count:      wikis_count,
                next_page_number: next_page_number,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/tags/anon_tag_wikis.stpl")]
            struct Template {
                tag:              Tag,
                wikis_list:       Vec<Wiki>,
                wikis_count:      usize,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                tag:              _tag.into_iter().nth(0).unwrap(),
                wikis_list:       _wikis,
                wikis_count:      wikis_count,
                next_page_number: next_page_number,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn tag_works_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use schema::tags::dsl::tags;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::models::Work;
    use crate::utils::get_device_and_page_and_ajax;

    let (is_desctop, page, is_ajax) = get_device_and_page_and_ajax(&req);
    let _connection = establish_connection();
    let _tag_id: i32 = *_id;
    let _tag = tags
        .filter(schema::tags::id.eq(_tag_id))
        .load::<Tag>(&_connection)
        .expect("E");

    let _tag_items = tags_items
        .filter(schema::tags_items::tag_id.eq(&_tag_id))
        .select(schema::tags_items::work_id)
        .load::<i32>(&_connection)
        .expect("E");

    let (_works, next_page_number) = Work::get_works_list_for_ids(page, 20, &_tag_items);
    let works_count = _works.len();
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/tags/tag_works.stpl")]
            struct Template {
                request_user:     User,
                tag:              Tag,
                works_list:       Vec<Work>,
                works_count:      usize,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                request_user:     _request_user,
                tag:              _tag.into_iter().nth(0).unwrap(),
                works_list:       _works,
                works_count:      works_count,
                next_page_number: next_page_number,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/tags/tag_works.stpl")]
            struct Template {
                request_user:     User,
                tag:              Tag,
                works_list:       Vec<Work>,
                works_count:      usize,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                request_user:     _request_user,
                tag:              _tag.into_iter().nth(0).unwrap(),
                works_list:       _works,
                works_count:      works_count,
                next_page_number: next_page_number,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/tags/anon_tag_works.stpl")]
            struct Template {
                tag:              Tag,
                works_list:       Vec<Work>,
                works_count:      usize,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                tag:              _tag.into_iter().nth(0).unwrap(),
                works_list:       _works,
                works_count:      works_count,
                next_page_number: next_page_number,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/tags/anon_tag_works.stpl")]
            struct Template {
                tag:              Tag,
                works_list:       Vec<Work>,
                works_count:      usize,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                tag:              _tag.into_iter().nth(0).unwrap(),
                works_list:       _works,
                works_count:      works_count,
                next_page_number: next_page_number,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn tags_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_page_and_ajax;

    let (is_desctop, page, is_ajax) = get_device_and_page_and_ajax(&req);
    let _connection = establish_connection();
    let (all_tags, next_page_number) = Tag::get_tags_list(page, 20);
    let tags_count = all_tags.len();

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/tags/tag_works.stpl")]
            struct Template {
                request_user:     User,
                all_tags:         Vec<Tag>,
                tags_count:       usize,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                request_user:     _request_user,
                all_tags:         all_tags,
                tags_count:       tags_count,
                next_page_number: next_page_number,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/tags/tag_works.stpl")]
            struct Template {
                request_user:     User,
                all_tags:         Vec<Tag>,
                tags_count:       usize,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                request_user:     _request_user,
                all_tags:         all_tags,
                tags_count:       tags_count,
                next_page_number: next_page_number,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/tags/anon_tag_works.stpl")]
            struct Template {
                all_tags:         Vec<Tag>,
                tags_count:       usize,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                all_tags:         all_tags,
                tags_count:       tags_count,
                next_page_number: next_page_number,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/tags/anon_tag_works.stpl")]
            struct Template {
                all_tags:         Vec<Tag>,
                tags_count:       usize,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                all_tags:         all_tags,
                tags_count:       tags_count,
                next_page_number: next_page_number,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn edit_tag_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use schema::tags::dsl::*;


    let _tag_id: i32 = *_id;
    let _connection = establish_connection();
    let _tag = tags
        .filter(schema::tags::id.eq(&_tag_id))
        .load::<Tag>(&_connection)
        .expect("E");

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/tags/edit_tag.stpl")]
            struct Template {
                request_user: User,
                tag:          Tag,
                is_ajax:      bool,
            }
            let body = Template {
                request_user: _request_user,
                tag:          _tag.into_iter().nth(0).unwrap(),
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/tags/edit_tag.stpl")]
            struct Template {
                request_user: User,
                tag:          Tag,
                is_ajax:      bool,
            }
            let body = Template {
                request_user: _request_user,
                tag:          _tag.into_iter().nth(0).unwrap(),
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied."))
    }
}

pub async fn edit_tag(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::models::EditTag;
    use crate::schema::tags::dsl::tags;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let _tag_id : i32 = *_id;
            let _tag = tags
                .filter(schema::tags::id.eq(_tag_id))
                .load::<Tag>(&_connection)
                .expect("E");

            let form = category_form(payload.borrow_mut()).await;
            let _new_tag = EditTag {
                name:     form.name.clone(),
                position: form.position,
            };

            diesel::update(&_tag[0])
                .set(_new_tag)
                .get_result::<Tag>(&_connection)
                .expect("E");
        }
    }

    HttpResponse::Ok()
}

pub async fn delete_tag(session: Session, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::tags::dsl::tags;
    use crate::schema::tags_items::dsl::tags_items;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let _tag_id: i32 = *_id;
            let _tag = tags
                .filter(schema::tags::id.eq(_tag_id))
                .load::<Tag>(&_connection)
                .expect("E");
            diesel::delete(tags_items.filter(schema::tags_items::tag_id.eq(_tag_id))).execute(&_connection).expect("E");
            diesel::delete(tags.filter(schema::tags::id.eq(_tag_id))).execute(&_connection).expect("E");
        }
    }
    HttpResponse::Ok()
}
