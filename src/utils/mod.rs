mod payload_handler;
pub use self::{
    payload_handler::*
};

use actix_web::HttpRequest;
use crate::diesel::{Connection, PgConnection, RunQueryDsl};
use crate::models::{
    BlogCategories,
    ServiceCategories,
    StoreCategories,
    WikiCategories,
    WorkCategories,
};
use tera::Tera;
use lazy_static::lazy_static;


lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Template parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec!["html", ".sql"]);
        tera
    };
}

pub fn establish_connection() -> PgConnection {
    use dotenv::dotenv;

    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_template_2(req: HttpRequest) -> (
    String,
    bool,
    Vec<ServiceCategories>,
    Vec<StoreCategories>,
    Vec<BlogCategories>,
    Vec<WikiCategories>,
    Vec<WorkCategories>) {

    use crate::schema::service_categories::dsl::service_categories;
    use crate::schema::store_categories::dsl::store_categories;
    use crate::schema::blog_categories::dsl::blog_categories;
    use crate::schema::work_categories::dsl::work_categories;
    use crate::schema::wiki_categories::dsl::wiki_categories;

    let mut _type = "".to_string();
    let mut _is_admin = "".to_string();
    for header in req.headers().into_iter() {
        if header.0 == "user-agent" {
            let _val = format!("{:?}", header.1);
            if _val.contains("Mobile"){
                _type = "mobile/".to_string();
            } else {
                _type = "desctop/".to_string();
            };
        }
    };
    let _conn = establish_connection();
    let _service_cats :Vec<ServiceCategories> = service_categories.load(&_conn).expect("Error");
    let _store_cats :Vec<StoreCategories> = store_categories.load(&_conn).expect("Error");
    let _blog_cats :Vec<BlogCategories> = blog_categories.load(&_conn).expect("Error");
    let _wiki_cats :Vec<WikiCategories> = wiki_categories.load(&_conn).expect("Error");
    let _work_cats :Vec<WorkCategories> = work_categories.load(&_conn).expect("Error");

    let mut _is_admin : bool = false;
    let _val = format!("{:?}", Some(req.peer_addr()));
    if _val.contains(&"91.239.184.81".to_string()) {
        _is_admin = true;
    };
    (_type,
    _is_admin,
    _service_cats,
    _store_cats,
    _blog_cats,
    _wiki_cats,
    _work_cats)
}
