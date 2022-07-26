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


#[derive(Debug, Serialize, PartialEq, Identifiable, Queryable, Associations)]
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
impl Tag {
    pub fn get_tags_list(page: i32, limit: i32) -> (Vec<Tag>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Tag>;

        if page > 1 {
            have_next = page * limit + 1;
            object_list = Tag::get_tags(limit.into(), have_next.into());
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
    pub fn get_tags(limit: i64, offset: i64) -> Vec<Tag> {
        use crate::schema::tags::dsl::tags;

        let _connection = establish_connection();
        return tags
            .order(schema::tags::count.desc())
            .limit(limit)
            .offset(offset)
            .load::<Tag>(&_connection)
            .expect("E.");
    }
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

#[derive(Identifiable, Serialize, PartialEq, Queryable, Associations)]
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
