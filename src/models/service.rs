use crate::schema;
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::models::User;
use crate::schema::{
    service_categories,
    services,
    service_category,
    service_images,
    service_videos,
};
use crate::utils::establish_connection;


#[derive(Debug, Serialize, Identifiable, Queryable, Associations)]
#[table_name="service_categories"]
pub struct ServiceCategories {
    pub id:          i32,
    pub name:        String,
    pub description: Option<String>,
    pub position:    i32,
    pub image:       Option<String>,
    pub count:       i32,
}
#[derive(Insertable)]
#[table_name="service_categories"]
pub struct NewServiceCategories {
    pub name:        String,
    pub description: Option<String>,
    pub position:    i32,
    pub image:       Option<String>,
    pub count:       i32,
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="service_categories"]
pub struct EditServiceCategories {
    pub name:        String,
    pub description: Option<String>,
    pub position:    i32,
    pub image:       Option<String>,
    pub count:       i32,
}

#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(User)]
pub struct Service {
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

impl Service {
    pub fn get_100_description(&self) -> String {
        if self.content.is_some() {
            let _content = self.content.as_deref().unwrap();
            if _content.len() > 100 {
                return _content[..100].to_string();
            }
            else {
                return _content.to_string();
            }
        }
        else {
            return "".to_string();
        }
    }
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="services"]
pub struct EditService {
    pub title:       String,
    pub description: Option<String>,
    pub link:        Option<String>,
    pub image:       Option<String>,
    pub is_active:   bool,
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(ServiceCategories)]
#[belongs_to(Service)]
#[table_name="service_category"]
pub struct ServiceCategory {
    pub id:                    i32,
    pub service_categories_id: i32,
    pub service_id:            i32,
}

#[derive(Insertable)]
#[table_name="service_category"]
pub struct NewServiceCategory {
    pub service_categories_id: i32,
    pub service_id:            i32,
}

#[derive(Serialize, Insertable)]
#[table_name="services"]
pub struct NewService {
    pub title:       String,
    pub description: Option<String>,
    pub link:        Option<String>,
    pub image:       Option<String>,
    pub is_active:   bool,
    pub user_id:     i32,
    pub created:     chrono::NaiveDateTime,
}

impl NewService {
    pub fn from_service_form (
        title: String,
        description: String,
        link: String,
        image: String,
        is_active: bool,
        user_id: i32
    ) -> Self {
        NewService {
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

#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(Service, foreign_key="service")]
pub struct ServiceImage {
    pub id:      i32,
    pub service: i32,
    pub src:     String
}

#[derive(Serialize, Insertable)]
#[table_name="service_images"]
pub struct NewServiceImage {
    pub service: i32,
    pub src:     String
}

impl NewServiceImage {
    pub fn from_service_images_form (
        service_id: i32, src: String) -> Self {
        NewServiceImage {
            service: service_id,
            src:     src
        }
    }
}

#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(Service, foreign_key="service")]
pub struct ServiceVideo {
    pub id:      i32,
    pub service: i32,
    pub src:     String
}

#[derive(Serialize, Insertable)]
#[table_name="service_videos"]
pub struct NewServiceVideo {
    pub service: i32,
    pub src:     String
}

impl NewServiceVideo {
    pub fn from_service_videos_form(
        service_id: i32, src: String) -> Self {
        NewServiceVideo {
            service: service_id,
            src:     src
        }
    }
}
