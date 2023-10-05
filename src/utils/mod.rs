mod forms;
mod auth;
mod stat;

pub use self::{
    forms::*,
    auth::*,
    stat::*,
};
use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
};

use crate::schema;
use serde::{Deserialize, Serialize};
use crate::models::{
    Categories,
    User,
    Cat,
};
use crate::diesel::{
    Connection,
    PgConnection,
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use actix_session::Session;
use crate::errors::AuthError;
use sailfish::TemplateOnce;
use std::cell::Cell;
use std::sync::{Arc, Mutex};


pub struct AppState {
    pub server_id: usize,
    pub request_count: Cell<usize>,
    pub messages: Arc<Mutex<Vec<String>>>,
}
#[derive(Serialize)]
pub struct IndexResponse {
    pub server_id: usize,
    pub request_count: usize,
    pub messages: Vec<String>,
}

pub fn get_price_acc_values(price: &i32) -> Option<i32> {
    if price > &3_000_000 {
        let acc = (price * 10) / 100; // 10% скидка
        return Some(acc);
    }
    else if price > &2_000_000 && price < &3_000_000 {
        let acc = (price * 7) / 100; // 10% скидка
        return Some(acc);
    }
    else if price > &1_000_000 && price < &2_000_000 {
        let acc = (price * 5) / 100; // 5% скидка
        return Some(acc);
    }
    else {
        return None;
    }
}

pub fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_auth_template(req: &HttpRequest) -> i16 {
    #[derive(Deserialize)]
    struct TemplateParams {
        pub template: Option<i16>,
    }
    let params_some = web::Query::<TemplateParams>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        if params.template.is_some() {
        let template = params.template.unwrap();
            if template > 0 && template < 3 {
                set_template(template);
                return template;
            }
            else {
                return 1;
            }
        }
        else {
            return get_template_storage();
        }
    }
    else {
        return get_template_storage();
    }
}

pub fn get_template_storage() -> u8 {
    let template_res = web_local_storage_api::get_item("template");
    if template_res.is_ok() {
        let template_some = template_res.expect("E.");
        if template_some.is_some() {
            return template_some.unwrap();
        }
    }
    return 1;
}
pub fn get_linguage_storage() -> u8 {
    let linguage_res = web_local_storage_api::get_item("linguage");
    if linguage_res.is_ok() {
        let linguage_some = linguage_res.expect("E.");
        if linguage_some.is_some() {
            return linguage_some.unwrap();
        }
    }
    return 1;
}

pub struct StorageParams {
    pub template: u8,
    pub linguage: u8, 
}
pub fn get_all_storage() -> StorageParams {
    return StorageParams {
        template: get_template_storage(),
        linguage: get_linguage_storage(),
    }
}

pub fn set_template(types: u8) -> () {
    web_local_storage_api::set_item("template", types);
}
pub fn set_linguage(types: u8) -> () {
    web_local_storage_api::set_item("linguage", types);
}


fn get_content_type<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    return req.headers().get("user-agent")?.to_str().ok();
}
pub fn is_desctop(req: &HttpRequest) -> bool {
    return get_content_type(req).unwrap().contains("Mobile");
} 

    pub fn get_device_and_ajax(req: &HttpRequest) -> (bool, i32) {
        #[derive(Debug, Deserialize)]
        struct Params {
            pub ajax: Option<i32>,
        }
        let params_some = web::Query::<Params>::from_query(&req.query_string());
        let mut is_ajax = 0;
        let _type = true;

        if params_some.is_ok() {
            let params = params_some.unwrap();
            if params.ajax.is_some() {
                is_ajax = params.ajax.unwrap();
            }
            else {
                is_ajax = 0;
            }
        }

        (is_desctop(req), is_ajax)
    }

    pub fn get_categories_2() -> (
        Vec<Cat>,
        Vec<Cat>,
        Vec<Cat>,
        Vec<Cat>,
        Vec<Cat>,
        Vec<Cat>
    ) { 
        let _service_cats = Categories::get_categories_for_types(2).expect("E.");
        let _store_cats = Categories::get_categories_for_types(3).expect("E.");
        let _blog_cats = Categories::get_categories_for_types(1).expect("E.");
        let _wiki_cats = Categories::get_categories_for_types(4).expect("E.");
        let _work_cats = Categories::get_categories_for_types(5).expect("E.");
        let _help_cats = Categories::get_categories_for_types(6).expect("E.");

        return (
            _service_cats,
            _store_cats,
            _blog_cats,
            _wiki_cats,
            _work_cats,
            _help_cats
        );
    }
//}

pub fn get_page(req: &HttpRequest) -> i32 {
    #[derive(Debug, Deserialize)]
    struct Params {
        pub page: Option<i32>,
    }
    let params_some = web::Query::<Params>::from_query(&req.query_string());
    let page: i32;
    if params_some.is_ok() {
        let params = params_some.unwrap();
        if params.page.is_some() {
            page = params.page.unwrap();
        }
        else {
            page = 1;
        }
    }
    else {
        page = 1;
    }
    page
}


pub fn get_request_user_data(session: &Session) -> User {
    use crate::models::SessionUser;
    use crate::schema::users::dsl::users;

    let _connection = establish_connection();
    let mut user_id = 0;
    if let Some(user_str) = session.get::<String>("user")
        .map_err(|_| AuthError::AuthenticationError(String::from("Не удалось извлечь пользователя из сеанса")))
        .unwrap() {
            let user: SessionUser = serde_json::from_str(&user_str).expect("E.");
            user_id = user.id;
        }
    if user_id != 0 {
        users
            .filter(schema::users::id.eq(user_id))
            .first::<User>(&_connection)
            .expect("E")
    } else {
        users
            .filter(schema::users::id.eq(1))
            .first::<User>(&_connection)
            .expect("E")
    }
}

pub async fn get_first_load_page (
    req:         &HttpRequest
    session:     &Session,
    is_desctop:  bool,
    title:       String,
    description: String,
    uri:         String,
    image:       String,
    t:           u8,
    l:           u8,
) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)] 
            #[template(path = "desctop/generic/first_load.stpl")]
            struct Template {
                request_user:   User,
                title:          String,
                description:    String,
                image:          String,
                uri:            String,
                template_types: u8,
                linguage:       u8,
            }
            let body = Template {
                request_user:   _request_user,
                title:          title,
                description:    description,
                image:          image,
                uri:            uri,
                template_types: t,
                linguage:       l,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/generic/first_load.stpl")]
            struct Template {
                request_user:   User,
                title:          String,
                description:    String,
                image:          String,
                uri:            String,
                template_types: u8,
                linguage:       u8,
            }
            let body = Template {
                request_user:   _request_user,
                title:          title,
                description:    description,
                image:          image,
                uri:            uri,
                template_types: t,
                linguage:       l,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/generic/anon_first_load.stpl")]
            struct Template {
                title:          String,
                description:    String,
                image:          String,
                uri:            String,
                template_types: u8,
                linguage:       u8,
            }
            let body = Template {
                title:          title,
                description:    description,
                image:          image,
                uri:            uri,
                template_types: t,
                linguage:       l,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/generic/anon_first_load.stpl")]
            struct Template {
                title:          String,
                description:    String,
                image:          String,
                uri:            String,
                template_types: u8,
                linguage:       u8,
            }
            let body = Template {
                title:          title,
                description:    description,
                image:          image,
                uri:            uri,
                template_types: t,
                linguage:       l,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn get_private_page (
    is_ajax:     i32,
    user:        User,
    is_desctop:  bool,
    title:       String,
    description: String,
    uri:         String,
    image:       String,
    t:           u8,
    l:           u8,
) -> actix_web::Result<HttpResponse> {
    if is_desctop {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/generic/private_object.stpl")]
        struct Template {
            is_ajax:        i32,
            request_user:   User,
            title:          String,
            description:    String,
            image:          String,
            uri:            String,
            template_types: u8,
            linguage:       u8,
        }
        let body = Template {
            is_ajax:        is_ajax,
            request_user:   user,
            title:          title,
            description:    description,
            image:          image,
            uri:            uri,
            template_types: t,
            linguage:       l,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/generic/private_object.stpl")]
        struct Template {
            is_ajax:        i32,
            title:          String,
            description:    String,
            image:          String,
            uri:            String,
            template_types: u8,
            linguage:       u8,
        }
        let body = Template {
            is_ajax:        is_ajax,
            title:          title,
            description:    description,
            image:          image,
            uri:            uri,
            template_types: t,
            linguage:       l,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub async fn get_anon_private_page (
    is_ajax:     i32,
    is_desctop:  bool,
    title:       String,
    description: String,
    uri:         String,
    image:       String,
    t:           u8,
    l:           u8,
) -> actix_web::Result<HttpResponse> {
    if is_desctop {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/generic/anon_private_object.stpl")]
        struct Template {
            is_ajax:        i32,
            title:          String,
            description:    String,
            image:          String,
            uri:            String,
            template_types: u8,
            linguage:       u8,
        }
        let body = Template {
            is_ajax:        is_ajax,
            title:          title,
            description:    description,
            image:          image,
            uri:            uri,
            template_types: t,
            linguage:       l,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/generic/anon_private_object.stpl")]
        struct Template {
            is_ajax:        i32,
            title:          String,
            description:    String,
            image:          String,
            uri:            String,
            template_types: u8,
            linguage:       u8,
        }
        let body = Template {
            is_ajax:        is_ajax,
            title:          title,
            description:    description,
            image:          image,
            uri:            uri,
            template_types: t,
            linguage:       l,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub fn get_count_for_ru(count: i16, word1: String, word2: String, word3: String) -> String {
    let a = count % 10;
    let b = count % 100;
    let count_str: String = count.to_string().parse().unwrap();
    if a == 1 && b != 11 {
        return count_str + &word1;
    }
    else if a >= 2 && a <= 4 && (b < 10 || b >= 20) {
        return count_str + &word2;
    }
    else {
        return count_str + &word3;
    }
}
