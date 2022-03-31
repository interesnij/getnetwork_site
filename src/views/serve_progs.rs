extern crate diesel;

use actix_web::{web, HttpRequest, HttpResponse, Responder};
use tera::{Tera, Context};
use std::borrow::BorrowMut;
use diesel::prelude::*;
use crate::utils::{
    category_form,
    serve_category_form,
    get_template_2,
    establish_connection
};
use crate::schema;
use crate::models::{
    ServeCategories,
    NewServeCategories,
    Serve,
    NewServe,
    TechCategories,
    NewTechCategories,
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
    let _serve_cats :Vec<ServeCategories> = serve_categories.load(&_connection).expect("E");

    let mut data = Context::new();
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("serve_categories", &_serve_cats);
    data.insert("is_admin", &_is_admin);

    let mut _count: i32 = 0;
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

pub async fn create_tech_categories_page(req: HttpRequest, tera: web::Data<Tera>) -> impl Responder {
    use schema::tech_categories::dsl::tech_categories;
    let mut data = Context::new();

    let _connection = establish_connection();
    let _categories = tech_categories.load::<TechCategories>(&_connection).expect("E");
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("categories", &_categories);
    data.insert("is_admin", &_is_admin);
    let _template = _type + &"serve/create_tech_categories.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}
pub async fn create_serve_categories_page(req: HttpRequest, tera: web::Data<Tera>) -> impl Responder {
    use schema::serve_categories::dsl::serve_categories;
    use schema::tech_categories::dsl::tech_categories;

    let mut data = Context::new();
    let _connection = establish_connection();

    let _tech_categories = tech_categories.load::<TechCategories>(&_connection).expect("E");
    let _categories = serve_categories.load::<ServeCategories>(&_connection).expect("E");
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);

    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("categories", &_categories);
    data.insert("tech_categories", &_tech_categories);
    data.insert("is_admin", &_is_admin);
    let _template = _type + &"serve/create_serve_categories.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}
pub async fn create_serve_page(req: HttpRequest, tera: web::Data<Tera>) -> impl Responder {
    use crate::schema::{
        serve::dsl::serve,
        serve_categories::dsl::serve_categories,
        tech_categories::dsl::tech_categories,
    };

    let _connection = establish_connection();
    let _categories = serve_categories.load::<ServeCategories>(&_connection).expect("E");
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
    data.insert("categories", &_categories);

    let _template = _type + &"serve/create_serve.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn edit_tech_category_page(req: HttpRequest, tera: web::Data<Tera>, _id: web::Path<i32>) -> impl Responder {
    use schema::tech_categories::dsl::*;

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
    let _category = tech_categories.filter(schema::tech_categories::id.eq(&_cat_id)).load::<TechCategories>(&_connection).expect("E");

    data.insert("category", &_category[0]);
    let _template = _type + &"serve/edit_tech_category.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn edit_serve_category_page(req: HttpRequest, tera: web::Data<Tera>, _id: web::Path<i32>) -> impl Responder {
    use schema::serve_categories::dsl::*;
    use schema::tech_categories::dsl::tech_categories;

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
    let _t_categories = tech_categories.load::<TechCategories>(&_connection).expect("E");

    data.insert("category", &_category[0]);
    data.insert("tech_categories", &_t_categories);
    let _template = _type + &"serve/edit_serve_category.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn edit_serve_page(req: HttpRequest, tera: web::Data<Tera>, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::{
        serve::dsl::serve,
        serve_categories::dsl::serve_categories,
        tech_categories::dsl::tech_categories,
    };

    let _connection = establish_connection();
    let mut _count: i32 = 0;
    let mut data = Context::new();

    let all_tech_categories :Vec<TechCategories> = tech_categories
        .order(schema::tech_categories::tech_position.asc())
        .load(&_connection)
        .expect("E.");

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

    let _serve_id : i32 = *_id;

    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);
    data.insert("tech_categories", &all_tech_categories);

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

pub async fn create_tech_categories(mut payload: Multipart) -> impl Responder {
    use schema::tech_categories;

    let _connection = establish_connection();
    let form = category_form(payload.borrow_mut()).await;
    let new_cat = NewTechCategories {
        name: form.name.clone(),
        description: Some(form.description.clone()),
        tech_position: form.position.clone(),
        tech_count: 0,
    };
    let _new_tech = diesel::insert_into(tech_categories::table)
        .values(&new_cat)
        .get_result::<TechCategories>(&_connection)
        .expect("E.");
    return HttpResponse::Ok();
}

pub async fn create_serve_categories(mut payload: Multipart) -> impl Responder {
    use schema::serve_categories;
    use schema::tech_categories::dsl::tech_categories;

    let _connection = establish_connection();
    let form = serve_category_form(payload.borrow_mut()).await;
    let _s_category = tech_categories.filter(schema::tech_categories::id.eq(form.tech_categories.clone())).load::<TechCategories>(&_connection).expect("E");

    let new_cat = NewServeCategories {
        name: form.name.clone(),
        description: Some(form.description.clone()),
        cat_name: _s_category[0].name.clone(),
        tech_categories: form.tech_categories.clone(),
        serve_position: form.position.clone(),
        serve_count: 0
    };
    let _new_serve = diesel::insert_into(serve_categories::table)
        .values(&new_cat)
        .get_result::<ServeCategories>(&_connection)
        .expect("Error saving post.");
    return HttpResponse::Ok();
}
pub async fn edit_tech_category(mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::tech_categories::dsl::tech_categories;

    let _connection = establish_connection();
    let _cat_id : i32 = *_id;
    let _category = tech_categories.filter(schema::tech_categories::id.eq(_cat_id)).load::<TechCategories>(&_connection).expect("E");

    let form = category_form(payload.borrow_mut()).await;
    let new_cat = NewTechCategories {
        name: form.name.clone(),
        description: Some(form.description.clone()),
        tech_position: form.position.clone(),
        tech_count: _category[0].tech_count,
    };
    diesel::update(&_category[0])
        .set(new_cat)
        .get_result::<TechCategories>(&_connection)
        .expect("E");
    return HttpResponse::Ok();
}

pub async fn edit_serve_category(mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::serve_categories::dsl::serve_categories;
    use crate::schema::tech_categories::dsl::tech_categories;

    let _connection = establish_connection();
    let _cat_id : i32 = *_id;
    let s_category = serve_categories.filter(schema::serve_categories::id.eq(_cat_id)).load::<ServeCategories>(&_connection).expect("E");
    let t_category = tech_categories.filter(schema::tech_categories::id.eq(s_category[0].tech_categories)).load::<TechCategories>(&_connection).expect("E");

    let form = serve_category_form(payload.borrow_mut()).await;
    let new_cat = NewServeCategories {
        name: form.name.clone(),
        description: Some(form.description.clone()),
        cat_name: t_category[0].name.clone(),
        tech_categories: form.tech_categories.clone(),
        serve_position: form.position.clone(),
        serve_count: s_category[0].serve_count.clone(),
    };
    diesel::update(&s_category[0])
        .set(new_cat)
        .get_result::<ServeCategories>(&_connection)
        .expect("E");
    return HttpResponse::Ok();
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ServeForm {
    pub name: String,
    pub cat_name: String,
    pub description: String,
    pub serve_position: i32,
    pub serve_categories: i32,
    pub price: i32,
    pub price_acc: i32,
    pub social_price: i32,
    pub man_hours: i32,
    pub is_default: bool,
}

pub async fn serve_split_payload(payload: &mut Multipart) -> ServeForm {
    let mut form: ServeForm = ServeForm {
        name: "".to_string(),
        cat_name: "".to_string(),
        description: "".to_string(),
        serve_position: 0,
        serve_categories: 0,
        price: 0,
        price_acc: 0,
        social_price: 0,
        man_hours: 0,
        is_default: true,
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
        else if name == "man_hours" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.man_hours = _int;
                }
            }
        }
        else if name == "is_default" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    if s.to_string() == "on" {
                        form.is_default = true;
                    } else {
                        form.is_default = false;
                    }
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
                    } else if field.name() == "cat_name" {
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
    let _cat_id = form.serve_categories.clone();
    let _category = serve_categories.filter(schema::serve_categories::id.eq(_cat_id)).load::<ServeCategories>(&_connection).expect("E");

    let mut is_default = false;
    if form.is_default.clone() == true {
        is_default = true;
    };
    let _new_serve = NewServe {
        name: form.name.clone(),
        cat_name: _category[0].name.clone(),
        description: form.description.clone(),
        serve_position: form.serve_position.clone(),
        serve_categories: _cat_id,
        price: form.price.clone(),
        price_acc: Some(form.price_acc.clone()),
        social_price: Some(form.social_price.clone()),
        man_hours: form.man_hours.clone(),
        is_default: is_default,
    };

    let _serve = diesel::insert_into(serve::table)
        .values(&_new_serve)
        .get_result::<Serve>(&_connection)
        .expect("E.");

    diesel::update(&_category[0])
        .set(schema::serve_categories::serve_count.eq(_category[0].serve_count + 1))
        .get_result::<ServeCategories>(&_connection)
        .expect("E.");

    return HttpResponse::Ok();
}

pub async fn edit_serve(mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::serve::dsl::serve;
    use crate::schema::serve_categories::dsl::serve_categories;

    let _serve_id : i32 = *_id;
    let _connection = establish_connection();

    let _serve = serve.filter(schema::serve::id.eq(&_serve_id)).load::<Serve>(&_connection).expect("E");
    let _category = serve_categories.filter(schema::serve_categories::id.eq(_serve[0].serve_categories)).load::<ServeCategories>(&_connection).expect("E");
    let form = serve_split_payload(payload.borrow_mut()).await;

    let mut is_default = false;
    if form.is_default.clone() == true {
        is_default = true;
    };
    let _new_serve = NewServe {
        name: form.name.clone(),
        cat_name: _category[0].name.clone(),
        description: form.description.clone(),
        serve_position: form.serve_position.clone(),
        serve_categories: form.serve_categories.clone(),
        price: form.price.clone(),
        price_acc: Some(form.price_acc.clone()),
        social_price: Some(form.social_price.clone()),
        man_hours: form.man_hours.clone(),
        is_default: is_default,
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

pub async fn delete_tech_category(_id: web::Path<i32>) -> impl Responder {
    use crate::schema::tech_categories::dsl::tech_categories;

    let _connection = establish_connection();
    let _cat_id : i32 = *_id;
    let _category = tech_categories.filter(schema::tech_categories::id.eq(_cat_id)).load::<TechCategories>(&_connection).expect("E");
    diesel::delete(tech_categories.filter(schema::tech_categories::id.eq(_cat_id))).execute(&_connection).expect("E");
    HttpResponse::Ok()
}
pub async fn delete_serve_category(_id: web::Path<i32>) -> impl Responder {
    use crate::schema::serve_categories::dsl::serve_categories;
    use crate::schema::tech_categories::dsl::tech_categories;

    let _connection = establish_connection();
    let _cat_id : i32 = *_id;
    let _category = serve_categories.filter(schema::serve_categories::id.eq(_cat_id)).load::<ServeCategories>(&_connection).expect("E");
    diesel::delete(serve_categories.filter(schema::serve_categories::id.eq(_cat_id))).execute(&_connection).expect("E");

    let _category = tech_categories
        .filter(schema::tech_categories::id.eq(_cat_id))
        .load::<TechCategories>(&_connection)
        .expect("E");
    diesel::update(&_category[0])
            .set(schema::tech_categories::tech_count.eq(&_category[0].tech_count - 1))
            .get_result::<TechCategories>(&_connection)
            .expect("E");
    HttpResponse::Ok()
}
