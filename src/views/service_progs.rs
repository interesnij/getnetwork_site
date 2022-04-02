extern crate diesel;

use actix_web::{web, HttpRequest, HttpResponse, Responder};
use tera::{Tera, Context};
use actix_multipart::Multipart;
use std::borrow::BorrowMut;
use diesel::prelude::*;
use crate::utils::{
    store_form,
    category_form,
    get_template_2,
    establish_connection
};
use crate::schema;
use crate::models::{
    ServiceCategories,
    NewServiceCategories,
    Service,
    NewService,
    ServiceCategory,
    NewServiceCategory,
    ServiceImage,
    NewServiceImage,
    ServiceVideo,
    NewServiceVideo,
    TagItems,
    NewTagItems,
    Tag,
    Serve,
    ServeCategories,
    TechCategories,
    NewServeItems,
    ServeItems,
};

fn get_cats_for_service(service: &Service) -> (Vec<ServiceCategories>, Vec<String>) {
    use diesel::pg::expression::dsl::any;
    let _connection = establish_connection();

    let ids = ServiceCategory::belonging_to(service).select(schema::service_category::service_categories_id);
    let categories = schema::service_categories::table
        .filter(schema::service_categories::id.eq(any(ids)))
        .load::<ServiceCategories>(&_connection)
        .expect("E");
    let mut categories_names = Vec::new();
    for _cat in categories.iter() {
        categories_names.push(_cat.name.clone());
    }
    (categories, categories_names)
}
fn get_tags_for_service(service: &Service) -> Vec<Tag> {
    use crate::schema::tags_items::dsl::tags_items;
    use diesel::dsl::any;
    let _connection = establish_connection();

    let _tag_items = tags_items.filter(schema::tags_items::service_id.eq(&service.id)).load::<TagItems>(&_connection).expect("E");
    let mut stack = Vec::new();
    for _tag_item in _tag_items.iter() {
        stack.push(_tag_item.tag_id);
    };
    schema::tags::table
        .filter(schema::tags::id.eq(any(stack)))
        .load::<Tag>(&_connection)
        .expect("E")
}
fn get_6_service_for_category(category: &ServiceCategories) -> Vec<Service> {
    use diesel::pg::expression::dsl::any;
    let _connection = establish_connection();

    let ids = ServiceCategory::belonging_to(category).select(schema::service_category::service_id);
    schema::services::table
        .filter(schema::services::id.eq(any(ids)))
        .order(schema::services::service_created.desc())
        .limit(6)
        .load::<Service>(&_connection)
        .expect("E")
}
fn get_service_for_category(category: &ServiceCategories) -> Vec<Service> {
    use diesel::pg::expression::dsl::any;
    let _connection = establish_connection();

    let ids = ServiceCategory::belonging_to(category).select(schema::service_category::service_id);
    schema::services::table
        .filter(schema::services::id.eq(any(ids)))
        .order(schema::services::service_created.desc())
        .load::<Service>(&_connection)
        .expect("E")
}
fn get_serves_for_service(service: &Service) -> Vec<Serve> {
    use diesel::pg::expression::dsl::any;
    use schema::serve_items::dsl::serve_items;
    let _connection = establish_connection();

    let _serve_items = serve_items.filter(schema::serve_items::service_id.eq(&service.id)).load::<ServeItems>(&_connection).expect("E");
    let mut stack = Vec::new();
    for _serve_item in _serve_items.iter() {
        stack.push(_serve_item.serve_id);
    };
    schema::serve::table
        .filter(schema::serve::id.eq(any(stack)))
        .load::<Serve>(&_connection)
        .expect("E")
}

pub async fn create_service_categories_page(req: HttpRequest, tera: web::Data<Tera>) -> impl Responder {
        let mut data = Context::new();
        let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
        data.insert("service_categories", &_service_cats);
        data.insert("store_categories", &_store_cats);
        data.insert("blog_categories", &_blog_cats);
        data.insert("wiki_categories", &_wiki_cats);
        data.insert("work_categories", &_work_cats);
        data.insert("is_admin", &_is_admin);
    let _template = _type + &"services/create_categories.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn create_service_page(req: HttpRequest, tera: web::Data<Tera>) -> impl Responder {
    use schema::tags::dsl::tags;
    use schema::serve::dsl::serve;
    use schema::serve_categories::dsl::serve_categories;
    use schema::tech_categories::dsl::tech_categories;

    let mut data = Context::new();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);

    let _connection = establish_connection();
    let all_tags :Vec<Tag> = tags
        .load(&_connection)
        .expect("Error.");

    let all_tech_categories :Vec<TechCategories> = tech_categories.load(&_connection).expect("E.");

    // генерация переменных шаблона, хранящих: категории опций и опции.
    let mut _count: i32 = 0;
    for _cat in all_tech_categories.iter() {
        _count += 1;
        let mut _let_int : String = _count.to_string().parse().unwrap();
        let _let_serve_categories: String = "serve_categories".to_string() + &_let_int;
        let __serve_categories :Vec<ServeCategories> = serve_categories.filter(schema::serve_categories::tech_categories.eq(_cat.id)).load(&_connection).expect("E.");
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

    data.insert("tags", &all_tags);
    data.insert("tech_categories", &all_tech_categories);

    let _template = _type + &"services/create_service.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn create_service_categories(mut payload: Multipart) -> impl Responder {
    use schema::service_categories;

    let _connection = establish_connection();
    let form = category_form(payload.borrow_mut()).await;
    let new_cat = NewServiceCategories {
        name: form.name.clone(),
        description: Some(form.description.clone()),
        service_position: form.position.clone(),
        image: Some(form.image.clone()),
        service_count: 0
    };
    let _new_service = diesel::insert_into(service_categories::table)
        .values(&new_cat)
        .get_result::<ServiceCategories>(&_connection)
        .expect("Error saving post.");
    return HttpResponse::Ok();
}
pub async fn create_service(mut payload: Multipart) -> impl Responder {
    use schema::{services,service_images,service_videos,service_category,tags_items,serve_items};
    use crate::schema::tags::dsl::tags;
    use crate::schema::service_categories::dsl::service_categories;

    let _connection = establish_connection();

    let form = store_form(payload.borrow_mut()).await;
    let new_service = NewService::from_service_form(
        form.title.clone(),
        form.description.clone(),
        form.link.clone(),
        form.main_image.clone(),
        form.is_active.clone(),
        1
    );

    let _service = diesel::insert_into(services::table)
        .values(&new_service)
        .get_result::<Service>(&_connection)
        .expect("Error saving service.");

    for image in form.images.iter().enumerate() {
        let new_image = NewServiceImage::from_service_images_form(
            _service.id,
            image.1.to_string()
        );
        diesel::insert_into(service_images::table)
            .values(&new_image)
            .get_result::<ServiceImage>(&_connection)
            .expect("Error saving service.");
        };
    for video in form.videos.iter().enumerate() {
        let new_video = NewServiceVideo::from_service_videos_form(
            _service.id,
            video.1.to_string()
        );
        diesel::insert_into(service_videos::table)
            .values(&new_video)
            .get_result::<ServiceVideo>(&_connection)
            .expect("Error saving service.");
    };
    for category_id in form.category_list.iter().enumerate() {
        let new_category = NewServiceCategory {
            service_categories_id: *category_id.1,
            service_id: _service.id
        };
        diesel::insert_into(service_category::table)
            .values(&new_category)
            .get_result::<ServiceCategory>(&_connection)
            .expect("Error saving service.");
        let _category = service_categories.filter(schema::service_categories::id.eq(category_id.1)).load::<ServiceCategories>(&_connection).expect("E");
        diesel::update(&_category[0])
            .set(schema::service_categories::service_count.eq(_category[0].service_count + 1))
            .get_result::<ServiceCategories>(&_connection)
            .expect("Error.");
    };
    for tag_id in form.tags_list.iter().enumerate() {
        let new_tag = NewTagItems{
            tag_id: *tag_id.1,
            service_id: _service.id,
            store_id: 0,
            blog_id: 0,
            wiki_id: 0,
            work_id: 0,
            tag_created: chrono::Local::now().naive_utc(),
        };
        diesel::insert_into(tags_items::table)
            .values(&new_tag)
            .get_result::<TagItems>(&_connection)
            .expect("Error.");
        let _tag = tags.filter(schema::tags::id.eq(tag_id.1)).load::<Tag>(&_connection).expect("E");
        diesel::update(&_tag[0])
            .set((schema::tags::tag_count.eq(_tag[0].tag_count + 1), schema::tags::service_count.eq(_tag[0].service_count + 1)))
            .get_result::<Tag>(&_connection)
            .expect("Error.");
    };

    for serve_id in form.serve_list.iter().enumerate() {
        let new_serve = NewServeItems{
            serve_id: *serve_id.1,
            service_id: _service.id,
            store_id: 0,
            work_id: 0,
        };
        diesel::insert_into(serve_items::table)
            .values(&new_serve)
            .get_result::<ServeItems>(&_connection)
            .expect("Error.");
    };
    HttpResponse::Ok()
}

pub async fn get_service_page(req: HttpRequest, tera: web::Data<Tera>, param: web::Path<(i32,i32)>) -> impl Responder {
    use diesel::pg::expression::dsl::any;
    use schema::services::dsl::services;
    use schema::service_images::dsl::service_images;
    use schema::service_videos::dsl::service_videos;
    use schema::service_categories::dsl::service_categories;
    use crate::schema::{
        serve::dsl::serve,
        serve_categories::dsl::serve_categories,
        tech_categories::dsl::tech_categories,
    };

    let _connection = establish_connection();
    let _service_id : i32 = param.1;
    let _cat_id : i32 = param.0;
    let mut default_price : i32 = 0;

    let _service = services.filter(schema::services::id.eq(&_service_id)).load::<Service>(&_connection).expect("E");
    let _s_category = service_categories
        .filter(schema::service_categories::id.eq(&_cat_id))
        .load::<ServiceCategories>(&_connection)
        .expect("E");

    let mut data = Context::new();

    let _category_services = get_service_for_category(&_s_category[0]);
    let _category_services_len : usize = _category_services.len();
    for (i, item) in _category_services.iter().enumerate().rev() {
        if item.id == _service_id {
            if (i + 1) != _category_services_len {
                let _prev = Some(&_category_services[i + 1]);
                data.insert("prev", &_prev);
            };
            if i != 0 {
                let _next = Some(&_category_services[i - 1]);
                data.insert("next", &_next);
            };
            break;
        }
    };

    let _images :Vec<ServiceImage> = service_images.filter(schema::service_images::service.eq(&_service_id)).load(&_connection).expect("E");
    let _videos :Vec<ServiceVideo> = service_videos.filter(schema::service_videos::service.eq(&_service_id)).load(&_connection).expect("E");
    let (_categories, _categories_names) = get_cats_for_service(&_service[0]);
    let _tags = get_tags_for_service(&_service[0]);

    // нам надо показать выбор опций только в нужном диапазоне.
    // поэтому мы по выбранным для объекта сервиса опциям получаем
    // категории опций, а уже по тем - тех. категории.
    // ИТАК: 1. получаем опции
    let __serves = get_serves_for_service(&_service[0]);
    // 2. получаем категории опций, исключая дубли.
    let mut serve_categories_ids = Vec::new();
    let mut serve_ids = Vec::new();
    for _serve in __serves.iter() {
        serve_ids.push(_serve.id);
        if serve_categories_ids.iter().any(|&i| i==_serve.serve_categories) {
            continue;
        } else {
            serve_categories_ids.push(_serve.serve_categories);
        }
    };
    let __serve_categories = serve_categories
        .filter(schema::serve_categories::id.eq(any(&serve_categories_ids)))
        .load::<ServeCategories>(&_connection)
        .expect("E");

    // 3. получаем технические категории, исключая дубли
    let mut tech_categories_ids = Vec::new();

    // сначала добавим категорию технологий по умолчанию
    for _serve_cat in __serve_categories.iter() {
        if _categories_names.iter().any(|i| i.to_string()==_serve_cat.cat_name) {
            tech_categories_ids.push(_serve_cat.tech_categories);
            default_price = _serve_cat.default_price
        }
    };
    // теперь добавим остальные категории технологий
    for _serve_cat in __serve_categories.iter() {
        if tech_categories_ids.iter().any(|&i2| i2==_serve_cat.tech_categories) {
            continue;
        } else {
            tech_categories_ids.push(_serve_cat.tech_categories);
        }
    };
    let __tech_categories = tech_categories
        .filter(schema::tech_categories::id.eq(any(&tech_categories_ids)))
        .load::<TechCategories>(&_connection)
        .expect("E");

    // 3. генерируем переменные для шаблона, которые будут хранить наши объекты опций
    let mut _count: i32 = 0;
    for tech_cat_id in tech_categories_ids.iter() {
        _count += 1;
        let mut _let_int : String = _count.to_string().parse().unwrap();
        let _let_serve_categories: String = "serve_categories".to_string() + &_let_int;
        data.insert(&_let_serve_categories, &serve_categories
            .filter(schema::serve_categories::tech_categories.eq(tech_cat_id))
            .filter(schema::serve_categories::id.eq(any(&serve_categories_ids)))
            .load::<ServeCategories>(&_connection)
            .expect("E."));

        let mut _serve_count: i32 = 0;
        for __cat in __serve_categories.iter() {
            _serve_count += 1;
            let mut _serve_int : String = _serve_count.to_string().parse().unwrap();
            let _serve_int_dooble = "_".to_string() + &_let_int;
            let _let_serves: String = _serve_int_dooble.to_owned() + &"serves".to_string() + &_serve_int;
            let __serves :Vec<Serve> = serve
                .filter(schema::serve::serve_categories.eq(__cat.id))
                .filter(schema::serve::id.eq(any(&serve_ids)))
                .order(schema::serve::serve_position.asc())
                .load(&_connection)
                .expect("E.");
            data.insert(&_let_serves, &__serves);
        }
    };

    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("tech_categories", &__tech_categories);
    data.insert("object", &_service[0]);
    data.insert("images", &_images);
    data.insert("videos", &_videos);
    data.insert("categories", &_categories);
    data.insert("service_categories_names", &_categories_names);
    data.insert("category", &_s_category[0]);
    data.insert("tags", &_tags);
    data.insert("tags_count", &_tags.len());
    data.insert("is_admin", &_is_admin);
    data.insert("default_price", &default_price);

    let _template = _type + &"services/service.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn service_category_page(req: HttpRequest, tera: web::Data<Tera>, id: web::Path<i32>) -> impl Responder {
    use schema::service_categories::dsl::service_categories;
    use diesel::dsl::any;
    use crate::schema::tags_items::dsl::tags_items;

    let mut data = Context::new();
    let page_size = 20;
    let mut offset = 0;

    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);
    let _connection = establish_connection();

    let _category = service_categories.filter(schema::service_categories::id.eq(*id)).load::<ServiceCategories>(&_connection).expect("E");
    loop {
        let ids = ServiceCategory::belonging_to(&_category).select(schema::service_category::service_id);
        let _services = schema::services::table
        .filter(schema::services::id.eq(any(ids)))
        .limit(page_size)
        .offset(offset)
        .order(schema::services::service_created.desc())
        .load::<Service>(&_connection)
        .expect("could not load tags");
        if _services.len() > 0 {
            data.insert("services", &_services);
            offset += page_size;
        }
        else {break;}
    };

    let mut stack = Vec::new();
    let _tag_items = tags_items.filter(schema::tags_items::service_id.ne(0)).load::<TagItems>(&_connection).expect("E");
    for _tag_item in _tag_items.iter() {
        if stack.iter().any(|&i| i==_tag_item.tag_id) {
            continue;
        } else {
            stack.push(_tag_item.tag_id);
        }
    };
    let _tags = schema::tags::table
        .filter(schema::tags::id.eq(any(stack)))
        .load::<Tag>(&_connection)
        .expect("could not load tags");

    data.insert("tags", &_tags);
    data.insert("tags_count", &_tags.len());

    data.insert("category", &_category[0]);

    let _template = _type + &"services/category.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn service_categories_page(req: HttpRequest, tera: web::Data<Tera>) -> impl Responder {
    use diesel::dsl::any;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::services::dsl::services;

    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    let _connection = establish_connection();
    let mut data = Context::new();
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);

    let _services = services.filter(schema::services::is_service_active.eq(true)).load::<Service>(&_connection).expect("E");
    let mut _count: i32 = 0;
    for _cat in _service_cats.iter() {
        _count += 1;
        // для генерации переменной 1 2 3
        let mut _let_int : String = _count.to_string().parse().unwrap();
        let _let_data_services: String = "services".to_string() + &_let_int;
        data.insert(&_let_data_services, &get_6_service_for_category(_cat));
    };


    let mut stack = Vec::new();
    for service in _services.iter() {
        let _tag_items = tags_items.filter(schema::tags_items::service_id.eq(service.id)).load::<TagItems>(&_connection).expect("E");
        for _tag_item in _tag_items.iter() {
            if stack.iter().any(|&i| i==_tag_item.tag_id) {
                continue;
            } else {
                stack.push(_tag_item.tag_id);
            }
        };
    };
    let _tags = schema::tags::table
        .filter(schema::tags::id.eq(any(stack)))
        .load::<Tag>(&_connection)
        .expect("could not load tags");

    data.insert("tags", &_tags);
    data.insert("tags_count", &_tags.len());
    let _template = _type + &"services/categories.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn edit_service_page(req: HttpRequest, tera: web::Data<Tera>, _id: web::Path<i32>) -> impl Responder {
    use schema::services::dsl::*;
    use schema::tags::dsl::*;
    use schema::serve::dsl::serve;
    use schema::serve_categories::dsl::serve_categories;
    use schema::tech_categories::dsl::tech_categories;
    use crate::schema::service_images::dsl::service_images;
    use crate::schema::service_videos::dsl::service_videos;

    let _service_id : i32 = *_id;
    let mut data = Context::new();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);
    let _connection = establish_connection();
    let _service = services.filter(schema::services::id.eq(&_service_id)).load::<Service>(&_connection).expect("E");

    let _categories = get_cats_for_service(&_service[0]).0;
    let _all_tags :Vec<Tag> = tags.load(&_connection).expect("Error.");
    let _service_tags = get_tags_for_service(&_service[0]);
    let _serve_list = get_serves_for_service(&_service[0]);

    let _images = service_images.filter(schema::service_images::service.eq(_service[0].id)).load::<ServiceImage>(&_connection).expect("E");
    let _videos = service_videos.filter(schema::service_videos::service.eq(_service[0].id)).load::<ServiceVideo>(&_connection).expect("E");

    let all_tech_categories :Vec<TechCategories> = tech_categories.load(&_connection).expect("E.");

    // генерация переменных шаблона, хранящих: категории опций и опции.
    let mut _count: i32 = 0;
    for _cat in all_tech_categories.iter() {
        _count += 1;
        let mut _let_int : String = _count.to_string().parse().unwrap();
        let _let_serve_categories: String = "serve_categories".to_string() + &_let_int;
        let __serve_categories :Vec<ServeCategories> = serve_categories.filter(schema::serve_categories::tech_categories.eq(_cat.id)).load(&_connection).expect("E.");
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

    data.insert("tech_categories", &all_tech_categories);
    data.insert("service", &_service[0]);
    data.insert("service_tags", &_service_tags);
    data.insert("all_tags", &_all_tags);
    data.insert("categories", &_categories);
    data.insert("serve_list", &_serve_list);
    data.insert("images", &_images);
    data.insert("videos", &_videos);

    let _template = _type + &"services/edit_service.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct ServiceParams {
    content: String,
}
pub async fn edit_content_service_page(req: HttpRequest, tera: web::Data<Tera>, _id: web::Path<i32>) -> impl Responder {
    use schema::services::dsl::*;

    let _service_id : i32 = *_id;
    let _connection = establish_connection();
    let _service = services.filter(schema::services::id.eq(&_service_id)).load::<Service>(&_connection).expect("E");

    let params = web::Query::<ServiceParams>::from_query(&req.query_string()).unwrap();
    if params.content.clone() != "".to_string() {
        diesel::update(&_service[0])
            .set(schema::services::content.eq(&params.content.clone()))
            .get_result::<Service>(&_connection)
            .expect("E.");
    }

    let mut data = Context::new();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);
    data.insert("service", &_service[0]);

    let _template = _type + &"services/edit_content_service.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn edit_service_category_page(req: HttpRequest, tera: web::Data<Tera>, _id: web::Path<i32>) -> impl Responder {
    use schema::service_categories::dsl::*;

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
    let _category = service_categories.filter(schema::service_categories::id.eq(&_cat_id)).load::<ServiceCategories>(&_connection).expect("E");

    data.insert("category", &_category[0]);
    let _template = _type + &"services/edit_category.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}


pub async fn edit_service(mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::models::EditService;
    use crate::schema::services::dsl::services;
    use crate::schema::service_category::dsl::service_category;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::service_videos::dsl::service_videos;
    use crate::schema::service_images::dsl::service_images;
    use crate::schema::service_categories::dsl::service_categories;
    use crate::schema::tags::dsl::tags;
    use crate::schema::serve_items::dsl::serve_items;

    let _connection = establish_connection();
    let _service_id : i32 = *_id;
    let _service = services.filter(schema::services::id.eq(_service_id)).load::<Service>(&_connection).expect("E");

    let _categories = get_cats_for_service(&_service[0]).0;
    let _tags = get_tags_for_service(&_service[0]);
    for _category in _categories.iter() {
        diesel::update(_category)
            .set(schema::service_categories::service_count.eq(_category.service_count - 1))
            .get_result::<ServiceCategories>(&_connection)
            .expect("Error.");
    };
    for _tag in _tags.iter() {
        diesel::update(_tag)
            .set((schema::tags::tag_count.eq(_tag.tag_count - 1), schema::tags::service_count.eq(_tag.service_count - 1)))
            .get_result::<Tag>(&_connection)
            .expect("Error.");
    };

    diesel::delete(service_images.filter(schema::service_images::service.eq(_service_id))).execute(&_connection).expect("E");
    diesel::delete(service_videos.filter(schema::service_videos::service.eq(_service_id))).execute(&_connection).expect("E");
    diesel::delete(tags_items.filter(schema::tags_items::service_id.eq(_service_id))).execute(&_connection).expect("E");
    diesel::delete(serve_items.filter(schema::serve_items::service_id.eq(_service_id))).execute(&_connection).expect("E");
    diesel::delete(service_category.filter(schema::service_category::service_id.eq(_service_id))).execute(&_connection).expect("E");

    let form = store_form(payload.borrow_mut()).await;
    let _new_service = EditService {
        title: form.title.clone(),
        description: Some(form.description.clone()),
        link: Some(form.link.clone()),
        image: Some(form.main_image.clone()),
        is_service_active: form.is_active.clone()
    };

    diesel::update(&_service[0])
        .set(_new_service)
        .get_result::<Service>(&_connection)
        .expect("E");

    for _image in form.images.iter().enumerate() {
        let new_edit_image = NewServiceImage::from_service_images_form(
            _service_id,
            _image.1.to_string()
        );
        diesel::insert_into(schema::service_images::table)
            .values(&new_edit_image)
            .get_result::<ServiceImage>(&_connection)
            .expect("E.");
        };
    for _video in form.videos.iter().enumerate() {
        let new_video = NewServiceVideo::from_service_videos_form(
            _service_id,
            _video.1.to_string()
        );
        diesel::insert_into(schema::service_videos::table)
            .values(&new_video)
            .get_result::<ServiceVideo>(&_connection)
            .expect("E.");
    };
    for category_id in form.category_list.iter().enumerate() {
        let new_category = NewServiceCategory {
            service_categories_id: *category_id.1,
            service_id: _service_id
        };
        diesel::insert_into(schema::service_category::table)
            .values(&new_category)
            .get_result::<ServiceCategory>(&_connection)
            .expect("E.");
        let _category_2 = service_categories.filter(schema::service_categories::id.eq(category_id.1)).load::<ServiceCategories>(&_connection).expect("E");
        diesel::update(&_category_2[0])
            .set(schema::service_categories::service_count.eq(_category_2[0].service_count + 1))
            .get_result::<ServiceCategories>(&_connection)
            .expect("Error.");
    };
    for _tag_id in form.tags_list.iter().enumerate() {
        let _new_tag = NewTagItems{
            tag_id: *_tag_id.1,
            service_id: _service_id,
            store_id: 0,
            blog_id: 0,
            wiki_id: 0,
            work_id: 0,
            tag_created: chrono::Local::now().naive_utc(),
        };
        diesel::insert_into(schema::tags_items::table)
            .values(&_new_tag)
            .get_result::<TagItems>(&_connection)
            .expect("Error.");
        let _tag_2 = tags.filter(schema::tags::id.eq(_tag_id.1)).load::<Tag>(&_connection).expect("E");
        diesel::update(&_tag_2[0])
            .set((schema::tags::tag_count.eq(_tag_2[0].tag_count + 1), schema::tags::service_count.eq(_tag_2[0].service_count + 1)))
            .get_result::<Tag>(&_connection)
            .expect("Error.");
    };
    for _serve_id in form.serve_list.iter().enumerate() {
        let _new_serve = NewServeItems{
            serve_id: *_serve_id.1,
            service_id: _service_id,
            store_id: 0,
            work_id: 0,
        };
        diesel::insert_into(schema::serve_items::table)
            .values(&_new_serve)
            .get_result::<ServeItems>(&_connection)
            .expect("Error.");
    };
    HttpResponse::Ok()
}

pub async fn edit_service_category(mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::models::EditServiceCategories;
    use crate::schema::service_categories::dsl::service_categories;

    let _connection = establish_connection();
    let _cat_id : i32 = *_id;
    let _category = service_categories.filter(schema::service_categories::id.eq(_cat_id)).load::<ServiceCategories>(&_connection).expect("E");

    let form = category_form(payload.borrow_mut()).await;
    let _new_cat = EditServiceCategories {
        name: form.name.clone(),
        description: Some(form.description.clone()),
        service_position: form.position.clone(),
        image: Some(form.image.clone()),
        service_count: _category[0].service_count,
    };

    diesel::update(&_category[0])
        .set(_new_cat)
        .get_result::<ServiceCategories>(&_connection)
        .expect("E");
    HttpResponse::Ok()
}


pub async fn delete_service(_id: web::Path<i32>) -> impl Responder {
    use crate::schema::services::dsl::services;
    use crate::schema::service_category::dsl::service_category;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::serve_items::dsl::serve_items;
    use crate::schema::service_videos::dsl::service_videos;
    use crate::schema::service_images::dsl::service_images;

    let _connection = establish_connection();
    let _service_id : i32 = *_id;
    let _service = services.filter(schema::services::id.eq(_service_id)).load::<Service>(&_connection).expect("E");

    let _categories = get_cats_for_service(&_service[0]).0;
    let _tags = get_tags_for_service(&_service[0]);
    for _category in _categories.iter() {
        diesel::update(_category)
            .set(schema::service_categories::service_count.eq(_category.service_count - 1))
            .get_result::<ServiceCategories>(&_connection)
            .expect("Error.");
    };
    for _tag in _tags.iter() {
        diesel::update(_tag)
            .set((schema::tags::tag_count.eq(_tag.tag_count - 1), schema::tags::service_count.eq(_tag.service_count - 1)))
            .get_result::<Tag>(&_connection)
            .expect("Error.");
    };

    diesel::delete(service_images.filter(schema::service_images::service.eq(_service_id))).execute(&_connection).expect("E");
    diesel::delete(service_videos.filter(schema::service_videos::service.eq(_service_id))).execute(&_connection).expect("E");
    diesel::delete(tags_items.filter(schema::tags_items::service_id.eq(_service_id))).execute(&_connection).expect("E");
    diesel::delete(serve_items.filter(schema::serve_items::service_id.eq(_service_id))).execute(&_connection).expect("E");
    diesel::delete(service_category.filter(schema::service_category::service_id.eq(_service_id))).execute(&_connection).expect("E");
    diesel::delete(&_service[0]).execute(&_connection).expect("E");
    HttpResponse::Ok()
}
pub async fn delete_service_category(_id: web::Path<i32>) -> impl Responder {
    use crate::schema::service_categories::dsl::service_categories;

    let _connection = establish_connection();
    let _cat_id : i32 = *_id;
    let _category = service_categories.filter(schema::service_categories::id.eq(_cat_id)).load::<ServiceCategories>(&_connection).expect("E");
    diesel::delete(service_categories.filter(schema::service_categories::id.eq(_cat_id))).execute(&_connection).expect("E");
    HttpResponse::Ok()
}
