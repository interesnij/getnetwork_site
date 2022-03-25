use actix_web::{
    HttpRequest,
    Responder,
    HttpResponse,
    web
};
use tera::{Tera, Context};
use serde::Deserialize;
use crate::utils::{get_template_2, establish_connection};
use crate::NewUser;
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

//use actix_multipart::Multipart;
pub async fn create_feedback(mut payload: actix_multipart::Multipart) -> impl Responder {
    use crate::schema::feedbacks;
    use std::borrow::BorrowMut;
    use crate::models::{Feedback,NewFeedback};
    use crate::utils::feedback_form;

    let _connection = establish_connection();
    let form = feedback_form(payload.borrow_mut()).await;
    let new_feedback = NewFeedback {
        username: form.username.clone(),
        email: form.email.clone(),
        message: form.message.clone()
    };
    let _new_feedback = diesel::insert_into(feedbacks::table)
        .values(&new_feedback)
        .get_result::<Feedback>(&_connection)
        .expect("E.");
    return HttpResponse::Ok();
}

pub async fn feedback_list_page(req: HttpRequest, tera: web::Data<Tera>) -> impl Responder {
    use crate::schema::feedbacks::dsl::feedbacks;
    use crate::models::Feedback;

    let _connection = establish_connection();
    let _feedbacks = feedbacks.load::<Feedback>(&_connection).expect("E");

    let mut data = Context::new();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);
    data.insert("feedback_list", &_feedbacks);

    let _template = _type + &"main/feedback_list.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn serve_list_page(req: HttpRequest, tera: web::Data<Tera>) -> impl Responder {
    use diesel::prelude::*;
    use crate::models::{Serve, TechCategories, ServeCategories};
    use crate::schema;
    use crate::schema::{
        serve::dsl::serve,
        serve_categories::dsl::serve_categories,
        tech_categories::dsl::tech_categories,
    };

    let _connection = establish_connection();
    let mut data = Context::new();

    let all_tech_categories :Vec<TechCategories> = tech_categories
        .order(schema::tech_categories::tech_position.asc())
        .load(&_connection)
        .expect("E.");
    let mut _count: i32 = 0;
    for _cat in all_tech_categories.iter() {
        _count += 1;
        let mut _let_int : String = _count.to_string().parse().unwrap();
        let _let_serve_categories: String = "serve_categories".to_string() + &_let_int;
        let __serve_categories :Vec<ServeCategories> = serve_categories
            .filter(schema::serve_categories::tech_categories.eq(_cat.id))
            .order(schema::serve_categories::serve_position.asc())
            .load(&_connection)
            .expect("E.");
        data.insert(&_let_serve_categories, &__serve_categories);

        let mut _serve_count: i32 = 0;
        for __cat in __serve_categories.iter() {
            _serve_count += 1;
            let mut _serve_int : String = _serve_count.to_string().parse().unwrap();
            let _serve_int_dooble = "_".to_string() + &_let_int;
            let _let_serves: String = _serve_int_dooble.to_owned() + &"serves".to_string() + &_serve_int;
            let __serves :Vec<Serve> = serve.filter(schema::serve::serve_categories.eq(__cat.id)).load(&_connection).expect("E.");
            data.insert(&_let_serves, &__serves);
        }
    };
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("tech_categories", &all_tech_categories);
    data.insert("is_admin", &_is_admin);

    let _template = _type + &"main/serve_list.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

#[derive(Debug, Deserialize)]
pub struct LoadParams {
    pub _object_type: String,
    pub _owner_type: String,
    pub _object_pk: i32,
    pub _owner_pk: i32,
}
pub async fn get_load_page(req: HttpRequest, tera: web::Data<Tera>) -> impl Responder {
    use crate::schema;
    use diesel::prelude::*;

    let _connection = establish_connection();
    let params = web::Query::<LoadParams>::from_query(&req.query_string()).unwrap();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    let mut data = Context::new();
    let mut _template = "".to_string();
    let _object_id : i32 = params._object_pk.clone();

    if params._object_type.clone() == "tech_category".to_string() {
        // тип запрашиваемого объекта "tech_category".
        // получаем объект и записываем в контекст, получаем строку шаблона
        use crate::models::TechCategories;
        use crate::schema::tech_categories::dsl::*;

        let _tech_category = tech_categories
            .filter(schema::tech_categories::id.eq(&_object_id))
            .load::<TechCategories>(&_connection)
            .expect("E");
        data.insert("object", &_tech_category[0]);
        data.insert("object_type", &"tech_category".to_string());
        _template = _type + &"load/tech_category.html".to_string();
    } else if params._object_type.clone() == "serve".to_string() {
        // тип запрашиваемого объекта - опция.
        // получаем объект и записываем в контекст, получаем строку шаблона
        use crate::models::Serve;
        use crate::schema::serve::dsl::serve;
        use diesel::pg::expression::dsl::any;
        use schema::serve_items::dsl::serve_items;

        let _serve = serve
            .filter(schema::serve::id.eq(&_object_id))
            .load::<Serve>(&_connection)
            .expect("E");
        data.insert("object", &_serve[0]);
        data.insert("object_type", &"serve".to_string());
        if params._owner_type.clone() == "service".to_string() {
            // тип объекта-владельца - услуга.
            // получаем объект и записываем в контекст, получаем строку шаблона
            use crate::models::{Service, ServeItems};
            use crate::schema::services::dsl::services;
            let _service_id : i32 = params._owner_pk.clone();
            let _service = services
                .filter(schema::services::id.eq(&_service_id))
                .load::<Service>(&_connection)
                .expect("E");
            data.insert("service", &_service[0]);
            data.insert("owner_type", &"service".to_string());

            // получаем предыдущую и следующую опцию. Как вариант.
            // Ведь можем передать и весь список опций
            let _serve_items = serve_items.filter(schema::serve_items::service_id.eq(_service_id)).load::<ServeItems>(&_connection).expect("E");
            let mut serve_stack_of_service = Vec::new();
            for _serve_item in _serve_items.iter() {
                serve_stack_of_service.push(_serve_item.serve_id);
            };
            let serve_of_service = schema::serve::table
                .filter(schema::serve::id.eq(any(serve_stack_of_service)))
                .order(schema::serve::serve_position.asc())
                .load::<Serve>(&_connection)
                .expect("E");

            for (i, item) in serve_of_service.iter().enumerate().rev() {
                if item.id == _object_id {
                    if (i + 1) != serve_of_service.len() {
                        let _prev = Some(&serve_of_service[i + 1]);
                        data.insert("prev", &_prev);
                    };
                    if i != 0 {
                        let _next = Some(&serve_of_service[i - 1]);
                        data.insert("next", &_next);
                    };
                    break;
                }
            };

        }
        _template = _type + &"load/serve.html".to_string();
    }
    data.insert("is_admin", &_is_admin);
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

#[derive(Debug, Deserialize)]
pub struct SearchParams {
    pub q: Option<String>,
}

pub async fn search_page(req: HttpRequest, tera: web::Data<Tera>) -> impl Responder {
    use crate::schema;
    use diesel::prelude::*;
    use crate::models::{Work, Blog, Service, Store, Wiki};

    let _connection = establish_connection();
    let params = web::Query::<SearchParams>::from_query(&req.query_string()).unwrap();
    let __q = Some(params.q.clone());
    let _y = "".to_string();
    let _q = __q.or(_y);

    let _blogs = schema::blogs::table
        .filter(schema::blogs::title.eq(&_q))
        .or_filter(schema::blogs::description.eq(&_q))
        .or_filter(schema::blogs::content.eq(&_q))
        .order(schema::blogs::blog_created.desc())
        .limit(3)
        .load::<Blog>(&_connection)
        .expect("e");
    let _services = schema::services::table
        .filter(schema::services::title.eq(&_q))
        .or_filter(schema::services::description.eq(&_q))
        .or_filter(schema::services::content.eq(&_q))
        .order(schema::services::service_created.desc())
        .limit(3)
        .load::<Service>(&_connection)
        .expect("e");
    let _stores = schema::stores::table
        .filter(schema::stores::title.eq(&_q))
        .or_filter(schema::stores::description.eq(&_q))
        .or_filter(schema::stores::content.eq(&_q))
        .order(schema::stores::store_created.desc())
        .limit(3)
        .load::<Store>(&_connection)
        .expect("e");
    let _wikis = schema::wikis::table
        .filter(schema::wikis::title.eq(&_q))
        .or_filter(schema::wikis::description.eq(&_q))
        .or_filter(schema::wikis::content.eq(&_q))
        .order(schema::wikis::wiki_created.desc())
        .limit(3)
        .load::<Wiki>(&_connection)
        .expect("e");
    let _works = schema::works::table
        .filter(schema::works::title.eq(&_q))
        .or_filter(schema::works::description.eq(&_q))
        .or_filter(schema::works::content.eq(&_q))
        .order(schema::works::work_created.desc())
        .limit(3)
        .load::<Work>(&_connection)
        .expect("e");

    let mut data = Context::new();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("blogs", &_blogs);
    data.insert("services", &_services);
    data.insert("stores", &_stores);
    data.insert("wikis", &_wikis);
    data.insert("works", &_works);
    data.insert("blogs_count", &_blogs.len());
    data.insert("services_count", &_services.len());
    data.insert("stores_count", &_stores.len());
    data.insert("wikis_count", &_wikis.len());
    data.insert("works_count", &_works.len());
    data.insert("is_admin", &_is_admin);

    let _template = _type + &"search/all.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}
