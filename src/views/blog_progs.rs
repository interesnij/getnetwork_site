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
    establish_connection,
    is_signed_in,
    get_request_user_data,
    get_first_load_page,
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
    BlogVideo,
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
    config.service(web::resource("/edit_content_blog/{id}/")
        .route(web::get().to(edit_content_blog_page))
        .route(web::post().to(edit_content_blog))
    );
    config.service(web::resource("/blog/{cat_id}/{blog_id}/").route(web::get().to(get_blog_page)));
    config.service(web::resource("/blogs/{id}/").route(web::get().to(blog_category_page)));

    config.route("/delete_blog/{id}/", web::get().to(delete_blog));
    config.route("/delete_blog_category/{id}/", web::get().to(delete_blog_category));
    config.route("/publish_blog/{id}/", web::get().to(publish_blog));
    config.route("/hide_blog/{id}/", web::get().to(hide_blog));

    config.route("/create_blog_images/{id}/", web::post().to(create_blog_images));
    config.route("/create_blog_videos/{id}/", web::post().to(create_blog_videos));
    config.route("/delete_blog_image/{id}/", web::get().to(delete_blog_image));
    config.route("/delete_blog_video/{id}/", web::get().to(delete_blog_video));
}

pub async fn create_blog_categories_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Создание категории блога".to_string(),
            "вебсервисы.рф: Создание категории блога".to_string(),
            "/create_blog_categories/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
        ).await
    }
    else if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use schema::blog_categories::dsl::blog_categories;

            let _connection = establish_connection();
            let _blog_cats:Vec<BlogCategories> = blog_categories
                .load(&_connection)
                .expect("Error");

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/blogs/create_categories.stpl")]
                struct Template {
                    request_user: User,
                    blog_cats:    Vec<BlogCategories>,
                    is_ajax:      i32,
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
                #[template(path = "mobile/blogs/create_categories.stpl")]
                struct Template {
                    blog_cats:    Vec<BlogCategories>,
                    is_ajax:      i32,
                }
                let body = Template {
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
    use crate::utils::get_device_and_ajax;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Создание статьи блога".to_string(),
            "вебсервисы.рф: Создание статьи блога".to_string(),
            "/create_blog/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
        ).await
    }
    else if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use schema::tags::dsl::tags;
            use schema::blog_categories::dsl::blog_categories;

            let _connection = establish_connection();
            let _blog_cats:Vec<BlogCategories> = blog_categories
                .load(&_connection)
                .expect("Error");

            let all_tags: Vec<Tag> = tags
                .load(&_connection)
                .expect("Error.");

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/blogs/create_blog.stpl")]
                struct Template {
                    request_user: User,
                    blog_cats:    Vec<BlogCategories>,
                    all_tags:     Vec<Tag>,
                    is_ajax:      i32,
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
                #[template(path = "mobile/blogs/create_blog.stpl")]
                struct Template {
                    blog_cats:    Vec<BlogCategories>,
                    all_tags:     Vec<Tag>,
                    is_ajax:      i32,
                }
                let body = Template {
                    blog_cats:    _blog_cats,
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
pub async fn edit_blog_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Создание статьи блога".to_string(),
            "вебсервисы.рф: Создание статьи блога".to_string(),
            "/create_blog/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
        ).await
    }
    else {
        use schema::blogs::dsl::blogs;

        let _blog_id: i32 = *_id;
        let _connection = establish_connection();
        let _blogs = blogs.filter(schema::blogs::id.eq(&_blog_id)).load::<Blog>(&_connection).expect("E");
        let _blog = _blogs.into_iter().nth(0).unwrap();

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            if _request_user.perm == 60 && _blog.user_id == _request_user.id {
                use schema::{
                    tags::dsl::tags,
                    blog_categories::dsl::blog_categories,
                };

                let _categories = _blog.get_categories();
                let _all_tags: Vec<Tag> = tags.load(&_connection).expect("Error.");
                let _blog_tags = _blog.get_tags();

                let _blog_cats:Vec<BlogCategories> = blog_categories
                    .load(&_connection)
                    .expect("Error");
                if is_desctop {
                    #[derive(TemplateOnce)]
                    #[template(path = "desctop/blogs/edit_blog.stpl")]
                    struct Template {
                        request_user: User,
                        object:       Blog,
                        categories:   Vec<BlogCategories>,
                        is_ajax:      i32,
                        all_tags:     Vec<Tag>,
                        blog_tags:    Vec<Tag>,
                        blog_cats:    Vec<BlogCategories>,
                    }
                    let body = Template {
                        request_user: _request_user,
                        object:       _blog,
                        categories:   _categories,
                        is_ajax:      is_ajax,
                        all_tags:     _all_tags,
                        blog_tags:    _blog_tags,
                        blog_cats:    _blog_cats,
                    }
                    .render_once()
                    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
                }
                else {
                    #[derive(TemplateOnce)]
                    #[template(path = "mobile/blogs/edit_blog.stpl")]
                    struct Template {
                        object:       Blog,
                        categories:   Vec<BlogCategories>,
                        is_ajax:      i32,
                        all_tags:     Vec<Tag>,
                        blog_tags:    Vec<Tag>,
                        blog_cats:    Vec<BlogCategories>,
                    }
                    let body = Template {
                        object:       _blog,
                        categories:   _categories,
                        is_ajax:      is_ajax,
                        all_tags:     _all_tags,
                        blog_tags:    _blog_tags,
                        blog_cats:    _blog_cats,
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
}

pub async fn edit_content_blog_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;
    use schema::blogs::dsl::blogs;

    let _blog_id: i32 = *_id;
    let _connection = establish_connection();
    let _blogs = blogs
        .filter(schema::blogs::id.eq(&_blog_id))
        .load::<Blog>(&_connection)
        .expect("E");

    let _blog = _blogs.into_iter().nth(0).unwrap();

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Изменение текста статьи блога ".to_string() + &_blog.title,
            "вебсервисы.рф: Изменение текста статьи блога ".to_string() + &_blog.title,
            "/edit_content_blog/".to_string() + &_blog.id.to_string() + &"/".to_string(),
            _blog.get_image(),
        ).await
    }

    else if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 && _request_user.id == _blog.user_id {
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/blogs/edit_content_blog.stpl")]
                struct Template {
                    request_user: User,
                    blog:         Blog,
                    is_ajax:      i32,
                }
                let body = Template {
                    request_user: _request_user,
                    blog:         _blog,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/blogs/edit_content_blog.stpl")]
                struct Template {
                    blog:         Blog,
                    is_ajax:      i32,
                }
                let body = Template {
                    blog:         _blog,
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
pub async fn edit_content_blog(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::blogs::dsl::blogs;

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
            use crate::utils::content_form;

            let form = content_form(payload.borrow_mut()).await;
            diesel::update(&_blog)
            .set(schema::blogs::content.eq(form.content.clone()))
            .get_result::<Blog>(&_connection)
            .expect("E");
        }
    }
    HttpResponse::Ok().body("")
}

pub async fn edit_blog_category_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;
    use schema::blog_categories::dsl::blog_categories;

    let _cat_id: i32 = *_id;
    let _connection = establish_connection();
    let _categorys = blog_categories
        .filter(schema::blog_categories::id.eq(&_cat_id))
        .load::<BlogCategories>(&_connection)
        .expect("E");
    let _category = _categorys.into_iter().nth(0).unwrap();

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Изменение категории блога ".to_string() + &_category.name,
            "вебсервисы.рф: Изменение категории блога ".to_string() + &_category.name,
            "/edit_blog_category/".to_string() + &_category.id.to_string() + &"/".to_string(),
            _category.get_image(),
        ).await
    }
    else if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/blogs/edit_category.stpl")]
                struct Template {
                    request_user: User,
                    category:     BlogCategories,
                    is_ajax:      i32,
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
                    category:     BlogCategories,
                    is_ajax:      i32,
                }
                let body = Template {
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
            use crate::utils::category_form;

            let _connection = establish_connection();
            let form = category_form(payload.borrow_mut(), _request_user.id).await;
            let new_cat = NewBlogCategories {
                name:        form.name.clone(),
                description: Some(form.description.clone()),
                position:    form.position,
                image:       Some(form.image.clone()),
                count:       0,
                view:        0,
                height:      0.0,
                seconds:     0,
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
            use crate::utils::item_form;

            let _connection = establish_connection();

            let form = item_form(payload.borrow_mut(), _request_user.id).await;
            let new_blog = NewBlog::create (
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
    use crate::schema::tags::dsl::tags;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::blog_category::dsl::blog_category;
    use crate::schema::blog_categories::dsl::blog_categories;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use crate::utils::item_form;

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

            diesel::delete(tags_items.filter(schema::tags_items::blog_id.eq(_blog_id))).execute(&_connection).expect("E");
            diesel::delete(blog_category.filter(schema::blog_category::blog_id.eq(_blog_id))).execute(&_connection).expect("E");

            let form = item_form(payload.borrow_mut(), _request_user.id).await;
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
            use crate::utils::category_form;

            let _connection = establish_connection();
            let _cat_id: i32 = *_id;
            let _category = blog_categories.filter(schema::blog_categories::id.eq(_cat_id)).load::<BlogCategories>(&_connection).expect("E");

            let form = category_form(payload.borrow_mut(), _request_user.id).await;
            let _new_cat = EditBlogCategories {
                name:        form.name.clone(),
                description: Some(form.description.clone()),
                position:    form.position,
                image:       Some(form.image.clone()),
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
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::blog_category::dsl::blog_category;
    use crate::schema::blog_videos::dsl::blog_videos;
    use crate::schema::blog_images::dsl::blog_images;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let _blog_id: i32 = *_id;
            let _blogs = blogs.filter(schema::blogs::id.eq(_blog_id)).load::<Blog>(&_connection).expect("E");

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

            diesel::delete(blog_images.filter(schema::blog_images::blog.eq(_blog_id))).execute(&_connection).expect("E");
            diesel::delete(blog_videos.filter(schema::blog_videos::blog.eq(_blog_id))).execute(&_connection).expect("E");
            diesel::delete(tags_items.filter(schema::tags_items::blog_id.eq(_blog_id))).execute(&_connection).expect("E");
            diesel::delete(blog_category.filter(schema::blog_category::blog_id.eq(_blog_id))).execute(&_connection).expect("E");
            diesel::delete(&_blog).execute(&_connection).expect("E");
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
    use crate::utils::get_device_and_ajax;
    use schema::blogs::dsl::blogs;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let _connection = establish_connection();
    let _blog_id: i32 = param.1;
    let _cat_id: i32 = param.0;
    let _blogs = blogs
        .filter(schema::blogs::id.eq(&_blog_id))
        .load::<Blog>(&_connection)
        .expect("E");
    let _blog = _blogs.into_iter().nth(0).unwrap();

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Статья блога ".to_string() + &_blog.title,
            "вебсервисы.рф: Статья блога ".to_string() + &_blog.title,
            "/blog/".to_string() + &_cat_id.to_string() + &"/".to_string() + &_blog.id.to_string() + &"/".to_string(),
            _blog.get_image(),
        ).await
    }
    else {
        use schema::blog_categories::dsl::blog_categories;
        use schema::blog_images::dsl::blog_images;
        use schema::blog_videos::dsl::blog_videos;

        let all_categories = blog_categories
            .load::<BlogCategories>(&_connection)
            .expect("E");

        let _categorys = blog_categories
            .filter(schema::blog_categories::id.eq(&_cat_id))
            .load::<BlogCategories>(&_connection)
            .expect("E");
        let _category = _categorys.into_iter().nth(0).unwrap();

        let _images: Vec<BlogImage> = blog_images.filter(schema::blog_images::blog.eq(&_blog_id)).load(&_connection).expect("E");
        let _videos: Vec<BlogVideo> = blog_videos.filter(schema::blog_videos::blog.eq(&_blog_id)).load(&_connection).expect("E");
        let _tags = _blog.get_tags();

        let mut prev: Option<Blog> = None;
        let mut next: Option<Blog> = None;

        let _category_blogs = _category.get_blogs_ids();
        let _category_blogs_len = _category_blogs.len();

        for (i, item) in _category_blogs.iter().enumerate().rev() {
            if item == &_blog_id {
                if (i + 1) != _category_blogs_len {
                    let _next = Some(&_category_blogs[i + 1]);
                    next = blogs
                        .filter(schema::blogs::id.eq(_next.unwrap()))
                        .filter(schema::blogs::is_active.eq(true))
                        .load::<Blog>(&_connection)
                        .expect("E")
                        .into_iter()
                        .nth(0);
                };
                if i != 0 {
                    let _prev = Some(&_category_blogs[i - 1]);
                    prev = blogs
                        .filter(schema::blogs::id.eq(_prev.unwrap()))
                        .filter(schema::blogs::is_active.eq(true))
                        .load::<Blog>(&_connection)
                        .expect("E")
                        .into_iter()
                        .nth(0);
                };
                break;
            }
        };

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            if _blog.is_active == false && _request_user.perm < 10 {
                use crate::utils::get_private_page;
                get_private_page (
                    is_ajax,
                    _request_user,
                    is_desctop,
                    "Статья блога ".to_string() + &_blog.title,
                    "вебсервисы.рф: Статья блога ".to_string() + &_blog.title,
                    "/blog/".to_string() + &_cat_id.to_string() + &"/".to_string() + &_blog.id.to_string() + &"/".to_string(),
                    _blog.get_image(),
                ).await
            }
            else if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/blogs/blog.stpl")]
                struct Template {
                    request_user: User,
                    object:       Blog,
                    images:       Vec<BlogImage>,
                    videos:       Vec<BlogVideo>,
                    blog_cats:    Vec<BlogCategories>,
                    category:     BlogCategories,
                    all_tags:     Vec<Tag>,
                    prev:         Option<Blog>,
                    next:         Option<Blog>,
                    is_ajax:      i32,
                }
                let body = Template {
                    request_user: _request_user,
                    object:       _blog,
                    images:       _images,
                    videos:       _videos,
                    blog_cats:    all_categories,
                    category:     _category,
                    all_tags:     _tags,
                    prev:         prev,
                    next:         next,
                    is_ajax:      is_ajax,
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
                    blog_cats:    Vec<BlogCategories>,
                    category:     BlogCategories,
                    all_tags:     Vec<Tag>,
                    prev:         Option<Blog>,
                    next:         Option<Blog>,
                    is_ajax:      i32,
                }
                let body = Template {
                    request_user: _request_user,
                    object:       _blog,
                    images:       _images,
                    videos:       _videos,
                    category:     _category,
                    blog_cats:    all_categories,
                    all_tags:     _tags,
                    prev:         prev,
                    next:         next,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            if _blog.is_active == false {
                use crate::utils::get_anon_private_page;
                get_anon_private_page (
                    is_ajax,
                    is_desctop,
                    "Статья блога ".to_string() + &_blog.title,
                    "вебсервисы.рф: Статья блога ".to_string() + &_blog.title,
                    "/blog/".to_string() + &_cat_id.to_string() + &"/".to_string() + &_blog.id.to_string() + &"/".to_string(),
                    _blog.get_image(),
                ).await
            }
            else if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/blogs/anon_blog.stpl")]
                struct Template {
                    object:     Blog,
                    images:     Vec<BlogImage>,
                    videos:     Vec<BlogVideo>,
                    blog_cats:  Vec<BlogCategories>,
                    category:   BlogCategories,
                    all_tags:   Vec<Tag>,
                    prev:       Option<Blog>,
                    next:       Option<Blog>,
                    is_ajax:    i32,
                }
                let body = Template {
                    object:     _blog,
                    images:     _images,
                    videos:     _videos,
                    blog_cats:  all_categories,
                    category:   _category,
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
                #[template(path = "mobile/blogs/anon_blog.stpl")]
                struct Template {
                    object:     Blog,
                    images:     Vec<BlogImage>,
                    videos:     Vec<BlogVideo>,
                    blog_cats:  Vec<BlogCategories>,
                    category:   BlogCategories,
                    all_tags:   Vec<Tag>,
                    prev:       Option<Blog>,
                    next:       Option<Blog>,
                    is_ajax:    i32,
                }
                let body = Template {
                    object:     _blog,
                    images:     _images,
                    videos:     _videos,
                    blog_cats:  all_categories,
                    category:   _category,
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
}

pub async fn blog_category_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::schema::blog_categories::dsl::blog_categories;
    use crate::utils::{get_device_and_ajax, get_page};

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let _cat_id: i32 = *_id;
    let _connection = establish_connection();

    let all_categories = blog_categories
        .load::<BlogCategories>(&_connection)
        .expect("E");

    let _categorys = blog_categories
        .filter(schema::blog_categories::id.eq(_cat_id))
        .load::<BlogCategories>(&_connection)
        .expect("E");
    let _category = _categorys.into_iter().nth(0).unwrap();

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Категория блога ".to_string() + &_category.name,
            "вебсервисы.рф: Категория блога ".to_string() + &_category.name,
            "/blogs/".to_string() + &_category.id.to_string() + &"/".to_string(),
            _category.get_image(),
        ).await
    }
    else {
        use crate::schema::tags_items::dsl::tags_items;

        let page = get_page(&req);
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

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            let (object_list, next_page_number) = _category.get_blogs_list(page, 20, _request_user.is_superuser());
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/blogs/category.stpl")]
                struct Template {
                    request_user:     User,
                    all_tags:         Vec<Tag>,
                    category:         BlogCategories,
                    blog_cats:        Vec<BlogCategories>,
                    object_list:      Vec<Blog>,
                    next_page_number: i32,
                    is_ajax:          i32,
                }
                let body = Template {
                    request_user:     _request_user,
                    all_tags:         _tags,
                    category:        _category,
                    blog_cats:        all_categories,
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
                    all_tags:         Vec<Tag>,
                    category:         BlogCategories,
                    blog_cats:        Vec<BlogCategories>,
                    object_list:      Vec<Blog>,
                    next_page_number: i32,
                    is_ajax:          i32,
                }
                let body = Template {
                    all_tags:         _tags,
                    category:        _category,
                    blog_cats:        all_categories,
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
            let (object_list, next_page_number) = _category.get_blogs_list(page, 20, false);
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/blogs/anon_category.stpl")]
                struct Template {
                    all_tags:         Vec<Tag>,
                    category:         BlogCategories,
                    blog_cats:        Vec<BlogCategories>,
                    object_list:      Vec<Blog>,
                    next_page_number: i32,
                    is_ajax:          i32,
                }
                let body = Template {
                    all_tags:         _tags,
                    category:         _category,
                    blog_cats:        all_categories,
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
                    category:         BlogCategories,
                    blog_cats:        Vec<BlogCategories>,
                    object_list:      Vec<Blog>,
                    next_page_number: i32,
                    is_ajax:          i32,
                }
                let body = Template {
                    all_tags:         _tags,
                    category:         _category,
                    blog_cats:        all_categories,
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

pub async fn blog_categories_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Категории блога".to_string(),
            "вебсервисы.рф: Категории блога".to_string(),
            "/work_categories/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
        ).await
    }
    else {
        use crate::schema::{
            tags_items::dsl::tags_items,
            tags::dsl::tags,
            blog_categories::dsl::blog_categories,
            stat_blog_categories::dsl::stat_blog_categories,
        };
        use crate::models::StatBlogCategorie;

        let _connection = establish_connection();
        let mut stack = Vec::new();

        let _tag_items = tags_items
            .filter(schema::tags_items::blog_id.ne(0))
            .select(schema::tags_items::tag_id)
            .load::<i32>(&_connection)
            .expect("E");

        let _stat: StatBlogCategorie;
        let _stats = stat_blog_categories
            .limit(1)
            .load::<StatBlogCategorie>(&_connection)
            .expect("E");
        if _stats.len() > 0 {
            _stat = _stats.into_iter().nth(0).unwrap();
        }
        else {
            use crate::models::NewStatBlogCategorie;
            let form = NewStatBlogCategorie {
                view: 0,
                height: 0.0,
                seconds: 0,
            };
            _stat = diesel::insert_into(schema::stat_blog_categories::table)
                .values(&form)
                .get_result::<StatBlogCategorie>(&_connection)
                .expect("Error.");
        }

        for _tag_item in _tag_items.iter() {
            if !stack.iter().any(|&i| i==_tag_item) {
                stack.push(_tag_item);
            }
        };
        let _tags = tags
            .filter(schema::tags::id.eq_any(stack))
            .load::<Tag>(&_connection)
            .expect("could not load tags");

        let _blog_cats = blog_categories
            .load::<BlogCategories>(&_connection)
            .expect("Error");

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/blogs/categories.stpl")]
                struct Template {
                    request_user: User,
                    is_ajax:      i32,
                    blog_cats:    Vec<BlogCategories>,
                    all_tags:     Vec<Tag>,
                    stat:         StatBlogCategorie,
                }
                let body = Template {
                    request_user: _request_user,
                    is_ajax:      is_ajax,
                    blog_cats:    _blog_cats,
                    all_tags:     _tags,
                    stat:         _stat,
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
                    is_ajax:      i32,
                    blog_cats:    Vec<BlogCategories>,
                    all_tags:     Vec<Tag>,
                    stat:         StatBlogCategorie,
                }
                let body = Template {
                    request_user: _request_user,
                    is_ajax:      is_ajax,
                    blog_cats:    _blog_cats,
                    all_tags:     _tags,
                    stat:         _stat,
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
                    is_ajax:      i32,
                    blog_cats:    Vec<BlogCategories>,
                    all_tags:     Vec<Tag>,
                    stat:         StatBlogCategorie,
                }
                let body = Template {
                    is_ajax:      is_ajax,
                    blog_cats:    _blog_cats,
                    all_tags:     _tags,
                    stat:         _stat,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/blogs/anon_categories.stpl")]
                struct Template {
                    is_ajax:      i32,
                    blog_cats:    Vec<BlogCategories>,
                    all_tags:     Vec<Tag>,
                    stat:         StatBlogCategorie,
                }
                let body = Template {
                    is_ajax:      is_ajax,
                    blog_cats:    _blog_cats,
                    all_tags:     _tags,
                    stat:         _stat,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn publish_blog(session: Session, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use crate::schema::blogs::dsl::blogs;

            let _connection = establish_connection();
            let _id: i32 = *_id;
            let _blog = blogs
                .filter(schema::blogs::id.eq(_id))
                .load::<Blog>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();

            let _categories = _blog.get_categories();
            for _category in _categories.iter() {
                diesel::update(_category)
                    .set(schema::blog_categories::count.eq(_category.count + 1))
                    .get_result::<BlogCategories>(&_connection)
                    .expect("Error.");
            };

            diesel::update(&_blog)
                .set(schema::blogs::is_active.eq(true))
                .get_result::<Blog>(&_connection)
                .expect("Error.");
        }
    }
    HttpResponse::Ok()
}
pub async fn hide_blog(session: Session, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use crate::schema::blogs::dsl::blogs;

            let _connection = establish_connection();
            let _id: i32 = *_id;
            let _blog = blogs
                .filter(schema::blogs::id.eq(_id))
                .load::<Blog>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
            .unwrap();

            let _categories = _blog.get_categories();
            for _category in _categories.iter() {
                diesel::update(_category)
                    .set(schema::blog_categories::count.eq(_category.count - 1))
                    .get_result::<BlogCategories>(&_connection)
                    .expect("Error.");
            };

            diesel::update(&_blog)
                .set(schema::blogs::is_active.eq(false))
                .get_result::<Blog>(&_connection)
                .expect("Error.");
        }
    }
    HttpResponse::Ok()
}

pub async fn create_blog_images(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use crate::utils::images_form;
            use crate::schema::blogs::dsl::blogs;
            use crate::models::NewBlogImage;

            let _connection = establish_connection();
            let _blogs = blogs.filter(schema::blogs::id.eq(*_id)).load::<Blog>(&_connection).expect("E");
            let _blog = _blogs.into_iter().nth(0).unwrap();

            let form = images_form(payload.borrow_mut(), _request_user.id).await;
            for image in form.images.iter() {
                let new_image = NewBlogImage::create (
                    _blog.id,
                    image.to_string()
                );
                diesel::insert_into(schema::blog_images::table)
                    .values(&new_image)
                    .get_result::<BlogImage>(&_connection)
                    .expect("E.");
                };
        }
    }
    HttpResponse::Ok()
}
pub async fn create_blog_videos(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use crate::utils::videos_form;
            use crate::schema::blogs::dsl::blogs;
            use crate::models::NewBlogVideo;

            let _connection = establish_connection();
            let _blogs = blogs.filter(schema::blogs::id.eq(*_id)).load::<Blog>(&_connection).expect("E");
            let _blog = _blogs.into_iter().nth(0).unwrap();

            let form = videos_form(payload.borrow_mut(), _request_user.id).await;
            for video in form.videos.iter() {
                let new_video = NewBlogVideo::create (
                    _blog.id,
                    video.to_string()
                );
                diesel::insert_into(schema::blog_videos::table)
                    .values(&new_video)
                    .get_result::<BlogVideo>(&_connection)
                    .expect("Error saving blog.");
            };
        }
    }
    HttpResponse::Ok()
}

pub async fn delete_blog_image(session: Session, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use crate::schema::blog_images::dsl::blog_images;
            diesel::delete(blog_images.filter(schema::blog_images::id.eq(*_id))).execute(&establish_connection()).expect("E");
        }
    }
    HttpResponse::Ok()
}
pub async fn delete_blog_video(session: Session, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use crate::schema::blog_videos::dsl::blog_videos;
            diesel::delete(blog_videos.filter(schema::blog_videos::id.eq(*_id))).execute(&establish_connection()).expect("E");
        }
    }
    HttpResponse::Ok()
}
