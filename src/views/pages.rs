use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    web,
    error::InternalError,
    http::StatusCode,
};

use crate::models::{User, CookieUser, HistoryResponse};
use serde::{Deserialize, Serialize};
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


pub fn pages_routes(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index));
    config.route("/about/", web::get().to(about));
    config.route("/feedback/", web::post().to(create_feedback));
    config.route("/feedback_list/", web::get().to(feedback_list_page));
    config.route("/serve_list/", web::get().to(serve_list_page));
    config.route("/load_item/", web::get().to(get_load_page));
    config.route("/create_history/", web::get().to(create_history));
    config.route("/object_history/{id}/", web::get().to(object_history));
}

#[derive(Debug, Deserialize)]
pub struct SParams {
    pub q: String,
}

pub async fn index(req: HttpRequest, session: Session) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    // первая отрисовка страницы - организуем скрытие информации
    if is_ajax == 0 {
        get_first_load_page(&session, is_desctop, "Главная страница".to_string()).await
    }
    else {
        use crate::models::{Work, Service, Wiki, Blog, Store};

        //for header in req.headers().into_iter() {
        //    if header.0 == "cookie" {
        //        let str_cookie = header.1.to_str().unwrap();
        //        let _cookie: Vec<&str> = str_cookie.split(";").collect();
        //        for c in _cookie.iter() {
        //            let split_c: Vec<&str> = c.split("=").collect();
        //            println!("name {:?}", split_c[0].trim());
        //            println!("value {:?}", split_c[1]);
        //        }
        //    }
        //};
        let _last_works = Work::get_3_works();
        let _last_services = Service::get_6_services();
        let _last_wikis = Wiki::get_3_wikis();
        let _last_blogs = Blog::get_3_blogs();
        let _last_stores = Store::get_3_stores();

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            _request_user.create_superuser();
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
                    title:         String,
                }
                let body = Template {
                    request_user:  _request_user,
                    last_works:    _last_works,
                    last_services: _last_services,
                    last_wikis:    _last_wikis,
                    last_blogs:    _last_blogs,
                    last_stores:   _last_stores,
                    is_ajax:       is_ajax,
                    title:         "Главная страница".to_string(),
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/main/mainpage.stpl")]
                struct Template {
                    //request_user:  User,
                    last_works:    Vec<Work>,
                    last_services: Vec<Service>,
                    last_wikis:    Vec<Wiki>,
                    last_blogs:    Vec<Blog>,
                    last_stores:   Vec<Store>,
                    is_ajax:       i32,
                    title:         String,
                }
                let body = Template {
                    //request_user:  _request_user,
                    last_works:    _last_works,
                    last_services: _last_services,
                    last_wikis:    _last_wikis,
                    last_blogs:    _last_blogs,
                    last_stores:   _last_stores,
                    is_ajax:       is_ajax,
                    title:         "Главная страница".to_string(),
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
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
                    title:         String,
                }
                let body = Template {
                    last_works:    _last_works,
                    last_services: _last_services,
                    last_wikis:    _last_wikis,
                    last_blogs:    _last_blogs,
                    last_stores:   _last_stores,
                    is_ajax:       is_ajax,
                    title:         "Главная страница".to_string(),
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
                    title:         String,
                }
                let body = Template {
                    last_works:    _last_works,
                    last_services: _last_services,
                    last_wikis:    _last_wikis,
                    last_blogs:    _last_blogs,
                    last_stores:   _last_stores,
                    is_ajax:       is_ajax,
                    title:         "Главная страница".to_string(),
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn about(req: HttpRequest, session: Session) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    // первая отрисовка страницы - организуем скрытие информации
    if is_ajax == 0 {
        get_first_load_page(&session, is_desctop, "О нас".to_string()).await
    }
    else if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/pages/about.stpl")]
            struct Template {
                request_user: User,
                is_ajax:      i32,
                title:        String,
            }
            let body = Template {
                request_user: _request_user,
                is_ajax:      is_ajax,
                title:        "О нас".to_string(),
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/pages/about.stpl")]
            struct Template {
                //request_user: User,
                is_ajax:      i32,
                title:        String,
            }
            let body = Template {
                //request_user: _request_user,
                is_ajax:      is_ajax,
                title:        "О нас".to_string(),
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/pages/anon_about.stpl")]
            struct Template {
                is_ajax: i32,
                title:   String,
            }
            let body = Template {
                is_ajax: is_ajax,
                title:   "О нас".to_string(),
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/pages/anon_about.stpl")]
            struct Template {
                is_ajax: i32,
                title:   String,
            }
            let body = Template {
                is_ajax: is_ajax,
                title:   "О нас".to_string(),
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn create_feedback(mut payload: actix_multipart::Multipart) -> impl Responder {
    use crate::schema::feedbacks;
    use std::borrow::BorrowMut;
    use crate::models::{Feedback, NewFeedback};
    use crate::utils::feedback_form;

    let _connection = establish_connection();
    let form = feedback_form(payload.borrow_mut()).await;
    let new_feedback = NewFeedback {
        username: form.username.clone(),
        email:    form.email.clone(),
        message:  form.message.clone()
    };
    let _new_feedback = diesel::insert_into(feedbacks::table)
        .values(&new_feedback)
        .get_result::<Feedback>(&_connection)
        .expect("E.");
    return HttpResponse::Ok();
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
                    title:         String,
                    request_user:  User,
                    is_ajax:       i32,
                    feedback_list: Vec<Feedback>,
                }
                let body = Template {
                    title:         "Сообщения с формы".to_string(),
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
                    title:         String,
                    //request_user:  User,
                    is_ajax:       i32,
                    feedback_list: Vec<Feedback>,
                }
                let body = Template {
                    title:         "Сообщения с формы".to_string(),
                    //request_user:  _request_user,
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
    let all_tech_categories: Vec<TechCategories> = tech_categories
        .order(schema::tech_categories::position.asc())
        .load(&_connection)
        .expect("E.");

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/serve_list.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                is_ajax:      i32,
                tech_cats:    Vec<TechCategories>,
            }
            let body = Template {
                title:        "Список опций и услуг".to_string(),
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
                title:        String,
                request_user: User,
                is_ajax:      i32,
                tech_cats:    Vec<TechCategories>,
            }
            let body = Template {
                title:        "Список опций и услуг".to_string(),
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
                title:     String,
                is_ajax:   i32,
                tech_cats: Vec<TechCategories>,
            }
            let body = Template {
                title:     "Список опций и услуг".to_string(),
                is_ajax:   is_ajax,
                tech_cats: all_tech_categories,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/main/anon_serve_list.stpl")]
            struct Template {
                title:     String,
                is_ajax:   i32,
                tech_cats: Vec<TechCategories>,
            }
            let body = Template {
                title:     "Список опций и услуг".to_string(),
                is_ajax:   is_ajax,
                tech_cats: all_tech_categories,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LoadParams {
    pub _object_type: String,
    pub _owner_type:  String,
    pub _object_pk:   i32,
    pub _owner_pk:    i32,
}
pub async fn get_load_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::schema;

    let mut _object_type: String = "".to_string();
    let mut _owner_type: String = "".to_string();
    let mut _object_pk: i32 = 0;
    let mut _owner_pk: i32 = 0;

    let _connection = establish_connection();
    let params = web::Query::<LoadParams>::from_query(&req.query_string());
    if params.is_ok() {
        let wrap = params.unwrap();
        if wrap._object_type != "".to_string() {
            _object_type = wrap._object_type.clone();
        }
        if wrap._owner_type != "".to_string() {
            _owner_type = wrap._owner_type.clone();
        }
        if wrap._object_pk != 0 {
            _object_pk = wrap._object_pk.clone();
        }
        if wrap._owner_pk != 0 {
            _owner_pk = wrap._owner_pk.clone();
        }
    }

    if _object_type == "serve_category".to_string() {
        use crate::models::ServeCategories;
        use crate::schema::serve_categories::dsl::serve_categories;

        let _serve_categorys = serve_categories
            .filter(schema::serve_categories::id.eq(&_object_pk))
            .load::<ServeCategories>(&_connection)
            .expect("E");
        let _serve_category = _serve_categorys.into_iter().nth(0).unwrap();

        #[derive(TemplateOnce)]
        #[template(path = "desctop/load/serve_category.stpl")]
        struct Template {
            title:        String,
            object:      ServeCategories,
            //object_type: String,
        }
        let body = Template {
            title:        "Информация о технология услуг".to_string() + &_serve_category.name,
            object:      _serve_category,
            //object_type: "serve_category".to_string(),
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

    } else if _object_type == "serve".to_string() && _owner_type != "service".to_string() {
        use crate::models::Serve;
        use crate::schema::serve::dsl::serve;

        let _serves = serve
            .filter(schema::serve::id.eq(&_object_pk))
            .load::<Serve>(&_connection)
            .expect("E");

        let _serve = _serves.into_iter().nth(0).unwrap();

        #[derive(TemplateOnce)]
        #[template(path = "desctop/load/serve.stpl")]
        struct Template {
            title:        String,
            object:      Serve,
            //object_type: String,
        }
        let body = Template {
            title:        "Информация об опции".to_string() + &_serve.name,
            object:      _serve,
            //object_type: "serve".to_string(),
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else if _object_type == "serve".to_string() && _owner_type == "service".to_string() {
        use crate::models::{Serve, Service};
        use crate::schema::{
            serve::dsl::serve,
            services::dsl::services
        };

        let _serves = serve
            .filter(schema::serve::id.eq(&_object_pk))
            .load::<Serve>(&_connection)
            .expect("E");
        let _serve = _serves.into_iter().nth(0).unwrap();

        let _service_id: i32 = _owner_pk;
        let _service = services
            .filter(schema::services::id.eq(&_service_id))
            .load::<Service>(&_connection)
            .expect("E");

        #[derive(TemplateOnce)]
        #[template(path = "desctop/load/serve.stpl")]
        struct Template {
            title:        String,
            object:      Serve,
            //object_type: String,
            //service:     Service,
        }
        let body = Template {
            title:        "Информация об опции услуги".to_string() + &_serve.name,
            object:      _serve,
            //object_type: "serve".to_string(),
            //service:     _service.into_iter().nth(0).unwrap(),
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn create_c_user(req: &HttpRequest) -> CookieUser {
    use crate::models::NewCookieUser;
    use crate::schema;

    #[derive(Debug, Deserialize)]
    pub struct UserLoc {
        pub city:    CityLoc,
        pub region:  RegionLoc,
        pub country: CountryLoc,
    }
    #[derive(Debug, Deserialize)]
    pub struct CityLoc {
        pub name_ru: String,
        pub name_en: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct RegionLoc {
        pub name_ru: String,
        pub name_en: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct CountryLoc {
        pub name_ru: String,
        pub name_en: String,
    }

    let _connection = establish_connection();
    let ip = &req.peer_addr().unwrap().ip().to_string();

    let mut device: i16 = 1;
    for header in req.headers().into_iter() {
        if header.0 == "user-agent" {
            let str_agent = header.1.to_str().unwrap();
            if str_agent.contains("Mobile") {
                device = 2;
            };
            break;
        }
    };

    let _geo_url = "http://api.sypexgeo.net/J5O6d/json/".to_owned() + &ip;
    let _geo_request = reqwest::get(_geo_url).await.expect("E.");
    let new_request = _geo_request.text().await.unwrap();
    let location200: UserLoc = serde_json::from_str(&new_request).unwrap();
    let _user = NewCookieUser {
        ip:         ip.to_string(),
        device:     device,
        city_ru:    Some(location200.city.name_ru),
        city_en:    Some(location200.city.name_en),
        region_ru:  Some(location200.region.name_ru),
        region_en:  Some(location200.region.name_en),
        country_ru: Some(location200.country.name_ru),
        country_en: Some(location200.country.name_en),
        height:     0.0,
        seconds:    0,
        created:    chrono::Local::now().naive_utc(),
    };
    let _new_user = diesel::insert_into(schema::cookie_users::table)
        .values(&_user)
        .get_result::<CookieUser>(&_connection)
        .expect("Error.");
    return _new_user;
}

pub async fn get_c_user(id: i32, req: &HttpRequest) -> CookieUser {
    if id > 0 {
        use crate::schema;
        use crate::schema::cookie_users::dsl::cookie_users;

        let _connection = establish_connection();
        let _users = cookie_users
            .filter(schema::cookie_users::id.eq(id))
            .load::<CookieUser>(&_connection)
            .expect("E");

        if _users.len() > 0 {
            return _users.into_iter().nth(0).unwrap();
        }
        else {
            return create_c_user(&req).await;
        }
    }
    else {
        return create_c_user(&req).await;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryParams {
    pub user_id:   i32,
    pub object_id: Option<i32>,
    pub page_id:   i16,
    pub link:      String,
    pub title:     String,
    pub height:    f64,
    pub seconds:   i32,
}
pub async fn create_history(req: HttpRequest) -> web::Json<HistoryResponse> {
    use crate::schema;
    use crate::models::CookieStat;

    let params = web::Query::<HistoryParams>::from_query(&req.query_string());
    let params_2 = params.unwrap();
    let p_id = params_2.user_id;
    let user = get_c_user(p_id, &req).await;

    let p_object_id = params_2.object_id;
    let p_page_id = params_2.page_id;
    let p_height = params_2.height;
    let p_seconds = params_2.seconds;
    let _connection = establish_connection();

    diesel::update(&user)
        .set ((
            schema::cookie_users::height.eq(user.height + p_height),
            schema::cookie_users::seconds.eq(user.seconds + p_seconds),
        ))
        .get_result::<CookieUser>(&_connection)
        .expect("Error.");
    if p_object_id.is_some() {
        match p_page_id {
            42 => {
                use crate::utils::plus_blog_category_stat;
                plus_blog_category_stat(p_object_id.unwrap(), p_height, p_seconds)
            },
            43 => {
                use crate::utils::plus_blog_stat;
                plus_blog_stat(p_object_id.unwrap(), p_height, p_seconds)
            },
            62 => {
                use crate::utils::plus_service_category_stat;
                plus_service_category_stat(p_object_id.unwrap(), p_height, p_seconds)
            },
            63 => {
                use crate::utils::plus_service_stat;
                plus_service_stat(p_object_id.unwrap(), p_height, p_seconds)
            },
            72 => {
                use crate::utils::plus_store_category_stat;
                plus_store_category_stat(p_object_id.unwrap(), p_height, p_seconds)
            },
            73 => {
                use crate::utils::plus_store_stat;
                plus_store_stat(p_object_id.unwrap(), p_height, p_seconds)
            },
            82 => {
                use crate::utils::plus_wiki_category_stat;
                plus_wiki_category_stat(p_object_id.unwrap(), p_height, p_seconds)
            },
            83 => {
                use crate::utils::plus_wiki_stat;
                plus_wiki_stat(p_object_id.unwrap(), p_height, p_seconds)
            },
            92 => {
                use crate::utils::plus_work_category_stat;
                plus_work_category_stat(p_object_id.unwrap(), p_height, p_seconds)
            },
            93 => {
                use crate::utils::plus_work_stat;
                plus_work_stat(p_object_id.unwrap(), p_height, p_seconds)
            },
            32 => {
                use crate::utils::plus_tag_stat;
                plus_tag_stat(p_object_id.unwrap(), p_height, p_seconds)
            },
            _ => println!("no value"),
        };
    }
    else {
        match p_page_id {
            1 => {
                use crate::utils::plus_mainpage_stat;
                plus_mainpage_stat(p_height, p_seconds)
            },
            2 => {
                use crate::utils::plus_about_stat;
                plus_about_stat(p_height, p_seconds)
            },
            3 => {
                use crate::utils::plus_contact_stat;
                plus_contact_stat(p_height, p_seconds)
            },
            4 => {
                use crate::utils::plus_team_stat;
                plus_team_stat(p_height, p_seconds)
            },
            5 => {
                use crate::utils::plus_partnership_stat;
                plus_partnership_stat(p_height, p_seconds)
            },
            6 => {
                use crate::utils::plus_login_stat;
                plus_login_stat(p_height, p_seconds)
            },
            7 => {
                use crate::utils::plus_signup_stat;
                plus_signup_stat(p_height, p_seconds)
            },
            8 => {
                use crate::utils::plus_logout_stat;
                plus_logout_stat(p_height, p_seconds)
            },
            9 => {
                use crate::utils::plus_help_stat;
                plus_help_stat(p_height, p_seconds)
            },
            10 => {
                use crate::utils::plus_info_stat;
                plus_info_stat(p_height, p_seconds)
            },
            11 => {
                use crate::utils::plus_profil_stat;
                plus_profil_stat(p_height, p_seconds)
            },
            31 => {
                use crate::utils::plus_tags_stat;
                plus_tags_stat(p_height, p_seconds)
            },
            41 => {
                use crate::utils::plus_blog_categories_stat;
                plus_blog_categories_stat(p_height, p_seconds)
            },
            61 => {
                use crate::utils::plus_service_categories_stat;
                plus_service_categories_stat(p_height, p_seconds)
            },
            71 => {
                use crate::utils::plus_store_categories_stat;
                plus_store_categories_stat(p_height, p_seconds)
            },
            81 => {
                use crate::utils::plus_wiki_categories_stat;
                plus_wiki_categories_stat(p_height, p_seconds)
            },
            91 => {
                use crate::utils::plus_work_categories_stat;
                plus_work_categories_stat(p_height, p_seconds)
            },
            _ => println!("no value"),
        }
    }

    let p_link = params_2.link.clone();
    let p_title = params_2.title.clone();

    return CookieStat::create (
        user.id,
        p_page_id,
        p_link,
        p_title,
        p_height,
        p_seconds,
    )
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectResponse {
    pub id:         i32,
    pub ip:         String,
    pub device:     i16,
    pub city_ru:    Option<String>,
    pub city_en:    Option<String>,
    pub region_ru:  Option<String>,
    pub region_en:  Option<String>,
    pub country_ru: Option<String>,
    pub country_en: Option<String>,
}
pub async fn object_history(req: HttpRequest, id: web::Path<i32>) -> web::Json<ObjectResponse> {
    let _user = get_c_user(*id, &req).await;
    return web::Json( ObjectResponse {
        id:         _user.id,
        ip:         _user.ip,
        device:     _user.device,
        city_ru:    _user.city_ru,
        city_en:    _user.city_en,
        region_ru:  _user.region_ru,
        region_en:  _user.region_en,
        country_ru: _user.country_ru,
        country_en: _user.country_en,
    })
}
