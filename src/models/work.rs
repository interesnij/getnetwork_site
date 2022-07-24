use crate::schema;
use crate::diesel::{
    Queryable,
    Insertable,
    BelongingToDsl,
    QueryDsl,
    RunQueryDsl,
    ExpressionMethods,
};
use serde::{Serialize, Deserialize};
use crate::models::{User, Tag};
use crate::schema::{
    work_categories,
    works,
    work_category,
    work_images,
    work_videos,
};
use crate::utils::establish_connection;


/////// WorkCategories //////
#[derive(Debug, Serialize, Identifiable, Queryable, Associations)]
#[table_name="work_categories"]
pub struct WorkCategories {
    pub id:          i32,
    pub name:        String,
    pub description: Option<String>,
    pub position:    i32,
    pub image:       Option<String>,
    pub count:       i32,
}
impl WorkCategories {
    pub fn get_all_works(&self) -> Vec<Work> {
        use crate::schema::work_category::dsl::work_category;

        let _connection = establish_connection();
        let ids = work_category
            .filter(schema::work_category::work_categories_id.eq(self.id))
            .select(schema::work_category::work_id)
            .load::<i32>(&_connection)
            .expect("E");

        return stores
            .filter(schema::works::id.eq_any(ids))
            .filter(schema::works::is_active.eq(true))
            .order(schema::works::created.desc())
            .load::<Wiki>(&_connection)
            .expect("E.");
    }
    pub fn get_works_list(&self, page: i32, limit: i32) -> (Vec<Work>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Work>;

        if page > 1 {
            have_next = page * limit + 1;
            object_list = self.get_works(limit.into(), have_next.into());
        }
        else {
            have_next = limit + 1;
            object_list = self.get_works(limit.into(), 0);
        }
        if self.get_works(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        return (object_list, next_page_number);
    }

    pub fn get_works(&self, limit: i64, offset: i64) -> Vec<Work> {
        use crate::schema::works::dsl::works;

        let _connection = establish_connection();
        let ids = WorkCategory::belonging_to(self)
            .select(schema::work_category::work_id);
        return works
            .filter(schema::works::id.eq_any(ids))
            .filter(schema::works::is_active.eq(true))
            .order(schema::works::created.desc())
            .limit(limit)
            .offset(offset)
            .load::<Work>(&_connection)
            .expect("E.");
    }
    pub fn get_6_works(&self) -> Vec<Work> {
        use crate::schema::works::dsl::works;

        let _connection = establish_connection();
        let ids = WorkCategory::belonging_to(self)
            .select(schema::work_category::work_id);
        return works
            .filter(schema::works::id.eq_any(ids))
            .filter(schema::works::is_active.eq(true))
            .order(schema::works::created.desc())
            .limit(6)
            .load::<Work>(&_connection)
            .expect("E.");
    }
}

#[derive(Insertable)]
#[table_name="work_categories"]
pub struct NewWorkCategories {
    pub name:        String,
    pub description: Option<String>,
    pub position:    i32,
    pub image:       Option<String>,
    pub count:       i32,
}
#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="work_categories"]
pub struct EditWorkCategories {
    pub name:        String,
    pub description: Option<String>,
    pub position:    i32,
    pub image:       Option<String>,
    pub count:       i32,
}

/////// Work //////
#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(User)]
pub struct Work {
    pub id:          i32,
    pub title:       String,
    pub description: Option<String>,
    pub content:     Option<String>,
    pub link:        Option<String>,
    pub image:       Option<String>,
    pub is_active:   bool,
    pub user_id:     i32,
    pub created:     chrono::NaiveDateTime,
}
impl Work {
    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/img.jpg".to_string();
        }
    }
    pub fn get_categories(&self) -> Vec<WorkCategories> {
        use crate::schema::work_categories::dsl::work_categories;

        let _connection = establish_connection();
        let ids = WorkCategory::belonging_to(self).select(schema::work_category::work_categories_id);
        return work_categories
            .filter(schema::work_categories::id.eq_any(ids))
            .load::<WorkCategories>(&_connection)
            .expect("E");
    }

    pub fn get_3_works() -> Vec<Work> {
        use crate::schema::works::dsl::works;

        let _connection = establish_connection();
        return works
            .filter(schema::works::is_active.eq(true))
            .order(schema::works::created.desc())
            .limit(3)
            .load::<Work>(&_connection)
            .expect("E.");
    }

    pub fn get_tags(&self) -> Vec<Tag> {
        use crate::schema::tags_items::dsl::tags_items;
        let _connection = establish_connection();

        let _tag_items = tags_items
            .filter(schema::tags_items::work_id.eq(&self.id))
            .select(schema::tags_items::tag_id)
            .load::<i32>(&_connection)
            .expect("E");
        schema::tags::table
            .filter(schema::tags::id.eq_any(_tag_items))
            .load::<Tag>(&_connection)
            .expect("E")
    }

    pub fn get_works_list_for_ids(page: i32, limit: i32, ids: &Vec<i32>) -> (Vec<Work>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Work>;

        if page > 1 {
            have_next = page * limit + 1;
            object_list = Work::get_works_for_ids(limit.into(), have_next.into(), &ids);
        }
        else {
            have_next = limit + 1;
            object_list = Work::get_works_for_ids(limit.into(), 0, &ids);
        }
        if Work::get_works_for_ids(1, have_next.into(), &ids).len() > 0 {
            next_page_number = page + 1;
        }
        // возвращает порцию статей и следующую страницу, если она есть
        return (object_list, next_page_number);
    }
    pub fn get_works_for_ids(limit: i64, offset: i64, ids: &Vec<i32>) -> Vec<Work> {
        use crate::schema::works::dsl::works;

        let _connection = establish_connection();
        return works
            .filter(schema::works::id.eq_any(ids))
            .filter(schema::works::is_active.eq(true))
            .order(schema::works::created.desc())
            .limit(limit)
            .offset(offset)
            .load::<Work>(&_connection)
            .expect("E.");
    }
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="works"]
pub struct EditWork {
    pub title:       String,
    pub description: Option<String>,
    pub link:        Option<String>,
    pub image:       Option<String>,
    pub is_active:   bool,
}
#[derive(Serialize, Insertable)]
#[table_name="works"]
pub struct NewWork {
    pub title:       String,
    pub description: Option<String>,
    pub link:        Option<String>,
    pub image:       Option<String>,
    pub is_active:   bool,
    pub user_id:     i32,
    pub created:     chrono::NaiveDateTime,
}

impl NewWork {
    pub fn from_work_form (
        title: String,
        description: String,
        link: String,
        image: String,
        is_active: bool,
        user_id: i32
    ) -> Self {
        NewWork {
            title: title,
            description: Some(description),
            link: Some(link),
            image: Some(image),
            is_active: is_active,
            user_id: user_id,
            created: chrono::Local::now().naive_utc(),
        }
    }
}

/////// WorkCategory //////
#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(WorkCategories)]
#[belongs_to(Work)]
#[table_name="work_category"]
pub struct WorkCategory {
    pub id:                 i32,
    pub work_categories_id: i32,
    pub work_id:            i32,
}
#[derive(Insertable)]
#[table_name="work_category"]
pub struct NewWorkCategory {
    pub work_categories_id: i32,
    pub work_id:            i32,
}

/////// WorkImage //////
#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(Work, foreign_key="work")]
pub struct WorkImage {
    pub id:   i32,
    pub work: i32,
    pub src:  String,
}
#[derive(Serialize, Insertable)]
#[table_name="work_images"]
pub struct NewWorkImage {
    pub work: i32,
    pub src:  String,
}
impl NewWorkImage {
    pub fn from_work_images_form (
        work_id: i32, src: String) -> Self {
        NewWorkImage {
            work: work_id,
            src:  src,
        }
    }
}

/////// WorkVideo //////
#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(Work, foreign_key="work")]
pub struct WorkVideo {
    pub id:   i32,
    pub work: i32,
    pub src:  String,
}
#[derive(Serialize, Insertable)]
#[table_name="work_videos"]
pub struct NewWorkVideo {
    pub work: i32,
    pub src:  String,
}
impl NewWorkVideo {
    pub fn from_work_videos_form (
        work_id: i32, src: String) -> Self {
        NewWorkVideo {
            work: work_id,
            src:  src,
        }
    }
}
