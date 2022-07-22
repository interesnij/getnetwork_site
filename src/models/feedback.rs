use crate::schema::feedbacks;
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::schema::feedbacks;

#[derive(Debug ,Queryable, Serialize, Identifiable)]
pub struct Feedback {
    pub id:       i32,
    pub username: String,
    pub email:    String,
    pub message:  String,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="feedbacks"]
pub struct NewFeedback {
    pub username: String,
    pub email:    String,
    pub message:  String,
}
