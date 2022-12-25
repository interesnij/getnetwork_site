use argonautica::{Hasher, Verifier};
use actix_session::Session;
use actix_web::{
  http::header::CONTENT_TYPE,
  HttpRequest,
};
//use crate::schema;
use crate::{errors::AuthError, vars};
use crate::models::SessionUser;
use actix_web::dev::ConnectionInfo;


pub fn hash_password(password: &str) -> String {
  Hasher::default()
      .with_password(password)
      .with_secret_key(vars::secret_key().as_str())
      .hash()
      .expect("E.")
      //.map_err(|_| AuthError::AuthenticationError(String::from("Не удалось хэшировать пароль")))
}

pub fn verify(hash: &str, password: &str) -> Result<bool, AuthError> {
  Verifier::default()
      .with_hash(hash)
      .with_password(password)
      .with_secret_key(vars::secret_key().as_str())
      .verify()
      .map_err(|_| AuthError::AuthenticationError(String::from("Не удалось подтвердить пароль")))
}

pub fn is_json_request(req: &HttpRequest) -> bool {
    req
      .headers()
      .get(CONTENT_TYPE)
      .map_or(
        false,
        |header| header.to_str().map_or(false, |content_type| "application/json" == content_type)
      )
}

pub fn is_signed_in(session: &Session) -> bool {
  match get_current_user(session) {
      Ok(_) => true,
      _ => false,
  }
}

pub fn set_current_user(session: &Session, user: &SessionUser) -> () {
    // сериализация в строку подходит для этого случая,
    // но двоичный код был бы предпочтительнее в производственных вариантах использования.
    session.insert("user", serde_json::to_string(user).unwrap()).unwrap();
}

pub fn check_auth(session: &Session) -> bool {
    match session.get::<String>("id").unwrap() {
        Some(_) => true,
        None => false,
    }
} 
 
pub fn get_current_user(session: &Session) -> Result<SessionUser, AuthError> {
    let msg = "Не удалось извлечь пользователя из сеанса";

    session.get::<String>("user")
        .map_err(|_| AuthError::AuthenticationError(String::from(msg)))
        .unwrap() 
        .map_or(
          Err(AuthError::AuthenticationError(String::from(msg))),
          |user| serde_json::from_str(&user).or_else(|_| Err(AuthError::AuthenticationError(String::from(msg))))
        )
}


pub async fn get_cookie_user_id(req: &HttpRequest) -> i32 {
    let mut user_id = 0;
    for header in req.headers().into_iter() {
        if header.0 == "cookie" {
            let str_cookie = header.1.to_str().unwrap();
            let _cookie: Vec<&str> = str_cookie.split(";").collect();
            for c in _cookie.iter() {
                let split_c: Vec<&str> = c.split("=").collect();
                if split_c[0] == "user" {
                    user_id = split_c[1].parse().unwrap();
                }
                println!("name {:?}", split_c[0].trim());
                println!("value {:?}", split_c[1]);
            }
        }
    };
    user_id
}
pub async fn get_or_create_cookie_user_id(conn: ConnectionInfo, req: &HttpRequest) -> i32 {
    let mut user_id = 0;
    for header in req.headers().into_iter() {
        if header.0 == "cookie" {
            let str_cookie = header.1.to_str().unwrap();
            let _cookie: Vec<&str> = str_cookie.split(";").collect();
            for c in _cookie.iter() {
                let split_c: Vec<&str> = c.split("=").collect();
                if split_c[0] == "user" {
                    user_id = split_c[1].parse().unwrap();
                }
                println!("name {:?}", split_c[0].trim());
                println!("value {:?}", split_c[1]);
            }
        }
    };
    if user_id == 0 {
        use crate::views::create_c_user;

        let user = create_c_user(conn, &req).await;
        user_id = user.id;
    }
    else {
        use crate::views::get_c_user;

        let user = get_c_user(conn, user_id, &req).await;
        user_id = user.id;
    }
    user_id
}
