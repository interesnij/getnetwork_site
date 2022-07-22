use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
};
use crate::diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use actix_session::Session;
use actix_multipart::Multipart;
use std::borrow::BorrowMut;
use crate::utils::{
    item_form,
    category_form,
    establish_connection,
};
use crate::schema;
use crate::models::{
    WorkCategories,
    NewWorkCategories,
    Work,
    NewWork,
    WorkCategory,
    NewWorkCategory,
    WorkImage,
    NewWorkImage,
    WorkVideo,
    NewWorkVideo,
    TagItems,
    NewTagItems,
    Tag,
};
use sailfish::TemplateOnce;


pub fn work_routes(config: &mut web::ServiceConfig) {
    config.route("/work_categories/", web::get().to(work_categories_page));
    config.service(web::resource("/create_work_categories/")
        .route(web::get().to(create_work_categories_page))
        .route(web::post().to(create_work_categories))
    );
    config.service(web::resource("/edit_work_category/{id}/")
        .route(web::get().to(edit_work_category_page))
        .route(web::post().to(edit_work_category))
    );
    config.service(web::resource("/create_work/")
        .route(web::get().to(create_work_page))
        .route(web::post().to(create_work))
    );
    config.service(web::resource("/edit_work/{id}/")
        .route(web::get().to(edit_work_page))
        .route(web::post().to(edit_work))
    );
    config.route("/edit_content_work/{id}/", web::get().to(edit_content_work_page));
    config.route("/delete_work/{id}/", web::get().to(delete_work));
    config.route("/delete_work_category/{id}/", web::get().to(delete_work_category));
    config.service(web::resource("/work/{cat_id}/{work_id}/").route(web::get().to(get_work_page)));
    config.service(web::resource("/work/{id}/").route(web::get().to(work_category_page)));
}

fn get_cats_for_work(work: &Work) -> Vec<WorkCategories> {
    use diesel::pg::expression::dsl::any;
    let _connection = establish_connection();

    let ids = WorkCategory::belonging_to(work).select(schema::work_category::work_categories_id);
    schema::work_categories::table
        .filter(schema::work_categories::id.eq(any(ids)))
        .load::<WorkCategories>(&_connection)
        .expect("could not load tags")
}
fn get_tags_for_work(work: &Work) -> Vec<Tag> {
    use crate::schema::tags_items::dsl::tags_items;
    use diesel::dsl::any;
    let _connection = establish_connection();

    let _tag_items = tags_items.filter(schema::tags_items::work_id.eq(&work.id)).load::<TagItems>(&_connection).expect("E");
    let mut stack = Vec::new();
    for _tag_item in _tag_items.iter() {
        stack.push(_tag_item.tag_id);
    };
    schema::tags::table
        .filter(schema::tags::id.eq(any(stack)))
        .load::<Tag>(&_connection)
        .expect("could not load tags")
}
fn get_6_work_for_category(category: &WorkCategories) -> Vec<Work> {
    use diesel::pg::expression::dsl::any;
    let _connection = establish_connection();

    let ids = WorkCategory::belonging_to(category).select(schema::work_category::work_id);
    schema::works::table
        .filter(schema::works::id.eq(any(ids)))
        .order(schema::works::created.desc())
        .limit(6)
        .load::<Work>(&_connection)
        .expect("could not load tags")
}
fn get_work_for_category(category: &WorkCategories) -> Vec<Work> {
    use diesel::pg::expression::dsl::any;
    let _connection = establish_connection();

    let ids = WorkCategory::belonging_to(category).select(schema::work_category::work_id);
    schema::works::table
        .filter(schema::works::id.eq(any(ids)))
        .order(schema::works::created.desc())
        .load::<Work>(&_connection)
        .expect("could not load tags")
}

pub async fn create_work_categories_page(req: HttpRequest) -> impl Responder {
    let mut data = Context::new();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);
    let _template = _type + &"works/create_categories.html".to_string();
    let _rendered = TEMPLATES.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn create_work_page(req: HttpRequest) -> impl Responder {
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
    let _template = _type + &"works/create_work.html".to_string();
    let _rendered = TEMPLATES.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn create_work_categories(mut payload: Multipart) -> impl Responder {
    use schema::work_categories;

    let _connection = establish_connection();
    let form = category_form(payload.borrow_mut()).await;
    let new_cat = NewWorkCategories {
        name: form.name.clone(),
        description: Some(form.description.clone()),
        position: form.position.clone(),
        image: Some(form.image.clone()),
        count: 0
    };
    let _new_work = diesel::insert_into(work_categories::table)
        .values(&new_cat)
        .get_result::<WorkCategories>(&_connection)
        .expect("Error saving post.");
    return HttpResponse::Ok();
}
pub async fn create_work(mut payload: Multipart) -> impl Responder {
    use schema::{works,work_images,work_videos,work_category,tags_items};
    use crate::schema::tags::dsl::tags;
    use crate::schema::work_categories::dsl::work_categories;

    let _connection = establish_connection();

    let form = item_form(payload.borrow_mut()).await;
    let new_work = NewWork::from_work_form(
        form.title.clone(),
        form.description.clone(),
        form.link.clone(),
        form.main_image.clone(),
        form.is_active.clone(),
        1
    );

    let _work = diesel::insert_into(works::table)
        .values(&new_work)
        .get_result::<Work>(&_connection)
        .expect("Error saving work.");

    for image in form.images.iter().enumerate() {
        let new_image = NewWorkImage::from_work_images_form(
            _work.id,
            image.1.to_string()
        );
        diesel::insert_into(work_images::table)
            .values(&new_image)
            .get_result::<WorkImage>(&_connection)
            .expect("Error saving work.");
        };
    for video in form.videos.iter().enumerate() {
        let new_video = NewWorkVideo::from_work_videos_form(
            _work.id,
            video.1.to_string()
        );
        diesel::insert_into(work_videos::table)
            .values(&new_video)
            .get_result::<WorkVideo>(&_connection)
            .expect("Error saving work.");
    };
    for category_id in form.category_list.iter().enumerate() {
        let new_category = NewWorkCategory {
            work_categories_id: *category_id.1,
            work_id: _work.id
        };
        let _new_work_category = diesel::insert_into(work_category::table)
            .values(&new_category)
            .get_result::<WorkCategory>(&_connection)
            .expect("E.");
        let _category = work_categories.filter(schema::work_categories::id.eq(category_id.1)).load::<WorkCategories>(&_connection).expect("E");
        diesel::update(&_category[0])
            .set(schema::work_categories::count.eq(_category[0].count + 1))
            .get_result::<WorkCategories>(&_connection)
            .expect("Error.");
    };
    for tag_id in form.tags_list.iter().enumerate() {
        let new_tag = NewTagItems{
            tag_id: *tag_id.1,
            service_id: 0,
            store_id: 0,
            blog_id: 0,
            wiki_id: 0,
            work_id: _work.id,
            created: chrono::Local::now().naive_utc(),
        };
        diesel::insert_into(tags_items::table)
            .values(&new_tag)
            .get_result::<TagItems>(&_connection)
            .expect("Error.");
        let _tag = tags.filter(schema::tags::id.eq(tag_id.1)).load::<Tag>(&_connection).expect("E");
        diesel::update(&_tag[0])
            .set((schema::tags::count.eq(_tag[0].count + 1), schema::tags::work_count.eq(_tag[0].work_count + 1)))
            .get_result::<Tag>(&_connection)
            .expect("Error.");
    };
    HttpResponse::Ok()
}

pub async fn get_work_page(req: HttpRequest, param: web::Path<(i32,i32)>) -> impl Responder {
    use schema::works::dsl::works;
    use schema::work_images::dsl::work_images;
    use schema::work_videos::dsl::work_videos;
    use schema::work_categories::dsl::work_categories;

    let _connection = establish_connection();
    let _work_id : i32 = param.1;
    let _cat_id : i32 = param.0;

    let _work = works.filter(schema::works::id.eq(&_work_id)).load::<Work>(&_connection).expect("E");
    let _s_category = work_categories
        .filter(schema::work_categories::id.eq(&_cat_id))
        .load::<WorkCategories>(&_connection)
        .expect("E");

    let mut data = Context::new();

    let _category_works = get_work_for_category(&_s_category[0]);
    let _category_works_len : usize = _category_works.len();
    for (i, item) in _category_works.iter().enumerate().rev() {
        if item.id == _work_id {
            if (i + 1) != _category_works_len {
                let _prev = Some(&_category_works[i + 1]);
                data.insert("prev", &_prev);
            };
            if i != 0 {
                let _next = Some(&_category_works[i - 1]);
                data.insert("next", &_next);
            };
            break;
        }
    };

    let _images :Vec<WorkImage> = work_images.filter(schema::work_images::work.eq(&_work_id)).load(&_connection).expect("E");
    let _videos :Vec<WorkVideo> = work_videos.filter(schema::work_videos::work.eq(&_work_id)).load(&_connection).expect("E");
    let _categories = get_cats_for_work(&_work[0]);
    let _tags = get_tags_for_work(&_work[0]);

    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("object", &_work[0]);
    data.insert("images", &_images);
    data.insert("videos", &_videos);
    data.insert("categories", &_categories);
    data.insert("category", &_s_category[0]);
    data.insert("tags", &_tags);
    data.insert("tags_count", &_tags.len());
    data.insert("is_admin", &_is_admin);

    let _template = _type + &"works/work.html".to_string();
    let _rendered = TEMPLATES.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn work_category_page(req: HttpRequest, id: web::Path<i32>) -> impl Responder {
    use schema::work_categories::dsl::work_categories;
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

    let _category = work_categories.filter(schema::work_categories::id.eq(*id)).load::<WorkCategories>(&_connection).expect("E");

    data.insert("category", &_category[0]);

    loop {
        let ids = WorkCategory::belonging_to(&_category).select(schema::work_category::work_id);
        let _works = schema::works::table
        .filter(schema::works::id.eq(any(ids)))
        .limit(page_size)
        .offset(offset)
        .order(schema::works::created.desc())
        .load::<Work>(&_connection)
        .expect("could not load tags");
        if _works.len() > 0 {
            data.insert("works", &_works);
            offset += page_size;
        }
        else {break;}
    };

    let mut stack = Vec::new();
    let _tag_items = tags_items.filter(schema::tags_items::work_id.ne(0)).load::<TagItems>(&_connection).expect("E");
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

    let _template = _type + &"works/category.html".to_string();
    let _rendered = TEMPLATES.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn work_categories_page(req: HttpRequest) -> impl Responder {
    use diesel::dsl::any;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::works::dsl::works;

    let _connection = establish_connection();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    let mut data = Context::new();
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);

    let _works = works.filter(schema::works::is_active.eq(true)).load::<Work>(&_connection).expect("E");
    let mut _count: i32 = 0;
    for _cat in _work_cats.iter() {
        _count += 1;
        // для генерации переменной 1 2 3
        let mut _let_int : String = _count.to_string().parse().unwrap();
        let _let_data_works: String = "works".to_string() + &_let_int;
        data.insert(&_let_data_works, &get_6_work_for_category(_cat));
    };


    let mut stack = Vec::new();
    for work in _works.iter() {
        let _tag_items = tags_items.filter(schema::tags_items::work_id.eq(work.id)).load::<TagItems>(&_connection).expect("E");
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

    let _template = _type + &"works/categories.html".to_string();
    let _rendered = TEMPLATES.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn edit_work_page(req: HttpRequest, _id: web::Path<i32>) -> impl Responder {
    use schema::works::dsl::*;
    use schema::tags::dsl::*;
    use crate::schema::work_images::dsl::work_images;
    use crate::schema::work_videos::dsl::work_videos;

    let _work_id : i32 = *_id;
    let mut data = Context::new();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);
    let _connection = establish_connection();
    let _work = works.filter(schema::works::id.eq(&_work_id)).load::<Work>(&_connection).expect("E");

    let _categories = get_cats_for_work(&_work[0]);
    let _all_tags :Vec<Tag> = tags.load(&_connection).expect("Error.");
    let _work_tags = get_tags_for_work(&_work[0]);

    let _images = work_images.filter(schema::work_images::work.eq(_work[0].id)).load::<WorkImage>(&_connection).expect("E");
    let _videos = work_videos.filter(schema::work_videos::work.eq(_work[0].id)).load::<WorkVideo>(&_connection).expect("E");

    data.insert("work", &_work[0]);
    data.insert("work_tags", &_work_tags);
    data.insert("all_tags", &_all_tags);
    data.insert("categories", &_categories);
    data.insert("images", &_images);
    data.insert("videos", &_videos);

    let _template = _type + &"works/edit_work.html".to_string();
    let _rendered = TEMPLATES.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct WorkParams {
    content: String,
}
pub async fn edit_content_work_page(req: HttpRequest, _id: web::Path<i32>) -> impl Responder {
    use schema::works::dsl::*;

    let _work_id : i32 = *_id;
    let _connection = establish_connection();
    let _work = works.filter(schema::works::id.eq(&_work_id)).load::<Work>(&_connection).expect("E");

    let params = web::Query::<WorkParams>::from_query(&req.query_string()).unwrap();
    println!("params {:?}", params);
    if params.content.clone() != "".to_string() {
        diesel::update(&_work[0])
            .set(schema::works::content.eq(&params.content.clone()))
            .get_result::<Work>(&_connection)
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
    data.insert("work", &_work[0]);

    let _template = _type + &"works/edit_content_work.html".to_string();
    let _rendered = TEMPLATES.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn edit_work_category_page(req: HttpRequest, _id: web::Path<i32>) -> impl Responder {
    use schema::work_categories::dsl::*;

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
    let _category = work_categories.filter(schema::work_categories::id.eq(&_cat_id)).load::<WorkCategories>(&_connection).expect("E");

    data.insert("category", &_category[0]);
    let _template = _type + &"works/edit_category.html".to_string();
    let _rendered = TEMPLATES.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}


pub async fn edit_work(mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::models::EditWork;
    use crate::schema::works::dsl::works;
    use crate::schema::work_category::dsl::work_category;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::work_videos::dsl::work_videos;
    use crate::schema::work_images::dsl::work_images;
    use crate::schema::work_categories::dsl::work_categories;
    use crate::schema::tags::dsl::tags;

    let _connection = establish_connection();
    let _work_id : i32 = *_id;
    let _work = works.filter(schema::works::id.eq(_work_id)).load::<Work>(&_connection).expect("E");

    let _categories = get_cats_for_work(&_work[0]);
    let _tags = get_tags_for_work(&_work[0]);
    for _category in _categories.iter() {
        diesel::update(_category)
            .set(schema::work_categories::count.eq(_category.count - 1))
            .get_result::<WorkCategories>(&_connection)
            .expect("Error.");
    };
    for _tag in _tags.iter() {
        diesel::update(_tag)
            .set((schema::tags::count.eq(_tag.count - 1), schema::tags::work_count.eq(_tag.work_count - 1)))
            .get_result::<Tag>(&_connection)
            .expect("Error.");
    };

    diesel::delete(work_images.filter(schema::work_images::work.eq(_work_id))).execute(&_connection).expect("E");
    diesel::delete(work_videos.filter(schema::work_videos::work.eq(_work_id))).execute(&_connection).expect("E");
    diesel::delete(tags_items.filter(schema::tags_items::work_id.eq(_work_id))).execute(&_connection).expect("E");
    diesel::delete(work_category.filter(schema::work_category::work_id.eq(_work_id))).execute(&_connection).expect("E");

    let form = item_form(payload.borrow_mut()).await;
    let _new_work = EditWork {
        title: form.title.clone(),
        description: Some(form.description.clone()),
        link: Some(form.link.clone()),
        image: Some(form.main_image.clone()),
        is_active: form.is_active.clone()
    };

    diesel::update(&_work[0])
        .set(_new_work)
        .get_result::<Work>(&_connection)
        .expect("E");

    for _image in form.images.iter().enumerate() {
        let new_edit_image = NewWorkImage::from_work_images_form(
            _work_id,
            _image.1.to_string()
        );
        diesel::insert_into(schema::work_images::table)
            .values(&new_edit_image)
            .get_result::<WorkImage>(&_connection)
            .expect("E.");
        };
    for _video in form.videos.iter().enumerate() {
        let new_video = NewWorkVideo::from_work_videos_form(
            _work_id,
            _video.1.to_string()
        );
        diesel::insert_into(schema::work_videos::table)
            .values(&new_video)
            .get_result::<WorkVideo>(&_connection)
            .expect("E.");
    };
    for category_id in form.category_list.iter().enumerate() {
        let new_category = NewWorkCategory {
            work_categories_id: *category_id.1,
            work_id: _work_id
        };
        diesel::insert_into(schema::work_category::table)
            .values(&new_category)
            .get_result::<WorkCategory>(&_connection)
            .expect("E.");
        let _category_2 = work_categories.filter(schema::work_categories::id.eq(category_id.1)).load::<WorkCategories>(&_connection).expect("E");
        diesel::update(&_category_2[0])
            .set(schema::work_categories::count.eq(_category_2[0].count + 1))
            .get_result::<WorkCategories>(&_connection)
            .expect("Error.");
    };
    for _tag_id in form.tags_list.iter().enumerate() {
        let _new_tag = NewTagItems{
            tag_id: *_tag_id.1,
            service_id: 0,
            store_id: 0,
            blog_id: 0,
            wiki_id: 0,
            work_id: _work_id,
            created: chrono::Local::now().naive_utc(),
        };
        diesel::insert_into(schema::tags_items::table)
            .values(&_new_tag)
            .get_result::<TagItems>(&_connection)
            .expect("Error.");
        let _tag_2 = tags.filter(schema::tags::id.eq(_tag_id.1)).load::<Tag>(&_connection).expect("E");
        diesel::update(&_tag_2[0])
            .set((schema::tags::count.eq(_tag_2[0].count + 1), schema::tags::work_count.eq(_tag_2[0].work_count + 1)))
            .get_result::<Tag>(&_connection)
            .expect("Error.");
    };
    HttpResponse::Ok()
}

pub async fn edit_work_category(mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::models::EditWorkCategories;
    use crate::schema::work_categories::dsl::work_categories;

    let _connection = establish_connection();
    let _cat_id : i32 = *_id;
    let _category = work_categories.filter(schema::work_categories::id.eq(_cat_id)).load::<WorkCategories>(&_connection).expect("E");

    let form = category_form(payload.borrow_mut()).await;
    let _new_cat = EditWorkCategories {
        name: form.name.clone(),
        description: Some(form.description.clone()),
        position: form.position.clone(),
        image: Some(form.image.clone()),
        count: _category[0].count,
    };

    diesel::update(&_category[0])
        .set(_new_cat)
        .get_result::<WorkCategories>(&_connection)
        .expect("E");
    HttpResponse::Ok()
}


pub async fn delete_work(_id: web::Path<i32>) -> impl Responder {
    use crate::schema::works::dsl::works;
    use crate::schema::work_category::dsl::work_category;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::work_videos::dsl::work_videos;
    use crate::schema::work_images::dsl::work_images;

    let _connection = establish_connection();
    let _work_id : i32 = *_id;
    let _work = works.filter(schema::works::id.eq(_work_id)).load::<Work>(&_connection).expect("E");

    let _categories = get_cats_for_work(&_work[0]);
    let _tags = get_tags_for_work(&_work[0]);
    for _category in _categories.iter() {
        diesel::update(_category)
            .set(schema::work_categories::count.eq(_category.count - 1))
            .get_result::<WorkCategories>(&_connection)
            .expect("Error.");
    };
    for _tag in _tags.iter() {
        diesel::update(_tag)
            .set((schema::tags::count.eq(_tag.count - 1), schema::tags::work_count.eq(_tag.work_count - 1)))
            .get_result::<Tag>(&_connection)
            .expect("Error.");
    };

    diesel::delete(work_images.filter(schema::work_images::work.eq(_work_id))).execute(&_connection).expect("E");
    diesel::delete(work_videos.filter(schema::work_videos::work.eq(_work_id))).execute(&_connection).expect("E");
    diesel::delete(tags_items.filter(schema::tags_items::work_id.eq(_work_id))).execute(&_connection).expect("E");
    diesel::delete(work_category.filter(schema::work_category::work_id.eq(_work_id))).execute(&_connection).expect("E");
    diesel::delete(&_work[0]).execute(&_connection).expect("E");
    HttpResponse::Ok()
}
pub async fn delete_work_category(_id: web::Path<i32>) -> impl Responder {
    use crate::schema::work_categories::dsl::work_categories;

    let _connection = establish_connection();
    let _cat_id : i32 = *_id;
    let _category = work_categories.filter(schema::work_categories::id.eq(_cat_id)).load::<WorkCategories>(&_connection).expect("E");
    diesel::delete(work_categories.filter(schema::work_categories::id.eq(_cat_id))).execute(&_connection).expect("E");
    HttpResponse::Ok()
}
