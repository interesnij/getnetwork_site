use crate::schema::{
    chats,
    messages,
};
use diesel::{
    Queryable,
    Insertable,
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
    NullableExpressionMethods,
};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use crate::schema;
use crate::errors::Error;


#[derive(Debug ,Queryable, Serialize, Identifiable)]
pub struct Chat {
    pub id:      i32,
    pub user_id: i32,
    pub created: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="chats"]
pub struct NewChat {
    pub user_id: i32,
    pub created: chrono::NaiveDateTime,
}

// view
// 1. создано
// 2. показано
// 3. прочитано

// types
// 1. обычное
// 2. измененное
// 3. удаленное

#[derive(Debug ,Queryable, Serialize, Identifiable)]
pub struct Message {
    pub id:      i32,
    pub user_id: i32,
    pub chat_id: i32,
    pub created: chrono::NaiveDateTime,
    pub content: Option<String>,
    pub view:    i16,
    pub types:   i16,
}

impl Message {
    pub fn get_files(&self) -> (
        Vec<(i32, String, Option<String>)>, // photos id, src, description
        Vec<(i32, String, Option<String>)>, // videos id, src, description
        Vec<(i32, String, Option<String>)>, // audios id, src, description
        Vec<(i32, String, Option<String>)>  // docs id, src, description
    ) {
        use schema::files::dsl::files;

        let _connection = establish_connection();
        let photos = files
            .filter(schema::files::item_id.eq(self.id))
            .filter(schema::files::item_types.eq(self.types))
            .filter(schema::files::types.eq(1))
            .select((schema::files::id, schema::files::src, schema::files::description.nullable()))
            .load::<(i32, String, Option<String>)>(&_connection)
            .expect("E");
        let videos = files
            .filter(schema::files::item_id.eq(self.id))
            .filter(schema::files::item_types.eq(self.types))
            .filter(schema::files::types.eq(2))
            .select((schema::files::id, schema::files::src, schema::files::description.nullable()))
            .load::<(i32, String, Option<String>)>(&_connection)
            .expect("E");
        let audios = files
            .filter(schema::files::item_id.eq(self.id))
            .filter(schema::files::item_types.eq(self.types))
            .filter(schema::files::types.eq(3))
            .select((schema::files::id, schema::files::src, schema::files::description.nullable()))
            .load::<(i32, String, Option<String>)>(&_connection)
            .expect("E");
        let docs = files
            .filter(schema::files::item_id.eq(self.id))
            .filter(schema::files::item_types.eq(self.types))
            .filter(schema::files::types.eq(4))
            .select((schema::files::id, schema::files::src, schema::files::description.nullable()))
            .load::<(i32, String, Option<String>)>(&_connection)
            .expect("E");

        return (photos, videos, audios, docs);
    }
    pub fn create (
        &self,
        user_id: i32,
        chat_id: i32,
        content: Option<String>,
        photos:  Option<Vec<String>>,
        videos:  Option<Vec<String>>,
        audios:  Option<Vec<String>>,
        docs:    Option<Vec<String>>,
    ) -> Result<Message, Error> {
        use chrono::Duration;
        use crate::models::NewFile;

        let _connection = establish_connection();

        let new_message_form = NewMessage {
            user_id: user_id,
            chat_id: chat_id,
            created: chrono::Local::now().naive_utc() + Duration::hours(3),
            content: content,
            view:    1,
            types:   1,
        };

        let _message = diesel::insert_into(schema::messages::table)
            .values(&new_message_form)
            .get_result::<Message>(&_connection)?;

        let _id: i32 = match _message {
            Ok(_ok) => _ok.id,
            Err(_error) => 0,
        };
        if _id > 0 {
            if photos.is_some() {
                for i in photos.unwrap() {
                    NewFile::create(user_id, _id, 11, 1, i.clone());
                }
            }
            if videos.is_some() {
                for i in videos.unwrap() {
                    NewFile::create(user_id, _id, 11, 2, i.clone());
                }
            }
            if audios.is_some() {
                for i in audios.unwrap() {
                    NewFile::create(user_id, _id, 11, 3, i.clone());
                }
            }
            if docs.is_some() {
                for i in docs.unwrap() {
                    NewFile::create(user_id, _id, 11, 4, i.clone());
                }
            }
        }
        return _message;
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="messages"]
pub struct NewMessage {
    pub user_id: i32,
    pub chat_id: i32,
    pub created: chrono::NaiveDateTime,
    pub content: Option<String>,
    pub view:    i16,
    pub types:   i16,
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="messages"]
pub struct EditMessage {
    pub content: Option<String>,
}
