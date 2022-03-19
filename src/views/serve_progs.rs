extern crate diesel;

use actix_web::{web, HttpRequest, HttpResponse, Responder};
use tera::{Tera, Context};
use std::borrow::BorrowMut;
use diesel::prelude::*;
use crate::utils::{
    category_split_payload,
    get_template_2,
    establish_connection
};
use crate::schema;
use crate::models::{
    ServeCategories,
    NewServeCategories,
    Serve,
    NewServe,
};
use actix_multipart::{Field, Multipart};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::str;


pub async fn serve_categories_page(req: HttpRequest, tera: web::Data<Tera>) -> impl Responder {
    use crate::schema::serve::dsl::serve;
    use crate::schema::serve_categories::dsl::serve_categories;

    let _connection = establish_connection();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    let mut data = Context::new();
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);

    let mut _count: i32 = 0;
    let _serve_cats :Vec<ServeCategories> = serve_categories.load(&_connection).expect("E");
    for _cat in _serve_cats.iter() {
        _count += 1;
        // для генерации переменной 1 2 3
        let mut _let_int : String = _count.to_string().parse().unwrap();
        let _serves :Vec<Serve> = serve.filter(schema::serve::serve_categories.eq(&_cat.id)).load(&_connection).expect("E");
        let _let_data_serves: String = "serves".to_string() + &_let_int;
        data.insert(&_let_data_serves, &_serves);
    };

    let _template = _type + &"serve/categories.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn get_serve_page(req: HttpRequest, tera: web::Data<Tera>, _id: web::Path<i32>) -> impl Responder {
    use schema::serve::dsl::serve;
    use schema::serve_categories::dsl::serve_categories;

    let _connection = establish_connection();
    let _serve_id : i32 = *_id;

    let _serve = serve.filter(schema::serve::id.eq(&_serve_id)).load::<Serve>(&_connection).expect("E");
    let _s_category = serve_categories.filter(schema::serve_categories::id.eq(&_serve[0].serve_categories)).load::<ServeCategories>(&_connection).expect("E");

    let mut data = Context::new();

    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("object", &_serve[0]);
    data.insert("category", &_s_category[0]);
    data.insert("is_admin", &_is_admin);

    let _template = _type + &"serve/serve.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn create_serve_categories_page(req: HttpRequest, tera: web::Data<Tera>) -> impl Responder {
    use schema::serve_categories::dsl::serve_categories;
    let mut data = Context::new();

    let _connection = establish_connection();
    let _categories = serve_categories.load::<ServeCategories>(&_connection).expect("E");
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("categories", &_categories);
    data.insert("is_admin", &_is_admin);
    let _template = _type + &"serve/create_categories.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}
pub async fn create_serve_page(req: HttpRequest, tera: web::Data<Tera>) -> impl Responder {
    use schema::serve_categories::dsl::serve_categories;

    let _connection = establish_connection();
    let _categories = serve_categories.load::<ServeCategories>(&_connection).expect("E");

    let mut data = Context::new();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);
    data.insert("categories", &_categories);

    let _template = _type + &"serve/create_serve.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn edit_serve_category_page(req: HttpRequest, tera: web::Data<Tera>, _id: web::Path<i32>) -> impl Responder {
    use schema::serve_categories::dsl::*;

    let _cat_id : i32 = *_id;
    let mut data = Context::new();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);
    let _connection = establish_connection();
    let _category = serve_categories.filter(schema::serve_categories::id.eq(&_cat_id)).load::<ServeCategories>(&_connection).expect("E");

    data.insert("category", &_category[0]);
    let _template = _type + &"serve/edit_category.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn edit_serve_page(req: HttpRequest, tera: web::Data<Tera>, _id: web::Path<i32>) -> impl Responder {
    use schema::serve::dsl::*;
    use schema::serve_categories::dsl::serve_categories;

    let _serve_id : i32 = *_id;
    let mut data = Context::new();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);
    let _connection = establish_connection();
    let _serve = serve.filter(schema::serve::id.eq(&_serve_id)).load::<Serve>(&_connection).expect("E");
    let _cat_id : i32 = _serve[0].serve_categories;
    let _s_category = serve_categories.filter(schema::serve_categories::id.eq(_cat_id)).load::<ServeCategories>(&_connection).expect("E");
    let _serve_cats :Vec<ServeCategories> = serve_categories.load(&_connection).expect("E");

    data.insert("serve", &_serve[0]);
    data.insert("category", &_s_category[0]);
    data.insert("serve_categories", &_serve_cats);

    let _template = _type + &"serve/edit_serve.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn create_serve_categories(mut payload: Multipart) -> impl Responder {
    use schema::serve_categories;

    let _connection = establish_connection();
    let form = category_split_payload(payload.borrow_mut()).await;
    let new_cat = NewServeCategories {
        name: form.name.clone(),
        serve_position: form.position.clone(),
        serve_count: 0
    };
    let _new_serve = diesel::insert_into(serve_categories::table)
        .values(&new_cat)
        .get_result::<ServeCategories>(&_connection)
        .expect("Error saving post.");
    return HttpResponse::Ok();
}
pub async fn edit_serve_category(mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::serve_categories::dsl::serve_categories;

    let _connection = establish_connection();
    let _cat_id : i32 = *_id;
    let _category = serve_categories.filter(schema::serve_categories::id.eq(_cat_id)).load::<ServeCategories>(&_connection).expect("E");

    let form = category_split_payload(payload.borrow_mut()).await;
    let new_cat = NewServeCategories {
        name: form.name.clone(),
        serve_position: form.position.clone(),
        serve_count: 0
    };
    diesel::update(&_category[0])
        .set(new_cat)
        .get_result::<ServeCategories>(&_connection)
        .expect("E");
    return HttpResponse::Ok();
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ServeForm {
    pub name: String,
    pub description: String,
    pub serve_position: i32,
    pub serve_categories: i32,
    pub price: i32,
    pub price_acc: i32,
    pub social_price: i32,
}

pub async fn serve_split_payload(payload: &mut Multipart) -> ServeForm {
    let mut form: ServeForm = ServeForm {
        name: "".to_string(),
        description: "".to_string(),
        serve_position: 0,
        serve_categories: 0,
        price: 0,
        price_acc: 0,
        social_price: 0,
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();

        if name == "serve_position" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.serve_position = _int;
                }
            }
        }
        else if name == "serve_categories" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.serve_categories = _int;
                }
            }
        }
        else if name == "price" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.price = _int;
                }
            }
        }
        else if name == "price_acc" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.price_acc = _int;
                }
            }
        }
        else if name == "social_price" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.social_price = _int;
                }
            }
        }

        else {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "name" {
                        form.name = data_string
                    } else if field.name() == "description" {
                        form.description = data_string
                    };
                }
            }
        }
    }
    form
}

pub async fn create_serve(mut payload: Multipart) -> impl Responder {
    use schema::serve;
    use crate::schema::serve_categories::dsl::serve_categories;

    let _connection = establish_connection();

    let form = serve_split_payload(payload.borrow_mut()).await;
    let _new_serve = NewServe {
        name: form.name.clone(),
        description: form.description.clone(),
        serve_position: form.serve_position.clone(),
        serve_categories: form.serve_categories.clone(),
        price: form.price.clone(),
        price_acc: Some(form.price_acc.clone()),
        social_price: Some(form.social_price.clone())
    };

    let _serve = diesel::insert_into(serve::table)
        .values(&_new_serve)
        .get_result::<Serve>(&_connection)
        .expect("E.");

    let _category = serve_categories.filter(schema::serve_categories::id.eq(_serve.serve_categories)).load::<ServeCategories>(&_connection).expect("E");
    diesel::update(&_category[0])
        .set(schema::serve_categories::serve_count.eq(_category[0].serve_count + 1))
        .get_result::<ServeCategories>(&_connection)
        .expect("E.");

    return HttpResponse::Ok();
}

pub async fn edit_serve(mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::serve::dsl::serve;

    let _serve_id : i32 = *_id;
    let _connection = establish_connection();

    let _serve = serve.filter(schema::serve::id.eq(&_serve_id)).load::<Serve>(&_connection).expect("E");

    let form = serve_split_payload(payload.borrow_mut()).await;
    let _new_serve = NewServe {
        name: form.name.clone(),
        description: form.description.clone(),
        serve_position: form.serve_position.clone(),
        serve_categories: form.serve_categories.clone(),
        price: form.price.clone(),
        price_acc: Some(form.price_acc.clone()),
        social_price: Some(form.social_price.clone())
    };

    diesel::update(&_serve[0])
        .set(_new_serve)
        .get_result::<Serve>(&_connection)
        .expect("E");

    return HttpResponse::Ok();
}


pub async fn delete_serve(_id: web::Path<i32>) -> impl Responder {
    use crate::schema::serve::dsl::serve;
    use crate::schema::serve_categories::dsl::serve_categories;

    let _connection = establish_connection();
    let _serve_id : i32 = *_id;
    let _serve = serve.filter(schema::serve::id.eq(_serve_id)).load::<Serve>(&_connection).expect("E");

    let _cat_id : i32 = _serve[0].serve_categories;
    let _category = serve_categories
        .filter(schema::serve_categories::id.eq(_cat_id))
        .load::<ServeCategories>(&_connection)
        .expect("E");
    diesel::update(&_category[0])
            .set(schema::serve_categories::serve_count.eq(&_category[0].serve_count - 1))
            .get_result::<ServeCategories>(&_connection)
            .expect("Error.");

    diesel::delete(&_serve[0]).execute(&_connection).expect("E");
    HttpResponse::Ok()
}

pub async fn delete_serve_category(_id: web::Path<i32>) -> impl Responder {
    use crate::schema::serve_categories::dsl::serve_categories;

    let _connection = establish_connection();
    let _cat_id : i32 = *_id;
    let _category = serve_categories.filter(schema::serve_categories::id.eq(_cat_id)).load::<ServeCategories>(&_connection).expect("E");
    diesel::delete(serve_categories.filter(schema::serve_categories::id.eq(_cat_id))).execute(&_connection).expect("E");
    HttpResponse::Ok()
}
