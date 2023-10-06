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
    is_signed_in,
    verify,
    get_first_load_page,
    get_all_storage,
    StorageParams, NewUserForm,
};
use futures::StreamExt;
use crate::models::{User, SessionUser, StatPage};
use actix_session::Session;
use crate::errors::AuthError;
use actix_multipart::{Field, Multipart};
use std::borrow::BorrowMut;
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
    if is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let (t, l) = get_all_storage();
        if is_ajax == 0 { 
            get_first_load_page (
                &session,
                is_desctop,
                "Регистрация".to_string(),
                "вебсервисы.рф: Регистрация".to_string(),
                "/signup/".to_string(),
                "/static/images/dark/store.jpg".to_string(),
                t, 
                l,
            ).await
        }
        else {
            let _stat = crate::models::StatPage::get_or_create(7);
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/auth/signup.stpl")]
                struct Template {
                    is_ajax:        i32,
                    stat:           StatPage,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    is_ajax:        is_ajax,
                    stat:           _stat,
                    template_types: t,
                    linguage:       l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/auth/signup.stpl")]
                struct Template {
                    is_ajax:        i32,
                    stat:           StatPage,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    is_ajax:        is_ajax,
                    stat:           _stat,
                    template_types: t,
                    linguage:       l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}
pub async fn login_page(req: HttpRequest, session: Session) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let (t, l) = get_all_storage();
        if is_ajax == 0 { 
            get_first_load_page (
                &session,
                is_desctop,
                "Вход".to_string(),
                "вебсервисы.рф: Вход".to_string(),
                "/login/".to_string(),
                "/static/images/dark/store.jpg".to_string(),
                t, 
                l,
            ).await
        }
        else {
            let _stat = crate::models::StatPage::get_or_create(6);

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/auth/login.stpl")]
                struct Template {
                    is_ajax:        i32,
                    stat:           StatPage,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    is_ajax:        is_ajax,
                    stat:           _stat,
                    template_types: t,
                    linguage:       l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/auth/login.stpl")]
                struct Template {
                    is_ajax:        i32,
                    stat:           StatPage,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    is_ajax:        is_ajax,
                    stat:           _stat,
                    template_types: t,
                    linguage:       l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn logout_page(req: HttpRequest, session: Session) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _stat = crate::models::StatPage::get_or_create(8);
        session.clear();
        let (t, l) = get_all_storage();
        if crate::utils::is_desctop(&req) {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/auth/logout.stpl")]
            struct Template {
                is_ajax:        i32,
                stat:           StatPage,
                template_types: u8,
                linguage:       u8,
            }
            let body = Template {
                is_ajax:        0,
                stat:           _stat,
                template_types: t,
                linguage:       l,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/auth/logout.stpl")]
            struct Template {
                is_ajax:        i32,
                stat:           StatPage,
                template_types: u8,
                linguage:       u8,
            }
            let body = Template {
                is_ajax:        0,
                stat:           _stat,
                template_types: t,
                linguage:       l,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

fn find_user(data: LoginUser2) -> Result<SessionUser, AuthError> {
    let user_some = User::get_user_with_username(&data.username); 
    if user_some.is_ok() {
        let _user = user_some.expect("Error.");
        if let Ok(matching) = verify(&_user.password, &data.password) {
            if matching {
                let f_user = SessionUser {
                    id:       _user.id,
                    username: _user.username,
                };
                return Ok(f_user.into());
            }
        }
    }
    Err(AuthError::NotFound(String::from("User not found")))
}

fn handle_sign_in (
    data: LoginUser2,
    session: &Session,
    req: &HttpRequest
) -> Result<HttpResponse, AuthError> {
    use crate::utils::{is_json_request, set_current_user};

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
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let form = login_form(payload.borrow_mut()).await;
        //println!("{:?}", form.username.clone());
        //println!("{:?}", form.password.clone());
        handle_sign_in(form, &session, &req)
    }
}

pub async fn signup_form(payload: &mut Multipart) -> NewUserForm {
    let mut form: NewUserForm = NewUserForm {
        username: "".to_string(),
        email:    "".to_string(),
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
                }
                else if field.name() == "email" {
                    form.email = data_string
                }
                else if field.name() == "password" {
                    form.password = data_string
                }
            }
        }
    }
    form
}
pub async fn process_signup(session: Session, mut payload: Multipart) -> impl Responder {
    // Если пользователь не аноним, то отправляем его на страницу новостей
    if is_signed_in(&session) {
        HttpResponse::Ok().content_type("text/html; charset=utf-8").body("")
    }
    else {
        let form = signup_form(payload.borrow_mut()).await;
        let _new_user = User::create(form);

        let _session_user = SessionUser {
            id:       _new_user.id,
            username: _new_user.username,
        };

        crate::utils::set_current_user(&session, &_session_user);
        HttpResponse::Ok().content_type("text/html; charset=utf-8").body("")
    }
}
