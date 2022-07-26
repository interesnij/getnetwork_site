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
    WikiCategories,
    NewWikiCategories,
    Wiki,
    NewWiki,
    WikiCategory,
    NewWikiCategory,
    WikiImage,
    NewWikiImage,
    WikiVideo,
    NewWikiVideo,
    TagItems,
    NewTagItems,
    Tag,
};
use sailfish::TemplateOnce;


pub fn wiki_routes(config: &mut web::ServiceConfig) {
    config.route("/wiki_categories/", web::get().to(wiki_categories_page));
    config.service(web::resource("/create_wiki_categories/")
        .route(web::get().to(create_wiki_categories_page))
        .route(web::post().to(create_wiki_categories))
    );
    config.service(web::resource("/edit_wiki_category/{id}/")
        .route(web::get().to(edit_wiki_category_page))
        .route(web::post().to(edit_wiki_category))
    );
    config.service(web::resource("/create_wiki/")
        .route(web::get().to(create_wiki_page))
        .route(web::post().to(create_wiki))
    );
    config.service(web::resource("/edit_wiki/{id}/")
        .route(web::get().to(edit_wiki_page))
        .route(web::post().to(edit_wiki))
    );
    config.service(web::resource("/edit_content_wiki/{id}/")
        .route(web::get().to(edit_content_wiki_page))
        .route(web::post().to(edit_content_wiki))
    );
    config.route("/delete_wiki/{id}/", web::get().to(delete_wiki));
    config.route("/delete_wiki_category/{id}/", web::get().to(delete_wiki_category));
    config.service(web::resource("/wiki/{cat_id}/{wiki_id}/").route(web::get().to(get_wiki_page)));
    config.service(web::resource("/wiki/{id}/").route(web::get().to(wiki_category_page)));
}

pub async fn create_wiki_categories_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use schema::wiki_categories::dsl::wiki_categories;
            use crate::utils::get_device_and_ajax;

            let _connection = establish_connection();
            let _wiki_cats:Vec<WikiCategories> = wiki_categories
                .load(&_connection)
                .expect("Error");

            let (is_desctop, is_ajax) = get_device_and_ajax(&req);

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/wikis/create_categories.stpl")]
                struct Template {
                    request_user: User,
                    wiki_cats:    Vec<WikiCategories>,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    wiki_cats:    _wiki_cats,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/wikis/create_categories.stpl")]
                struct Template {
                    request_user: User,
                    wiki_cats:    Vec<WikiCategories>,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    wiki_cats:    _wiki_cats,
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

pub async fn create_wiki_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use schema::tags::dsl::tags;
            use schema::wiki_categories::dsl::wiki_categories;
            use crate::utils::get_device_and_ajax;

            let _connection = establish_connection();
            let _wiki_cats:Vec<WikiCategories> = wiki_categories
                .load(&_connection)
                .expect("Error");

            let all_tags: Vec<Tag> = tags
                .load(&_connection)
                .expect("Error.");

            let (is_desctop, is_ajax) = get_device_and_ajax(&req);

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/wikis/create_wiki.stpl")]
                struct Template {
                    request_user: User,
                    wiki_cats:    Vec<WikiCategories>,
                    all_tags:     Vec<Tag>,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    wiki_cats:    _wiki_cats,
                    all_tags:     all_tags,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/wikis/create_wiki.stpl")]
                struct Template {
                    request_user: User,
                    wiki_cats:    Vec<WikiCategories>,
                    all_tags:     Vec<Tag>,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    wiki_cats:    _wiki_cats,
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
pub async fn edit_wiki_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use schema::wikis::dsl::wikis;

    let _wiki_id: i32 = *_id;
    let _connection = establish_connection();
    let _wikis = wikis.filter(schema::wikis::id.eq(&_wiki_id)).load::<Wiki>(&_connection).expect("E");
    let _wiki = _wikis.into_iter().nth(0).unwrap();

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 && _wiki.user_id == _request_user.id {
            use schema::{
                tags::dsl::tags,
                wiki_images::dsl::wiki_images,
                wiki_videos::dsl::wiki_videos,
                wiki_categories::dsl::wiki_categories,
            };
            use crate::utils::get_device_and_ajax;

            let (is_desctop, is_ajax) = get_device_and_ajax(&req);
            let _categories = _wiki.get_categories();
            let _all_tags: Vec<Tag> = tags.load(&_connection).expect("Error.");
            let _wiki_tags = _wiki.get_tags();

            let _images = wiki_images.filter(schema::wiki_images::wiki.eq(_wiki.id)).load::<WikiImage>(&_connection).expect("E");
            let _videos = wiki_videos.filter(schema::wiki_videos::wiki.eq(_wiki.id)).load::<WikiVideo>(&_connection).expect("E");

            let _wiki_cats:Vec<WikiCategories> = wiki_categories
                .load(&_connection)
                .expect("Error");
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/wikis/edit_wiki.stpl")]
                struct Template {
                    request_user: User,
                    wiki:         Wiki,
                    categories:   Vec<WikiCategories>,
                    is_ajax:      bool,
                    images:       Vec<WikiImage>,
                    videos:       Vec<WikiVideo>,
                    all_tags:     Vec<Tag>,
                    wiki_tags:    Vec<Tag>,
                    wiki_cats:    Vec<WikiCategories>,

                }
                let body = Template {
                    request_user: _request_user,
                    wiki:         _wiki,
                    categories:   _categories,
                    is_ajax:      is_ajax,
                    images:       _images,
                    videos:       _videos,
                    all_tags:     _all_tags,
                    wiki_tags:    _wiki_tags,
                    wiki_cats:    _wiki_cats,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/wikis/edit_wiki.stpl")]
                struct Template {
                    request_user: User,
                    wiki:         Wiki,
                    categories:   Vec<WikiCategories>,
                    is_ajax:      bool,
                    images:       Vec<WikiImage>,
                    videos:       Vec<WikiVideo>,
                    all_tags:     Vec<Tag>,
                    wiki_tags:    Vec<Tag>,
                    wiki_cats:    Vec<WikiCategories>,

                }
                let body = Template {
                    request_user: _request_user,
                    wiki:         _wiki,
                    categories:   _categories,
                    is_ajax:      is_ajax,
                    images:       _images,
                    videos:       _videos,
                    all_tags:     _all_tags,
                    wiki_tags:    _wiki_tags,
                    wiki_cats:    _wiki_cats,
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

pub async fn edit_content_wiki_page(session: Session, payload: Multipart, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::schema::wikis::dsl::wikis;

    let _wiki_id: i32 = *_id;
    let _connection = establish_connection();
    let _wikis = wikis
        .filter(schema::wikis::id.eq(&_wiki_id))
        .load::<Wiki>(&_connection)
        .expect("E");

    let _wiki = _wikis.into_iter().nth(0).unwrap();

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 && _request_user.id == _wiki.user_id {
            use crate::utils::get_device_and_ajax;

            let (is_desctop, is_ajax) = get_device_and_ajax(&req);

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/wikis/edit_content_wiki.stpl")]
                struct Template {
                    request_user: User,
                    wiki:         Wiki,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    wiki:        _wiki,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/wikis/edit_content_wiki.stpl")]
                struct Template {
                    request_user: User,
                    wiki:         Wiki,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    wiki:         _wiki,
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
pub async fn edit_content_wiki(session: Session, mut payload: Multipart, req: HttpRequest, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::wikis::dsl::wikis;

    let _wiki_id: i32 = *_id;
    let _connection = establish_connection();
    let _wikis = wikis
        .filter(schema::wikis::id.eq(&_wiki_id))
        .load::<Wiki>(&_connection)
        .expect("E");

    let _wiki = _wikis.into_iter().nth(0).unwrap();

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 && _request_user.id == _wiki.user_id {
            use crate::utils::content_form;

            let form = content_form(payload.borrow_mut()).await;
            diesel::update(&_wiki)
            .set(schema::wikis::content.eq(form.content.clone()))
            .get_result::<Wiki>(&_connection)
            .expect("E");
        }
    }
    HttpResponse::Ok().body("")
}

pub async fn edit_wiki_category_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use schema::wiki_categories::dsl::wiki_categories;
            use crate::utils::get_device_and_ajax;

            let (is_desctop, is_ajax) = get_device_and_ajax(&req);

            let _cat_id: i32 = *_id;
            let _connection = establish_connection();
            let _categorys = wiki_categories
                .filter(schema::wiki_categories::id.eq(&_cat_id))
                .load::<WikiCategories>(&_connection)
                .expect("E");

            let _category = _categorys.into_iter().nth(0).unwrap();
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/wikis/edit_category.stpl")]
                struct Template {
                    request_user: User,
                    category:     WikiCategories,
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
                #[template(path = "mobile/wikis/edit_category.stpl")]
                struct Template {
                    request_user: User,
                    category:     WikiCategories,
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

pub async fn create_wiki_categories(session: Session, mut payload: Multipart) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let form = category_form(payload.borrow_mut(), _request_user.id).await;
            let new_cat = NewWikiCategories {
                name:        form.name.clone(),
                description: Some(form.description.clone()),
                position:    form.position,
                image:       Some(form.image.clone()),
                count:       0
            };
            let _new_wiki = diesel::insert_into(schema::wiki_categories::table)
                .values(&new_cat)
                .get_result::<WikiCategories>(&_connection)
                .expect("Error saving post.");
        }
    }
    return HttpResponse::Ok();
}

pub async fn create_wiki(session: Session, mut payload: Multipart) -> impl Responder {
    use crate::schema::tags::dsl::tags;
    use crate::schema::wiki_categories::dsl::wiki_categories;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();

            let form = item_form(payload.borrow_mut(), _request_user.id).await;
            let new_wiki = NewWiki::from_wiki_form (
                form.title.clone(),
                form.description.clone(),
                form.link.clone(),
                form.main_image.clone(),
                form.is_active.clone(),
                _request_user.id,
            );

            let _wiki = diesel::insert_into(schema::wikis::table)
                .values(&new_wiki)
                .get_result::<Wiki>(&_connection)
                .expect("E.");

            for image in form.images.iter() {
                let new_image = NewWikiImage::from_wiki_images_form (
                    _wiki.id,
                    image.to_string()
                );
                diesel::insert_into(schema::wiki_images::table)
                    .values(&new_image)
                    .get_result::<WikiImage>(&_connection)
                    .expect("E.");
                };
            for video in form.videos.iter() {
                let new_video = NewWikiVideo::from_wiki_videos_form (
                    _wiki.id,
                    video.to_string()
                );
                diesel::insert_into(schema::wiki_videos::table)
                    .values(&new_video)
                    .get_result::<WikiVideo>(&_connection)
                    .expect("E.");
            };
            for category_id in form.category_list.iter() {
                let new_category = NewWikiCategory {
                    wiki_categories_id: *category_id,
                    wiki_id: _wiki.id
                };
                diesel::insert_into(schema::wiki_category::table)
                    .values(&new_category)
                    .get_result::<WikiCategory>(&_connection)
                    .expect("E.");

                let _category = wiki_categories.filter(schema::wiki_categories::id.eq(category_id)).load::<WikiCategories>(&_connection).expect("E");
                diesel::update(&_category[0])
                    .set(schema::wiki_categories::count.eq(_category[0].count + 1))
                    .get_result::<WikiCategories>(&_connection)
                    .expect("Error.");
            };
            for tag_id in form.tags_list.iter() {
                let new_tag = NewTagItems {
                    tag_id: *tag_id,
                    service_id: 0,
                    store_id: 0,
                    blog_id: 0,
                    wiki_id: _wiki.id,
                    work_id: 0,
                    created: chrono::Local::now().naive_utc(),
                };
                diesel::insert_into(schema::tags_items::table)
                    .values(&new_tag)
                    .get_result::<TagItems>(&_connection)
                    .expect("Error.");

                let _tag = tags.filter(schema::tags::id.eq(tag_id)).load::<Tag>(&_connection).expect("E");
                diesel::update(&_tag[0])
                    .set((schema::tags::count.eq(_tag[0].count + 1), schema::tags::wiki_count.eq(_tag[0].wiki_count + 1)))
                    .get_result::<Tag>(&_connection)
                    .expect("Error.");
            }
        }
    };
    HttpResponse::Ok()
}

pub async fn edit_wiki(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::models::EditWiki;
    use crate::schema::wikis::dsl::wikis;
    use crate::schema::tags::dsl::tags;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::wiki_images::dsl::wiki_images;
    use crate::schema::wiki_videos::dsl::wiki_videos;
    use crate::schema::wiki_category::dsl::wiki_category;
    use crate::schema::wiki_categories::dsl::wiki_categories;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let _wiki_id: i32 = *_id;
            let _wikis = wikis
                .filter(schema::wikis::id.eq(_wiki_id))
                .load::<Wiki>(&_connection)
                .expect("E");

            let _wiki = _wikis.into_iter().nth(0).unwrap();

            let _categories = _wiki.get_categories();
            let _tags = _wiki.get_tags();
            for _category in _categories.iter() {
                diesel::update(_category)
                    .set(schema::wiki_categories::count.eq(_category.count - 1))
                    .get_result::<WikiCategories>(&_connection)
                    .expect("Error.");
            };
            for _tag in _tags.iter() {
                diesel::update(_tag)
                .set((schema::tags::count.eq(_tag.count - 1), schema::tags::wiki_count.eq(_tag.wiki_count - 1)))
                .get_result::<Tag>(&_connection)
                .expect("Error.");
            };

            diesel::delete(wiki_images.filter(schema::wiki_images::wiki.eq(_wiki_id))).execute(&_connection).expect("E");
            diesel::delete(wiki_videos.filter(schema::wiki_videos::wiki.eq(_wiki_id))).execute(&_connection).expect("E");
            diesel::delete(tags_items.filter(schema::tags_items::wiki_id.eq(_wiki_id))).execute(&_connection).expect("E");
            diesel::delete(wiki_category.filter(schema::wiki_category::wiki_id.eq(_wiki_id))).execute(&_connection).expect("E");

            let form = item_form(payload.borrow_mut(), _request_user.id).await;
            let _new_wiki = EditWiki {
                title:       form.title.clone(),
                description: Some(form.description.clone()),
                link:        Some(form.link.clone()),
                image:       Some(form.main_image.clone()),
                is_active:   form.is_active.clone()
            };

            diesel::update(&_wiki)
            .set(_new_wiki)
            .get_result::<Wiki>(&_connection)
            .expect("E");

            for _image in form.images.iter() {
                let new_edit_image = NewWikiImage::from_wiki_images_form (
                    _wiki_id,
                    _image.to_string()
                );
                diesel::insert_into(schema::wiki_images::table)
                .values(&new_edit_image)
                .get_result::<WikiImage>(&_connection)
                .expect("E.");
            };
            for _video in form.videos.iter() {
                let new_video = NewWikiVideo::from_wiki_videos_form (
                    _wiki_id,
                    _video.to_string()
                );
                diesel::insert_into(schema::wiki_videos::table)
                .values(&new_video)
                .get_result::<WikiVideo>(&_connection)
                .expect("E.");
            };
            for category_id in form.category_list.iter() {
                let new_category = NewWikiCategory {
                    wiki_categories_id: *category_id,
                    wiki_id:            _wiki_id
                };
                diesel::insert_into(schema::wiki_category::table)
                .values(&new_category)
                .get_result::<WikiCategory>(&_connection)
                .expect("E.");

                let _category_2 = wiki_categories.filter(schema::wiki_categories::id.eq(category_id)).load::<WikiCategories>(&_connection).expect("E");
                diesel::update(&_category_2[0])
                    .set(schema::wiki_categories::count.eq(_category_2[0].count + 1))
                    .get_result::<WikiCategories>(&_connection)
                    .expect("Error.");
            };
            for _tag_id in form.tags_list.iter() {
                let _new_tag = NewTagItems {
                    tag_id:     *_tag_id,
                    service_id: 0,
                    store_id:   0,
                    blog_id:    0,
                    wiki_id:    _wiki_id,
                    work_id:    0,
                    created:    chrono::Local::now().naive_utc(),
                };
                diesel::insert_into(schema::tags_items::table)
                    .values(&_new_tag)
                    .get_result::<TagItems>(&_connection)
                    .expect("Error.");
                let _tag_2 = tags.filter(schema::tags::id.eq(_tag_id)).load::<Tag>(&_connection).expect("E");
                diesel::update(&_tag_2[0])
                    .set((schema::tags::count.eq(_tag_2[0].count + 1), schema::tags::wiki_count.eq(_tag_2[0].wiki_count + 1)))
                    .get_result::<Tag>(&_connection)
                    .expect("Error.");
            };
        }
    }
    HttpResponse::Ok()
}

pub async fn edit_wiki_category(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::models::EditWikiCategories;
    use crate::schema::wiki_categories::dsl::wiki_categories;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let _cat_id: i32 = *_id;
            let _category = wiki_categories.filter(schema::wiki_categories::id.eq(_cat_id)).load::<WikiCategories>(&_connection).expect("E");

            let form = category_form(payload.borrow_mut(), _request_user.id).await;
            let _new_cat = EditWikiCategories {
                name:        form.name.clone(),
                description: Some(form.description.clone()),
                position:    form.position,
                image:       Some(form.image.clone()),
                count:       _category[0].count,
            };

            diesel::update(&_category[0])
                .set(_new_cat)
                .get_result::<WikiCategories>(&_connection)
                .expect("E");
        }
    }
    HttpResponse::Ok()
}

pub async fn delete_wiki(session: Session, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::wikis::dsl::wikis;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::wiki_category::dsl::wiki_category;
    use crate::schema::wiki_videos::dsl::wiki_videos;
    use crate::schema::wiki_images::dsl::wiki_images;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let _wiki_id: i32 = *_id;
            let _wikis = wikis.filter(schema::wikis::id.eq(_wiki_id)).load::<Wiki>(&_connection).expect("E");

            let _wiki = _wikis.into_iter().nth(0).unwrap();
            let _categories = _wiki.get_categories();
            let _tags = _wiki.get_tags();
            for _category in _categories.iter() {
                diesel::update(_category)
                .set(schema::wiki_categories::count.eq(_category.count - 1))
                .get_result::<WikiCategories>(&_connection)
                .expect("Error.");
            };
            for _tag in _tags.iter() {
                diesel::update(_tag)
                .set((schema::tags::count.eq(_tag.count - 1), schema::tags::wiki_count.eq(_tag.wiki_count - 1)))
                .get_result::<Tag>(&_connection)
                .expect("Error.");
            };

            diesel::delete(wiki_images.filter(schema::wiki_images::wiki.eq(_wiki_id))).execute(&_connection).expect("E");
            diesel::delete(wiki_videos.filter(schema::wiki_videos::wiki.eq(_wiki_id))).execute(&_connection).expect("E");
            diesel::delete(tags_items.filter(schema::tags_items::wiki_id.eq(_wiki_id))).execute(&_connection).expect("E");
            diesel::delete(wiki_category.filter(schema::wiki_category::wiki_id.eq(_wiki_id))).execute(&_connection).expect("E");
            diesel::delete(&_wiki).execute(&_connection).expect("E");
        }
    }
    HttpResponse::Ok()
}

pub async fn delete_wiki_category(session: Session, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::wiki_categories::dsl::wiki_categories;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let _cat_id: i32 = *_id;
            let _category = wiki_categories.filter(schema::wiki_categories::id.eq(_cat_id)).load::<WikiCategories>(&_connection).expect("E");
            diesel::delete(wiki_categories.filter(schema::wiki_categories::id.eq(_cat_id))).execute(&_connection).expect("E");
        }
    }
    HttpResponse::Ok()
}

pub async fn get_wiki_page(session: Session, req: HttpRequest, param: web::Path<(i32,i32)>) -> actix_web::Result<HttpResponse> {
    use schema::wikis::dsl::wikis;
    use schema::wiki_categories::dsl::wiki_categories;
    use schema::wiki_images::dsl::wiki_images;
    use schema::wiki_videos::dsl::wiki_videos;
    use crate::utils::get_device_and_ajax;

    let _connection = establish_connection();
    let _wiki_id: i32 = param.1;
    let _cat_id: i32 = param.0;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    let _wikis = wikis
        .filter(schema::wikis::id.eq(&_wiki_id))
        .load::<Wiki>(&_connection)
        .expect("E");

    let _wiki = _wikis.into_iter().nth(0).unwrap();

    let _categorys = wiki_categories
        .filter(schema::wiki_categories::id.eq(&_cat_id))
        .load::<WikiCategories>(&_connection)
        .expect("E");
    let _category = _categorys.into_iter().nth(0).unwrap();

    let _images: Vec<WikiImage> = wiki_images.filter(schema::wiki_images::wiki.eq(&_wiki_id)).load(&_connection).expect("E");
    let _videos: Vec<WikiVideo> = wiki_videos.filter(schema::wiki_videos::wiki.eq(&_wiki_id)).load(&_connection).expect("E");
    let _categories = _wiki.get_categories();
    let _tags = _wiki.get_tags();
    let _tags_count = _tags.len();

    let mut prev: Option<Wiki> = None;
    let mut next: Option<Wiki> = None;

    let _category_wikis = _category.get_wikis_ids();
    let _category_wikis_len = _category_wikis.len();

    for (i, item) in _category_wikis.iter().enumerate().rev() {
        if item == &_wiki_id {
            if (i + 1) != _category_wikis_len {
                let _next = Some(&_category_wikis[i + 1]);
                next = wikis
                    .filter(schema::wikis::id.eq(_next.unwrap()))
                    .filter(schema::wikis::is_active.eq(true))
                    .load::<Wiki>(&_connection)
                    .expect("E")
                    .into_iter()
                    .nth(0);
            };
            if i != 0 {
                let _prev = Some(&_category_wikis[i - 1]);
                prev = wikis
                    .filter(schema::wikis::id.eq(_prev.unwrap()))
                    .filter(schema::wikis::is_active.eq(true))
                    .load::<Wiki>(&_connection)
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
            #[template(path = "desctop/wikis/wiki.stpl")]
            struct Template {
                request_user: User,
                object:       Wiki,
                images:       Vec<WikiImage>,
                videos:       Vec<WikiVideo>,
                categories:   Vec<WikiCategories>,
                category:     WikiCategories,
                all_tags:     Vec<Tag>,
                tags_count:   usize,
                prev:         Option<Wiki>,
                next:         Option<Wiki>,
                is_ajax:      bool,
            }
            let body = Template {
                request_user: _request_user,
                object:     _wiki,
                images:     _images,
                videos:     _videos,
                categories: _categories,
                category:   _category,
                all_tags:   _tags,
                tags_count: _tags_count,
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
            #[template(path = "mobile/wikis/wiki.stpl")]
            struct Template {
                request_user: User,
                object:       Wiki,
                images:       Vec<WikiImage>,
                videos:       Vec<WikiVideo>,
                categories:   Vec<WikiCategories>,
                category:     WikiCategories,
                all_tags:     Vec<Tag>,
                tags_count:   usize,
                prev:         Option<Wiki>,
                next:         Option<Wiki>,
                is_ajax:      bool,
            }
            let body = Template {
                request_user: _request_user,
                object:     _wiki,
                images:     _images,
                videos:     _videos,
                categories: _categories,
                category:   _category,
                all_tags:   _tags,
                tags_count: _tags_count,
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
            #[template(path = "desctop/wikis/anon_wiki.stpl")]
            struct Template {
                object:     Wiki,
                images:     Vec<WikiImage>,
                videos:     Vec<WikiVideo>,
                categories: Vec<WikiCategories>,
                category:   WikiCategories,
                all_tags:   Vec<Tag>,
                tags_count: usize,
                prev:       Option<Wiki>,
                next:       Option<Wiki>,
                is_ajax:    bool,
            }
            let body = Template {
                object:     _wiki,
                images:     _images,
                videos:     _videos,
                categories: _categories,
                category:   _category,
                all_tags:   _tags,
                tags_count: _tags_count,
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
            #[template(path = "mobile/wikis/anon_wiki.stpl")]
            struct Template {
                object:     Wiki,
                images:     Vec<WikiImage>,
                videos:     Vec<WikiVideo>,
                categories: Vec<WikiCategories>,
                category:   WikiCategories,
                all_tags:   Vec<Tag>,
                tags_count: usize,
                prev:       Option<Wiki>,
                next:       Option<Wiki>,
                is_ajax:    bool,
            }
            let body = Template {
                object:     _wiki,
                images:     _images,
                videos:     _videos,
                categories: _categories,
                category:   _category,
                all_tags:   _tags,
                tags_count: _tags_count,
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

pub async fn wiki_category_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::schema::wiki_categories::dsl::wiki_categories;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::utils::{get_device_and_ajax, get_page};

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let page = get_page(&req);

    let _cat_id: i32 = *_id;
    let _connection = establish_connection();

    let _categorys = wiki_categories.filter(schema::wiki_categories::id.eq(_cat_id)).load::<WikiCategories>(&_connection).expect("E");
    let _category = _categorys.into_iter().nth(0).unwrap();
    let (object_list, next_page_number) = _category.get_wikis_list(page, 20);

    let mut stack = Vec::new();
    let _tag_items = tags_items
        .filter(schema::tags_items::wiki_id.ne(0))
        .select(schema::tags_items::tag_id)
        .load::<i32>(&_connection)
        .expect("E");

    let _wiki_categorys = wiki_categories
        .load::<WikiCategories>(&_connection)
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

    let tags_count = _tags.len();

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/wikis/category.stpl")]
            struct Template {
                request_user:     User,
                all_tags:         Vec<Tag>,
                tags_count:       usize,
                category:         WikiCategories,
                wiki_cats:        Vec<WikiCategories>,
                object_list:      Vec<Wiki>,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                request_user:     _request_user,
                all_tags:         _tags,
                tags_count:       tags_count,
                category:         _category,
                wiki_cats:        _wiki_categorys,
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
            #[template(path = "mobile/wikis/category.stpl")]
            struct Template {
                request_user:     User,
                all_tags:         Vec<Tag>,
                tags_count:       usize,
                category:         WikiCategories,
                wiki_cats:        Vec<WikiCategories>,
                object_list:      Vec<Wiki>,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                request_user:     _request_user,
                all_tags:         _tags,
                tags_count:       tags_count,
                category:         _category,
                wiki_cats:        _wiki_categorys,
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
            #[template(path = "desctop/wikis/anon_category.stpl")]
            struct Template {
                all_tags:         Vec<Tag>,
                tags_count:       usize,
                category:         WikiCategories,
                object_list:      Vec<Wiki>,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                all_tags:         _tags,
                tags_count:       tags_count,
                category:        _category,
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
            #[template(path = "mobile/wikis/anon_category.stpl")]
            struct Template {
                all_tags:         Vec<Tag>,
                tags_count:       usize,
                category:         WikiCategories,
                object_list:      Vec<Wiki>,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                all_tags:         _tags,
                tags_count:       tags_count,
                category:        _category,
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

pub async fn wiki_categories_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::tags::dsl::tags;
    use crate::schema::wiki_categories::dsl::wiki_categories;
    use crate::utils::get_device_and_ajax;

    let _connection = establish_connection();
    let mut stack = Vec::new();

    let _tag_items = tags_items
        .filter(schema::tags_items::wiki_id.ne(0))
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

    let _wiki_cats :Vec<WikiCategories> = wiki_categories
        .load(&_connection)
        .expect("Error");

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/wikis/categories.stpl")]
            struct Template {
                request_user: User,
                is_ajax:      bool,
                wiki_cats:    Vec<WikiCategories>,
                all_tags:     Vec<Tag>,
            }
            let body = Template {
                request_user: _request_user,
                is_ajax:      is_ajax,
                wiki_cats:    _wiki_cats,
                all_tags:     _tags,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/wikis/categories.stpl")]
            struct Template {
                request_user: User,
                is_ajax:      bool,
                wiki_cats:    Vec<WikiCategories>,
                all_tags:     Vec<Tag>,
            }
            let body = Template {
                request_user: _request_user,
                is_ajax:      is_ajax,
                wiki_cats:    _wiki_cats,
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
            #[template(path = "desctop/wikis/anon_categories.stpl")]
            struct Template {
                is_ajax:      bool,
                wiki_cats:    Vec<WikiCategories>,
                all_tags:     Vec<Tag>,
            }
            let body = Template {
                is_ajax:      is_ajax,
                wiki_cats:    _wiki_cats,
                all_tags:     _tags,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/wikis/anon_categories.stpl")]
            struct Template {
                is_ajax:      bool,
                wiki_cats:    Vec<WikiCategories>,
                all_tags:     Vec<Tag>,
            }
            let body = Template {
                is_ajax:      is_ajax,
                wiki_cats:    _wiki_cats,
                all_tags:     _tags,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}
