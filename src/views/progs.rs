use actix::Addr;
use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    web,
    web::{block, Data, Json},
};
use crate::schema;
use crate::models::{
    CookieUser,
    Categories,
    Tag,
    Item,
    CookieStat,
};
use serde::{Deserialize, Serialize};

use crate::utils::{
    establish_connection,
    is_signed_in,
    get_request_user_data,
};
use crate::diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use actix_session::Session;
use actix_multipart::Multipart;
use std::str;
use std::borrow::BorrowMut;
use actix_web::dev::ConnectionInfo;
use crate::errors::Error;
use crate::websocket::{
    //MessageToClient, 
    Server, 
    ws_index
};


pub fn progs_routes(config: &mut web::ServiceConfig) {
    config.route("/ws", web::get().to(ws_index));
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
        created:    chrono::Local::now().naive_utc() + chrono::Duration::hours(3),
    };
    let _new_user = diesel::insert_into(schema::cookie_users::table)
        .values(&_user)
        .get_result::<CookieUser>(&_connection)
        .expect("Error.");
    return _new_user;
}

pub async fn get_c_user(conn: ConnectionInfo, id: i32, req: &HttpRequest) -> CookieUser {
    if id > 0 {
        let _connection = establish_connection();
        let _user = schema::cookie_users::table
            .filter(schema::cookie_users::id.eq(id))
            .first::<CookieUser>(&_connection);

        if _user.is_ok() {
            return _user.expect("E");
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
    pub title_en:  String,
    pub height:    f64,
    pub seconds:   i32,
    pub template:  String,
}
pub async fn create_history (
    conn: ConnectionInfo,
    data: Json<HistoryData>,
    req: HttpRequest,
) -> Result<Json<CookieStat>, Error> {
    let p_id = data.user_id;
    let user = get_c_user(conn, p_id, &req).await;

    let p_object_id = data.object_id;
    let p_page_id = data.page_id;
    let p_height = data.height;

    let p_seconds = data.seconds;
    let p_link = data.link.clone();
    let p_title = data.title.clone();
    let p_title = data.title_en.clone();
    let p_template = data.template.clone();

    let _connection = establish_connection();
    let is_cookie_stats_exists = schema::cookie_stats::table
        .filter(schema::cookie_stats::user_id.eq(p_id))
        .filter(schema::cookie_stats::link.eq(p_link.clone()))
        .select(schema::cookie_stats::id)
        .first::<i32>(&_connection)
        .is_ok();

    if is_cookie_stats_exists {
        diesel::update(&user)
            .set ((
                schema::cookie_users::height.eq(user.height + p_height),
                schema::cookie_users::seconds.eq(user.seconds + p_seconds),
            ))
            .execute(&_connection)
            .expect("Error.");
    }
    if p_object_id > 0 {
        match p_page_id {
            42 => {
                crate::utils::plus_category_stat(p_object_id, p_height, p_seconds, is_cookie_stats_exists)
            },
            43 => {
                crate::utils::plus_item_stat(p_object_id, p_height, p_seconds, is_cookie_stats_exists)
            },
            62 => {
                crate::utils::plus_category_stat(p_object_id, p_height, p_seconds, is_cookie_stats_exists)
            },
            63 => {
                crate::utils::plus_item_stat(p_object_id, p_height, p_seconds, is_cookie_stats_exists)
            },
            72 => {
                crate::utils::plus_category_stat(p_object_id, p_height, p_seconds, is_cookie_stats_exists)
            },
            73 => {
                crate::utils::plus_item_stat(p_object_id, p_height, p_seconds, is_cookie_stats_exists)
            },
            82 => {
                crate::utils::plus_category_stat(p_object_id, p_height, p_seconds, is_cookie_stats_exists)
            },
            83 => {
                crate::utils::plus_item_stat(p_object_id, p_height, p_seconds, is_cookie_stats_exists)
            },
            92 => {
                crate::utils::plus_category_stat(p_object_id, p_height, p_seconds, is_cookie_stats_exists)
            },
            93 => {
                crate::utils::plus_item_stat(p_object_id, p_height, p_seconds, is_cookie_stats_exists)
            },
            32 => {
                crate::utils::plus_tag_stat(p_object_id, p_height, p_seconds, is_cookie_stats_exists)
            },
            9 => {
                crate::utils::plus_category_stat(p_object_id, p_height, p_seconds, is_cookie_stats_exists)
            },
            _ => (),
        };
    }
    else {
        crate::utils::plus_page_stat(p_page_id, p_height, p_seconds, is_cookie_stats_exists)
    }
    let _res = block(move || CookieStat::create (
        user.id,
        p_page_id,
        p_link,
        p_title,
        p_title_en,
        p_height,
        p_seconds,
        p_template
    )).await?;
    let res = _res?;

    Ok(Json(res))
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
    let _connection = establish_connection();
    let form = crate::utils::feedback_form(payload.borrow_mut()).await;
    let new_feedback = crate::models::NewFeedback {
        username: form.username.clone(),
        email:    form.email.clone(),
        message:  form.message.clone()
    };
    let _new_feedback = diesel::insert_into(schema::feedbacks::table)
        .values(&new_feedback)
        .execute(&_connection)
        .expect("E.");
    return HttpResponse::Ok();
}


pub async fn create_item(session: Session, mut payload: Multipart) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use crate::models::{
                NewTechCategoriesItem,
                Serve,
                NewServeItems,
                NewCategory,
                NewItem,
                NewTagItems,
            };

            let _connection = establish_connection();

            let form = crate::utils::item_form(payload.borrow_mut(), _request_user.id).await;
            let types = form.types;
            let new_item = NewItem::create (
                form.title.clone(),
                form.title_en.clone(),
                form.description.clone(),
                form.description_en.clone(),
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

            for category_id in form.category_list.into_iter() {
                let new_category = NewCategory {
                    category_id: category_id,
                    item_id:     _item.id,
                    types:       types,
                };
                diesel::insert_into(schema::category::table)
                    .values(&new_category)
                    .execute(&_connection)
                    .expect("E.");
            };
            for tag_id in form.tags_list.into_iter() {
                let new_tag = NewTagItems {
                    tag_id:  tag_id,
                    item_id: _item.id,
                    types:   types,
                    created: chrono::Local::now().naive_utc(),
                };
                diesel::insert_into(schema::tags_items::table)
                    .values(&new_tag)
                    .execute(&_connection)
                    .expect("Error.");
            }

            // создаем связь с тех категориями, которые будут
            // расширять списки опций, предлагая доп возможности и услуги
            for cat_id in form.close_tech_cats_list.into_iter() {
                let new_cat = NewTechCategoriesItem {
                    category_id: cat_id,
                    item_id:     _item.id,
                    types:       types,
                    is_active:   2,
                };
                diesel::insert_into(schema::tech_categories_items::table)
                    .values(&new_cat)
                    .execute(&_connection)
                    .expect("Error.");
            }

            // создаем опции услуги и записываем id опций в вектор.
            let mut serve_ids = Vec::new();
            for serve_id in form.serve_list.into_iter() {
                let new_serve_form = NewServeItems {
                    serve_id: serve_id,
                    item_id:  _item.id,
                    types:    types,
                };
                diesel::insert_into(schema::serve_items::table)
                    .values(&new_serve_form)
                    .execute(&_connection)
                    .expect("Error.");
                serve_ids.push(serve_id);
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

            for id in tech_cat_ids.into_iter() {
                let new_cat = NewTechCategoriesItem {
                    category_id: id,
                    item_id:     _item.id,
                    types:       types,
                    is_active:   1,
                };
                diesel::insert_into(schema::tech_categories_items::table)
                    .values(&new_cat)
                    .execute(&_connection)
                    .expect("Error.");
            }

            // фух. Связи созданы все, но надо еще посчитать цену
            // услуги для калькулятора. Как? А  это будет сумма всех
            // цен выбранных опций.
            let price_acc = crate::utils::get_price_acc_values(&item_price);
            diesel::update(&_item)
                .set((
                    schema::items::price.eq(item_price),
                    schema::items::price_acc.eq(price_acc),
                ))
                .execute(&_connection)
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
                NewTechCategoriesItem,
                Serve,
                NewServeItems,
                NewCategory,
                NewTagItems,
                EditItem,
            };

            let _connection = establish_connection();
            let _item_id: i32 = *_id;
            let _item = items
                .filter(schema::items::id.eq(_item_id))
                .first::<Item>(&_connection)
                .expect("E");

            if _item.is_active {
                let _categories: Vec<Categories>;
                let _tags: Vec<Tag>;

                let _categories = _item.get_categories_obj().expect("E");
                let _tags = _item.get_tags_obj().expect("E");

                for _category in _categories.iter() {
                    diesel::update(_category)
                        .set(schema::categories::count.eq(_category.count - 1))
                        .execute(&_connection)
                        .expect("Error.");
                };
                for _tag in _tags.iter() {
                    diesel::update(_tag)
                        .set(schema::tags::count.eq(_tag.count - 1))
                        .execute(&_connection)
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

            let form = crate::utils::item_form(payload.borrow_mut(), _request_user.id).await;
            let _new_item = EditItem {
                title:          form.title.clone(),
                title_en:       form.title_en.clone(),
                description:    form.description.clone(),
                description_en: form.description_en.clone(),
                link:           form.link.clone(),
                image:          form.main_image.clone(),
                position:       form.position,
                slug:           form.slug.clone(),
            };

            diesel::update(&_item)
                .set(_new_item)
                .execute(&_connection)
                .expect("E");

            for category_id in form.category_list.into_iter() {
                let new_category = NewCategory {
                    category_id: category_id,
                    item_id:     _item.id,
                    types:       _item.types,
                };
                diesel::insert_into(schema::category::table)
                    .values(&new_category)
                    .execute(&_connection)
                    .expect("E.");

                if _item.is_active {
                    let _category = categories
                        .filter(schema::categories::id.eq(category_id))
                        .filter(schema::categories::types.eq(_item.types))
                        .first::<Categories>(&_connection)
                        .expect("E");
                    diesel::update(&_category)
                        .set(schema::categories::count.eq(_category.count + 1))
                        .execute(&_connection)
                        .expect("Error.");
                }
            };
            for tag_id in form.tags_list.into_iter() {
                let new_tag = NewTagItems {
                    tag_id:  tag_id,
                    item_id: _item.id,
                    types:   _item.types,
                    created: chrono::Local::now().naive_utc(),
                };
                diesel::insert_into(schema::tags_items::table)
                    .values(&new_tag)
                    .execute(&_connection)
                    .expect("Error.");

                if _item.is_active {
                    let _tag = tags
                        .filter(schema::tags::id.eq(tag_id))
                        .first::<Tag>(&_connection)
                        .expect("E");

                    diesel::update(&_tag)
                        .set(schema::tags::count.eq(_tag.count + 1))
                        .execute(&_connection)
                        .expect("Error.");
                }
            }

            // создаем связь с тех категориями, которые будут
            // расширять списки опций, предлагая доп возможности и услуги
            for cat_id in form.close_tech_cats_list.into_iter() {
                let new_cat = NewTechCategoriesItem {
                    category_id: cat_id,
                    item_id:     _item.id,
                    types:       _item.types,
                    is_active:   2,
                };
                diesel::insert_into(schema::tech_categories_items::table)
                    .values(&new_cat)
                    .execute(&_connection)
                    .expect("Error.");
            }

            // создаем опции услуги и записываем id опций в вектор.
            let mut serve_ids = Vec::new();
            for serve_id in form.serve_list.into_iter() {
                let new_serve_form = NewServeItems {
                    serve_id: serve_id,
                    item_id:  _item.id,
                    types:    _item.types,
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
            let mut item_price = 0;
            for _serve in _serves.iter() {
                if !tech_cat_ids.iter().any(|&i| i==_serve.tech_cat_id) {
                    tech_cat_ids.push(_serve.tech_cat_id);
                }
                item_price += _serve.price;
            }

            for id in tech_cat_ids.into_iter() {
                let new_cat = NewTechCategoriesItem {
                    category_id: id,
                    item_id:     _item.id,
                    types:       _item.types,
                    is_active:   1,
                };
                diesel::insert_into(schema::tech_categories_items::table)
                    .values(&new_cat)
                    .execute(&_connection)
                    .expect("Error.");
            }

            // фух. Связи созданы все, но надо еще посчитать цену
            // услуги для калькулятора. Как? А  это будет сумма всех
            // цен выбранных опций.
            let price_acc = crate::utils::get_price_acc_values(&item_price);
            diesel::update(&_item)
                .set((
                    schema::items::price.eq(item_price),
                    schema::items::price_acc.eq(price_acc),
                ))
                .execute(&_connection)
                .expect("Error.");
        }
    };
    HttpResponse::Ok()
}

pub async fn create_category(session: Session, mut payload: Multipart) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let form = crate::utils::category_form(payload.borrow_mut(), _request_user.id).await;
            let new_cat = crate::models::NewCategories {
                name:           form.name.clone(),
                name_en:        form.name_en.clone(),
                description:    Some(form.description.clone()),
                description_en: Some(form.description_en.clone()),
                position:       form.position,
                image:          Some(form.image.clone()),
                count:          0,
                view:           0,
                height:         0.0,
                seconds:        0,
                types:          form.types,
                slug:           form.slug,
            };
            diesel::insert_into(schema::categories::table)
                .values(&new_cat)
                .execute(&_connection)
                .expect("E.");
        }
    }
    return HttpResponse::Ok();
}

pub async fn edit_category(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let _category = schema::categories::table
                .filter(schema::categories::id.eq(*_id))
                .first::<Categories>(&_connection)
                .expect("E");

            let form = crate::utils::category_form(payload.borrow_mut(), _request_user.id).await;
            let _new_cat = crate::models::EditCategories {
                name:           form.name.clone(),
                name_en:        form.name_en.clone(),
                description:    Some(form.description.clone()),
                description_en: Some(form.description_en.clone()),
                position:       form.position,
                image:          Some(form.image.clone()),
                slug:           form.slug,
            };
            diesel::update(&_category)
                .set(_new_cat)
                .execute(&_connection)
                .expect("E");
        }
    }
    HttpResponse::Ok()
}

pub async fn edit_content_item(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    let _connection = establish_connection();
    let _item = schema::items::table
        .filter(schema::items::id.eq(*_id))
        .first::<Item>(&_connection)
        .expect("E");

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 || _request_user.id == _item.user_id {
            let form = crate::utils::content_form(payload.borrow_mut()).await;
            diesel::update(&_item)
                .set((
                    schema::items::content.eq(form.content.clone()),
                    schema::items::content_en.eq(form.content_en.clone()),
                ))
                .execute(&_connection)
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
            let _item = items
                .filter(schema::items::id.eq(*_id))
                .first::<Item>(&_connection)
                .expect("E");

            let _src_list = files
                .filter(schema::files::item_id.eq(*_id))
                .filter(schema::files::item_types.eq(_item.types))
                .select(schema::files::src)
                .load::<String>(&_connection)
                .expect("E");

            for f in _src_list.iter() {
                std::fs::remove_file(f);
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

            let _categories = _item.get_categories_obj().expect("E");
            let _tags = _item.get_tags_obj().expect("E");

            for _category in _categories.iter() {
                diesel::update(_category)
                    .set(schema::categories::count.eq(_category.count - 1))
                    .execute(&_connection)
                    .expect("Error.");
            };
            for _tag in _tags.iter() {
                diesel::update(_tag)
                    .set(schema::tags::count.eq(_tag.count - 1))
                    .execute(&_connection)
                    .expect("Error.");
            };

        }
    }
    HttpResponse::Ok()
}

pub async fn delete_category(session: Session, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            diesel::delete(schema::categories::table.filter(schema::categories::id.eq(*_id)))
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
            use crate::models::NewFile;

            let form = files_form(payload.borrow_mut(), _request_user.id).await;
            let types = form.types;
            let item_types = form.item_types;

            let _connection = establish_connection();
            let _item = items
                .filter(schema::items::id.eq(*id))
                .filter(schema::items::types.eq(item_types))
                .first::<Item>(&_connection)
                .expect("E");

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
                    .execute(&_connection)
                    .expect("E.");
            };
        }
    }
    HttpResponse::Ok()
}

pub async fn edit_file(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let _file = schema::files::table
                .filter(schema::files::id.eq(*_id))
                .first::<crate::models::File>(&_connection)
                .expect("E");

            let form = crate::utils::category_form(payload.borrow_mut(), _request_user.id).await;
            let _new_file = crate::models::EditFile {
                description:    Some(form.description.clone()),
                description_en: Some(form.description_en.clone()),
                position:       form.position,
            };

            diesel::update(&_file)
                .set(_new_file)
                .execute(&_connection)
                .expect("E");
        }
    }
    HttpResponse::Ok()
}

pub async fn delete_file(session: Session, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let _file = schema::files::table
                .filter(schema::files::id.eq(*_id))
                .first::<crate::models::File>(&_connection)
                .expect("E");
            std::fs::remove_file(_file.src).expect("E");

            diesel::delete(files.filter(schema::files::id.eq(*_id)))
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
            let _connection = establish_connection();
            let _item = schema::items::table
                .filter(schema::items::id.eq(*_id))
                .first::<Item>(&_connection)
                .expect("E");

            diesel::update(&_item)
                .set(schema::items::is_active.eq(true))
                .execute(&_connection)
                .expect("Error.");

            let _categories: Vec<Categories>;
            let _tags: Vec<Tag>;

            let tags_o = _item.get_tags_obj().expect("E");
            let categories_o = _item.get_categories_obj().expect("E");
            let cats_res = block(move || categories_o).await;
            let tags_res = block(move || tags_o).await;
            _categories = match cats_res {
                Ok(_ok) => _ok,
                Err(_error) => Vec::new(),
            };
            for _category in _categories.iter() {
                diesel::update(_category)
                    .set(schema::categories::count.eq(_category.count + 1))
                    .execute(&_connection)
                    .expect("Error.");
            }
            _tags = match tags_res {
                Ok(_list) => _list,
                Err(_error) => Vec::new(),
            };
            for _tag in _tags.iter() {
                diesel::update(_tag)
                    .set(schema::tags::count.eq(_tag.count + 1))
                    .execute(&_connection)
                    .expect("Error.");
            }
        }
    }
    HttpResponse::Ok()
}
pub async fn hide_item(session: Session, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let _item = schema::items::table
                .filter(schema::items::id.eq(*_id))
                .first::<Item>(&_connection)
                .expect("E");

            diesel::update(&_item)
                .set(schema::items::is_active.eq(true))
                .execute(&_connection)
                .expect("Error.");

            let _categories: Vec<Categories>;
            let _tags: Vec<Tag>;

            let _categories_0 = _item.get_categories_obj().expect("E");
            let _tags_0 = _item.get_tags_obj().expect("E");
            let cats_res = block(move || _categories_0).await;
            let tags_res = block(move || _tags_0).await;

            _categories = match cats_res {
                Ok(_ok) => _ok,
                Err(_error) => Vec::new(),
            };
            for _category in _categories.iter() {
                diesel::update(_category)
                    .set(schema::categories::count.eq(_category.count - 1))
                    .execute(&_connection)
                    .expect("Error.");
            }
            _tags = match tags_res {
                Ok(_list) => _list,
                Err(_error) => Vec::new(),
            };
            for _tag in _tags.iter() {
                diesel::update(_tag)
                    .set(schema::tags::count.eq(_tag.count - 1))
                    .execute(&_connection)
                    .expect("Error.");
            }
        }
    }
    HttpResponse::Ok()
}
