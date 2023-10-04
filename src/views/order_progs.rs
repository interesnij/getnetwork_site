use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    web::block,
    error::InternalError,
    http::StatusCode,
    Responder,
};
use std::borrow::BorrowMut;
use crate::diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use crate::utils::{
    establish_connection,
    is_signed_in,
    get_request_user_data,
    get_first_load_page,
    get_or_create_cookie_user_id,
    get_cookie_user_id,
    get_template,
};
use crate::schema;
use crate::models::{
    Order,
    NewOrder,
    OrderFile,
    NewOrderFile,
};
use actix_session::Session;
use actix_multipart::Multipart;
use sailfish::TemplateOnce;
use crate::models::User;
use actix_web::dev::ConnectionInfo;


pub fn order_routes(config: &mut web::ServiceConfig) {
    config.route("/orders/", web::get().to(get_orders_page));
    config.route("/user_orders/", web::get().to(get_user_orders_page));
    config.route("/order/{id}/", web::get().to(get_order_page));
    config.service(web::resource("/create_order/")
        .route(web::get().to(create_order_page))
        .route(web::post().to(create_order))
    );
    //config.service(web::resource("/edit_order/{id}/")
    //    .route(web::get().to(edit_order_page))
    //    .route(web::post().to(edit_order))
    //);
    config.route("/delete_order/{id}/", web::get().to(delete_order));
}

pub async fn get_orders_page(req: HttpRequest, session: Session) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let template_types = get_template(&req);
    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Заказы".to_string(),
            "вебсервисы.рф: Заказы".to_string(),
            "/orders/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
            template_types,
        ).await
    }
    else if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied"))
    }
    else {
        use crate::utils::get_page;

        let _connection = establish_connection();
        let (_orders, next_page_number) = Order::get_orders_list(get_page(&req), 20);

        let _request_user = get_request_user_data(&session);
        if _request_user.perm < 60 {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied"))
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/pages/orders_list.stpl")]
            struct Template {
                request_user:     User,
                is_ajax:          i32,
                object_list:      Vec<Order>,
                next_page_number: i32,
                template_types:   i16,
            }
            let body = Template {
                request_user:     _request_user,
                is_ajax:          is_ajax,
                object_list:      _orders,
                next_page_number: next_page_number,
                template_types:   template_types,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/pages/orders_list.stpl")]
            struct Template {
                is_ajax:          i32,
                object_list:      Vec<Order>,
                next_page_number: i32,
                template_types:   i16,
            }
            let body = Template {
                is_ajax:          is_ajax,
                object_list:      _orders,
                next_page_number: next_page_number,
                template_types:   template_types,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn get_user_orders_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::utils::{get_device_and_ajax, get_page};

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let template_types = get_template(&req);
    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Ваши заказы".to_string(),
            "вебсервисы.рф: Ваши заказы".to_string(),
            "/user_orders/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
            template_types,
        ).await
    }
    else {
        let user_id = get_cookie_user_id(&req).await;
        let (_orders, next_page_number) = Order::get_user_orders_list(user_id, get_page(&req), 20);
        if user_id == 0 {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Информация о заказчике не найдена"))
        }
        else if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/pages/user_orders.stpl")]
                struct Template {
                    request_user:     User,
                    object_list:      Vec<Order>,
                    is_ajax:          i32,
                    next_page_number: i32,
                    template_types:   i16,
                }
                let body = Template {
                    request_user:     _request_user,
                    object_list:      _orders,
                    is_ajax:          is_ajax,
                    next_page_number: next_page_number,
                    template_types:   template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/pages/user_orders.stpl")]
                struct Template {
                    object_list:      Vec<Order>,
                    is_ajax:          i32,
                    next_page_number: i32,
                    template_types:   i16,
                }
                let body = Template {
                    object_list:      _orders,
                    is_ajax:          is_ajax,
                    next_page_number: next_page_number,
                    template_types:   template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/pages/anon_user_orders.stpl")]
                struct Template {
                    object_list:      Vec<Order>,
                    is_ajax:          i32,
                    next_page_number: i32,
                    template_types:   i16,
                }
                let body = Template {
                    object_list:      _orders,
                    is_ajax:          is_ajax,
                    next_page_number: next_page_number,
                    template_types:   template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/pages/anon_user_orders.stpl")]
                struct Template {
                    object_list:      Vec<Order>,
                    is_ajax:          i32,
                    next_page_number: i32,
                    template_types:   i16,
                }
                let body = Template {
                    object_list:      _orders,
                    is_ajax:          is_ajax,
                    next_page_number: next_page_number,
                    template_types:   template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}


pub async fn get_order_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;
    use schema::orders::dsl::orders;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let template_types = get_template(&req);
    let _connection = establish_connection();
    let user_id = get_cookie_user_id(&req).await;

    let _order = orders
        .filter(schema::orders::id.eq(*_id))
        .first::<Order>(&_connection)
        .expect("E");
    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Заказ ".to_string() + &_order.title,
            "вебсервисы.рф: Заказ ".to_string() + &_order.title,
            "/order/".to_string() + &_order.id.to_string() + &"/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
            template_types,
        ).await
    }
    else if user_id != _order.user_id {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Информация о заказчике не найдена"))
    }
    else {
        use schema::order_files::dsl::order_files;

        let _files = order_files
            .filter(schema::order_files::order_id.eq(*_id))
            .load::<OrderFile>(&_connection)
            .expect("E");

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/pages/order.stpl")]
                struct Template {
                    request_user:   User,
                    object:         Order,
                    files:          Vec<OrderFile>,
                    is_ajax:        i32,
                    template_types: i16,
                }
                let body = Template {
                    request_user:   _request_user,
                    object:         _order,
                    files:          _files,
                    is_ajax:        is_ajax,
                    template_types: template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/pages/order.stpl")]
                struct Template {
                    object:         Order,
                    files:          Vec<OrderFile>,
                    is_ajax:        i32,
                    template_types: i16,
                }
                let body = Template {
                    object:         _order,
                    files:          _files,
                    is_ajax:        is_ajax,
                    template_types: template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/pages/anon_order.stpl")]
                struct Template {
                    object:         Order,
                    files:          Vec<OrderFile>,
                    is_ajax:        i32,
                    template_types: i16,
                }
                let body = Template {
                    object:         _order,
                    files:          _files,
                    is_ajax:        is_ajax,
                    template_types: template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/pages/anon_order.stpl")]
                struct Template {
                    object:         Order,
                    files:          Vec<OrderFile>,
                    is_ajax:        i32,
                    template_types: i16,
                }
                let body = Template {
                    object:         _order,
                    files:          _files,
                    is_ajax:        is_ajax,
                    template_types: template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn create_order_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let template_types = get_template(&req);
    #[derive(TemplateOnce)]
    #[template(path = "desctop/pages/create_order.stpl")]
    struct Template {
        template_types: i16,
    }
    let body = Template {
        template_types: template_types,
    }
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
}

pub async fn create_order(conn: ConnectionInfo, req: HttpRequest, mut payload: Multipart) -> impl Responder {
    use crate::schema::serve::dsl::serve;
    use crate::models::{
        NewTechCategoriesItem,
        Serve,
        NewServeItems,
    };
    use crate::utils::{
        order_form,
        get_price_acc_values,
    };

    let _connection = establish_connection();
    let user_id = get_or_create_cookie_user_id(conn, &req).await;

    if user_id != 0 {
        let form = order_form(payload.borrow_mut(), user_id).await;
        let new_order = NewOrder::create (
            form.title.clone(),
            form.title_en.clone(),
            form.types,
            form.object_id,
            form.username.clone(),
            form.email.clone(),
            form.description.clone(),
            form.description_en,
            user_id,
        );

        let _order = diesel::insert_into(schema::orders::table)
            .values(&new_order)
            .get_result::<Order>(&_connection)
            .expect("E.");

        for file in form.files.iter() {
            let new_file = NewOrderFile::create (
                _order.id,
                file.to_string()
            );
            diesel::insert_into(schema::order_files::table)
                .values(&new_file)
                .execute(&_connection)
                .expect("E.");
        };

        // создаем опции услуги и записываем id опций в вектор.
        let mut serve_ids = Vec::new();
        for serve_id in form.serve_list.into_iter() {
            let new_serve_form = NewServeItems {
                serve_id: serve_id,
                item_id:  form.object_id,
                types:    form.types,
            };
            diesel::insert_into(schema::serve_items::table)
                .values(&new_serve_form)
                .execute(&_connection)
                .expect("Error.");
            serve_ids.push(serve_id);
        }

        // получаем опции, чтобы создать связи с их тех. категорией.
        // это надо отрисовки тех категорий услуги, которые активны
        let _serves = serve
            .filter(schema::serve::id.eq_any(serve_ids))
            .load::<Serve>(&_connection)
            .expect("E");

        let mut tech_cat_ids = Vec::new();
        let mut order_price = 0;
        for _serve in _serves.iter() {
            if !tech_cat_ids.into_iter().any(|i| i==_serve.tech_cat_id) {
                tech_cat_ids.push(_serve.tech_cat_id);
            }
            order_price += _serve.price;
        }

        for id in tech_cat_ids.iter() {
            let new_cat = NewTechCategoriesItem {
                category_id: *id,
                item_id:     form.object_id,
                types:       form.types,
                is_active:   1,
            };
            diesel::insert_into(schema::tech_categories_items::table)
                .values(&new_cat)
                .execute(&_connection)
                .expect("Error.");
        }

        // фух. Связи созданы все, но надо еще посчитать цену
        // услуги для калькулятора. Как? А это будет сумма всех
        // цен выбранных опций.
        let price_acc = get_price_acc_values(&order_price);
        diesel::update(&_order)
            .set((
                schema::orders::price.eq(order_price),
                schema::orders::price_acc.eq(price_acc),
            ))
            .execute(&_connection)
            .expect("Error.");
    }
    HttpResponse::Ok()
}

pub async fn delete_order(req: HttpRequest, _id: web::Path<i32>) -> impl Responder {
    let _connection = establish_connection();
    let _order = schema::orders::table
        .filter(schema::orders::id.eq(*_id))
        .first::<Order>(&_connection)
        .expect("E");

    let user_id = get_cookie_user_id(&req).await;

    if user_id == _order.user_id {
        use crate::schema::{
            serve_items::dsl::serve_items,
            tech_categories_items::dsl::tech_categories_items,
        };

        diesel::delete (
            serve_items
                .filter(schema::serve_items::item_id.eq(*_id))
                .filter(schema::serve_items::types.eq(7))
            )
            .execute(&_connection)
            .expect("E");
        diesel::delete(
            tech_categories_items
                .filter(schema::tech_categories_items::item_id.eq(*_id))
                .filter(schema::tech_categories_items::types.eq(7))
            )
            .execute(&_connection)
            .expect("E");
        diesel::delete(&_order).execute(&_connection).expect("E");
    }
    HttpResponse::Ok()
}
