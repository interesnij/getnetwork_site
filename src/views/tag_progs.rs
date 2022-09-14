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
    establish_connection,
    is_signed_in,
    get_request_user_data,
    get_first_load_page,
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

pub async fn create_tag_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Создание тега".to_string(),
            "вебсервисы.рф: Создание тега".to_string(),
            "/create_tag/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
        ).await
    }
    else {
        use schema::tags::dsl::tags;

        let _connection = establish_connection();
        let all_tags: Vec<Tag> = tags
            .load(&_connection)
            .expect("Error.");

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/tags/create_tag.stpl")]
                struct Template {
                    request_user: User,
                    all_tags:     Vec<Tag>,
                    is_ajax:      i32,
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
                    all_tags:     Vec<Tag>,
                    is_ajax:      i32,
                }
                let body = Template {
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
}

pub async fn create_tag(session: Session, mut payload: Multipart) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use crate::utils::category_form;

            let _connection = establish_connection();
            let form = category_form(payload.borrow_mut(), _request_user.id).await;
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
                view:          0,
                height:        0.0,
                seconds:       0,
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
    use crate::utils::get_device_and_ajax;
    use schema::tags::dsl::tags;

    let _connection = establish_connection();
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let _tag_id: i32 = *_id;
    let _tags = tags
        .filter(schema::tags::id.eq(_tag_id))
        .load::<Tag>(&_connection)
        .expect("E");
    let _tag = _tags.into_iter().nth(0).unwrap();

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Тег ".to_string() + &_tag.name,
            "вебсервисы.рф: Тег ".to_string() + &_tag.name,
            "/tag/".to_string() + &_tag.id.to_string() + &"/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
        ).await
    }
    else {
        use schema::tags_items::dsl::tags_items;
        use crate::models::{Work, Blog, Service, Store, Wiki};

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

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);

            let _blogs = Blog::get_blogs_list_for_ids(&_request_user, 0, 3, &blog_stack).0;
            let _services = Service::get_services_list_for_ids(&_request_user, 0, 3, &service_stack).0;
            let _stores = Store::get_stores_list_for_ids(&_request_user, 0, 3, &store_stack).0;
            let _wikis = Wiki::get_wikis_list_for_ids(&_request_user, 0, 3, &wiki_stack).0;
            let _works = Work::get_works_list_for_ids(&_request_user, 0, 3, &work_stack).0;

            let blogs_count = _blogs.len();
            let services_count = _services.len();
            let stores_count = _stores.len();
            let wikis_count = _wikis.len();
            let works_count = _works.len();
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
                    is_ajax:       i32,
                }
                let body = Template {
                    tag:           _tag,
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
                    is_ajax:       i32,
                }
                let body = Template {
                    tag:           _tag,
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
            let _blogs = Blog::get_publish_blogs_list_for_ids(0, 3, &blog_stack).0;
            let _services = Service::get_publish_services_list_for_ids(0, 3, &service_stack).0;
            let _stores = Store::get_publish_stores_list_for_ids(0, 3, &store_stack).0;
            let _wikis = Wiki::get_publish_wikis_list_for_ids(0, 3, &wiki_stack).0;
            let _works = Work::get_publish_works_list_for_ids(0, 3, &work_stack).0;

            let blogs_count = _blogs.len();
            let services_count = _services.len();
            let stores_count = _stores.len();
            let wikis_count = _wikis.len();
            let works_count = _works.len();

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
                    is_ajax:       i32,
                }
                let body = Template {
                    tag:           _tag,
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
                    is_ajax:       i32,
                }
                let body = Template {
                    tag:           _tag,
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
}

pub async fn tag_blogs_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;
    use schema::tags::dsl::tags;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let _connection = establish_connection();
    let _tag_id: i32 = *_id;
    let _tags = tags
        .filter(schema::tags::id.eq(_tag_id))
        .load::<Tag>(&_connection)
        .expect("E");
    let _tag = _tags.into_iter().nth(0).unwrap();

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Статьи тега ".to_string() + &_tag.name,
            "вебсервисы.рф: Статьи тега ".to_string() + &_tag.name,
            "/tag_blogs/".to_string() + &_tag.id.to_string() + &"/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
        ).await
    }
    else {
        use crate::schema::tags_items::dsl::tags_items;
        use crate::models::Blog;
        use crate::utils::get_page;

        let page = get_page(&req);

        let _tag_items = tags_items
            .filter(schema::tags_items::tag_id.eq(&_tag_id))
            .select(schema::tags_items::blog_id)
            .load::<i32>(&_connection)
            .expect("E");

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            let (_blogs, next_page_number) = Blog::get_blogs_list_for_ids(&_request_user, page, 20, &_tag_items);
            let blog_count = _blogs.len();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/tags/tag_blogs.stpl")]
                struct Template {
                    request_user:     User,
                    tag:              Tag,
                    blogs_list:       Vec<Blog>,
                    blogs_count:      usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                }
                let body = Template {
                    request_user:     _request_user,
                    tag:              _tag,
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
                    tag:              Tag,
                    blogs_list:       Vec<Blog>,
                    blogs_count:      usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                }
                let body = Template {
                    tag:              _tag,
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
            let (_blogs, next_page_number) = Blog::get_publish_blogs_list_for_ids(page.into(), 20, &_tag_items);
            let blog_count = _blogs.len();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/tags/anon_tag_blogs.stpl")]
                struct Template {
                    tag:              Tag,
                    blogs_list:       Vec<Blog>,
                    blogs_count:      usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                }
                let body = Template {
                    tag:              _tag,
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
                    is_ajax:          i32,
                }
                let body = Template {
                    tag:              _tag,
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
}

pub async fn tag_services_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use schema::tags::dsl::tags;
    use crate::utils::get_device_and_ajax;

    let _connection = establish_connection();
    let _tag_id: i32 = *_id;
    let _tags = tags
        .filter(schema::tags::id.eq(_tag_id))
        .load::<Tag>(&_connection)
        .expect("E");
    let _tag = _tags.into_iter().nth(0).unwrap();
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Услуги тега ".to_string() + &_tag.name,
            "вебсервисы.рф: Услуги тега ".to_string() + &_tag.name,
            "/tag_services/".to_string() + &_tag.id.to_string() + &"/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
        ).await
    }
    else {
        use crate::schema::tags_items::dsl::tags_items;
        use crate::models::Service;
        use crate::utils::get_page;

        let page = get_page(&req);
        let _tag_items = tags_items
            .filter(schema::tags_items::tag_id.eq(&_tag_id))
            .select(schema::tags_items::service_id)
            .load::<i32>(&_connection)
            .expect("E");

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            let (_services, next_page_number) = Service::get_services_list_for_ids(&_request_user, page, 20, &_tag_items);
            let service_count = _services.len();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/tags/tag_services.stpl")]
                struct Template {
                    request_user:     User,
                    tag:              Tag,
                    services_list:    Vec<Service>,
                    services_count:   usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                }
                let body = Template {
                    request_user:     _request_user,
                    tag:              _tag,
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
                    tag:              Tag,
                    services_list:    Vec<Service>,
                    services_count:   usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                }
                let body = Template {
                    tag:              _tag,
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
            let (_services, next_page_number) = Service::get_publish_services_list_for_ids(page, 20, &_tag_items);
            let service_count = _services.len();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/tags/anon_tag_services.stpl")]
                struct Template {
                    tag:              Tag,
                    services_list:    Vec<Service>,
                    services_count:   usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                }
                let body = Template {
                    tag:              _tag,
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
                    is_ajax:          i32,
                }
                let body = Template {
                    tag:              _tag,
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
}

pub async fn tag_stores_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use schema::tags::dsl::tags;
    use crate::utils::get_device_and_ajax;

    let _connection = establish_connection();
    let _tag_id: i32 = *_id;
    let _tags = tags
        .filter(schema::tags::id.eq(_tag_id))
        .load::<Tag>(&_connection)
        .expect("E");
    let _tag = _tags.into_iter().nth(0).unwrap();
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Товары тега ".to_string() + &_tag.name,
            "вебсервисы.рф: Товары тега ".to_string() + &_tag.name,
            "/tag_stores/".to_string() + &_tag.id.to_string() + &"/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
        ).await
    }
    else {
        use crate::schema::tags_items::dsl::tags_items;
        use crate::models::Store;
        use crate::utils::get_page;

        let page = get_page(&req);

        let _tag_items = tags_items
            .filter(schema::tags_items::tag_id.eq(&_tag_id))
            .select(schema::tags_items::store_id)
            .load::<i32>(&_connection)
            .expect("E");

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            let (_stores, next_page_number) = Store::get_stores_list_for_ids(&_request_user, page, 20, &_tag_items);
            let stores_count = _stores.len();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/tags/tag_stores.stpl")]
                struct Template {
                    request_user:     User,
                    tag:              Tag,
                    stores_list:      Vec<Store>,
                    stores_count:     usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                }
                let body = Template {
                    request_user:     _request_user,
                    tag:              _tag,
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
                    tag:              Tag,
                    stores_list:      Vec<Store>,
                    stores_count:     usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                }
                let body = Template {
                    tag:              _tag,
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
            let (_stores, next_page_number) = Store::get_publish_stores_list_for_ids(page, 20, &_tag_items);
            let stores_count = _stores.len();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/tags/anon_tag_stores.stpl")]
                struct Template {
                    tag:              Tag,
                    stores_list:      Vec<Store>,
                    stores_count:     usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                }
                let body = Template {
                    tag:              _tag,
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
                    is_ajax:          i32,
                }
                let body = Template {
                    tag:              _tag,
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
}

pub async fn tag_wikis_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use schema::tags::dsl::tags;
    use crate::utils::get_device_and_ajax;

    let _connection = establish_connection();
    let _tag_id: i32 = *_id;
    let _tags = tags
        .filter(schema::tags::id.eq(_tag_id))
        .load::<Tag>(&_connection)
        .expect("E");
    let _tag = _tags.into_iter().nth(0).unwrap();
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Статьи тега ".to_string() + &_tag.name,
            "вебсервисы.рф: Статьи тега ".to_string() + &_tag.name,
            "/tag_wikis/".to_string() + &_tag.id.to_string() + &"/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
        ).await
    }
    else {
        use crate::schema::tags_items::dsl::tags_items;
        use crate::models::Wiki;
        use crate::utils::get_page;

        let page = get_page(&req);

        let _tag_items = tags_items
            .filter(schema::tags_items::tag_id.eq(&_tag_id))
            .select(schema::tags_items::wiki_id)
            .load::<i32>(&_connection)
            .expect("E");

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            let (_wikis, next_page_number) = Wiki::get_wikis_list_for_ids(&_request_user, page, 20, &_tag_items);
            let wikis_count = _wikis.len();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/tags/tag_wikis.stpl")]
                struct Template {
                    request_user:     User,
                    tag:              Tag,
                    wikis_list:       Vec<Wiki>,
                    wikis_count:      usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                }
                let body = Template {
                    request_user:     _request_user,
                    tag:              _tag,
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
                    is_ajax:          i32,
                }
                let body = Template {
                    tag:              _tag,
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
            let (_wikis, next_page_number) = Wiki::get_publish_wikis_list_for_ids(page, 20, &_tag_items);
            let wikis_count = _wikis.len();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/tags/anon_tag_wikis.stpl")]
                struct Template {
                    tag:              Tag,
                    wikis_list:       Vec<Wiki>,
                    wikis_count:      usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                }
                let body = Template {
                    tag:              _tag,
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
                    is_ajax:          i32,
                }
                let body = Template {
                    tag:              _tag,
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
}

pub async fn tag_works_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use schema::tags::dsl::tags;
    use crate::utils::get_device_and_ajax;

    let _connection = establish_connection();
    let _tag_id: i32 = *_id;
    let _tags = tags
        .filter(schema::tags::id.eq(_tag_id))
        .load::<Tag>(&_connection)
        .expect("E");
    let _tag = _tags.into_iter().nth(0).unwrap();
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Работы тега ".to_string() + &_tag.name,
            "вебсервисы.рф: Работы тега ".to_string() + &_tag.name,
            "/tag_works/".to_string() + &_tag.id.to_string() + &"/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
        ).await
    }
    else {
        use crate::schema::tags_items::dsl::tags_items;
        use crate::models::Work;
        use crate::utils::get_page;

        let page = get_page(&req);

        let _tag_items = tags_items
            .filter(schema::tags_items::tag_id.eq(&_tag_id))
            .select(schema::tags_items::work_id)
            .load::<i32>(&_connection)
            .expect("E");

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            let (_works, next_page_number) = Work::get_works_list_for_ids(&_request_user, page, 20, &_tag_items);
            let works_count = _works.len();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/tags/tag_works.stpl")]
                struct Template {
                    request_user:     User,
                    tag:              Tag,
                    works_list:       Vec<Work>,
                    works_count:      usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                }
                let body = Template {
                    request_user:     _request_user,
                    tag:              _tag,
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
                    tag:              Tag,
                    works_list:       Vec<Work>,
                    works_count:      usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                }
                let body = Template {
                    tag:              _tag,
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
            let (_works, next_page_number) = Work::get_publish_works_list_for_ids(page, 20, &_tag_items);
            let works_count = _works.len();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/tags/anon_tag_works.stpl")]
                struct Template {
                    tag:              Tag,
                    works_list:       Vec<Work>,
                    works_count:      usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                }
                let body = Template {
                    tag:              _tag,
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
                    is_ajax:          i32,
                }
                let body = Template {
                    tag:              _tag,
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
}

pub async fn tags_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Ключевые слова".to_string(),
            "вебсервисы.рф: Ключевые слова".to_string(),
            "/tags/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
        ).await
    }
    else {
        use crate::utils::get_page;
        use crate::schema::stat_tags::dsl::stat_tags;
        use crate::models::StatTag;

        let page = get_page(&req);

        let _connection = establish_connection();
        let (all_tags, next_page_number) = Tag::get_tags_list(page, 20);
        let tags_count = all_tags.len();

        let _stat: StatTag;
        let _stats = stat_tags
            .limit(1)
            .load::<StatTag>(&_connection)
            .expect("E");
        if _stats.len() > 0 {
            _stat = _stats.into_iter().nth(0).unwrap();
        }
        else {
            use crate::models::NewStatTag;
            let form = NewStatTag {
                view: 0,
                height: 0.0,
                seconds: 0,
            };
            _stat = diesel::insert_into(schema::stat_tags::table)
                .values(&form)
                .get_result::<StatTag>(&_connection)
                .expect("Error.");
        }

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/tags/tags.stpl")]
                struct Template {
                    request_user:     User,
                    all_tags:         Vec<Tag>,
                    tags_count:       usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                    stat:             StatTag,
                }
                let body = Template {
                    request_user:     _request_user,
                    all_tags:         all_tags,
                    tags_count:       tags_count,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
                    stat:             _stat,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/tags/tags.stpl")]
                struct Template {
                    all_tags:         Vec<Tag>,
                    tags_count:       usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                    stat:             StatTag,
                }
                let body = Template {
                    all_tags:         all_tags,
                    tags_count:       tags_count,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
                    stat:             _stat,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/tags/anon_tags.stpl")]
                struct Template {
                    all_tags:         Vec<Tag>,
                    tags_count:       usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                    stat:             StatTag,
                }
                let body = Template {
                    all_tags:         all_tags,
                    tags_count:       tags_count,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
                    stat:             _stat,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/tags/anon_tags.stpl")]
                struct Template {
                    all_tags:         Vec<Tag>,
                    tags_count:       usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                    stat:             StatTag,
                }
                let body = Template {
                    all_tags:         all_tags,
                    tags_count:       tags_count,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
                    stat:             _stat,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn edit_tag_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;
    use schema::tags::dsl::tags;

    let _tag_id: i32 = *_id;
    let _connection = establish_connection();
    let _tags = tags
        .filter(schema::tags::id.eq(&_tag_id))
        .load::<Tag>(&_connection)
        .expect("E");

    let _tag = _tags.into_iter().nth(0).unwrap();
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Изменение тега ".to_string() + &_tag.name,
            "вебсервисы.рф: Изменение тега ".to_string() + &_tag.name,
            "/edit_tag/".to_string() + &_tag.id.to_string() + &"/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
        ).await
    }
    else if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/tags/edit_tag.stpl")]
                struct Template {
                    request_user: User,
                    tag:          Tag,
                    is_ajax:      i32,
                }
                let body = Template {
                    request_user: _request_user,
                    tag:          _tag,
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
                    tag:          Tag,
                    is_ajax:      i32,
                }
                let body = Template {
                    tag:          _tag,
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
    else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied."))
    }
}

pub async fn edit_tag(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use crate::models::EditTag;
            use crate::schema::tags::dsl::tags;
            use crate::utils::category_form;

            let _connection = establish_connection();
            let _tag_id : i32 = *_id;
            let _tag = tags
                .filter(schema::tags::id.eq(_tag_id))
                .load::<Tag>(&_connection)
                .expect("E");

            let form = category_form(payload.borrow_mut(), _request_user.id).await;
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

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use crate::schema::tags::dsl::tags;
            use crate::schema::tags_items::dsl::tags_items;

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
