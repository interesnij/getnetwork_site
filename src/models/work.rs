use crate::schema::*;
use diesel::{Queryable, Insertable};
use serde::{
    Serialize,
    Deserialize
};

use crate::models::User;

#[derive(Debug, Serialize, Identifiable, Queryable, Associations)]
#[table_name="work_categories"]
pub struct WorkCategories {
    pub id: i32,
    pub name: String,
    pub work_position: i32,
    pub image: Option<String>,
    pub work_count: i32,
}
#[derive(Insertable)]
#[table_name="work_categories"]
pub struct NewWorkCategories {
    pub name: String,
    pub work_position: i32,
    pub image: Option<String>,
    pub work_count: i32,
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="work_categories"]
pub struct EditWorkCategories {
    pub name: String,
    pub work_position: i32,
    pub image: Option<String>,
    pub work_count: i32,
}

#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(User, foreign_key="creator")]
pub struct Work {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub content: Option<String>,
    pub link: Option<String>,
    pub image: Option<String>,
    pub is_work_active: bool,
    pub creator: i32,
    pub work_created: chrono::NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="works"]
pub struct EditWork {
    pub title: String,
    pub description: Option<String>,
    pub link: Option<String>,
    pub image: Option<String>,
    pub is_work_active: bool,
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(WorkCategories)]
#[belongs_to(Work)]
#[table_name="work_category"]
pub struct WorkCategory {
    pub id: i32,
    pub work_categories_id: i32,
    pub work_id: i32,
}

#[derive(Insertable)]
#[table_name="work_category"]
pub struct NewWorkCategory {
    pub work_categories_id: i32,
    pub work_id: i32,
}

#[derive(Serialize, Insertable)]
#[table_name="works"]
pub struct NewWork {
    pub title: String,
    pub description: Option<String>,
    pub link: Option<String>,
    pub image: Option<String>,
    pub is_work_active: bool,
    pub creator: i32,
    pub work_created: chrono::NaiveDateTime,
}

impl NewWork {
    pub fn from_work_form(
        title: String,
        description: String,
        link: String,
        image: String,
        is_work_active: bool,
        creator_id: i32
    ) -> Self {
        NewWork {
            title: title,
            description: Some(description),
            link: Some(link),
            image: Some(image),
            is_work_active: is_work_active,
            creator: creator_id,
            work_created: chrono::Local::now().naive_utc(),
        }
    }
}

#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(Work, foreign_key="work")]
pub struct WorkImage {
    pub id: i32,
    pub work: i32,
    pub src: String,
}

#[derive(Serialize, Insertable)]
#[table_name="work_images"]
pub struct NewWorkImage {
    pub work: i32,
    pub src: String,
}

impl NewWorkImage {
    pub fn from_work_images_form(
        work_id: i32, src: String) -> Self {
        NewWorkImage {
            work: work_id,
            src: src,
        }
    }
}

#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(Work, foreign_key="work")]
pub struct WorkVideo {
    pub id: i32,
    pub work: i32,
    pub src: String,
}

#[derive(Serialize, Insertable)]
#[table_name="work_videos"]
pub struct NewWorkVideo {
    pub work: i32,
    pub src: String,
}

impl NewWorkVideo {
    pub fn from_work_videos_form(
        work_id: i32, src: String) -> Self {
        NewWorkVideo {
            work: work_id,
            src: src,
        }
    }
}
