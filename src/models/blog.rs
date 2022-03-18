use crate::schema::*;
use diesel::{Queryable, Insertable};
use serde::{
    Serialize,
    Deserialize
};

use crate::models::User;

#[derive(Debug, Serialize, Identifiable, Queryable, Associations)]
#[table_name="blog_categories"]
pub struct BlogCategories {
    pub id: i32,
    pub name: String,
    pub blog_position: i32,
    pub image: Option<String>,
    pub blog_count: i32,
}

#[derive(Insertable)]
#[table_name="blog_categories"]
pub struct NewBlogCategories {
    pub name: String,
    pub blog_position: i32,
    pub image: Option<String>,
    pub blog_count: i32,
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="blog_categories"]
pub struct EditBlogCategories {
    pub name: String,
    pub blog_position: i32,
    pub image: Option<String>,
    pub blog_count: i32,
}

#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(User, foreign_key="creator")]
pub struct Blog {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub content: Option<String>,
    pub link: Option<String>,
    pub image: Option<String>,
    pub is_blog_active: bool,
    pub creator: i32,
    pub blog_created: chrono::NaiveDateTime,
}

#[derive(Serialize, Insertable)]
#[table_name="blogs"]
pub struct NewBlog {
    pub title: String,
    pub description: Option<String>,
    pub content: Option<String>,
    pub link: Option<String>,
    pub image: Option<String>,
    pub is_blog_active: bool,
    pub creator: i32,
    pub blog_created: chrono::NaiveDateTime,
}

impl NewBlog {
    pub fn from_blog_form(
        title: String,
        description: String,
        content: String,
        link: String,
        image: String,
        is_blog_active: bool,
        creator_id: i32) -> Self {
        NewBlog {
            title: title,
            description: Some(description),
            content: Some(content),
            link: Some(link),
            image: Some(image),
            is_blog_active: is_blog_active,
            creator: creator_id,
            blog_created: chrono::Local::now().naive_utc(),
        }
    }
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="blogs"]
pub struct EditBlog {
    pub title: String,
    pub description: Option<String>,
    pub link: Option<String>,
    pub image: Option<String>,
    pub is_blog_active: bool,
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(BlogCategories)]
#[belongs_to(Blog)]
#[table_name="blog_category"]
pub struct BlogCategory {
    pub id: i32,
    pub blog_categories_id: i32,
    pub blog_id: i32,
}

#[derive(Insertable)]
#[table_name="blog_category"]
pub struct NewBlogCategory {
    pub blog_categories_id: i32,
    pub blog_id: i32,
}

#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(User, foreign_key="blog")]
pub struct BlogImage {
    pub id: i32,
    pub blog: i32,
    pub src: String
}

#[derive(Serialize, Insertable)]
#[table_name="blog_images"]
pub struct NewBlogImage {
    pub blog: i32,
    pub src: String
}

impl NewBlogImage {
    pub fn from_blog_images_form(
        blog_id: i32, src: String) -> Self {
        NewBlogImage {
            blog: blog_id,
            src: src
        }
    }
}

#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(User, foreign_key="blog")]
pub struct BlogVideo {
    pub id: i32,
    pub blog: i32,
    pub src: String
}

#[derive(Serialize, Insertable)]
#[table_name="blog_videos"]
pub struct NewBlogVideo {
    pub blog: i32,
    pub src: String
}

impl NewBlogVideo {
    pub fn from_blog_videos_form(
        blog_id: i32, src: String) -> Self {
        NewBlogVideo {
            blog: blog_id,
            src: src
        }
    }
}

#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(Blog)]
#[belongs_to(User)]
pub struct BlogComment {
    pub id: i32,
    pub comment: String,
    pub blog_id: i32,
    pub user_id: i32,
    pub parent_comment_id: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Insertable)]
#[table_name="blog_comments"]
pub struct NewBlogComment {
    pub comment: String,
    pub blog_id: i32,
    pub user_id: i32,
    pub parent_comment_id: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
}

impl NewBlogComment {
    pub fn new(comment: String, blog_id: i32,
        user_id: i32, parent_comment_id: Option<i32>) -> Self{
        NewBlogComment {
            comment: comment,
            blog_id: blog_id,
            user_id: user_id,
            parent_comment_id: parent_comment_id,
            created_at: chrono::Local::now().naive_utc(),
        }
    }
}
