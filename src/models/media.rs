use crate::schema;
use crate::diesel::{
    Queryable,
    Insertable,
    QueryDsl,
    RunQueryDsl,
    ExpressionMethods,
};
use serde::{Serialize, Deserialize};
use crate::schema::files;
use crate::utils::{
    establish_connection,
    get_linguage_storage,
};
use crate::models::User;


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

#[derive(Debug, Serialize, Identifiable, Queryable)]
#[table_name="files"]
pub struct File {
    pub id:             i32,
    pub user_id:        i32,
    pub item_id:        i32,
    pub item_types:     i16,
    pub types:          i16,
    pub src:            String,
    pub description:    Option<String>,
    pub description_en: Option<String>,
    pub position:       i16,
    pub view:           i32,
    pub seconds:        i32,
}
impl File {
    pub fn update_file_with_id(user: User, file_id: i32, form: crate::utils::CategoriesForm) -> i16 {
        let _connection = establish_connection();
        let l = get_linguage_storage();
        let _file = schema::files::table
            .filter(schema::files::id.eq(file_id))
            .first::<File>(&_connection)
            .expect("E.");
        let _item = schema::items::table
            .filter(schema::items::id.eq(_file.item_id))
            .filter(schema::items::types.eq(_file.item_types))
            .first::<crate::models::Item>(&_connection)
            .expect("E.");

        if user.perm < 60 && _item.user_id != user.id {
            return 0;
        }
        if l == 1 { 
            diesel::update(&_file)
                .set(schema::files::description.eq(&form.description))
                .execute(&_connection)
                .expect("E");
        }
        else if l == 2 {
            diesel::update(&_file)
                .set(schema::files::description_en.eq(&form.description))
                .execute(&_connection)
                .expect("E");
        }
        return 1;
    }
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="files"]
pub struct NewFile {
    pub user_id:        i32,
    pub item_id:        i32,
    pub item_types:     i16,
    pub types:          i16,
    pub src:            String,
    pub description:    Option<String>,
    pub description_en: Option<String>,
    pub position:       i16,
    pub view:           i32,
    pub seconds:        i32,
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
            user_id:        user_id,
            item_id:        item_id,
            item_types:     item_types,
            types:          types,
            src:            src,
            description:    None,
            description_en: None,
            position:       0,
            view:           0,
            seconds:        0,
        }
    }
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="files"]
pub struct EditFile {
    pub description:    Option<String>,
    pub description_en: Option<String>,
    pub position:       i16,
}
