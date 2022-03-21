pub mod work_progs;
pub mod blog_progs;
pub mod service_progs;
pub mod serve_progs;
pub mod store_progs;
pub mod wiki_progs;
pub mod tag_progs;
use actix_web::{
    HttpRequest,
    Responder,
    HttpResponse,
    web
};
use tera::{Tera, Context};

use crate::utils::{get_template_2, establish_connection, feedback_form};
pub use self::{
    work_progs::*,
    blog_progs::*,
    service_progs::*,
    serve_progs::*,
    store_progs::*,
    wiki_progs::*
};
use crate::{NewUser,Feedback,NewFeedback,};
use crate::diesel::RunQueryDsl;


pub async fn index(req: HttpRequest, tera: web::Data<Tera>) -> impl Responder {
    use crate::diesel::QueryDsl;
    use crate::diesel::ExpressionMethods;
    use crate::schema::works::dsl::*;
    use crate::schema::services::dsl::*;
    use crate::schema::blogs::dsl::*;
    use crate::schema::stores::dsl::*;
    use crate::schema::wikis::dsl::*;

    use crate::models::{Work,Service,Wiki,Blog,Store};

    let _connection = establish_connection();
    let _last_works :Vec<Work> = works.filter(is_work_active.eq(true)).order(work_created.desc()).limit(3).load(&_connection).expect(".");
    let _last_services :Vec<Service> = services.filter(is_service_active.eq(true)).order(service_created.desc()).limit(3).load(&_connection).expect(".");
    let _last_wikis :Vec<Wiki> = wikis.filter(is_wiki_active.eq(true)).order(wiki_created.desc()).limit(3).load(&_connection).expect(".");
    let _last_blogs :Vec<Blog> = blogs.filter(is_blog_active.eq(true)).order(blog_created.desc()).order(blog_created.desc()).limit(3).load(&_connection).expect(".");
    let _last_stores :Vec<Store> = stores.filter(is_store_active.eq(true)).order(store_created.desc()).limit(3).load(&_connection).expect(".");

    let mut data = Context::new();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("last_works", &_last_works);
    data.insert("last_services", &_last_services);
    data.insert("last_wikis", &_last_wikis);
    data.insert("last_blogs", &_last_blogs);
    data.insert("last_stores", &_last_stores);
    data.insert("is_admin", &_is_admin);

    let _template = _type + &"main/mainpage.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}
pub async fn about(req: HttpRequest, tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);

    let _template = _type + &"about.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}
pub async fn signup(req: HttpRequest, tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);

    let _template = _type + &"signup.html".to_string();
    let rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(rendered)
}
pub async fn process_signup(data: web::Form<NewUser>) -> impl Responder {
    use crate::schema::users;
    use crate::models::User;

    let connection = establish_connection();

    diesel::insert_into(users::table)
        .values(&*data)
        .get_result::<User>(&connection)
        .expect("Error registering user.");

    println!("{:?}", data);
    HttpResponse::Ok().body(format!("Successfully saved user: {}", data.username))
}

use actix_multipart::Multipart;
pub async fn create_feedback(mut payload: Multipart) -> impl Responder {
    use schema::feedback;
    use std::borrow::BorrowMut;

    let _connection = establish_connection();
    let form = feedback_form(payload.borrow_mut()).await;
    let new_feedback = NewFeedback {
        username: form.username.clone(),
        email: form.email.clone(),
        message: form.message.clone()
    };
    let _new_feedback = diesel::insert_into(feedback::table)
        .values(&new_feedback)
        .get_result::<Feedback>(&_connection)
        .expect("E.");
    return HttpResponse::Ok();
}
