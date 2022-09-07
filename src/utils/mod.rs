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
use serde::Deserialize;
use crate::models::{
    BlogCategories,
    ServiceCategories,
    StoreCategories,
    WikiCategories,
    WorkCategories,
    User,
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
//lazy_static! {
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

    pub fn get_device_and_ajax(req: &HttpRequest) -> (bool, i32) {
        #[derive(Debug, Deserialize)]
        struct Params {
            pub ajax: Option<i32>,
        }
        let params_some = web::Query::<Params>::from_query(&req.query_string());
        let mut is_ajax = 0;
        let mut _type = true;

        if params_some.is_ok() {
            let params = params_some.unwrap();
            if params.ajax.is_some() {
                is_ajax = params.ajax.unwrap();
            }
            else {
                is_ajax = 0;
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

    pub fn get_categories() -> (
        Vec<ServiceCategories>,
        Vec<StoreCategories>,
        Vec<BlogCategories>,
        Vec<WikiCategories>,
        Vec<WorkCategories>
    ) {
        use crate::schema::{
            service_categories::dsl::service_categories,
            store_categories::dsl::store_categories,
            blog_categories::dsl::blog_categories,
            work_categories::dsl::work_categories,
            wiki_categories::dsl::wiki_categories,
        };

        let _conn = establish_connection();
        let _service_cats = service_categories
            .load::<ServiceCategories>(&_conn)
            .expect("Error");
        let _store_cats = store_categories
            .load::<StoreCategories>(&_conn)
            .expect("Error");
        let _blog_cats = blog_categories
            .load::<BlogCategories>(&_conn)
            .expect("Error");
        let _wiki_cats = wiki_categories
            .load::<WikiCategories>(&_conn)
            .expect("Error");
        let _work_cats = work_categories
            .load::<WorkCategories>(&_conn)
            .expect("Error");

        return (
            _service_cats,
            _store_cats,
            _blog_cats,
            _wiki_cats,
            _work_cats,
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
            .load::<User>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap()
    } else {
        users
            .filter(schema::users::id.eq(1))
            .load::<User>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap()
    }
}

pub async fn get_first_load_page (
    session:     &Session,
    is_desctop:  bool,
    title:       String,
    description: String,
    uri:         String,
    image:       String,
) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/generic/first_load.stpl")]
            struct Template {
                request_user: User,
                title:        String,
                description:  String,
                image:        String,
                uri:          String,
            }
            let body = Template {
                request_user: _request_user,
                title:        title,
                description:  description,
                image:        image,
                uri:          uri,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/generic/first_load.stpl")]
            struct Template {
                request_user: User,
                title:        String,
                description:  String,
                image:        String,
                uri:          String,
            }
            let body = Template {
                request_user: _request_user,
                title:        title,
                description:  description,
                image:        image,
                uri:          uri,
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
                title:        String,
                description:  String,
                image:        String,
                uri:          String,
            }
            let body = Template {
                title:        title,
                description:  description,
                image:        image,
                uri:          uri,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/generic/anon_first_load.stpl")]
            struct Template {
                title:        String,
                description:  String,
                image:        String,
                uri:          String,
            }
            let body = Template {
                title:        title,
                description:  description,
                image:        image,
                uri:          uri,
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
) -> actix_web::Result<HttpResponse> {
    if is_desctop {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/generic/private_object.stpl")]
        struct Template {
            is_ajax:      i32,
            request_user: User,
            title:        String,
            description:  String,
            image:        String,
            uri:          String,
        }
        let body = Template {
            is_ajax:      is_ajax,
            request_user: user,
            title:        title,
            description:  description,
            image:        image,
            uri:          uri,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/generic/private_object.stpl")]
        struct Template {
            is_ajax:      i32,
            request_user: User,
            title:        String,
            description:  String,
            image:        String,
            uri:          String,
        }
        let body = Template {
            is_ajax:      is_ajax,
            request_user: user,
            title:        title,
            description:  description,
            image:        image,
            uri:          uri,
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
) -> actix_web::Result<HttpResponse> {
    if is_desctop {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/generic/anon_private_object.stpl")]
        struct Template {
            is_ajax:     i32,
            title:       String,
            description: String,
            image:       String,
            uri:         String,
        }
        let body = Template {
            is_ajax:     is_ajax,
            title:       title,
            description: description,
            image:       image,
            uri:         uri,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/generic/anon_private_object.stpl")]
        struct Template {
            is_ajax:     i32,
            title:       String,
            description: String,
            image:       String,
            uri:         String,
        }
        let body = Template {
            title:       title,
            description: description,
            image:       image,
            uri:         uri,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
