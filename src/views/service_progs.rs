use actix_web::{
    web,
    HttpRequest,
    HttpResponse,
    error::InternalError,
    http::StatusCode,
};

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
use crate::models::{
    Categories,
    Item,
    User,
    Cat,
    SmallTag,
    CatDetail,
};
use sailfish::TemplateOnce;


pub fn service_routes(config: &mut web::ServiceConfig) {
    config.route("/service_categories/", web::get().to(service_categories_page));
    config.service(web::resource("/service/{cat_slug}/{service_slug}/").route(web::get().to(get_service_page)));
    config.service(web::resource("/services/{slug}/").route(web::get().to(service_category_page)));
}


pub async fn get_service_page(session: Session, req: HttpRequest, param: web::Path<(String,String)>) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;
    use schema::items::dsl::items;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let _connection = establish_connection();
    let _item_id: String = param.1.clone();
    let _cat_id: String = param.0.clone();

    let _items = items
        .filter(schema::items::slug.eq(&_item_id))
        .load::<Item>(&_connection)
        .expect("E");
    let _item = _items.into_iter().nth(0).unwrap();
    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            _item.title.clone() + &" | Услуга".to_string(),
            _item.title.clone() + &" | Услуга: вебсервисы.рф".to_string(),
            "/service/".to_string() + &_cat_id.to_string() + &"/".to_string() + &_item_id.to_string() + &"/".to_string(),
            _item.get_image(),
        ).await
    }
    else {
        use schema::{
            categories::dsl::categories,
            tech_categories::dsl::tech_categories,
        };
        use crate::models::{TechCategories, FeaturedItem};

        let _tech_categories = tech_categories
            .load::<TechCategories>(&_connection)
            .expect("E");

        let _categorys = categories
            .filter(schema::categories::slug.eq(&_cat_id))
            .filter(schema::categories::types.eq(_item.types))
            .load::<Categories>(&_connection)
            .expect("E");
        let _category = _categorys.into_iter().nth(0).unwrap();
        let _cats = Categories::get_categories_for_types(2);

        let _tags = _item.get_tags();

        let (prev, next) = _category.get_featured_items(_item.types, _item.id);

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            if _item.is_active == false && _request_user.perm < 10 {
                use crate::utils::get_private_page;
                get_private_page (
                    is_ajax,
                    _request_user,
                    is_desctop,
                    _item.title.clone() + &" | Услуга".to_string(),
                    _item.title.clone() + &" | Услуга: вебсервисы.рф".to_string(),
                    "/service/".to_string() + &_cat_id.to_string() + &"/".to_string() + &_item_id.to_string() + &"/".to_string(),
                    _item.get_image(),
                ).await
            }
            else if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/services/service.stpl")]
                struct Template {
                    request_user: User,
                    object:   Item,
                    category: Categories,
                    cats:     Vec<Cat>,
                    all_tags: Vec<SmallTag>,
                    prev:     Option<FeaturedItem>,
                    next:     Option<FeaturedItem>,
                    is_ajax:  i32,
                }
                let body = Template {
                    request_user: _request_user,
                    object:   _item,
                    category: _category,
                    cats:     _cats,
                    all_tags: _tags,
                    prev:     prev,
                    next:     next,
                    is_ajax:  is_ajax,
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
                    object:   Item,
                    category: Categories,
                    cats:     Vec<Cat>,
                    all_tags: Vec<SmallTag>,
                    prev:     Option<FeaturedItem>,
                    next:     Option<FeaturedItem>,
                    is_ajax:  i32,
                }
                let body = Template {
                    request_user: _request_user,
                    object:   _item,
                    category: _category,
                    cats:     _cats,
                    all_tags: _tags,
                    prev:     prev,
                    next:     next,
                    is_ajax:  is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            if _item.is_active == false {
                use crate::utils::get_anon_private_page;
                get_anon_private_page (
                    is_ajax,
                    is_desctop,
                    _item.title.clone() + &" | Услуга".to_string(),
                    _item.title.clone() + &" | Услуга: вебсервисы.рф".to_string(),
                    "/service/".to_string() + &_cat_id.to_string() + &"/".to_string() + &_item_id.to_string() + &"/".to_string(),
                    _item.get_image(),
                ).await
            }
            else if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/services/anon_service.stpl")]
                struct Template {
                    object:   Item,
                    category: Categories,
                    cats:     Vec<Cat>,
                    all_tags: Vec<SmallTag>,
                    prev:     Option<FeaturedItem>,
                    next:     Option<FeaturedItem>,
                    is_ajax:  i32,
                }
                let body = Template {
                    object:   _item,
                    category: _category,
                    cats:     _cats,
                    all_tags: _tags,
                    prev:     prev,
                    next:     next,
                    is_ajax:  is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/services/anon_service.stpl")]
                struct Template {
                    object:   Item,
                    category: Categories,
                    cats:     Vec<Cat>,
                    all_tags: Vec<SmallTag>,
                    prev:     Option<FeaturedItem>,
                    next:     Option<FeaturedItem>,
                    is_ajax:  i32,
                }
                let body = Template {
                    object:   _item,
                    category: _category,
                    cats:     _cats,
                    all_tags: _tags,
                    prev:     prev,
                    next:     next,
                    is_ajax:  is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn service_category_page(session: Session, req: HttpRequest, _id: web::Path<String>) -> actix_web::Result<HttpResponse> {
    use crate::schema::categories::dsl::categories;
    use crate::utils::get_device_and_ajax;

    let _cat_id: String = _id.clone();
    let _connection = establish_connection();

    let _categorys = categories
        .filter(schema::categories::slug.eq(&_cat_id))
        .filter(schema::categories::types.eq(2))
        .limit(1)
        .select((
            schema::categories::name,
            schema::categories::slug,
            schema::categories::count,
            schema::categories::id,
            schema::categories::image,
            schema::categories::view,
            schema::categories::height,
            schema::categories::seconds
        ))
        .load::<CatDetail>(&_connection)
        .expect("E");

    let _category = _categorys.into_iter().nth(0).unwrap();
    let cat_image: String;
    if _category.image.is_some() {
        cat_image = _category.image.as_deref().unwrap().to_string();
    }
    else {
        cat_image = "/static/images/dark/store.jpg".to_string();
    }

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            _category.name.clone() + &" | Категория услуг ".to_string(),
            _category.name.clone() + &" | Категория услуг - вебсервисы.рф".to_string(),
            "/services/".to_string() + &_category.slug.clone() + &"/".to_string(),
            cat_image,
        ).await
    }
    else {
        use crate::utils::get_page;
        use crate::schema::tags_items::dsl::tags_items;
        use crate::models::Service;

        let page = get_page(&req);
        let _cats = Categories::get_categories_for_types(2);

        let mut stack = Vec::new();
        let _tag_items = tags_items
            .filter(schema::tags_items::types.eq(2))
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
            .select((schema::tags::name, schema::tags::count))
            .load::<SmallTag>(&_connection)
            .expect("E");

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            let (object_list, next_page_number) = Categories::get_services_list(_category.id, page, 20, _request_user.is_superuser());
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/services/category.stpl")]
                struct Template {
                    request_user:     User,
                    //all_tags:         Vec<SmallTag>,
                    category:         CatDetail,
                    //cats:             Vec<Cat>,
                    object_list:      Vec<Service>,
                    next_page_number: i32,
                    is_ajax:          i32,
                }
                let body = Template {
                    request_user:     _request_user,
                    //all_tags:         _tags,
                    category:         _category,
                    //cats:             _cats,
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
                    all_tags:         Vec<SmallTag>,
                    category:         CatDetail,
                    cats:             Vec<Cat>,
                    object_list:      Vec<Service>,
                    next_page_number: i32,
                    is_ajax:          i32,
                }
                let body = Template {
                    all_tags:         _tags,
                    category:         _category,
                    cats:             _cats,
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
            let (object_list, next_page_number) = Categories::get_services_list(_category.id, page, 20, false);

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/services/anon_category.stpl")]
                struct Template {
                    //all_tags:         Vec<SmallTag>,
                    category:         CatDetail,
                    //cats:             Vec<Cat>,
                    object_list:      Vec<Service>,
                    next_page_number: i32,
                    is_ajax:          i32,
                }
                let body = Template {
                    //all_tags:         _tags,
                    category:         _category,
                    //cats:             _cats,
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
                    all_tags:         Vec<SmallTag>,
                    category:         CatDetail,
                    cats:             Vec<Cat>,
                    object_list:      Vec<Service>,
                    next_page_number: i32,
                    is_ajax:          i32,
                }
                let body = Template {
                    all_tags:         _tags,
                    category:         _category,
                    cats:             _cats,
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

pub async fn service_categories_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Категории услуг".to_string(),
            "вебсервисы.рф: Категории услуг".to_string(),
            "/service_categories/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
        ).await
    }
    else {
        use crate::schema::{
            tags_items::dsl::tags_items,
            tags::dsl::tags,
            stat_pages::dsl::stat_pages,
        };
        use crate::models::StatPage;

        let _connection = establish_connection();
        let _stat: StatPage;
        let _stats = stat_pages
            .filter(schema::stat_pages::types.eq(61))
            .limit(1)
            .load::<StatPage>(&_connection)
            .expect("E");
        if _stats.len() > 0 {
            _stat = _stats.into_iter().nth(0).unwrap();
        }
        else {
            use crate::models::NewStatPage;
            let form = NewStatPage {
                types:   61,
                view:    0,
                height:  0.0,
                seconds: 0,
            };
            _stat = diesel::insert_into(schema::stat_pages::table)
                .values(&form)
                .get_result::<StatPage>(&_connection)
                .expect("Error.");
        }

        let mut stack = Vec::new();
        let _tag_items = tags_items
            .filter(schema::tags_items::types.eq(2))
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
            .select((schema::tags::name, schema::tags::count))
            .load::<SmallTag>(&_connection)
            .expect("could not load tags");

        let _cats = Categories::get_categories_for_types(2);

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/services/categories.stpl")]
                struct Template {
                    request_user: User,
                    is_ajax:      i32,
                    cats:         Vec<Cat>,
                    //all_tags:     Vec<SmallTag>,
                    stat:         StatPage,
                }
                let body = Template {
                    request_user: _request_user,
                    is_ajax:      is_ajax,
                    cats:         _cats,
                    //all_tags:     _tags,
                    stat:         _stat,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/services/categories.stpl")]
                struct Template {
                    //request_user: User,
                    is_ajax:      i32,
                    cats:         Vec<Cat>,
                    all_tags:     Vec<SmallTag>,
                    stat:         StatPage,
                }
                let body = Template {
                    //request_user: _request_user,
                    is_ajax:      is_ajax,
                    cats:         _cats,
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
                #[template(path = "desctop/services/anon_categories.stpl")]
                struct Template {
                    is_ajax:  i32,
                    cats:     Vec<Cat>,
                    //all_tags: Vec<SmallTag>,
                    stat:     StatPage,
                }
                let body = Template {
                    is_ajax:  is_ajax,
                    cats:     _cats,
                    //all_tags: _tags,
                    stat:     _stat,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/services/anon_categories.stpl")]
                struct Template {
                    is_ajax:  i32,
                    cats:     Vec<Cat>,
                    all_tags: Vec<SmallTag>,
                    stat:     StatPage,
                }
                let body = Template {
                    is_ajax:  is_ajax,
                    cats:     _cats,
                    all_tags: _tags,
                    stat:     _stat,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}
