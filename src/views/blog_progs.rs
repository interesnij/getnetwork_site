use actix_web::{
    web,
    HttpRequest,
    HttpResponse,
    error::InternalError,
    http::StatusCode,
};
use actix_multipart::{Field, Multipart};
use std::borrow::BorrowMut;
use crate::utils::{
    item_form,
    category_form,
    establish_connection,
};
use actix_session::Session;
use crate::schema;
use crate::diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
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
use sailfish::TemplateOnce;


pub fn blog_routes(config: &mut web::ServiceConfig) {
    config.route("/blog_categories/", web::get().to(blog_categories_page));
    config.service(web::resource("/create_blog_categories/")
        .route(web::get().to(create_blog_categories_page))
        .route(web::post().to(create_blog_categories))
    );
    config.service(web::resource("/edit_blog_category/{id}/")
        .route(web::get().to(edit_blog_category_page))
        .route(web::post().to(edit_blog_category))
    );
    config.service(web::resource("/create_blog/")
        .route(web::get().to(create_blog_page))
        .route(web::post().to(create_blog))
    );
    config.service(web::resource("/edit_blog/{id}/")
        .route(web::get().to(edit_blog_page))
        .route(web::post().to(edit_blog))
    );
    config.route("/edit_content_blog/{id}/", web::get().to(edit_content_blog_page));
    config.route("/delete_blog/{id}/", web::get().to(delete_blog));
    config.route("/delete_blog_category/{id}/", web::get().to(delete_blog_category));
    config.service(web::resource("/blog/{cat_id}/{blog_id}/").route(web::get().to(get_blog_page)));
    config.service(web::resource("/blog/{id}/").route(web::get().to(blog_category_page)));
}

fn get_cats_for_blog(blog: &Blog) -> Vec<BlogCategories> {
    let _connection = establish_connection();

    let ids = BlogCategory::belonging_to(blog)
        .select(schema::blog_category::blog_categories_id);

    schema::blog_categories::table
        .filter(schema::blog_categories::id.eq_any(ids))
        .load::<BlogCategories>(&_connection)
        .expect("E")
}
fn get_tags_for_blog(blog: &Blog) -> Vec<Tag> {
    use crate::schema::tags_items::dsl::tags_items;
    let _connection = establish_connection();

    let _tag_items = tags_items
        .filter(schema::tags_items::blog_id.eq(&blog.id))
        .select(schema::tags_items::tag_id)
        .load::<i32>(&_connection)
        .expect("E");
    schema::tags::table
        .filter(schema::tags::id.eq_any(_tag_items))
        .load::<Tag>(&_connection)
        .expect("E")
}

fn get_6_blog_for_category(category: &BlogCategories) -> Vec<Blog> {
    let _connection = establish_connection();

    let ids = BlogCategory::belonging_to(category).select(schema::blog_category::blog_id);
    schema::blogs::table
        .filter(schema::blogs::id.eq_any(ids))
        .order(schema::blogs::created.desc())
        .limit(6)
        .load::<Blog>(&_connection)
        .expect("E")
}
fn get_blog_for_category(category: &BlogCategories) -> Vec<Blog> {
    let _connection = establish_connection();

    let ids = BlogCategory::belonging_to(category)
        .select(schema::blog_category::blog_id);
    schema::blogs::table
        .filter(schema::blogs::id.eq_any(ids))
        .order(schema::blogs::created.desc())
        .load::<Blog>(&_connection)
        .expect("could not load tags")
}

pub async fn create_blog_categories_page(req: HttpRequest) -> impl Responder {

    HttpResponse::Ok().body(_rendered)
}

pub async fn create_blog_page(req: HttpRequest) -> impl Responder {
    use schema::tags::dsl::tags;


    let _connection = establish_connection();
    let all_tags :Vec<Tag> = tags
        .load(&_connection)
        .expect("Error.");

    data.insert("tags", &all_tags);
    //"blogs/create_blog.html".to_string();
    HttpResponse::Ok().body(_rendered)
}
pub async fn edit_blog_page(req: HttpRequest, _id: web::Path<i32>) -> impl Responder {
    use schema::blogs::dsl::blogs;
    use schema::tags::dsl::tags;
    use crate::schema::blog_images::dsl::blog_images;
    use crate::schema::blog_videos::dsl::blog_videos;

    let _blog_id: i32 = *_id;

    let _connection = establish_connection();
    let _blog = blogs.filter(schema::blogs::id.eq(&_blog_id)).load::<Blog>(&_connection).expect("E");

    let _categories = get_cats_for_blog(&_blog[0]);
    let _all_tags: Vec<Tag> = tags.load(&_connection).expect("Error.");
    let _blog_tags = get_tags_for_blog(&_blog[0]);

    let _images = blog_images.filter(schema::blog_images::blog.eq(_blog[0].id)).load::<BlogImage>(&_connection).expect("E");
    let _videos = blog_videos.filter(schema::blog_videos::blog.eq(_blog[0].id)).load::<BlogVideo>(&_connection).expect("E");


    //"blogs/edit_blog.html".to_string();
    HttpResponse::Ok().body(_rendered)
}

use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct BlogParams {
    content: String,
}
pub async fn edit_content_blog_page(req: HttpRequest, _id: web::Path<i32>) -> impl Responder {
    use schema::blogs::dsl::*;

    let _blog_id: i32 = *_id;
    let _connection = establish_connection();
    let _blog = blogs.filter(schema::blogs::id.eq(&_blog_id)).load::<Blog>(&_connection).expect("E");

    let params = web::Query::<BlogParams>::from_query(&req.query_string()).unwrap();
    if params.content.clone() != "".to_string() {
        diesel::update(&_blog[0])
            .set(schema::blogs::content.eq(&params.content.clone()))
            .get_result::<Blog>(&_connection)
            .expect("E.");
    }

    data.insert("blog", &_blog[0]);

    //"blogs/edit_content_blog.html".to_string();
    HttpResponse::Ok().body(_rendered)
}

pub async fn edit_blog_category_page(req: HttpRequest, _id: web::Path<i32>) -> impl Responder {
    use schema::blog_categories::dsl::*;

    let _cat_id : i32 = *_id;

    let _connection = establish_connection();
    let _category = blog_categories.filter(schema::blog_categories::id.eq(&_cat_id)).load::<BlogCategories>(&_connection).expect("E");

    data.insert("category", &_category[0]);
    HttpResponse::Ok().body(_rendered)
}

pub async fn create_blog_categories(mut payload: Multipart) -> impl Responder {
    let _connection = establish_connection();
    let form = category_form(payload.borrow_mut()).await;
    let new_cat = NewBlogCategories {
        name: form.name.clone(),
        description: Some(form.description.clone()),
        position: form.position,
        image: Some(form.image.clone()),
        count: 0
    };
    let _new_blog = diesel::insert_into(schema::blog_categories::table)
        .values(&new_cat)
        .get_result::<BlogCategories>(&_connection)
        .expect("Error saving post.");
    return HttpResponse::Ok();
}
pub async fn create_blog(mut payload: Multipart) -> impl Responder {
    use crate::schema::tags::dsl::tags;
    use crate::schema::blog_categories::dsl::blog_categories;

    let _connection = establish_connection();

    let form = item_form(payload.borrow_mut()).await;
    let new_blog = NewBlog::from_blog_form (
        form.title.clone(),
        form.description.clone(),
        form.link.clone(),
        form.main_image.clone(),
        form.is_active.clone(),
        1
    );

    let _blog = diesel::insert_into(schema::blogs::table)
        .values(&new_blog)
        .get_result::<Blog>(&_connection)
        .expect("Error saving blog.");

    for image in form.images.iter().enumerate() {
        let new_image = NewBlogImage::from_blog_images_form (
            _blog.id,
            image.1.to_string()
        );
        diesel::insert_into(schema::blog_images::table)
            .values(&new_image)
            .get_result::<BlogImage>(&_connection)
            .expect("Error saving blog.");
        };
    for video in form.videos.iter().enumerate() {
        let new_video = NewBlogVideo::from_blog_videos_form (
            _blog.id,
            video.1.to_string()
        );
        diesel::insert_into(schema::blog_videos::table)
            .values(&new_video)
            .get_result::<BlogVideo>(&_connection)
            .expect("Error saving blog.");
    };
    for category_id in form.category_list.iter().enumerate() {
        let new_category = NewBlogCategory {
            blog_categories_id: *category_id.1,
            blog_id: _blog.id
        };
        diesel::insert_into(schema::blog_category::table)
            .values(&new_category)
            .get_result::<BlogCategory>(&_connection)
            .expect("Error saving blog.");
            let _category = blog_categories.filter(schema::blog_categories::id.eq(category_id.1)).load::<BlogCategories>(&_connection).expect("E");
        diesel::update(&_category[0])
            .set(schema::blog_categories::count.eq(_category[0].count + 1))
            .get_result::<BlogCategories>(&_connection)
            .expect("Error.");
    };
    for tag_id in form.tags_list.iter().enumerate() {
        let new_tag = NewTagItems {
            tag_id: *tag_id.1,
            service_id: 0,
            store_id: 0,
            blog_id: _blog.id,
            wiki_id: 0,
            work_id: 0,
            created: chrono::Local::now().naive_utc(),
        };
        diesel::insert_into(schema::tags_items::table)
            .values(&new_tag)
            .get_result::<TagItems>(&_connection)
            .expect("Error.");
        let _tag = tags.filter(schema::tags::id.eq(tag_id.1)).load::<Tag>(&_connection).expect("E");
        diesel::update(&_tag[0])
            .set((schema::tags::count.eq(_tag[0].count + 1), schema::tags::blog_count.eq(_tag[0].blog_count + 1)))
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
            .set(schema::blog_categories::count.eq(_category.count - 1))
            .get_result::<BlogCategories>(&_connection)
            .expect("Error.");
    };
    for _tag in _tags.iter() {
        diesel::update(_tag)
            .set((schema::tags::count.eq(_tag.count - 1), schema::tags::blog_count.eq(_tag.blog_count - 1)))
            .get_result::<Tag>(&_connection)
            .expect("Error.");
    };

    diesel::delete(blog_images.filter(schema::blog_images::blog.eq(_blog_id))).execute(&_connection).expect("E");
    diesel::delete(blog_videos.filter(schema::blog_videos::blog.eq(_blog_id))).execute(&_connection).expect("E");
    diesel::delete(tags_items.filter(schema::tags_items::blog_id.eq(_blog_id))).execute(&_connection).expect("E");
    diesel::delete(blog_category.filter(schema::blog_category::blog_id.eq(_blog_id))).execute(&_connection).expect("E");

    let form = item_form(payload.borrow_mut()).await;
    let _new_blog = EditBlog {
        title: form.title.clone(),
        description: Some(form.description.clone()),
        link: Some(form.link.clone()),
        image: Some(form.main_image.clone()),
        is_active: form.is_active.clone()
    };

    diesel::update(&_blog[0])
        .set(_new_blog)
        .get_result::<Blog>(&_connection)
        .expect("E");

    for _image in form.images.iter().enumerate() {
        let new_edit_image = NewBlogImage::from_blog_images_form (
            _blog_id,
            _image.1.to_string()
        );
        diesel::insert_into(schema::blog_images::table)
            .values(&new_edit_image)
            .get_result::<BlogImage>(&_connection)
            .expect("Error saving blog.");
        };
    for _video in form.videos.iter().enumerate() {
        let new_video = NewBlogVideo::from_blog_videos_form (
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
            .set(schema::blog_categories::count.eq(_category_2[0].count + 1))
            .get_result::<BlogCategories>(&_connection)
            .expect("Error.");
    };
    for _tag_id in form.tags_list.iter() {
        let _new_tag = NewTagItems {
            tag_id: *_tag_id.1,
            service_id: 0,
            store_id: 0,
            blog_id: _blog_id,
            wiki_id: 0,
            work_id: 0,
            created: chrono::Local::now().naive_utc(),
        };
        diesel::insert_into(schema::tags_items::table)
            .values(&_new_tag)
            .get_result::<TagItems>(&_connection)
            .expect("Error.");
        let _tag_2 = tags.filter(schema::tags::id.eq(_tag_id.1)).load::<Tag>(&_connection).expect("E");
        diesel::update(&_tag_2[0])
            .set((schema::tags::count.eq(_tag_2[0].count + 1), schema::tags::blog_count.eq(_tag_2[0].blog_count + 1)))
            .get_result::<Tag>(&_connection)
            .expect("Error.");
    };
    HttpResponse::Ok()
}

pub async fn edit_blog_category(mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::models::EditBlogCategories;
    use crate::schema::blog_categories::dsl::blog_categories;

    let _connection = establish_connection();
    let _cat_id: i32 = *_id;
    let _category = blog_categories.filter(schema::blog_categories::id.eq(_cat_id)).load::<BlogCategories>(&_connection).expect("E");

    let form = category_form(payload.borrow_mut()).await;
    let _new_cat = EditBlogCategories {
        name: form.name.clone(),
        description: Some(form.description.clone()),
        position: form.position,
        image: Some(form.image.clone()),
        count: _category[0].count,
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
    let _blog_id: i32 = *_id;
    let _blog = blogs.filter(schema::blogs::id.eq(_blog_id)).load::<Blog>(&_connection).expect("E");

    let _categories = get_cats_for_blog(&_blog[0]);
    let _tags = get_tags_for_blog(&_blog[0]);
    for _category in _categories.iter() {
        diesel::update(_category)
            .set(schema::blog_categories::count.eq(_category.count - 1))
            .get_result::<BlogCategories>(&_connection)
            .expect("Error.");
    };
    for _tag in _tags.iter() {
        diesel::update(_tag)
            .set((schema::tags::count.eq(_tag.count - 1), schema::tags::blog_count.eq(_tag.blog_count - 1)))
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
    let _cat_id: i32 = *_id;
    let _category = blog_categories.filter(schema::blog_categories::id.eq(_cat_id)).load::<BlogCategories>(&_connection).expect("E");
    diesel::delete(blog_categories.filter(schema::blog_categories::id.eq(_cat_id))).execute(&_connection).expect("E");
    HttpResponse::Ok()
}

pub async fn get_blog_page(req: HttpRequest, param: web::Path<(i32,i32)>) -> impl Responder {
    use schema::blogs::dsl::blogs;
    use schema::blog_categories::dsl::blog_categories;
    use schema::blog_images::dsl::blog_images;
    use schema::blog_videos::dsl::blog_videos;

    let _connection = establish_connection();
    let _blog_id: i32 = param.1;
    let _cat_id: i32 = param.0;

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

    let _category_blogs = get_blog_for_category(&_s_category[0]);
    let _category_blogs_len: usize = _category_blogs.len();
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

    data.insert("object", &_blog[0]);
    data.insert("images", &_images);
    data.insert("videos", &_videos);
    data.insert("categories", &_categories);
    data.insert("category", &_s_category[0]);
    data.insert("tags", &_tags);
    data.insert("tags_count", &_tags.len());
    //"blogs/blog.html".to_string();
    HttpResponse::Ok().body(_rendered)
}

pub async fn blog_category_page(req: HttpRequest, _id: web::Path<i32>) -> impl Responder {
    use schema::blog_categories::dsl::blog_categories;
    use crate::schema::tags_items::dsl::tags_items;

    let _cat_id: i32 = *_id;
    let page_size = 20;
    let mut offset = 0;

    let _connection = establish_connection();

    let _category = blog_categories.filter(schema::blog_categories::id.eq(_cat_id)).load::<BlogCategories>(&_connection).expect("E");
    loop {
        let ids = BlogCategory::belonging_to(&_category).select(schema::blog_category::blog_id);
        let _blogs = schema::blogs::table
        .filter(schema::blogs::id.eq_any(ids))
        .limit(page_size)
        .offset(offset)
        .order(schema::blogs::created.desc())
        .load::<Blog>(&_connection)
        .expect("could not load tags");
        if _blogs.len() > 0 {
            data.insert("blogs", &_blogs);
            offset += page_size;
        }
        else {break;}
    };

    let mut stack = Vec::new();
    let _tag_items = tags_items
        .filter(schema::tags_items::blog_id.ne(0))
        .select(schema::tags_items::tag_id)
        .load::<i32>(&_connection)
        .expect("E");
    for _tag_item in _tag_items.iter() {
        if !stack.iter().any(|&i| i==_tag_item) {
            stack.push(_tag_item);
        }
    };
    let _tags = schema::tags::table
        .filter(schema::tags::id.eq_any(stack))
        .load::<Tag>(&_connection)
        .expect("could not load tags");

    data.insert("tags", &_tags);
    data.insert("tags_count", &_tags.len());
    data.insert("category", &_category[0]);

    //"blogs/category.html".to_string();
    HttpResponse::Ok().body(_rendered)
}

pub async fn blog_categories_page(req: HttpRequest) -> impl Responder {
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::blogs::dsl::blogs;

    let _connection = establish_connection();

    let _blogs = blogs.filter(schema::blogs::is_active.eq(true)).load::<Blog>(&_connection).expect("E");
    let mut _count: i32 = 0;
    for _cat in _blog_cats.iter() {
        _count += 1;
        // для генерации переменной 1 2 3
        let mut _let_int: String = _count.to_string().parse().unwrap();
        let _let_data_blogs: String = "blogs".to_string() + &_let_int;
        data.insert(&_let_data_blogs, &get_6_blog_for_category(_cat));
    };


    let mut stack = Vec::new();
    for blog in _blogs.iter() {
        let _tag_items = tags_items
            .filter(schema::tags_items::blog_id.eq(blog.id))
            .select(schema::tags_items::tag_id)
            .load::<i32>(&_connection)
            .expect("E");
        for _tag_item in _tag_items.iter() {
            if !stack.iter().any(|&i| i==_tag_item) {
                stack.push(_tag_item);
            }
        };
    };
    let _tags = schema::tags::table
        .filter(schema::tags::id.eq_any(stack))
        .load::<Tag>(&_connection)
        .expect("could not load tags");

    data.insert("tags", &_tags);
    data.insert("tags_count", &_tags.len());

    //"blogs/categories.html".to_string();
    HttpResponse::Ok().body(_rendered)
}
