use crate::schema;
use crate::schema::{
    users,
    cookie_users,
    cookie_stats,
};
use diesel::{
    Queryable,
    Insertable,
};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use crate::diesel::{ExpressionMethods, RunQueryDsl};
use actix_web::{HttpRequest, web::Json};


#[derive(Debug, Queryable, PartialEq, Serialize, Identifiable)]
pub struct User {
    pub id:       i32,
    pub username: String,
    pub email:    String,
    pub password: String,
    pub bio:      Option<String>,
    pub image:    Option<String>,
    pub perm:     i16,
}

impl User {
    pub fn is_superuser(&self) -> bool {
        return self.perm > 59;
    }
    pub fn create_superuser(&self) -> () {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::users::perm.eq(60))
            .get_result::<User>(&_connection)
            .expect("E");
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub username: String,
    pub email:    String,
    pub password: String,
    pub bio:      Option<String>,
    pub image:    Option<String>,
    pub perm:     i16,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, AsChangeset)]
#[table_name = "users"]
pub struct UserChange {
    pub username: String,
    pub email:    String,
    pub password: String,
    pub bio:      String,
    pub image:    String,
}

#[derive(Debug, Serialize, PartialEq, Deserialize)]
pub struct SessionUser {
    pub id:       i32,
    pub username: String,
}

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

#[derive(Debug, Queryable, PartialEq, Serialize, Identifiable)]
pub struct CookieUser {
    pub id:         i32,
    pub ip:         String,
    pub device:     i16,
    pub city_ru:    Option<String>,
    pub city_en:    Option<String>,
    pub region_ru:  Option<String>,
    pub region_en:  Option<String>,
    pub country_ru: Option<String>,
    pub country_en: Option<String>,
    pub created:    chrono::NaiveDateTime,
}

impl CookieUser {
    pub fn create_user(req: &HttpRequest, device: i16) -> CookieUser {
        let ip = req.peer_addr().unwrap().ip().to_string();
        let _geo_url = "http://api.sypexgeo.net/J5O6d/json/".to_owned() + &ipaddr;
        let _geo_request = reqwest::get(_geo_url).await.expect("E.");
        let new_request = _geo_request.text().await.unwrap();
        let location200: UserLoc = serde_json::from_str(&new_request).unwrap();
        let _user = NewCookieUser {
            ip:         ip,
            device:     device,
            city_ru:    Some(location200.city.name_ru),
            city_en:    Some(location200.city.name_en),
            region_ru:  Some(location200.region.name_ru),
            region_en:  Some(location200.region.name_en),
            country_ru: Some(location200.country.name_ru),
            country_en: Some(location200.country.name_en),
            created:    chrono::Local::now().naive_utc(),
        };
        let _new_user = diesel::insert_into(schema::cookie_users::table)
            .values(&_user)
            .get_result::<CookieUser>(&_connection)
            .expect("Error.");

        return _new_user;
    }
    pub fn get_user(id: i32, req: &HttpRequest) -> i32 {
        if user_id > 0 {
            let _connection = establish_connection();
            let _users = cookie_users
                .filter(schema::cookie_users::id.eq(id))
                .load::<CookieUser>(&_connection)
                .expect("E");

            if _users.len() > 0 {
                return _users.into_iter().nth(0).unwrap();
            }
            else {
                let mut device: i16 = 1;
                for header in req.headers().into_iter() {
                    if header.0 == "user-agent" {
                        let str_agent = header.1.to_str().unwrap();
                        if _val.contains("Mobile") {
                            device = 2;
                        };
                        break;
                    }
                };
                return CookieUser::create_user(device);
            }
        }
        else {
            let mut device: i16 = 1;
            for header in req.headers().into_iter() {
                if header.0 == "user-agent" {
                    let str_agent = header.1.to_str().unwrap();
                    if _val.contains("Mobile") {
                        device = 2;
                    };
                    break;
                }
            };
            return CookieUser::create_user(device);
        }
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="cookie_users"]
pub struct NewCookieUser {
    pub ip:         String,
    pub device:     i16,
    pub city_ru:    Option<String>,
    pub city_en:    Option<String>,
    pub region_ru:  Option<String>,
    pub region_en:  Option<String>,
    pub country_ru: Option<String>,
    pub country_en: Option<String>,
    pub created:    chrono::NaiveDateTime,
}

/////////////////////////
// Шифры посещаемых страниц
// 1 - главная
// 2 - о сайте
// 3 - контакты
// 4 - команда
// 5 - сотрудничество
// 6 - вход
// 7 - регитрация
// 8 - выход
// 9 - вопросы ответы
// 10 - инфо

// 11 - профиль

// 21 - общий поиск
// 22 - поиск статей блога
// 23 - поиск услуг
// 24 - поиск товаров
// 25 - поиск статей обучающих
// 26 - поиск работ

// 31 - теги
// 32 - тег
// 33 - тег - статьи блога
// 34 - тег - услуги
// 35 - тег - товары
// 36 - тег - статьи обучающие
// 37 - тег - работы

// 41 - категории блога
// 42 - категория блога
// 43 - статья блога

// 51 - категории опций
// 52 - категория опций
// 53 - технологии опций
// 54 - технология опций
// 55 - опция

// 61 - категории услуг
// 62 - категория услуг
// 63 - услуга

// 71 - категории товаров
// 72 - категория товаров
// 73 - товар

// 81 - категории обучения
// 82 - категория обучения
// 83 - статья обучения

// 91 - категории работ
// 92 - категория работ
// 93 - работа
////////////////////

#[derive(Debug, Queryable, PartialEq, Serialize, Identifiable)]
#[belongs_to(CookieUser, foreign_key="user_id")]
pub struct CookieStat {
    pub id:       i32,
    pub user_id:  i32,
    pub page:     i16,
    pub link:     String,
    pub title:    String,
    pub height:   f64,
    pub speed:    i16,
    pub created:  chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct HistoryResponse {
    pub id:     String,
    pub link:   String,
    pub title:  String,
    pub height: f64,
    pub speed:  i32,
}

impl CookieStat {
    pub fn create(user_id: i32, page: i16, link: String,
        title: String, height: f64, speed: i32) -> Json<HistoryResponse> {
        let _h = NewCookieStat {
            user_id: user_id,
            page:    page,
            link:    link.clone(),
            title:   title.clone(),
            height:  height,
            speed:   speed,
            created: chrono::Local::now().naive_utc(),
        };
        diesel::insert_into(schema::cookie_stats::table)
            .values(&_h)
            .get_result::<CookieStat>(&_connection)
            .expect("Error.");

        return Json(HistoryResponse {
            id:     user_id,
            link:   link,
            title:  title,
            height: height,
            speed:  speed,
        });
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="cookie_stats"]
pub struct NewCookieStat {
    pub user_id:  i32,
    pub page:     i16,
    pub link:     String,
    pub title:    String,
    pub height:   f64,
    pub speed:    i16,
    pub created:  chrono::NaiveDateTime,
}
