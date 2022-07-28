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
    store_form,
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
    StoreCategories,
    NewStoreCategories,
    Store,
    NewStore,
    StoreCategory,
    NewStoreCategory,
    StoreImage,
    NewStoreImage,
    StoreVideo,
    NewStoreVideo,
    TagItems,
    NewTagItems,
    Tag,
};
use sailfish::TemplateOnce;


pub fn store_routes(config: &mut web::ServiceConfig) {
    config.route("/store_categories/", web::get().to(store_categories_page));
    config.service(web::resource("/create_store_categories/")
        .route(web::get().to(create_store_categories_page))
        .route(web::post().to(create_store_categories))
    );
    config.service(web::resource("/edit_store_category/{id}/")
        .route(web::get().to(edit_store_category_page))
        .route(web::post().to(edit_store_category))
    );
    config.service(web::resource("/create_store/")
        .route(web::get().to(create_store_page))
        .route(web::post().to(create_store))
    );
    config.service(web::resource("/edit_store/{id}/")
        .route(web::get().to(edit_store_page))
        .route(web::post().to(edit_store))
    );
    config.service(web::resource("/edit_content_store/{id}/")
        .route(web::get().to(edit_content_store_page))
        .route(web::post().to(edit_content_store))
    );
    config.route("/delete_store/{id}/", web::get().to(delete_store));
    config.route("/delete_store_category/{id}/", web::get().to(delete_store_category));
    config.service(web::resource("/store/{cat_id}/{store_id}/").route(web::get().to(get_store_page)));
    config.service(web::resource("/stores/{id}/").route(web::get().to(store_category_page)));
}

pub async fn create_store_categories_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use schema::store_categories::dsl::store_categories;
            use crate::utils::get_device_and_ajax;

            let _connection = establish_connection();
            let _store_cats:Vec<StoreCategories> = store_categories
                .load(&_connection)
                .expect("Error");

            let (is_desctop, is_ajax) = get_device_and_ajax(&req);

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/stores/create_categories.stpl")]
                struct Template {
                    request_user: User,
                    store_cats: Vec<StoreCategories>,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    store_cats: _store_cats,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/stores/create_categories.stpl")]
                struct Template {
                    request_user: User,
                    store_cats: Vec<StoreCategories>,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    store_cats: _store_cats,
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

pub async fn create_store_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use schema::{
                tags::dsl::tags,
                tech_categories::dsl::tech_categories,
                store_categories::dsl::store_categories,
            };
            use crate::models::TechCategories;

            use crate::utils::get_device_and_ajax;

            let _connection = establish_connection();
            let _store_cats:Vec<StoreCategories> = store_categories
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
                #[template(path = "desctop/stores/create_store.stpl")]
                struct Template {
                    request_user: User,
                    store_cats:   Vec<StoreCategories>,
                    tech_cats:    Vec<TechCategories>,
                    all_tags:     Vec<Tag>,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    store_cats:   _store_cats,
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
                #[template(path = "mobile/stores/create_store.stpl")]
                struct Template {
                    request_user: User,
                    store_cats:   Vec<StoreCategories>,
                    tech_cats:    Vec<TechCategories>,
                    all_tags:     Vec<Tag>,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    store_cats:   _store_cats,
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
pub async fn edit_store_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use schema::stores::dsl::stores;

    let _store_id: i32 = *_id;
    let _connection = establish_connection();
    let _stores = stores.filter(schema::stores::id.eq(&_store_id)).load::<Store>(&_connection).expect("E");
    let _store = _stores.into_iter().nth(0).unwrap();

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 && _store.user_id == _request_user.id {
            use schema::{
                tags::dsl::tags,
                store_images::dsl::store_images,
                store_videos::dsl::store_videos,
                store_categories::dsl::store_categories,
                tech_categories::dsl::tech_categories,
            };
            use crate::utils::get_device_and_ajax;
            use crate::models::TechCategories;

            let (is_desctop, is_ajax) = get_device_and_ajax(&req);
            let _all_tags: Vec<Tag> = tags.load(&_connection).expect("Error.");
            let _store_tags = _store.get_tags();

            let _images = store_images.filter(schema::store_images::store.eq(_store.id)).load::<StoreImage>(&_connection).expect("E");
            let _videos = store_videos.filter(schema::store_videos::store.eq(_store.id)).load::<StoreVideo>(&_connection).expect("E");

            let _store_cats:Vec<StoreCategories> = store_categories
                .load(&_connection)
                .expect("Error");

            let _tech_categories = tech_categories
                .load::<TechCategories>(&_connection)
                .expect("E");

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/stores/edit_store.stpl")]
                struct Template {
                    request_user: User,
                    object:       Store,
                    store_cats:   Vec<StoreCategories>,
                    is_ajax:      bool,
                    images:       Vec<StoreImage>,
                    videos:       Vec<StoreVideo>,
                    all_tags:     Vec<Tag>,
                    store_tags:   Vec<Tag>,
                    tech_cats:    Vec<TechCategories>,

                }
                let body = Template {
                    request_user: _request_user,
                    object:       _store,
                    store_cats:   _store_cats,
                    is_ajax:      is_ajax,
                    images:       _images,
                    videos:       _videos,
                    all_tags:     _all_tags,
                    store_tags:   _store_tags,
                    tech_cats:    _tech_categories,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/stores/edit_store.stpl")]
                struct Template {
                    request_user: User,
                    object:       Store,
                    store_cats:   Vec<StoreCategories>,
                    is_ajax:      bool,
                    images:       Vec<StoreImage>,
                    videos:       Vec<StoreVideo>,
                    all_tags:     Vec<Tag>,
                    store_tags:   Vec<Tag>,
                    tech_cats:    Vec<TechCategories>,
                }
                let body = Template {
                    request_user: _request_user,
                    object:       _store,
                    store_cats:   _store_cats,
                    is_ajax:      is_ajax,
                    images:       _images,
                    videos:       _videos,
                    all_tags:     _all_tags,
                    store_tags:   _store_tags,
                    tech_cats:    _tech_categories,
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

pub async fn edit_content_store_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::schema::stores::dsl::stores;

    let _store_id: i32 = *_id;
    let _connection = establish_connection();
    let _stores = stores
        .filter(schema::stores::id.eq(&_store_id))
        .load::<Store>(&_connection)
        .expect("E");

    let _store = _stores.into_iter().nth(0).unwrap();

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 && _request_user.id == _store.user_id {
            use crate::utils::get_device_and_ajax;

            let (is_desctop, is_ajax) = get_device_and_ajax(&req);

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/stores/edit_content_store.stpl")]
                struct Template {
                    request_user: User,
                    store:      Store,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    store:      _store,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/stores/edit_content_store.stpl")]
                struct Template {
                    request_user: User,
                    store:      Store,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    store:      _store,
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
pub async fn edit_content_store(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::stores::dsl::stores;

    let _store_id: i32 = *_id;
    let _connection = establish_connection();
    let _stores = stores
        .filter(schema::stores::id.eq(&_store_id))
        .load::<Store>(&_connection)
        .expect("E");

    let _store = _stores.into_iter().nth(0).unwrap();

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 && _request_user.id == _store.user_id {
            use crate::utils::content_form;

            let form = content_form(payload.borrow_mut()).await;
            diesel::update(&_store)
            .set(schema::stores::content.eq(form.content.clone()))
            .get_result::<Store>(&_connection)
            .expect("E");
        }
    }
    HttpResponse::Ok().body("")
}

pub async fn edit_store_category_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use schema::store_categories::dsl::store_categories;
            use crate::utils::get_device_and_ajax;

            let (is_desctop, is_ajax) = get_device_and_ajax(&req);

            let _cat_id: i32 = *_id;
            let _connection = establish_connection();
            let _categorys = store_categories
                .filter(schema::store_categories::id.eq(&_cat_id))
                .load::<StoreCategories>(&_connection)
                .expect("E");

            let _category = _categorys.into_iter().nth(0).unwrap();
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/stores/edit_category.stpl")]
                struct Template {
                    request_user: User,
                    category:     StoreCategories,
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
                #[template(path = "mobile/stores/edit_category.stpl")]
                struct Template {
                    request_user: User,
                    category:     StoreCategories,
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

pub async fn create_store_categories(session: Session, mut payload: Multipart) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let form = category_form(payload.borrow_mut(), _request_user.id).await;
            let new_cat = NewStoreCategories {
                name:        form.name.clone(),
                description: Some(form.description.clone()),
                position:    form.position,
                image:       Some(form.image.clone()),
                count:       0
            };
            let _new_store = diesel::insert_into(schema::store_categories::table)
                .values(&new_cat)
                .get_result::<StoreCategories>(&_connection)
                .expect("E.");
        }
    }
    return HttpResponse::Ok();
}

pub async fn create_store(session: Session, mut payload: Multipart) -> impl Responder {
    use crate::schema::tags::dsl::tags;
    use crate::schema::store_categories::dsl::store_categories;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();

            let form = store_form(payload.borrow_mut(), _request_user.id).await;
            let new_store = NewStore::from_store_form (
                form.title.clone(),
                form.description.clone(),
                form.link.clone(),
                form.main_image.clone(),
                form.is_active.clone(),
                0,
                _request_user.id,
            );

            let _store = diesel::insert_into(schema::stores::table)
                .values(&new_store)
                .get_result::<Store>(&_connection)
                .expect("E.");

            for image in form.images.iter() {
                let new_image = NewStoreImage::from_store_images_form (
                    _store.id,
                    image.to_string()
                );
                diesel::insert_into(schema::store_images::table)
                    .values(&new_image)
                    .get_result::<StoreImage>(&_connection)
                    .expect("E.");
                };
            for video in form.videos.iter() {
                let new_video = NewStoreVideo::from_store_videos_form (
                    _store.id,
                    video.to_string()
                );
                diesel::insert_into(schema::store_videos::table)
                    .values(&new_video)
                    .get_result::<StoreVideo>(&_connection)
                    .expect("E.");
            };
            for category_id in form.category_list.iter() {
                let new_category = NewStoreCategory {
                    store_categories_id: *category_id,
                    store_id: _store.id
                };
                diesel::insert_into(schema::store_category::table)
                    .values(&new_category)
                    .get_result::<StoreCategory>(&_connection)
                    .expect("E.");

                let _category = store_categories.filter(schema::store_categories::id.eq(category_id)).load::<StoreCategories>(&_connection).expect("E");
                diesel::update(&_category[0])
                    .set(schema::store_categories::count.eq(_category[0].count + 1))
                    .get_result::<StoreCategories>(&_connection)
                    .expect("Error.");
            };
            for tag_id in form.tags_list.iter() {
                let new_tag = NewTagItems {
                    tag_id: *tag_id,
                    service_id: 0,
                    store_id: _store.id,
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
                    .set((schema::tags::count.eq(_tag[0].count + 1), schema::tags::store_count.eq(_tag[0].store_count + 1)))
                    .get_result::<Tag>(&_connection)
                    .expect("Error.");
            }
        }
    };
    HttpResponse::Ok()
}

pub async fn edit_store(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::models::EditStore;
    use crate::schema::stores::dsl::stores;
    use crate::schema::tags::dsl::tags;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::store_images::dsl::store_images;
    use crate::schema::store_videos::dsl::store_videos;
    use crate::schema::store_category::dsl::store_category;
    use crate::schema::store_categories::dsl::store_categories;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let _store_id: i32 = *_id;
            let _stores = stores
                .filter(schema::stores::id.eq(_store_id))
                .load::<Store>(&_connection)
                .expect("E");

            let _store = _stores.into_iter().nth(0).unwrap();

            let _categories = _store.get_categories();
            let _tags = _store.get_tags();
            for _category in _categories.iter() {
                diesel::update(_category)
                    .set(schema::store_categories::count.eq(_category.count - 1))
                    .get_result::<StoreCategories>(&_connection)
                    .expect("Error.");
            };
            for _tag in _tags.iter() {
                diesel::update(_tag)
                .set((schema::tags::count.eq(_tag.count - 1), schema::tags::store_count.eq(_tag.store_count - 1)))
                .get_result::<Tag>(&_connection)
                .expect("Error.");
            };

            diesel::delete(store_images.filter(schema::store_images::store.eq(_store_id))).execute(&_connection).expect("E");
            diesel::delete(store_videos.filter(schema::store_videos::store.eq(_store_id))).execute(&_connection).expect("E");
            diesel::delete(tags_items.filter(schema::tags_items::store_id.eq(_store_id))).execute(&_connection).expect("E");
            diesel::delete(store_category.filter(schema::store_category::store_id.eq(_store_id))).execute(&_connection).expect("E");

            let form = store_form(payload.borrow_mut(), _request_user.id).await;
            let _new_store = EditStore {
                title:       form.title.clone(),
                description: Some(form.description.clone()),
                link:        Some(form.link.clone()),
                image:       Some(form.main_image.clone()),
                is_active:   form.is_active.clone(),
                price:       0,
            };

            diesel::update(&_store)
            .set(_new_store)
            .get_result::<Store>(&_connection)
            .expect("E");

            for _image in form.images.iter() {
                let new_edit_image = NewStoreImage::from_store_images_form (
                    _store_id,
                    _image.to_string()
                );
                diesel::insert_into(schema::store_images::table)
                .values(&new_edit_image)
                .get_result::<StoreImage>(&_connection)
                .expect("E.");
            };
            for _video in form.videos.iter() {
                let new_video = NewStoreVideo::from_store_videos_form (
                    _store_id,
                    _video.to_string()
                );
                diesel::insert_into(schema::store_videos::table)
                .values(&new_video)
                .get_result::<StoreVideo>(&_connection)
                .expect("E.");
            };
            for category_id in form.category_list.iter() {
                let new_category = NewStoreCategory {
                    store_categories_id: *category_id,
                    store_id:            _store_id
                };
                diesel::insert_into(schema::store_category::table)
                .values(&new_category)
                .get_result::<StoreCategory>(&_connection)
                .expect("E.");

                let _category_2 = store_categories.filter(schema::store_categories::id.eq(category_id)).load::<StoreCategories>(&_connection).expect("E");
                diesel::update(&_category_2[0])
                    .set(schema::store_categories::count.eq(_category_2[0].count + 1))
                    .get_result::<StoreCategories>(&_connection)
                    .expect("Error.");
            };
            for _tag_id in form.tags_list.iter() {
                let _new_tag = NewTagItems {
                    tag_id:     *_tag_id,
                    service_id: 0,
                    store_id:   _store.id,
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
                    .set((schema::tags::count.eq(_tag_2[0].count + 1), schema::tags::store_count.eq(_tag_2[0].store_count + 1)))
                    .get_result::<Tag>(&_connection)
                    .expect("Error.");
            };
        }
    }
    HttpResponse::Ok()
}

pub async fn edit_store_category(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::models::EditStoreCategories;
    use crate::schema::store_categories::dsl::store_categories;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let _cat_id: i32 = *_id;
            let _category = store_categories.filter(schema::store_categories::id.eq(_cat_id)).load::<StoreCategories>(&_connection).expect("E");

            let form = category_form(payload.borrow_mut(), _request_user.id).await;
            let _new_cat = EditStoreCategories {
                name:        form.name.clone(),
                description: Some(form.description.clone()),
                position:    form.position,
                image:       Some(form.image.clone()),
                count:       _category[0].count,
            };

            diesel::update(&_category[0])
                .set(_new_cat)
                .get_result::<StoreCategories>(&_connection)
                .expect("E");
        }
    }
    HttpResponse::Ok()
}

pub async fn delete_store(session: Session, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::stores::dsl::stores;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::store_category::dsl::store_category;
    use crate::schema::store_videos::dsl::store_videos;
    use crate::schema::store_images::dsl::store_images;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let _store_id: i32 = *_id;
            let _stores = stores.filter(schema::stores::id.eq(_store_id)).load::<Store>(&_connection).expect("E");

            let _store = _stores.into_iter().nth(0).unwrap();
            let _categories = _store.get_categories();
            let _tags = _store.get_tags();
            for _category in _categories.iter() {
                diesel::update(_category)
                .set(schema::store_categories::count.eq(_category.count - 1))
                .get_result::<StoreCategories>(&_connection)
                .expect("Error.");
            };
            for _tag in _tags.iter() {
                diesel::update(_tag)
                .set((schema::tags::count.eq(_tag.count - 1), schema::tags::store_count.eq(_tag.store_count - 1)))
                .get_result::<Tag>(&_connection)
                .expect("Error.");
            };

            diesel::delete(store_images.filter(schema::store_images::store.eq(_store_id))).execute(&_connection).expect("E");
            diesel::delete(store_videos.filter(schema::store_videos::store.eq(_store_id))).execute(&_connection).expect("E");
            diesel::delete(tags_items.filter(schema::tags_items::store_id.eq(_store_id))).execute(&_connection).expect("E");
            diesel::delete(store_category.filter(schema::store_category::store_id.eq(_store_id))).execute(&_connection).expect("E");
            diesel::delete(&_store).execute(&_connection).expect("E");
        }
    }
    HttpResponse::Ok()
}

pub async fn delete_store_category(session: Session, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::store_categories::dsl::store_categories;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let _cat_id: i32 = *_id;
            let _category = store_categories.filter(schema::store_categories::id.eq(_cat_id)).load::<StoreCategories>(&_connection).expect("E");
            diesel::delete(store_categories.filter(schema::store_categories::id.eq(_cat_id))).execute(&_connection).expect("E");
        }
    }
    HttpResponse::Ok()
}

pub async fn get_store_page(session: Session, req: HttpRequest, param: web::Path<(i32,i32)>) -> actix_web::Result<HttpResponse> {
    use schema::{
        stores::dsl::stores,
        store_categories::dsl::store_categories,
        tech_categories::dsl::tech_categories,
        store_videos::dsl::store_videos,
        store_images::dsl::store_images,
    };
    use crate::utils::get_device_and_ajax;
    use crate::models::TechCategories;

    let _connection = establish_connection();
    let _store_id: i32 = param.1;
    let _cat_id: i32 = param.0;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    let _stores = stores
        .filter(schema::stores::id.eq(&_store_id))
        .load::<Store>(&_connection)
        .expect("E");

    let _store = _stores.into_iter().nth(0).unwrap();

    let _categorys = store_categories
        .filter(schema::store_categories::id.eq(&_cat_id))
        .load::<StoreCategories>(&_connection)
        .expect("E");
    let _category = _categorys.into_iter().nth(0).unwrap();
    let _store_categories = store_categories
        .load::<StoreCategories>(&_connection)
        .expect("E");

    let _tech_categories = tech_categories
        .load::<TechCategories>(&_connection)
        .expect("E");

    let _images: Vec<StoreImage> = store_images.filter(schema::store_images::store.eq(&_store_id)).load(&_connection).expect("E");
    let _videos: Vec<StoreVideo> = store_videos.filter(schema::store_videos::store.eq(&_store_id)).load(&_connection).expect("E");
    let _tags = _store.get_tags();

    let mut prev: Option<Store> = None;
    let mut next: Option<Store> = None;

    let _category_stores = _category.get_stores_ids();
    let _category_stores_len = _category_stores.len();

    for (i, item) in _category_stores.iter().enumerate().rev() {
        if item == &_store_id {
            if (i + 1) != _category_stores_len {
                let _next = Some(&_category_stores[i + 1]);
                next = stores
                    .filter(schema::stores::id.eq(_next.unwrap()))
                    .filter(schema::stores::is_active.eq(true))
                    .load::<Store>(&_connection)
                    .expect("E")
                    .into_iter()
                    .nth(0);
            };
            if i != 0 {
                let _prev = Some(&_category_stores[i - 1]);
                prev = stores
                    .filter(schema::stores::id.eq(_prev.unwrap()))
                    .filter(schema::stores::is_active.eq(true))
                    .load::<Store>(&_connection)
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
            #[template(path = "desctop/stores/store.stpl")]
            struct Template {
                request_user: User,
                object:       Store,
                images:       Vec<StoreImage>,
                videos:       Vec<StoreVideo>,
                category:     StoreCategories,
                //all_tags:     Vec<Tag>,
                prev:         Option<Store>,
                next:         Option<Store>,
                is_ajax:      bool,
            }
            let body = Template {
                request_user: _request_user,
                object:     _store,
                images:     _images,
                videos:     _videos,
                category:   _category,
                //all_tags:   _tags,
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
            #[template(path = "mobile/stores/store.stpl")]
            struct Template {
                request_user: User,
                object:       Store,
                images:       Vec<StoreImage>,
                videos:       Vec<StoreVideo>,
                category:     StoreCategories,
                tech_cats:    Vec<TechCategories>,
                store_cats:   Vec<StoreCategories>,
                all_tags:     Vec<Tag>,
                prev:         Option<Store>,
                next:         Option<Store>,
                is_ajax:      bool,
            }
            let body = Template {
                request_user: _request_user,
                object:     _store,
                images:     _images,
                videos:     _videos,
                category:   _category,
                tech_cats:  _tech_categories,
                store_cats: _store_categories,
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
            #[template(path = "desctop/stores/anon_store.stpl")]
            struct Template {
                object:     Store,
                images:     Vec<StoreImage>,
                videos:     Vec<StoreVideo>,
                category:   StoreCategories,
                //all_tags:   Vec<Tag>,
                prev:       Option<Store>,
                next:       Option<Store>,
                is_ajax:    bool,
            }
            let body = Template {
                object:     _store,
                images:     _images,
                videos:     _videos,
                category:   _category,
                //all_tags:   _tags,
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
            #[template(path = "mobile/stores/anon_store.stpl")]
            struct Template {
                object:     Store,
                images:     Vec<StoreImage>,
                videos:     Vec<StoreVideo>,
                category:   StoreCategories,
                tech_cats:  Vec<TechCategories>,
                store_cats: Vec<StoreCategories>,
                all_tags:   Vec<Tag>,
                prev:       Option<Store>,
                next:       Option<Store>,
                is_ajax:    bool,
            }
            let body = Template {
                object:     _store,
                images:     _images,
                videos:     _videos,
                category:   _category,
                tech_cats:  _tech_categories,
                store_cats: _store_categories,
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

pub async fn store_category_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::schema::store_categories::dsl::store_categories;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::utils::{get_device_and_ajax, get_page};

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let page = get_page(&req);

    let _cat_id: i32 = *_id;
    let _connection = establish_connection();

    let _categorys = store_categories.filter(schema::store_categories::id.eq(_cat_id)).load::<StoreCategories>(&_connection).expect("E");
    let _category = _categorys.into_iter().nth(0).unwrap();
    let (object_list, next_page_number) = _category.get_stores_list(page, 20);
    let _wiki_store_categories = store_categories
        .load::<StoreCategories>(&_connection)
        .expect("E");

    let mut stack = Vec::new();
    let _tag_items = tags_items
        .filter(schema::tags_items::store_id.ne(0))
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
            #[template(path = "desctop/stores/category.stpl")]
            struct Template {
                request_user:     User,
                all_tags:         Vec<Tag>,
                category:         StoreCategories,
                store_cats:       Vec<StoreCategories>,
                object_list:      Vec<Store>,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                request_user:     _request_user,
                all_tags:         _tags,
                category:         _category,
                store_cats:       _wiki_store_categories,
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
            #[template(path = "mobile/stores/category.stpl")]
            struct Template {
                request_user:     User,
                all_tags:         Vec<Tag>,
                category:         StoreCategories,
                store_cats:       Vec<StoreCategories>,
                object_list:      Vec<Store>,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                request_user:     _request_user,
                all_tags:         _tags,
                category:         _category,
                store_cats:       _wiki_store_categories,
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
            #[template(path = "desctop/stores/anon_category.stpl")]
            struct Template {
                all_tags:         Vec<Tag>,
                category:         StoreCategories,
                store_cats:       Vec<StoreCategories>,
                object_list:      Vec<Store>,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                all_tags:         _tags,
                category:         _category,
                store_cats:       _wiki_store_categories,
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
            #[template(path = "mobile/stores/anon_category.stpl")]
            struct Template {
                all_tags:         Vec<Tag>,
                category:         StoreCategories,
                store_cats:       Vec<StoreCategories>,
                object_list:      Vec<Store>,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                all_tags:         _tags,
                category:         _category,
                store_cats:       _wiki_store_categories,
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

pub async fn store_categories_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::tags::dsl::tags;
    use crate::schema::store_categories::dsl::store_categories;
    use crate::utils::get_device_and_ajax;

    let _connection = establish_connection();
    let mut stack = Vec::new();

    let _tag_items = tags_items
        .filter(schema::tags_items::store_id.ne(0))
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

    let _store_cats: Vec<StoreCategories> = store_categories
        .load(&_connection)
        .expect("Error");

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/stores/categories.stpl")]
            struct Template {
                request_user: User,
                is_ajax:      bool,
                store_cats: Vec<StoreCategories>,
                all_tags:     Vec<Tag>,
            }
            let body = Template {
                request_user: _request_user,
                is_ajax:      is_ajax,
                store_cats: _store_cats,
                all_tags:     _tags,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/stores/categories.stpl")]
            struct Template {
                request_user: User,
                is_ajax:      bool,
                store_cats: Vec<StoreCategories>,
                all_tags:     Vec<Tag>,
            }
            let body = Template {
                request_user: _request_user,
                is_ajax:      is_ajax,
                store_cats: _store_cats,
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
            #[template(path = "desctop/stores/anon_categories.stpl")]
            struct Template {
                is_ajax:      bool,
                store_cats: Vec<StoreCategories>,
                all_tags:     Vec<Tag>,
            }
            let body = Template {
                is_ajax:      is_ajax,
                store_cats: _store_cats,
                all_tags:     _tags,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/stores/anon_categories.stpl")]
            struct Template {
                is_ajax:      bool,
                store_cats: Vec<StoreCategories>,
                all_tags:     Vec<Tag>,
            }
            let body = Template {
                is_ajax:      is_ajax,
                store_cats: _store_cats,
                all_tags:     _tags,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}
