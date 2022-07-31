use actix_web::{
    web,
    HttpRequest,
    HttpResponse,
    Responder,
    error::InternalError,
    http::StatusCode,
};
use actix_multipart::Multipart;
use std::borrow::BorrowMut;
use crate::utils::{
    category_form,
    establish_connection,
    is_signed_in,
    get_request_user_data,
};
use actix_session::Session;
use crate::schema;
use crate::diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use crate::models::User;
use crate::models::{
    ServiceCategories,
    NewServiceCategories,
    Service,
    NewService,
    ServiceCategory,
    NewServiceCategory,
    ServiceImage,
    NewServiceImage,
    ServiceVideo,
    NewServiceVideo,
    TagItems,
    NewTagItems,
    Tag,
};
use sailfish::TemplateOnce;


pub fn service_routes(config: &mut web::ServiceConfig) {
    config.route("/service_categories/", web::get().to(service_categories_page));
    config.service(web::resource("/create_service_categories/")
        .route(web::get().to(create_service_categories_page))
        .route(web::post().to(create_service_categories))
    );
    config.service(web::resource("/edit_service_category/{id}/")
        .route(web::get().to(edit_service_category_page))
        .route(web::post().to(edit_service_category))
    );
    config.service(web::resource("/create_service/")
        .route(web::get().to(create_service_page))
        .route(web::post().to(create_service))
    );
    config.service(web::resource("/edit_service/{id}/")
        .route(web::get().to(edit_service_page))
        .route(web::post().to(edit_service))
    );
    config.service(web::resource("/edit_content_service/{id}/")
        .route(web::get().to(edit_content_service_page))
        .route(web::post().to(edit_content_service))
    );
    config.route("/delete_service/{id}/", web::get().to(delete_service));
    config.route("/delete_service_category/{id}/", web::get().to(delete_service_category));
    config.service(web::resource("/service/{cat_id}/{service_id}/").route(web::get().to(get_service_page)));
    config.service(web::resource("/services/{id}/").route(web::get().to(service_category_page)));
}

pub async fn create_service_categories_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use schema::service_categories::dsl::service_categories;
            use crate::utils::get_device_and_ajax;

            let _connection = establish_connection();
            let _service_cats:Vec<ServiceCategories> = service_categories
                .load(&_connection)
                .expect("Error");

            let (is_desctop, is_ajax) = get_device_and_ajax(&req);

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/services/create_categories.stpl")]
                struct Template {
                    request_user: User,
                    service_cats: Vec<ServiceCategories>,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    service_cats: _service_cats,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/services/create_categories.stpl")]
                struct Template {
                    request_user: User,
                    service_cats: Vec<ServiceCategories>,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    service_cats: _service_cats,
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

pub async fn create_service_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use schema::{
                tags::dsl::tags,
                tech_categories::dsl::tech_categories,
                service_categories::dsl::service_categories,
            };
            use crate::utils::get_device_and_ajax;
            use crate::models::TechCategories;

            let _connection = establish_connection();
            let _service_cats:Vec<ServiceCategories> = service_categories
                .load(&_connection)
                .expect("Error");

            let all_tags: Vec<Tag> = tags
                .load(&_connection)
                .expect("Error.");
            let _tech_categories = tech_categories
                .load::<TechCategories>(&_connection)
                .expect("E");

            let (is_desctop, is_ajax) = get_device_and_ajax(&req);

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/services/create_service.stpl")]
                struct Template {
                    request_user: User,
                    service_cats: Vec<ServiceCategories>,
                    tech_cats:    Vec<TechCategories>,
                    all_tags:     Vec<Tag>,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    service_cats: _service_cats,
                    tech_cats:    _tech_categories,
                    all_tags:     all_tags,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/services/create_service.stpl")]
                struct Template {
                    request_user: User,
                    service_cats: Vec<ServiceCategories>,
                    tech_cats:    Vec<TechCategories>,
                    all_tags:     Vec<Tag>,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    service_cats: _service_cats,
                    tech_cats:    _tech_categories,
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
    else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied."))
    }
}
pub async fn edit_service_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use schema::services::dsl::services;

    let _service_id: i32 = *_id;
    let _connection = establish_connection();
    let _services = services.filter(schema::services::id.eq(&_service_id)).load::<Service>(&_connection).expect("E");
    let _service = _services.into_iter().nth(0).unwrap();

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 && _service.user_id == _request_user.id {
            use schema::{
                tags::dsl::tags,
                service_images::dsl::service_images,
                service_videos::dsl::service_videos,
                service_categories::dsl::service_categories,
                tech_categories::dsl::tech_categories,
            };
            use crate::utils::get_device_and_ajax;
            use crate::models:: TechCategories;

            let (is_desctop, is_ajax) = get_device_and_ajax(&req);
            let _categories = _service.get_categories();
            let _all_tags: Vec<Tag> = tags.load(&_connection).expect("Error.");
            let _service_tags = _service.get_tags();

            let _images = service_images.filter(schema::service_images::service.eq(_service.id)).load::<ServiceImage>(&_connection).expect("E");
            let _videos = service_videos.filter(schema::service_videos::service.eq(_service.id)).load::<ServiceVideo>(&_connection).expect("E");

            let _service_cats = service_categories
                .load::<ServiceCategories>(&_connection)
                .expect("Error");

            let _tech_categories = tech_categories
                .load::<TechCategories>(&_connection)
                .expect("E");

            let level = _tech_categories[0].level;
            let _tech_categories = tech_categories
                .filter(schema::tech_categories::level.eq(level))
                .load::<TechCategories>(&_connection)
                .expect("E");

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/services/edit_service.stpl")]
                struct Template {
                    request_user: User,
                    object:       Service,
                    categories:   Vec<ServiceCategories>,
                    is_ajax:      bool,
                    images:       Vec<ServiceImage>,
                    videos:       Vec<ServiceVideo>,
                    all_tags:     Vec<Tag>,
                    service_tags: Vec<Tag>,
                    service_cats: Vec<ServiceCategories>,
                    tech_cats:    Vec<TechCategories>,
                    level:        i16,
                }
                let body = Template {
                    request_user: _request_user,
                    object:       _service,
                    categories:   _categories,
                    is_ajax:      is_ajax,
                    images:       _images,
                    videos:       _videos,
                    all_tags:     _all_tags,
                    service_tags: _service_tags,
                    service_cats: _service_cats,
                    tech_cats:    _tech_categories,
                    level:        level,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/services/edit_service.stpl")]
                struct Template {
                    request_user: User,
                    object:       Service,
                    categories:   Vec<ServiceCategories>,
                    is_ajax:      bool,
                    images:       Vec<ServiceImage>,
                    videos:       Vec<ServiceVideo>,
                    all_tags:     Vec<Tag>,
                    service_tags: Vec<Tag>,
                    service_cats: Vec<ServiceCategories>,
                    tech_cats:    Vec<TechCategories>,
                    level:        i16,
                }
                let body = Template {
                    request_user: _request_user,
                    object:       _service,
                    categories:   _categories,
                    is_ajax:      is_ajax,
                    images:       _images,
                    videos:       _videos,
                    all_tags:     _all_tags,
                    service_tags: _service_tags,
                    service_cats: _service_cats,
                    tech_cats:    _tech_categories,
                    level:        level,
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

pub async fn edit_content_service_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::schema::services::dsl::services;

    let _service_id: i32 = *_id;
    let _connection = establish_connection();
    let _services = services
        .filter(schema::services::id.eq(&_service_id))
        .load::<Service>(&_connection)
        .expect("E");

    let _service = _services.into_iter().nth(0).unwrap();

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 && _request_user.id == _service.user_id {
            use crate::utils::get_device_and_ajax;

            let (is_desctop, is_ajax) = get_device_and_ajax(&req);

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/services/edit_content_service.stpl")]
                struct Template {
                    request_user: User,
                    service:      Service,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    service:      _service,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/services/edit_content_service.stpl")]
                struct Template {
                    request_user: User,
                    service:      Service,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    service:      _service,
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
pub async fn edit_content_service(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::services::dsl::services;

    let _service_id: i32 = *_id;
    let _connection = establish_connection();
    let _services = services
        .filter(schema::services::id.eq(&_service_id))
        .load::<Service>(&_connection)
        .expect("E");

    let _service = _services.into_iter().nth(0).unwrap();

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 && _request_user.id == _service.user_id {
            use crate::utils::content_form;

            let form = content_form(payload.borrow_mut()).await;
            diesel::update(&_service)
            .set(schema::services::content.eq(form.content.clone()))
            .get_result::<Service>(&_connection)
            .expect("E");
        }
    }
    HttpResponse::Ok().body("")
}

pub async fn edit_service_category_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use schema::service_categories::dsl::service_categories;
            use crate::utils::get_device_and_ajax;

            let (is_desctop, is_ajax) = get_device_and_ajax(&req);

            let _cat_id: i32 = *_id;
            let _connection = establish_connection();
            let _categorys = service_categories
                .filter(schema::service_categories::id.eq(&_cat_id))
                .load::<ServiceCategories>(&_connection)
                .expect("E");

            let _category = _categorys.into_iter().nth(0).unwrap();
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/services/edit_category.stpl")]
                struct Template {
                    request_user: User,
                    category:     ServiceCategories,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    category:     _category,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/services/edit_category.stpl")]
                struct Template {
                    request_user: User,
                    category:     ServiceCategories,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    category:     _category,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied"))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied"))
    }
}

pub async fn create_service_categories(session: Session, mut payload: Multipart) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let form = category_form(payload.borrow_mut(), _request_user.id).await;
            let new_cat = NewServiceCategories {
                name:        form.name.clone(),
                description: Some(form.description.clone()),
                position:    form.position,
                image:       Some(form.image.clone()),
                count:       0
            };
            let _new_service = diesel::insert_into(schema::service_categories::table)
                .values(&new_cat)
                .get_result::<ServiceCategories>(&_connection)
                .expect("E.");
        }
    }
    return HttpResponse::Ok();
}

pub async fn create_service(session: Session, mut payload: Multipart) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use crate::schema::{
                tags::dsl::tags,
                serve::dsl::serve,
                service_categories::dsl::service_categories,
            };
            use crate::models::{
                TechCategoriesItem,
                NewTechCategoriesItem,
                Serve,
                ServeItems,
                NewServeItems,
            };
            use crate::utils::store_form;

            let _connection = establish_connection();

            let form = store_form(payload.borrow_mut(), _request_user.id).await;
            let new_service = NewService::from_service_form (
                form.title.clone(),
                form.description.clone(),
                form.link.clone(),
                form.main_image.clone(),
                form.is_active.clone(),
                0,
                _request_user.id,
                form.position,
            );

            let _service = diesel::insert_into(schema::services::table)
                .values(&new_service)
                .get_result::<Service>(&_connection)
                .expect("E.");

            for image in form.images.iter() {
                let new_image = NewServiceImage::from_service_images_form (
                    _service.id,
                    image.to_string()
                );
                diesel::insert_into(schema::service_images::table)
                    .values(&new_image)
                    .get_result::<ServiceImage>(&_connection)
                    .expect("E.");
                };
            for video in form.videos.iter() {
                let new_video = NewServiceVideo::from_service_videos_form (
                    _service.id,
                    video.to_string()
                );
                diesel::insert_into(schema::service_videos::table)
                    .values(&new_video)
                    .get_result::<ServiceVideo>(&_connection)
                    .expect("E.");
            };
            for category_id in form.category_list.iter() {
                let new_category = NewServiceCategory {
                    service_categories_id: *category_id,
                    service_id: _service.id
                };
                diesel::insert_into(schema::service_category::table)
                    .values(&new_category)
                    .get_result::<ServiceCategory>(&_connection)
                    .expect("E.");

                let _category = service_categories.filter(schema::service_categories::id.eq(category_id)).load::<ServiceCategories>(&_connection).expect("E");
                diesel::update(&_category[0])
                    .set(schema::service_categories::count.eq(_category[0].count + 1))
                    .get_result::<ServiceCategories>(&_connection)
                    .expect("Error.");
            };
            for tag_id in form.tags_list.iter() {
                let new_tag = NewTagItems {
                    tag_id: *tag_id,
                    service_id: _service.id,
                    store_id: 0,
                    blog_id: 0,
                    wiki_id: 0,
                    work_id: 0,
                    created: chrono::Local::now().naive_utc(),
                };
                diesel::insert_into(schema::tags_items::table)
                    .values(&new_tag)
                    .get_result::<TagItems>(&_connection)
                    .expect("Error.");

                let _tag = tags.filter(schema::tags::id.eq(tag_id)).load::<Tag>(&_connection).expect("E");
                diesel::update(&_tag[0])
                    .set((schema::tags::count.eq(_tag[0].count + 1), schema::tags::service_count.eq(_tag[0].service_count + 1)))
                    .get_result::<Tag>(&_connection)
                    .expect("Error.");
            }

            // создаем связь с тех категориями, которые будут
            // расширять списки опций, предлагая доп возможности и услуги
            for cat_id in form.close_tech_cats_list.iter() {
                let new_cat = NewTechCategoriesItem {
                    category_id: *cat_id,
                    service_id:  _service.id,
                    store_id:    0,
                    work_id:     0,
                    types:       2,
                };
                diesel::insert_into(schema::tech_categories_items::table)
                    .values(&new_cat)
                    .get_result::<TechCategoriesItem>(&_connection)
                    .expect("Error.");
            }

            // создаем опции услуги и записываем id опций в вектор.
            let mut serve_ids = Vec::new();
            for serve_id in form.serve_list.iter() {
                let new_serve_form = NewServeItems {
                    serve_id:   *serve_id,
                    service_id: _service.id,
                    store_id:   0,
                    work_id:    0,
                };
                diesel::insert_into(schema::serve_items::table)
                    .values(&new_serve_form)
                    .get_result::<ServeItems>(&_connection)
                    .expect("Error.");
                serve_ids.push(*serve_id);
            }

            // получаем опции, чтобы создать связи с их тех. категорией.
            // это надо отрисовки тех категорий услуги, которые активны
            let _serves = serve
                .filter(schema::serve::id.eq_any(serve_ids))
                .load::<Serve>(&_connection)
                .expect("E");

            let mut tech_cat_ids = Vec::new();
            let mut service_price = 0;
            for _serve in _serves.iter() {
                if !tech_cat_ids.iter().any(|&i| i==_serve.tech_cat_id) {
                    tech_cat_ids.push(_serve.tech_cat_id);
                }
                service_price += _serve.price;
            }

            for id in tech_cat_ids.iter() {
                let new_cat = NewTechCategoriesItem {
                    category_id: *id,
                    service_id:  _service.id,
                    store_id:    0,
                    work_id:     0,
                    types:       1,
                };
                diesel::insert_into(schema::tech_categories_items::table)
                    .values(&new_cat)
                    .get_result::<TechCategoriesItem>(&_connection)
                    .expect("Error.");
            }

            // фух. Связи созданы все, но надо еще посчитать цену
            // услуги для калькулятора. Как? А  это будет сумма всех
            // цен выбранных опций.
            diesel::update(&_service)
                .set(schema::services::price.eq(service_price))
                .get_result::<Service>(&_connection)
                .expect("Error.");
        }
    };
    HttpResponse::Ok()
}

pub async fn edit_service(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use crate::schema::{
                tags::dsl::tags,
                serve::dsl::serve,
                service_categories::dsl::service_categories,
                service_images::dsl::service_images,
                service_videos::dsl::service_videos,
                tags_items::dsl::tags_items,
                serve_items::dsl::serve_items,
                tech_categories_items::dsl::tech_categories_items,
                service_category::dsl::service_category,
                services::dsl::services,
            };
            use crate::models::{
                TechCategoriesItem,
                NewTechCategoriesItem,
                Serve,
                ServeItems,
                NewServeItems,
                EditService,
            };
            use crate::utils::store_form;

            let _connection = establish_connection();
            let _service_id: i32 = *_id;
            let _services = services
                .filter(schema::services::id.eq(_service_id))
                .load::<Service>(&_connection)
                .expect("E");

            let _service = _services.into_iter().nth(0).unwrap();

            let _categories = _service.get_categories();
            let _tags = _service.get_tags();
            for _category in _categories.iter() {
                diesel::update(_category)
                    .set(schema::service_categories::count.eq(_category.count - 1))
                    .get_result::<ServiceCategories>(&_connection)
                    .expect("Error.");
            };
            for _tag in _tags.iter() {
                diesel::update(_tag)
                .set((schema::tags::count.eq(_tag.count - 1), schema::tags::service_count.eq(_tag.service_count - 1)))
                .get_result::<Tag>(&_connection)
                .expect("Error.");
            };

            diesel::delete(service_images.filter(schema::service_images::service.eq(_service_id))).execute(&_connection).expect("E");
            diesel::delete(service_videos.filter(schema::service_videos::service.eq(_service_id))).execute(&_connection).expect("E");
            diesel::delete(tags_items.filter(schema::tags_items::service_id.eq(_service_id))).execute(&_connection).expect("E");
            diesel::delete(serve_items.filter(schema::serve_items::service_id.eq(_service_id))).execute(&_connection).expect("E");
            diesel::delete(tech_categories_items.filter(schema::tech_categories_items::service_id.eq(_service_id))).execute(&_connection).expect("E");
            diesel::delete(service_category.filter(schema::service_category::service_id.eq(_service_id))).execute(&_connection).expect("E");

            let form = store_form(payload.borrow_mut(), _request_user.id).await;
            let _new_service = EditService {
                title:       form.title.clone(),
                description: Some(form.description.clone()),
                link:        Some(form.link.clone()),
                image:       Some(form.main_image.clone()),
                is_active:   form.is_active.clone(),
                position:    form.position,
            };

            diesel::update(&_service)
            .set(_new_service)
            .get_result::<Service>(&_connection)
            .expect("E");

            for _image in form.images.iter() {
                let new_edit_image = NewServiceImage::from_service_images_form (
                    _service_id,
                    _image.to_string()
                );
                diesel::insert_into(schema::service_images::table)
                .values(&new_edit_image)
                .get_result::<ServiceImage>(&_connection)
                .expect("E.");
            };
            for _video in form.videos.iter() {
                let new_video = NewServiceVideo::from_service_videos_form (
                    _service_id,
                    _video.to_string()
                );
                diesel::insert_into(schema::service_videos::table)
                .values(&new_video)
                .get_result::<ServiceVideo>(&_connection)
                .expect("E.");
            };
            for category_id in form.category_list.iter() {
                let new_category = NewServiceCategory {
                    service_categories_id: *category_id,
                    service_id:            _service_id
                };
                diesel::insert_into(schema::service_category::table)
                .values(&new_category)
                .get_result::<ServiceCategory>(&_connection)
                .expect("E.");

                let _category_2 = service_categories.filter(schema::service_categories::id.eq(category_id)).load::<ServiceCategories>(&_connection).expect("E");
                diesel::update(&_category_2[0])
                    .set(schema::service_categories::count.eq(_category_2[0].count + 1))
                    .get_result::<ServiceCategories>(&_connection)
                    .expect("Error.");
            };
            for _tag_id in form.tags_list.iter() {
                let _new_tag = NewTagItems {
                    tag_id:     *_tag_id,
                    service_id: _service_id,
                    store_id:   0,
                    blog_id:    0,
                    wiki_id:    0,
                    work_id:    0,
                    created:    chrono::Local::now().naive_utc(),
                };
                diesel::insert_into(schema::tags_items::table)
                    .values(&_new_tag)
                    .get_result::<TagItems>(&_connection)
                    .expect("Error.");
                let _tag_2 = tags.filter(schema::tags::id.eq(_tag_id)).load::<Tag>(&_connection).expect("E");
                diesel::update(&_tag_2[0])
                    .set((schema::tags::count.eq(_tag_2[0].count + 1), schema::tags::service_count.eq(_tag_2[0].service_count + 1)))
                    .get_result::<Tag>(&_connection)
                    .expect("Error.");
            };

            // создаем связь с тех категориями, которые будут
            // расширять списки опций, предлагая доп возможности и услуги
            for cat_id in form.close_tech_cats_list.iter() {
                let new_cat = NewTechCategoriesItem {
                    category_id: *cat_id,
                    service_id:  _service.id,
                    store_id:    0,
                    work_id:     0,
                    types:       2,
                };
                diesel::insert_into(schema::tech_categories_items::table)
                    .values(&new_cat)
                    .get_result::<TechCategoriesItem>(&_connection)
                    .expect("Error.");
            }

            // создаем опции услуги и записываем id опций в вектор.
            let mut serve_ids = Vec::new();
            for serve_id in form.serve_list.iter() {
                let new_serve_form = NewServeItems {
                    serve_id:   *serve_id,
                    service_id: _service.id,
                    store_id:   0,
                    work_id:    0,
                };
                diesel::insert_into(schema::serve_items::table)
                    .values(&new_serve_form)
                    .get_result::<ServeItems>(&_connection)
                    .expect("Error.");
                serve_ids.push(*serve_id);
            }

            // получаем опции, чтобы создать связи с их тех. категорией.
            // это надо отрисовки тех категорий услуги, которые активны
            let _serves = serve
                .filter(schema::serve::id.eq_any(serve_ids))
                .load::<Serve>(&_connection)
                .expect("E");

            let mut tech_cat_ids = Vec::new();
            let mut service_price = 0;
            for _serve in _serves.iter() {
                if !tech_cat_ids.iter().any(|&i| i==_serve.tech_cat_id) {
                    tech_cat_ids.push(_serve.tech_cat_id);
                }
                service_price += _serve.price;
            }

            for id in tech_cat_ids.iter() {
                let new_cat = NewTechCategoriesItem {
                    category_id: *id,
                    service_id:  _service.id,
                    store_id:    0,
                    work_id:     0,
                    types:       1,
                };
                diesel::insert_into(schema::tech_categories_items::table)
                    .values(&new_cat)
                    .get_result::<TechCategoriesItem>(&_connection)
                    .expect("Error.");
            }

            // фух. Связи созданы все, но надо еще посчитать цену
            // услуги для калькулятора. Как? А  это будет сумма всех
            // цен выбранных опций.
            diesel::update(&_service)
                .set(schema::services::price.eq(service_price))
                .get_result::<Service>(&_connection)
                .expect("Error.");
        }
    }
    HttpResponse::Ok()
}

pub async fn edit_service_category(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::models::EditServiceCategories;
    use crate::schema::service_categories::dsl::service_categories;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let _cat_id: i32 = *_id;
            let _category = service_categories.filter(schema::service_categories::id.eq(_cat_id)).load::<ServiceCategories>(&_connection).expect("E");

            let form = category_form(payload.borrow_mut(), _request_user.id).await;
            let _new_cat = EditServiceCategories {
                name:        form.name.clone(),
                description: Some(form.description.clone()),
                position:    form.position,
                image:       Some(form.image.clone()),
                count:       _category[0].count,
            };

            diesel::update(&_category[0])
                .set(_new_cat)
                .get_result::<ServiceCategories>(&_connection)
                .expect("E");
        }
    }
    HttpResponse::Ok()
}

pub async fn delete_service(session: Session, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::services::dsl::services;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::service_category::dsl::service_category;
    use crate::schema::service_videos::dsl::service_videos;
    use crate::schema::service_images::dsl::service_images;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let _service_id: i32 = *_id;
            let _services = services.filter(schema::services::id.eq(_service_id)).load::<Service>(&_connection).expect("E");

            let _service = _services.into_iter().nth(0).unwrap();
            let _categories = _service.get_categories();
            let _tags = _service.get_tags();
            for _category in _categories.iter() {
                diesel::update(_category)
                .set(schema::service_categories::count.eq(_category.count - 1))
                .get_result::<ServiceCategories>(&_connection)
                .expect("Error.");
            };
            for _tag in _tags.iter() {
                diesel::update(_tag)
                .set((schema::tags::count.eq(_tag.count - 1), schema::tags::service_count.eq(_tag.service_count - 1)))
                .get_result::<Tag>(&_connection)
                .expect("Error.");
            };

            diesel::delete(service_images.filter(schema::service_images::service.eq(_service_id))).execute(&_connection).expect("E");
            diesel::delete(service_videos.filter(schema::service_videos::service.eq(_service_id))).execute(&_connection).expect("E");
            diesel::delete(tags_items.filter(schema::tags_items::service_id.eq(_service_id))).execute(&_connection).expect("E");
            diesel::delete(service_category.filter(schema::service_category::service_id.eq(_service_id))).execute(&_connection).expect("E");
            diesel::delete(&_service).execute(&_connection).expect("E");
        }
    }
    HttpResponse::Ok()
}

pub async fn delete_service_category(session: Session, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::service_categories::dsl::service_categories;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let _cat_id: i32 = *_id;
            let _category = service_categories.filter(schema::service_categories::id.eq(_cat_id)).load::<ServiceCategories>(&_connection).expect("E");
            diesel::delete(service_categories.filter(schema::service_categories::id.eq(_cat_id))).execute(&_connection).expect("E");
        }
    }
    HttpResponse::Ok()
}

pub async fn get_service_page(session: Session, req: HttpRequest, param: web::Path<(i32,i32)>) -> actix_web::Result<HttpResponse> {
    use schema::{
        services::dsl::services,
        service_categories::dsl::service_categories,
        service_images::dsl::service_images,
        service_videos::dsl::service_videos,
        tech_categories::dsl::tech_categories,
    };
    use crate::utils::get_device_and_ajax;
    use crate::models::TechCategories;

    let _connection = establish_connection();
    let _service_id: i32 = param.1;
    let _cat_id: i32 = param.0;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    let _services = services
        .filter(schema::services::id.eq(&_service_id))
        .load::<Service>(&_connection)
        .expect("E");

    let _tech_categories = tech_categories
        .load::<TechCategories>(&_connection)
        .expect("E");

    let _service = _services.into_iter().nth(0).unwrap();

    let _categorys = service_categories
        .filter(schema::service_categories::id.eq(&_cat_id))
        .load::<ServiceCategories>(&_connection)
        .expect("E");
    let _category = _categorys.into_iter().nth(0).unwrap();
    let service_cats = service_categories
        .load::<ServiceCategories>(&_connection)
        .expect("E");

    let _images: Vec<ServiceImage> = service_images.filter(schema::service_images::service.eq(&_service_id)).load(&_connection).expect("E");
    let _videos: Vec<ServiceVideo> = service_videos.filter(schema::service_videos::service.eq(&_service_id)).load(&_connection).expect("E");
    let _tags = _service.get_tags();

    let mut prev: Option<Service> = None;
    let mut next: Option<Service> = None;

    let _category_services = _category.get_services_ids();
    let _category_services_len = _category_services.len();

    for (i, item) in _category_services.iter().enumerate().rev() {
        if item == &_service_id {
            if (i + 1) != _category_services_len {
                let _next = Some(&_category_services[i + 1]);
                next = services
                    .filter(schema::services::id.eq(_next.unwrap()))
                    .filter(schema::services::is_active.eq(true))
                    .load::<Service>(&_connection)
                    .expect("E")
                    .into_iter()
                    .nth(0);
            };
            if i != 0 {
                let _prev = Some(&_category_services[i - 1]);
                prev = services
                    .filter(schema::services::id.eq(_prev.unwrap()))
                    .filter(schema::services::is_active.eq(true))
                    .load::<Service>(&_connection)
                    .expect("E")
                    .into_iter()
                    .nth(0);
            };
            break;
        }
    };

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/services/service.stpl")]
            struct Template {
                request_user: User,
                object:       Service,
                images:       Vec<ServiceImage>,
                videos:       Vec<ServiceVideo>,
                category:     ServiceCategories,
                service_cats: Vec<ServiceCategories>,
                //tech_cats:    Vec<TechCategories>,
                all_tags:     Vec<Tag>,
                prev:         Option<Service>,
                next:         Option<Service>,
                is_ajax:      bool,
            }
            let body = Template {
                request_user: _request_user,
                object:     _service,
                images:     _images,
                videos:     _videos,
                category:   _category,
                service_cats: service_cats,
                //tech_cats:  _tech_categories,
                all_tags:   _tags,
                prev:       prev,
                next:       next,
                is_ajax:    is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/services/service.stpl")]
            struct Template {
                request_user: User,
                object:       Service,
                images:       Vec<ServiceImage>,
                videos:       Vec<ServiceVideo>,
                category:     ServiceCategories,
                service_cats: Vec<ServiceCategories>,
                //tech_cats:    Vec<TechCategories>,
                all_tags:     Vec<Tag>,
                prev:         Option<Service>,
                next:         Option<Service>,
                is_ajax:      bool,
            }
            let body = Template {
                request_user: _request_user,
                object:     _service,
                images:     _images,
                videos:     _videos,
                category:   _category,
                service_cats: service_cats,
                //tech_cats:  _tech_categories,
                all_tags:   _tags,
                prev:       prev,
                next:       next,
                is_ajax:    is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/services/anon_service.stpl")]
            struct Template {
                object:     Service,
                images:     Vec<ServiceImage>,
                videos:     Vec<ServiceVideo>,
                category:   ServiceCategories,
                service_cats: Vec<ServiceCategories>,
                //tech_cats:  Vec<TechCategories>,
                all_tags:   Vec<Tag>,
                prev:       Option<Service>,
                next:       Option<Service>,
                is_ajax:    bool,
            }
            let body = Template {
                object:     _service,
                images:     _images,
                videos:     _videos,
                category:   _category,
                service_cats: service_cats,
                //tech_cats:  _tech_categories,
                all_tags:   _tags,
                prev:       prev,
                next:       next,
                is_ajax:    is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/services/anon_service.stpl")]
            struct Template {
                object:     Service,
                images:     Vec<ServiceImage>,
                videos:     Vec<ServiceVideo>,
                category:   ServiceCategories,
                service_cats: Vec<ServiceCategories>,
                //tech_cats:    Vec<TechCategories>,
                all_tags:   Vec<Tag>,
                prev:       Option<Service>,
                next:       Option<Service>,
                is_ajax:    bool,
            }
            let body = Template {
                object:     _service,
                images:     _images,
                videos:     _videos,
                category:   _category,
                service_cats: service_cats,
                //tech_cats:  _tech_categories,
                all_tags:   _tags,
                prev:       prev,
                next:       next,
                is_ajax:    is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn service_category_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::schema::service_categories::dsl::service_categories;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::utils::{get_device_and_ajax, get_page};

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let page = get_page(&req);

    let _cat_id: i32 = *_id;
    let _connection = establish_connection();

    let _categorys = service_categories.filter(schema::service_categories::id.eq(_cat_id)).load::<ServiceCategories>(&_connection).expect("E");
    let _category = _categorys.into_iter().nth(0).unwrap();
    let service_cats = service_categories
        .load::<ServiceCategories>(&_connection)
        .expect("E");

    let (object_list, next_page_number) = _category.get_services_list(page, 20);

    let mut stack = Vec::new();
    let _tag_items = tags_items
        .filter(schema::tags_items::service_id.ne(0))
        .select(schema::tags_items::tag_id)
        .load::<i32>(&_connection)
        .expect("E");
    for _tag_item in _tag_items.iter() {
        if !stack.iter().any(|&i| i==_tag_item) {
            stack.push(_tag_item);
        }
    };
    let _tags = schema::tags::table
        .filter(schema::tags::id.eq_any(stack))
        .load::<Tag>(&_connection)
        .expect("E");

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/services/category.stpl")]
            struct Template {
                request_user:     User,
                //all_tags:         Vec<Tag>,
                category:         ServiceCategories,
                //service_cats:     Vec<ServiceCategories>,
                object_list:      Vec<Service>,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                request_user:     _request_user,
                //all_tags:         _tags,
                category:         _category,
                //service_cats:     service_cats,
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
            #[template(path = "mobile/services/category.stpl")]
            struct Template {
                request_user:     User,
                all_tags:         Vec<Tag>,
                category:         ServiceCategories,
                service_cats:     Vec<ServiceCategories>,
                object_list:      Vec<Service>,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                request_user:     _request_user,
                all_tags:         _tags,
                category:         _category,
                service_cats:     service_cats,
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
            #[template(path = "desctop/services/anon_category.stpl")]
            struct Template {
                //all_tags:         Vec<Tag>,
                category:         ServiceCategories,
                //service_cats:     Vec<ServiceCategories>,
                object_list:      Vec<Service>,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                //all_tags:         _tags,
                category:         _category,
                //service_cats:     service_cats,
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
            #[template(path = "mobile/services/anon_category.stpl")]
            struct Template {
                all_tags:         Vec<Tag>,
                category:         ServiceCategories,
                service_cats:     Vec<ServiceCategories>,
                object_list:      Vec<Service>,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                all_tags:         _tags,
                category:        _category,
                service_cats:     service_cats,
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

pub async fn service_categories_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::tags::dsl::tags;
    use crate::schema::service_categories::dsl::service_categories;
    use crate::utils::get_device_and_ajax;

    let _connection = establish_connection();
    let mut stack = Vec::new();

    let _tag_items = tags_items
        .filter(schema::tags_items::service_id.ne(0))
        .select(schema::tags_items::tag_id)
        .load::<i32>(&_connection)
        .expect("E");

    for _tag_item in _tag_items.iter() {
        if !stack.iter().any(|&i| i==_tag_item) {
            stack.push(_tag_item);
        }
    };
    let _tags = tags
        .filter(schema::tags::id.eq_any(stack))
        .load::<Tag>(&_connection)
        .expect("could not load tags");

    let _service_cats :Vec<ServiceCategories> = service_categories
        .load(&_connection)
        .expect("Error");

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/services/categories.stpl")]
            struct Template {
                request_user: User,
                is_ajax:      bool,
                service_cats: Vec<ServiceCategories>,
                //all_tags:     Vec<Tag>,
            }
            let body = Template {
                request_user: _request_user,
                is_ajax:      is_ajax,
                service_cats: _service_cats,
                //all_tags:     _tags,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/services/categories.stpl")]
            struct Template {
                request_user: User,
                is_ajax:      bool,
                service_cats: Vec<ServiceCategories>,
                all_tags:     Vec<Tag>,
            }
            let body = Template {
                request_user: _request_user,
                is_ajax:      is_ajax,
                service_cats: _service_cats,
                all_tags:     _tags,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/services/anon_categories.stpl")]
            struct Template {
                is_ajax:      bool,
                service_cats: Vec<ServiceCategories>,
                //all_tags:     Vec<Tag>,
            }
            let body = Template {
                is_ajax:      is_ajax,
                service_cats: _service_cats,
                //all_tags:     _tags,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/services/anon_categories.stpl")]
            struct Template {
                is_ajax:      bool,
                service_cats: Vec<ServiceCategories>,
                all_tags:     Vec<Tag>,
            }
            let body = Template {
                is_ajax:      is_ajax,
                service_cats: _service_cats,
                all_tags:     _tags,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}
