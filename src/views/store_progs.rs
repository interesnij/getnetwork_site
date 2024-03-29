use actix_web::{
    web,
    web::block,
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
    get_all_storage,
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
    Item, StatPage,
    User,
    Cat,
    SmallTag,
    CatDetail,
};
use sailfish::TemplateOnce;


pub fn store_routes(config: &mut web::ServiceConfig) {
    config.route("/store_categories/", web::get().to(store_categories_page));
    config.service(web::resource("/store/{cat_slug}/{store_slug}/").route(web::get().to(get_store_page)));
    config.service(web::resource("/stores/{slug}/").route(web::get().to(store_category_page)));
}


pub async fn get_store_page(session: Session, req: HttpRequest, param: web::Path<(String,String)>) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;
    use schema::items::dsl::items;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let _connection = establish_connection();
    let _item_id: String = param.1.clone();
    let _cat_id: String = param.0.clone();
    let (t, l) = get_all_storage();

    let _item = items
        .filter(schema::items::slug.eq(&_item_id))
        .first::<Item>(&_connection)
        .expect("E");
    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            _item.title.clone() + &" | Товар".to_string(),
            _item.title.clone() + &" | Товар: вебсервисы.рф".to_string(),
            "/store/".to_string() + &_cat_id.to_string() + &"/".to_string() + &_item_id.to_string() + &"/".to_string(),
            _item.get_image(),
            t, 
            l,
        ).await
    }
    else {
        use schema::{
            categories::dsl::categories,
        };
        use crate::models::FeaturedItem;

        let _category = categories
            .filter(schema::categories::slug.eq(&_cat_id))
            .filter(schema::categories::types.eq(_item.types))
            .first::<Categories>(&_connection)
            .expect("E");
        let _cats = block(move || Categories::get_categories_for_types(3, l)).await?;
        let _tags = block(move || Categories::get_tags(3, l)).await?;

        let (prev, next) = _category.get_featured_items(_item.id, _item.types, l);

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            if !_item.is_active && _request_user.perm < 10 {
                crate::utils::get_private_page (
                    is_ajax,
                    _request_user,
                    is_desctop,
                    _item.title.clone() + &" | Товар".to_string(),
                    _item.title.clone() + &" | Товар: вебсервисы.рф".to_string(),
                    "/store/".to_string() + &_cat_id.to_string() + &"/".to_string() + &_item_id.to_string() + &"/".to_string(),
                    _item.get_image(),
                    t, 
                    l,
                ).await
            }
            else if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/stores/store.stpl")]
                struct Template {
                    request_user:   User,
                    object:         Item,
                    category:       Categories,
                    prev:           Option<FeaturedItem>,
                    next:           Option<FeaturedItem>,
                    is_ajax:        i32,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    request_user:   _request_user,
                    object:         _item,
                    category:       _category,
                    prev:           prev,
                    next:           next,
                    is_ajax:        is_ajax,
                    template_types: t,
                    linguage:       l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/stores/store.stpl")]
                struct Template {
                    request_user:   User,
                    object:         Item,
                    category:       Categories,
                    cats:           Vec<Cat>,
                    all_tags:       Vec<SmallTag>,
                    prev:           Option<FeaturedItem>,
                    next:           Option<FeaturedItem>,
                    is_ajax:        i32,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    request_user:   _request_user,
                    object:         _item,
                    category:       _category,
                    cats:           _cats,
                    all_tags:       _tags,
                    prev:           prev,
                    next:           next,
                    is_ajax:        is_ajax,
                    template_types: t,
                    linguage:       l,
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
                    _item.title.clone() + &" | Товар".to_string(),
                    _item.title.clone() + &" | Товар: вебсервисы.рф".to_string(),
                    "/store/".to_string() + &_cat_id.to_string() + &"/".to_string() + &_item_id.to_string() + &"/".to_string(),
                    _item.get_image(),
                    t, 
                    l,
                ).await
            }
            else if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/stores/anon_store.stpl")]
                struct Template {
                    object:         Item,
                    category:       Categories,
                    prev:           Option<FeaturedItem>,
                    next:           Option<FeaturedItem>,
                    is_ajax:        i32,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    object:         _item,
                    category:       _category,
                    prev:           prev,
                    next:           next,
                    is_ajax:        is_ajax,
                    template_types: t,
                    linguage:       l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/stores/anon_store.stpl")]
                struct Template {
                    object:         Item,
                    category:       Categories,
                    cats:           Vec<Cat>,
                    all_tags:       Vec<SmallTag>,
                    prev:           Option<FeaturedItem>,
                    next:           Option<FeaturedItem>,
                    is_ajax:        i32,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    object:         _item,
                    category:       _category,
                    cats:           _cats,
                    all_tags:       _tags,
                    prev:           prev,
                    next:           next,
                    is_ajax:        is_ajax,
                    template_types: t,
                    linguage:       l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn store_category_page(session: Session, req: HttpRequest, _id: web::Path<String>) -> actix_web::Result<HttpResponse> {
    use crate::schema::categories::dsl::categories;
    use crate::utils::get_device_and_ajax;

    let _cat_id: String = _id.clone();
    let _connection = establish_connection();
    let (t, l) = get_all_storage();

    let _category = categories
        .filter(schema::categories::slug.eq(&_cat_id))
        .filter(schema::categories::types.eq(3))
        .select((
            schema::categories::name,
            schema::categories::slug,
            schema::categories::count,
            schema::categories::id,
            schema::categories::image,
            schema::categories::view,
            schema::categories::height,
            schema::categories::seconds,
        ))
        .first::<CatDetail>(&_connection)
        .expect("E");

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
            _category.name.clone() + &" | Категория товаров ".to_string(),
            _category.name.clone() + &" | Категория товаров - вебсервисы.рф".to_string(),
            "/stores/".to_string() + &_category.slug.clone() + &"/".to_string(),
            cat_image,
            t, 
            l,
        ).await
    }
    else {
        use crate::utils::get_page;
        use crate::models::Store;

        let page = get_page(&req);
        let object_list: Vec<Store>;
        let next_page_number: i32;
        let _cats = block(move || Categories::get_categories_for_types(3, l)).await?;
        let _tags = block(move || Categories::get_tags(3, l)).await?;

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            let _res = block(move || Categories::get_stores_list(_category.id, page, 20, _request_user.perm == 60, l)).await?;
            let _dict = match _res {
                Ok(_ok) => {object_list = _ok.0; next_page_number = _ok.1},
                Err(_error) => {object_list = Vec::new(); next_page_number = 0},
            };
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/stores/category.stpl")]
                struct Template {
                    request_user:     User,
                    all_tags:         Vec<SmallTag>,
                    category:         CatDetail,
                    cats:             Vec<Cat>,
                    object_list:      Vec<Store>,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   u8,
                    linguage:         u8,
                }
                let body = Template {
                    request_user:     _request_user,
                    all_tags:         _tags,
                    category:         _category,
                    cats:             _cats,
                    object_list:      object_list,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
                    template_types:   t,
                    linguage:         l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/stores/category.stpl")]
                struct Template {
                    all_tags:         Vec<SmallTag>,
                    category:         CatDetail,
                    cats:             Vec<Cat>,
                    object_list:      Vec<Store>,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   u8,
                    linguage:         u8,
                }

                let body = Template {
                    all_tags:         _tags,
                    category:         _category,
                    cats:             _cats,
                    object_list:      object_list,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
                    template_types:   t,
                    linguage:         l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            let _res = block(move || Categories::get_stores_list(_category.id, page, 20, false, l)).await?;
            let _dict = match _res {
                Ok(_ok) => {object_list = _ok.0; next_page_number = _ok.1},
                Err(_error) => {object_list = Vec::new(); next_page_number = 0},
            };

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/stores/anon_category.stpl")]
                struct Template {
                    all_tags:         Vec<SmallTag>,
                    category:         CatDetail,
                    cats:             Vec<Cat>,
                    object_list:      Vec<Store>,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   u8,
                    linguage:         u8,
                }
                let body = Template {
                    all_tags:         _tags,
                    category:         _category,
                    cats:             _cats,
                    object_list:      object_list,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
                    template_types:   t,
                    linguage:         l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/stores/anon_category.stpl")]
                struct Template {
                    all_tags:         Vec<SmallTag>,
                    category:         CatDetail,
                    cats:             Vec<Cat>,
                    object_list:      Vec<Store>,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   u8,
                    linguage:         u8,
                }
                let body = Template {
                    all_tags:         _tags,
                    category:         _category,
                    cats:             _cats,
                    object_list:      object_list,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
                    template_types:   t,
                    linguage:         l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn store_categories_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;

    let (t, l) = get_all_storage();
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Категории товаров".to_string(),
            "вебсервисы.рф: Категории товаров".to_string(),
            "/store_categories/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
            t, 
            l,
        ).await
    }
    else {
        let _stat = crate::models::StatPage::get_or_create(71);
        let _cats = block(move || Categories::get_categories_for_types(3, l)).await?;
        let _tags = block(move || Categories::get_tags(3, l)).await?;

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/stores/categories.stpl")]
                struct Template {
                    request_user:   User,
                    is_ajax:        i32,
                    cats:           Vec<Cat>,
                    all_tags:       Vec<SmallTag>,
                    stat:           StatPage,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    request_user:   _request_user,
                    is_ajax:        is_ajax,
                    cats:           _cats,
                    all_tags:       _tags,
                    stat:           _stat,
                    template_types: t,
                    linguage:       l,
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
                    is_ajax:        i32,
                    cats:           Vec<Cat>,
                    all_tags:       Vec<SmallTag>,
                    stat:           StatPage,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    request_user:   _request_user,
                    is_ajax:        is_ajax,
                    cats:           _cats,
                    all_tags:       _tags,
                    stat:           _stat,
                    template_types: t,
                    linguage:       l,
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
                    is_ajax:        i32,
                    cats:           Vec<Cat>,
                    all_tags:       Vec<SmallTag>,
                    stat:           StatPage,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    is_ajax:        is_ajax,
                    cats:           _cats,
                    all_tags:       _tags,
                    stat:           _stat,
                    template_types: t,
                    linguage:       l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/stores/anon_categories.stpl")]
                struct Template {
                    is_ajax:        i32,
                    cats:           Vec<Cat>,
                    all_tags:       Vec<SmallTag>,
                    stat:           StatPage,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    is_ajax:        is_ajax,
                    cats:           _cats,
                    all_tags:       _tags,
                    stat:           _stat,
                    template_types: t,
                    linguage:       l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}
