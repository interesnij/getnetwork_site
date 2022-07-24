use crate::schema;
use crate::diesel::{
    Queryable,
    Insertable,
    BelongingToDsl,
    QueryDsl,
    RunQueryDsl,
    ExpressionMethods,
};
use serde::{Serialize, Deserialize,};
use crate::models::{User, Tag};
use crate::schema::{
    wiki_categories,
    wikis,
    wiki_category,
    wiki_images,
    wiki_videos,
};
use crate::utils::establish_connection;


#[derive(Debug, Serialize, Identifiable, Queryable, Associations)]
#[table_name="wiki_categories"]
pub struct WikiCategories {
    pub id:          i32,
    pub name:        String,
    pub description: Option<String>,
    pub position:    i32,
    pub image:       Option<String>,
    pub count:       i32,
}
impl WikiCategories {
    pub fn get_all_wikis(&self) -> Vec<Wiki> {
        use crate::schema::wiki_category::dsl::wiki_category;
        use crate::schema::wikis::dsl::wikis;

        let _connection = establish_connection();
        let ids = wiki_category
            .filter(schema::wiki_category::wiki_categories_id.eq(self.id))
            .select(schema::wiki_category::wiki_id)
            .load::<i32>(&_connection)
            .expect("E");

        return wikis
            .filter(schema::wikis::id.eq_any(ids))
            .filter(schema::wikis::is_active.eq(true))
            .order(schema::wikis::created.desc())
            .load::<Wiki>(&_connection)
            .expect("E.");
    }
    pub fn get_wikis_list(&self, page: i32, limit: i32) -> (Vec<Wiki>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Wiki>;

        if page > 1 {
            have_next = page * limit + 1;
            object_list = self.get_wikis(limit.into(), have_next.into());
        }
        else {
            have_next = limit + 1;
            object_list = self.get_wikis(limit.into(), 0);
        }
        if self.get_wikis(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        return (object_list, next_page_number);
    }

    pub fn get_wikis(&self, limit: i64, offset: i64) -> Vec<Wiki> {
        use crate::schema::wikis::dsl::wikis;

        let _connection = establish_connection();
        let ids = WikiCategory::belonging_to(self)
            .select(schema::wiki_category::wiki_id);
        return wikis
            .filter(schema::wikis::id.eq_any(ids))
            .filter(schema::wikis::is_active.eq(true))
            .order(schema::wikis::created.desc())
            .limit(limit)
            .offset(offset)
            .load::<Wiki>(&_connection)
            .expect("E.");
    }

    pub fn get_6_wikis(&self) -> Vec<Wiki> {
        use crate::schema::wikis::dsl::wikis;

        let _connection = establish_connection();
        let ids = WikiCategory::belonging_to(self)
            .select(schema::wiki_category::wiki_id);
        return wikis
            .filter(schema::wikis::id.eq_any(ids))
            .filter(schema::wikis::is_active.eq(true))
            .order(schema::wikis::created.desc())
            .limit(6)
            .load::<Wiki>(&_connection)
            .expect("E.");
    }
}

#[derive(Insertable)]
#[table_name="wiki_categories"]
pub struct NewWikiCategories {
    pub name:        String,
    pub description: Option<String>,
    pub position:    i32,
    pub image:       Option<String>,
    pub count:       i32,
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="wiki_categories"]
pub struct EditWikiCategories {
    pub name:        String,
    pub description: Option<String>,
    pub position:    i32,
    pub image:       Option<String>,
    pub count:       i32,
}

#[derive(Debug, Serialize, Clone, Queryable, Identifiable, Associations)]
#[belongs_to(User)]
pub struct Wiki {
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

impl Wiki {
    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/img.jpg".to_string();
        }
    }
    pub fn get_categories(&self) -> Vec<WikiCategories> {
        use crate::schema::wiki_categories::dsl::wiki_categories;

        let _connection = establish_connection();
        let ids = WikiCategory::belonging_to(self).select(schema::wiki_category::wiki_categories_id);
        return wiki_categories
            .filter(schema::wiki_categories::id.eq_any(ids))
            .load::<WikiCategories>(&_connection)
            .expect("E");
    }

    pub fn get_tags(&self) -> Vec<Tag> {
        use crate::schema::tags_items::dsl::tags_items;
        let _connection = establish_connection();

        let _tag_items = tags_items
            .filter(schema::tags_items::wiki_id.eq(&self.id))
            .select(schema::tags_items::tag_id)
            .load::<i32>(&_connection)
            .expect("E");
        schema::tags::table
            .filter(schema::tags::id.eq_any(_tag_items))
            .load::<Tag>(&_connection)
            .expect("E")
    }

    pub fn get_3_wikis() -> Vec<Wiki> {
        use crate::schema::wikis::dsl::wikis;

        let _connection = establish_connection();
        return wikis
            .filter(schema::wikis::is_active.eq(true))
            .order(schema::wikis::created.desc())
            .limit(6)
            .load::<Wiki>(&_connection)
            .expect("E.");
    }

    pub fn get_wikis_list_for_ids(page: i32, limit: i32, ids: &Vec<i32>) -> (Vec<Wiki>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Wiki>;

        if page > 1 {
            have_next = page * limit + 1;
            object_list = Wiki::get_wikis_for_ids(limit.into(), have_next.into(), &ids);
        }
        else {
            have_next = limit + 1;
            object_list = Wiki::get_wikis_for_ids(limit.into(), 0, &ids);
        }
        if Wiki::get_wikis_for_ids(1, have_next.into(), &ids).len() > 0 {
            next_page_number = page + 1;
        }
        // возвращает порцию статей и следующую страницу, если она есть
        return (object_list, next_page_number);
    }
    pub fn get_wikis_for_ids(limit: i64, offset: i64, ids: &Vec<i32>) -> Vec<Wiki> {
        use crate::schema::wikis::dsl::wikis;

        let _connection = establish_connection();
        return wikis
            .filter(schema::wikis::id.eq_any(ids))
            .filter(schema::wikis::is_active.eq(true))
            .order(schema::wikis::created.desc())
            .limit(limit)
            .offset(offset)
            .load::<Wiki>(&_connection)
            .expect("E.");
    }
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="wikis"]
pub struct EditWiki {
    pub title:       String,
    pub description: Option<String>,
    pub link:        Option<String>,
    pub image:       Option<String>,
    pub is_active:   bool,
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(WikiCategories)]
#[belongs_to(Wiki)]
#[table_name="wiki_category"]
pub struct WikiCategory {
    pub id:                 i32,
    pub wiki_categories_id: i32,
    pub wiki_id:            i32,
}

#[derive(Insertable)]
#[table_name="wiki_category"]
pub struct NewWikiCategory {
    pub wiki_categories_id: i32,
    pub wiki_id:            i32,
}

#[derive(Serialize, Insertable)]
#[table_name="wikis"]
pub struct NewWiki {
    pub title:       String,
    pub description: Option<String>,
    pub link:        Option<String>,
    pub image:       Option<String>,
    pub is_active:   bool,
    pub user_id:     i32,
    pub created:     chrono::NaiveDateTime,
}

impl NewWiki {
    pub fn from_wiki_form(
        title: String,
        description: String,
        link: String,
        image: String,
        is_active: bool,
        user_id: i32
    ) -> Self {
        NewWiki {
            title:       title,
            description: Some(description),
            link:        Some(link),
            image:       Some(image),
            is_active:   is_active,
            user_id:     user_id,
            created:     chrono::Local::now().naive_utc(),
        }
    }
}

#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(Wiki, foreign_key="wiki")]
pub struct WikiImage {
    pub id:   i32,
    pub wiki: i32,
    pub src:  String,
}

#[derive(Serialize, Insertable)]
#[table_name="wiki_images"]
pub struct NewWikiImage {
    pub wiki: i32,
    pub src:  String,
}

impl NewWikiImage {
    pub fn from_wiki_images_form(
        wiki_id: i32, src: String) -> Self {
        NewWikiImage {
            wiki: wiki_id,
            src:  src,
        }
    }
}

#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(Wiki, foreign_key="wiki")]
pub struct WikiVideo {
    pub id:   i32,
    pub wiki: i32,
    pub src:  String,
}

#[derive(Serialize, Insertable)]
#[table_name="wiki_videos"]
pub struct NewWikiVideo {
    pub wiki: i32,
    pub src:  String,
}

impl NewWikiVideo {
    pub fn from_wiki_videos_form(
        wiki_id: i32, src: String) -> Self {
        NewWikiVideo {
            wiki: wiki_id,
            src:  src,
        }
    }
}
