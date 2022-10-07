use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    web::block,
    error::InternalError,
    http::StatusCode,
};

use actix_session::Session;
use crate::utils::{
    establish_connection,
    is_signed_in,
    get_request_user_data,
    get_first_load_page,
};

use sailfish::TemplateOnce;
use crate::models::User;


pub fn search_routes(config: &mut web::ServiceConfig) {
    config.route("/search/", web::get().to(empty_search_page));
    config.route("/search/{q}/", web::get().to(search_page));
    config.route("/search_blogs/{q}/", web::get().to(search_blogs_page));
    config.route("/search_services/{q}/", web::get().to(search_services_page));
    config.route("/search_stores/{q}/", web::get().to(search_stores_page));
    config.route("/search_wikis/{q}/", web::get().to(search_wikis_page));
    config.route("/search_works/{q}/", web::get().to(search_works_page));
    config.route("/search_help/{q}/", web::get().to(search_help_page));
}


pub async fn empty_search_page(req: HttpRequest, session: Session) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    // первая отрисовка страницы - организуем скрытие информации
    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Общий поиск".to_string(),
            "вебсервисы.рф: Общий поиск".to_string(),
            "/search/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
        ).await
    }
    else if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/search/empty_search.stpl")]
            struct Template {
                request_user: User,
                is_ajax:      i32,
            }
            let body = Template {
                request_user: _request_user,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/search/empty_search.stpl")]
            struct Template {
                is_ajax:      i32,
            }
            let body = Template {
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/search/anon_empty_search.stpl")]
            struct Template {
                is_ajax: i32,
            }
            let body = Template {
                is_ajax: is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/search/anon_empty_search.stpl")]
            struct Template {
                is_ajax: i32,
            }
            let body = Template {
                is_ajax: is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn search_page(session: Session, req: HttpRequest, q: web::Path<String>) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;


    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let _q = q.clone();
    let _q_standalone = "%".to_owned() + &_q + "%";

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Общий поиск по фрагменту ".to_string() + &q,
            "вебсервисы.рф: Общий поиск по фрагменту ".to_string() + &q,
            "/search/".to_string() + &q + &"/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
        ).await
    }
    else {
        use crate::models::{Item, Blog, Service, Store, Wiki, Work};

        let _connection = establish_connection();

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            let is_admin = _request_user.is_superuser();

            let work_list = Item::search_works(&_q_standalone, 3, 0, is_admin);
            let service_list = Item::search_services(&_q_standalone, 3, 0, is_admin);
            let wiki_list = Item::search_wikis(&_q_standalone, 3, 0, is_admin);
            let blog_list = Item::search_blogs(&_q_standalone, 3, 0, is_admin);
            let store_list = Item::search_stores(&_q_standalone, 3, 0, is_admin);

            let blog_count = blog_list.len();
            let service_count = service_list.len();
            let store_count = store_list.len();
            let wiki_count = wiki_list.len();
            let work_count = work_list.len();

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
                    is_ajax:        i32,
                    q:              String,
                }
                let body = Template {
                    request_user:   _request_user,
                    works_list:     work_list,
                    services_list:  service_list,
                    wikis_list:     wiki_list,
                    blogs_list:     blog_list,
                    stores_list:    store_list,

                    works_count:    work_count,
                    services_count: service_count,
                    wikis_count:    wiki_count,
                    blogs_count:    blog_count,
                    stores_count:   store_count,
                    is_ajax:        is_ajax,
                    q:              _q,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/search/all.stpl")]
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
                    is_ajax:        i32,
                    q:              String,
                }
                let body = Template {
                    works_list:     work_list,
                    services_list:  service_list,
                    wikis_list:     wiki_list,
                    blogs_list:     blog_list,
                    stores_list:    store_list,

                    works_count:    work_count,
                    services_count: service_count,
                    wikis_count:    wiki_count,
                    blogs_count:    blog_count,
                    stores_count:   store_count,
                    is_ajax:        is_ajax,
                    q:              _q,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            let work_list = Item::search_works(&_q_standalone, 3, 0, false);
            let service_list = Item::search_services(&_q_standalone, 3, 0, false);
            let wiki_list = Item::search_wikis(&_q_standalone, 3, 0, false);
            let blog_list = Item::search_blogs(&_q_standalone, 3, 0, false);
            let store_list = Item::search_stores(&_q_standalone, 3, 0, false);

            let blog_count = blog_list.len();
            let service_count = service_list.len();
            let store_count = store_list.len();
            let wiki_count = wiki_list.len();
            let work_count = work_list.len();

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
                    is_ajax:        i32,
                    q:              String,
                }
                let body = Template {
                    works_list:     work_list,
                    services_list:  service_list,
                    wikis_list:     wiki_list,
                    blogs_list:     blog_list,
                    stores_list:    store_list,

                    works_count:    work_count,
                    services_count: service_count,
                    wikis_count:    wiki_count,
                    blogs_count:    blog_count,
                    stores_count:   store_count,
                    is_ajax:        is_ajax,
                    q:              _q,
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
                    is_ajax:        i32,
                    q:              String,
                }
                let body = Template {
                    works_list:     work_list,
                    services_list:  service_list,
                    wikis_list:     wiki_list,
                    blogs_list:     blog_list,
                    stores_list:    store_list,

                    works_count:    work_count,
                    services_count: service_count,
                    wikis_count:    wiki_count,
                    blogs_count:    blog_count,
                    stores_count:   store_count,
                    is_ajax:        is_ajax,
                    q:              _q,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn search_blogs_page(session: Session, req: HttpRequest, q: web::Path<String>) -> actix_web::Result<HttpResponse> {
    use crate::utils::{get_device_and_ajax, get_page};

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let _q = q.clone();

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Поиск статей по фрагменту ".to_string() + &q,
            "вебсервисы.рф: Поиск статей по фрагменту ".to_string() + &q,
            "/search_blogs/".to_string() + &q + &"/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
        ).await
    }
    else {
        use crate::models::{Item, Blog};

        let page = get_page(&req);
        let _connection = establish_connection();

        let _q_standalone = "%".to_owned() + &_q + "%";

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

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            let is_admin = _request_user.is_superuser();
            let blog_list = Item::search_blogs(&_q_standalone, 20, offset.into(), is_admin);

            if Item::search_blogs(&_q_standalone, 1, next_item.into(), is_admin).len() > 0 {
                next_page_number = page + 1;
            }

            let blogs_count = blog_list.len();
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/blogs.stpl")]
                struct Template {
                    request_user:     User,
                    blogs_list:       Vec<Blog>,
                    blogs_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                }
                let body = Template {
                    request_user:     _request_user,
                    blogs_list:       blog_list,
                    blogs_count:      blogs_count,
                    is_ajax:          is_ajax,
                    q:                _q,
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
                    blogs_list:       Vec<Blog>,
                    blogs_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                }
                let body = Template {
                    blogs_list:       blog_list,
                    blogs_count:      blogs_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            let is_admin = false;
            let blog_list = Item::search_blogs(&_q_standalone, 20, offset.into(), is_admin);

            if Item::search_blogs(&_q_standalone, 1, next_item.into(), is_admin).len() > 0 {
                next_page_number = page + 1;
            }

            let blogs_count = blog_list.len();
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/anon_blogs.stpl")]
                struct Template {
                    blogs_list:       Vec<Blog>,
                    blogs_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                }
                let body = Template {
                    blogs_list:       blog_list,
                    blogs_count:      blogs_count,
                    is_ajax:          is_ajax,
                    q:                _q,
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
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                }
                let body = Template {
                    blogs_list:       blog_list,
                    blogs_count:      blogs_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn search_services_page(session: Session, req: HttpRequest, q: web::Path<String>) -> actix_web::Result<HttpResponse> {
    use crate::utils::{get_device_and_ajax, get_page};

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let _q = q.clone();

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Поиск услуг по фрагменту ".to_string() + &q,
            "вебсервисы.рф: Поиск услуг по фрагменту ".to_string() + &q,
            "/search_services/".to_string() + &q + &"/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
        ).await
    }
    else {
        use crate::models::{Item, Service};

        let page = get_page(&req);
        let _connection = establish_connection();

        let _q_standalone = "%".to_owned() + &_q + "%";

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

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            let is_admin = _request_user.is_superuser();
            let services_list = Item::search_services(&_q_standalone, 20, offset.into(), is_admin);

            if Item::search_services(&_q_standalone, 1, next_item.into(), is_admin).len() > 0 {
                next_page_number = page + 1;
            }
            let services_count = services_list.len();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/services.stpl")]
                struct Template {
                    request_user:     User,
                    services_list:    Vec<Service>,
                    services_count:   usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                }
                let body = Template {
                    request_user:     _request_user,
                    services_list:    services_list,
                    services_count:   services_count,
                    is_ajax:          is_ajax,
                    q:                _q,
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
                    services_list:    Vec<Service>,
                    services_count:   usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                }
                let body = Template {
                    services_list:    services_list,
                    services_count:   services_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            let is_admin = false;
            let services_list = Item::search_services(&_q_standalone, 20, offset.into(), is_admin);

            if Item::search_services(&_q_standalone, 1, next_item.into(), is_admin).len() > 0 {
                next_page_number = page + 1;
            }
            let services_count = services_list.len();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/anon_services.stpl")]
                struct Template {
                    services_list:    Vec<Service>,
                    services_count:   usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                }
                let body = Template {
                    services_list:    services_list,
                    services_count:   services_count,
                    is_ajax:          is_ajax,
                    q:                _q,
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
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                }
                let body = Template {
                    services_list:    services_list,
                    services_count:   services_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn search_stores_page(session: Session, req: HttpRequest, q: web::Path<String>) -> actix_web::Result<HttpResponse> {
    use crate::utils::{get_device_and_ajax, get_page};

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let _q = q.clone();

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Поиск товаров по фрагменту ".to_string() + &q,
            "вебсервисы.рф: Поиск товаров по фрагменту ".to_string() + &q,
            "/search_stores/".to_string() + &q + &"/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
        ).await
    }
    else {
        use crate::models::{Item, Store};

        let page = get_page(&req);

        let _connection = establish_connection();
        let _q_standalone = "%".to_owned() + &_q + "%";

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

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            let is_admin = _request_user.is_superuser();
            let store_list = Item::search_stores(&_q_standalone, 20, offset.into(), is_admin);

            if Item::search_stores(&_q_standalone, 1, next_item.into(), is_admin).len() > 0 {
                next_page_number = page + 1;
            }

            let stores_count = store_list.len();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/stores.stpl")]
                struct Template {
                    request_user:     User,
                    stores_list:      Vec<Store>,
                    stores_count:     usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                }
                let body = Template {
                    request_user:     _request_user,
                    stores_list:       store_list,
                    stores_count:      stores_count,
                    is_ajax:          is_ajax,
                    q:                _q,
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
                    stores_list:      Vec<Store>,
                    stores_count:     usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                }
                let body = Template {
                    stores_list:      store_list,
                    stores_count:     stores_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            let is_admin = false;
            let store_list = Item::search_stores(&_q_standalone, 20, offset.into(), is_admin);

            if Item::search_stores(&_q_standalone, 1, next_item.into(), is_admin).len() > 0 {
                next_page_number = page + 1;
            }

            let stores_count = store_list.len();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/anon_stores.stpl")]
                struct Template {
                    stores_list:      Vec<Store>,
                    stores_count:     usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                }
                let body = Template {
                    stores_list:      store_list,
                    stores_count:     stores_count,
                    is_ajax:          is_ajax,
                    q:                _q,
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
                    stores_list:      Vec<Store>,
                    stores_count:     usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                }
                let body = Template {
                    stores_list:      store_list,
                    stores_count:     stores_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn search_wikis_page(session: Session, req: HttpRequest, q: web::Path<String>) -> actix_web::Result<HttpResponse> {
    use crate::utils::{get_device_and_ajax, get_page};

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let _q = q.clone();

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Поиск статей по фрагменту ".to_string() + &q,
            "вебсервисы.рф: Поиск статей по фрагменту ".to_string() + &q,
            "/search_wikis/".to_string() + &q + &"/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
        ).await
    }
    else {
        use crate::models::{Item, Wiki};

        let page = get_page(&req);
        let _connection = establish_connection();
        let _q_standalone = "%".to_owned() + &_q + "%";

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

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            let is_admin = _request_user.is_superuser();
            let wiki_list = Item::search_wikis(&_q_standalone, 20, offset.into(), is_admin);

            if Item::search_wikis(&_q_standalone, 1, next_item.into(), is_admin).len() > 0 {
                next_page_number = page + 1;
            }

            let wikis_count = wiki_list.len();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/wikis.stpl")]
                struct Template {
                    request_user:     User,
                    wikis_list:       Vec<Wiki>,
                    wikis_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                }
                let body = Template {
                    request_user:     _request_user,
                    wikis_list:       wiki_list,
                    wikis_count:      wikis_count,
                    is_ajax:          is_ajax,
                    q:                _q,
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
                    wikis_list:       Vec<Wiki>,
                    wikis_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                }
                let body = Template {
                    wikis_list:       wiki_list,
                    wikis_count:      wikis_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            let is_admin = false;
            let wiki_list = Item::search_wikis(&_q_standalone, 20, offset.into(), is_admin);

            if Item::search_wikis(&_q_standalone, 1, next_item.into(), is_admin).len() > 0 {
                next_page_number = page + 1;
            }
            let wikis_count = wiki_list.len();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/anon_wikis.stpl")]
                struct Template {
                    wikis_list:       Vec<Wiki>,
                    wikis_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                }
                let body = Template {
                    wikis_list:       wiki_list,
                    wikis_count:      wikis_count,
                    is_ajax:          is_ajax,
                    q:                _q,
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
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                }
                let body = Template {
                    wikis_list:       wiki_list,
                    wikis_count:      wikis_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn search_works_page(session: Session, req: HttpRequest, q: web::Path<String>) -> actix_web::Result<HttpResponse> {
    use crate::utils::{get_device_and_ajax, get_page};

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let _q = q.clone();

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Поиск работ по фрагменту ".to_string() + &q,
            "вебсервисы.рф: Поиск работ по фрагменту ".to_string() + &q,
            "/search_works/".to_string() + &q + &"/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
        ).await
    }
    else {
        use crate::models::{Item, Work};

        let page = get_page(&req);
        let _connection = establish_connection();
        let _q_standalone = "%".to_owned() + &_q + "%";

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

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);

            let is_admin = _request_user.is_superuser();
            let work_list = Item::search_works(&_q_standalone, 20, offset.into(), is_admin);

            if Item::search_works(&_q_standalone, 1, next_item.into(), is_admin).len() > 0 {
                next_page_number = page + 1;
            }

            let works_count = work_list.len();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/works.stpl")]
                struct Template {
                    request_user:     User,
                    works_list:       Vec<Work>,
                    works_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                }
                let body = Template {
                    request_user:     _request_user,
                    works_list:       work_list,
                    works_count:      works_count,
                    is_ajax:          is_ajax,
                    q:                _q,
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
                    works_list:       Vec<Work>,
                    works_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                }
                let body = Template {
                    works_list:       work_list,
                    works_count:      works_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            let is_admin = false;
            let work_list = Item::search_works(&_q_standalone, 20, offset.into(), is_admin);

            if Item::search_works(&_q_standalone, 1, next_item.into(), is_admin).len() > 0 {
                next_page_number = page + 1;
            }
            let works_count = work_list.len();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/anon_works.stpl")]
                struct Template {
                    works_list:       Vec<Work>,
                    works_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                }
                let body = Template {
                    works_list:       work_list,
                    works_count:      works_count,
                    is_ajax:          is_ajax,
                    q:                _q,
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
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                }
                let body = Template {
                    works_list:       work_list,
                    works_count:      works_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn search_help_page(session: Session, req: HttpRequest, q: web::Path<String>) -> actix_web::Result<HttpResponse> {
    use crate::utils::{get_device_and_ajax, get_page};

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let _q = q.clone();

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Поиск по фрагменту ".to_string() + &q,
            "вебсервисы.рф: Поиск по фрагменту ".to_string() + &q,
            "/search_help/".to_string() + &q + &"/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
        ).await
    }
    else {
        use crate::models::{Item, Help};

        let page = get_page(&req);
        let _connection = establish_connection();
        let _q_standalone = "%".to_owned() + &_q + "%";

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

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            let is_admin = _request_user.is_superuser();
            let _items = Item::search_helps(&_q_standalone, 20, offset.into(), is_admin);
            let items_count = _items.len();

            if Item::search_helps(&_q_standalone, 1, next_item.into(), is_admin).len() > 0 {
                next_page_number = page + 1;
            }
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/help.stpl")]
                struct Template {
                    request_user:     User,
                    items_list:       Vec<Help>,
                    items_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                }
                let body = Template {
                    request_user:     _request_user,
                    items_list:       _items,
                    items_count:      items_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/search/help.stpl")]
                struct Template {
                    items_list:       Vec<Help>,
                    items_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                }
                let body = Template {
                    items_list:       _items,
                    items_count:      items_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            let is_admin = false;
            let _items = Item::search_helps(&_q_standalone, 20, offset.into(), is_admin);
            let items_count = _items.len();
            if Item::search_helps(&_q_standalone, 1, next_item.into(), is_admin).len() > 0 {
                next_page_number = page + 1;
            }
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/anon_help.stpl")]
                struct Template {
                    items_list:       Vec<Help>,
                    items_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                }
                let body = Template {
                    items_list:       _items,
                    items_count:      items_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/search/anon_help.stpl")]
                struct Template {
                    items_list:       Vec<Help>,
                    items_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                }
                let body = Template {
                    items_list:       _items,
                    items_count:      items_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}
