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
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        use crate::utils::get_device_and_ajax;

        let (is_desctop, is_ajax) = get_device_and_ajax(&req);
        let _request_user = get_request_user_data(&session);
        if _request_user.perm != 60 {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
        else {
            use crate::schema::serve::dsl::serve;
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
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    serve_cats:   _serve_cats,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/serve/categories.stpl")]
                struct Template {
                    request_user: User,
                    serve_cats:   Vec<ServeCategories>,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    serve_cats:   _serve_cats,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn get_serve_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        use crate::utils::get_device_and_ajax;

        let (is_desctop, is_ajax) = get_device_and_ajax(&req);
        let _request_user = get_request_user_data(&session);
        if _request_user.perm != 60 {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
        else {
            use schema::serve::dsl::serve;
            use schema::serve_categories::dsl::serve_categories;

            let _connection = establish_connection();
            let _serve_id: i32 = *_id;

            let _serves = serve
                .filter(schema::serve::id.eq(&_serve_id))
                .load::<Serve>(&_connection)
                .expect("E");
            let _serve = _serves.into_iter().nth(0).unwrap();

            let _s_categorys = serve_categories
                .filter(schema::serve_categories::id.eq(&_serve.serve_categories))
                .load::<ServeCategories>(&_connection)
                .expect("E");
            let _s_category = _s_categorys.into_iter().nth(0).unwrap();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/serve/serve.stpl")]
                struct Template {
                    request_user: User,
                    category:     ServeCategories,
                    object:       Serve,
                    is_ajax:      bool,
                }
                let body = Template {
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
                    request_user: User,
                    category:     ServeCategories,
                    object:       Serve,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
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
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        use crate::utils::get_device_and_ajax;

        let (is_desctop, is_ajax) = get_device_and_ajax(&req);
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
                    request_user: User,
                    tech_cats:    Vec<TechCategories>,
                    is_ajax:      bool,
                }
                let body = Template {
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
                    request_user: User,
                    tech_cats:    Vec<TechCategories>,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
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
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        use crate::utils::get_device_and_ajax;

        let (is_desctop, is_ajax) = get_device_and_ajax(&req);
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
                    request_user: User,
                    tech_cats:    Vec<TechCategories>,
                    serve_cats:   Vec<ServeCategories>,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    tech_cats:    _tech_categories,
                    serve_cats:   _categories,
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
                    request_user: User,
                    tech_cats:    Vec<TechCategories>,
                    serve_cats:   Vec<ServeCategories>,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    tech_cats:    _tech_categories,
                    serve_cats:   _categories,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}
pub async fn create_serve_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        use crate::utils::get_device_and_ajax;

        let (is_desctop, is_ajax) = get_device_and_ajax(&req);
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
                    request_user: User,
                    tech_cats:    Vec<TechCategories>,
                    serve_cats:   Vec<ServeCategories>,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    tech_cats:    _tech_categories,
                    serve_cats:   _categories,
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
                    request_user: User,
                    tech_cats:    Vec<TechCategories>,
                    serve_cats:   Vec<ServeCategories>,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    tech_cats:    _tech_categories,
                    serve_cats:   _categories,
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
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        use crate::utils::get_device_and_ajax;
        use crate::schema::tech_categories::dsl::tech_categories;

        let (is_desctop, is_ajax) = get_device_and_ajax(&req);
        let _request_user = get_request_user_data(&session);
        let _cat_id: i32 = *_id;
        let _connection = establish_connection();
        let _categorys = tech_categories.filter(schema::tech_categories::id.eq(&_cat_id)).load::<TechCategories>(&_connection).expect("E");
        let _category = _categorys.into_iter().nth(0).unwrap();

        if _category.user_id != _request_user.id {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
        else {
            let _connection = establish_connection();
            let _tech_categories = tech_categories.load::<TechCategories>(&_connection).expect("E");

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/serve/edit_tech_category.stpl")]
                struct Template {
                    request_user: User,
                    tech_cats:    Vec<TechCategories>,
                    category:     TechCategories,
                    is_ajax:      bool,
                }
                let body = Template {
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
                    request_user: User,
                    tech_cats:    Vec<TechCategories>,
                    category:     TechCategories,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
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
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        use crate::utils::get_device_and_ajax;
        use crate::schema::serve_categories::dsl::serve_categories;
        use crate::schema::tech_categories::dsl::tech_categories;

        let (is_desctop, is_ajax) = get_device_and_ajax(&req);
        let _request_user = get_request_user_data(&session);
        let _cat_id: i32 = *_id;
        let _connection = establish_connection();

        let _categorys = serve_categories.filter(schema::serve_categories::id.eq(&_cat_id)).load::<ServeCategories>(&_connection).expect("E");
        let _category = _categorys.into_iter().nth(0).unwrap();
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
                    request_user: User,
                    tech_cats:    Vec<TechCategories>,
                    serve_cats:   Vec<ServeCategories>,
                    category:     ServeCategories,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    tech_cats:    _tech_categories,
                    serve_cats:   _categories,
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
                    request_user: User,
                    tech_cats:    Vec<TechCategories>,
                    serve_cats:   Vec<ServeCategories>,
                    category:     ServeCategories,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    tech_cats:    _tech_categories,
                    serve_cats:   _categories,
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
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        use crate::utils::get_device_and_ajax;
        use crate::schema::{
            serve::dsl::serve,
            serve_categories::dsl::serve_categories,
            tech_categories::dsl::tech_categories,
        };

        let _connection = establish_connection();
        let (is_desctop, is_ajax) = get_device_and_ajax(&req);

        let _request_user = get_request_user_data(&session);
        let _cat_id: i32 = *_id;
        let all_tech_categories :Vec<TechCategories> = tech_categories
            .order(schema::tech_categories::position.asc())
            .load(&_connection)
            .expect("E.");
        let _serve_id: i32 = *_id;
        let _serves = serve.filter(schema::serve::id.eq(&_serve_id)).load::<Serve>(&_connection).expect("E");
        let _serve = _serves.into_iter().nth(0).unwrap();
        let _serve_cats:Vec<ServeCategories> = serve_categories.load(&_connection).expect("E");

        if _serve.user_id != _request_user.id {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
        else {
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/serve/edit_serve.stpl")]
                struct Template {
                    request_user: User,
                    tech_cats:    Vec<TechCategories>,
                    serve_cats:   Vec<ServeCategories>,
                    object:       Serve,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    tech_cats:    all_tech_categories,
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
                    request_user: User,
                    tech_cats:    Vec<TechCategories>,
                    serve_cats:   Vec<ServeCategories>,
                    object:       Serve,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    tech_cats:    all_tech_categories,
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
                name: form.name.clone(),
                description: Some(form.description.clone()),
                position: form.position,
                count: 0,
                user_id: _request_user.id,
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

            use schema::serve_categories::dsl::serve_categories;
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
                default_price: Some(0),
                user_id: _request_user.id,
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
    use crate::schema::tech_categories::dsl::tech_categories;

    let _connection = establish_connection();
    let _cat_id: i32 = *_id;
    let _categorys = tech_categories.filter(schema::tech_categories::id.eq(_cat_id)).load::<TechCategories>(&_connection).expect("E");
    let _category = _categorys.into_iter().nth(0).unwrap();

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 && _category.user_id == _request_user.id {

            let form = category_form(payload.borrow_mut(), _request_user.id).await;
            let new_cat = NewTechCategories {
                name: form.name.clone(),
                description: Some(form.description.clone()),
                position: form.position,
                count: _category.count,
                user_id: _request_user.id,
            };
            diesel::update(&_category)
                .set(new_cat)
                .get_result::<TechCategories>(&_connection)
                .expect("E");
        }
    }
    return HttpResponse::Ok();
}

pub async fn edit_serve_category(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::serve_categories::dsl::serve_categories;
    use crate::schema::tech_categories::dsl::tech_categories;

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
                default_price: Some(form.default_price),
                user_id: _request_user.id,
            };
            diesel::update(&s_category)
                .set(new_cat)
                .get_result::<ServeCategories>(&_connection)
                .expect("E");
        }
    }
    return HttpResponse::Ok();
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ServeForm {
    pub name: String,
    pub cat_name: String,
    pub description: String,
    pub position: i32,
    pub serve_categories: i32,
    pub price: i32,
    pub price_acc: i32,
    pub social_price: i32,
    pub man_hours: i32,
    pub is_default: bool,
}

pub async fn serve_split_payload(payload: &mut Multipart) -> ServeForm {
    let mut form: ServeForm = ServeForm {
        name: "".to_string(),
        cat_name: "".to_string(),
        description: "".to_string(),
        position: 0,
        serve_categories: 0,
        price: 0,
        price_acc: 0,
        social_price: 0,
        man_hours: 0,
        is_default: true,
    };

    let mut is_default = false;
    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();

        if name == "position" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
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
        else if name == "price" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.price = _int;
                }
            }
        }
        else if name == "price_acc" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.price_acc = _int;
                }
            }
        }
        else if name == "social_price" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.social_price = _int;
                }
            }
        }
        else if name == "man_hours" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
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
                name: form.name.clone(),
                cat_name: _category[0].name.clone(),
                description: Some(form.description.clone()),
                position: form.position,
                serve_categories: _cat_id,
                price: Some(form.price),
                price_acc: Some(form.price_acc),
                social_price: Some(form.social_price),
                man_hours: Some(form.man_hours),
                is_default: is_default,
                user_id: _request_user.id,
                tech_cat_id: _category[0].tech_cat_id;
            };

            let _serve = diesel::insert_into(schema::serve::table)
                .values(&_new_serve)
                .get_result::<Serve>(&_connection)
                .expect("E.");

            if is_default == true && _category[0].default_price.is_some() && _serve.price.is_some() {
                diesel::update(&_category[0])
                .set(schema::serve_categories::default_price.eq(_category[0].default_price.unwrap() + _serve.price.unwrap()))
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
                if is_default == false && _category[0].default_price.is_some() && _serve.price.is_some() {
                    // если в форме галочка снята
                    diesel::update(&_category[0])
                        .set(schema::serve_categories::default_price.eq(_category[0].default_price.unwrap() - _serve.price.unwrap()))
                        .get_result::<ServeCategories>(&_connection)
                        .expect("E.");
                    }
                }
            else {
                // если опция не дефолтная
                if is_default == true  && _category[0].default_price.is_some() && _serve.price.is_some(){
                    // если в форме галочка поставлена
                    diesel::update(&_category[0])
                        .set(schema::serve_categories::default_price.eq(_category[0].default_price.unwrap() + _serve.price.unwrap()))
                        .get_result::<ServeCategories>(&_connection)
                        .expect("E.");
                }
            }

            let _new_serve = NewServe {
                name: form.name.clone(),
                cat_name: _category[0].name.clone(),
                description: Some(form.description.clone()),
                position: form.position,
                serve_categories: form.serve_categories,
                price: Some(form.price),
                price_acc: Some(form.price_acc),
                social_price: Some(form.social_price),
                man_hours: Some(form.man_hours),
                is_default: is_default,
                user_id: _request_user.id,
                tech_cat_id: _category[0].tech_cat_id;
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
