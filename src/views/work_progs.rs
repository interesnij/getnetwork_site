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
    item_form,
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
    WorkCategories,
    NewWorkCategories,
    Work,
    NewWork,
    WorkCategory,
    NewWorkCategory,
    WorkImage,
    NewWorkImage,
    WorkVideo,
    NewWorkVideo,
    TagItems,
    NewTagItems,
    Tag,
};
use sailfish::TemplateOnce;


pub fn work_routes(config: &mut web::ServiceConfig) {
    config.route("/work_categories/", web::get().to(work_categories_page));
    config.service(web::resource("/create_work_categories/")
        .route(web::get().to(create_work_categories_page))
        .route(web::post().to(create_work_categories))
    );
    config.service(web::resource("/edit_work_category/{id}/")
        .route(web::get().to(edit_work_category_page))
        .route(web::post().to(edit_work_category))
    );
    config.service(web::resource("/create_work/")
        .route(web::get().to(create_work_page))
        .route(web::post().to(create_work))
    );
    config.service(web::resource("/edit_work/{id}/")
        .route(web::get().to(edit_work_page))
        .route(web::post().to(edit_work))
    );
    config.service(web::resource("/edit_content_work/{id}/")
        .route(web::get().to(edit_content_work_page))
        .route(web::post().to(edit_content_work))
    );
    config.route("/delete_work/{id}/", web::get().to(delete_work));
    config.route("/delete_work_category/{id}/", web::get().to(delete_work_category));
    config.service(web::resource("/work/{cat_id}/{work_id}/").route(web::get().to(get_work_page)));
    config.service(web::resource("/works/{id}/").route(web::get().to(work_category_page)));
}

pub async fn create_work_categories_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use schema::work_categories::dsl::work_categories;
            use crate::utils::get_device_and_ajax;

            let _connection = establish_connection();
            let _work_cats:Vec<WorkCategories> = work_categories
                .load(&_connection)
                .expect("Error");

            let (is_desctop, is_ajax) = get_device_and_ajax(&req);

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/works/create_categories.stpl")]
                struct Template {
                    request_user: User,
                    work_cats:    Vec<WorkCategories>,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    work_cats:    _work_cats,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/works/create_categories.stpl")]
                struct Template {
                    request_user: User,
                    work_cats:    Vec<WorkCategories>,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    work_cats:    _work_cats,
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

pub async fn create_work_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use schema::{
                tags::dsl::tags,
                work_categories::dsl::work_categories,
                tech_categories::dsl::tech_categories,
            };
            use crate::utils::get_device_and_ajax;
            use crate::models::TechCategories;

            let _connection = establish_connection();
            let _work_cats = work_categories
                .load::<WorkCategories>(&_connection)
                .expect("Error");

            let _tech_cats = work_categories
                .load::<TechCategories>(&_connection)
                .expect("Error");

            let all_tags = tags
                .load::<Tag>(&_connection)
                .expect("Error.");

            let (is_desctop, is_ajax) = get_device_and_ajax(&req);

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/works/create_work.stpl")]
                struct Template {
                    request_user: User,
                    work_cats:    Vec<WorkCategories>,
                    tech_cats:    Vec<TechCategories>,
                    all_tags:     Vec<Tag>,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    work_cats:    _work_cats,
                    tech_cats:    _tech_cats,
                    all_tags:     all_tags,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/works/create_work.stpl")]
                struct Template {
                    request_user: User,
                    work_cats:    Vec<WorkCategories>,
                    tech_cats:    Vec<TechCategories>,
                    all_tags:     Vec<Tag>,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    work_cats:    _work_cats,
                    tech_cats:    _tech_cats,
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
pub async fn edit_work_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use schema::works::dsl::works;

    let _work_id: i32 = *_id;
    let _connection = establish_connection();
    let _works = works.filter(schema::works::id.eq(&_work_id)).load::<Work>(&_connection).expect("E");
    let _work = _works.into_iter().nth(0).unwrap();

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 && _work.user_id == _request_user.id {
            use schema::{
                tags::dsl::tags,
                work_images::dsl::work_images,
                work_videos::dsl::work_videos,
                work_categories::dsl::work_categories,
            };
            use crate::utils::get_device_and_ajax;

            let (is_desctop, is_ajax) = get_device_and_ajax(&req);
            let _categories = _work.get_categories();
            let _all_tags = tags.load::<Tag>(&_connection).expect("Error.");
            let _work_tags = _work.get_tags();

            let _images = work_images.filter(schema::work_images::work.eq(_work.id)).load::<WorkImage>(&_connection).expect("E");
            let _videos = work_videos.filter(schema::work_videos::work.eq(_work.id)).load::<WorkVideo>(&_connection).expect("E");

            let _work_cats = work_categories
                .load::<WorkCategories>(&_connection)
                .expect("Error");

            let _tech_cats = work_categories
                .load::<TechCategories>(&_connection)
                .expect("Error");

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/works/edit_work.stpl")]
                struct Template {
                    request_user: User,
                    object:       Work,
                    categories:   Vec<WorkCategories>,
                    tech_cats:    Vec<TechCategories>,
                    is_ajax:      bool,
                    images:       Vec<WorkImage>,
                    videos:       Vec<WorkVideo>,
                    all_tags:     Vec<Tag>,
                    work_tags:    Vec<Tag>,
                    work_cats:    Vec<WorkCategories>,

                }
                let body = Template {
                    request_user: _request_user,
                    object:       _work,
                    categories:   _categories,
                    tech_cats:    _tech_cats,
                    is_ajax:      is_ajax,
                    images:       _images,
                    videos:       _videos,
                    all_tags:     _all_tags,
                    work_tags:    _work_tags,
                    work_cats:    _work_cats,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/works/edit_work.stpl")]
                struct Template {
                    request_user: User,
                    object:       Work,
                    categories:   Vec<WorkCategories>,
                    tech_cats:    Vec<TechCategories>,
                    is_ajax:      bool,
                    images:       Vec<WorkImage>,
                    videos:       Vec<WorkVideo>,
                    all_tags:     Vec<Tag>,
                    work_tags:    Vec<Tag>,
                    work_cats:    Vec<WorkCategories>,

                }
                let body = Template {
                    request_user: _request_user,
                    object:       _work,
                    categories:   _categories,
                    tech_cats:    _tech_cats,
                    is_ajax:      is_ajax,
                    images:       _images,
                    videos:       _videos,
                    all_tags:     _all_tags,
                    work_tags:    _work_tags,
                    work_cats:    _work_cats,
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

pub async fn edit_content_work_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::schema::works::dsl::works;

    let _work_id: i32 = *_id;
    let _connection = establish_connection();
    let _works = works
        .filter(schema::works::id.eq(&_work_id))
        .load::<Work>(&_connection)
        .expect("E");

    let _work = _works.into_iter().nth(0).unwrap();

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 && _request_user.id == _work.user_id {
            use crate::utils::get_device_and_ajax;

            let (is_desctop, is_ajax) = get_device_and_ajax(&req);

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/works/edit_content_work.stpl")]
                struct Template {
                    request_user: User,
                    work:         Work,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    work:        _work,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/works/edit_content_work.stpl")]
                struct Template {
                    request_user: User,
                    work:         Work,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    work:         _work,
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
pub async fn edit_content_work(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::works::dsl::works;

    let _work_id: i32 = *_id;
    let _connection = establish_connection();
    let _works = works
        .filter(schema::works::id.eq(&_work_id))
        .load::<Work>(&_connection)
        .expect("E");

    let _work = _works.into_iter().nth(0).unwrap();

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 && _request_user.id == _work.user_id {
            use crate::utils::content_form;

            let form = content_form(payload.borrow_mut()).await;
            diesel::update(&_work)
            .set(schema::works::content.eq(form.content.clone()))
            .get_result::<Work>(&_connection)
            .expect("E");
        }
    }
    HttpResponse::Ok().body("")
}

pub async fn edit_work_category_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use schema::work_categories::dsl::work_categories;
            use crate::utils::get_device_and_ajax;

            let (is_desctop, is_ajax) = get_device_and_ajax(&req);

            let _cat_id: i32 = *_id;
            let _connection = establish_connection();
            let _categorys = work_categories
                .filter(schema::work_categories::id.eq(&_cat_id))
                .load::<WorkCategories>(&_connection)
                .expect("E");

            let _category = _categorys.into_iter().nth(0).unwrap();
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/works/edit_category.stpl")]
                struct Template {
                    request_user: User,
                    category:     WorkCategories,
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
                #[template(path = "mobile/works/edit_category.stpl")]
                struct Template {
                    request_user: User,
                    category:     WorkCategories,
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

pub async fn create_work_categories(session: Session, mut payload: Multipart) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let form = category_form(payload.borrow_mut(), _request_user.id).await;
            let new_cat = NewWorkCategories {
                name:        form.name.clone(),
                description: Some(form.description.clone()),
                position:    form.position,
                image:       Some(form.image.clone()),
                count:       0
            };
            let _new_work = diesel::insert_into(schema::work_categories::table)
                .values(&new_cat)
                .get_result::<WorkCategories>(&_connection)
                .expect("Error saving post.");
        }
    }
    return HttpResponse::Ok();
}

pub async fn create_work(session: Session, mut payload: Multipart) -> impl Responder {
    use crate::schema::tags::dsl::tags;
    use crate::schema::work_categories::dsl::work_categories;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();

            let form = item_form(payload.borrow_mut(), _request_user.id).await;
            let new_work = NewWork::from_work_form (
                form.title.clone(),
                form.description.clone(),
                form.link.clone(),
                form.main_image.clone(),
                form.is_active.clone(),
                _request_user.id,
            );

            let _work = diesel::insert_into(schema::works::table)
                .values(&new_work)
                .get_result::<Work>(&_connection)
                .expect("E.");

            for image in form.images.iter() {
                let new_image = NewWorkImage::from_work_images_form (
                    _work.id,
                    image.to_string()
                );
                diesel::insert_into(schema::work_images::table)
                    .values(&new_image)
                    .get_result::<WorkImage>(&_connection)
                    .expect("E.");
                };
            for video in form.videos.iter() {
                let new_video = NewWorkVideo::from_work_videos_form (
                    _work.id,
                    video.to_string()
                );
                diesel::insert_into(schema::work_videos::table)
                    .values(&new_video)
                    .get_result::<WorkVideo>(&_connection)
                    .expect("E.");
            };
            for category_id in form.category_list.iter() {
                let new_category = NewWorkCategory {
                    work_categories_id: *category_id,
                    work_id: _work.id
                };
                diesel::insert_into(schema::work_category::table)
                    .values(&new_category)
                    .get_result::<WorkCategory>(&_connection)
                    .expect("E.");

                let _category = work_categories.filter(schema::work_categories::id.eq(category_id)).load::<WorkCategories>(&_connection).expect("E");
                diesel::update(&_category[0])
                    .set(schema::work_categories::count.eq(_category[0].count + 1))
                    .get_result::<WorkCategories>(&_connection)
                    .expect("Error.");
            };
            for tag_id in form.tags_list.iter() {
                let new_tag = NewTagItems {
                    tag_id: *tag_id,
                    service_id: 0,
                    store_id: 0,
                    blog_id: 0,
                    wiki_id: 0,
                    work_id: _work.id,
                    created: chrono::Local::now().naive_utc(),
                };
                diesel::insert_into(schema::tags_items::table)
                    .values(&new_tag)
                    .get_result::<TagItems>(&_connection)
                    .expect("Error.");

                let _tag = tags.filter(schema::tags::id.eq(tag_id)).load::<Tag>(&_connection).expect("E");
                diesel::update(&_tag[0])
                    .set((schema::tags::count.eq(_tag[0].count + 1), schema::tags::work_count.eq(_tag[0].work_count + 1)))
                    .get_result::<Tag>(&_connection)
                    .expect("Error.");
            }
        }
    };
    HttpResponse::Ok()
}

pub async fn edit_work(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::models::EditWork;
    use crate::schema::works::dsl::works;
    use crate::schema::tags::dsl::tags;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::work_images::dsl::work_images;
    use crate::schema::work_videos::dsl::work_videos;
    use crate::schema::work_category::dsl::work_category;
    use crate::schema::work_categories::dsl::work_categories;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let _work_id: i32 = *_id;
            let _works = works
                .filter(schema::works::id.eq(_work_id))
                .load::<Work>(&_connection)
                .expect("E");

            let _work = _works.into_iter().nth(0).unwrap();

            let _categories = _work.get_categories();
            let _tags = _work.get_tags();
            for _category in _categories.iter() {
                diesel::update(_category)
                    .set(schema::work_categories::count.eq(_category.count - 1))
                    .get_result::<WorkCategories>(&_connection)
                    .expect("Error.");
            };
            for _tag in _tags.iter() {
                diesel::update(_tag)
                .set((schema::tags::count.eq(_tag.count - 1), schema::tags::work_count.eq(_tag.work_count - 1)))
                .get_result::<Tag>(&_connection)
                .expect("Error.");
            };

            diesel::delete(work_images.filter(schema::work_images::work.eq(_work_id))).execute(&_connection).expect("E");
            diesel::delete(work_videos.filter(schema::work_videos::work.eq(_work_id))).execute(&_connection).expect("E");
            diesel::delete(tags_items.filter(schema::tags_items::work_id.eq(_work_id))).execute(&_connection).expect("E");
            diesel::delete(work_category.filter(schema::work_category::work_id.eq(_work_id))).execute(&_connection).expect("E");

            let form = item_form(payload.borrow_mut(), _request_user.id).await;
            let _new_work = EditWork {
                title:       form.title.clone(),
                description: Some(form.description.clone()),
                link:        Some(form.link.clone()),
                image:       Some(form.main_image.clone()),
                is_active:   form.is_active.clone()
            };

            diesel::update(&_work)
            .set(_new_work)
            .get_result::<Work>(&_connection)
            .expect("E");

            for _image in form.images.iter() {
                let new_edit_image = NewWorkImage::from_work_images_form (
                    _work_id,
                    _image.to_string()
                );
                diesel::insert_into(schema::work_images::table)
                .values(&new_edit_image)
                .get_result::<WorkImage>(&_connection)
                .expect("E.");
            };
            for _video in form.videos.iter() {
                let new_video = NewWorkVideo::from_work_videos_form (
                    _work_id,
                    _video.to_string()
                );
                diesel::insert_into(schema::work_videos::table)
                .values(&new_video)
                .get_result::<WorkVideo>(&_connection)
                .expect("E.");
            };
            for category_id in form.category_list.iter() {
                let new_category = NewWorkCategory {
                    work_categories_id: *category_id,
                    work_id:            _work_id
                };
                diesel::insert_into(schema::work_category::table)
                .values(&new_category)
                .get_result::<WorkCategory>(&_connection)
                .expect("E.");

                let _category_2 = work_categories.filter(schema::work_categories::id.eq(category_id)).load::<WorkCategories>(&_connection).expect("E");
                diesel::update(&_category_2[0])
                    .set(schema::work_categories::count.eq(_category_2[0].count + 1))
                    .get_result::<WorkCategories>(&_connection)
                    .expect("Error.");
            };
            for _tag_id in form.tags_list.iter() {
                let _new_tag = NewTagItems {
                    tag_id:     *_tag_id,
                    service_id: 0,
                    store_id:   0,
                    blog_id:    0,
                    wiki_id:    0,
                    work_id:    _work_id,
                    created:    chrono::Local::now().naive_utc(),
                };
                diesel::insert_into(schema::tags_items::table)
                    .values(&_new_tag)
                    .get_result::<TagItems>(&_connection)
                    .expect("Error.");
                let _tag_2 = tags.filter(schema::tags::id.eq(_tag_id)).load::<Tag>(&_connection).expect("E");
                diesel::update(&_tag_2[0])
                    .set((schema::tags::count.eq(_tag_2[0].count + 1), schema::tags::work_count.eq(_tag_2[0].work_count + 1)))
                    .get_result::<Tag>(&_connection)
                    .expect("Error.");
            };
        }
    }
    HttpResponse::Ok()
}

pub async fn edit_work_category(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::models::EditWorkCategories;
    use crate::schema::work_categories::dsl::work_categories;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let _cat_id: i32 = *_id;
            let _category = work_categories.filter(schema::work_categories::id.eq(_cat_id)).load::<WorkCategories>(&_connection).expect("E");

            let form = category_form(payload.borrow_mut(), _request_user.id).await;
            let _new_cat = EditWorkCategories {
                name:        form.name.clone(),
                description: Some(form.description.clone()),
                position:    form.position,
                image:       Some(form.image.clone()),
                count:       _category[0].count,
            };

            diesel::update(&_category[0])
                .set(_new_cat)
                .get_result::<WorkCategories>(&_connection)
                .expect("E");
        }
    }
    HttpResponse::Ok()
}

pub async fn delete_work(session: Session, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::works::dsl::works;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::work_category::dsl::work_category;
    use crate::schema::work_videos::dsl::work_videos;
    use crate::schema::work_images::dsl::work_images;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let _work_id: i32 = *_id;
            let _works = works.filter(schema::works::id.eq(_work_id)).load::<Work>(&_connection).expect("E");

            let _work = _works.into_iter().nth(0).unwrap();
            let _categories = _work.get_categories();
            let _tags = _work.get_tags();
            for _category in _categories.iter() {
                diesel::update(_category)
                .set(schema::work_categories::count.eq(_category.count - 1))
                .get_result::<WorkCategories>(&_connection)
                .expect("Error.");
            };
            for _tag in _tags.iter() {
                diesel::update(_tag)
                .set((schema::tags::count.eq(_tag.count - 1), schema::tags::work_count.eq(_tag.work_count - 1)))
                .get_result::<Tag>(&_connection)
                .expect("Error.");
            };

            diesel::delete(work_images.filter(schema::work_images::work.eq(_work_id))).execute(&_connection).expect("E");
            diesel::delete(work_videos.filter(schema::work_videos::work.eq(_work_id))).execute(&_connection).expect("E");
            diesel::delete(tags_items.filter(schema::tags_items::work_id.eq(_work_id))).execute(&_connection).expect("E");
            diesel::delete(work_category.filter(schema::work_category::work_id.eq(_work_id))).execute(&_connection).expect("E");
            diesel::delete(&_work).execute(&_connection).expect("E");
        }
    }
    HttpResponse::Ok()
}

pub async fn delete_work_category(session: Session, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::work_categories::dsl::work_categories;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let _cat_id: i32 = *_id;
            let _category = work_categories.filter(schema::work_categories::id.eq(_cat_id)).load::<WorkCategories>(&_connection).expect("E");
            diesel::delete(work_categories.filter(schema::work_categories::id.eq(_cat_id))).execute(&_connection).expect("E");
        }
    }
    HttpResponse::Ok()
}

pub async fn get_work_page(session: Session, req: HttpRequest, param: web::Path<(i32,i32)>) -> actix_web::Result<HttpResponse> {
    use schema::works::dsl::works;
    use schema::work_categories::dsl::work_categories;
    use schema::work_images::dsl::work_images;
    use schema::work_videos::dsl::work_videos;
    use crate::utils::get_device_and_ajax;

    let _connection = establish_connection();
    let _work_id: i32 = param.1;
    let _cat_id: i32 = param.0;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    let _works = works
        .filter(schema::works::id.eq(&_work_id))
        .load::<Work>(&_connection)
        .expect("E");

    let _work = _works.into_iter().nth(0).unwrap();

    let _categorys = work_categories
        .filter(schema::work_categories::id.eq(&_cat_id))
        .load::<WorkCategories>(&_connection)
        .expect("E");
    let _category = _categorys.into_iter().nth(0).unwrap();
    let _work_categories = work_categories
        .load::<WorkCategories>(&_connection)
        .expect("E");

    let _images: Vec<WorkImage> = work_images.filter(schema::work_images::work.eq(&_work_id)).load(&_connection).expect("E");
    let _videos: Vec<WorkVideo> = work_videos.filter(schema::work_videos::work.eq(&_work_id)).load(&_connection).expect("E");
    let _tags = _work.get_tags();

    let mut prev: Option<Work> = None;
    let mut next: Option<Work> = None;

    let _category_works = _category.get_works_ids();
    let _category_works_len = _category_works.len();

    for (i, item) in _category_works.iter().enumerate().rev() {
        if item == &_work_id {
            if (i + 1) != _category_works_len {
                let _next = Some(&_category_works[i + 1]);
                next = works
                    .filter(schema::works::id.eq(_next.unwrap()))
                    .filter(schema::works::is_active.eq(true))
                    .load::<Work>(&_connection)
                    .expect("E")
                    .into_iter()
                    .nth(0);
            };
            if i != 0 {
                let _prev = Some(&_category_works[i - 1]);
                prev = works
                    .filter(schema::works::id.eq(_prev.unwrap()))
                    .filter(schema::works::is_active.eq(true))
                    .load::<Work>(&_connection)
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
            #[template(path = "desctop/works/work.stpl")]
            struct Template {
                request_user: User,
                object:       Work,
                images:       Vec<WorkImage>,
                videos:       Vec<WorkVideo>,
                category:     WorkCategories,
                //work_cats:    Vec<WorkCategories>,
                //all_tags:     Vec<Tag>,
                prev:         Option<Work>,
                next:         Option<Work>,
                is_ajax:      bool,
            }
            let body = Template {
                request_user: _request_user,
                object:     _work,
                images:     _images,
                videos:     _videos,
                category:   _category,
                //work_cats:  _work_categories,
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
            #[template(path = "mobile/works/work.stpl")]
            struct Template {
                request_user: User,
                object:       Work,
                images:       Vec<WorkImage>,
                videos:       Vec<WorkVideo>,
                category:     WorkCategories,
                work_cats:    Vec<WorkCategories>,
                all_tags:     Vec<Tag>,
                prev:         Option<Work>,
                next:         Option<Work>,
                is_ajax:      bool,
            }
            let body = Template {
                request_user: _request_user,
                object:     _work,
                images:     _images,
                videos:     _videos,
                category:   _category,
                work_cats:  _work_categories,
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
            #[template(path = "desctop/works/anon_work.stpl")]
            struct Template {
                object:     Work,
                images:     Vec<WorkImage>,
                videos:     Vec<WorkVideo>,
                category:   WorkCategories,
                //work_cats:  Vec<WorkCategories>,
                //all_tags:   Vec<Tag>,
                prev:       Option<Work>,
                next:       Option<Work>,
                is_ajax:    bool,
            }
            let body = Template {
                object:     _work,
                images:     _images,
                videos:     _videos,
                category:   _category,
                //work_cats:  _work_categories,
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
            #[template(path = "mobile/works/anon_work.stpl")]
            struct Template {
                object:     Work,
                images:     Vec<WorkImage>,
                videos:     Vec<WorkVideo>,
                category:   WorkCategories,
                work_cats:  Vec<WorkCategories>,
                all_tags:   Vec<Tag>,
                prev:       Option<Work>,
                next:       Option<Work>,
                is_ajax:    bool,
            }
            let body = Template {
                object:     _work,
                images:     _images,
                videos:     _videos,
                category:   _category,
                work_cats:  _work_categories,
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

pub async fn work_category_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::schema::work_categories::dsl::work_categories;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::utils::{get_device_and_ajax, get_page};

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let page = get_page(&req);

    let _cat_id: i32 = *_id;
    let _connection = establish_connection();

    let _categorys = work_categories.filter(schema::work_categories::id.eq(_cat_id)).load::<WorkCategories>(&_connection).expect("E");
    let _category = _categorys.into_iter().nth(0).unwrap();
    let (object_list, next_page_number) = _category.get_works_list(page, 20);

    let _work_categories = work_categories
        .load::<WorkCategories>(&_connection)
        .expect("E");

    let mut stack = Vec::new();
    let _tag_items = tags_items
        .filter(schema::tags_items::work_id.ne(0))
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
        .expect("could not load tags");

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/works/category.stpl")]
            struct Template {
                request_user:     User,
                all_tags:         Vec<Tag>,
                category:         WorkCategories,
                //work_cats:        Vec<WorkCategories>,
                object_list:      Vec<Work>,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                request_user:     _request_user,
                all_tags:         _tags,
                category:         _category,
                //work_cats:        _work_categories,
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
            #[template(path = "mobile/works/category.stpl")]
            struct Template {
                request_user:     User,
                all_tags:         Vec<Tag>,
                category:         WorkCategories,
                work_cats:        Vec<WorkCategories>,
                object_list:      Vec<Work>,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                request_user:     _request_user,
                all_tags:         _tags,
                category:         _category,
                work_cats:        _work_categories,
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
            #[template(path = "desctop/works/anon_category.stpl")]
            struct Template {
                all_tags:         Vec<Tag>,
                category:         WorkCategories,
                //work_cats:        Vec<WorkCategories>,
                object_list:      Vec<Work>,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                all_tags:         _tags,
                category:         _category,
                //work_cats:        _work_categories,
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
            #[template(path = "mobile/works/anon_category.stpl")]
            struct Template {
                all_tags:         Vec<Tag>,
                category:         WorkCategories,
                work_cats:        Vec<WorkCategories>,
                object_list:      Vec<Work>,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                all_tags:         _tags,
                category:         _category,
                work_cats:        _work_categories,
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

pub async fn work_categories_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::tags::dsl::tags;
    use crate::schema::work_categories::dsl::work_categories;
    use crate::utils::get_device_and_ajax;

    let _connection = establish_connection();
    let mut stack = Vec::new();

    let _tag_items = tags_items
        .filter(schema::tags_items::work_id.ne(0))
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

    let _work_cats :Vec<WorkCategories> = work_categories
        .load(&_connection)
        .expect("Error");

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/works/categories.stpl")]
            struct Template {
                request_user: User,
                is_ajax:      bool,
                work_cats:    Vec<WorkCategories>,
                //all_tags:     Vec<Tag>,
            }
            let body = Template {
                request_user: _request_user,
                is_ajax:      is_ajax,
                work_cats:    _work_cats,
                //all_tags:     _tags,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/works/categories.stpl")]
            struct Template {
                request_user: User,
                is_ajax:      bool,
                work_cats:    Vec<WorkCategories>,
                all_tags:     Vec<Tag>,
            }
            let body = Template {
                request_user: _request_user,
                is_ajax:      is_ajax,
                work_cats:    _work_cats,
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
            #[template(path = "desctop/works/anon_categories.stpl")]
            struct Template {
                is_ajax:      bool,
                work_cats:    Vec<WorkCategories>,
                //all_tags:     Vec<Tag>,
            }
            let body = Template {
                is_ajax:      is_ajax,
                work_cats:    _work_cats,
                //all_tags:     _tags,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/works/anon_categories.stpl")]
            struct Template {
                is_ajax:      bool,
                work_cats:    Vec<WorkCategories>,
                all_tags:     Vec<Tag>,
            }
            let body = Template {
                is_ajax:      is_ajax,
                work_cats:    _work_cats,
                all_tags:     _tags,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}
