use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    web,
    error::InternalError,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use crate::utils::{
    establish_connection,
    is_signed_in,
    verify,
};
use crate::diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use crate::schema;
use futures::StreamExt;
use crate::models::{User, NewUser, SessionUser};
use actix_session::Session;
use crate::errors::AuthError;
use actix_multipart::{Field, Multipart};
use std::borrow::BorrowMut;
use futures_util::stream::StreamExt as _;
use sailfish::TemplateOnce;


pub fn auth_routes(config: &mut web::ServiceConfig) {
    config.service(web::resource("/login/")
        .route(web::get().to(login_page))
        .route(web::post().to(login))
    );
    config.service(web::resource("/signup/")
        .route(web::get().to(signup_page))
        .route(web::post().to(process_signup))
    );
    config.route("/logout/", web::get().to(logout_page));
}


pub async fn signup_page(req: HttpRequest, session: Session) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    if is_desctop {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/auth/signup.stpl")]
        struct Template {
            is_ajax: bool,
        }
        let body = Template {
            is_ajax: is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/auth/signup.stpl")]
        struct Template {
            is_ajax: bool,
        }
        let body = Template {
            is_ajax: is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub async fn login_page(req: HttpRequest, session: Session) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    if is_desctop {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/auth/login.stpl")]
        struct Template {
            is_ajax: bool,
        }
        let body = Template {
            is_ajax: is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/auth/login.stpl")]
        struct Template {
            is_ajax: bool,
        }
        let body = Template {
            is_ajax: is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub async fn logout_page(req: HttpRequest, session: Session) -> actix_web::Result<HttpResponse> {
    use crate::utils::is_desctop;

    session.clear();
    if is_desctop(&req) {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/auth/logout.stpl")]
        struct Template;
        let body = Template {}
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/auth/logout.stpl")]
        struct Template;
        let body = Template {}
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

fn find_user(data: LoginUser2) -> Result<SessionUser, AuthError> {
    use crate::schema::users::dsl::users;

    let _connection = establish_connection();
    let mut items = users
        .filter(schema::users::username.eq(&data.username))
        .load::<User>(&_connection)
        .expect("Error.");

    if let Some(user) = items.pop() {
        if let Ok(matching) = verify(&user.password, &data.password) {
            if matching {
                let __user = SessionUser {
                    id:       user.id,
                    username: user.username,
                };
                return Ok(__user.into());
            }
        }
    }
    Err(AuthError::NotFound(String::from("User not found")))
}

fn handle_sign_in(data: LoginUser2,
                session: &Session,
                req: &HttpRequest) -> Result<HttpResponse, AuthError> {
    use crate::utils::{is_json_request, set_current_user};

    let _connection = establish_connection();
    let result = find_user(data);
    let is_json = is_json_request(req);

    match result {
        Ok(user) => {
            set_current_user(&session, &user);
            if is_json {
                Ok(HttpResponse::Ok().json(user))
            } else {
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
            }
        },
        Err(err) => {
            if is_json {
                Ok(HttpResponse::Unauthorized().json(err.to_string()))
            } else {
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
            }
        },
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginUser2 {
    pub username: String,
    pub password: String,
}
pub async fn login_form(payload: &mut Multipart) -> LoginUser2 {
    let mut form: LoginUser2 = LoginUser2 {
        username: "".to_string(),
        password: "".to_string(),
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        while let Some(chunk) = field.next().await {
            let data = chunk.expect("split_payload err chunk");
            if let Ok(s) = std::str::from_utf8(&data) {
                let data_string = s.to_string();
                if field.name() == "username" {
                    form.username = data_string
                } else if field.name() == "password" {
                    form.password = data_string
                }
            }
        }
    }
    form
}

pub async fn login(mut payload: Multipart, session: Session, req: HttpRequest) -> impl Responder {
    if is_signed_in(&session) {
        HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Вы уже авторизованы")
    }
    else {
        let form = login_form(payload.borrow_mut()).await;
        println!("{:?}", form.username.clone());
        println!("{:?}", form.password.clone());
        handle_sign_in(form, &session, &req)
    }
}

#[derive(Deserialize)]
pub struct NewUserForm {
    pub username: String,
    pub email:    String,
    pub password: String,
    pub bio:      Option<String>,
    pub image:    Option<String>,
}

pub async fn process_signup(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::utils::{hash_password, set_current_user};
    use chrono::NaiveDate;

    let params = web::Query::<NewUserForm>::from_query(&req.query_string());
     // Если пользователь не аноним, то отправляем его на страницу новостей
    if is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Вы уже авторизованы"))
    }
    else if params.is_err() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Параметры неверные"))
    }
    else {
        let _connection = establish_connection();
        let params_2 = params.unwrap();
        let form_user = NewUser {
            username: params_2.username.clone(),
            email:    params_2.email.clone(),
            password: params_2.password.clone(),
            bio:      params_2.bio.clone(),
            image:    params_2.image.clone(),
            perm:     1,
        };

        let _new_user = diesel::insert_into(schema::users::table)
            .values(&form_user)
            .get_result::<User>(&_connection)
            .expect("Error saving user.");

        let _session_user = SessionUser {
            id:       _new_user.id,
            username: _new_user.username,
        };

        set_current_user(&session, &_session_user);
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
    }
}
