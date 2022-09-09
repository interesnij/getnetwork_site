use actix_web::{
    HttpRequest,
    HttpResponse,
    //Responder,
    web,
    error::InternalError,
    http::StatusCode,
};

use crate::models::User;
use crate::utils::{
    establish_connection,
    get_device_and_ajax,
    get_request_user_data,
    is_signed_in,
    get_first_load_page,
};
use crate::diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use actix_session::Session;
use sailfish::TemplateOnce;
use actix_web::dev::ConnectionInfo;


pub fn pages_routes(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index_page));
    config.route("/info/", web::get().to(info_page));
    config.route("/history/", web::get().to(history_page));
    config.route("/feedback_list/", web::get().to(feedback_list_page));
    config.route("/serve_list/", web::get().to(serve_list_page));
    config.route("/load_tech_category/{id}/", web::get().to(get_tech_category_page));
    config.route("/load_serve_category/{id}/", web::get().to(get_serve_category_page));
    config.route("/load_serve/{id}/", web::get().to(get_serve_page));
    config.route("/load_feedback/", web::get().to(get_feedback_page));
    config.route("/cookie_users_list/", web::get().to(cookie_users_list_page));
}

//#[derive(Debug, Deserialize)]
//pub struct SParams {
//    pub q: String,
//}

pub async fn index_page(req: HttpRequest, session: Session) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    if is_ajax == 0 {
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
        use crate::schema;
        use crate::models::{Work, Service, Wiki, Blog, Store, StatMainpage};
        use crate::schema::stat_mainpages::dsl::stat_mainpages;

        let _connection = establish_connection();
        let _stat: StatMainpage;

        let _stats = stat_mainpages
            .limit(1)
            .load::<StatMainpage>(&_connection)
            .expect("E");
        if _stats.len() > 0 {
            _stat = _stats.into_iter().nth(0).unwrap();
        }
        else {
            use crate::models::NewStatMainpage;
            let form = NewStatMainpage {
                view: 0,
                height: 0.0,
                seconds: 0,
            };
            _stat = diesel::insert_into(schema::stat_mainpages::table)
                .values(&form)
                .get_result::<StatMainpage>(&_connection)
                .expect("Error.");
        }

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            //_request_user.create_superuser();
            let _last_works = Work::get_3_works(&_request_user);
            let _last_services = Service::get_3_services(&_request_user);
            let _last_wikis = Wiki::get_3_wikis(&_request_user);
            let _last_blogs = Blog::get_3_blogs(&_request_user);
            let _last_stores = Store::get_3_stores(&_request_user);

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
                    stat:          StatMainpage,
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
                    stat:          StatMainpage,
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
            let _last_works = Work::get_3_publish_works();
            let _last_services = Service::get_3_publish_services();
            let _last_wikis = Wiki::get_3_publish_wikis();
            let _last_blogs = Blog::get_3_publish_blogs();
            let _last_stores = Store::get_3_publish_stores();

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
                    stat:          StatMainpage,
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
                    stat:          StatMainpage,
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
        use crate::schema;
        use schema::{
            help_item_categories::dsl::help_item_categories,
            stat_infos::dsl::stat_infos,
        };
        use crate::models::{HelpItemCategorie, StatInfo};

        let _connection = establish_connection();
        let _stat: StatInfo;
        let _stats = stat_infos
            .limit(1)
            .load::<StatInfo>(&_connection)
            .expect("E");
        if _stats.len() > 0 {
            _stat = _stats.into_iter().nth(0).unwrap();
        }
        else {
            use crate::models::NewStatInfo;
            let form = NewStatInfo {
                view: 0,
                height: 0.0,
                seconds: 0,
            };
            _stat = diesel::insert_into(schema::stat_infos::table)
                .values(&form)
                .get_result::<StatInfo>(&_connection)
                .expect("Error.");
        }

        let _help_cats = help_item_categories
            .load::<HelpItemCategorie>(&_connection)
            .expect("Error");

        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/pages/info.stpl")]
            struct Template {
                request_user: User,
                is_ajax:      i32,
                help_cats:    Vec<HelpItemCategorie>,
                stat:         StatInfo,
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
                help_cats: Vec<HelpItemCategorie>,
                stat:      StatInfo,
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
        use crate::schema;
        use schema::{
            help_item_categories::dsl::help_item_categories,
            stat_infos::dsl::stat_infos,
        };
        use crate::models::{HelpItemCategorie, StatInfo};

        let _connection = establish_connection();
        let _stat: StatInfo;
        let _stats = stat_infos
            .limit(1)
            .load::<StatInfo>(&_connection)
            .expect("E");
        if _stats.len() > 0 {
            _stat = _stats.into_iter().nth(0).unwrap();
        }
        else {
            use crate::models::NewStatInfo;
            let form = NewStatInfo {
                view: 0,
                height: 0.0,
                seconds: 0,
            };
            _stat = diesel::insert_into(schema::stat_infos::table)
                .values(&form)
                .get_result::<StatInfo>(&_connection)
                .expect("Error.");
        }

        let _help_cats = help_item_categories
            .load::<HelpItemCategorie>(&_connection)
            .expect("Error");

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/pages/anon_info.stpl")]
            struct Template {
                is_ajax:   i32,
                help_cats: Vec<HelpItemCategorie>,
                stat:      StatInfo,
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
                help_cats: Vec<HelpItemCategorie>,
                is_ajax:   i32,
                stat:      StatInfo,
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
        use crate::schema;
        use schema::cookie_users::dsl::cookie_users;
        use crate::models::{CookieUser, CookieStat};
        use crate::utils::{get_page, get_or_create_cookie_user_id};

        let user_id = get_or_create_cookie_user_id(conn, &req).await;
        let _connection = establish_connection();
        let _cookie_user = cookie_users
            .filter(schema::cookie_users::id.eq(&user_id))
            .load::<CookieUser>(&_connection)
            .expect("Error")
            .into_iter()
            .nth(0)
            .unwrap();
        let (object_list, next_page_number) = CookieStat::get_stat_list(user_id, get_page(&req), 20);

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
                    //request_user:     User,
                    user:             CookieUser,
                    object_list:      Vec<CookieStat>,
                    is_ajax:          i32,
                    next_page_number: i32,
                }
                let body = Template {
                    //request_user:     _request_user,
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
    use crate::schema;
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
        if _request_user.perm == 60 {
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
                    //request_user: User,
                    is_ajax:      i32,
                    tech_cats:    Vec<TechCategories>,
                }
                let body = Template {
                    //request_user: _request_user,
                    is_ajax:      is_ajax,
                    tech_cats:    all_tech_categories,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    }
    else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn get_tech_category_page(_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::models::TechCategories;
    use crate::schema;
    use crate::schema::tech_categories::dsl::tech_categories;

    let _connection = establish_connection();
    let tech_category = tech_categories
        .filter(schema::tech_categories::id.eq(*_id))
        .load::<TechCategories>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();

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
    use crate::schema;
    use crate::schema::serve_categories::dsl::serve_categories;

    let _connection = establish_connection();
    let serve_category = serve_categories
        .filter(schema::serve_categories::id.eq(*_id))
        .load::<ServeCategories>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();

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
    use crate::schema;
    use crate::schema::serve::dsl::serve;

    let _connection = establish_connection();
    let _serve = serve
        .filter(schema::serve::id.eq(*_id))
        .load::<Serve>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();

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
