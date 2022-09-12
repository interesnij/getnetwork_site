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
    stat_helps,
    stat_infos,
    help_items,
    help_item_categories,
};
use crate::diesel::{
    Queryable,
    Insertable,
    QueryDsl,
    ExpressionMethods,
    RunQueryDsl
};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use actix_web::web::Json;


#[derive(Debug, Queryable, Serialize, Identifiable)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionUser {
    pub id:       i32,
    pub username: String,
}

#[derive(Debug, Queryable, Serialize, Identifiable)]
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
    pub height:     f64,
    pub seconds:    i32,
    pub created:    chrono::NaiveDateTime,
}
impl CookieUser {
    pub fn get_users_list(page: i32, limit: i32) -> (Vec<CookieUser>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<CookieUser>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = CookieUser::get_users(limit.into(), step.into());
        }
        else {
            have_next = limit + 1;
            object_list = CookieUser::get_users(limit.into(), 0);
        }
        if CookieUser::get_users(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        return (object_list, next_page_number);
    }
    pub fn get_users(limit: i64, offset: i64) -> Vec<CookieUser> {
        use crate::schema::cookie_users::dsl::cookie_users;

        let _connection = establish_connection();
        return cookie_users
            .filter(schema::cookie_users::seconds.ne(0))
            .filter(schema::cookie_users::height.ne(0.0))
            .order(schema::cookie_users::created.desc())
            .limit(limit)
            .offset(offset)
            .load::<CookieUser>(&_connection)
            .expect("E.");
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
    pub height:     f64,
    pub seconds:    i32,
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
// 12 - заказы
// 13 - история
// 14 - статистика

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

#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CookieStat {
    pub id:       i32,
    pub user_id:  i32,
    pub page:     i16,
    pub link:     String,
    pub title:    String,
    pub height:   f64,
    pub seconds:  i32,
    pub created:  chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryResponse {
    pub id:      i32,
    pub link:    String,
    pub title:   String,
    pub height:  f64,
    pub seconds: i32,
}

impl CookieStat {
    pub fn get_stat_list(user_id: i32, page: i32, limit: i32) -> (Vec<CookieStat>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<CookieStat>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = CookieStat::get_stat_items(user_id, limit.into(), step.into());
        }
        else {
            have_next = limit + 1;
            object_list = CookieStat::get_stat_items(user_id, limit.into(), 0);
        }
        if CookieStat::get_stat_items(user_id, 1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        return (object_list, next_page_number);
    }
    pub fn get_stat_items(user_id: i32, limit: i64, offset: i64) -> Vec<CookieStat> {
        use crate::schema::cookie_stats::dsl::cookie_stats;

        let _connection = establish_connection();
        return cookie_stats
            .filter(schema::cookie_stats::user_id.eq(user_id))
            .order(schema::cookie_stats::created.desc())
            .limit(limit)
            .offset(offset)
            .load::<CookieStat>(&_connection)
            .expect("E.");
    }
    pub fn create(user_id: i32, page: i16, link: String,
        title: String, height: f64, seconds: i32) -> Json<HistoryResponse> {
        use chrono::Duration;

        let _connection = establish_connection();
        let _h = NewCookieStat {
            user_id: user_id,
            page:    page,
            link:    link.clone(),
            title:   title.clone(),
            height:  height,
            seconds: seconds,
            created: chrono::Local::now().naive_utc() + Duration::hours(3),
        };
        diesel::insert_into(schema::cookie_stats::table)
            .values(&_h)
            .get_result::<CookieStat>(&_connection)
            .expect("Error.");

        return Json(HistoryResponse {
            id:      user_id,
            link:    link,
            title:   title,
            height:  height,
            seconds: seconds,
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
    pub seconds:  i32,
    pub created:  chrono::NaiveDateTime,
}


////////////////////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct StatMainpage {
    pub id:      i32,
    pub view:    i32,
    pub height:  f64,
    pub seconds: i32,
}
////////////////////
#[derive(Debug, Deserialize, Insertable)]
#[table_name="stat_mainpages"]
pub struct NewStatMainpage {
    pub view:    i32,
    pub height:  f64,
    pub seconds: i32,
}

////////////////////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct StatBlogCategorie {
    pub id:      i32,
    pub view:    i32,
    pub height:  f64,
    pub seconds: i32,
}
////////////////////
#[derive(Debug, Deserialize, Insertable)]
#[table_name="stat_blog_categories"]
pub struct NewStatBlogCategorie {
    pub view:    i32,
    pub height:  f64,
    pub seconds: i32,
}

////////////////////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct StatServiceCategorie {
    pub id:      i32,
    pub view:    i32,
    pub height:  f64,
    pub seconds: i32,
}
////////////////////
#[derive(Debug, Deserialize, Insertable)]
#[table_name="stat_service_categories"]
pub struct NewStatServiceCategorie {
    pub view:    i32,
    pub height:  f64,
    pub seconds: i32,
}

////////////////////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct StatStoreCategorie {
    pub id:      i32,
    pub view:    i32,
    pub height:  f64,
    pub seconds: i32,
}
////////////////////
#[derive(Debug, Deserialize, Insertable)]
#[table_name="stat_store_categories"]
pub struct NewStatStoreCategorie {
    pub view:    i32,
    pub height:  f64,
    pub seconds: i32,
}

////////////////////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct StatWikiCategorie {
    pub id:      i32,
    pub view:    i32,
    pub height:  f64,
    pub seconds: i32,
}
////////////////////
#[derive(Debug, Deserialize, Insertable)]
#[table_name="stat_wiki_categories"]
pub struct NewStatWikiCategorie {
    pub view:    i32,
    pub height:  f64,
    pub seconds: i32,
}

////////////////////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct StatWorkCategorie {
    pub id:      i32,
    pub view:    i32,
    pub height:  f64,
    pub seconds: i32,
}
////////////////////
#[derive(Debug, Deserialize, Insertable)]
#[table_name="stat_work_categories"]
pub struct NewStatWorkCategorie {
    pub view:    i32,
    pub height:  f64,
    pub seconds: i32,
}

////////////////////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct StatTag {
    pub id:       i32,
    pub view:    i32,
    pub height:  f64,
    pub seconds: i32,
}
////////////////////
#[derive(Debug, Deserialize, Insertable)]
#[table_name="stat_tags"]
pub struct NewStatTag {
    pub view:    i32,
    pub height:  f64,
    pub seconds: i32,
}

////////////////////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct StatInfo {
    pub id:      i32,
    pub view:    i32,
    pub height:  f64,
    pub seconds: i32,
}
////////////////////
#[derive(Debug, Deserialize, Insertable)]
#[table_name="stat_infos"]
pub struct NewStatInfo {
    pub view:    i32,
    pub height:  f64,
    pub seconds: i32,
}


////////////////////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct StatHelp {
    pub id:      i32,
    pub view:    i32,
    pub height:  f64,
    pub seconds: i32,
}
////////////////////
#[derive(Debug, Deserialize, Insertable)]
#[table_name="stat_helps"]
pub struct NewStatHelp {
    pub view:    i32,
    pub height:  f64,
    pub seconds: i32,
}

////////////////////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct HelpItemCategorie {
    pub id:       i32,
    pub title:    String,
    pub view:     i32,
    pub height:   f64,
    pub seconds:  i32,
    pub position: i32,

}
impl HelpItemCategorie {
    pub fn get_list(&self) -> Vec<HelpItem> {
        use crate::schema::help_items::dsl::help_items;

        let _connection = establish_connection();
        return help_items
            .filter(schema::help_items::category_id.eq(self.id))
            .order(schema::help_items::id.asc())
            .load::<HelpItem>(&_connection)
            .expect("E");
    }
}
////////////////////
#[derive(Debug, Deserialize, AsChangeset, Insertable)]
#[table_name="help_item_categories"]
pub struct NewHelpItemCategorie {
    pub title:    String,
    pub view:     i32,
    pub height:   f64,
    pub seconds:  i32,
    pub position: i32,
}

#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct HelpItem {
    pub id:          i32,
    pub category_id: i32,
    pub title:       String,
    pub content:     String,
    pub position:    i16,
}

#[derive(Debug, Deserialize, AsChangeset, Insertable)]
#[table_name="help_items"]
pub struct NewHelpItem {
    pub category_id: i32,
    pub title:       String,
    pub content:     String,
    pub position:    i16,
}
