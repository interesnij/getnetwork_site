use crate::schema;
use crate::diesel::{
    Queryable,
    Insertable,
    QueryDsl,
    RunQueryDsl,
    ExpressionMethods,
};
use serde::{Serialize, Deserialize};
use crate::schema::{
    tags,
    tags_items,
};
use crate::utils::establish_connection;


#[derive(Serialize, Queryable)]
pub struct SmallTag {
    pub name:  String,
    pub count: i16,
}

#[derive(Debug, Serialize, Queryable, Identifiable)]
#[table_name="tags"]
pub struct Tag {
    pub id:       i32,
    pub name:     String,
    pub name_en:  String,
    pub position: i16,
    pub count:    i16,
    pub user_id:  i32,
    pub view:     i32,
    pub height:   f64,
    pub seconds:  i32,
}
impl Tag {
    pub fn get_tags_list(page: i32, limit: i32) -> (Vec<SmallTag>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<SmallTag>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Tag::get_tags(limit.into(), step.into());
        }
        else {
            have_next = limit + 1;
            object_list = Tag::get_tags(limit.into(), 0);
        }
        if Tag::get_tags(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        return (object_list, next_page_number);
    }
    pub fn get_tags(limit: i64, offset: i64) -> Vec<SmallTag> {
        use crate::schema::tags::dsl::tags;

        let _connection = establish_connection();
        return tags
            .order(schema::tags::count.desc())
            .limit(limit)
            .offset(offset)
            .select((
                schema::tags::name,
                schema::tags::count
            ))
            .load::<SmallTag>(&_connection)
            .expect("E.");
    }
}

#[derive(Insertable)]
#[table_name="tags"]
pub struct NewTag {
    pub name:     String,
    pub name_en:  String,
    pub position: i16,
    pub count:    i16,
    pub user_id:  i32,
    pub view:     i32,
    pub height:   f64,
    pub seconds:  i32,
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset)]
#[table_name="tags"]
pub struct EditTag {
    pub name:     String,
    pub name_en:  String,
    pub position: i16,
}

///////////
// types:
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
#[derive(Identifiable, Serialize, Queryable)]
#[table_name="tags_items"]
pub struct TagItems {
    pub id:      i32,
    pub tag_id:  i32,
    pub item_id: i32,
    pub types:   i16,
    pub created: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="tags_items"]
pub struct NewTagItems {
    pub tag_id:  i32,
    pub item_id: i32,
    pub types:   i16,
    pub created: chrono::NaiveDateTime,
}
