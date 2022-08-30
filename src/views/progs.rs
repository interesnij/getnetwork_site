use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    web,
    //error::InternalError,
    //http::StatusCode,
};

use crate::models::{CookieUser, HistoryResponse};
use serde::{Deserialize, Serialize};
use crate::utils::{
    establish_connection,
};
use crate::diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};

use std::borrow::BorrowMut;
use actix_multipart::{Field, Multipart};
use futures::StreamExt;
use std::str;


pub fn progs_routes(config: &mut web::ServiceConfig) {
    config.route("/create_history/", web::post().to(create_history));
    config.route("/object_history/{id}/", web::get().to(object_history));
    config.route("/feedback/", web::post().to(create_feedback));
}


pub async fn create_c_user(req: &HttpRequest) -> CookieUser {
    use crate::models::NewCookieUser;
    use crate::schema;

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
    let mut ipaddr: String = String::new();
    if let Some(val) = &req.peer_addr() {
        ipaddr = val.ip().to_string();
    };
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
        created:    chrono::Local::now().naive_utc(),
    };
    let _new_user = diesel::insert_into(schema::cookie_users::table)
        .values(&_user)
        .get_result::<CookieUser>(&_connection)
        .expect("Error.");
    return _new_user;
}

pub async fn get_c_user(id: i32, req: &HttpRequest) -> CookieUser {
    if id > 0 {
        use crate::schema;
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
            return create_c_user(&req).await;
        }
    }
    else {
        return create_c_user(&req).await;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryForm {
    pub user_id:   i32,
    pub object_id: Option<i32>,
    pub page_id:   i16,
    pub link:      String,
    pub title:     String,
    pub height:    f64,
    pub seconds:   i32,
}
pub async fn history_form(payload: &mut Multipart) -> HistoryForm {
    let mut form: HistoryForm = HistoryForm {
        user_id:   0,
        object_id: None,
        page_id:   0,
        link:      "".to_string(),
        title:     "".to_string(),
        height:    0.0,
        seconds:   0,
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();

        if name == "user_id" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.user_id = _int;
                }
            }
        }
        else if name == "object_id" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.object_id = Some(_int);
                }
            }
        }
        else if name == "page_id" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i16 = s.parse().unwrap();
                    form.page_id = _int;
                }
            }
        }
        else if name == "height" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: f64 = s.parse().unwrap();
                    form.height = _int;
                }
            }
        }
        else if name == "seconds" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.seconds = _int;
                }
            }
        }

        else {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "title" {
                        form.title = data_string
                    } else if field.name() == "link" {
                        form.link = data_string
                    }
                }
            }
        }
    }
    form
}

pub async fn create_history(payload: &mut Multipart, req: HttpRequest) -> web::Json<HistoryResponse> {
    use crate::schema;
    use crate::models::CookieStat;
    use crate::schema::cookie_stats::dsl::cookie_stats;

    let form = history_form(payload.borrow_mut()).await;
    let p_id = form.user_id;
    let user = get_c_user(p_id, &req).await;

    let p_object_id = form.object_id;
    let p_page_id = form.page_id;
    let p_height = form.height;
    let p_seconds = form.seconds;
    let p_link = form.link.clone();
    let p_title = form.title.clone();

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
        if p_object_id.is_some() {
            match p_page_id {
                42 => {
                    use crate::utils::plus_blog_category_stat;
                    plus_blog_category_stat(p_object_id.unwrap(), p_height, p_seconds)
                },
                43 => {
                    use crate::utils::plus_blog_stat;
                    plus_blog_stat(p_object_id.unwrap(), p_height, p_seconds)
                },
                62 => {
                    use crate::utils::plus_service_category_stat;
                    plus_service_category_stat(p_object_id.unwrap(), p_height, p_seconds)
                },
                63 => {
                    use crate::utils::plus_service_stat;
                    plus_service_stat(p_object_id.unwrap(), p_height, p_seconds)
                },
                72 => {
                    use crate::utils::plus_store_category_stat;
                    plus_store_category_stat(p_object_id.unwrap(), p_height, p_seconds)
                },
                73 => {
                    use crate::utils::plus_store_stat;
                    plus_store_stat(p_object_id.unwrap(), p_height, p_seconds)
                },
                82 => {
                    use crate::utils::plus_wiki_category_stat;
                    plus_wiki_category_stat(p_object_id.unwrap(), p_height, p_seconds)
                },
                83 => {
                    use crate::utils::plus_wiki_stat;
                    plus_wiki_stat(p_object_id.unwrap(), p_height, p_seconds)
                },
                92 => {
                    use crate::utils::plus_work_category_stat;
                    plus_work_category_stat(p_object_id.unwrap(), p_height, p_seconds)
                },
                93 => {
                    use crate::utils::plus_work_stat;
                    plus_work_stat(p_object_id.unwrap(), p_height, p_seconds)
                },
                32 => {
                    use crate::utils::plus_tag_stat;
                    plus_tag_stat(p_object_id.unwrap(), p_height, p_seconds)
                },
                _ => println!("no value"),
            };
        }
        else {
            match p_page_id {
                1 => {
                    use crate::utils::plus_mainpage_stat;
                    plus_mainpage_stat(p_height, p_seconds)
                },
                9 => {
                    use crate::utils::plus_help_stat;
                    plus_help_stat(p_height, p_seconds)
                },
                10 => {
                    use crate::utils::plus_info_stat;
                    plus_info_stat(p_height, p_seconds)
                },
                31 => {
                    use crate::utils::plus_tags_stat;
                    plus_tags_stat(p_height, p_seconds)
                },
                41 => {
                    use crate::utils::plus_blog_categories_stat;
                    plus_blog_categories_stat(p_height, p_seconds)
                },
                61 => {
                    use crate::utils::plus_service_categories_stat;
                    plus_service_categories_stat(p_height, p_seconds)
                },
                71 => {
                    use crate::utils::plus_store_categories_stat;
                    plus_store_categories_stat(p_height, p_seconds)
                },
                81 => {
                    use crate::utils::plus_wiki_categories_stat;
                    plus_wiki_categories_stat(p_height, p_seconds)
                },
                91 => {
                    use crate::utils::plus_work_categories_stat;
                    plus_work_categories_stat(p_height, p_seconds)
                },
                _ => println!("no value"),
            }
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
pub async fn object_history(req: HttpRequest, id: web::Path<i32>) -> web::Json<ObjectResponse> {
    let _user = get_c_user(*id, &req).await;
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
    use std::borrow::BorrowMut;
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
