use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
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
    category_form,
    serve_category_form,
    establish_connection,
    is_signed_in,
    get_request_user_data,
    get_first_load_page,
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

    if is_ajax == 0 {
        get_first_load_page(&session, is_desctop, "Технологии услуг и опций".to_string()).await
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
            let _serve_cats: Vec<ServeCategories> = serve_categories
                .load(&_connection)
                .expect("E");

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/serve/categories.stpl")]
                struct Template {
                    request_user: User,
                    serve_cats:   Vec<ServeCategories>,
                    is_ajax:      i32,
                    title:        String,
                }
                let body = Template {
                    request_user: _request_user,
                    serve_cats:   _serve_cats,
                    is_ajax:      is_ajax,
                    title:        "Технологии услуг и опций".to_string(),
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/serve/categories.stpl")]
                struct Template {
                    //request_user: User,
                    serve_cats:   Vec<ServeCategories>,
                    is_ajax:      i32,
                    title:        String,
                }
                let body = Template {
                    //request_user: _request_user,
                    serve_cats:   _serve_cats,
                    is_ajax:      is_ajax,
                    title:        "Технологии услуг и опций".to_string(),
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

    let _connection = establish_connection();
    let _serve_id: i32 = *_id;

    let _serves = serve
        .filter(schema::serve::id.eq(&_serve_id))
        .load::<Serve>(&_connection)
        .expect("E");
    let _serve = _serves.into_iter().nth(0).unwrap();

    if is_ajax == 0 {
        get_first_load_page(&session, is_desctop, "Опция услуг ".to_string() + &_serve.name).await
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

            let _s_categorys = serve_categories
                .filter(schema::serve_categories::id.eq(&_serve.serve_categories))
                .load::<ServeCategories>(&_connection)
                .expect("E");
            let _s_category = _s_categorys.into_iter().nth(0).unwrap();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/serve/serve.stpl")]
                struct Template {
                    //title:        String,
                    request_user: User,
                    category:     ServeCategories,
                    object:       Serve,
                    is_ajax:      i32,
                }
                let body = Template {
                    //title:        "Опция услуг ".to_string() + &_serve.name,
                    request_user: _request_user,
                    category:     _s_category,
                    object:       _serve,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/serve/serve.stpl")]
                struct Template {
                    //title:        String,
                    //request_user: User,
                    category:     ServeCategories,
                    object:       Serve,
                    is_ajax:      i32,
                }
                let body = Template {
                    //title:        "Опция услуг ".to_string() + &_serve.name,
                    //request_user: _request_user,
                    category:     _s_category,
                    object:       _serve,
                    is_ajax:      is_ajax,
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
    if is_ajax == 0 {
        get_first_load_page(&session, is_desctop, "Создание категории услуг".to_string()).await
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
            let _categories = tech_categories.load::<TechCategories>(&_connection).expect("E");

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/serve/create_tech_categories.stpl")]
                struct Template {
                    title:        String,
                    request_user: User,
                    tech_cats:    Vec<TechCategories>,
                    is_ajax:      i32,
                }
                let body = Template {
                    title:        "Создание категории услуг".to_string(),
                    request_user: _request_user,
                    tech_cats:    _categories,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/serve/create_tech_categories.stpl")]
                struct Template {
                    title:        String,
                    //request_user: User,
                    tech_cats:    Vec<TechCategories>,
                    is_ajax:      i32,
                }
                let body = Template {
                    title:        "Создание категории услуг".to_string(),
                    //request_user: _request_user,
                    tech_cats:    _categories,
                    is_ajax:      is_ajax,
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

    if is_ajax == 0 {
        get_first_load_page(&session, is_desctop, "Создание технологии услуг".to_string()).await
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
            use schema::tech_categories::dsl::tech_categories;

            let _connection = establish_connection();
            let _tech_categories = tech_categories.load::<TechCategories>(&_connection).expect("E");
            let _categories = serve_categories.load::<ServeCategories>(&_connection).expect("E");

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/serve/create_serve_categories.stpl")]
                struct Template {
                    title:        String,
                    request_user: User,
                    tech_cats:    Vec<TechCategories>,
                    is_ajax:      i32,
                }
                let body = Template {
                    title:        "Создание технологии услуг".to_string(),
                    request_user: _request_user,
                    tech_cats:    _tech_categories,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/serve/create_serve_categories.stpl")]
                struct Template {
                    title:        String,
                    tech_cats:    Vec<TechCategories>,
                    is_ajax:      i32,
                }
                let body = Template {
                    title:        "Создание технологии услуг".to_string(),
                    tech_cats:    _tech_categories,
                    is_ajax:      is_ajax,
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
        let _request_user = get_request_user_data(&session);
        if _request_user.perm != 60 {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/serve/load_serve_categories.stpl")]
            struct Template {
                serve_cats: Vec<ServeCategories>,
            }
            let body = Template {
                serve_cats: ServeCategories::get_categories_from_level(&*level),
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}
pub async fn load_form_from_level(session: Session, level: web::Path<i16>) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm != 60 {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
        else {
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
                tech_cats: Vec<TechCategories>,
            }
            let body = Template {
                tech_cats: _tech_categories,
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

    if is_ajax == 0 {
        get_first_load_page(&session, is_desctop, "Создание опции услуг".to_string()).await
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
            use crate::schema::{
                serve_categories::dsl::serve_categories,
                tech_categories::dsl::tech_categories,
            };

            let _connection = establish_connection();
            let _tech_categories = tech_categories.load::<TechCategories>(&_connection).expect("E");
            let _categories = serve_categories.load::<ServeCategories>(&_connection).expect("E");

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/serve/create_serve.stpl")]
                struct Template {
                    title:        String,
                    request_user: User,
                    is_ajax:      i32,
                }
                let body = Template {
                    title:        "Создание опции услуг".to_string(),
                    request_user: _request_user,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/serve/create_serve.stpl")]
                struct Template {
                    title:        String,
                    //request_user: User,
                    is_ajax:      i32,
                }
                let body = Template {
                    title:        "Создание опции услуг".to_string(),
                    //request_user: _request_user,
                    is_ajax:      is_ajax,
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
    let _categorys = tech_categories.filter(schema::tech_categories::id.eq(&_cat_id)).load::<TechCategories>(&_connection).expect("E");
    let _category = _categorys.into_iter().nth(0).unwrap();
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    if is_ajax == 0 {
        get_first_load_page(&session, is_desctop, "Изменение категории услуг ".to_string() + &_category.name).await
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
                    title:        String,
                    request_user: User,
                    tech_cats:    Vec<TechCategories>,
                    category:     TechCategories,
                    is_ajax:      i32,
                }
                let body = Template {
                    title:        "Изменение категории услуг ".to_string() + &_category.name,
                    request_user: _request_user,
                    tech_cats:    _tech_categories,
                    category:     _category,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/serve/edit_tech_category.stpl")]
                struct Template {
                    title:        String,
                    //request_user: User,
                    tech_cats:    Vec<TechCategories>,
                    category:     TechCategories,
                    is_ajax:      i32,
                }
                let body = Template {
                    title:        "Изменение категории услуг ".to_string() + &_category.name,
                    //request_user: _request_user,
                    tech_cats:    _tech_categories,
                    category:     _category,
                    is_ajax:      is_ajax,
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
    let _categorys = serve_categories.filter(schema::serve_categories::id.eq(&_cat_id)).load::<ServeCategories>(&_connection).expect("E");
    let _category = _categorys.into_iter().nth(0).unwrap();
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    if is_ajax == 0 {
        get_first_load_page(&session, is_desctop, "Изменение технологии услуг ".to_string() + &_category.name).await
    }
    else if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        use crate::schema::tech_categories::dsl::tech_categories;

        let _request_user = get_request_user_data(&session);
        let _categories = serve_categories.load::<ServeCategories>(&_connection).expect("E");
        let _tech_categories = tech_categories.load::<TechCategories>(&_connection).expect("E");

        if _category.user_id != _request_user.id {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
        else {
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/serve/edit_serve_category.stpl")]
                struct Template {
                    title:        String,
                    request_user: User,
                    tech_cats:    Vec<TechCategories>,
                    category:     ServeCategories,
                    is_ajax:      i32,
                }
                let body = Template {
                    title:        "Изменение технологии услуг ".to_string() + &_category.name,
                    request_user: _request_user,
                    tech_cats:    _tech_categories,
                    category:     _category,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/serve/edit_serve_category.stpl")]
                struct Template {
                    title:        String,
                    tech_cats:    Vec<TechCategories>,
                    category:     ServeCategories,
                    is_ajax:      i32,
                }
                let body = Template {
                    title:        "Изменение технологии услуг ".to_string() + &_category.name,
                    tech_cats:    _tech_categories,
                    category:     _category,
                    is_ajax:      is_ajax,
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
    let _serves = serve.filter(schema::serve::id.eq(&_serve_id)).load::<Serve>(&_connection).expect("E");
    let _serve = _serves.into_iter().nth(0).unwrap();

    if is_ajax == 0 {
        get_first_load_page(&session, is_desctop, "Изменение опции услуг ".to_string() + &_serve.name).await
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
            .load::<ServeCategories>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
        let _tech_category = tech_categories
            .filter(schema::tech_categories::id.eq(_serve_cat.tech_categories))
            .load::<TechCategories>(&_connection)
            .expect("E.")
            .into_iter()
            .nth(0)
            .unwrap();

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
                    title:        String,
                    request_user: User,
                    level:        i16,
                    serve_cats:   Vec<ServeCategories>,
                    object:       Serve,
                    is_ajax:      i32,
                }
                let body = Template {
                    title:        "Изменение опции услуг ".to_string() + &_serve.name,
                    request_user: _request_user,
                    level:        _level,
                    serve_cats:   _serve_cats,
                    object:       _serve,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/serve/edit_serve.stpl")]
                struct Template {
                    title:        String,
                    //request_user: User,
                    level:        i16,
                    serve_cats:   Vec<ServeCategories>,
                    object:       Serve,
                    is_ajax:      i32,
                }
                let body = Template {
                    title:        "Изменение опции услуг ".to_string() + &_serve.name,
                    //request_user: _request_user,
                    level:        _level,
                    serve_cats:   _serve_cats,
                    object:       _serve,
                    is_ajax:      is_ajax,
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
                .get_result::<TechCategories>(&_connection)
                .expect("E.");
        }
    }
    return HttpResponse::Ok();
}

pub async fn create_serve_categories(session: Session, mut payload: Multipart) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use schema::tech_categories::dsl::tech_categories;

            let _connection = establish_connection();
            let form = serve_category_form(payload.borrow_mut(), _request_user.id).await;
            let _s_category = tech_categories
                .filter(schema::tech_categories::id.eq(form.tech_categories))
                .load::<TechCategories>(&_connection).expect("E");

            let new_cat = NewServeCategories {
                name: form.name.clone(),
                description: Some(form.description.clone()),
                cat_name: _s_category[0].name.clone(),
                tech_categories: form.tech_categories,
                position: form.position,
                count: 0,
                default_price: 0,
                user_id: _request_user.id,
                view:        0,
                height:      0.0,
                seconds:     0,
            };
            let _new_serve = diesel::insert_into(schema::serve_categories::table)
                .values(&new_cat)
                .get_result::<ServeCategories>(&_connection)
                .expect("E.");
        }
    }
    return HttpResponse::Ok();
}

pub async fn edit_tech_category(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::{
        tech_categories::dsl::tech_categories,
        serve_categories::dsl::serve_categories,
    };

    let _connection = establish_connection();
    let _cat_id: i32 = *_id;
    let _categorys = tech_categories.filter(schema::tech_categories::id.eq(_cat_id)).load::<TechCategories>(&_connection).expect("E");
    let _category = _categorys.into_iter().nth(0).unwrap();

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 && _category.user_id == _request_user.id {

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
                .get_result::<TechCategories>(&_connection)
                .expect("E");
        }
        let _serve_cats = serve_categories
            .filter(schema::serve_categories::tech_categories.eq(_cat_id))
            .load::<ServeCategories>(&_connection)
            .expect("E");

        for _cat in _serve_cats.iter() {
            diesel::update(_cat)
                .set(schema::serve_categories::cat_name.eq(_category.name.clone()))
                .get_result::<ServeCategories>(&_connection)
                .expect("Error.");
        };
    }
    return HttpResponse::Ok();
}

pub async fn edit_serve_category(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::{
        serve_categories::dsl::serve_categories,
        serve::dsl::serve,
        tech_categories::dsl::tech_categories,
    };

    let _connection = establish_connection();
    let _cat_id: i32 = *_id;

    let s_categorys = serve_categories
        .filter(schema::serve_categories::id.eq(_cat_id))
        .load::<ServeCategories>(&_connection)
        .expect("E");

    let s_category = s_categorys.into_iter().nth(0).unwrap();

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 && s_category.user_id == _request_user.id {
            let t_category = tech_categories
                .filter(schema::tech_categories::id.eq(s_category.tech_categories))
                .load::<TechCategories>(&_connection)
                .expect("E");

            let form = serve_category_form(payload.borrow_mut(), _request_user.id).await;
            let new_cat = NewServeCategories {
                name: form.name.clone(),
                description: Some(form.description.clone()),
                cat_name: t_category[0].name.clone(),
                tech_categories: form.tech_categories,
                position: form.position,
                count: s_category.count,
                default_price: form.default_price,
                user_id: _request_user.id,
                view:        0,
                height:      0.0,
                seconds:     0,
            };
            diesel::update(&s_category)
                .set(new_cat)
                .get_result::<ServeCategories>(&_connection)
                .expect("E");
        }
        let _serves = serve
            .filter(schema::serve::serve_categories.eq(_cat_id))
            .load::<Serve>(&_connection)
            .expect("E");

        for _serve in _serves.iter() {
            diesel::update(_serve)
                .set(schema::serve::cat_name.eq(s_category.name.clone()))
                .get_result::<Serve>(&_connection)
                .expect("Error.");
        };
    }
    return HttpResponse::Ok();
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ServeForm {
    pub name:             String,
    pub cat_name:         String,
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
        cat_name:         "".to_string(),
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
                    } else if field.name() == "cat_name" {
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
            let _category = serve_categories.filter(schema::serve_categories::id.eq(_cat_id)).load::<ServeCategories>(&_connection).expect("E");

            let mut is_default = false;
            if form.is_default.clone() == true {
                is_default = true;
            };
            let _new_serve = NewServe {
                name:             form.name.clone(),
                cat_name:         _category[0].name.clone(),
                description:      Some(form.description.clone()),
                position:         form.position,
                serve_categories: _cat_id,
                price:            form.price,
                man_hours:        form.man_hours,
                is_default:       is_default,
                user_id:          _request_user.id,
                tech_cat_id:      _category[0].tech_categories,
                view:             0,
                height:           0.0,
                seconds:          0,
                serve_id:         form.serve_id,
            };

            let _serve = diesel::insert_into(schema::serve::table)
                .values(&_new_serve)
                .get_result::<Serve>(&_connection)
                .expect("E.");

            if is_default == true {
                diesel::update(&_category[0])
                .set(schema::serve_categories::default_price.eq(_category[0].default_price + _serve.price))
                .get_result::<ServeCategories>(&_connection)
                .expect("E.");
            }
            diesel::update(&_category[0])
                .set(schema::serve_categories::count.eq(_category[0].count + 1))
                .get_result::<ServeCategories>(&_connection)
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
    let _serves = serve.filter(schema::serve::id.eq(&_serve_id)).load::<Serve>(&_connection).expect("E");
    let _serve = _serves.into_iter().nth(0).unwrap();

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 && _serve.user_id == _request_user.id {
            let _category = serve_categories.filter(schema::serve_categories::id.eq(_serve.serve_categories)).load::<ServeCategories>(&_connection).expect("E");
            let form = serve_split_payload(payload.borrow_mut()).await;

            let mut is_default = false;
            if form.is_default.clone() == true {
                is_default = true;
            };

            if _serve.is_default == true {
                // если опция дефолтная
                if is_default == false {
                    // если в форме галочка снята
                    diesel::update(&_category[0])
                        .set(schema::serve_categories::default_price.eq(_category[0].default_price - _serve.price))
                        .get_result::<ServeCategories>(&_connection)
                        .expect("E.");
                    }
                }
            else {
                // если опция не дефолтная
                if is_default == true {
                    // если в форме галочка поставлена
                    diesel::update(&_category[0])
                        .set(schema::serve_categories::default_price.eq(_category[0].default_price + _serve.price))
                        .get_result::<ServeCategories>(&_connection)
                        .expect("E.");
                }
            }

            let _new_serve = NewServe {
                name:             form.name.clone(),
                cat_name:         _category[0].name.clone(),
                description:      Some(form.description.clone()),
                position:         form.position,
                serve_categories: _serve.serve_categories,
                price:            form.price,
                man_hours:        form.man_hours,
                is_default:       is_default,
                user_id:          _request_user.id,
                tech_cat_id:      _category[0].tech_categories,
                view:             0,
                height:           0.0,
                seconds:          0,
                serve_id:         form.serve_id,
            };

            diesel::update(&_serve)
                .set(_new_serve)
                .get_result::<Serve>(&_connection)
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
    let _serves = serve.filter(schema::serve::id.eq(_serve_id)).load::<Serve>(&_connection).expect("E");
    let _serve = _serves.into_iter().nth(0).unwrap();

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 && _serve.user_id == _request_user.id {
            let _cat_id: i32 = _serve.serve_categories;
            let _category = serve_categories
                .filter(schema::serve_categories::id.eq(_cat_id))
                .load::<ServeCategories>(&_connection)
                .expect("E");
            diesel::update(&_category[0])
                .set(schema::serve_categories::count.eq(&_category[0].count - 1))
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
    let _categorys = tech_categories.filter(schema::tech_categories::id.eq(_cat_id)).load::<TechCategories>(&_connection).expect("E");
    let _category = _categorys.into_iter().nth(0).unwrap();

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 && _category.user_id == _request_user.id {
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
    let s_categories = serve_categories.filter(schema::serve_categories::id.eq(_cat_id)).load::<ServeCategories>(&_connection).expect("E");
    let s_category = s_categories.into_iter().nth(0).unwrap();
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 && s_category.user_id == _request_user.id {
            diesel::delete(serve_categories.filter(schema::serve_categories::id.eq(_cat_id))).execute(&_connection).expect("E");

            let _category = tech_categories
                .filter(schema::tech_categories::id.eq(_cat_id))
                .load::<TechCategories>(&_connection)
                .expect("E");
            diesel::update(&_category[0])
                .set(schema::tech_categories::count.eq(&_category[0].count - 1))
                .get_result::<TechCategories>(&_connection)
                .expect("E");
        }
    }
    HttpResponse::Ok()
}
