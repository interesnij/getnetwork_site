extern crate diesel;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use tera::{Tera,Context,Function};
use actix_multipart::Multipart;
use std::borrow::BorrowMut;
use diesel::prelude::*;
use crate::utils::{
    split_payload,
    category_split_payload,
    get_template_2,
    establish_connection
};
use crate::schema;
use crate::models::{
    BlogCategories,
    NewBlogCategories,
    Blog,
    NewBlog,
    BlogCategory,
    NewBlogCategory,
    BlogImage,
    NewBlogImage,
    BlogVideo,
    NewBlogVideo,
    TagItems,
    NewTagItems,
    Tag,
};

fn get_cats_for_blog(blog: &Blog) -> Vec<BlogCategories> {
    use diesel::pg::expression::dsl::any;
    let _connection = establish_connection();

    let ids = BlogCategory::belonging_to(blog).select(schema::blog_category::blog_categories_id);
    schema::blog_categories::table
        .filter(schema::blog_categories::id.eq(any(ids)))
        .load::<BlogCategories>(&_connection)
        .expect("could not load tags")
}
fn get_tags_for_blog(blog: &Blog) -> Vec<Tag> {
    use crate::schema::tags_items::dsl::tags_items;
    use diesel::dsl::any;
    let _connection = establish_connection();

    let _tag_items = tags_items.filter(schema::tags_items::blog_id.eq(&blog.id)).load::<TagItems>(&_connection).expect("E");
    let mut stack = Vec::new();
    for _tag_item in _tag_items.iter() {
        stack.push(_tag_item.tag_id);
    };
    schema::tags::table
        .filter(schema::tags::id.eq(any(stack)))
        .load::<Tag>(&_connection)
        .expect("could not load tags")
}

fn get_6_blog_for_category(category: &BlogCategories) -> Vec<Blog> {
    use diesel::pg::expression::dsl::any;
    let _connection = establish_connection();

    let ids = BlogCategory::belonging_to(category).select(schema::blog_category::blog_id);
    schema::blogs::table
        .filter(schema::blogs::id.eq(any(ids)))
        .order(schema::blogs::blog_created.desc())
        .limit(6)
        .load::<Blog>(&_connection)
        .expect("could not load tags")
}
fn get_blog_for_category(category: &BlogCategories) -> Vec<Blog> {
    use diesel::pg::expression::dsl::any;
    let _connection = establish_connection();

    let ids = BlogCategory::belonging_to(category).select(schema::blog_category::blog_id);
    schema::blogs::table
        .filter(schema::blogs::id.eq(any(ids)))
        .order(schema::blogs::blog_created.desc())
        .load::<Blog>(&_connection)
        .expect("could not load tags")
}

pub async fn create_blog_categories_page(req: HttpRequest, tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);
    let _template = _type + &"blogs/create_categories.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn create_blog_page(req: HttpRequest, tera: web::Data<Tera>) -> impl Responder {
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
    let _template = _type + &"blogs/create_blog.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}
pub async fn edit_blog_page(req: HttpRequest, tera: web::Data<Tera>, _id: web::Path<i32>) -> impl Responder {
    use schema::blogs::dsl::*;
    use schema::tags::dsl::*;
    use crate::schema::blog_images::dsl::blog_images;
    use crate::schema::blog_videos::dsl::blog_videos;

    let _blog_id : i32 = *_id;
    let mut data = Context::new();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);
    let _connection = establish_connection();
    let _blog = blogs.filter(schema::blogs::id.eq(&_blog_id)).load::<Blog>(&_connection).expect("E");

    let _categories = get_cats_for_blog(&_blog[0]);
    let _all_tags :Vec<Tag> = tags.load(&_connection).expect("Error.");
    let _blog_tags = get_tags_for_blog(&_blog[0]);

    let _images = blog_images.filter(schema::blog_images::blog.eq(_blog[0].id)).load::<BlogImage>(&_connection).expect("E");
    let _videos = blog_videos.filter(schema::blog_videos::blog.eq(_blog[0].id)).load::<BlogVideo>(&_connection).expect("E");

    data.insert("blog", &_blog[0]);
    data.insert("blog_tags", &_blog_tags);
    data.insert("all_tags", &_all_tags);
    data.insert("categories", &_categories);
    data.insert("images", &_images);
    data.insert("videos", &_videos);

    let _template = _type + &"blogs/edit_blog.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn edit_content_blog_page(req: HttpRequest, tera: web::Data<Tera>, _id: web::Path<i32>) -> impl Responder {
    use schema::blogs::dsl::*;

    let _blog_id : i32 = *_id;
    let mut data = Context::new();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);
    let _connection = establish_connection();
    let _blog = blogs.filter(schema::blogs::id.eq(&_blog_id)).load::<Blog>(&_connection).expect("E");

    data.insert("blog", &_blog[0]);

    let _template = _type + &"blogs/edit_content_blog.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn edit_blog_category_page(req: HttpRequest, tera: web::Data<Tera>, _id: web::Path<i32>) -> impl Responder {
    use schema::blog_categories::dsl::*;

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
    let _category = blog_categories.filter(schema::blog_categories::id.eq(&_cat_id)).load::<BlogCategories>(&_connection).expect("E");

    data.insert("category", &_category[0]);
    let _template = _type + &"blogs/edit_category.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn create_blog_categories(mut payload: Multipart) -> impl Responder {
    use schema::blog_categories;

    let _connection = establish_connection();
    let form = category_split_payload(payload.borrow_mut()).await;
    let new_cat = NewBlogCategories {
        name: form.name.clone(),
        blog_position: form.position.clone(),
        image: Some(form.image.clone()),
        blog_count: 0
    };
    let _new_blog = diesel::insert_into(blog_categories::table)
        .values(&new_cat)
        .get_result::<BlogCategories>(&_connection)
        .expect("Error saving post.");
    return HttpResponse::Ok();
}
pub async fn create_blog(mut payload: Multipart) -> impl Responder {
    use schema::{blogs,blog_images,blog_videos,blog_category,tags_items};
    use crate::schema::tags::dsl::tags;
    use crate::schema::blog_categories::dsl::blog_categories;

    let _connection = establish_connection();

    let form = split_payload(payload.borrow_mut()).await;
    let new_blog = NewBlog::from_blog_form(
        form.title.clone(),
        form.description.clone(),
        form.link.clone(),
        form.main_image.clone(),
        form.is_active.clone(),
        1
    );

    let _blog = diesel::insert_into(blogs::table)
        .values(&new_blog)
        .get_result::<Blog>(&_connection)
        .expect("Error saving blog.");

    for image in form.images.iter().enumerate() {
        let new_image = NewBlogImage::from_blog_images_form(
            _blog.id,
            image.1.to_string()
        );
        diesel::insert_into(blog_images::table)
            .values(&new_image)
            .get_result::<BlogImage>(&_connection)
            .expect("Error saving blog.");
        };
    for video in form.videos.iter().enumerate() {
        let new_video = NewBlogVideo::from_blog_videos_form(
            _blog.id,
            video.1.to_string()
        );
        diesel::insert_into(blog_videos::table)
            .values(&new_video)
            .get_result::<BlogVideo>(&_connection)
            .expect("Error saving blog.");
    };
    for category_id in form.category_list.iter().enumerate() {
        let new_category = NewBlogCategory {
            blog_categories_id: *category_id.1,
            blog_id: _blog.id
        };
        diesel::insert_into(blog_category::table)
            .values(&new_category)
            .get_result::<BlogCategory>(&_connection)
            .expect("Error saving blog.");
            let _category = blog_categories.filter(schema::blog_categories::id.eq(category_id.1)).load::<BlogCategories>(&_connection).expect("E");
        diesel::update(&_category[0])
            .set(schema::blog_categories::blog_count.eq(_category[0].blog_count + 1))
            .get_result::<BlogCategories>(&_connection)
            .expect("Error.");
    };
    for tag_id in form.tags_list.iter().enumerate() {
        let new_tag = NewTagItems{
            tag_id: *tag_id.1,
            service_id: 0,
            store_id: 0,
            blog_id: _blog.id,
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
            .set((schema::tags::tag_count.eq(_tag[0].tag_count + 1), schema::tags::blog_count.eq(_tag[0].blog_count + 1)))
            .get_result::<Tag>(&_connection)
            .expect("Error.");
    };
    HttpResponse::Ok()
}

pub async fn edit_blog(mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::models::EditBlog;
    use crate::schema::blogs::dsl::blogs;
    use crate::schema::blog_category::dsl::blog_category;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::blog_videos::dsl::blog_videos;
    use crate::schema::blog_images::dsl::blog_images;
    use crate::schema::blog_categories::dsl::blog_categories;
    use crate::schema::tags::dsl::tags;

    let _connection = establish_connection();
    let _blog_id : i32 = *_id;
    let _blog = blogs.filter(schema::blogs::id.eq(_blog_id)).load::<Blog>(&_connection).expect("E");

    let _categories = get_cats_for_blog(&_blog[0]);
    let _tags = get_tags_for_blog(&_blog[0]);
    for _category in _categories.iter() {
        diesel::update(_category)
            .set(schema::blog_categories::blog_count.eq(_category.blog_count - 1))
            .get_result::<BlogCategories>(&_connection)
            .expect("Error.");
    };
    for _tag in _tags.iter() {
        diesel::update(_tag)
            .set((schema::tags::tag_count.eq(_tag.tag_count - 1), schema::tags::blog_count.eq(_tag.blog_count - 1)))
            .get_result::<Tag>(&_connection)
            .expect("Error.");
    };

    diesel::delete(blog_images.filter(schema::blog_images::blog.eq(_blog_id))).execute(&_connection).expect("E");
    diesel::delete(blog_videos.filter(schema::blog_videos::blog.eq(_blog_id))).execute(&_connection).expect("E");
    diesel::delete(tags_items.filter(schema::tags_items::blog_id.eq(_blog_id))).execute(&_connection).expect("E");
    diesel::delete(blog_category.filter(schema::blog_category::blog_id.eq(_blog_id))).execute(&_connection).expect("E");

    let form = split_payload(payload.borrow_mut()).await;
    let _new_blog = EditBlog {
        title: form.title.clone(),
        description: Some(form.description.clone()),
        link: Some(form.link.clone()),
        image: Some(form.main_image.clone()),
        is_blog_active: form.is_active.clone()
    };

    diesel::update(&_blog[0])
        .set(_new_blog)
        .get_result::<Blog>(&_connection)
        .expect("E");

    for _image in form.images.iter().enumerate() {
        let new_edit_image = NewBlogImage::from_blog_images_form(
            _blog_id,
            _image.1.to_string()
        );
        diesel::insert_into(schema::blog_images::table)
            .values(&new_edit_image)
            .get_result::<BlogImage>(&_connection)
            .expect("Error saving blog.");
        };
    for _video in form.videos.iter().enumerate() {
        let new_video = NewBlogVideo::from_blog_videos_form(
            _blog_id,
            _video.1.to_string()
        );
        diesel::insert_into(schema::blog_videos::table)
            .values(&new_video)
            .get_result::<BlogVideo>(&_connection)
            .expect("Error saving blog.");
    };
    for category_id in form.category_list.iter().enumerate() {
        let new_category = NewBlogCategory {
            blog_categories_id: *category_id.1,
            blog_id: _blog_id
        };
        diesel::insert_into(schema::blog_category::table)
            .values(&new_category)
            .get_result::<BlogCategory>(&_connection)
            .expect("Error saving blog.");
        let _category_2 = blog_categories.filter(schema::blog_categories::id.eq(category_id.1)).load::<BlogCategories>(&_connection).expect("E");
        diesel::update(&_category_2[0])
            .set(schema::blog_categories::blog_count.eq(_category_2[0].blog_count + 1))
            .get_result::<BlogCategories>(&_connection)
            .expect("Error.");
    };
    for _tag_id in form.tags_list.iter().enumerate() {
        let _new_tag = NewTagItems{
            tag_id: *_tag_id.1,
            service_id: 0,
            store_id: 0,
            blog_id: _blog_id,
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
            .set((schema::tags::tag_count.eq(_tag_2[0].tag_count + 1), schema::tags::blog_count.eq(_tag_2[0].blog_count + 1)))
            .get_result::<Tag>(&_connection)
            .expect("Error.");
    };
    HttpResponse::Ok()
}

use serde::Deserialize;
#[derive(Deserialize)]
pub struct BlogContent {
    pub content: String,
}
pub async fn edit_content_blog(form: web::Form<BlogContent>, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::blogs::dsl::blogs;

    let _connection = establish_connection();
    let _blog_id : i32 = *_id;
    let _blog = blogs.filter(schema::blogs::id.eq(_blog_id)).load::<Blog>(&_connection).expect("E");

    let _new_content = Some(form.content.clone());
    diesel::update(&_blog[0])
    .set(schema::blogs::content.eq(&_new_content))
    .get_result::<Blog>(&_connection)
    .expect("Error.");

    HttpResponse::Ok()
}

pub async fn edit_blog_category(mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::models::EditBlogCategories;
    use crate::schema::blog_categories::dsl::blog_categories;

    let _connection = establish_connection();
    let _cat_id : i32 = *_id;
    let _category = blog_categories.filter(schema::blog_categories::id.eq(_cat_id)).load::<BlogCategories>(&_connection).expect("E");

    let form = category_split_payload(payload.borrow_mut()).await;
    let _new_cat = EditBlogCategories {
        name: form.name.clone(),
        blog_position: form.position.clone(),
        image: Some(form.image.clone()),
        blog_count: _category[0].blog_count,
    };

    diesel::update(&_category[0])
        .set(_new_cat)
        .get_result::<BlogCategories>(&_connection)
        .expect("E");
    HttpResponse::Ok()
}

pub async fn delete_blog(_id: web::Path<i32>) -> impl Responder {
    use crate::schema::blogs::dsl::blogs;
    use crate::schema::blog_category::dsl::blog_category;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::blog_videos::dsl::blog_videos;
    use crate::schema::blog_images::dsl::blog_images;

    let _connection = establish_connection();
    let _blog_id : i32 = *_id;
    let _blog = blogs.filter(schema::blogs::id.eq(_blog_id)).load::<Blog>(&_connection).expect("E");

    let _categories = get_cats_for_blog(&_blog[0]);
    let _tags = get_tags_for_blog(&_blog[0]);
    for _category in _categories.iter() {
        diesel::update(_category)
            .set(schema::blog_categories::blog_count.eq(_category.blog_count - 1))
            .get_result::<BlogCategories>(&_connection)
            .expect("Error.");
    };
    for _tag in _tags.iter() {
        diesel::update(_tag)
            .set((schema::tags::tag_count.eq(_tag.tag_count - 1), schema::tags::blog_count.eq(_tag.blog_count - 1)))
            .get_result::<Tag>(&_connection)
            .expect("Error.");
    };

    diesel::delete(blog_images.filter(schema::blog_images::blog.eq(_blog_id))).execute(&_connection).expect("E");
    diesel::delete(blog_videos.filter(schema::blog_videos::blog.eq(_blog_id))).execute(&_connection).expect("E");
    diesel::delete(tags_items.filter(schema::tags_items::blog_id.eq(_blog_id))).execute(&_connection).expect("E");
    diesel::delete(blog_category.filter(schema::blog_category::blog_id.eq(_blog_id))).execute(&_connection).expect("E");
    diesel::delete(&_blog[0]).execute(&_connection).expect("E");
    HttpResponse::Ok()
}
pub async fn delete_blog_category(_id: web::Path<i32>) -> impl Responder {
    use crate::schema::blog_categories::dsl::blog_categories;

    let _connection = establish_connection();
    let _cat_id : i32 = *_id;
    let _category = blog_categories.filter(schema::blog_categories::id.eq(_cat_id)).load::<BlogCategories>(&_connection).expect("E");
    diesel::delete(blog_categories.filter(schema::blog_categories::id.eq(_cat_id))).execute(&_connection).expect("E");
    HttpResponse::Ok()
}

pub async fn get_blog_page(req: HttpRequest, tera: web::Data<Tera>, param: web::Path<(i32,i32)>) -> impl Responder {
    use schema::blogs::dsl::blogs;
    use schema::blog_categories::dsl::blog_categories;
    use schema::blog_images::dsl::blog_images;
    use schema::blog_videos::dsl::blog_videos;

    let _connection = establish_connection();
    let _blog_id : i32 = param.1;
    let _cat_id : i32 = param.0;

    let _blog = blogs
        .filter(schema::blogs::id.eq(&_blog_id))
        .load::<Blog>(&_connection)
        .expect("E");
    let _s_category = blog_categories
        .filter(schema::blog_categories::id.eq(&_cat_id))
        .load::<BlogCategories>(&_connection)
        .expect("E");

    let _images :Vec<BlogImage> = blog_images.filter(schema::blog_images::blog.eq(&_blog_id)).load(&_connection).expect("E");
    let _videos :Vec<BlogVideo> = blog_videos.filter(schema::blog_videos::blog.eq(&_blog_id)).load(&_connection).expect("E");
    let _categories = get_cats_for_blog(&_blog[0]);
    let _tags = get_tags_for_blog(&_blog[0]);

    let mut data = Context::new();

    let _category_blogs = get_blog_for_category(&_s_category[0]);
    let _category_blogs_len : usize = _category_blogs.len();
    for (i, item) in _category_blogs.iter().enumerate().rev() {
        if item.id == _blog_id {
            if (i + 1) != _category_blogs_len {
                let _prev = Some(&_category_blogs[i + 1]);
                data.insert("prev", &_prev);
            };
            if i != 0 {
                let _next = Some(&_category_blogs[i - 1]);
                data.insert("next", &_next);
            };
            break;
        }
    };

    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("object", &_blog[0]);
    data.insert("images", &_images);
    data.insert("videos", &_videos);
    data.insert("categories", &_categories);
    data.insert("category", &_s_category[0]);
    data.insert("tags", &_tags);
    data.insert("tags_count", &_tags.len());
    data.insert("is_admin", &_is_admin);
    let _template = _type + &"blogs/blog.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn blog_category_page(req: HttpRequest, tera: web::Data<Tera>, _id: web::Path<i32>) -> impl Responder {
    use schema::blog_categories::dsl::blog_categories;
    use crate::schema::tags_items::dsl::tags_items;
    use diesel::dsl::any;

    let _cat_id : i32 = *_id;
    let page_size = 20;
    let mut offset = 0;

    let mut data = Context::new();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);
    let _connection = establish_connection();

    let _category = blog_categories.filter(schema::blog_categories::id.eq(_cat_id)).load::<BlogCategories>(&_connection).expect("E");
    loop {
        let ids = BlogCategory::belonging_to(&_category).select(schema::blog_category::blog_id);
        let _blogs = schema::blogs::table
        .filter(schema::blogs::id.eq(any(ids)))
        .limit(page_size)
        .offset(offset)
        .order(schema::blogs::blog_created.desc())
        .load::<Blog>(&_connection)
        .expect("could not load tags");
         if _blogs.len() <= 0 { break;}
         offset += page_size;
         data.insert("blogs", &_blogs);
    };

    let mut stack = Vec::new();
    let _tag_items = tags_items.filter(schema::tags_items::blog_id.ne(0)).load::<TagItems>(&_connection).expect("E");
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

    data.insert("category", &_category[0]);

    let _template = _type + &"blogs/category.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn blog_categories_page(req: HttpRequest, tera: web::Data<Tera>) -> impl Responder {
    use diesel::dsl::any;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::blogs::dsl::blogs;

    let _connection = establish_connection();

    let mut data = Context::new();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);

    let _blogs = blogs.filter(schema::blogs::is_blog_active.eq(true)).load::<Blog>(&_connection).expect("E");
    let mut _count: i32 = 0;
    for _cat in _blog_cats.iter() {
        _count += 1;
        // для генерации переменной 1 2 3
        let mut _let_int : String = _count.to_string().parse().unwrap();
        let _let_data_blogs: String = "blogs".to_string() + &_let_int;
        data.insert(&_let_data_blogs, &get_6_blog_for_category(_cat));
    };


    let mut stack = Vec::new();
    for blog in _blogs.iter() {
        let _tag_items = tags_items.filter(schema::tags_items::blog_id.eq(blog.id)).load::<TagItems>(&_connection).expect("E");
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

    let _template = _type + &"blogs/categories.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}
