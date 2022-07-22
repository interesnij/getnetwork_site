use crate::schema;
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize,};
use crate::models::User;
use crate::schema::{
    wiki_categories,
    wikis,
    wiki_category,
    wiki_images,
    wiki_videos,
};


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

#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
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
