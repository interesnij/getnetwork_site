mod payload_handler;
mod auth;

pub use self::{
    payload_handler::*,
    auth::*,
};
use actix_web::HttpRequest;


pub fn establish_connection() -> PgConnection {
    use dotenv::dotenv;

    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn is_desctop(req: &HttpRequest) -> bool {
    let mut desctop = true;

    for header in req.headers().into_iter() {
        if header.0 == "user-agent" {
            let _val = format!("{:?}", header.1);
            if _val.contains("Mobile"){
                desctop = false;
            }
        }
    };
    desctop
}

pub fn get_ajax(req: &HttpRequest) -> bool {
    #[derive(Debug, Deserialize)]
    struct Params {
        pub ajax: Option<i32>,
    }
    let params_some = web::Query::<Params>::from_query(&req.query_string());
    let mut is_ajax = false;
    if params_some.is_ok() {
        let params = params_some.unwrap();
        if params.ajax.is_some() {
            is_ajax = true;
        }
    }
    is_ajax
}
pub fn get_device_and_ajax(req: &HttpRequest) -> (bool, bool) {
    #[derive(Debug, Deserialize)]
    struct Params {
        pub ajax: Option<i32>,
    }
    let params_some = web::Query::<Params>::from_query(&req.query_string());
    let mut is_ajax = false;
    let mut _type = true;

    if params_some.is_ok() {
        let params = params_some.unwrap();
        if params.ajax.is_some() {
            is_ajax = true;
        }
    }

    for header in req.headers().into_iter() {
        if header.0 == "user-agent" {
            let _val = format!("{:?}", header.1);
            if _val.contains("Mobile"){
                _type = false;
            }
        }
    };
    (_type, is_ajax)
}

pub fn get_device_and_page_and_ajax(req: &HttpRequest) -> (bool, i32, bool) {
    #[derive(Debug, Deserialize)]
    struct Params {
        pub page: Option<i32>,
        pub ajax: Option<i32>,
    }
    let params_some = web::Query::<Params>::from_query(&req.query_string());
    let page: i32;
    let mut is_ajax = false;
    if params_some.is_ok() {
        let params = params_some.unwrap();
        if params.page.is_some() {
            page = params.page.unwrap();
        }
        else {
            page = 1;
        }
        if params.ajax.is_some() {
            is_ajax = true;
        }
    }
    else {
        page = 1;
    }

    let mut _type = true;
    for header in req.headers().into_iter() {
        if header.0 == "user-agent" {
            let _val = format!("{:?}", header.1);
            if _val.contains("Mobile"){
                _type = false;
            }
        }
    };
    (_type, page, is_ajax)
}
