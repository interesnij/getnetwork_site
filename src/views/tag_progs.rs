
extern crate diesel;

use actix_web::{web, HttpRequest, HttpResponse, Responder};
use tera::{Tera, Context};
use actix_multipart::Multipart;
use std::borrow::BorrowMut;
use diesel::prelude::*;
use crate::utils::{
    category_form,
    get_template_2,
    establish_connection
};
use crate::schema;
use crate::models::{
    Tag,
    NewTag,
    TagItems,
};

pub async fn create_tag_page(req: HttpRequest, tera: web::Data<Tera>) -> impl Responder {
    use schema::tags::dsl::tags;

    let mut data = Context::new();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);
    let _connection = establish_connection();
    let all_tags :Vec<Tag> = tags.load(&_connection).expect("Error.");

    data.insert("all_tags", &all_tags);
    let _template = _type + &"tags/create_tag.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn create_tag(mut payload: Multipart) -> impl Responder {
    use schema::tags;

    let _connection = establish_connection();
    let form = category_form(payload.borrow_mut()).await;
    let new_tag = NewTag {
        name: form.name.clone(),
        tag_position: form.position.clone(),
        tag_count: 0,
        blog_count: 0,
        service_count: 0,
        store_count: 0,
        wiki_count: 0,
        work_count: 0,
    };
    let _new_tag = diesel::insert_into(tags::table)
        .values(&new_tag)
        .get_result::<Tag>(&_connection)
        .expect("Error saving tag.");
    return HttpResponse::Ok();
}

pub async fn tag_page(req: HttpRequest, tera: web::Data<Tera>, _id: web::Path<i32>) -> impl Responder {
    use schema::tags::dsl::tags;
    use crate::schema::tags_items::dsl::tags_items;
    use diesel::pg::expression::dsl::any;
    use crate::models::{Work, Blog, Service, Store, Wiki};

    let _connection = establish_connection();
    let _tag_id : i32 = *_id;
    let _tag = tags.filter(schema::tags::id.eq(_tag_id)).load::<Tag>(&_connection).expect("E");

    let _tag_items = tags_items.filter(schema::tags_items::tag_id.eq(&_tag_id)).load::<TagItems>(&_connection).expect("E");
    let mut blog_stack = Vec::new();
    let mut service_stack = Vec::new();
    let mut store_stack = Vec::new();
    let mut wiki_stack = Vec::new();
    let mut work_stack = Vec::new();
    for _tag_item in _tag_items.iter() {
        if _tag_item.blog_id > 0 {
            blog_stack.push(_tag_item.blog_id);
        } else if _tag_item.service_id > 0 {
            service_stack.push(_tag_item.service_id);
        } else if _tag_item.store_id > 0 {
            store_stack.push(_tag_item.store_id);
        } else if _tag_item.wiki_id > 0 {
            wiki_stack.push(_tag_item.wiki_id);
        } else if _tag_item.work_id > 0 {
            work_stack.push(_tag_item.work_id);
        }
    };

    let _blogs = schema::blogs::table
        .filter(schema::blogs::id.eq(any(blog_stack)))
        .order(schema::blogs::blog_created.desc())
        .limit(3)
        .load::<Blog>(&_connection)
        .expect("e");
    let _services = schema::services::table
        .filter(schema::services::id.eq(any(service_stack)))
        .order(schema::services::service_created.desc())
        .limit(3)
        .load::<Service>(&_connection)
        .expect("e");
    let _stores = schema::stores::table
        .filter(schema::stores::id.eq(any(store_stack)))
        .order(schema::stores::store_created.desc())
        .limit(3)
        .load::<Store>(&_connection)
        .expect("e");
    let _wikis = schema::wikis::table
        .filter(schema::wikis::id.eq(any(wiki_stack)))
        .order(schema::wikis::wiki_created.desc())
        .limit(3)
        .load::<Wiki>(&_connection)
        .expect("e");
    let _works = schema::works::table
        .filter(schema::works::id.eq(any(work_stack)))
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
    data.insert("tag", &_tag[0]);
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

    let _template = _type + &"tags/tag.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn tag_blogs_page(req: HttpRequest, tera: web::Data<Tera>, _id: web::Path<i32>) -> impl Responder {
    use schema::tags::dsl::tags;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::blogs::dsl::blogs;
    use diesel::pg::expression::dsl::any;
    use crate::models::Blog;

    let _connection = establish_connection();
    let _tag_id : i32 = *_id;
    let _tag = tags.filter(schema::tags::id.eq(_tag_id)).load::<Tag>(&_connection).expect("E");

    let _tag_items = tags_items.filter(schema::tags_items::tag_id.eq(&_tag_id)).load::<TagItems>(&_connection).expect("E");
    let mut blog_stack = Vec::new();
    for _tag_item in _tag_items.iter() {
        if _tag_item.blog_id > 0 {
            blog_stack.push(_tag_item.blog_id);
        }
    };

    let _blogs = blogs
        .filter(schema::blogs::id.eq(any(blog_stack)))
        .order(schema::blogs::blog_created.desc())
        .load::<Blog>(&_connection)
        .expect("e");

    let mut data = Context::new();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("tag", &_tag[0]);
    data.insert("blogs", &_blogs);
    data.insert("blogs_count", &_blogs.len());
    data.insert("is_admin", &_is_admin);

    let _template = _type + &"tags/tag_blogs.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn tag_services_page(req: HttpRequest, tera: web::Data<Tera>, _id: web::Path<i32>) -> impl Responder {
    use schema::tags::dsl::tags;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::services::dsl::services;
    use diesel::pg::expression::dsl::any;
    use crate::models::Service;

    let _connection = establish_connection();
    let _tag_id : i32 = *_id;
    let _tag = tags.filter(schema::tags::id.eq(_tag_id)).load::<Tag>(&_connection).expect("E");

    let _tag_items = tags_items.filter(schema::tags_items::tag_id.eq(&_tag_id)).load::<TagItems>(&_connection).expect("E");
    let mut service_stack = Vec::new();
    for _tag_item in _tag_items.iter() {
        if _tag_item.service_id > 0 {
            service_stack.push(_tag_item.service_id);
        }
    };

    let _services = services
        .filter(schema::services::id.eq(any(service_stack)))
        .order(schema::services::service_created.desc())
        .load::<Service>(&_connection)
        .expect("e");

    let mut data = Context::new();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("tag", &_tag[0]);
    data.insert("services", &_services);
    data.insert("services_count", &_services.len());
    data.insert("is_admin", &_is_admin);

    let _template = _type + &"tags/tag_services.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn tag_stores_page(req: HttpRequest, tera: web::Data<Tera>, _id: web::Path<i32>) -> impl Responder {
    use schema::tags::dsl::tags;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::stores::dsl::stores;
    use diesel::pg::expression::dsl::any;
    use crate::models::Store;

    let _connection = establish_connection();
    let _tag_id : i32 = *_id;
    let _tag = tags.filter(schema::tags::id.eq(_tag_id)).load::<Tag>(&_connection).expect("E");

    let _tag_items = tags_items.filter(schema::tags_items::tag_id.eq(&_tag_id)).load::<TagItems>(&_connection).expect("E");
    let mut store_stack = Vec::new();
    for _tag_item in _tag_items.iter() {
        if _tag_item.store_id > 0 {
            store_stack.push(_tag_item.store_id);
        }
    };

    let _stores = stores
        .filter(schema::stores::id.eq(any(store_stack)))
        .order(schema::schema::stores::store_created.desc())
        .load::<Store>(&_connection)
        .expect("e");

    let mut data = Context::new();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("tag", &_tag[0]);
    data.insert("stores", &_stores);
    data.insert("stores_count", &_stores.len());
    data.insert("is_admin", &_is_admin);

    let _template = _type + &"tags/tag_stores.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn tag_wikis_page(req: HttpRequest, tera: web::Data<Tera>, _id: web::Path<i32>) -> impl Responder {
    use schema::tags::dsl::tags;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::wikis::dsl::wikis;
    use diesel::pg::expression::dsl::any;
    use crate::models::Wiki;

    let _connection = establish_connection();
    let _tag_id : i32 = *_id;
    let _tag = tags.filter(schema::tags::id.eq(_tag_id)).load::<Tag>(&_connection).expect("E");

    let _tag_items = tags_items.filter(schema::tags_items::tag_id.eq(&_tag_id)).load::<TagItems>(&_connection).expect("E");
    let mut wiki_stack = Vec::new();
    for _tag_item in _tag_items.iter() {
        if _tag_item.wiki_id > 0 {
            wiki_stack.push(_tag_item.wiki_id);
        }
    };

    let _wikis = wikis
        .filter(schema::wikis::id.eq(any(wiki_stack)))
        .order(schema::wikis::wiki_created.desc())
        .load::<Wiki>(&_connection)
        .expect("e");

    let mut data = Context::new();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("tag", &_tag[0]);
    data.insert("wikis", &_wikis);
    data.insert("wikis_count", &_wikis.len());
    data.insert("is_admin", &_is_admin);

    let _template = _type + &"tags/tag_wikis.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn tag_works_page(req: HttpRequest, tera: web::Data<Tera>, _id: web::Path<i32>) -> impl Responder {
    use schema::tags::dsl::tags;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::works::dsl::works;
    use diesel::pg::expression::dsl::any;
    use crate::models::Work;

    let _connection = establish_connection();
    let _tag_id : i32 = *_id;
    let _tag = tags.filter(schema::tags::id.eq(_tag_id)).load::<Tag>(&_connection).expect("E");

    let _tag_items = tags_items.filter(schema::tags_items::tag_id.eq(&_tag_id)).load::<TagItems>(&_connection).expect("E");
    let mut work_stack = Vec::new();
    for _tag_item in _tag_items.iter() {
        if _tag_item.work_id > 0 {
            work_stack.push(_tag_item.work_id);
        }
    };

    let _works = works
        .filter(schema::works::id.eq(any(work_stack)))
        .order(schema::works::work_created.desc())
        .load::<Work>(&_connection)
        .expect("e");

    let mut data = Context::new();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("tag", &_tag[0]);
    data.insert("works", &_works);
    data.insert("works_count", &_works.len());
    data.insert("is_admin", &_is_admin);

    let _template = _type + &"tags/tag_works.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn tags_page(req: HttpRequest, tera: web::Data<Tera>) -> impl Responder {
    use schema::tags::dsl::*;

    let mut data = Context::new();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);
    let _connection = establish_connection();
    let all_tags :Vec<Tag> = tags.order(tag_count.desc()).load(&_connection).expect("Error.");

    data.insert("tags", &all_tags);
    data.insert("tags_count", &all_tags.len());
    let _template = _type + &"tags/tags.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn edit_tag_page(req: HttpRequest, tera: web::Data<Tera>, _id: web::Path<i32>) -> impl Responder {
    use schema::tags::dsl::*;

    let _tag_id : i32 = *_id;
    let mut data = Context::new();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);
    let _connection = establish_connection();
    let _tag = tags.filter(schema::tags::id.eq(&_tag_id)).load::<Tag>(&_connection).expect("E");

    data.insert("tag", &_tag[0]);
    let _template = _type + &"tags/edit_tag.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn edit_tag(mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::models::EditTag;
    use crate::schema::tags::dsl::tags;

    let _connection = establish_connection();
    let _tag_id : i32 = *_id;
    let _tag = tags.filter(schema::tags::id.eq(_tag_id)).load::<Tag>(&_connection).expect("E");

    let form = category_form(payload.borrow_mut()).await;
    let _new_tag = EditTag {
        name: form.name.clone(),
        tag_position: form.position.clone(),
    };

    diesel::update(&_tag[0])
        .set(_new_tag)
        .get_result::<Tag>(&_connection)
        .expect("E");
    HttpResponse::Ok()
}

pub async fn delete_tag(_id: web::Path<i32>) -> impl Responder {
    use crate::schema::tags::dsl::tags;
    use crate::schema::tags_items::dsl::tags_items;

    let _connection = establish_connection();
    let _tag_id : i32 = *_id;
    let _tag = tags.filter(schema::tags::id.eq(_tag_id)).load::<Tag>(&_connection).expect("E");
    diesel::delete(tags_items.filter(schema::tags_items::tag_id.eq(_tag_id))).execute(&_connection).expect("E");
    diesel::delete(tags.filter(schema::tags::id.eq(_tag_id))).execute(&_connection).expect("E");
    HttpResponse::Ok()
}
