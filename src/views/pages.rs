use actix::Addr;
use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
    web::{block, Data},
    Result,
};
use crate::schema;
use crate::models::{
    User,
    Item,
    Categories,
    Tag,
    StatPage,
    Cat,
};
use crate::utils::{
    establish_connection,
    get_device_and_ajax,
    get_request_user_data,
    is_signed_in,
    get_first_load_page,
    IndexResponse,
    AppState,

};
use crate::diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use actix_session::Session;
use sailfish::TemplateOnce;
use actix_web::dev::ConnectionInfo;
use serde_json::to_value;
use crate::websocket::Server;


pub fn pages_routes(config: &mut web::ServiceConfig) {
    config.route("/test/", web::get().to(test_page));
    config.route("/test/", web::get().to(test_page));
    config.route("/", web::get().to(index_page));
    config.route("/info/", web::get().to(info_page));
    config.route("/history/", web::get().to(history_page));
    config.route("/feedback_list/", web::get().to(feedback_list_page));
    config.route("/serve_list/", web::get().to(serve_list_page));
    config.route("/cookie_users_list/", web::get().to(cookie_users_list_page));

    config.route("/load_tech_category/{id}/", web::get().to(get_tech_category_page));
    config.route("/load_serve_category/{id}/", web::get().to(get_serve_category_page));
    config.route("/load_serve/{id}/", web::get().to(get_serve_page));
    config.route("/load_feedback/", web::get().to(get_feedback_page));
    config.route("/load_user_history/{id}/", web::get().to(get_user_history_page));
    config.route("/load_tech_objects/{id}/", web::get().to(get_tech_objects_page));
    config.route("/unical_object_form/{id}/", web::get().to(unical_object_form_page));

    config.route("/create_category/", web::get().to(create_category_page));
    config.route("/edit_category/{id}/", web::get().to(edit_category_page));
    config.route("/create_item/", web::get().to(create_item_page));
    config.route("/edit_item/{id}/", web::get().to(edit_item_page));
    config.route("/edit_content_item/{id}/", web::get().to(edit_content_item_page));

    config.route("/edit_file/{id}/", web::get().to(edit_file_page));
    config.route("/image/{id}/", web::get().to(image_page));
}


pub async fn not_found(req: HttpRequest, session: Session) -> actix_web::Result<HttpResponse> {
    use cookie::Cookie; 
    
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let cookie = Cookie::new("name", "1");
    let cookie_nv = cookie.name_value();
    let cookie_dom = cookie.domain();
    let secure_cookie = Cookie::build("secure_name", "1")
            .domain("https://вебсервисы.рф")
            .path("/")
            .secure(true)
            .http_only(true)
            .finish();

    for header in req.headers().into_iter() {
        if header.0 == "cookie" {
            let str_cookie = header.1.to_str().unwrap();
            let _cookie: Vec<&str> = str_cookie.split(";").collect();
            for c in _cookie.iter() {
                let split_c: Vec<&str> = c.split("=").collect();
                println!("name {:?}", split_c[0].trim());
                println!("value {:?}", split_c[1]);
            } 
        }
    };

    // первая отрисовка страницы - организуем скрытие информации
    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Страница не найдена".to_string(),
            "вебсервисы.рф: Страница не найдена".to_string(),
            "/not_found/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
        ).await
    }
    else {
        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/pages/404.stpl")]
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
                #[template(path = "mobile/pages/404.stpl")]
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
        else {
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/pages/anon_404.stpl")]
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
                #[template(path = "mobile/pages/anon_404.stpl")]
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
}

pub async fn test_page(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);
    let ms = state.messages.lock().unwrap();

    Ok(web::Json(IndexResponse {
        server_id: state.server_id,
        request_count,
        messages: ms.clone(),
    }))
}

pub async fn index_page (
    req: HttpRequest,
    session: Session,
    websocket_srv: Data<Addr<Server>>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    //use crate::utils::check_auth;


    if is_ajax == 0 {
        //let auth = check_auth(&session);
        get_first_load_page (
            &session,
            is_desctop,
            "Главная страница ".to_string(),
            "вебсервисы - Комплексное, экспертное создание и развитие высоконагруженных веб-ресурсов".to_string(),
            "/".to_string(),
            "/static/images/dark/store.jpg".to_string()
        ).await
    }
    else {
        use crate::schema::stat_pages::dsl::stat_pages;
        use crate::models::{Blog, Service, Store, Wiki, Work};
        use crate::websocket::MessageToClient;

        let _connection = establish_connection();
        let _stat: StatPage;

        let _stats = stat_pages
            .filter(schema::stat_pages::types.eq(1))
            .first::<StatPage>(&_connection);
        if _stats.is_ok() {
            _stat = _stats.expect("E");
            diesel::update(&_stat)
                .set(schema::stat_pages::now_u.eq(_stat.now_u + 1))
                .get_result::<StatPage>(&_connection)
                .expect("Error.");
        }
        else {
            use crate::models::NewStatPage;
            let form = NewStatPage {
                types:   1,
                view:    0,
                height:  0.0,
                seconds: 0,
                now_u:   1,
            };
            _stat = diesel::insert_into(schema::stat_pages::table)
                .values(&form)
                .get_result::<StatPage>(&_connection)
                .expect("Error.");

        }
        //if let Ok(res) = to_value(_stat.now_u.to_string()) {
        //    let msg = MessageToClient::new("page_view", _stat.types.into(), res);
        //    websocket_srv.do_send(msg);
        //}

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            let is_admin = _request_user.is_superuser();
            //User::create_superuser(_request_user.id);
            let _last_works = Item::get_works(3, 0, is_admin);
            let _last_services = Item::get_services(3, 0, is_admin);
            let _last_wikis = Item::get_wikis(3, 0, is_admin);
            let _last_blogs = Item::get_blogs(3, 0, is_admin);
            let _last_stores = Item::get_stores(3, 0, is_admin);

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/main/mainpage.stpl")]
                struct Template {
                    request_user:  User,
                    last_works:    Vec<Work>,
                    last_services: Vec<Service>,
                    last_wikis:    Vec<Wiki>,
                    last_blogs:    Vec<Blog>,
                    last_stores:   Vec<Store>,
                    is_ajax:       i32,
                    stat:          StatPage,
                }
                let body = Template {
                    request_user:  _request_user,
                    last_works:    _last_works,
                    last_services: _last_services,
                    last_wikis:    _last_wikis,
                    last_blogs:    _last_blogs,
                    last_stores:   _last_stores,
                    is_ajax:       is_ajax,
                    stat:          _stat,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/main/mainpage.stpl")]
                struct Template {
                    request_user:  User,
                    last_works:    Vec<Work>,
                    last_services: Vec<Service>,
                    last_wikis:    Vec<Wiki>,
                    last_blogs:    Vec<Blog>,
                    last_stores:   Vec<Store>,
                    is_ajax:       i32,
                    stat:          StatPage,
                }
                let body = Template {
                    request_user:  _request_user,
                    last_works:    _last_works,
                    last_services: _last_services,
                    last_wikis:    _last_wikis,
                    last_blogs:    _last_blogs,
                    last_stores:   _last_stores,
                    is_ajax:       is_ajax,
                    stat:          _stat,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            let _last_works = Item::get_works(3, 0, false);
            let _last_services = Item::get_services(3, 0, false);
            let _last_wikis = Item::get_wikis(3, 0, false);
            let _last_blogs = Item::get_blogs(3, 0, false);
            let _last_stores = Item::get_stores(3, 0, false);

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/main/anon_mainpage.stpl")]
                struct Template {
                    last_works:    Vec<Work>,
                    last_services: Vec<Service>,
                    last_wikis:    Vec<Wiki>,
                    last_blogs:    Vec<Blog>,
                    last_stores:   Vec<Store>,
                    is_ajax:       i32,
                    stat:          StatPage,
                }
                let body = Template {
                    last_works:    _last_works,
                    last_services: _last_services,
                    last_wikis:    _last_wikis,
                    last_blogs:    _last_blogs,
                    last_stores:   _last_stores,
                    is_ajax:       is_ajax,
                    stat:          _stat,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/main/anon_mainpage.stpl")]
                struct Template {
                    last_works:    Vec<Work>,
                    last_services: Vec<Service>,
                    last_wikis:    Vec<Wiki>,
                    last_blogs:    Vec<Blog>,
                    last_stores:   Vec<Store>,
                    is_ajax:       i32,
                    stat:          StatPage,
                }
                let body = Template {
                    last_works:    _last_works,
                    last_services: _last_services,
                    last_wikis:    _last_wikis,
                    last_blogs:    _last_blogs,
                    last_stores:   _last_stores,
                    is_ajax:       is_ajax,
                    stat:          _stat,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn info_page(req: HttpRequest, session: Session) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    // первая отрисовка страницы - организуем скрытие информации
    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Информация".to_string(),
            "вебсервисы.рф: Информация о нас, о сайте, контакты, вкладка помощи".to_string(),
            "/info/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
        ).await
    }
    else if is_signed_in(&session) {
        use schema::stat_pages::dsl::stat_pages;

        let _connection = establish_connection();
        let _stat: StatPage;
        let _stats = stat_pages
            .filter(schema::stat_pages::types.eq(10))
            .first::<StatPage>(&_connection);
        if _stats.is_ok() {
            _stat = _stats.expect("E");
        }
        else {
            use crate::models::NewStatPage;
            let form = NewStatPage {
                types:   10,
                view:    0,
                height:  0.0,
                seconds: 0,
                now_u:   0,
            };
            _stat = diesel::insert_into(schema::stat_pages::table)
                .values(&form)
                .get_result::<StatPage>(&_connection)
                .expect("Error.");
        }
        let _help_cats: Vec<Cat>;
        let cats_res = block(move || Categories::get_categories_for_types(6)).await?;
        let _help_cats = match cats_res {
            Ok(_ok) => _ok,
            Err(_error) => Vec::new(),
        };

        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/pages/info.stpl")]
            struct Template {
                request_user: User,
                is_ajax:      i32,
                help_cats:    Vec<Cat>,
                stat:         StatPage,
            }
            let body = Template {
                request_user: _request_user,
                is_ajax:      is_ajax,
                help_cats:    _help_cats,
                stat:         _stat,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/pages/info.stpl")]
            struct Template {
                is_ajax:   i32,
                help_cats: Vec<Cat>,
                stat:      StatPage,
            }
            let body = Template {
                is_ajax:   is_ajax,
                help_cats: _help_cats,
                stat:      _stat,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        use schema::stat_pages::dsl::stat_pages;

        let _connection = establish_connection();
        let _stat: StatPage;
        let _stats = stat_pages
            .filter(schema::stat_pages::types.eq(10))
            .first::<StatPage>(&_connection);
        if _stats.is_ok() {
            _stat = _stats.expect("E");
        }
        else {
            use crate::models::NewStatPage;
            let form = NewStatPage {
                types:   10,
                view:    0,
                height:  0.0,
                seconds: 0,
                now_u:   0,
            };
            _stat = diesel::insert_into(schema::stat_pages::table)
                .values(&form)
                .get_result::<StatPage>(&_connection)
                .expect("Error.");
        }
        let _help_cats: Vec<Cat>;
        let cats_res = block(move || Categories::get_categories_for_types(6)).await?;
        let _help_cats = match cats_res {
            Ok(_ok) => _ok,
            Err(_error) => Vec::new(),
        };

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/pages/anon_info.stpl")]
            struct Template {
                is_ajax:   i32,
                help_cats: Vec<Cat>,
                stat:      StatPage,
            }
            let body = Template {
                is_ajax:   is_ajax,
                help_cats: _help_cats,
                stat:      _stat,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/pages/anon_info.stpl")]
            struct Template {
                help_cats: Vec<Cat>,
                is_ajax:   i32,
                stat:      StatPage,
            }
            let body = Template {
                is_ajax:   is_ajax,
                help_cats: _help_cats,
                stat:      _stat,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn history_page(conn: ConnectionInfo, req: HttpRequest, session: Session) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    // первая отрисовка страницы - организуем скрытие информации
    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "История просмотров".to_string(),
            "вебсервисы.рф: История просмотров пользователя".to_string(),
            "/history/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
        ).await
    }
    else {
        use schema::cookie_users::dsl::cookie_users;
        use crate::models::{CookieUser, CookieStat};
        use crate::utils::{get_page, get_or_create_cookie_user_id};

        let user_id = get_or_create_cookie_user_id(conn, &req).await;
        let _connection = establish_connection();
        let _cookie_user = cookie_users
            .filter(schema::cookie_users::id.eq(&user_id))
            .first::<CookieUser>(&_connection)
            .expect("Error");

            let object_list: Vec<CookieStat>;
            let next_page_number: i32;
            let page = get_page(&req);
            let _res = block(move || CookieStat::get_stat_list(user_id, page, 20)).await?;
            let _dict = match _res {
                Ok(_ok) => {object_list = _ok.0; next_page_number = _ok.1},
                Err(_error) => {object_list = Vec::new(); next_page_number = 0},
            };

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/pages/history.stpl")]
                struct Template {
                    request_user:     User,
                    user:             CookieUser,
                    object_list:      Vec<CookieStat>,
                    is_ajax:          i32,
                    next_page_number: i32,

                }
                let body = Template {
                    request_user:     _request_user,
                    user:             _cookie_user,
                    object_list:      object_list,
                    is_ajax:          is_ajax,
                    next_page_number: next_page_number,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/pages/history.stpl")]
                struct Template {
                    user:             CookieUser,
                    object_list:      Vec<CookieStat>,
                    is_ajax:          i32,
                    next_page_number: i32,
                }
                let body = Template {
                    user:             _cookie_user,
                    object_list:      object_list,
                    is_ajax:          is_ajax,
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
                #[template(path = "desctop/pages/anon_history.stpl")]
                struct Template {
                    user:             CookieUser,
                    object_list:      Vec<CookieStat>,
                    is_ajax:          i32,
                    next_page_number: i32,
                }
                let body = Template {
                    user:             _cookie_user,
                    object_list:      object_list,
                    is_ajax:          is_ajax,
                    next_page_number: next_page_number,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/pages/anon_history.stpl")]
                struct Template {
                    user:             CookieUser,
                    object_list:      Vec<CookieStat>,
                    is_ajax:          i32,
                    next_page_number: i32,
                }
                let body = Template {
                    user:             _cookie_user,
                    object_list:      object_list,
                    is_ajax:          is_ajax,
                    next_page_number: next_page_number,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn feedback_list_page(req: HttpRequest, session: Session) -> actix_web::Result<HttpResponse> {
        if !is_signed_in(&session) {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied"))
        }
        else {
            use crate::schema::feedbacks::dsl::feedbacks;
            use crate::models::Feedback;

            let _connection = establish_connection();
            let _feedbacks = feedbacks
                .load::<Feedback>(&_connection)
                .expect("E");

            let _request_user = get_request_user_data(&session);
            let (is_desctop, is_ajax) = get_device_and_ajax(&req);
            if _request_user.perm < 60 {
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied"))
            }
            else if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/main/feedback_list.stpl")]
                struct Template {
                    request_user:  User,
                    is_ajax:       i32,
                    feedback_list: Vec<Feedback>,
                }
                let body = Template {
                    request_user:  _request_user,
                    is_ajax:       is_ajax,
                    feedback_list: _feedbacks,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/main/feedback_list.stpl")]
                struct Template {
                    is_ajax:       i32,
                    feedback_list: Vec<Feedback>,
                }
                let body = Template {
                    is_ajax:       is_ajax,
                    feedback_list: _feedbacks,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
}

pub async fn serve_list_page(req: HttpRequest, session: Session) -> actix_web::Result<HttpResponse> {
    use crate::models::TechCategories;
    use crate::schema::tech_categories::dsl::tech_categories;

    let _connection = establish_connection();
    let all_tech_categories = tech_categories
        .order(schema::tech_categories::level.asc())
        .load::<TechCategories>(&_connection)
        .expect("E.");

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Список опций и услуг".to_string(),
            "вебсервисы.рф: Список опций и услуг".to_string(),
            "/serve_list/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
        ).await
    }
    else if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/serve_list.stpl")]
            struct Template {
                request_user: User,
                is_ajax:      i32,
                tech_cats:    Vec<TechCategories>,
            }
            let body = Template {
                request_user: _request_user,
                is_ajax:      is_ajax,
                tech_cats:    all_tech_categories,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/main/serve_list.stpl")]
            struct Template {
                request_user: User,
                is_ajax:      i32,
                tech_cats:    Vec<TechCategories>,
            }
            let body = Template {
                request_user: _request_user,
                is_ajax:      is_ajax,
                tech_cats:    all_tech_categories,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/anon_serve_list.stpl")]
            struct Template {
                is_ajax:      i32,
                tech_cats:    Vec<TechCategories>,
            }
            let body = Template {
                is_ajax:      is_ajax,
                tech_cats:    all_tech_categories,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/main/anon_serve_list.stpl")]
            struct Template {
                is_ajax:      i32,
                tech_cats:    Vec<TechCategories>,
            }
            let body = Template {
                is_ajax:      is_ajax,
                tech_cats:    all_tech_categories,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn get_tech_category_page(_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::models::TechCategories;
    use crate::schema::tech_categories::dsl::tech_categories;

    let _connection = establish_connection();
    let tech_category = tech_categories
        .filter(schema::tech_categories::id.eq(*_id))
        .first::<TechCategories>(&_connection)
        .expect("E.");

    #[derive(TemplateOnce)]
    #[template(path = "desctop/load/tech_category.stpl")]
    struct Template {
        object: TechCategories,
    }
    let body = Template {
        object: tech_category,
    }
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
}

pub async fn get_serve_category_page(_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::models::ServeCategories;
    use crate::schema::serve_categories::dsl::serve_categories;

    let _connection = establish_connection();
    let serve_category = serve_categories
        .filter(schema::serve_categories::id.eq(*_id))
        .first::<ServeCategories>(&_connection)
        .expect("E.");

    #[derive(TemplateOnce)]
    #[template(path = "desctop/load/serve_category.stpl")]
    struct Template {
        object: ServeCategories,
    }
    let body = Template {
        object: serve_category,
    }
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
}

pub async fn get_serve_page(_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::models::Serve;
    use crate::schema::serve::dsl::serve;

    let _connection = establish_connection();
    let _serve = serve
        .filter(schema::serve::id.eq(*_id))
        .first::<Serve>(&_connection)
        .expect("E.");

    #[derive(TemplateOnce)]
    #[template(path = "desctop/load/serve.stpl")]
    struct Template {
        object: Serve,
    }
    let body = Template {
        object: _serve,
    }
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
}

pub async fn get_feedback_page() -> actix_web::Result<HttpResponse> {
    #[derive(TemplateOnce)]
    #[template(path = "desctop/load/feedback.stpl")]
    struct Template {}
    let body = Template {}
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
}

pub async fn cookie_users_list_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_page;
    use crate::models::CookieUser;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let _connection = establish_connection();
    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Общая статистика сайта".to_string(),
            "вебсервисы.рф: Общая статистика сайта".to_string(),
            "/cookie_users_list/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
        ).await
    }
    else {
        let (object_list, next_page_number) = CookieUser::get_users_list(get_page(&req), 20);

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/pages/stat.stpl")]
                struct Template {
                    request_user:     User,
                    object_list:      Vec<CookieUser>,
                    next_page_number: i32,
                    is_ajax:          i32,
                }
                let body = Template {
                    request_user:     _request_user,
                    object_list:      object_list,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/pages/stat.stpl")]
                struct Template {
                    request_user:     User,
                    object_list:      Vec<CookieUser>,
                    next_page_number: i32,
                    is_ajax:          i32,
                }
                let body = Template {
                    request_user:     _request_user,
                    object_list:      object_list,
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
                #[template(path = "desctop/pages/anon_stat.stpl")]
                struct Template {
                    object_list:      Vec<CookieUser>,
                    next_page_number: i32,
                    is_ajax:          i32,
                }
                let body = Template {
                    object_list:      object_list,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/pages/anon_stat.stpl")]
                struct Template {
                    object_list:      Vec<CookieUser>,
                    next_page_number: i32,
                    is_ajax:          i32,
                }
                let body = Template {
                    object_list:      object_list,
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

pub async fn get_user_history_page(session: Session, req: HttpRequest, user_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.is_superuser() {
            use crate::utils::get_page;
            use crate::models::CookieStat;

            let object_list: Vec<CookieStat>;
            let next_page_number: i32;
            let page = get_page(&req);
            let _res = block(move || CookieStat::get_stat_list(*user_id, page, 20)).await?;
            let _dict = match _res {
                Ok(_ok) => {object_list = _ok.0; next_page_number = _ok.1},
                Err(_error) => {object_list = Vec::new(); next_page_number = 0},
            };

            #[derive(TemplateOnce)]
            #[template(path = "desctop/load/user_stat.stpl")]
            struct Template {
                object_list:      Vec<CookieStat>,
                next_page_number: i32,
            }
            let body = Template {
                object_list:      object_list,
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied"))
        }
    }
    else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied"))
    }
}

pub async fn get_tech_objects_page(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::models::TechCategories;
    use crate::schema::tech_categories::dsl::tech_categories;

    let mut is_admin = false;
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.is_superuser() {
            is_admin = true;
        }
    }
    let _connection = establish_connection();
    let _cat = tech_categories
        .filter(schema::tech_categories::id.eq(*_id))
        .first::<TechCategories>(&_connection)
        .expect("E.");

    #[derive(TemplateOnce)]
    #[template(path = "desctop/load/tech_category_objects.stpl")]
    struct Template {
        object:   TechCategories,
        is_admin: bool,
    }
    let body = Template {
        object:   _cat,
        is_admin: is_admin
    }
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
}

pub async fn unical_object_form_page(session: Session, _id: web::Path<i16>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if !_request_user.is_superuser() {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied"))
        }
        else {
            let _connection = establish_connection();
            let types = *_id;
            let mut biznes_mode = false;
            if vec![2,3,5].iter().any(|i| i==&types) {
                biznes_mode = true;
            }
            let _cats: Vec<Cat>;
            let cats_res = block(move || Categories::get_categories_for_types(types)).await?;
            let _cats = match cats_res {
                Ok(_ok) => _ok,
                Err(_error) => Vec::new(),
            };

            #[derive(TemplateOnce)]
            #[template(path = "desctop/load/unical_object_form.stpl")]
            struct Template {
                cats:        Vec<Cat>,
                biznes_mode: bool,
            }
            let body = Template {
                cats:        _cats,
                biznes_mode: biznes_mode
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied"))
    }
}

pub async fn create_category_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Создание категории".to_string(),
            "вебсервисы.рф: Создание категории".to_string(),
            "/create_category/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
        ).await
    }
    else if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use schema::categories::dsl::categories;

            let _connection = establish_connection();
            let _cats = categories
                .load::<Categories>(&_connection)
                .expect("Error");

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/pages/create_category.stpl")]
                struct Template {
                    request_user: User,
                    cats:         Vec<Categories>,
                    is_ajax:      i32,
                }
                let body = Template {
                    request_user: _request_user,
                    cats:         _cats,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/pages/create_category.stpl")]
                struct Template {
                    cats:    Vec<Categories>,
                    is_ajax: i32,
                }
                let body = Template {
                    cats:    _cats,
                    is_ajax: is_ajax,
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

pub async fn edit_category_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let cat_id: i32 = *_id;
    let _connection = establish_connection();
    let _cats = schema::categories::table
        .filter(schema::categories::id.eq(&cat_id))
        .load::<Categories>(&_connection)
        .expect("E");
    let _cat = _cats.into_iter().nth(0).unwrap();

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Изменение категории ".to_string() + &_cat.name,
            "вебсервисы.рф: Изменение категории ".to_string() + &_cat.name,
            "/edit_category/".to_string() + &_cat.id.to_string() + &"/".to_string(),
            _cat.get_image(),
        ).await
    }
    else if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _cats = schema::categories::table
                .load::<Categories>(&_connection)
                .expect("Error");

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/pages/edit_category.stpl")]
                struct Template {
                    request_user: User,
                    cat:          Categories,
                    cats:         Vec<Categories>,
                    is_ajax:      i32,
                }
                let body = Template {
                    request_user: _request_user,
                    cat:          _cat,
                    cats:         _cats,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/pages/edit_category.stpl")]
                struct Template {
                    cat:     Categories,
                    cats:    Vec<Categories>,
                    is_ajax: i32,
                }
                let body = Template {
                    cat:     _cat,
                    cats:    _cats,
                    is_ajax: is_ajax,
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

pub async fn create_item_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Создание объекта".to_string(),
            "вебсервисы.рф: Создание объекта".to_string(),
            "/create_item/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
        ).await
    }
    else if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use schema::{
                tags::dsl::tags,
                tech_categories::dsl::tech_categories,
            };
            use crate::models::TechCategories;

            let _connection = establish_connection();
            let all_tags = tags
                .load::<Tag>(&_connection)
                .expect("Error.");

            let _tech_categories = tech_categories
                .load::<TechCategories>(&_connection)
                .expect("E");

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/pages/create_item.stpl")]
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
                #[template(path = "mobile/pages/create_item.stpl")]
                struct Template {
                    all_tags: Vec<Tag>,
                    is_ajax:  i32,
                }
                let body = Template {
                    all_tags: all_tags,
                    is_ajax:  is_ajax,
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
pub async fn edit_item_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use schema::items::dsl::items;

    let _item_id: i32 = *_id;
    let _connection = establish_connection();
    let _item = items
        .filter(schema::items::id.eq(&_item_id))
        .first::<Item>(&_connection)
        .expect("E");

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Изменение объекта ".to_string() + &_item.title,
            "вебсервисы.рф: Изменение объекта ".to_string() + &_item.title,
            "/edit_item/".to_string() + &_item.id.to_string() + &"/".to_string(),
            _item.get_image(),
        ).await
    }
    else if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 || _item.user_id == _request_user.id {
            use schema::{
                tags::dsl::tags,
                categories::dsl::categories,
                tech_categories::dsl::tech_categories,
            };
            use crate::models:: TechCategories;

            let item_cats = _item.get_categories_obj().expect("E");
            //item_cats = match cats_res {
            //    Ok(_ok) => _ok,
            //    Err(_error) => Vec::new(),
            //};
            let item_tags = _item.get_tags_obj().expect("E");
            //item_tags = match tags_res {
            //    Ok(_list) => _list,
            //    Err(_error) => Vec::new(),
            //};

            let _all_tags = tags
                .load::<Tag>(&_connection)
                .expect("Error.");

            let _cats = categories
                .filter(schema::categories::types.eq(_item.types))
                .load::<Categories>(&_connection)
                .expect("Error");

            let mut level: i16 = 0;
            let mut _tech_categories: Vec<TechCategories> = Vec::new();
            let _serve = _item.get_serves();
            if _serve.len() > 0 {
                let tech_id = _serve[0].tech_cat_id;
                let _tech_categories = tech_categories
                    .filter(schema::tech_categories::id.eq(tech_id))
                    .load::<TechCategories>(&_connection)
                    .expect("E");

                level = _tech_categories[0].level;
                let _tech_categories = tech_categories
                    .filter(schema::tech_categories::level.eq(level))
                    .load::<TechCategories>(&_connection)
                    .expect("E");
            }

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/pages/edit_item.stpl")]
                struct Template {
                    request_user: User,
                    object:       Item,
                    cats:         Vec<Categories>,
                    is_ajax:      i32,
                    all_tags:     Vec<Tag>,
                    item_tags:    Vec<Tag>,
                    item_cats:    Vec<Categories>,
                    tech_cats:    Vec<TechCategories>,
                    level:        i16,
                }
                let body = Template {
                    request_user: _request_user,
                    object:       _item,
                    cats:         _cats,
                    is_ajax:      is_ajax,
                    all_tags:     _all_tags,
                    item_tags:    item_tags,
                    item_cats:    item_cats,
                    tech_cats:    _tech_categories,
                    level:        level,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/pages/edit_item.stpl")]
                struct Template {
                    object:     Item,
                    cats:       Vec<Categories>,
                    is_ajax:    i32,
                    all_tags:   Vec<Tag>,
                    item_tags:  Vec<Tag>,
                    item_cats:  Vec<Categories>,
                    tech_cats:  Vec<TechCategories>,
                    level:      i16,
                }
                let body = Template {
                    object:     _item,
                    cats:       _cats,
                    is_ajax:    is_ajax,
                    all_tags:   _all_tags,
                    item_tags:  item_tags,
                    item_cats:  item_cats,
                    tech_cats:  _tech_categories,
                    level:      level,
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

pub async fn edit_content_item_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::schema::items::dsl::items;

    let _item_id: i32 = *_id;
    let _connection = establish_connection();
    let _item = items
        .filter(schema::items::id.eq(&_item_id))
        .first::<Item>(&_connection)
        .expect("E");

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Изменение текста объекта ".to_string() + &_item.title,
            "вебсервисы.рф: Изменение текста объекта ".to_string() + &_item.title,
            "/edit_content_item/".to_string() + &_item.id.to_string() + &"/".to_string(),
            _item.get_image(),
        ).await
    }
    else if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 || _request_user.id == _item.user_id {
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/pages/edit_content_item.stpl")]
                struct Template {
                    request_user: User,
                    item:         Item,
                    is_ajax:      i32,
                }
                let body = Template {
                    request_user: _request_user,
                    item:         _item,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/pages/edit_content_item.stpl")]
                struct Template {
                    item:    Item,
                    is_ajax: i32,
                }
                let body = Template {
                    item:    _item,
                    is_ajax: is_ajax,
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

pub async fn edit_file_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::schema::files::dsl::files;
    use crate::models::File;

    let _file_id: i32 = *_id;
    let _connection = establish_connection();
    let _file = files
        .filter(schema::files::id.eq(&_file_id))
        .first::<File>(&_connection)
        .expect("E");

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Изменение файла".to_string(),
            "вебсервисы.рф: Изменение файла".to_string(),
            "/edit_file/".to_string() + &_file.id.to_string() + &"/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
        ).await
    }
    else if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 || _request_user.id == _file.user_id {
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/pages/edit_file.stpl")]
                struct Template {
                    request_user: User,
                    file:         File,
                    is_ajax:      i32,
                }
                let body = Template {
                    request_user: _request_user,
                    file:         _file,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/pages/edit_file.stpl")]
                struct Template {
                    file:    File,
                    is_ajax: i32,
                }
                let body = Template {
                    file:    _file,
                    is_ajax: is_ajax,
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

pub async fn image_page(_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::schema::{
        files::dsl::files,
        items::dsl::items,
    };
    use crate::models::File;

    let _connection = establish_connection();
    let _id: i32 = *_id;
    let _file = files
        .filter(schema::files::id.eq(_id))
        .first::<File>(&_connection)
        .expect("E.");

    let _item = items
        .filter(schema::items::id.eq(_file.item_id))
        .filter(schema::items::types.eq(_file.item_types))
        .first::<Item>(&_connection)
        .expect("E.");

    let _images = _item.get_images_ids();
    let _images_len = _images.len();
    let mut prev: Option<File> = None;
    let mut next: Option<File> = None;

    for (i, obj) in _images.iter().enumerate().rev() {
        if obj == &_id {
            if (i + 1) != _images_len {
                let _next = Some(&_images[i + 1]);
                next = Some(files
                    .filter(schema::files::id.eq(_next.unwrap()))
                    .filter(schema::files::types.eq(_item.types))
                    .first::<File>(&_connection)
                    .expect("E"));
            };
            if i != 0 {
                let _prev = Some(&_images[i - 1]);
                prev = Some(files
                    .filter(schema::files::id.eq(_prev.unwrap()))
                    .filter(schema::files::types.eq(_item.types))
                    .first::<File>(&_connection)
                    .expect("E"));
            };
            break;
        }
    };

    #[derive(TemplateOnce)]
    #[template(path = "desctop/load/image.stpl")]
    struct Template {
        object: File,
        item:   Item,
        prev:   Option<File>,
        next:   Option<File>,
    }
    let body = Template {
        object: _file,
        item:   _item,
        prev:   prev,
        next:   next,
    }
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
}
