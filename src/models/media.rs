//use crate::schema;
use crate::diesel::{
    Queryable,
    Insertable,
    //QueryDsl,
    //RunQueryDsl,
};
use serde::{Serialize, Deserialize};
use crate::schema::files;
//use crate::utils::establish_connection;


///////////
// item_types:
// 1. блог
// 2. услуга
// 3. товар
// 4. wiki
// 5. работа
// 6. помощь
// 7. заказ
// 8. веб-сервис
// 9. язык / технология
// 10. опция

// 11. сообщение

// types:
// 1. photos
// 2. videos
// 3. audios
// 4. docs

#[derive(Serialize, Queryable)]
pub struct SmallFile {
    pub id:          i32,
    pub src:         String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Identifiable, Queryable, Associations)]
#[table_name="files"]
pub struct File {
    pub id:          i32,
    pub user_id:     i32,
    pub item_id:     i32,
    pub item_types:  i16,
    pub types:       i16,
    pub src:         String,
    pub description: Option<String>,
    pub position:    i16,
    pub view:        i32,
    pub seconds:     i32,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="files"]
pub struct NewFile {
    pub item_id:     i32,
    pub user_id:     i32,
    pub item_types:  i16,
    pub types:       i16,
    pub src:         String,
    pub description: Option<String>,
    pub position:    i16,
    pub view:        i32,
    pub seconds:     i32,
}

impl NewFile {
    pub fn create (
        user_id:     i32,
        item_id:     i32,
        item_types:  i16,
        types:       i16,
        src:         String
    ) -> Self {
        NewFile {
            user_id:     user_id,
            item_id:     item_id,
            item_types:  item_types,
            types:       types,
            src:         src,
            description: None,
            position:    0,
            view:        0,
            seconds:     0,
        }
    }
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="files"]
pub struct EditFile {
    pub description: Option<String>,
    pub position:    i16,
}
