use crate::schema;
use crate::diesel::{
    Queryable,
    Insertable,
    QueryDsl,
    RunQueryDsl,
    ExpressionMethods,
};
use serde::{Serialize, Deserialize};
use crate::models::{User, Serve};
use crate::schema::{
    orders,
    order_files,
};
use crate::utils::establish_connection;


#[derive(Debug, Serialize, Identifiable, Queryable, Associations)]
#[table_name="orders"]
pub struct Order {
    pub id:          i32,
    pub title:       String,
    pub types:       i16,
    pub object_id:   i32,
    pub username:    String,
    pub email:       String,
    pub description: Option<String>,
    pub created:     chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="orders"]
pub struct NewOrder {
    pub title:       String,
    pub types:       i16,
    pub object_id:   i32,
    pub username:    String,
    pub email:       String,
    pub description: Option<String>,
    pub created:     chrono::NaiveDateTime,
}
impl NewOrder {
    pub fn create (
        title:       String,
        types:       i16,
        object_id:   i32,
        username:    String,
        email:       String,
        description: Option<String>,
    ) -> Self {
        NewOrder {
            title:       title,
            types:       types,
            object_id:   object_id,
            username:    username,
            email:       email,
            description: description,
            created:     chrono::Local::now().naive_utc(),
        }
    }
}

#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(Order, foreign_key="order_id")]
pub struct OrderFile {
    pub id:       i32,
    pub order_id: i32,
    pub src:      String,
}

#[derive(Serialize, Insertable)]
#[table_name="order_files"]
pub struct NewOrderFile {
    pub order_id: i32,
    pub src:      String,
}

impl NewOrderFile {
    pub fn create (order_id: i32, src: String) -> Self {
        NewServiceImage {
            order_id: order_id,
            src:      src,
        }
    }
}
