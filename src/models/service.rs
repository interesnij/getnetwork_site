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
use crate::models::{User, Tag, Serve};
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
impl ServiceCategories {
    pub fn get_services_list(&self, page: i32, limit: i32) -> (Vec<Service>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Service>;

        if page > 1 {
            have_next = page * limit + 1;
            object_list = self.get_services(limit.into(), have_next.into());
        }
        else {
            have_next = limit + 1;
            object_list = self.get_services(limit.into(), 0);
        }
        if self.get_services(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        return (object_list, next_page_number);
    }

    pub fn get_services(&self, limit: i64, offset: i64) -> Vec<Service> {
        use crate::schema::services::dsl::services;

        let _connection = establish_connection();
        let ids = ServiceCategory::belonging_to(self)
            .select(schema::service_category::service_id);
        return services
            .filter(schema::services::id.eq_any(ids))
            .filter(schema::services::is_active.eq(true))
            .order(schema::services::created.desc())
            .limit(limit)
            .offset(offset)
            .load::<Service>(&_connection)
            .expect("E.");
    }
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
    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/img.jpg".to_string();
        }
    }
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

    pub fn get_categories(&self) -> Vec<ServiceCategories> {
        use crate::schema::service_categories::dsl::service_categories;

        let _connection = establish_connection();
        let ids = ServiceCategory::belonging_to(self)
            .select(schema::service_category::service_categories_id);
        return service_categories
            .filter(schema::service_categories::id.eq_any(ids))
            .load::<ServiceCategories>(&_connection)
            .expect("E");
    }

    pub fn get_tags(&self) -> Vec<Tag> {
        use crate::schema::tags_items::dsl::tags_items;
        use crate::schema::tags::dsl::tags;
        let _connection = establish_connection();

        let _tag_items = tags_items
            .filter(schema::tags_items::service_id.eq(&self.id))
            .select(schema::tags_items::tag_id)
            .load::<i32>(&_connection)
            .expect("E");
        return tags
            .filter(schema::tags::id.eq_any(_tag_items))
            .load::<Tag>(&_connection)
            .expect("E");
    }

    pub fn get_serves(&self) -> Vec<Serve> {
        use schema::serve_items::dsl::serve_items;
        use schema::serve::dsl::serve;

        let _connection = establish_connection();
        let _serve_items = serve_items
            .filter(schema::serve_items::service_id.eq(&self.id))
            .select(schema::serve_items::serve_id)
            .load::<i32>(&_connection)
            .expect("E");

        return serve
            .filter(schema::serve::id.eq_any(_serve_items))
            .load::<Serve>(&_connection)
            .expect("E");
    }

    pub fn get_6_services() -> Vec<Service> {
        use crate::schema::services::dsl::services;

        let _connection = establish_connection();
        return services
            .filter(schema::services::is_active.eq(true))
            .order(schema::services::created.desc())
            .limit(6)
            .load::<Service>(&_connection)
            .expect("E.");
    }

    pub fn get_services_list_for_ids(page: i32, limit: i32, ids: &Vec<i32>) -> (Vec<Service>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Service>;

        if page > 1 {
            have_next = page * limit + 1;
            object_list = Service::get_services_for_ids(limit.into(), have_next.into(), &ids);
        }
        else {
            have_next = limit + 1;
            object_list = Service::get_services_for_ids(limit.into(), 0, &ids);
        }
        if Service::get_services_for_ids(1, have_next.into(), &ids).len() > 0 {
            next_page_number = page + 1;
        }
        // возвращает порцию статей и следующую страницу, если она есть
        return (object_list, next_page_number);
    }
    pub fn get_services_for_ids(limit: i64, offset: i64, ids: &Vec<i32>) -> Vec<Service> {
        use crate::schema::services::dsl::services;

        let _connection = establish_connection();
        return services
            .filter(schema::services::id.eq_any(ids))
            .filter(schema::services::is_active.eq(true))
            .order(schema::services::created.desc())
            .limit(limit)
            .offset(offset)
            .load::<Service>(&_connection)
            .expect("E.");
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
