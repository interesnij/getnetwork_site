use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
    Responder,
};
use crate::models::User;
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
};
use crate::schema;
use crate::models::{
    Order,
    NewOrder,
    OrderFile,
    NewOrderFile,
};
use actix_session::Session;
use actix_multipart::{Field, Multipart};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::str;
use sailfish::TemplateOnce;


pub fn serve_routes(config: &mut web::ServiceConfig) {
    config.route("/orders/", web::get().to(get_orders_page));
    config.route("/user_orders/{id}/", web::get().to(get_user_orders_page));
    config.route("/order/{id}/", web::get().to(get_order_page));
    config.service(web::resource("/create_order/")
        .route(web::get().to(create_order_page))
        .route(web::post().to(create_order))
    );
    config.service(web::resource("/edit_order/{id}/")
        .route(web::get().to(edit_order_page))
        .route(web::post().to(edit_order))
    );
    config.route("/delete_order/{id}/", web::get().to(delete_order));
}

pub async fn get_orders_page(req: HttpRequest, session: Session) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    if is_ajax == 0 {
        get_first_load_page(&session, is_desctop, "Заказы".to_string()).await
    }
    else if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied"))
    }
    else {
        use crate::models::Order;
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
                title:            String,
                request_user:     User,
                is_ajax:          i32,
                order_list:       Vec<Order>,
                next_page_number: i32,
            }
            let body = Template {
                title:            "Заказы".to_string(),
                request_user:     _request_user,
                is_ajax:          is_ajax,
                order_list:       _orders,
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/pages/orders_list.stpl")]
            struct Template {
                title:            String,
                is_ajax:          i32,
                order_list:       Vec<Order>,
                next_page_number: i32,
            }
            let body = Template {
                title:            "Заказы".to_string(),
                is_ajax:          is_ajax,
                order_list:       _orders,
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn get_user_orders_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::utils::{get_device_and_ajax, get_page};

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    if is_ajax == 0 {
        get_first_load_page(&session, is_desctop, "Ваши заказы".to_string()).await
    }
    else {
        let _user_id: i32 = _id;
        let (_orders, next_page_number) = Order::get_user_orders_list(_user_id, get_page(&req), 20);

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/pages/user_orders.stpl")]
                struct Template {
                    title:            String,
                    request_user:     User,
                    object:           Order,
                    files:            Vec<OrderFile>,
                    is_ajax:          i32,
                    next_page_number: i32,
                }
                let body = Template {
                    title:            "Ваши заказы".to_string(),
                    request_user:     _request_user,
                    object:           _order,
                    files:            _files,
                    is_ajax:          is_ajax,
                    next_page_number: next_page_number,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/pages/user_orders.stpl")]
                struct Template {
                    title:            String,
                    request_user:     User,
                    object:           Order,
                    files:            Vec<OrderFile>,
                    is_ajax:          i32,
                    next_page_number: i32,
                }
                let body = Template {
                    title:            "Ваши заказы".to_string(),
                    request_user:     _request_user,
                    object:           _order,
                    files:            _files,
                    is_ajax:          is_ajax,
                    next_page_number: next_page_number,
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
                    title:            String,
                    object:           Order,
                    files:            Vec<OrderFile>,
                    is_ajax:          i32,
                    next_page_number: i32,
                }
                let body = Template {
                    title:            "Ваши заказы".to_string(),
                    object:           _order,
                    files:            _files,
                    is_ajax:          is_ajax,
                    next_page_number: next_page_number,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/pages/anon_user_orders.stpl")]
                struct Template {
                    title:            String,
                    object:           Order,
                    files:            Vec<OrderFile>,
                    is_ajax:          i32,
                    next_page_number: i32,
                }
                let body = Template {
                    title:            "Ваши заказы".to_string(),
                    object:           _order,
                    files:            _files,
                    is_ajax:          is_ajax,
                    next_page_number: next_page_number,
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
    let _connection = establish_connection();
    let _order_id: i32 = _id;

    let _orders = orders
        .filter(schema::orders::id.eq(&_order_id))
        .load::<Order>(&_connection)
        .expect("E");
    let _order = _orders.into_iter().nth(0).unwrap();
    if is_ajax == 0 {
        get_first_load_page(&session, is_desctop, "Заказ ".to_string() + &_order.title).await
    }
    else {
        use schema::order_files::dsl::order_files;

        let _files = order_files
            .filter(schema::order_files::order_id.eq(&_order_id))
            .load::<OrderFile>(&_connection)
            .expect("E");

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/pages/order.stpl")]
                struct Template {
                    title:        String,
                    request_user: User,
                    object:       Order,
                    files:        Vec<OrderFile>,
                    is_ajax:      i32,
                }
                let body = Template {
                    title:        "Заказ ".to_string() + &_order.title,
                    request_user: _request_user,
                    object:       _order,
                    files:        _files,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/pages/order.stpl")]
                struct Template {
                    title:        String,
                    request_user: User,
                    object:       Order,
                    files:        Vec<OrderFile>,
                    is_ajax:      i32,
                }
                let body = Template {
                    title:        "Заказ ".to_string() + &_order.title,
                    request_user: _request_user,
                    object:       _order,
                    files:        _files,
                    is_ajax:      is_ajax,
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
                    title:        String,
                    object:       Order,
                    files:        Vec<OrderFile>,
                    is_ajax:      i32,
                }
                let body = Template {
                    title:        "Заказ ".to_string() + &_order.title,
                    object:       _order,
                    files:        _files,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/pages/anon_order.stpl")]
                struct Template {
                    title:        String,
                    object:       Order,
                    files:        Vec<OrderFile>,
                    is_ajax:      i32,
                }
                let body = Template {
                    title:        "Заказ ".to_string() + &_order.title,
                    object:       _order,
                    files:        _files,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn create_order_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    #[derive(Debug, Deserialize)]
    struct OrderParams {
        pub object_id: Option<i32>,
        pub types:     Option<i16>,
    }
    let params_some = web::Query::<OrderParams>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        if params.object_id.is_some() {
            object_id = params.object_id.unwrap();
        }
        else {
            object_id = 0;
        }
        if params.types.is_some() {
            _type = params.types.unwrap();
        }
        else {
            _type = 0;
        }
    }
    if object_id == 0 || _type == 0 {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Важные параметры для заказа отсутствуют..."))
    }
    else {
        let mut user_id = 0;
        for header in req.headers().into_iter() {
            if header.0 == "cookie" {
                let str_cookie = header.1.to_str().unwrap();
                let _cookie: Vec<&str> = str_cookie.split(";").collect();
                for c in _cookie.iter() {
                    let split_c: Vec<&str> = c.split("=").collect();
                    if split_c[0] == "user" {
                        user_id = split_c[1];
                    }
                    println!("name {:?}", split_c[0].trim());
                    println!("value {:?}", split_c[1]);
                }
            }
        };
        if user_id == 0 {
            use crate::views::create_c_user;

            let user = create_c_user(p_id, &req).await;
            user_id = user.id;
        }
        else {
            use crate::views::get_c_user;

            let user = get_c_user(p_id, &req).await;
            user_id = user.id;
        }
        if _type == 1 {
            use schema::services::dsl::services;
            use crate::models::Service;
            let _services = services
                .filter(schema::services::id.eq(object_id))
                .load::<Service>(&_connection)
                .expect("E");
            let _service = _services
                .into_iter()
                .nth(0)
                .unwrap();

            #[derive(TemplateOnce)]
            #[template(path = "desctop/pages/create_order.stpl")]
            struct Template {
                title:  String,
                types:  i16,
                object: Service,
            }
            let body = Template {
                title:  "Создание заказа".to_string(),
                types:  _type,
                object: _service,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }

        else if _type == 2 {
            use schema::stores::dsl::stores;
            use crate::models::Store;
            let _stores = stores
                .filter(schema::stores::id.eq(object_id))
                .load::<Store>(&_connection)
                .expect("E");
            let _store = _stores
                .into_iter()
                .nth(0)
                .unwrap();

            #[derive(TemplateOnce)]
            #[template(path = "desctop/pages/create_order.stpl")]
            struct Template {
                title:  String,
                types:  i16,
                object: Store,
            }
            let body = Template {
                title:  "Создание заказа".to_string(),
                types:  _type,
                object: _store,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }

        else if _type == 3 {
            use schema::works::dsl::works;
            use crate::models::Work;
            let _works = works
                .filter(schema::works::id.eq(object_id))
                .load::<Work>(&_connection)
                .expect("E");
            let _work = _works
                .into_iter()
                .nth(0)
                .unwrap();

            #[derive(TemplateOnce)]
            #[template(path = "desctop/pages/create_order.stpl")]
            struct Template {
                title:  String,
                types:  i16,
                object: Work,
            }
            let body = Template {
                title:  "Создание заказа".to_string(),
                types:  _type,
                object: _work,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Непонятно, к какому типу относится объект..."))
        }
    }
}


pub async fn edit_order_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use schema::orders::dsl::orders;

    let _order_id: i32 = *_id;
    let _connection = establish_connection();
    let _orders = orders
        .filter(schema::orders::id.eq(&_order_id))
        .load::<Order>(&_connection)
        .expect("E");
    let _order = _orders
        .into_iter()
        .nth(0)
        .unwrap();

    let mut user_id = 0;
    for header in req.headers().into_iter() {
        if header.0 == "cookie" {
            let str_cookie = header.1.to_str().unwrap();
            let _cookie: Vec<&str> = str_cookie.split(";").collect();
            for c in _cookie.iter() {
                let split_c: Vec<&str> = c.split("=").collect();
                if split_c[0] == "user" {
                    user_id = split_c[1];
                }
                println!("name {:?}", split_c[0].trim());
                println!("value {:?}", split_c[1]);
            }
        }
    };

    if user_id == _order.user_id {
        use schema::{
            order_files::dsl::order_files,
            tech_categories::dsl::tech_categories,
        };
        use crate::models::TechCategories;

        let _files = order_files
            .filter(schema::order_files::order_id.eq(_order.id))
            .load::<OrderFile>(&_connection)
            .expect("E");

        let _serve = _order.get_serves();
        let tech_id = _serve[0].tech_cat_id;
        let _tech_categories = tech_categories
            .filter(schema::tech_categories::id.eq(tech_id))
            .load::<TechCategories>(&_connection)
            .expect("E");

        let level = _tech_categories[0].level;
        let _tech_categories = tech_categories
            .filter(schema::tech_categories::level.eq(level))
            .load::<TechCategories>(&_connection)
            .expect("E");

        if _order.types == 1 {
            use schema::services::dsl::services;
            use crate::models::Service;
            let _services = services
                .filter(schema::services::id.eq(object_id))
                .load::<Service>(&_connection)
                .expect("E");
            let _service = _services
                .into_iter()
                .nth(0)
                .unwrap();

            #[derive(TemplateOnce)]
            #[template(path = "desctop/pages/edit_order.stpl")]
            struct Template {
                title:     String,
                object:    Service,
                order:     Order,
                files:     Vec<OrderFile>,
                tech_cats: Vec<TechCategories>,
                level:     i16,
            }
            let body = Template {
                title:     "Редактирование заказа".to_string(),
                object:    _service,
                order:     _order,
                files:     _files,
                tech_cats: _tech_categories,
                level:     level,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }

        else if _order.types == 2 {
            use schema::stores::dsl::stores;
            use crate::models::Store;
            let _stores = stores
                .filter(schema::stores::id.eq(object_id))
                .load::<Store>(&_connection)
                .expect("E");
            let _store = _stores
                .into_iter()
                .nth(0)
                .unwrap();

            #[derive(TemplateOnce)]
            #[template(path = "desctop/pages/edit_order.stpl")]
            struct Template {
                title:     String,
                object:    Store,
                order:     Order,
                files:     Vec<OrderFile>,
                tech_cats: Vec<TechCategories>,
                level:     i16,
            }
            let body = Template {
                title:     "Редактирование заказа".to_string(),
                object:    _store,
                order:     _order,
                files:     _files,
                tech_cats: _tech_categories,
                level:     level,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }

        else if _order.types == 3 {
            use schema::works::dsl::works;
            use crate::models::Work;
            let _works = works
                .filter(schema::works::id.eq(object_id))
                .load::<Work>(&_connection)
                .expect("E");
            let _work = _works
                .into_iter()
                .nth(0)
                .unwrap();

            #[derive(TemplateOnce)]
            #[template(path = "desctop/pages/edit_order.stpl")]
            struct Template {
                title:     String,
                object:    Work,
                order:     Order,
                files:     Vec<OrderFile>,
                tech_cats: Vec<TechCategories>,
                level:     i16,
            }
            let body = Template {
                title:     "Редактирование заказа".to_string(),
                object:    _work,
                order:     _order,
                files:     _files,
                tech_cats: _tech_categories,
                level:     level,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Непонятно, к какому типу относится объект..."))
        }
    }
    else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied."))
    }
}
