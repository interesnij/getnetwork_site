use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    web::block,
    error::InternalError,
    http::StatusCode,
    Responder,
};
use crate::models::User;
use std::borrow::BorrowMut;
use crate::diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use crate::utils::{
    establish_connection,
    is_signed_in,
    get_request_user_data,
    get_first_load_page,
    get_template,
};
use crate::schema;
use crate::models::{
    ServeCategories,
    NewServeCategories,
    Serve,
    NewServe,
    TechCategories,
    NewTechCategories,
};
use actix_session::Session;
use actix_multipart::{Field, Multipart};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::str;
use sailfish::TemplateOnce;


pub fn serve_routes(config: &mut web::ServiceConfig) {
    config.route("/serve/{id}/", web::get().to(get_serve_page));
    config.route("/serve_categories/", web::get().to(serve_categories_page));

    config.service(web::resource("/create_tech_categories/")
        .route(web::get().to(create_tech_categories_page))
        .route(web::post().to(create_tech_categories))
    );
    config.route("/load_serve_categories_from_level/{level}/", web::get().to(load_serve_categories_from_level));
    config.route("/load_form_from_level/{level}/", web::get().to(load_form_from_level));
    config.service(web::resource("/create_serve_categories/")
        .route(web::get().to(create_serve_categories_page))
        .route(web::post().to(create_serve_categories))
    );
    config.service(web::resource("/edit_tech_category/{id}/")
        .route(web::get().to(edit_tech_category_page))
        .route(web::post().to(edit_tech_category))
    );
    config.service(web::resource("/edit_serve_category/{id}/")
        .route(web::get().to(edit_serve_category_page))
        .route(web::post().to(edit_serve_category))
    );

    config.service(web::resource("/create_serve/")
        .route(web::get().to(create_serve_page))
        .route(web::post().to(create_serve))
    );
    config.service(web::resource("/edit_serve/{id}/")
        .route(web::get().to(edit_serve_page))
        .route(web::post().to(edit_serve))
    );
    config.route("/delete_serve/{id}/", web::get().to(delete_serve));
    config.route("/delete_serve_category/{id}/", web::get().to(delete_serve_category));
    config.route("/delete_tech_category/{id}/", web::get().to(delete_tech_category));
}

pub async fn serve_categories_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let template_types = get_template(&req);

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Категории услуг".to_string(),
            "вебсервисы.рф: Категории услуг".to_string(),
            "/serve_categories/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
            template_types,
        ).await
    }
    else if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm != 60 {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
        else {
            use crate::schema::serve_categories::dsl::serve_categories;

            let _connection = establish_connection();
            let _serve_cats = serve_categories
                .load::<ServeCategories>(&_connection)
                .expect("E");

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/serve/categories.stpl")]
                struct Template {
                    request_user:   User,
                    serve_cats:     Vec<ServeCategories>,
                    is_ajax:        i32,
                    template_types: i16,
                }
                let body = Template {
                    request_user:   _request_user,
                    serve_cats:     _serve_cats,
                    is_ajax:        is_ajax,
                    template_types: template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/serve/categories.stpl")]
                struct Template {
                    serve_cats:     Vec<ServeCategories>,
                    is_ajax:        i32,
                    template_types: i16,
                }
                let body = Template {
                    serve_cats:     _serve_cats,
                    is_ajax:        is_ajax,
                    template_types: template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn get_serve_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;
    use schema::serve::dsl::serve;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let template_types = get_template(&req);

    let _connection = establish_connection();
    let _serve_id: i32 = *_id;

    let _serve = serve
        .filter(schema::serve::id.eq(&_serve_id))
        .first::<Serve>(&_connection)
        .expect("E");

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Опция ".to_string() + &_serve.name,
            "вебсервисы.рф: Опция ".to_string() + &_serve.name,
            "/serve/".to_string() + &_serve.id.to_string() + &"/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
            template_types,
        ).await
    }
    else if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm != 60 {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
        else {
            use schema::serve_categories::dsl::serve_categories;

            let _s_category = serve_categories
                .filter(schema::serve_categories::id.eq(&_serve.serve_categories))
                .first::<ServeCategories>(&_connection)
                .expect("E");

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/serve/serve.stpl")]
                struct Template {
                    request_user:   User,
                    category:       ServeCategories,
                    object:         Serve,
                    is_ajax:        i32,
                    template_types: i16,
                }
                let body = Template {
                    request_user:   _request_user,
                    category:       _s_category,
                    object:         _serve,
                    is_ajax:        is_ajax,
                    template_types: template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/serve/serve.stpl")]
                struct Template {
                    category:       ServeCategories,
                    object:         Serve,
                    is_ajax:        i32,
                    template_types: i16,
                }
                let body = Template {
                    category:       _s_category,
                    object:         _serve,
                    is_ajax:        is_ajax,
                    template_types: template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn create_tech_categories_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let template_types = get_template(&req);
    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Создание веб-сервиса".to_string(),
            "вебсервисы.рф: Создание веб-сервиса".to_string(),
            "/create_tech_categories/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
            template_types,
        ).await
    }
    else if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm != 60 {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
        else {
            use schema::tech_categories::dsl::tech_categories;

            let _connection = establish_connection();
            let _categories = tech_categories
                .load::<TechCategories>(&_connection)
                .expect("E");

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/serve/create_tech_categories.stpl")]
                struct Template {
                    request_user:   User,
                    tech_cats:      Vec<TechCategories>,
                    is_ajax:        i32,
                    template_types: i16,
                }
                let body = Template {
                    request_user:   _request_user,
                    tech_cats:      _categories,
                    is_ajax:        is_ajax,
                    template_types: template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/serve/create_tech_categories.stpl")]
                struct Template {
                    tech_cats:      Vec<TechCategories>,
                    is_ajax:        i32,
                    template_types: i16,
                }
                let body = Template {
                    tech_cats:      _categories,
                    is_ajax:        is_ajax,
                    template_types: template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}
pub async fn create_serve_categories_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let template_types = get_template(&req);

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Создание технологии услуг".to_string(),
            "вебсервисы.рф: Создание технологии услуг".to_string(),
            "/create_serve_categories/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
            template_types,
        ).await
    }
    else if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm != 60 {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
        else {
            use schema::tech_categories::dsl::tech_categories;

            let _connection = establish_connection();
            let _tech_categories = tech_categories.load::<TechCategories>(&_connection).expect("E");

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/serve/create_serve_categories.stpl")]
                struct Template {
                    request_user:   User,
                    tech_cats:      Vec<TechCategories>,
                    is_ajax:        i32,
                    template_types: i16,
                }
                let body = Template {
                    request_user:   _request_user,
                    tech_cats:      _tech_categories,
                    is_ajax:        is_ajax,
                    template_types: template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/serve/create_serve_categories.stpl")]
                struct Template {
                    tech_cats:      Vec<TechCategories>,
                    is_ajax:        i32,
                    template_types: i16,
                }
                let body = Template {
                    tech_cats:      _tech_categories,
                    is_ajax:        is_ajax,
                    template_types: template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn load_serve_categories_from_level(session: Session, level: web::Path<i16>) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let template_types = get_template(&req);
        let _request_user = get_request_user_data(&session);
        if _request_user.perm != 60 {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/serve/load_serve_categories.stpl")]
            struct Template {
                serve_cats:     Vec<ServeCategories>,
                template_types: i16,
            }
            let body = Template {
                serve_cats:     ServeCategories::get_categories_from_level(&*level),
                template_types: template_types,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}
pub async fn load_form_from_level(req: HttpRequest, session: Session, level: web::Path<i16>) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm != 60 {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
        else {
            let template_types = get_template(&req);
            use crate::schema::tech_categories::dsl::tech_categories;
            let _connection = establish_connection();
            let _tech_categories = tech_categories
                .filter(schema::tech_categories::level.eq(*level))
                .order(schema::tech_categories::position.desc())
                .load::<TechCategories>(&_connection)
                .expect("E");
            #[derive(TemplateOnce)]
            #[template(path = "desctop/serve/load_serve_form.stpl")]
            struct Template {
                tech_cats:      Vec<TechCategories>,
                template_types: i16,
            }
            let body = Template {
                tech_cats:      _tech_categories,
                template_types: template_types,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn create_serve_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let template_types = get_template(&req);
    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Создание опции".to_string(),
            "вебсервисы.рф: Создание опции".to_string(),
            "/create_serve/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
            template_types,
        ).await
    }
    else if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm != 60 {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
        else {
            let _connection = establish_connection();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/serve/create_serve.stpl")]
                struct Template {
                    request_user:   User,
                    is_ajax:        i32,
                    template_types: i16,
                }
                let body = Template {
                    request_user:   _request_user,
                    is_ajax:        is_ajax,
                    template_types: template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/serve/create_serve.stpl")]
                struct Template {
                    is_ajax:        i32,
                    template_types: i16,
                }
                let body = Template {
                    is_ajax:        is_ajax,
                    template_types: template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn edit_tech_category_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;
    use crate::schema::tech_categories::dsl::tech_categories;

    let _cat_id: i32 = *_id;
    let _connection = establish_connection();
    let template_types = get_template(&req);
    let _category = tech_categories
        .filter(schema::tech_categories::id.eq(&_cat_id))
        .first::<TechCategories>(&_connection)
        .expect("E");
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Изменение веб-сервиса ".to_string() + &_category.name,
            "вебсервисы.рф: Изменение веб-сервиса ".to_string() + &_category.name,
            "/edit_tech_category/".to_string() + &_category.id.to_string() + &"/".to_string(),
            "".to_string(),
            template_types,
        ).await
    }
    else if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(&session);
        if _category.user_id != _request_user.id {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
        else {
            let _tech_categories = tech_categories.load::<TechCategories>(&_connection).expect("E");

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/serve/edit_tech_category.stpl")]
                struct Template {
                    request_user:   User,
                    tech_cats:      Vec<TechCategories>,
                    category:       TechCategories,
                    is_ajax:        i32,
                    template_types: i16,
                }
                let body = Template {
                    request_user:   _request_user,
                    tech_cats:      _tech_categories,
                    category:       _category,
                    is_ajax:        is_ajax,
                    template_types: template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/serve/edit_tech_category.stpl")]
                struct Template {
                    tech_cats:      Vec<TechCategories>,
                    category:       TechCategories,
                    is_ajax:        i32,
                    template_types: i16,
                }
                let body = Template {
                    tech_cats:      _tech_categories,
                    category:       _category,
                    is_ajax:        is_ajax,
                    template_types: template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn edit_serve_category_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;
    use crate::schema::serve_categories::dsl::serve_categories;

    let _cat_id: i32 = *_id;
    let _connection = establish_connection();
    let template_types = get_template(&req);
    let _category = serve_categories
        .filter(schema::serve_categories::id.eq(&_cat_id))
        .first::<ServeCategories>(&_connection)
        .expect("E");
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Изменение категории опций ".to_string() + &_category.name,
            "вебсервисы.рф: Изменение категории опций ".to_string() + &_category.name,
            "/edit_serve_category/".to_string() + &_category.id.to_string() + &"/".to_string(),
            "".to_string(),
            template_types,
        ).await
    }
    else if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        use crate::schema::tech_categories::dsl::tech_categories;

        let _request_user = get_request_user_data(&session);
        let _tech_categories = tech_categories.load::<TechCategories>(&_connection).expect("E");

        if _category.user_id != _request_user.id {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
        else {
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/serve/edit_serve_category.stpl")]
                struct Template {
                    request_user:   User,
                    tech_cats:      Vec<TechCategories>,
                    category:       ServeCategories,
                    is_ajax:        i32,
                    template_types: i16,
                }
                let body = Template {
                    request_user:   _request_user,
                    tech_cats:      _tech_categories,
                    category:       _category,
                    is_ajax:        is_ajax,
                    template_types: template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/serve/edit_serve_category.stpl")]
                struct Template {
                    tech_cats:      Vec<TechCategories>,
                    category:       ServeCategories,
                    is_ajax:        i32,
                    template_types: i16,
                }
                let body = Template {
                    tech_cats:      _tech_categories,
                    category:       _category,
                    is_ajax:        is_ajax,
                    template_types: template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn edit_serve_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;
    use crate::schema::serve::dsl::serve;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let _connection = establish_connection();
    let _serve_id: i32 = *_id;
    let template_types = get_template(&req);
    let _serve = serve
        .filter(schema::serve::id.eq(&_serve_id))
        .first::<Serve>(&_connection)
        .expect("E");

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Изменение опции ".to_string() + &_serve.name,
            "вебсервисы.рф: Изменение опции ".to_string() + &_serve.name,
            "/edit_serve/".to_string() + &_serve.id.to_string() + &"/".to_string(),
            "".to_string(),
            template_types,
        ).await
    }
    else if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        use crate::schema::{
            serve_categories::dsl::serve_categories,
            tech_categories::dsl::tech_categories,
        };

        let _request_user = get_request_user_data(&session);

        let _serve_cat = serve_categories
            .filter(schema::serve_categories::id.eq(&_serve.serve_categories))
            .first::<ServeCategories>(&_connection)
            .expect("E");
        let _tech_category = tech_categories
            .filter(schema::tech_categories::id.eq(_serve_cat.tech_categories))
            .first::<TechCategories>(&_connection)
            .expect("E.");

        let _level = _tech_category.level;
        let _serve_cats = ServeCategories::get_categories_from_level(&_level);

        if _serve.user_id != _request_user.id {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
        else {
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/serve/edit_serve.stpl")]
                struct Template {
                    request_user:   User,
                    level:          i16,
                    serve_cats:     Vec<ServeCategories>,
                    object:         Serve,
                    is_ajax:        i32,
                    template_types: i16,
                }
                let body = Template {
                    request_user:   _request_user,
                    level:          _level,
                    serve_cats:     _serve_cats,
                    object:         _serve,
                    is_ajax:        is_ajax,
                    template_types: template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/serve/edit_serve.stpl")]
                struct Template {
                    level:          i16,
                    serve_cats:     Vec<ServeCategories>,
                    object:         Serve,
                    is_ajax:        i32,
                    template_types: i16,
                }
                let body = Template {
                    level:          _level,
                    serve_cats:     _serve_cats,
                    object:         _serve,
                    is_ajax:        is_ajax,
                    template_types: template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn create_tech_categories(session: Session, mut payload: Multipart) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {

            use schema::tech_categories;
            use crate::utils::category_form;

            let _connection = establish_connection();
            let form = category_form(payload.borrow_mut(), _request_user.id).await;
            let new_cat = NewTechCategories {
                name:        form.name.clone(),
                description: Some(form.description.clone()),
                position:    form.position,
                count:       0,
                level:       form.level,
                user_id:     _request_user.id,
                view:        0,
                height:      0.0,
                seconds:     0,
            };
            let _new_tech = diesel::insert_into(tech_categories::table)
                .values(&new_cat)
                .execute(&_connection)
                .expect("E.");
        }
    }
    return HttpResponse::Ok();
}

pub async fn create_serve_categories(session: Session, mut payload: Multipart) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use crate::utils::serve_category_form;

            let _connection = establish_connection();
            let form = serve_category_form(payload.borrow_mut(), _request_user.id).await;

            let new_cat = NewServeCategories {
                name: form.name.clone(),
                description:     Some(form.description.clone()),
                tech_categories: form.tech_categories,
                position:        form.position,
                count:           0,
                default_price:   0,
                user_id:         _request_user.id,
                view:            0,
                height:          0.0,
                seconds:         0,
            };
            let _new_serve = diesel::insert_into(schema::serve_categories::table)
                .values(&new_cat)
                .execute(&_connection)
                .expect("E.");
        }
    }
    return HttpResponse::Ok();
}

pub async fn edit_tech_category(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::{
        tech_categories::dsl::tech_categories,
    };

    let _connection = establish_connection();
    let _cat_id: i32 = *_id;
    let _category = tech_categories
        .filter(schema::tech_categories::id.eq(_cat_id))
        .first::<TechCategories>(&_connection)
        .expect("E");

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 || _category.user_id == _request_user.id {
            use crate::utils::category_form;

            let form = category_form(payload.borrow_mut(), _request_user.id).await;
            let new_cat = NewTechCategories {
                name:        form.name.clone(),
                description: Some(form.description.clone()),
                position:    form.position,
                count:       0,
                level:       form.level,
                user_id:     _request_user.id,
                view:        0,
                height:      0.0,
                seconds:     0,
            };
            diesel::update(&_category)
                .set(new_cat)
                .execute(&_connection)
                .expect("E");
        }
    }
    return HttpResponse::Ok();
}

pub async fn edit_serve_category(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::{
        serve_categories::dsl::serve_categories,
    };

    let _connection = establish_connection();
    let _cat_id: i32 = *_id;

    let s_category = serve_categories
        .filter(schema::serve_categories::id.eq(_cat_id))
        .first::<ServeCategories>(&_connection)
        .expect("E");

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 || s_category.user_id == _request_user.id {
            use crate::utils::serve_category_form;

            let form = serve_category_form(payload.borrow_mut(), _request_user.id).await;
            let new_cat = NewServeCategories {
                name:            form.name.clone(),
                description:     Some(form.description.clone()),
                tech_categories: form.tech_categories,
                position:        form.position,
                count:           s_category.count,
                default_price:   form.default_price,
                user_id:         _request_user.id,
                view:            0,
                height:          0.0,
                seconds:         0,
            };
            diesel::update(&s_category)
                .set(new_cat)
                .execute(&_connection)
                .expect("E");
        }
    }
    return HttpResponse::Ok();
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ServeForm {
    pub name:             String,
    pub description:      String,
    pub position:         i16,
    pub serve_categories: i32,
    pub price:            i32,
    pub man_hours:        i16,
    pub is_default:       bool,
    pub serve_id:         Option<i32>,
}

pub async fn serve_split_payload(payload: &mut Multipart) -> ServeForm {
    let mut form: ServeForm = ServeForm {
        name:             "".to_string(),
        description:      "".to_string(),
        position:         0,
        serve_categories: 0,
        price:            0,
        man_hours:        0,
        is_default:       true,
        serve_id:         None,
    };

    let mut is_default = false;
    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();

        if name == "position" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i16 = s.parse().unwrap();
                    form.position = _int;
                }
            }
        }
        else if name == "serve_categories" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.serve_categories = _int;
                }
            }
        }
        else if name == "serve_id" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.serve_id = Some(_int);
                }
            }
        }
        else if name == "price" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.price = _int;
                }
            }
        }
        else if name == "man_hours" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i16 = s.parse().unwrap();
                    form.man_hours = _int;
                }
            }
        }
        else if name == "is_default" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    if s.to_string() == "on" {
                        is_default = true;
                    }
                }
            }
        }
        else {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "name" {
                        form.name = data_string
                    } else if field.name() == "description" {
                        form.description = data_string
                    };
                }
            }
        }
    }
    form.is_default = is_default;
    form
}

pub async fn create_serve(session: Session, mut payload: Multipart) -> impl Responder {
    use crate::schema::serve_categories::dsl::serve_categories;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let form = serve_split_payload(payload.borrow_mut()).await;
            let _cat_id = form.serve_categories.clone();
            let _category = serve_categories
                .filter(schema::serve_categories::id.eq(_cat_id))
                .first::<ServeCategories>(&_connection)
                .expect("E");

            let mut is_default = false;
            if form.is_default.clone() == true {
                is_default = true;
            };
            let _new_serve = NewServe {
                name:             form.name.clone(),
                description:      Some(form.description.clone()),
                position:         form.position,
                serve_categories: _cat_id,
                price:            form.price,
                man_hours:        form.man_hours,
                is_default:       is_default,
                user_id:          _request_user.id,
                tech_cat_id:      _category.tech_categories,
                height:           0.0,
                seconds:          0,
                serve_id:         form.serve_id,
                view:             0,
            };

            let _serve = diesel::insert_into(schema::serve::table)
                .values(&_new_serve)
                .get_result::<Serve>(&_connection)
                .expect("E.");

            if is_default == true {
                diesel::update(&_category)
                    .set(schema::serve_categories::default_price.eq(_category.default_price + _serve.price))
                    .execute(&_connection)
                    .expect("E.");
            }
            diesel::update(&_category)
                .set(schema::serve_categories::count.eq(_category.count + 1))
                .execute(&_connection)
                .expect("E.");
        }
    }
    return HttpResponse::Ok();
}

pub async fn edit_serve(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::{
        serve::dsl::serve,
        serve_categories::dsl::serve_categories,
    };

    let _serve_id: i32 = *_id;
    let _connection = establish_connection();
    let _serve = serve
        .filter(schema::serve::id.eq(&_serve_id))
        .first::<Serve>(&_connection)
        .expect("E");

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 || _serve.user_id == _request_user.id {
            let _category = serve_categories
                .filter(schema::serve_categories::id.eq(_serve.serve_categories))
                .first::<ServeCategories>(&_connection)
                .expect("E");
            let form = serve_split_payload(payload.borrow_mut()).await;

            let mut is_default = false;
            if form.is_default.clone() == true {
                is_default = true;
            };

            if _serve.is_default == true {
                // если опция дефолтная
                if is_default == false {
                    // если в форме галочка снята
                    diesel::update(&_category)
                        .set(schema::serve_categories::default_price.eq(_category.default_price - _serve.price))
                        .execute(&_connection)
                        .expect("E.");
                    }
                }
            else {
                // если опция не дефолтная
                if is_default == true {
                    // если в форме галочка поставлена
                    diesel::update(&_category)
                        .set(schema::serve_categories::default_price.eq(_category.default_price + _serve.price))
                        .execute(&_connection)
                        .expect("E.");
                }
            }

            let _new_serve = NewServe {
                name:             form.name.clone(),
                description:      Some(form.description.clone()),
                position:         form.position,
                serve_categories: _serve.serve_categories,
                price:            form.price,
                man_hours:        form.man_hours,
                is_default:       is_default,
                user_id:          _request_user.id,
                tech_cat_id:      _category.tech_categories,
                height:           0.0,
                seconds:          0,
                serve_id:         form.serve_id,
                view:             0,
            };

            diesel::update(&_serve)
                .set(_new_serve)
                .execute(&_connection)
                .expect("E");
        }
    }
    return HttpResponse::Ok();
}


pub async fn delete_serve(session: Session, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::serve::dsl::serve;
    use crate::schema::serve_categories::dsl::serve_categories;

    let _connection = establish_connection();
    let _serve_id: i32 = *_id;
    let _serve = serve
        .filter(schema::serve::id.eq(_serve_id))
        .first::<Serve>(&_connection)
        .expect("E");

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 || _serve.user_id == _request_user.id {
            let _cat_id: i32 = _serve.serve_categories;
            let _category = serve_categories
                .filter(schema::serve_categories::id.eq(_cat_id))
                .first::<ServeCategories>(&_connection)
                .expect("E");
            diesel::update(&_category)
                .set(schema::serve_categories::count.eq(&_category.count - 1))
                .get_result::<ServeCategories>(&_connection)
                .expect("Error.");

            diesel::delete(&_serve).execute(&_connection).expect("E");
        }
    }
    HttpResponse::Ok()
}

pub async fn delete_tech_category(session: Session, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::tech_categories::dsl::tech_categories;

    let _connection = establish_connection();
    let _cat_id: i32 = *_id;
    let _category = tech_categories
        .filter(schema::tech_categories::id.eq(_cat_id))
        .first::<TechCategories>(&_connection)
        .expect("E");

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 || _category.user_id == _request_user.id {
            diesel::delete(tech_categories.filter(schema::tech_categories::id.eq(_cat_id))).execute(&_connection).expect("E");
        }
    }
    HttpResponse::Ok()
}
pub async fn delete_serve_category(session: Session, _id: web::Path<i32>) -> impl Responder {

    use crate::schema::serve_categories::dsl::serve_categories;
    use crate::schema::tech_categories::dsl::tech_categories;

    let _connection = establish_connection();
    let _cat_id: i32 = *_id;
    let s_category = serve_categories
        .filter(schema::serve_categories::id.eq(_cat_id))
        .first::<ServeCategories>(&_connection)
        .expect("E");
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 || s_category.user_id == _request_user.id {
            diesel::delete(serve_categories.filter(schema::serve_categories::id.eq(_cat_id))).execute(&_connection).expect("E");

            let _category = tech_categories
                .filter(schema::tech_categories::id.eq(_cat_id))
                .first::<TechCategories>(&_connection)
                .expect("E");
            diesel::update(&_category)
                .set(schema::tech_categories::count.eq(&_category.count - 1))
                .execute(&_connection)
                .expect("E");
        }
    }
    HttpResponse::Ok()
}
