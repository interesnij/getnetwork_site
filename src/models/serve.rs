use crate::schema::*;
use diesel::{Queryable, Insertable};
use serde::{
    Serialize,
    Deserialize
};

/////// ServeCategories //////
#[derive(Debug, Serialize, Identifiable, Queryable, Associations)]
#[table_name="serve_categories"]
pub struct ServeCategories {
    pub id: i32,
    pub name: String,
    pub serve_position: i32,
    pub serve_count: i32,
}
#[derive(Insertable)]
#[table_name="serve_categories"]
pub struct NewServeCategories {
    pub name: String,
    pub serve_position: i32,
    pub serve_count: i32,
}
#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="serve_categories"]
pub struct EditServeCategories {
    pub name: String,
    pub serve_position: i32,
    pub serve_count: i32,
}

/////// Serve //////
#[derive(Debug, Serialize, Identifiable, Queryable, Associations)]
#[belongs_to(ServeCategories, foreign_key="serve_categories")]
#[table_name="serve"]
pub struct Serve {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub serve_position: i32,
    pub category: i32,
    pub price: i32,
    pub price_acc: Option<i32>,
    pub social_price: Option<i32>,
}
#[derive(Insertable)]
#[table_name="serve"]
pub struct NewServe {
    pub name: String,
    pub description: String,
    pub serve_position: i32,
    pub category: i32,
    pub price: i32,
    pub price_acc: Option<i32>,
    pub social_price: Option<i32>,
}
#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="serve"]
pub struct EditServe {
    pub name: String,
    pub description: String,
    pub serve_position: i32,
    pub category: i32,
    pub price: i32,
    pub price_acc: Option<i32>,
    pub social_price: Option<i32>,
}

/////// ServeItems //////
#[derive(Identifiable, Queryable, Associations)]
#[table_name="serve_items"]
pub struct ServeItems {
    pub id: i32,
    pub serve_id: i32,
    pub service_id: i32,
    pub store_id: i32,
    pub work_id: i32,
}
#[derive(Insertable)]
#[table_name="serve_items"]
pub struct NewServeItems {
    pub serve_id: i32,
    pub service_id: i32,
    pub store_id: i32,
    pub work_id: i32,
}
