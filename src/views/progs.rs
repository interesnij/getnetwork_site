use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    Error,
    web,
    post,
    //error::InternalError,
    //http::StatusCode,
};
use crate::schema;
use crate::models::{
    CookieUser,
    HistoryResponse,
    Categories,
    Tag,
    Item,
};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use crate::utils::{
    establish_connection,
    is_signed_in,
    get_request_user_data,
    MyWs,
};
use crate::diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use actix_session::Session;
use actix_multipart::{Field, Multipart};
use futures::StreamExt;
use std::str;
use std::borrow::BorrowMut;
use actix_web::dev::ConnectionInfo;


pub fn progs_routes(config: &mut web::ServiceConfig) {
    config.route("/create_history/", web::post().to(create_history));
    config.route("/object_history/{id}/", web::get().to(object_history));
    config.route("/feedback/", web::post().to(create_feedback));

    config.route("/create_item/", web::post().to(create_item));
    config.route("/edit_item/{id}/", web::post().to(edit_item));
    config.route("/delete_item/{id}/", web::get().to(delete_item));
    config.route("/publish_item/{id}/", web::get().to(publish_item));
    config.route("/hide_item/{id}/", web::get().to(hide_item));
    config.route("/edit_content_item/{id}/", web::post().to(edit_content_item));

    config.route("/create_category/", web::post().to(create_category));
    config.route("/edit_category/{id}/", web::post().to(edit_category));
    config.route("/delete_category/{id}/", web::get().to(delete_category));

    config.route("/create_files/{id}/", web::post().to(create_files));
    config.route("/edit_file/{id}/", web::post().to(edit_file));
    config.route("/delete_file/{id}/", web::get().to(delete_file));
}

pub async fn create_c_user(conn: ConnectionInfo, req: &HttpRequest) -> CookieUser {
    use crate::models::NewCookieUser;
    use chrono::Duration;

    #[derive(Debug, Deserialize)]
    pub struct UserLoc {
        pub city:    CityLoc,
        pub region:  RegionLoc,
        pub country: CountryLoc,
    }
    #[derive(Debug, Deserialize)]
    pub struct CityLoc {
        pub name_ru: String,
        pub name_en: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct RegionLoc {
        pub name_ru: String,
        pub name_en: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct CountryLoc {
        pub name_ru: String,
        pub name_en: String,
    }

    let _connection = establish_connection();
    let mut device: i16 = 1;
    for header in req.headers().into_iter() {
        if header.0 == "user-agent" {
            let str_agent = header.1.to_str().unwrap();
            if str_agent.contains("Mobile") {
                device = 2;
            };
            break;
        }
    };

    let mut ipaddr: String = String::new();
    let ip = conn.realip_remote_addr();
    if ip.is_some() {
        ipaddr = ip.unwrap().to_string();
    }
    else if let Some(val) = &req.peer_addr() {
        ipaddr = val.ip().to_string();
    };
    let _geo_url = "http://api.sypexgeo.net/J5O6d/json/".to_string() + &ipaddr;
    let _geo_request = reqwest::get(_geo_url).await.expect("E.");
    let new_request = _geo_request.text().await.unwrap();
    //println!("request {:?}", new_request);

    let location200: UserLoc = serde_json::from_str(&new_request).unwrap();
    let _user = NewCookieUser {
        ip:         ipaddr,
        device:     device,
        city_ru:    Some(location200.city.name_ru),
        city_en:    Some(location200.city.name_en),
        region_ru:  Some(location200.region.name_ru),
        region_en:  Some(location200.region.name_en),
        country_ru: Some(location200.country.name_ru),
        country_en: Some(location200.country.name_en),
        height:     0.0,
        seconds:    0,
        created:    chrono::Local::now().naive_utc() + Duration::hours(3),
    };
    let _new_user = diesel::insert_into(schema::cookie_users::table)
        .values(&_user)
        .get_result::<CookieUser>(&_connection)
        .expect("Error.");
    return _new_user;
}

pub async fn get_c_user(conn: ConnectionInfo, id: i32, req: &HttpRequest) -> CookieUser {
    if id > 0 {
        use crate::schema::cookie_users::dsl::cookie_users;

        let _connection = establish_connection();
        let _users = cookie_users
            .filter(schema::cookie_users::id.eq(id))
            .load::<CookieUser>(&_connection)
            .expect("E");

        if _users.len() > 0 {
            return _users.into_iter().nth(0).unwrap();
        }
        else {
            return create_c_user(conn, &req).await;
        }
    }
    else {
        return create_c_user(conn, &req).await;
    }
}

#[derive(Debug, Deserialize)]
pub struct HistoryData {
    pub user_id:   i32,
    pub object_id: i32,
    pub page_id:   i16,
    pub link:      String,
    pub title:     String,
    pub height:    f64,
    pub seconds:   i32,
}
pub async fn create_history(conn: ConnectionInfo, data: web::Json<HistoryData>, req: HttpRequest) -> web::Json<HistoryResponse> {
    use crate::models::CookieStat;
    use crate::schema::cookie_stats::dsl::cookie_stats;
    use crate::utils::plus_page_stat;

    //let data: HistoryData = serde_json::from_str(&data).unwrap();
    println!("link{:?}", data.link);
    let p_id = data.user_id;
    let user = get_c_user(conn, p_id, &req).await;

    let p_object_id = data.object_id;
    let p_page_id = data.page_id;
    let p_height = data.height;
    //let format_height = format!("{:.2}", _p_height);
    //let p_height: f64 = format_height.parse().unwrap();

    let p_seconds = data.seconds;
    let p_link = data.link.clone();
    let p_title = data.title.clone();

    let _connection = establish_connection();

    if cookie_stats
        .filter(schema::cookie_stats::user_id.eq(p_id))
        .filter(schema::cookie_stats::link.eq(p_link.clone()))
        .select(schema::cookie_stats::id)
        .load::<i32>(&_connection)
        .expect("E.")
        .len() == 0 {
        diesel::update(&user)
            .set ((
                schema::cookie_users::height.eq(user.height + p_height),
                schema::cookie_users::seconds.eq(user.seconds + p_seconds),
            ))
            .get_result::<CookieUser>(&_connection)
            .expect("Error.");
        if p_object_id > 0 {
            match p_page_id {
                42 => {
                    use crate::utils::plus_category_stat;
                    plus_category_stat(p_object_id, p_height, p_seconds)
                },
                43 => {
                    use crate::utils::plus_item_stat;
                    plus_item_stat(p_object_id, p_height, p_seconds)
                },
                62 => {
                    use crate::utils::plus_category_stat;
                    plus_category_stat(p_object_id, p_height, p_seconds)
                },
                63 => {
                    use crate::utils::plus_item_stat;
                    plus_item_stat(p_object_id, p_height, p_seconds)
                },
                72 => {
                    use crate::utils::plus_category_stat;
                    plus_category_stat(p_object_id, p_height, p_seconds)
                },
                73 => {
                    use crate::utils::plus_item_stat;
                    plus_item_stat(p_object_id, p_height, p_seconds)
                },
                82 => {
                    use crate::utils::plus_category_stat;
                    plus_category_stat(p_object_id, p_height, p_seconds)
                },
                83 => {
                    use crate::utils::plus_item_stat;
                    plus_item_stat(p_object_id, p_height, p_seconds)
                },
                92 => {
                    use crate::utils::plus_category_stat;
                    plus_category_stat(p_object_id, p_height, p_seconds)
                },
                93 => {
                    use crate::utils::plus_item_stat;
                    plus_item_stat(p_object_id, p_height, p_seconds)
                },
                32 => {
                    use crate::utils::plus_tag_stat;
                    plus_tag_stat(p_object_id, p_height, p_seconds)
                },
                9 => {
                    use crate::utils::plus_category_stat;
                    plus_category_stat(p_object_id, p_height, p_seconds)
                },
                _ => println!("no value"),
            };
        }
        else {
            plus_page_stat(p_page_id, p_height, p_seconds)
        }
    }

    return CookieStat::create (
        user.id,
        p_page_id,
        p_link,
        p_title,
        p_height,
        p_seconds,
    )
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectResponse {
    pub id:         i32,
    pub ip:         String,
    pub device:     i16,
    pub city_ru:    Option<String>,
    pub city_en:    Option<String>,
    pub region_ru:  Option<String>,
    pub region_en:  Option<String>,
    pub country_ru: Option<String>,
    pub country_en: Option<String>,
}
pub async fn object_history(conn: ConnectionInfo, req: HttpRequest, id: web::Path<i32>) -> web::Json<ObjectResponse> {
    let _user = get_c_user(conn, *id, &req).await;
    return web::Json( ObjectResponse {
        id:         _user.id,
        ip:         _user.ip,
        device:     _user.device,
        city_ru:    _user.city_ru,
        city_en:    _user.city_en,
        region_ru:  _user.region_ru,
        region_en:  _user.region_en,
        country_ru: _user.country_ru,
        country_en: _user.country_en,
    })
}

pub async fn create_feedback(mut payload: actix_multipart::Multipart) -> impl Responder {
    use crate::schema::feedbacks;
    use crate::models::{Feedback, NewFeedback};
    use crate::utils::feedback_form;

    let _connection = establish_connection();
    let form = feedback_form(payload.borrow_mut()).await;
    let new_feedback = NewFeedback {
        username: form.username.clone(),
        email:    form.email.clone(),
        message:  form.message.clone()
    };
    let _new_feedback = diesel::insert_into(feedbacks::table)
        .values(&new_feedback)
        .get_result::<Feedback>(&_connection)
        .expect("E.");
    return HttpResponse::Ok();
}


pub async fn create_item(session: Session, mut payload: Multipart) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use crate::models::{
                TechCategoriesItem,
                NewTechCategoriesItem,
                Serve,
                ServeItems,
                NewServeItems,
                NewCategory,
                Category,
                NewItem,
                TagItems,
                NewTagItems,
            };
            use crate::utils::{
                item_form,
                get_price_acc_values,
            };

            let _connection = establish_connection();

            let form = item_form(payload.borrow_mut(), _request_user.id).await;
            let types = form.types;
            let new_item = NewItem::create (
                form.title.clone(),
                form.description.clone(),
                form.link.clone(),
                form.main_image.clone(),
                _request_user.id,
                form.position,
                types,
                form.slug.clone(),
            );

            let _item = diesel::insert_into(schema::items::table)
                .values(&new_item)
                .get_result::<Item>(&_connection)
                .expect("E.");

            for category_id in form.category_list.iter() {
                let new_category = NewCategory {
                    categories_id: *category_id,
                    item_id:       _item.id,
                    types:         types,
                };
                diesel::insert_into(schema::category::table)
                    .values(&new_category)
                    .get_result::<Category>(&_connection)
                    .expect("E.");
            };
            for tag_id in form.tags_list.iter() {
                let new_tag = NewTagItems {
                    tag_id: *tag_id,
                    item_id: _item.id,
                    types:   types,
                    created: chrono::Local::now().naive_utc(),
                };
                diesel::insert_into(schema::tags_items::table)
                    .values(&new_tag)
                    .get_result::<TagItems>(&_connection)
                    .expect("Error.");
            }

            // создаем связь с тех категориями, которые будут
            // расширять списки опций, предлагая доп возможности и услуги
            for cat_id in form.close_tech_cats_list.iter() {
                let new_cat = NewTechCategoriesItem {
                    category_id: *cat_id,
                    item_id:     _item.id,
                    types:       types,
                    is_active:   2,
                };
                diesel::insert_into(schema::tech_categories_items::table)
                    .values(&new_cat)
                    .get_result::<TechCategoriesItem>(&_connection)
                    .expect("Error.");
            }

            // создаем опции услуги и записываем id опций в вектор.
            let mut serve_ids = Vec::new();
            for serve_id in form.serve_list.iter() {
                let new_serve_form = NewServeItems {
                    serve_id: *serve_id,
                    item_id:  _item.id,
                    types:    types,
                };
                diesel::insert_into(schema::serve_items::table)
                    .values(&new_serve_form)
                    .get_result::<ServeItems>(&_connection)
                    .expect("Error.");
                serve_ids.push(*serve_id);
            }

            // получаем опции, чтобы создать связи с их тех. категорией.
            // это надо отрисовки тех категорий услуги, которые активны
            let _serves = schema::serve::table
                .filter(schema::serve::id.eq_any(serve_ids))
                .load::<Serve>(&_connection)
                .expect("E");

            let mut tech_cat_ids = Vec::new();
            let mut item_price = 0;
            for _serve in _serves.iter() {
                if !tech_cat_ids.iter().any(|&i| i==_serve.tech_cat_id) {
                    tech_cat_ids.push(_serve.tech_cat_id);
                }
                item_price += _serve.price;
            }

            for id in tech_cat_ids.iter() {
                let new_cat = NewTechCategoriesItem {
                    category_id: *id,
                    item_id:     _item.id,
                    types:       types,
                    is_active:   1,
                };
                diesel::insert_into(schema::tech_categories_items::table)
                    .values(&new_cat)
                    .get_result::<TechCategoriesItem>(&_connection)
                    .expect("Error.");
            }

            // фух. Связи созданы все, но надо еще посчитать цену
            // услуги для калькулятора. Как? А  это будет сумма всех
            // цен выбранных опций.
            let price_acc = get_price_acc_values(&item_price);
            diesel::update(&_item)
                .set((
                    schema::items::price.eq(item_price),
                    schema::items::price_acc.eq(price_acc),
                ))
                .get_result::<Item>(&_connection)
                .expect("Error.");
        }
    };
    HttpResponse::Ok()
}

pub async fn edit_item(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use crate::schema::{
                tags::dsl::tags,
                items::dsl::items,
                serve_items::dsl::serve_items,
                tags_items::dsl::tags_items,
                categories::dsl::categories,
                category::dsl::category,
                tech_categories_items::dsl::tech_categories_items,
                serve::dsl::serve,
            };

            use crate::models::{
                TechCategoriesItem,
                NewTechCategoriesItem,
                Serve,
                ServeItems,
                NewServeItems,
                Category,
                NewCategory,
                TagItems,
                NewTagItems,
                EditItem,
            };
            use crate::utils::{
                item_form,
                get_price_acc_values,
            };

            let _connection = establish_connection();
            let _item_id: i32 = *_id;
            let _items = items
                .filter(schema::items::id.eq(_item_id))
                .load::<Item>(&_connection)
                .expect("E");
            let _item = _items.into_iter().nth(0).unwrap();

            if _item.is_active {
                let _categories = _item.get_categories_obj();
                let _tags = _item.get_tags_obj();
                for _category in _categories.iter() {
                    diesel::update(_category)
                        .set(schema::categories::count.eq(_category.count - 1))
                        .get_result::<Categories>(&_connection)
                        .expect("Error.");
                };
                for _tag in _tags.iter() {
                    diesel::update(_tag)
                        .set(schema::tags::count.eq(_tag.count - 1))
                        .get_result::<Tag>(&_connection)
                        .expect("Error.");
                };
            }

            diesel::delete (
                tags_items
                    .filter(schema::tags_items::item_id.eq(_item_id))
                    .filter(schema::tags_items::types.eq(_item.types))
                )
                .execute(&_connection)
                .expect("E");
            diesel::delete (
                serve_items
                    .filter(schema::serve_items::item_id.eq(_item_id))
                    .filter(schema::serve_items::types.eq(_item.types))
                )
                .execute(&_connection)
                .expect("E");
            diesel::delete (
                tech_categories_items
                    .filter(schema::tech_categories_items::item_id.eq(_item_id))
                    .filter(schema::tech_categories_items::types.eq(_item.types))
                )
                .execute(&_connection)
                .expect("E");
            diesel::delete (
                category
                    .filter(schema::category::item_id.eq(_item_id))
                    .filter(schema::category::types.eq(_item.types))
                )
                .execute(&_connection)
                .expect("E");

            let form = item_form(payload.borrow_mut(), _request_user.id).await;
            let _new_item = EditItem {
                title:       form.title.clone(),
                description: form.description.clone(),
                link:        form.link.clone(),
                image:       form.main_image.clone(),
                position:    form.position,
                slug:        form.slug.clone(),
            };

            diesel::update(&_item)
                .set(_new_item)
                .get_result::<Item>(&_connection)
                .expect("E");

            for category_id in form.category_list.iter() {
                let new_category = NewCategory {
                    categories_id: *category_id,
                    item_id:       _item.id,
                    types:         _item.types,
                };
                diesel::insert_into(schema::category::table)
                    .values(&new_category)
                    .get_result::<Category>(&_connection)
                    .expect("E.");

                if _item.is_active {
                    let _category = categories
                        .filter(schema::categories::id.eq(category_id))
                        .filter(schema::categories::types.eq(_item.types))
                        .load::<Categories>(&_connection)
                        .expect("E");
                    diesel::update(&_category[0])
                        .set(schema::categories::count.eq(_category[0].count + 1))
                        .get_result::<Categories>(&_connection)
                        .expect("Error.");
                }
            };
            for tag_id in form.tags_list.iter() {
                let new_tag = NewTagItems {
                    tag_id: *tag_id,
                    item_id: _item.id,
                    types:   _item.types,
                    created: chrono::Local::now().naive_utc(),
                };
                diesel::insert_into(schema::tags_items::table)
                    .values(&new_tag)
                    .get_result::<TagItems>(&_connection)
                    .expect("Error.");

                if _item.is_active {
                    let _tag = tags.filter(schema::tags::id.eq(tag_id)).load::<Tag>(&_connection).expect("E");

                    diesel::update(&_tag[0])
                        .set(schema::tags::count.eq(_tag[0].count + 1))
                        .get_result::<Tag>(&_connection)
                        .expect("Error.");
                }
            }

            // создаем связь с тех категориями, которые будут
            // расширять списки опций, предлагая доп возможности и услуги
            for cat_id in form.close_tech_cats_list.iter() {
                let new_cat = NewTechCategoriesItem {
                    category_id: *cat_id,
                    item_id:     _item.id,
                    types:       _item.types,
                    is_active:   2,
                };
                diesel::insert_into(schema::tech_categories_items::table)
                    .values(&new_cat)
                    .get_result::<TechCategoriesItem>(&_connection)
                    .expect("Error.");
            }

            // создаем опции услуги и записываем id опций в вектор.
            let mut serve_ids = Vec::new();
            for serve_id in form.serve_list.iter() {
                let new_serve_form = NewServeItems {
                    serve_id: *serve_id,
                    item_id:  _item.id,
                    types:    _item.types,
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
            let mut item_price = 0;
            for _serve in _serves.iter() {
                if !tech_cat_ids.iter().any(|&i| i==_serve.tech_cat_id) {
                    tech_cat_ids.push(_serve.tech_cat_id);
                }
                item_price += _serve.price;
            }

            for id in tech_cat_ids.iter() {
                let new_cat = NewTechCategoriesItem {
                    category_id: *id,
                    item_id:     _item.id,
                    types:       _item.types,
                    is_active:   1,
                };
                diesel::insert_into(schema::tech_categories_items::table)
                    .values(&new_cat)
                    .get_result::<TechCategoriesItem>(&_connection)
                    .expect("Error.");
            }

            // фух. Связи созданы все, но надо еще посчитать цену
            // услуги для калькулятора. Как? А  это будет сумма всех
            // цен выбранных опций.
            let price_acc = get_price_acc_values(&item_price);
            diesel::update(&_item)
                .set((
                    schema::items::price.eq(item_price),
                    schema::items::price_acc.eq(price_acc),
                ))
                .get_result::<Item>(&_connection)
                .expect("Error.");
        }
    };
    HttpResponse::Ok()
}

pub async fn create_category(session: Session, mut payload: Multipart) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use crate::utils::category_form;
            use crate::models::NewCategories;

            let _connection = establish_connection();
            let form = category_form(payload.borrow_mut(), _request_user.id).await;
            let new_cat = NewCategories {
                name:        form.name.clone(),
                description: Some(form.description.clone()),
                position:    form.position,
                image:       Some(form.image.clone()),
                count:       0,
                view:        0,
                height:      0.0,
                seconds:     0,
                types:       form.types,
                slug:        form.slug,
            };
            diesel::insert_into(schema::categories::table)
                .values(&new_cat)
                .get_result::<Categories>(&_connection)
                .expect("E.");
        }
    }
    return HttpResponse::Ok();
}

pub async fn edit_category(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::models::EditCategories;
    use crate::schema::categories::dsl::categories;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use crate::utils::category_form;

            let _connection = establish_connection();
            let _cat_id: i32 = *_id;
            let _category = categories
                .filter(schema::categories::id.eq(_cat_id))
                .load::<Categories>(&_connection)
                .expect("E");

            let form = category_form(payload.borrow_mut(), _request_user.id).await;
            let _new_cat = EditCategories {
                name:        form.name.clone(),
                description: Some(form.description.clone()),
                position:    form.position,
                image:       Some(form.image.clone()),
                slug:        form.slug,
            };

            diesel::update(&_category[0])
                .set(_new_cat)
                .get_result::<Categories>(&_connection)
                .expect("E");
        }
    }
    HttpResponse::Ok()
}

pub async fn edit_content_item(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::items::dsl::items;

    let _item_id: i32 = *_id;
    let _connection = establish_connection();
    let _items = items
        .filter(schema::items::id.eq(&_item_id))
        .load::<Item>(&_connection)
        .expect("E");

    let _item = _items.into_iter().nth(0).unwrap();

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 || _request_user.id == _item.user_id {
            use crate::utils::content_form;

            let form = content_form(payload.borrow_mut()).await;
            diesel::update(&_item)
            .set(schema::items::content.eq(form.content.clone()))
            .get_result::<Item>(&_connection)
            .expect("E");
        }
    }
    HttpResponse::Ok().body("")
}

pub async fn delete_item(session: Session, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::{
        items::dsl::items,
        tags_items::dsl::tags_items,
        category::dsl::category,
        files::dsl::files,
    };

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let _item_id: i32 = *_id;
            let _items = items
                .filter(schema::items::id.eq(_item_id))
                .load::<Item>(&_connection)
                .expect("E");

            let _item = _items.into_iter().nth(0).unwrap();
            let _categories = _item.get_categories_obj();
            let _tags = _item.get_tags_obj();
            for _category in _categories.iter() {
                diesel::update(_category)
                .set(schema::categories::count.eq(_category.count - 1))
                .get_result::<Categories>(&_connection)
                .expect("Error.");
            };
            for _tag in _tags.iter() {
                diesel::update(_tag)
                .set(schema::tags::count.eq(_tag.count - 1))
                .get_result::<Tag>(&_connection)
                .expect("Error.");
            };

            let _src_list = files
                .filter(schema::files::item_id.eq(_item_id))
                .filter(schema::files::item_types.eq(_item.types))
                .select(schema::files::src)
                .load::<String>(&_connection)
                .expect("E");

            for f in _src_list.iter() {
                std::fs::remove_file(f).expect("E");
            }

            diesel::delete (
                files
                    .filter(schema::files::item_id.eq(_item_id))
                    .filter(schema::files::item_types.eq(_item.types))
                )
                .execute(&_connection)
                .expect("E");
            diesel::delete (
                tags_items
                    .filter(schema::tags_items::item_id.eq(_item_id))
                    .filter(schema::tags_items::types.eq(_item.types))
                )
                .execute(&_connection)
                .expect("E");
            diesel::delete (
                category
                    .filter(schema::category::item_id.eq(_item_id))
                    .filter(schema::category::types.eq(_item.types))
                )
                .execute(&_connection)
                .expect("E");
            diesel::delete(&_item).execute(&_connection).expect("E");
        }
    }
    HttpResponse::Ok()
}

pub async fn delete_category(session: Session, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::categories::dsl::categories;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let _cat_id: i32 = *_id;
            let _category = categories
                .filter(schema::categories::id.eq(_cat_id))
                .load::<Categories>(&_connection)
                .expect("E");

            diesel::delete(categories.filter(schema::categories::id.eq(_cat_id)))
                .execute(&_connection)
                .expect("E");
        }
    }
    HttpResponse::Ok()
}

pub async fn create_files(session: Session, mut payload: Multipart, id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use crate::utils::files_form;
            use crate::schema::items::dsl::items;
            use crate::models::{NewFile, File};

            let form = files_form(payload.borrow_mut(), _request_user.id).await;
            let types = form.types;
            let item_types = form.item_types;

            let _connection = establish_connection();
            let _items = items
                .filter(schema::items::id.eq(*id))
                .filter(schema::items::types.eq(item_types))
                .load::<Item>(&_connection)
                .expect("E");
            let _item = _items.into_iter().nth(0).unwrap();

            for file in form.files.iter() {
                let new_file = NewFile::create (
                    _request_user.id,
                    _item.id,
                    item_types,
                    types,
                    file.to_string()
                );
                diesel::insert_into(schema::files::table)
                    .values(&new_file)
                    .get_result::<File>(&_connection)
                    .expect("E.");
            };
        }
    }
    HttpResponse::Ok()
}

pub async fn edit_file(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::models::{EditFile, File};
    use crate::schema::files::dsl::files;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use crate::utils::category_form;

            let _connection = establish_connection();
            let _file_id: i32 = *_id;
            let _file = files
                .filter(schema::files::id.eq(_file_id))
                .load::<File>(&_connection)
                .expect("E");

            let form = category_form(payload.borrow_mut(), _request_user.id).await;
            let _new_file = EditFile {
                description: Some(form.description.clone()),
                position:    form.position,
            };

            diesel::update(&_file[0])
                .set(_new_file)
                .get_result::<File>(&_connection)
                .expect("E");
        }
    }
    HttpResponse::Ok()
}

pub async fn delete_file(session: Session, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::files::dsl::files;
    use crate::models::File;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let _file_id: i32 = *_id;
            let _file = files
                .filter(schema::files::id.eq(_file_id))
                .load::<File>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();
            std::fs::remove_file(_file.src).expect("E");

            diesel::delete(files.filter(schema::files::id.eq(_file_id)))
                .execute(&_connection)
                .expect("E");
        }
    }
    HttpResponse::Ok()
}

pub async fn publish_item(session: Session, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use crate::schema::items::dsl::items;

            let _connection = establish_connection();
            let _id: i32 = *_id;
            let _item = items
                .filter(schema::items::id.eq(_id))
                .load::<Item>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();

            let _categories = _item.get_categories_obj();
            for _category in _categories.iter() {
                diesel::update(_category)
                    .set(schema::categories::count.eq(_category.count + 1))
                    .get_result::<Categories>(&_connection)
                    .expect("Error.");
            };
            let _tag_list = _item.get_tags_obj();
            for _tag in _tag_list.iter() {
                diesel::update(_tag)
                    .set(schema::tags::count.eq(_tag.count + 1))
                    .get_result::<Tag>(&_connection)
                    .expect("Error.");
            };

            diesel::update(&_item)
                .set(schema::items::is_active.eq(true))
                .get_result::<Item>(&_connection)
                .expect("Error.");
        }
    }
    HttpResponse::Ok()
}
pub async fn hide_item(session: Session, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use crate::schema::items::dsl::items;

            let _connection = establish_connection();
            let _id: i32 = *_id;
            let _item = items
                .filter(schema::items::id.eq(_id))
                .load::<Item>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
            .unwrap();

            let _categories = _item.get_categories_obj();
            for _category in _categories.iter() {
                if _category.count < 0 {
                    diesel::update(_category)
                        .set(schema::categories::count.eq(0))
                        .get_result::<Categories>(&_connection)
                        .expect("Error.");
                } else {
                    diesel::update(_category)
                        .set(schema::categories::count.eq(_category.count - 1))
                        .get_result::<Categories>(&_connection)
                        .expect("Error.");
                }
            };
            let _tag_list = _item.get_tags_obj();
            for _tag in _tag_list.iter() {
                if _tag.count < 0 {
                    diesel::update(_tag)
                        .set(schema::tags::count.eq(0))
                        .get_result::<Tag>(&_connection)
                        .expect("Error.");
                } else {
                    diesel::update(_tag)
                        .set(schema::tags::count.eq(_tag.count - 1))
                        .get_result::<Tag>(&_connection)
                        .expect("Error.");
                }
            };

            diesel::update(&_item)
                .set(schema::items::is_active.eq(false))
                .get_result::<Item>(&_connection)
                .expect("Error.");
        }
    }
    HttpResponse::Ok()
}
