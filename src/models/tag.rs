use crate::schema;
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::schema::{
    tags,
    tags_items,
};

#[derive(Debug, Serialize, Identifiable, Queryable, Associations)]
#[table_name="tags"]
pub struct Tag {
    pub id:            i32,
    pub name:          String,
    pub position:      i32,
    pub count:         i32,
    pub blog_count:    i32,
    pub service_count: i32,
    pub store_count:   i32,
    pub wiki_count:    i32,
    pub work_count:    i32,
    pub user_id:       i32,
}
#[derive(Insertable)]
#[table_name="tags"]
pub struct NewTag {
    pub name:          String,
    pub position:      i32,
    pub count:         i32,
    pub blog_count:    i32,
    pub service_count: i32,
    pub store_count:   i32,
    pub wiki_count:    i32,
    pub work_count:    i32,
    pub user_id:       i32,
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="tags"]
pub struct EditTag {
    pub name:     String,
    pub position: i32,
}

#[derive(Identifiable, Serialize, Queryable, Associations)]
#[table_name="tags_items"]
pub struct TagItems {
    pub id:         i32,
    pub tag_id:     i32,
    pub service_id: i32,
    pub store_id:   i32,
    pub blog_id:    i32,
    pub wiki_id:    i32,
    pub work_id:    i32,
    pub created:    chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="tags_items"]
pub struct NewTagItems {
    pub tag_id:     i32,
    pub service_id: i32,
    pub store_id:   i32,
    pub blog_id:    i32,
    pub wiki_id:    i32,
    pub work_id:    i32,
    pub created:    chrono::NaiveDateTime,
}
