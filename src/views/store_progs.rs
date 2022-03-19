
extern crate diesel;

use actix_web::{web, HttpRequest, HttpResponse, Responder};
use tera::{Tera, Context};
use actix_multipart::Multipart;
use std::borrow::BorrowMut;
use diesel::prelude::*;
use crate::utils::{
    store_split_payload,
    category_split_payload,
    get_template_2,
    establish_connection
};
use crate::schema;
use crate::models::{
    StoreCategories,
    NewStoreCategories,
    Store,
    NewStore,
    StoreCategory,
    NewStoreCategory,
    StoreImage,
    NewStoreImage,
    StoreVideo,
    NewStoreVideo,
    TagItems,
    NewTagItems,
    Tag,
};

fn get_cats_for_store(store: &Store) -> Vec<StoreCategories> {
    use diesel::pg::expression::dsl::any;
    let _connection = establish_connection();

    let ids = StoreCategory::belonging_to(store).select(schema::store_category::store_categories_id);
    schema::store_categories::table
        .filter(schema::store_categories::id.eq(any(ids)))
        .load::<StoreCategories>(&_connection)
        .expect("could not load tags")
}
fn get_tags_for_store(store: &Store) -> Vec<Tag> {
    use crate::schema::tags_items::dsl::tags_items;
    use diesel::dsl::any;
    let _connection = establish_connection();

    let _tag_items = tags_items.filter(schema::tags_items::store_id.eq(&store.id)).load::<TagItems>(&_connection).expect("E");
    let mut stack = Vec::new();
    for _tag_item in _tag_items.iter() {
        stack.push(_tag_item.tag_id);
    };
    schema::tags::table
        .filter(schema::tags::id.eq(any(stack)))
        .load::<Tag>(&_connection)
        .expect("could not load tags")
}
fn get_6_store_for_category(category: &StoreCategories) -> Vec<Store> {
    use diesel::pg::expression::dsl::any;
    let _connection = establish_connection();

    let ids = StoreCategory::belonging_to(category).select(schema::store_category::store_id);
    schema::stores::table
        .filter(schema::stores::id.eq(any(ids)))
        .order(schema::stores::store_created.desc())
        .limit(6)
        .load::<Store>(&_connection)
        .expect("could not load tags")
}
fn get_store_for_category(category: &StoreCategories) -> Vec<Store> {
    use diesel::pg::expression::dsl::any;
    let _connection = establish_connection();

    let ids = StoreCategory::belonging_to(category).select(schema::store_category::store_id);
    schema::stores::table
        .filter(schema::stores::id.eq(any(ids)))
        .order(schema::stores::store_created.desc())
        .load::<Store>(&_connection)
        .expect("could not load tags")
}

pub async fn create_store_categories_page(req: HttpRequest, tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);
    let _template = _type + &"stores/create_categories.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn create_store_page(req: HttpRequest, tera: web::Data<Tera>) -> impl Responder {
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
    let all_tags :Vec<Tag> = tags
        .load(&_connection)
        .expect("Error.");

    data.insert("tags", &all_tags);
    let _template = _type + &"stores/create_store.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn create_store_categories(mut payload: Multipart) -> impl Responder {
    use schema::store_categories;

    let _connection = establish_connection();
    let form = category_split_payload(payload.borrow_mut()).await;
    let new_cat = NewStoreCategories {
        name: form.name.clone(),
        store_position: form.position.clone(),
        image: Some(form.image.clone()),
        store_count: 0
    };
    let _new_store = diesel::insert_into(store_categories::table)
        .values(&new_cat)
        .get_result::<StoreCategories>(&_connection)
        .expect("Error saving post.");
    return HttpResponse::Ok();
}
pub async fn create_store(mut payload: Multipart) -> impl Responder {
    use schema::{stores,store_images,store_videos,store_category,tags_items};
    use crate::schema::tags::dsl::tags;
    use crate::schema::store_categories::dsl::store_categories;

    let _connection = establish_connection();

    let form = store_split_payload(payload.borrow_mut()).await;
    let new_store = NewStore::from_store_form(
        form.title.clone(),
        form.description.clone(),
        form.link.clone(),
        form.main_image.clone(),
        form.is_active.clone(),
        form.price.clone(),
        Some(form.price_acc.clone()),
        Some(form.social_price.clone()),
        1
    );

    let _store = diesel::insert_into(stores::table)
        .values(&new_store)
        .get_result::<Store>(&_connection)
        .expect("Error saving store.");

    for image in form.images.iter().enumerate() {
        let new_image = NewStoreImage::from_store_images_form(
            _store.id,
            image.1.to_string()
        );
        diesel::insert_into(store_images::table)
            .values(&new_image)
            .get_result::<StoreImage>(&_connection)
            .expect("Error saving store.");
        };
    for video in form.videos.iter().enumerate() {
        let new_video = NewStoreVideo::from_store_videos_form(
            _store.id,
            video.1.to_string()
        );
        diesel::insert_into(store_videos::table)
            .values(&new_video)
            .get_result::<StoreVideo>(&_connection)
            .expect("Error saving store.");
    };
    for category_id in form.category_list.iter().enumerate() {
        let new_category = NewStoreCategory {
            store_categories_id: *category_id.1,
            store_id: _store.id
        };
        diesel::insert_into(store_category::table)
            .values(&new_category)
            .get_result::<StoreCategory>(&_connection)
            .expect("Error saving store.");
            let _category = store_categories.filter(schema::store_categories::id.eq(category_id.1)).load::<StoreCategories>(&_connection).expect("E");
        diesel::update(&_category[0])
            .set(schema::store_categories::store_count.eq(_category[0].store_count + 1))
            .get_result::<StoreCategories>(&_connection)
            .expect("Error.");
    };
    for tag_id in form.tags_list.iter().enumerate() {
        let new_tag = NewTagItems{
            tag_id: *tag_id.1,
            service_id: 0,
            store_id: _store.id,
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
            .set((schema::tags::tag_count.eq(_tag[0].tag_count + 1), schema::tags::store_count.eq(_tag[0].store_count + 1)))
            .get_result::<Tag>(&_connection)
            .expect("Error.");
    };
    HttpResponse::Ok()
}

pub async fn get_store_page(req: HttpRequest, tera: web::Data<Tera>, param: web::Path<(i32,i32)>) -> impl Responder {
    use schema::stores::dsl::stores;
    use schema::store_images::dsl::store_images;
    use schema::store_videos::dsl::store_videos;
    use schema::store_categories::dsl::store_categories;

    let _connection = establish_connection();
    let _store_id : i32 = param.1;
    let _cat_id : i32 = param.0;
    let _store = stores.filter(schema::stores::id.eq(&_store_id)).load::<Store>(&_connection).expect("E");

    let _s_category = store_categories
        .filter(schema::store_categories::id.eq(&_cat_id))
        .load::<StoreCategories>(&_connection)
        .expect("E");

    let mut data = Context::new();

    let _category_stores = get_store_for_category(&_s_category[0]);
    let _category_stores_len : usize = _category_stores.len();
    for (i, item) in _category_stores.iter().enumerate().rev() {
        if item.id == _store_id {
            if (i + 1) != _category_stores_len {
                let _prev = Some(&_category_stores[i + 1]);
                data.insert("prev", &_prev);
            };
            if i != 0 {
                let _next = Some(&_category_stores[i - 1]);
                data.insert("next", &_next);
            };
            break;
        }
    };

    let _images :Vec<StoreImage> = store_images.filter(schema::store_images::store.eq(&_store_id)).load(&_connection).expect("E");
    let _videos :Vec<StoreVideo> = store_videos.filter(schema::store_videos::store.eq(&_store_id)).load(&_connection).expect("E");
    let _categories = get_cats_for_store(&_store[0]);
    let _tags = get_tags_for_store(&_store[0]);

    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("object", &_store);
    data.insert("images", &_images);
    data.insert("videos", &_videos);
    data.insert("categories", &_categories);
    data.insert("category", &_s_category[0]);
    data.insert("tags", &_tags);
    data.insert("tags_count", &_tags.len());
    data.insert("is_admin", &_is_admin);

    let _template = _type + &"stores/store.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn store_category_page(req: HttpRequest, tera: web::Data<Tera>, id: web::Path<i32>) -> impl Responder {
    use schema::store_categories::dsl::store_categories;
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

    let _category = store_categories.filter(schema::store_categories::id.eq(*id)).load::<StoreCategories>(&_connection).expect("E");

    data.insert("category", &_category[0]);

    loop {
        let ids = StoreCategory::belonging_to(&_category).select(schema::store_category::store_id);
        let _stores = schema::stores::table
        .filter(schema::stores::id.eq(any(ids)))
        .limit(page_size)
        .offset(offset)
        .order(schema::stores::store_created.desc())
        .load::<Store>(&_connection)
        .expect("could not load tags");
        data.insert("stores", &_stores);
         if _stores.len() <= 0 { break;}
         offset += page_size;
    };

    let mut stack = Vec::new();
    let _tag_items = tags_items.filter(schema::tags_items::store_id.ne(0)).load::<TagItems>(&_connection).expect("E");
    for _tag_item in _tag_items.iter() {
        if stack.iter().any(|&i| i==_tag_item.tag_id) {
            println!("Exists!");
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

    let _template = _type + &"stores/category.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn store_categories_page(req: HttpRequest, tera: web::Data<Tera>) -> impl Responder {
    use diesel::dsl::any;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::stores::dsl::stores;

    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    let _connection = establish_connection();
    let mut data = Context::new();
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);

    let _stores = stores.filter(schema::stores::is_store_active.eq(true)).load::<Store>(&_connection).expect("E");
    let mut _count: i32 = 0;
    for _cat in _store_cats.iter() {
        _count += 1;
        // для генерации переменной 1 2 3
        let mut _let_int : String = _count.to_string().parse().unwrap();
        let _let_data_stores: String = "stores".to_string() + &_let_int;
        data.insert(&_let_data_stores, &get_6_store_for_category(_cat));
    };

    let mut stack = Vec::new();
    for store in _stores.iter() {
        let _tag_items = tags_items.filter(schema::tags_items::store_id.eq(store.id)).load::<TagItems>(&_connection).expect("E");
        for _tag_item in _tag_items.iter() {
            if stack.iter().any(|&i| i==_tag_item.tag_id) {
                println!("Exists!");
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

    let _template = _type + &"stores/categories.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn edit_store_page(req: HttpRequest, tera: web::Data<Tera>, _id: web::Path<i32>) -> impl Responder {
    use schema::stores::dsl::*;
    use schema::tags::dsl::*;
    use crate::schema::store_images::dsl::store_images;
    use crate::schema::store_videos::dsl::store_videos;

    let _store_id : i32 = *_id;
    let mut data = Context::new();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);
    let _connection = establish_connection();
    let _store = stores.filter(schema::stores::id.eq(&_store_id)).load::<Store>(&_connection).expect("E");

    let _categories = get_cats_for_store(&_store[0]);
    let _all_tags :Vec<Tag> = tags.load(&_connection).expect("Error.");
    let _store_tags = get_tags_for_store(&_store[0]);

    let _images = store_images.filter(schema::store_images::store.eq(_store[0].id)).load::<StoreImage>(&_connection).expect("E");
    let _videos = store_videos.filter(schema::store_videos::store.eq(_store[0].id)).load::<StoreVideo>(&_connection).expect("E");

    data.insert("store", &_store[0]);
    data.insert("store_tags", &_store_tags);
    data.insert("all_tags", &_all_tags);
    data.insert("categories", &_categories);
    data.insert("images", &_images);
    data.insert("videos", &_videos);

    let _template = _type + &"stores/edit_store.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct StoreParams {
    content: String,
}
pub async fn edit_content_store_page(req: HttpRequest, tera: web::Data<Tera>, _id: web::Path<i32>) -> impl Responder {
    use schema::stores::dsl::*;

    let _store_id : i32 = *_id;
    let _connection = establish_connection();
    let _store = stores.filter(schema::stores::id.eq(&_store_id)).load::<Store>(&_connection).expect("E");

    let params = web::Query::<StoreParams>::from_query(&req.query_string()).unwrap();
    if params.content.clone() != "".to_string() {
        diesel::update(&_store[0])
            .set(schema::stores::content.eq(&params.content.clone()))
            .get_result::<Store>(&_connection)
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
    data.insert("store", &_store[0]);

    let _template = _type + &"stores/edit_content_store.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn edit_store_category_page(req: HttpRequest, tera: web::Data<Tera>, _id: web::Path<i32>) -> impl Responder {
    use schema::store_categories::dsl::*;

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
    let _category = store_categories.filter(schema::store_categories::id.eq(&_cat_id)).load::<StoreCategories>(&_connection).expect("E");

    data.insert("category", &_category[0]);
    let _template = _type + &"stores/edit_category.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}


pub async fn edit_store(mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::models::EditStore;
    use crate::schema::stores::dsl::stores;
    use crate::schema::store_category::dsl::store_category;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::store_videos::dsl::store_videos;
    use crate::schema::store_images::dsl::store_images;
    use crate::schema::store_categories::dsl::store_categories;
    use crate::schema::tags::dsl::tags;

    let _connection = establish_connection();
    let _store_id : i32 = *_id;
    let _store = stores.filter(schema::stores::id.eq(_store_id)).load::<Store>(&_connection).expect("E");

    let _categories = get_cats_for_store(&_store[0]);
    let _tags = get_tags_for_store(&_store[0]);
    for _category in _categories.iter() {
        diesel::update(_category)
            .set(schema::store_categories::store_count.eq(_category.store_count - 1))
            .get_result::<StoreCategories>(&_connection)
            .expect("Error.");
    };
    for _tag in _tags.iter() {
        diesel::update(_tag)
            .set((schema::tags::tag_count.eq(_tag.tag_count - 1), schema::tags::store_count.eq(_tag.store_count - 1)))
            .get_result::<Tag>(&_connection)
            .expect("Error.");
    };

    diesel::delete(store_images.filter(schema::store_images::store.eq(_store_id))).execute(&_connection).expect("E");
    diesel::delete(store_videos.filter(schema::store_videos::store.eq(_store_id))).execute(&_connection).expect("E");
    diesel::delete(tags_items.filter(schema::tags_items::store_id.eq(_store_id))).execute(&_connection).expect("E");
    diesel::delete(store_category.filter(schema::store_category::store_id.eq(_store_id))).execute(&_connection).expect("E");

    let form = store_split_payload(payload.borrow_mut()).await;
    let _new_store = EditStore {
        title: form.title.clone(),
        description: Some(form.description.clone()),
        link: Some(form.link.clone()),
        image: Some(form.main_image.clone()),
        is_store_active: form.is_active.clone(),
        price: form.price.clone(),
        price_acc: form.price_acc.clone(),
        social_price: form.social_price.clone(),
    };

    diesel::update(&_store[0])
        .set(_new_store)
        .get_result::<Store>(&_connection)
        .expect("E");

    for _image in form.images.iter().enumerate() {
        let new_edit_image = NewStoreImage::from_store_images_form(
            _store_id,
            _image.1.to_string()
        );
        diesel::insert_into(schema::store_images::table)
            .values(&new_edit_image)
            .get_result::<StoreImage>(&_connection)
            .expect("E.");
        };
    for _video in form.videos.iter().enumerate() {
        let new_video = NewStoreVideo::from_store_videos_form(
            _store_id,
            _video.1.to_string()
        );
        diesel::insert_into(schema::store_videos::table)
            .values(&new_video)
            .get_result::<StoreVideo>(&_connection)
            .expect("E.");
    };
    for category_id in form.category_list.iter().enumerate() {
        let new_category = NewStoreCategory {
            store_categories_id: *category_id.1,
            store_id: _store_id
        };
        diesel::insert_into(schema::store_category::table)
            .values(&new_category)
            .get_result::<StoreCategory>(&_connection)
            .expect("E.");
        let _category_2 = store_categories.filter(schema::store_categories::id.eq(category_id.1)).load::<StoreCategories>(&_connection).expect("E");
        diesel::update(&_category_2[0])
            .set(schema::store_categories::store_count.eq(_category_2[0].store_count + 1))
            .get_result::<StoreCategories>(&_connection)
            .expect("Error.");
    };
    for _tag_id in form.tags_list.iter().enumerate() {
        let _new_tag = NewTagItems{
            tag_id: *_tag_id.1,
            service_id: 0,
            store_id: _store_id,
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
            .set((schema::tags::tag_count.eq(_tag_2[0].tag_count + 1), schema::tags::store_count.eq(_tag_2[0].store_count + 1)))
            .get_result::<Tag>(&_connection)
            .expect("Error.");
    };
    HttpResponse::Ok()
}

pub async fn edit_store_category(mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::models::EditStoreCategories;
    use crate::schema::store_categories::dsl::store_categories;

    let _connection = establish_connection();
    let _cat_id : i32 = *_id;
    let _category = store_categories.filter(schema::store_categories::id.eq(_cat_id)).load::<StoreCategories>(&_connection).expect("E");

    let form = category_split_payload(payload.borrow_mut()).await;
    let _new_cat = EditStoreCategories {
        name: form.name.clone(),
        store_position: form.position.clone(),
        image: Some(form.image.clone()),
        store_count: _category[0].store_count,
    };

    diesel::update(&_category[0])
        .set(_new_cat)
        .get_result::<StoreCategories>(&_connection)
        .expect("E");
    HttpResponse::Ok()
}


pub async fn delete_store(_id: web::Path<i32>) -> impl Responder {
    use crate::schema::stores::dsl::stores;
    use crate::schema::store_category::dsl::store_category;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::store_videos::dsl::store_videos;
    use crate::schema::store_images::dsl::store_images;

    let _connection = establish_connection();
    let _store_id : i32 = *_id;
    let _store = stores.filter(schema::stores::id.eq(_store_id)).load::<Store>(&_connection).expect("E");

    let _categories = get_cats_for_store(&_store[0]);
    let _tags = get_tags_for_store(&_store[0]);
    for _category in _categories.iter() {
        diesel::update(_category)
            .set(schema::store_categories::store_count.eq(_category.store_count - 1))
            .get_result::<StoreCategories>(&_connection)
            .expect("Error.");
    };
    for _tag in _tags.iter() {
        diesel::update(_tag)
            .set((schema::tags::tag_count.eq(_tag.tag_count - 1), schema::tags::store_count.eq(_tag.store_count - 1)))
            .get_result::<Tag>(&_connection)
            .expect("Error.");
    };

    diesel::delete(store_images.filter(schema::store_images::store.eq(_store_id))).execute(&_connection).expect("E");
    diesel::delete(store_videos.filter(schema::store_videos::store.eq(_store_id))).execute(&_connection).expect("E");
    diesel::delete(tags_items.filter(schema::tags_items::store_id.eq(_store_id))).execute(&_connection).expect("E");
    diesel::delete(store_category.filter(schema::store_category::store_id.eq(_store_id))).execute(&_connection).expect("E");
    diesel::delete(&_store[0]).execute(&_connection).expect("E");
    HttpResponse::Ok()
}
pub async fn delete_store_category(_id: web::Path<i32>) -> impl Responder {
    use crate::schema::store_categories::dsl::store_categories;

    let _connection = establish_connection();
    let _cat_id : i32 = *_id;
    let _category = store_categories.filter(schema::store_categories::id.eq(_cat_id)).load::<StoreCategories>(&_connection).expect("E");
    diesel::delete(store_categories.filter(schema::store_categories::id.eq(_cat_id))).execute(&_connection).expect("E");
    HttpResponse::Ok()
}
