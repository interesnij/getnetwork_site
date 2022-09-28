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
    User,
    Cat,
    SmallTag,
    CatDetail,
};
use sailfish::TemplateOnce;


pub fn help_routes(config: &mut web::ServiceConfig) {
    config.service(web::resource("/helps/{slug}/").route(web::get().to(help_category_page)));
}


pub async fn help_category_page(session: Session, req: HttpRequest, _id: web::Path<String>) -> actix_web::Result<HttpResponse> {
    use crate::schema::categories::dsl::categories;
    use crate::utils::get_device_and_ajax;

    let _cat_id: String = _id.clone();
    let _connection = establish_connection();

    let _categorys = categories
        .filter(schema::categories::slug.eq(&_cat_id))
        .filter(schema::categories::types.eq(6))
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
            _category.name.clone() + &" | Категория помощи ".to_string(),
            _category.name.clone() + &" | Категория помощи - вебсервисы.рф".to_string(),
            "/help/".to_string() + &_category.slug.clone() + &"/".to_string(),
            cat_image,
        ).await
    }
    else {
        use crate::utils::get_page;
        use crate::schema::tags_items::dsl::tags_items;
        use crate::models::Help;

        let page = get_page(&req);
        let _cats = Categories::get_categories_for_types(6);

        let mut stack = Vec::new();
        let _tag_items = tags_items
            .filter(schema::tags_items::types.eq(6))
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
            let (object_list, next_page_number) = Categories::get_helps_list(_category.id, page, 20, _request_user.is_superuser());
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/help/category.stpl")]
                struct Template {
                    request_user:     User,
                    all_tags:         Vec<SmallTag>,
                    category:         CatDetail,
                    cats:             Vec<Cat>,
                    object_list:      Vec<Help>,
                    next_page_number: i32,
                    is_ajax:          i32,
                }
                let body = Template {
                    request_user:     _request_user,
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
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/help/category.stpl")]
                struct Template {
                    request_user:     User,
                    all_tags:         Vec<SmallTag>,
                    category:         CatDetail,
                    cats:             Vec<Cat>,
                    object_list:      Vec<Help>,
                    next_page_number: i32,
                    is_ajax:          i32,
                }
                let body = Template {
                    request_user:     _request_user,
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
            let (object_list, next_page_number) = Categories::get_helps_list(_category.id, page, 20, false);

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/help/anon_category.stpl")]
                struct Template {
                    all_tags:         Vec<SmallTag>,
                    category:         CatDetail,
                    cats:             Vec<Cat>,
                    object_list:      Vec<Help>,
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
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/help/anon_category.stpl")]
                struct Template {
                    all_tags:         Vec<SmallTag>,
                    category:         CatDetail,
                    cats:             Vec<Cat>,
                    object_list:      Vec<Help>,
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
