use crate::schema;
use crate::diesel::{
    Queryable,
    Insertable,
    QueryDsl,
    RunQueryDsl,
    ExpressionMethods,
};
use serde::{Serialize, Deserialize};
use crate::models::{User, Tag, Serve, TechCategories};
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
    pub position:    i16,
    pub image:       Option<String>,
    pub count:       i16,
    pub view:        i32,
    pub height:      f64,
    pub seconds:     i32,
}
impl ServiceCategories {
    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/img.jpg".to_string();
        }
    }
    pub fn get_services_ids(&self) -> Vec<i32> {
        use crate::schema::service_category::dsl::service_category;

        let _connection = establish_connection();
        return service_category
            .filter(schema::service_category::service_categories_id.eq(self.id))
            .select(schema::service_category::service_id)
            .load::<i32>(&_connection)
            .expect("E");
    }
    pub fn get_all_services(&self) -> Vec<Service> {
        use crate::schema::services::dsl::services;

        let _connection = establish_connection();
        return services
            .filter(schema::services::id.eq_any(self.get_services_ids()))
            .order(schema::services::position.desc())
            .load::<Service>(&_connection)
            .expect("E");
    }
    pub fn get_services_list(&self, page: i32, limit: i32) -> (Vec<Service>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Service>;

        if page > 1 {
            have_next = page * limit + 1;
            let step = (page - 1) * 20;
            object_list = self.get_services(limit.into(), step.into());
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
        use crate::schema::{
            services::dsl::services,
            service_category::dsl::service_category,
        };

        let _connection = establish_connection();
        let ids = service_category
            .filter(schema::service_category::service_categories_id.eq(self.id))
            .select(schema::service_category::service_id)
            .load::<i32>(&_connection)
            .expect("E");
        return services
            .filter(schema::services::id.eq_any(ids))
            .filter(schema::services::is_active.eq(true))
            .order(schema::services::position.desc())
            .limit(limit)
            .offset(offset)
            .load::<Service>(&_connection)
            .expect("E.");
    }
    pub fn get_6_services(&self) -> Vec<Service> {
        use crate::schema::{
            services::dsl::services,
            service_category::dsl::service_category,
        };

        let _connection = establish_connection();
        let ids = service_category
            .filter(schema::service_category::service_categories_id.eq(self.id))
            .select(schema::service_category::service_id)
            .load::<i32>(&_connection)
            .expect("E");
        return services
            .filter(schema::services::id.eq_any(ids))
            .filter(schema::services::is_active.eq(true))
            .order(schema::services::position.desc())
            .limit(6)
            .load::<Service>(&_connection)
            .expect("E.");
    }
}

#[derive(Insertable)]
#[table_name="service_categories"]
pub struct NewServiceCategories {
    pub name:        String,
    pub description: Option<String>,
    pub position:    i16,
    pub image:       Option<String>,
    pub count:       i16,
    pub view:        i32,
    pub height:      f64,
    pub seconds:     i32,
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="service_categories"]
pub struct EditServiceCategories {
    pub name:        String,
    pub description: Option<String>,
    pub position:    i16,
    pub image:       Option<String>,
}

#[derive(Debug, Serialize, Clone, Queryable, Identifiable, Associations)]
#[belongs_to(User)]
pub struct Service {
    pub id:          i32,
    pub title:       String,
    pub description: Option<String>,
    pub content:     Option<String>,
    pub link:        Option<String>,
    pub image:       Option<String>,
    pub is_active:   bool,
    pub price:       i32,
    pub user_id:     i32,
    pub created:     chrono::NaiveDateTime,
    pub position:    i16,
    pub view:        i32,
    pub height:      f64,
    pub seconds:     i32,
    pub price_acc:   Option<i32>,
}

impl Service {
    pub fn get_order_type(&self) -> i16 {
        return 1;
    }
    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/img.jpg".to_string();
        }
    }
    pub fn get_100_description(&self) -> String {
        if self.description.is_some() {
            let _content = self.description.as_deref().unwrap();
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
        use crate::schema::{
            service_category::dsl::service_category,
            service_categories::dsl::service_categories,
        };

        let _connection = establish_connection();
        let ids = service_category
            .filter(schema::service_category::service_id.eq(self.id))
            .select(schema::service_category::service_categories_id)
            .load::<i32>(&_connection)
            .expect("E");
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
            .order(schema::serve::position.desc())
            .load::<Serve>(&_connection)
            .expect("E");
    }
    pub fn get_serves_ids(&self) -> Vec<i32> {
        use schema::serve_items::dsl::serve_items;

        let _connection = establish_connection();
        return serve_items
            .filter(schema::serve_items::service_id.eq(&self.id))
            .select(schema::serve_items::serve_id)
            .load::<i32>(&_connection)
            .expect("E");
    }
    pub fn get_close_tech_cats_ids(&self) -> Vec<i32> {
        use schema::tech_categories_items::dsl::tech_categories_items;

        let _connection = establish_connection();
        return tech_categories_items
            .filter(schema::tech_categories_items::service_id.eq(&self.id))
            .filter(schema::tech_categories_items::types.eq(2))
            .select(schema::tech_categories_items::category_id)
            .load::<i32>(&_connection)
            .expect("E");
    }
    pub fn get_open_tech_categories(&self) -> Vec<TechCategories> {
        // получаем открытые тех.категории услуги
        use schema::{
            tech_categories_items::dsl::tech_categories_items,
            tech_categories::dsl::tech_categories,
        };

        let _connection = establish_connection();
        let ids = tech_categories_items
            .filter(schema::tech_categories_items::service_id.eq(&self.id))
            .filter(schema::tech_categories_items::types.eq(1))
            .select(schema::tech_categories_items::category_id)
            .load::<i32>(&_connection)
            .expect("E");

        return tech_categories
            .filter(schema::tech_categories::id.eq_any(ids))
            .order(schema::tech_categories::position.desc())
            .load::<TechCategories>(&_connection)
            .expect("E");
    }
    pub fn get_close_tech_categories(&self) -> Vec<TechCategories> {
        // получаем закрытые тех.категории услуги
        use schema::{
            tech_categories_items::dsl::tech_categories_items,
            tech_categories::dsl::tech_categories,
        };

        let _connection = establish_connection();
        let ids = tech_categories_items
            .filter(schema::tech_categories_items::service_id.eq(&self.id))
            .filter(schema::tech_categories_items::types.eq(2))
            .select(schema::tech_categories_items::category_id)
            .load::<i32>(&_connection)
            .expect("E");

        return tech_categories
            .filter(schema::tech_categories::id.eq_any(ids))
            .order(schema::tech_categories::position.desc())
            .load::<TechCategories>(&_connection)
            .expect("E");
    }

    pub fn get_6_services(user: User) -> Vec<Service> {
        use crate::schema::services::dsl::services;

        let _connection = establish_connection();
        if user.is_superuser() {
            return services
                .order(schema::services::position.desc())
                .limit(6)
                .load::<Service>(&_connection)
                .expect("E.");
        } else {
            return services
                .filter(schema::services::is_active.eq(true))
                .order(schema::services::position.desc())
                .limit(6)
                .load::<Service>(&_connection)
                .expect("E.");
        }
    }

    pub fn get_services_list_for_ids(page: i32, limit: i32, ids: &Vec<i32>) -> (Vec<Service>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Service>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Service::get_services_for_ids(limit.into(), step.into(), &ids);
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
            .order(schema::services::position.desc())
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
    pub position:    i16,
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
    pub price:       i32,
    pub user_id:     i32,
    pub created:     chrono::NaiveDateTime,
    pub position:    i16,
    pub view:        i32,
    pub height:      f64,
    pub seconds:     i32,
}

impl NewService {
    pub fn create (
        title:       String,
        description: String,
        link:        String,
        image:       String,
        is_active:   bool,
        user_id:     i32,
        position:    i16,
    ) -> Self {
        NewService {
            title:       title,
            description: Some(description),
            link:        Some(link),
            image:       Some(image),
            is_active:   is_active,
            price:       0,
            user_id:     user_id,
            created:     chrono::Local::now().naive_utc(),
            position:    position,
            view:        0,
            height:      0.0,
            seconds:     0,
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
    pub fn create (
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
    pub fn create (
        service_id: i32, src: String) -> Self {
        NewServiceVideo {
            service: service_id,
            src:     src
        }
    }
}
