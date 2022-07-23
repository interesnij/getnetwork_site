use actix_web::{
    web,
    HttpRequest,
    HttpResponse,
    Responder,
    error::InternalError,
    http::StatusCode,
};
use actix_multipart::{Field, Multipart};
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
    BlogCategories,
    NewBlogCategories,
    Blog,
    NewBlog,
    BlogCategory,
    NewBlogCategory,
    BlogImage,
    NewBlogImage,
    BlogVideo,
    NewBlogVideo,
    TagItems,
    NewTagItems,
    Tag,
};
use sailfish::TemplateOnce;


pub fn blog_routes(config: &mut web::ServiceConfig) {
    config.route("/blog_categories/", web::get().to(blog_categories_page));
    config.service(web::resource("/create_blog_categories/")
        .route(web::get().to(create_blog_categories_page))
        .route(web::post().to(create_blog_categories))
    );
    config.service(web::resource("/edit_blog_category/{id}/")
        .route(web::get().to(edit_blog_category_page))
        .route(web::post().to(edit_blog_category))
    );
    config.service(web::resource("/create_blog/")
        .route(web::get().to(create_blog_page))
        .route(web::post().to(create_blog))
    );
    config.service(web::resource("/edit_blog/{id}/")
        .route(web::get().to(edit_blog_page))
        .route(web::post().to(edit_blog))
    );
    config.route("/edit_content_blog/{id}/", web::get().to(edit_content_blog_page));
    config.route("/delete_blog/{id}/", web::get().to(delete_blog));
    config.route("/delete_blog_category/{id}/", web::get().to(delete_blog_category));
    config.service(web::resource("/blog/{cat_id}/{blog_id}/").route(web::get().to(get_blog_page)));
    config.service(web::resource("/blog/{id}/").route(web::get().to(blog_category_page)));
}

pub async fn create_blog_categories_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use schema::blog_categories::dsl::blog_categories;
            use crate::utils::get_device_and_ajax;

            let _connection = establish_connection();
            let _blog_cats:Vec<BlogCategories> = blog_categories
                .load(&_connection)
                .expect("Error");

            let (is_desctop, is_ajax) = get_device_and_ajax(&req);

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/tags/create_categories.stpl")]
                struct Template {
                    request_user: User,
                    blog_cats:    Vec<BlogCategories>,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    blog_cats:    _blog_cats,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/tags/create_categories.stpl")]
                struct Template {
                    request_user: User,
                    blog_cats:    Vec<BlogCategories>,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    blog_cats:    _blog_cats,
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

pub async fn create_blog_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use schema::tags::dsl::tags;
            use schema::blog_categories::dsl::blog_categories;
            use crate::utils::get_device_and_ajax;

            let _connection = establish_connection();
            let _blog_cats:Vec<BlogCategories> = blog_categories
                .load(&_connection)
                .expect("Error");

            let all_tags: Vec<Tag> = tags
                .load(&_connection)
                .expect("Error.");

            let (is_desctop, is_ajax) = get_device_and_ajax(&req);

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/tags/create_blog.stpl")]
                struct Template {
                    request_user: User,
                    blog_cats:    Vec<BlogCategories>,
                    all_tags:     Vec<Tag>,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    blog_cats:    _blog_cats,
                    all_tags:     all_tags,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/tags/create_blog.stpl")]
                struct Template {
                    request_user: User,
                    blog_cats:    Vec<BlogCategories>,
                    all_tags:     Vec<Tag>,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    blog_cats:    _blog_cats,
                    all_tags:     all_tags,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}
pub async fn edit_blog_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let _blog_id: i32 = *_id;
    let _connection = establish_connection();
    let _blogs = blogs.filter(schema::blogs::id.eq(&_blog_id)).load::<Blog>(&_connection).expect("E");
    let _blog = _blogs.into_iter().nth(0).unwrap();

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 && _blog.user_id == _request_user.id {
            use schema::{
                blogs::dsl::blogs,
                tags::dsl::tags,
                blog_images::dsl::blog_images,
                blog_videos::dsl::blog_videos,
                blog_categories::dsl::blog_categories,
            };
            use crate::models::{BlogImage, BlogVideo, Tag};

            let (is_desctop, is_ajax) = get_device_and_ajax(&req);
            let _categories = get_cats_for_blog(&_blog);
            let _all_tags: Vec<Tag> = tags.load(&_connection).expect("Error.");
            let _blog_tags = get_tags_for_blog(&_blog);

            let _images = blog_images.filter(schema::blog_images::blog.eq(_blog.id)).load::<BlogImage>(&_connection).expect("E");
            let _videos = blog_videos.filter(schema::blog_videos::blog.eq(_blog.id)).load::<BlogVideo>(&_connection).expect("E");

            let _blog_cats:Vec<BlogCategories> = blog_categories
                .load(&_connection)
                .expect("Error");
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/tags/edit_blog.stpl")]
                struct Template {
                    request_user: User,
                    blog:         Blog,
                    is_ajax:      bool,
                    images:       Vec<BlogImage>,
                    videos:       Vec<BlogVideo>,
                    tags_list:    Vec<Tag>,
                    blog_cat:     Vec<BlogCategories>,

                }
                let body = Template {
                    request_user: _request_user,
                    blog:         _blog,
                    is_ajax:      is_ajax,
                    images:       _images,
                    videos:       _videos,
                    tags_list:    _all_tags,
                    blog_cat:     _blog_cat,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/tags/edit_blog.stpl")]
                struct Template {
                    request_user: User,
                    blog:         Blog,
                    is_ajax:      bool,
                    images:       Vec<BlogImage>,
                    videos:       Vec<BlogVideo>,
                    tags_list:    Vec<Tag>,
                    blog_cat:     Vec<BlogCategories>,

                }
                let body = Template {
                    request_user: _request_user,
                    blog:         _blog,
                    is_ajax:      is_ajax,
                    images:       _images,
                    videos:       _videos,
                    tags_list:    _all_tags,
                    blog_cat:     _blog_cat,
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

pub async fn edit_content_blog_page(session: Session, mut payload: Multipart, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let _blog_id: i32 = *_id;
    let _connection = establish_connection();
    let _blogs = blogs
        .filter(schema::blogs::id.eq(&_blog_id))
        .load::<Blog>(&_connection)
        .expect("E");

    let _blog = _blogs.into_iter().nth(0).unwrap();

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 && _request_user.id == _blog.user_id {
            use schema::blogs::dsl::blogs;
            use crate::utils::get_device_and_ajax;

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/tags/edit_tag.stpl")]
                struct Template {
                    request_user: User,
                    blog:         Blog,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    blog:        _blog,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/tags/edit_tag.stpl")]
                struct Template {
                    request_user: User,
                    blog:         Blog,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    blog:        _blog,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}
pub async fn edit_content_blog(session: Session, mut payload: Multipart, req: HttpRequest, _id: web::Path<i32>) -> impl Responder {
    let _blog_id: i32 = *_id;
    let _connection = establish_connection();
    let _blogs = blogs
        .filter(schema::blogs::id.eq(&_blog_id))
        .load::<Blog>(&_connection)
        .expect("E");

    let _blog = _blogs.into_iter().nth(0).unwrap();

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 && _request_user.id == _blog.user_id {
            use schema::blogs::dsl::blogs;
            use crate::utils::content_form;

            let form = content_form(payload.borrow_mut()).await;
            let new_content = Content {
                content: Some(form.content.clone()),
            };
            diesel::update(&_blog[0])
            .set(new_content)
            .get_result::<Blog>(&_connection)
            .expect("E");
        }
    }
    HttpResponse::Ok().body(_rendered)
}

pub async fn edit_blog_category_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use schema::blog_categories::dsl::blog_categories;

            let (is_desctop, is_ajax) = get_device_and_ajax(&req);

            let _cat_id: i32 = *_id;
            let _connection = establish_connection();
            let _categorys = blog_categories
                .filter(schema::blog_categories::id.eq(&_cat_id))
                .load::<BlogCategories>(&_connection)
                .expect("E");

            let _category = _categorys.into_iter().nth(0).unwrap();
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/blogs/edit_category.stpl")]
                struct Template {
                    request_user: User,
                    category:     BlogCategories,
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
                #[template(path = "mobile/blogs/edit_category.stpl")]
                struct Template {
                    request_user: User,
                    category:     BlogCategories,
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

pub async fn create_blog_categories(session: Session, mut payload: Multipart) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let form = category_form(payload.borrow_mut()).await;
            let new_cat = NewBlogCategories {
                name:        form.name.clone(),
                description: Some(form.description.clone()),
                position:    form.position,
                image:       Some(form.image.clone()),
                count:       0
            };
            let _new_blog = diesel::insert_into(schema::blog_categories::table)
                .values(&new_cat)
                .get_result::<BlogCategories>(&_connection)
                .expect("Error saving post.");
        }
    }
    return HttpResponse::Ok();
}

pub async fn create_blog(session: Session, mut payload: Multipart) -> impl Responder {
    use crate::schema::tags::dsl::tags;
    use crate::schema::blog_categories::dsl::blog_categories;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();

            let form = item_form(payload.borrow_mut()).await;
            let new_blog = NewBlog::from_blog_form (
                form.title.clone(),
                form.description.clone(),
                form.link.clone(),
                form.main_image.clone(),
                form.is_active.clone(),
                _request_user.id,
            );

            let _blog = diesel::insert_into(schema::blogs::table)
                .values(&new_blog)
                .get_result::<Blog>(&_connection)
                .expect("Error saving blog.");

            for image in form.images.iter() {
                let new_image = NewBlogImage::from_blog_images_form (
                    _blog.id,
                    image.to_string()
                );
                diesel::insert_into(schema::blog_images::table)
                    .values(&new_image)
                    .get_result::<BlogImage>(&_connection)
                    .expect("Error saving blog.");
                };
            for video in form.videos.iter() {
                let new_video = NewBlogVideo::from_blog_videos_form (
                    _blog.id,
                    video.to_string()
                );
                diesel::insert_into(schema::blog_videos::table)
                    .values(&new_video)
                    .get_result::<BlogVideo>(&_connection)
                    .expect("Error saving blog.");
            };
            for category_id in form.category_list.iter() {
                let new_category = NewBlogCategory {
                    blog_categories_id: *category_id,
                    blog_id: _blog.id
                };
                diesel::insert_into(schema::blog_category::table)
                    .values(&new_category)
                    .get_result::<BlogCategory>(&_connection)
                    .expect("Error saving blog.");

                let _category = blog_categories.filter(schema::blog_categories::id.eq(category_id)).load::<BlogCategories>(&_connection).expect("E");
                diesel::update(&_category[0])
                    .set(schema::blog_categories::count.eq(_category[0].count + 1))
                    .get_result::<BlogCategories>(&_connection)
                    .expect("Error.");
            };
            for tag_id in form.tags_list.iter() {
                let new_tag = NewTagItems {
                    tag_id: *tag_id,
                    service_id: 0,
                    store_id: 0,
                    blog_id: _blog.id,
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
                    .set((schema::tags::count.eq(_tag[0].count + 1), schema::tags::blog_count.eq(_tag[0].blog_count + 1)))
                    .get_result::<Tag>(&_connection)
                    .expect("Error.");
            }
        }
    };
    HttpResponse::Ok()
}

pub async fn edit_blog(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::models::EditBlog;
    use crate::schema::blogs::dsl::blogs;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let _blog_id: i32 = *_id;
            let _blogs = blogs
                .filter(schema::blogs::id.eq(_blog_id))
                .load::<Blog>(&_connection)
                .expect("E");

            let _blog = _blogs.into_iter().nth(0).unwrap();

            let _categories = _blog.get_categories();
            let _tags = _blog.get_tags();
            for _category in _categories.iter() {
                diesel::update(_category)
                    .set(schema::blog_categories::count.eq(_category.count - 1))
                    .get_result::<BlogCategories>(&_connection)
                    .expect("Error.");
            };
            for _tag in _tags.iter() {
                diesel::update(_tag)
                .set((schema::tags::count.eq(_tag.count - 1), schema::tags::blog_count.eq(_tag.blog_count - 1)))
                .get_result::<Tag>(&_connection)
                .expect("Error.");
            };

            diesel::delete(schema::blog_images.filter(schema::blog_images::blog.eq(_blog_id))).execute(&_connection).expect("E");
            diesel::delete(schema::blog_videos.filter(schema::blog_videos::blog.eq(_blog_id))).execute(&_connection).expect("E");
            diesel::delete(schema::tags_items.filter(schema::tags_items::blog_id.eq(_blog_id))).execute(&_connection).expect("E");
            diesel::delete(schema::blog_category.filter(schema::blog_category::blog_id.eq(_blog_id))).execute(&_connection).expect("E");

            let form = item_form(payload.borrow_mut()).await;
            let _new_blog = EditBlog {
                title:       form.title.clone(),
                description: Some(form.description.clone()),
                link:        Some(form.link.clone()),
                image:       Some(form.main_image.clone()),
                is_active:   form.is_active.clone()
            };

            diesel::update(&_blog)
            .set(_new_blog)
            .get_result::<Blog>(&_connection)
            .expect("E");

            for _image in form.images.iter() {
                let new_edit_image = NewBlogImage::from_blog_images_form (
                    _blog_id,
                    _image.to_string()
                );
                diesel::insert_into(schema::blog_images::table)
                .values(&new_edit_image)
                .get_result::<BlogImage>(&_connection)
                .expect("E.");
            };
            for _video in form.videos.iter() {
                let new_video = NewBlogVideo::from_blog_videos_form (
                    _blog_id,
                    _video.to_string()
                );
                diesel::insert_into(schema::blog_videos::table)
                .values(&new_video)
                .get_result::<BlogVideo>(&_connection)
                .expect("E.");
            };
            for category_id in form.category_list.iter() {
                let new_category = NewBlogCategory {
                    blog_categories_id: *category_id,
                    blog_id:            _blog_id
                };
                diesel::insert_into(schema::blog_category::table)
                .values(&new_category)
                .get_result::<BlogCategory>(&_connection)
                .expect("E.");

                let _category_2 = blog_categories.filter(schema::blog_categories::id.eq(category_id)).load::<BlogCategories>(&_connection).expect("E");
                diesel::update(&_category_2[0])
                    .set(schema::blog_categories::count.eq(_category_2[0].count + 1))
                    .get_result::<BlogCategories>(&_connection)
                    .expect("Error.");
            };
            for _tag_id in form.tags_list.iter() {
                let _new_tag = NewTagItems {
                    tag_id:     *_tag_id,
                    service_id: 0,
                    store_id:   0,
                    blog_id:    _blog_id,
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
                    .set((schema::tags::count.eq(_tag_2[0].count + 1), schema::tags::blog_count.eq(_tag_2[0].blog_count + 1)))
                    .get_result::<Tag>(&_connection)
                    .expect("Error.");
            };
        }
    }
    HttpResponse::Ok()
}

pub async fn edit_blog_category(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::models::EditBlogCategories;
    use crate::schema::blog_categories::dsl::blog_categories;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let _cat_id: i32 = *_id;
            let _category = blog_categories.filter(schema::blog_categories::id.eq(_cat_id)).load::<BlogCategories>(&_connection).expect("E");

            let form = category_form(payload.borrow_mut()).await;
            let _new_cat = EditBlogCategories {
                name:        form.name.clone(),
                description: Some(form.description.clone()),
                position:    form.position,
                image:       Some(form.image.clone()),
                count:       _category[0].count,
            };

            diesel::update(&_category[0])
                .set(_new_cat)
                .get_result::<BlogCategories>(&_connection)
                .expect("E");
        }
    }
    HttpResponse::Ok()
}

pub async fn delete_blog(session: Session, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::blogs::dsl::blogs;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let _blog_id: i32 = *_id;
            let _blog = blogs.filter(schema::blogs::id.eq(_blog_id)).load::<Blog>(&_connection).expect("E");

            let _categories = get_cats_for_blog(&_blog[0]);
            let _tags = get_tags_for_blog(&_blog[0]);
            for _category in _categories.iter() {
                diesel::update(_category)
                .set(schema::blog_categories::count.eq(_category.count - 1))
                .get_result::<BlogCategories>(&_connection)
                .expect("Error.");
            };
            for _tag in _tags.iter() {
                diesel::update(_tag)
                .set((schema::tags::count.eq(_tag.count - 1), schema::tags::blog_count.eq(_tag.blog_count - 1)))
                .get_result::<Tag>(&_connection)
                .expect("Error.");
            };

            diesel::delete(schema::blog_images.filter(schema::blog_images::blog.eq(_blog_id))).execute(&_connection).expect("E");
            diesel::delete(schema::blog_videos.filter(schema::blog_videos::blog.eq(_blog_id))).execute(&_connection).expect("E");
            diesel::delete(schema::tags_items.filter(schema::tags_items::blog_id.eq(_blog_id))).execute(&_connection).expect("E");
            diesel::delete(schema::blog_category.filter(schema::blog_category::blog_id.eq(_blog_id))).execute(&_connection).expect("E");
            diesel::delete(&_blog[0]).execute(&_connection).expect("E");
        }
    }
    HttpResponse::Ok()
}

pub async fn delete_blog_category(session: Session, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::blog_categories::dsl::blog_categories;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let _cat_id: i32 = *_id;
            let _category = blog_categories.filter(schema::blog_categories::id.eq(_cat_id)).load::<BlogCategories>(&_connection).expect("E");
            diesel::delete(blog_categories.filter(schema::blog_categories::id.eq(_cat_id))).execute(&_connection).expect("E");
        }
    }
    HttpResponse::Ok()
}

pub async fn get_blog_page(session: Session, req: HttpRequest, param: web::Path<(i32,i32)>) -> actix_web::Result<HttpResponse> {
    use schema::blogs::dsl::blogs;
    use schema::blog_categories::dsl::blog_categories;
    use schema::blog_images::dsl::blog_images;
    use schema::blog_videos::dsl::blog_videos;

    let _connection = establish_connection();
    let _blog_id: i32 = param.1;
    let _cat_id: i32 = param.0;

    let (id_desctop, is_ajax) = get_device_and_ajax(&req);

    let _blogs = blogs
        .filter(schema::blogs::id.eq(&_blog_id))
        .load::<Blog>(&_connection)
        .expect("E");

    let _blog = _blogs.into_iter().nth(0).unwrap();

    let _categorys = blog_categories
        .filter(schema::blog_categories::id.eq(&_cat_id))
        .load::<BlogCategories>(&_connection)
        .expect("E");
    let _category = _categorys.into_iter().nth(0).unwrap();

    let _images: Vec<BlogImage> = blog_images.filter(schema::blog_images::blog.eq(&_blog_id)).load(&_connection).expect("E");
    let _videos: Vec<BlogVideo> = blog_videos.filter(schema::blog_videos::blog.eq(&_blog_id)).load(&_connection).expect("E");
    let _categories = _blog.get_categories();
    let _tags = blog.get_tags();
    let _tags_count = _tags.len();

    let prev: Option<i32> = None;
    let next: Option<i32> = None;

    let _category_blogs = _category.get_blogs_ids();
    let _category_blogs_len: usize = _category_blogs.len();
    for (i, item) in _category_blogs.iter().enumerate().rev() {
        if item == _blog_id {
            if (i + 1) != _category_blogs_len {
                prev = Some(_category_blogs[i + 1]);
            };
            if i != 0 {
                next = Some(_category_blogs[i - 1]);
            };
            break;
        }
    };

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/blogs/blog.stpl")]
            struct Template {
                request_user: User,
                object:       Blog,
                images:       Vec<BlogImage>,
                videos:       Vec<BlogVideo>,
                categories:   Vec<BlogCategories>,
                category:     BlogCategories,
                all_tags:     Vec<Tag>,
                tags_count:   usize,
                prev:         Option<i32>,
                next:         Option<i32>,
                is_ajax:      bool,
            }
            let body = Template {
                request_user: _request_user,
                object:     _blog,
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
            #[template(path = "mobile/blogs/blog.stpl")]
            struct Template {
                request_user: User,
                object:       Blog,
                images:       Vec<BlogImage>,
                videos:       Vec<BlogVideo>,
                categories:   Vec<BlogCategories>,
                category:     BlogCategories,
                all_tags:     Vec<Tag>,
                tags_count:   usize,
                prev:         Option<i32>,
                next:         Option<i32>,
                is_ajax:      bool,
            }
            let body = Template {
                request_user: _request_user,
                object:     _blog,
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
            #[template(path = "desctop/blogs/anon_blog.stpl")]
            struct Template {
                object:     Blog,
                images:     Vec<BlogImage>,
                videos:     Vec<BlogVideo>,
                categories: Vec<BlogCategories>,
                category:   BlogCategories,
                all_tags:   Vec<Tag>,
                tags_count: usize,
                prev:       Option<i32>,
                next:       Option<i32>,
                is_ajax:    bool,
            }
            let body = Template {
                object:     _blog,
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
            #[template(path = "mobile/blogs/blog.stpl")]
            struct Template {
                object:     Blog,
                images:     Vec<BlogImage>,
                videos:     Vec<BlogVideo>,
                categories: Vec<BlogCategories>,
                category:   BlogCategories,
                all_tags:   Vec<Tag>,
                tags_count: usize,
                prev:       Option<i32>,
                next:       Option<i32>,
                is_ajax:    bool,
            }
            let body = Template {
                object:     _blog,
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

pub async fn blog_category_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::schema::blog_categories::dsl::blog_categories;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::utils::get_device_and_page_and_ajax;

    let _cat_id: i32 = *_id;
    let _connection = establish_connection();
    let (is_desctop, page, is_ajax) = get_device_and_page_and_ajax(&req);

    let _categorys = blog_categories.filter(schema::blog_categories::id.eq(_cat_id)).load::<BlogCategories>(&_connection).expect("E");
    let _category = _categorys.into_iter().nth(0).unwrap();
    let (object_list, next_page_number) = _category.get_blogs_list(page, 20)

    let mut stack = Vec::new();
    let _tag_items = tags_items
        .filter(schema::tags_items::blog_id.ne(0))
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

    let tags_count = _tags.len();

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/blogs/category.stpl")]
            struct Template {
                request_user:     User,
                all_tags:         Vec<Tag>,
                tags_count:       usize,
                category:         BlogCategories,
                object_list:      Vec<Blog>,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                request_user:     _request_user,
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
            #[template(path = "mobile/blogs/category.stpl")]
            struct Template {
                request_user:     User,
                all_tags:         Vec<Tag>,
                tags_count:       usize,
                category:         BlogCategories,
                object_list:      Vec<Blog>,
                next_page_number: i32,
                is_ajax:          bool,
            }
            let body = Template {
                request_user:     _request_user,
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
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/blogs/anon_category.stpl")]
            struct Template {
                all_tags:         Vec<Tag>,
                tags_count:       usize,
                category:         BlogCategories,
                object_list:      Vec<Blog>,
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
            #[template(path = "mobile/blogs/anon_category.stpl")]
            struct Template {
                all_tags:         Vec<Tag>,
                tags_count:       usize,
                category:         BlogCategories,
                object_list:      Vec<Blog>,
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

pub async fn blog_categories_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::tags::dsl::tags;
    use crate::schema::blog_categories::dsl::blog_categories;
    use crate::utils::get_device_and_ajax;

    let _connection = establish_connection();
    let mut stack = Vec::new();

    let _tag_items = tags_items
        .filter(schema::tags_items::blog_id.ne(0))
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

    let _blog_cats :Vec<BlogCategories> = blog_categories
        .load(&_connection)
        .expect("Error");

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/blogs/categories.stpl")]
            struct Template {
                request_user: User,
                is_ajax:      bool,
                blog_cats:    Vec<BlogCategories>,
                all_tags:     Vec<Tag>,
            }
            let body = Template {
                request_user: _request_user,
                is_ajax:      is_ajax,
                blog_cats:    _blog_cats,
                all_tags:     _tags,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/blogs/categories.stpl")]
            struct Template {
                request_user: User,
                is_ajax:      bool,
                blog_cats:    Vec<BlogCategories>,
                all_tags:     Vec<Tag>,
            }
            let body = Template {
                request_user: _request_user,
                is_ajax:      is_ajax,
                blog_cats:    _blog_cats,
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
            #[template(path = "desctop/blogs/anon_categories.stpl")]
            struct Template {
                is_ajax:      bool,
                blog_cats:    Vec<BlogCategories>,
                all_tags:     Vec<Tag>,
            }
            let body = Template {
                is_ajax:      is_ajax,
                blog_cats:    _blog_cats,
                all_tags:     _tags,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/blogs/anon_categories.stpl")]
            struct Template {
                is_ajax:      bool,
                blog_cats:    Vec<BlogCategories>,
                all_tags:     Vec<Tag>,
            }
            let body = Template {
                is_ajax:      is_ajax,
                blog_cats:    _blog_cats,
                all_tags:     _tags,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}
