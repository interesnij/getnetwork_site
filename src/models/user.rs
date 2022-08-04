use crate::schema;
use crate::schema::{
    users,
    cookie_users,
    cookie_stats,
    stat_mainpages,
    stat_blog_categories,
    stat_service_categories,
    stat_store_categories,
    stat_wiki_categories,
    stat_work_categories,
    stat_tags,
    stat_abouts,
    stat_contacts,
    stat_teams,
    stat_partnerships,
    stat_logins,
    stat_logouts,
    stat_signups,
    stat_helps,
    stat_infos,
    stat_profils,
};
use diesel::{
    Queryable,
    Insertable,
};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use crate::diesel::{ExpressionMethods, RunQueryDsl};
use actix_web::web::Json;


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

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryResponse {
    pub id:     i32,
    pub link:   String,
    pub title:  String,
    pub height: f64,
    pub speed:  i16,
}

impl CookieStat {
    pub fn create(user_id: i32, page: i16, link: String,
        title: String, height: f64, speed: i16) -> Json<HistoryResponse> {

        let _connection = establish_connection();
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


////////////////////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct StatMainpage {
    pub id:   i32,
    pub view: i32,
}
////////////////////
#[derive(Debug, Deserialize, Insertable)]
#[table_name="stat_mainpages"]
pub struct NewStatMainpage {
    pub view: i32,
}

////////////////////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct StatBlogCategorie {
    pub id:   i32,
    pub view: i32,
}
////////////////////
#[derive(Debug, Deserialize, Insertable)]
#[table_name="stat_blog_categories"]
pub struct NewStatBlogCategorie {
    pub view: i32,
}

////////////////////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct StatServiceCategorie {
    pub id:   i32,
    pub view: i32,
}
////////////////////
#[derive(Debug, Deserialize, Insertable)]
#[table_name="stat_service_categories"]
pub struct NewStatServiceCategorie {
    pub view: i32,
}

////////////////////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct StatStoreCategorie {
    pub id:   i32,
    pub view: i32,
}
////////////////////
#[derive(Debug, Deserialize, Insertable)]
#[table_name="stat_store_categories"]
pub struct NewStatStoreCategorie {
    pub view: i32,
}

////////////////////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct StatWikiCategorie {
    pub id:   i32,
    pub view: i32,
}
////////////////////
#[derive(Debug, Deserialize, Insertable)]
#[table_name="stat_wiki_categories"]
pub struct NewStatWikiCategorie {
    pub view: i32,
}

////////////////////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct StatWorkCategorie {
    pub id:   i32,
    pub view: i32,
}
////////////////////
#[derive(Debug, Deserialize, Insertable)]
#[table_name="stat_work_categories"]
pub struct NewStatWorkCategorie {
    pub view: i32,
}

////////////////////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct StatTag {
    pub id:   i32,
    pub view: i32,
}
////////////////////
#[derive(Debug, Deserialize, Insertable)]
#[table_name="stat_tags"]
pub struct NewStatTag {
    pub view: i32,
}

////////////////////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct StatAbout {
    pub id:   i32,
    pub view: i32,
}
////////////////////
#[derive(Debug, Deserialize, Insertable)]
#[table_name="stat_abouts"]
pub struct NewStatAbout {
    pub view: i32,
}

////////////////////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct StatInfo {
    pub id:   i32,
    pub view: i32,
}
////////////////////
#[derive(Debug, Deserialize, Insertable)]
#[table_name="stat_infos"]
pub struct NewStatInfo {
    pub view: i32,
}

////////////////////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct StatContact {
    pub id:   i32,
    pub view: i32,
}
////////////////////
#[derive(Debug, Deserialize, Insertable)]
#[table_name="stat_contacts"]
pub struct NewStatContact {
    pub view: i32,
}

////////////////////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct StatTeam {
    pub id:   i32,
    pub view: i32,
}
////////////////////
#[derive(Debug, Deserialize, Insertable)]
#[table_name="stat_teams"]
pub struct NewStatTeam {
    pub view: i32,
}

////////////////////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct StatPartnership {
    pub id:   i32,
    pub view: i32,
}
////////////////////
#[derive(Debug, Deserialize, Insertable)]
#[table_name="stat_partnerships"]
pub struct NewStatPartnership {
    pub view: i32,
}

////////////////////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct StatLogin {
    pub id:   i32,
    pub view: i32,
}
////////////////////
#[derive(Debug, Deserialize, Insertable)]
#[table_name="stat_logins"]
pub struct NewStatLogin {
    pub view: i32,
}

////////////////////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct StatLogout {
    pub id:   i32,
    pub view: i32,
}
////////////////////
#[derive(Debug, Deserialize, Insertable)]
#[table_name="stat_logouts"]
pub struct NewStatLogout {
    pub view: i32,
}

////////////////////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct StatSignup {
    pub id:   i32,
    pub view: i32,
}
////////////////////
#[derive(Debug, Deserialize, Insertable)]
#[table_name="stat_signups"]
pub struct NewStatSignup {
    pub view: i32,
}

////////////////////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct StatHelp {
    pub id:   i32,
    pub view: i32,
}
////////////////////
#[derive(Debug, Deserialize, Insertable)]
#[table_name="stat_helps"]
pub struct NewStatHelp {
    pub view: i32,
}

////////////////////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct StatProfil {
    pub id:   i32,
    pub view: i32,
}
////////////////////
#[derive(Debug, Deserialize, Insertable)]
#[table_name="stat_profils"]
pub struct NewStatProfil {
    pub view: i32,
}
