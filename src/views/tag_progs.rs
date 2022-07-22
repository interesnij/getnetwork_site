use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
};
use crate::models::User;
use actix_multipart::Multipart;
use std::borrow::BorrowMut;
use crate::diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use actix_session::Session;
use crate::utils::{
    category_form,
    establish_connection,
    is_signed_in,
};
use crate::schema;
use crate::models::{
    Tag,
    NewTag,
    TagItems,
};
use sailfish::TemplateOnce;


pub fn tag_routes(config: &mut web::ServiceConfig) {
    config.route("/tags/", web::get().to(tags_page));
    config.route("/tag/{id}/", web::get().to(tag_page));
    config.route("/tag_blogs/{id}/", web::get().to(tag_blogs_page));
    config.route("/tag_services/{id}/", web::get().to(tag_services_page));
    config.route("/tag_stores/{id}/", web::get().to(tag_stores_page));
    config.route("/tag_wikis/{id}/", web::get().to(tag_wikis_page));
    config.route("/tag_works/{id}/", web::get().to(tag_works_page));
    config.service(web::resource("/create_tag/")
        .route(web::get().to(create_tag_page))
        .route(web::post().to(create_tag))
    );
    config.service(web::resource("/edit_tag/{id}/")
        .route(web::get().to(edit_tag_page))
        .route(web::post().to(edit_tag))
    );
    config.route("/delete_tag/{id}/", web::get().to(delete_tag));
}

pub async fn create_tag_page(req: HttpRequest) -> impl Responder {
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
    let all_tags: Vec<Tag> = tags
        .load(&_connection)
        .expect("Error.");

    data.insert("all_tags", &all_tags);
    let _template = _type + &"tags/create_tag.html".to_string();
    let _rendered = TEMPLATES.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn create_tag(mut payload: Multipart) -> impl Responder {
    let _connection = establish_connection();
    let form = category_form(payload.borrow_mut()).await;
    let new_tag = NewTag {
        name: form.name.clone(),
        position: form.position.clone(),
        count: 0,
        blog_count: 0,
        service_count: 0,
        store_count: 0,
        wiki_count: 0,
        work_count: 0,
    };
    let _new_tag = diesel::insert_into(schema::tags::table)
        .values(&new_tag)
        .get_result::<Tag>(&_connection)
        .expect("Error saving tag.");
    return HttpResponse::Ok();
}

pub async fn tag_page(req: HttpRequest, _id: web::Path<i32>) -> impl Responder {
    use schema::tags::dsl::tags;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::blogs::dsl::blogs;
    use crate::schema::services::dsl::services;
    use crate::schema::stores::dsl::stores;
    use crate::schema::wikis::dsl::wikis;
    use crate::schema::works::dsl::works;
    use crate::models::{Work, Blog, Service, Store, Wiki};

    let _connection = establish_connection();
    let _tag_id: i32 = *_id;
    let _tag = tags
        .filter(schema::tags::id.eq(_tag_id))
        .load::<Tag>(&_connection)
        .expect("E");

    let _tag_items = tags_items
        .filter(schema::tags_items::tag_id.eq(&_tag_id))
        .load::<TagItems>(&_connection)
        .expect("E");
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

    let mut data = Context::new();

    let _blogs = schema::blogs::table
        .filter(schema::blogs::id.eq(any(&blog_stack)))
        .order(schema::blogs::created.desc())
        .limit(3)
        .load::<Blog>(&_connection)
        .expect("e");
    if _blogs.len() > 0 {
        data.insert("blogs", &_blogs);
        data.insert("blogs_count", &blogs
            .filter(schema::blogs::id.eq_any(&blog_stack))
            .load::<Blog>(&_connection)
            .expect("E")
            .len());
    }

    let _services = schema::services::table
        .filter(schema::services::id.eq(any(&service_stack)))
        .order(schema::services::created.desc())
        .limit(3)
        .load::<Service>(&_connection)
        .expect("e");
    if _services.len() > 0 {
        data.insert("services", &_services);
        data.insert("services_count", &services
            .filter(schema::services::id.eq_any(&service_stack))
            .load::<Service>(&_connection)
            .expect("E")
            .len());
    }

    let _stores = schema::stores::table
        .filter(schema::stores::id.eq_any(&store_stack))
        .order(schema::stores::created.desc())
        .limit(3)
        .load::<Store>(&_connection)
        .expect("e");
    if _stores.len() > 0 {
        data.insert("stores", &_stores);
        data.insert("stores_count", &stores
            .filter(schema::stores::id.eq_any(&store_stack))
            .load::<Store>(&_connection)
            .expect("E")
            .len());
    }

    let _wikis = schema::wikis::table
        .filter(schema::wikis::id.eq_any(&wiki_stack))
        .order(schema::wikis::created.desc())
        .limit(3)
        .load::<Wiki>(&_connection)
        .expect("e");
    if _wikis.len() > 0 {
        data.insert("wikis", &_wikis);
        data.insert("wikis_count", &wikis
            .filter(schema::wikis::id.eq_any(&wiki_stack))
            .load::<Wiki>(&_connection)
            .expect("E")
            .len());
    }

    let _works = schema::works::table
        .filter(schema::works::id.eq_any(&work_stack))
        .order(schema::works::created.desc())
        .limit(3)
        .load::<Work>(&_connection)
        .expect("e");
    if _works.len() > 0 {
        data.insert("works", &_works);
        data.insert("works_count", &works
            .filter(schema::works::id.eq_any(&work_stack))
            .load::<Work>(&_connection)
            .expect("E")
            .len());
    }

    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("tag", &_tag[0]);
    data.insert("is_admin", &_is_admin);

    let _template = _type + &"tags/tag.html".to_string();
    let _rendered = TEMPLATES.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn tag_blogs_page(req: HttpRequest, _id: web::Path<i32>) -> impl Responder {
    use schema::tags::dsl::tags;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::blogs::dsl::blogs;
    use crate::models::Blog;

    let _connection = establish_connection();
    let _tag_id : i32 = *_id;
    let page_size = 20;
    let mut offset = 0;
    let mut data = Context::new();
    let _tag = tags
        .filter(schema::tags::id.eq(_tag_id))
        .load::<Tag>(&_connection)
        .expect("E");

    let _tag_items = tags_items
        .filter(schema::tags_items::tag_id.eq(&_tag_id))
        .select(schema::tags_items::blog_id)
        .load::<i32>(&_connection)
        .expect("E");

    loop {
        let _blogs = blogs
            .filter(schema::blogs::id.eq_any(&_tag_items))
            .limit(page_size)
            .offset(offset)
            .order(schema::blogs::created.desc())
            .load::<Blog>(&_connection)
            .expect("e");
        if _blogs.len() > 0 {
            data.insert("blogs", &_blogs);
            data.insert("blogs_count", &blogs
                .filter(schema::blogs::id.eq_any(&_tag_items))
                .load::<Blog>(&_connection)
                .expect("could not load tags")
                .len());
            offset += page_size;
        }
        else {break;}
    }

    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("tag", &_tag[0]);
    data.insert("is_admin", &_is_admin);

    let _template = _type + &"tags/tag_blogs.html".to_string();
    let _rendered = TEMPLATES.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn tag_services_page(req: HttpRequest, _id: web::Path<i32>) -> impl Responder {
    use schema::tags::dsl::tags;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::services::dsl::services;
    use crate::models::Service;

    let _connection = establish_connection();
    let _tag_id : i32 = *_id;
    let page_size = 20;
    let mut offset = 0;
    let mut data = Context::new();
    let _tag = tags
        .filter(schema::tags::id.eq(_tag_id))
        .load::<Tag>(&_connection)
        .expect("E");

    let _tag_items = tags_items
        .filter(schema::tags_items::tag_id.eq(&_tag_id))
        .select(schema::tags_items::service_id)
        .load::<i32>(&_connection)
        .expect("E");
    loop {
        let _services = services
            .filter(schema::services::id.eq_any(&_tag_items))
            .limit(page_size)
            .offset(offset)
            .order(schema::services::created.desc())
            .load::<Service>(&_connection)
            .expect("e");
        if _services.len() > 0 {
            data.insert("services", &_services);
            data.insert("services_count", &services
                .filter(schema::services::id.eq(any(&_tag_items)))
                .load::<Service>(&_connection)
                .expect("could not load tags")
                .len());
            offset += page_size;
        }
        else {break;}
    }

    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("tag", &_tag[0]);
    data.insert("is_admin", &_is_admin);

    let _template = _type + &"tags/tag_services.html".to_string();
    let _rendered = TEMPLATES.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn tag_stores_page(req: HttpRequest, _id: web::Path<i32>) -> impl Responder {
    use schema::tags::dsl::tags;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::stores::dsl::stores;
    use crate::models::Store;

    let _connection = establish_connection();
    let _tag_id : i32 = *_id;
    let page_size = 20;
    let mut offset = 0;
    let mut data = Context::new();
    let _tag = tags
        .filter(schema::tags::id.eq(_tag_id))
        .load::<Tag>(&_connection)
        .expect("E");

    let _tag_items = tags_items
        .filter(schema::tags_items::tag_id.eq(&_tag_id))
        .select(schema::tags_items::store_id)
        .load::<i32>(&_connection)
        .expect("E");

    loop {
        let _stores = stores
            .filter(schema::stores::id.eq_any(&_tag_items))
            .limit(page_size)
            .offset(offset)
            .order(schema::stores::created.desc())
            .load::<Store>(&_connection)
            .expect("e");
        if _stores.len() > 0 {
            data.insert("stores", &_stores);
            data.insert("stores_count", &stores
                .filter(schema::stores::id.eq_any(&_tag_items))
                .load::<Store>(&_connection)
                .expect("could not load tags")
                .len());
            offset += page_size;
        }
        else {break;}
    }

    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("tag", &_tag[0]);
    data.insert("is_admin", &_is_admin);

    let _template = _type + &"tags/tag_stores.html".to_string();
    let _rendered = TEMPLATES.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn tag_wikis_page(req: HttpRequest, _id: web::Path<i32>) -> impl Responder {
    use schema::tags::dsl::tags;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::wikis::dsl::wikis;
    use crate::models::Wiki;

    let _connection = establish_connection();
    let _tag_id : i32 = *_id;
    let page_size = 20;
    let mut offset = 0;
    let mut data = Context::new();
    let _tag = tags
        .filter(schema::tags::id.eq(_tag_id))
        .load::<Tag>(&_connection)
        .expect("E");

    let _tag_items = tags_items
        .filter(schema::tags_items::tag_id.eq(&_tag_id))
        .select(schema::tags_items::wiki_id)
        .load::<TagItems>(&_connection)
        .expect("E");

    loop {
        let _wikis = wikis
            .filter(schema::wikis::id.eq_any(&_tag_items))
            .limit(page_size)
            .offset(offset)
            .order(schema::wikis::created.desc())
            .load::<Wiki>(&_connection)
            .expect("e");
        if _wikis.len() > 0 {
            data.insert("wikis", &_wikis);
            data.insert("wikis_count", &wikis
                .filter(schema::wikis::id.eq_any(&_tag_items))
                .load::<Wiki>(&_connection)
                .expect("could not load tags")
                .len());
            offset += page_size;
        }
        else {break;}
    }

    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("tag", &_tag[0]);
    data.insert("is_admin", &_is_admin);

    let _template = _type + &"tags/tag_wikis.html".to_string();
    let _rendered = TEMPLATES.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn tag_works_page(req: HttpRequest, _id: web::Path<i32>) -> impl Responder {
    use schema::tags::dsl::tags;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::works::dsl::works;
    use crate::models::Work;

    let _connection = establish_connection();
    let _tag_id : i32 = *_id;
    let page_size = 20;
    let mut offset = 0;
    let mut data = Context::new();
    let _tag = tags
        .filter(schema::tags::id.eq(_tag_id))
        .load::<Tag>(&_connection)
        .expect("E");

    let _tag_items = tags_items
        .filter(schema::tags_items::tag_id.eq(&_tag_id))
        .select(schema::tags_items::work_id)
        .load::<i32>(&_connection)
        .expect("E");

    loop {
        let _works = works
            .filter(schema::works::id.eq_any(&_tag_items))
            .limit(page_size)
            .offset(offset)
            .order(schema::works::created.desc())
            .load::<Work>(&_connection)
            .expect("e");
        if _works.len() > 0 {
            data.insert("works", &_works);
            data.insert("works_count", &works
                .filter(schema::works::id.eq_any(&_tag_items))
                .load::<Work>(&_connection)
                .expect("could not load tags")
                .len());
            offset += page_size;
        }
        else {break;}
    }

    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("tag", &_tag[0]);
    data.insert("is_admin", &_is_admin);

    let _template = _type + &"tags/tag_works.html".to_string();
    let _rendered = TEMPLATES.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn tags_page(req: HttpRequest) -> impl Responder {
    use schema::tags::dsl::tags;

    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);
    let _connection = establish_connection();
    let all_tags: Vec<Tag> = tags
        .order(tag_count.desc())
        .load(&_connection)
        .expect("Error.");

    data.insert("tags", &all_tags);
    data.insert("tags_count", &all_tags.len());
    let _template = _type + &"tags/tags.html".to_string();
    let _rendered = TEMPLATES.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn edit_tag_page(req: HttpRequest, _id: web::Path<i32>) -> impl Responder {
    use schema::tags::dsl::*;

    let _tag_id: i32 = *_id;

    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);
    let _connection = establish_connection();
    let _tag = tags
        .filter(schema::tags::id.eq(&_tag_id))
        .load::<Tag>(&_connection)
        .expect("E");

    data.insert("tag", &_tag[0]);
    let _template = _type + &"tags/edit_tag.html".to_string();
    let _rendered = TEMPLATES.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn edit_tag(mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::models::EditTag;
    use crate::schema::tags::dsl::tags;

    let _connection = establish_connection();
    let _tag_id : i32 = *_id;
    let _tag = tags
        .filter(schema::tags::id.eq(_tag_id))
        .load::<Tag>(&_connection)
        .expect("E");

    let form = category_form(payload.borrow_mut()).await;
    let _new_tag = EditTag {
        name:     form.name.clone(),
        position: form.position,
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
    let _tag_id: i32 = *_id;
    let _tag = tags
        .filter(schema::tags::id.eq(_tag_id))
        .load::<Tag>(&_connection)
        .expect("E");
    diesel::delete(tags_items.filter(schema::tags_items::tag_id.eq(_tag_id))).execute(&_connection).expect("E");
    diesel::delete(tags.filter(schema::tags::id.eq(_tag_id))).execute(&_connection).expect("E");
    HttpResponse::Ok()
}
