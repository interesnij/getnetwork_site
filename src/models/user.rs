use crate::schema::users;
use diesel::{
    Queryable,
    Insertable,
    RunQueryDsl
};
use serde::{Serialize, Deserialize};
use crate::models::{
    BlogCategories,
    ServiceCategories,
    StoreCategories,
    WikiCategories,
    WorkCategories,
};
use actix_web::HttpRequest;
use crate::utils::establish_connection;


#[derive(Debug ,Queryable, Serialize, Identifiable)]
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
    pub fn is_superuser(&self, req: HttpRequest) -> bool {
        return self.perm > 59;
    }
    pub fn get_categories() -> (
        Vec<ServiceCategories>,
        Vec<StoreCategories>,
        Vec<BlogCategories>,
        Vec<WikiCategories>,
        Vec<WorkCategories>
    ) {
        use crate::schema::service_categories::dsl::service_categories;
        use crate::schema::store_categories::dsl::store_categories;
        use crate::schema::blog_categories::dsl::blog_categories;
        use crate::schema::work_categories::dsl::work_categories;
        use crate::schema::wiki_categories::dsl::wiki_categories;

        let _conn = establish_connection();
        let _service_cats :Vec<ServiceCategories> = service_categories.load(&_conn).expect("Error");
        let _store_cats :Vec<StoreCategories> = store_categories.load(&_conn).expect("Error");
        let _blog_cats :Vec<BlogCategories> = blog_categories.load(&_conn).expect("Error");
        let _wiki_cats :Vec<WikiCategories> = wiki_categories.load(&_conn).expect("Error");
        let _work_cats :Vec<WorkCategories> = work_categories.load(&_conn).expect("Error");

        return (
            _service_cats,
            _store_cats,
            _blog_cats,
            _wiki_cats,
            _work_cats
        );
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
