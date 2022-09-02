use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
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
use serde::Deserialize;
use std::str;
use sailfish::TemplateOnce;
use crate::models::User;


pub fn order_routes(config: &mut web::ServiceConfig) {
    config.route("/orders/", web::get().to(get_orders_page));
    config.route("/user_orders/", web::get().to(get_user_orders_page));
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
                object_list:      Vec<Order>,
                next_page_number: i32,
            }
            let body = Template {
                title:            "Заказы".to_string(),
                request_user:     _request_user,
                is_ajax:          is_ajax,
                object_list:      _orders,
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
                object_list:      Vec<Order>,
                next_page_number: i32,
            }
            let body = Template {
                title:            "Заказы".to_string(),
                is_ajax:          is_ajax,
                object_list:      _orders,
                next_page_number: next_page_number,
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
    if is_ajax == 0 {
        get_first_load_page(&session, is_desctop, "Ваши заказы".to_string()).await
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
                    title:            String,
                    request_user:     User,
                    object_list:      Vec<Order>,
                    is_ajax:          i32,
                    next_page_number: i32,
                }
                let body = Template {
                    title:            "Ваши заказы".to_string(),
                    request_user:     _request_user,
                    object_list:      _orders,
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
                    object_list:      Vec<Order>,
                    is_ajax:          i32,
                    next_page_number: i32,
                }
                let body = Template {
                    title:            "Ваши заказы".to_string(),
                    request_user:     _request_user,
                    object_list:      _orders,
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
                    object_list:      Vec<Order>,
                    is_ajax:          i32,
                    next_page_number: i32,
                }
                let body = Template {
                    title:            "Ваши заказы".to_string(),
                    object_list:      _orders,
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
                    object_list:      Vec<Order>,
                    is_ajax:          i32,
                    next_page_number: i32,
                }
                let body = Template {
                    title:            "Ваши заказы".to_string(),
                    object_list:      _orders,
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
    let _order_id: i32 = *_id;
    let user_id = get_cookie_user_id(&req).await;

    let _orders = orders
        .filter(schema::orders::id.eq(&_order_id))
        .load::<Order>(&_connection)
        .expect("E");
    let _order = _orders.into_iter().nth(0).unwrap();
    if is_ajax == 0 {
        get_first_load_page(&session, is_desctop, "Заказ ".to_string() + &_order.title).await
    }
    else if user_id != _order.user_id {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Информация о заказчике не найдена"))
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
    #[derive(TemplateOnce)]
    #[template(path = "desctop/pages/create_order.stpl")]
    struct Template {
        title:  String,
    }
    let body = Template {
        title:  "Создание заказа".to_string(),
    }
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
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

    let user_id = get_cookie_user_id(&req).await;

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
                .filter(schema::services::id.eq(_order.object_id))
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
                .filter(schema::stores::id.eq(_order.object_id))
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
                .filter(schema::works::id.eq(_order.object_id))
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

pub async fn create_order(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    use crate::schema::serve::dsl::serve;
    use crate::models::{
        TechCategoriesItem,
        NewTechCategoriesItem,
        Serve,
        ServeItems,
        NewServeItems,
    };
    use crate::utils::{
        order_form,
        get_price_acc_values,
    };

    let _connection = establish_connection();
    let user_id = get_or_create_cookie_user_id(&req).await;

    if user_id != 0 {
        let form = order_form(payload.borrow_mut(), user_id).await;
        let new_order = NewOrder::create (
            form.title.clone(),
            form.types,
            form.object_id,
            form.username.clone(),
            form.email.clone(),
            form.description.clone(),
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
                .get_result::<OrderFile>(&_connection)
                .expect("E.");
        };

        // создаем опции услуги и записываем id опций в вектор.
        let mut serve_ids = Vec::new();
        for serve_id in form.serve_list.iter() {
            let new_serve_form = NewServeItems {
                serve_id:   *serve_id,
                service_id: 0,
                store_id:   0,
                work_id:    0,
                orders_id:  Some(_order.id),
            };
            diesel::insert_into(schema::serve_items::table)
                .values(&new_serve_form)
                .get_result::<ServeItems>(&_connection)
                .expect("Error.");
            serve_ids.push(*serve_id);
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
            if !tech_cat_ids.iter().any(|&i| i==_serve.tech_cat_id) {
                tech_cat_ids.push(_serve.tech_cat_id);
            }
            order_price += _serve.price;
        }

        for id in tech_cat_ids.iter() {
            let new_cat = NewTechCategoriesItem {
                category_id: *id,
                service_id:  0,
                store_id:    0,
                work_id:     0,
                types:       1,
                orders_id:   Some(_order.id),
            };
            diesel::insert_into(schema::tech_categories_items::table)
                .values(&new_cat)
                .get_result::<TechCategoriesItem>(&_connection)
                .expect("Error.");
        }

        // фух. Связи созданы все, но надо еще посчитать цену
        // услуги для калькулятора. Как? А  это будет сумма всех
        // цен выбранных опций.
        let price_acc = get_price_acc_values(&order_price);
        diesel::update(&_order)
            .set((
                schema::orders::price.eq(order_price),
                schema::orders::price_acc.eq(price_acc),
            ))
            .get_result::<Order>(&_connection)
            .expect("Error.");
    }
    HttpResponse::Ok()
}

pub async fn edit_order(req: HttpRequest, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
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

    let user_id = get_cookie_user_id(&req).await;

    if user_id == _order.user_id {
        use crate::schema::{
            serve::dsl::serve,
            order_files::dsl::order_files,
            serve_items::dsl::serve_items,
            tech_categories_items::dsl::tech_categories_items,
        };
        use crate::models::{
            TechCategoriesItem,
            NewTechCategoriesItem,
            Serve,
            ServeItems,
            NewServeItems,
            EditOrder,
        };
        use crate::utils::{
            order_form,
            get_price_acc_values,
        };

        diesel::delete(order_files.filter(schema::order_files::order_id.eq(_order_id))).execute(&_connection).expect("E");
        diesel::delete(serve_items.filter(schema::serve_items::orders_id.eq(_order_id))).execute(&_connection).expect("E");
        diesel::delete(tech_categories_items.filter(schema::tech_categories_items::orders_id.eq(_order_id))).execute(&_connection).expect("E");

        let form = order_form(payload.borrow_mut(), user_id).await;
        let _new_order = EditOrder {
            username:    form.username.clone(),
            email:       form.email.clone(),
            description: form.description.clone(),
        };

        diesel::update(&_order)
        .set(_new_order)
        .get_result::<Order>(&_connection)
        .expect("E");

        for _file in form.files.iter() {
            let new_edit_file = NewOrderFile::create (
                _order_id,
                _file.to_string()
            );
            diesel::insert_into(schema::order_files::table)
                .values(&new_edit_file)
                .get_result::<OrderFile>(&_connection)
                .expect("E.");
        };

        // создаем опции услуги и записываем id опций в вектор.
        let mut serve_ids = Vec::new();
        for serve_id in form.serve_list.iter() {
            let new_serve_form = NewServeItems {
                serve_id:   *serve_id,
                service_id: 0,
                store_id:   0,
                work_id:    0,
                orders_id: Some(_order_id)
            };
            diesel::insert_into(schema::serve_items::table)
                .values(&new_serve_form)
                .get_result::<ServeItems>(&_connection)
                .expect("Error.");
            serve_ids.push(*serve_id);
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
            if !tech_cat_ids.iter().any(|&i| i==_serve.tech_cat_id) {
                tech_cat_ids.push(_serve.tech_cat_id);
            }
            order_price += _serve.price;
        }

        for id in tech_cat_ids.iter() {
            let new_cat = NewTechCategoriesItem {
                category_id: *id,
                service_id:  0,
                store_id:    0,
                work_id:     0,
                types:       1,
                orders_id:   Some(_order_id),
            };
            diesel::insert_into(schema::tech_categories_items::table)
                .values(&new_cat)
                .get_result::<TechCategoriesItem>(&_connection)
                .expect("Error.");
        }

        // фух. Связи созданы все, но надо еще посчитать цену
        // услуги для калькулятора, а также скидку. Как? А  это будет сумма всех
        // цен выбранных опций.
        let price_acc = get_price_acc_values(&order_price);
        diesel::update(&_order)
            .set((
                schema::orders::price.eq(order_price),
                schema::orders::price_acc.eq(price_acc),
            ))
            .get_result::<Order>(&_connection)
            .expect("Error.");
    }
    HttpResponse::Ok()
}

pub async fn delete_order(req: HttpRequest, _id: web::Path<i32>) -> impl Responder {
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

    let user_id = get_cookie_user_id(&req).await;

    if user_id == _order.user_id {
        use crate::schema::{
            order_files::dsl::order_files,
            serve_items::dsl::serve_items,
            tech_categories_items::dsl::tech_categories_items,
        };

        diesel::delete(order_files.filter(schema::order_files::order_id.eq(_order_id))).execute(&_connection).expect("E");
        diesel::delete(serve_items.filter(schema::serve_items::orders_id.eq(_order_id))).execute(&_connection).expect("E");
        diesel::delete(tech_categories_items.filter(schema::tech_categories_items::orders_id.eq(_order_id))).execute(&_connection).expect("E");
        diesel::delete(&_order).execute(&_connection).expect("E");
    }
    HttpResponse::Ok()
}
