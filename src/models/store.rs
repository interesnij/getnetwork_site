use crate::schema::*;
use diesel::{Queryable, Insertable};
use serde::{
    Serialize,
    Deserialize,
};

use crate::models::User;

#[derive(Debug, Serialize, Identifiable, Queryable, Associations)]
#[table_name="store_categories"]
pub struct StoreCategories {
    pub id: i32,
    pub name: String,
    pub store_position: i32,
    pub image: Option<String>,
    pub store_count: i32,
}
#[derive(Insertable)]
#[table_name="store_categories"]
pub struct NewStoreCategories {
    pub name: String,
    pub store_position: i32,
    pub image: Option<String>,
    pub store_count: i32,
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="store_categories"]
pub struct EditStoreCategories {
    pub name: String,
    pub store_position: i32,
    pub image: Option<String>,
    pub store_count: i32,
}

#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(User, foreign_key="creator")]
pub struct Store {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub content: Option<String>,
    pub link: Option<String>,
    pub image: Option<String>,
    pub is_store_active: bool,
    pub creator: i32,
    pub store_created: chrono::NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="stores"]
pub struct EditStore {
    pub title: String,
    pub description: Option<String>,
    pub content: Option<String>,
    pub link: Option<String>,
    pub image: Option<String>,
    pub is_store_active: bool,
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(StoreCategories)]
#[belongs_to(Store)]
#[table_name="store_category"]
pub struct StoreCategory {
    pub id: i32,
    pub store_categories_id: i32,
    pub store_id: i32,
}

#[derive(Insertable)]
#[table_name="store_category"]
pub struct NewStoreCategory {
    pub store_categories_id: i32,
    pub store_id: i32,
}

#[derive(Serialize, Insertable)]
#[table_name="stores"]
pub struct NewStore {
    pub title: String,
    pub description: Option<String>,
    pub content: Option<String>,
    pub link: Option<String>,
    pub image: Option<String>,
    pub is_store_active: bool,
    pub creator: i32,
    pub store_created: chrono::NaiveDateTime,
}

impl NewStore {
    pub fn from_store_form(
        title: String,
        description: String,
        content: String,
        link: String,
        image: String,
        is_store_active: bool,
        creator_id: i32
    ) -> Self {
        NewStore {
            title: title,
            description: Some(description),
            content: Some(content),
            link: Some(link),
            image: Some(image),
            is_store_active: is_store_active,
            creator: creator_id,
            store_created: chrono::Local::now().naive_utc(),
        }
    }
}

#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(Store, foreign_key="store")]
pub struct StoreImage {
    pub id: i32,
    pub store: i32,
    pub src: String,
}

#[derive(Serialize, Insertable)]
#[table_name="store_images"]
pub struct NewStoreImage {
    pub store: i32,
    pub src: String,
}

impl NewStoreImage {
    pub fn from_store_images_form(
        store_id: i32, src: String) -> Self {
        NewStoreImage {
            store: store_id,
            src: src,
        }
    }
}

#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(Store, foreign_key="store")]
pub struct StoreVideo {
    pub id: i32,
    pub store: i32,
    pub src: String,
}

#[derive(Serialize, Insertable)]
#[table_name="store_videos"]
pub struct NewStoreVideo {
    pub store: i32,
    pub src: String,
}

impl NewStoreVideo {
    pub fn from_store_videos_form(
        store_id: i32, src: String) -> Self {
        NewStoreVideo {
            store: store_id,
            src: src,
        }
    }
}
