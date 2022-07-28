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
use crate::models::{User, Tag, Serve, TechCategories};
use crate::schema::{
    store_categories,
    stores,
    store_category,
    store_images,
    store_videos,
};
use crate::utils::establish_connection;


#[derive(Debug, Serialize, PartialEq, Identifiable, Queryable, Associations)]
#[table_name="store_categories"]
pub struct StoreCategories {
    pub id:          i32,
    pub name:        String,
    pub description: Option<String>,
    pub position:    i32,
    pub image:       Option<String>,
    pub count:       i32,
}
impl StoreCategories {
    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/img.jpg".to_string();
        }
    }
    pub fn get_stores_ids(&self) -> Vec<i32> {
        use crate::schema::store_category::dsl::store_category;

        let _connection = establish_connection();
        return store_category
            .filter(schema::store_category::store_categories_id.eq(self.id))
            .select(schema::store_category::store_id)
            .load::<i32>(&_connection)
            .expect("E");
    }
    pub fn get_stores_list(&self, page: i32, limit: i32) -> (Vec<Store>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Store>;

        if page > 1 {
            have_next = page * limit + 1;
            object_list = self.get_stores(limit.into(), have_next.into());
        }
        else {
            have_next = limit + 1;
            object_list = self.get_stores(limit.into(), 0);
        }
        if self.get_stores(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        return (object_list, next_page_number);
    }

    pub fn get_stores(&self, limit: i64, offset: i64) -> Vec<Store> {
        use crate::schema::stores::dsl::stores;

        let _connection = establish_connection();
        let ids = StoreCategory::belonging_to(self)
            .select(schema::store_category::store_id);
        return stores
            .filter(schema::stores::id.eq_any(ids))
            .filter(schema::stores::is_active.eq(true))
            .order(schema::stores::created.desc())
            .limit(limit)
            .offset(offset)
            .load::<Store>(&_connection)
            .expect("E.");
    }

    pub fn get_6_stores(&self) -> Vec<Store> {
        use crate::schema::stores::dsl::stores;

        let _connection = establish_connection();
        let ids = StoreCategory::belonging_to(self)
            .select(schema::store_category::store_id);
        return stores
            .filter(schema::stores::id.eq_any(ids))
            .filter(schema::stores::is_active.eq(true))
            .order(schema::stores::created.desc())
            .limit(6)
            .load::<Store>(&_connection)
            .expect("E.");
    }
}

#[derive(Insertable)]
#[table_name="store_categories"]
pub struct NewStoreCategories {
    pub name:        String,
    pub description: Option<String>,
    pub position:    i32,
    pub image:       Option<String>,
    pub count:       i32,
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="store_categories"]
pub struct EditStoreCategories {
    pub name:        String,
    pub description: Option<String>,
    pub position:    i32,
    pub image:       Option<String>,
    pub count:       i32,
}

#[derive(Debug, Serialize, PartialEq, Clone, Queryable, Identifiable, Associations)]
#[belongs_to(User)]
pub struct Store {
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
}

impl Store {
    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/img.jpg".to_string();
        }
    }
    pub fn get_tags(&self) -> Vec<Tag> {
        use crate::schema::tags_items::dsl::tags_items;
        use crate::schema::tags::dsl::tags;

        let _connection = establish_connection();
        let _tag_items = tags_items
            .filter(schema::tags_items::store_id.eq(&self.id))
            .select(schema::tags_items::tag_id)
            .load::<i32>(&_connection)
            .expect("E");
        return tags
            .filter(schema::tags::id.eq_any(_tag_items))
            .load::<Tag>(&_connection)
            .expect("E");
    }

    pub fn get_categories(&self) -> Vec<StoreCategories> {
        use crate::schema::store_categories::dsl::store_categories;

        let _connection = establish_connection();
        let ids = StoreCategory::belonging_to(self)
            .select(schema::store_category::store_categories_id);
        return store_categories
            .filter(schema::store_categories::id.eq_any(ids))
            .load::<StoreCategories>(&_connection)
            .expect("E");
    }

    pub fn get_serves(&self) -> Vec<Serve> {
        use schema::serve_items::dsl::serve_items;
        use schema::serve::dsl::serve;

        let _connection = establish_connection();
        let _serve_items = serve_items
            .filter(schema::serve_items::store_id.eq(&self.id))
            .select(schema::serve_items::store_id)
            .load::<i32>(&_connection)
            .expect("E");

        return serve
            .filter(schema::serve::id.eq_any(_serve_items))
            .load::<Serve>(&_connection)
            .expect("E");
    }

    pub fn get_serves_ids(&self) -> Vec<i32> {
        use schema::serve_items::dsl::serve_items;

        let _connection = establish_connection();
        return serve_items
            .filter(schema::serve_items::store_id.eq(&self.id))
            .select(schema::serve_items::serve_id)
            .load::<i32>(&_connection)
            .expect("E");
    }
    pub fn get_open_tech_categories(&self) -> Vec<TechCategories> {
        // получаем открытые тех.категории товара
        use schema::{
            tech_categories_items::dsl::tech_categories_items,
            tech_categories::dsl::tech_categories,
        };

        let _connection = establish_connection();
        let ids = tech_categories_items
            .filter(schema::tech_categories_items::store_id.eq(&self.id))
            .filter(schema::tech_categories_items::types.eq(1))
            .select(schema::tech_categories_items::category_id)
            .load::<i32>(&_connection)
            .expect("E");

        return tech_categories
            .filter(schema::tech_categories::id.eq_any(ids))
            .load::<TechCategories>(&_connection)
            .expect("E");
    }
    pub fn get_close_tech_categories(&self) -> Vec<TechCategories> {
        // получаем закрытые тех.категории товара
        use schema::{
            tech_categories_items::dsl::tech_categories_items,
            tech_categories::dsl::tech_categories,
        };

        let _connection = establish_connection();
        let ids = tech_categories_items
            .filter(schema::tech_categories_items::store_id.eq(&self.id))
            .filter(schema::tech_categories_items::types.eq(2))
            .select(schema::tech_categories_items::category_id)
            .load::<i32>(&_connection)
            .expect("E");

        return tech_categories
            .filter(schema::tech_categories::id.eq_any(ids))
            .load::<TechCategories>(&_connection)
            .expect("E");
    }

    pub fn get_3_stores() -> Vec<Store> {
        use crate::schema::stores::dsl::stores;

        let _connection = establish_connection();
        return stores
            .filter(schema::stores::is_active.eq(true))
            .order(schema::stores::created.desc())
            .limit(6)
            .load::<Store>(&_connection)
            .expect("E.");
    }

    pub fn get_stores_list_for_ids(page: i32, limit: i32, ids: &Vec<i32>) -> (Vec<Store>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Store>;

        if page > 1 {
            have_next = page * limit + 1;
            object_list = Store::get_stores_for_ids(limit.into(), have_next.into(), &ids);
        }
        else {
            have_next = limit + 1;
            object_list = Store::get_stores_for_ids(limit.into(), 0, &ids);
        }
        if Store::get_stores_for_ids(1, have_next.into(), &ids).len() > 0 {
            next_page_number = page + 1;
        }
        // возвращает порцию статей и следующую страницу, если она есть
        return (object_list, next_page_number);
    }
    pub fn get_stores_for_ids(limit: i64, offset: i64, ids: &Vec<i32>) -> Vec<Store> {
        use crate::schema::stores::dsl::stores;

        let _connection = establish_connection();
        return stores
            .filter(schema::stores::id.eq_any(ids))
            .filter(schema::stores::is_active.eq(true))
            .order(schema::stores::created.desc())
            .limit(limit)
            .offset(offset)
            .load::<Store>(&_connection)
            .expect("E.");
    }
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="stores"]
pub struct EditStore {
    pub title:       String,
    pub description: Option<String>,
    pub link:        Option<String>,
    pub image:       Option<String>,
    pub is_active:   bool,
    pub price:       i32,
}

#[derive(Serialize, Insertable)]
#[table_name="stores"]
pub struct NewStore {
    pub title:       String,
    pub description: Option<String>,
    pub link:        Option<String>,
    pub image:       Option<String>,
    pub is_active:   bool,
    pub price:       i32,
    pub user_id:     i32,
    pub created:     chrono::NaiveDateTime,
}

impl NewStore {
    pub fn from_store_form (
        title:       String,
        description: String,
        link:        String,
        image:       String,
        is_active:   bool,
        price:       i32,
        user_id:     i32
    ) -> Self {
        NewStore {
            title:       title,
            description: Some(description),
            link:        Some(link),
            image:       Some(image),
            is_active:   is_active,
            price:       price,
            user_id:     user_id,
            created:     chrono::Local::now().naive_utc(),
        }
    }
}


#[derive(Identifiable, PartialEq, Queryable, Associations)]
#[belongs_to(StoreCategories)]
#[belongs_to(Store)]
#[table_name="store_category"]
pub struct StoreCategory {
    pub id:                  i32,
    pub store_categories_id: i32,
    pub store_id:            i32,
}

#[derive(Insertable)]
#[table_name="store_category"]
pub struct NewStoreCategory {
    pub store_categories_id: i32,
    pub store_id:            i32,
}

#[derive(Debug, Serialize, PartialEq, Queryable, Identifiable, Associations)]
#[belongs_to(Store, foreign_key="store")]
pub struct StoreImage {
    pub id:    i32,
    pub store: i32,
    pub src:   String,
}

#[derive(Serialize, Insertable)]
#[table_name="store_images"]
pub struct NewStoreImage {
    pub store: i32,
    pub src:   String,
}

impl NewStoreImage {
    pub fn from_store_images_form(
        store_id: i32, src: String) -> Self {
        NewStoreImage {
            store: store_id,
            src:   src,
        }
    }
}

#[derive(Debug, Serialize, PartialEq, Queryable, Identifiable, Associations)]
#[belongs_to(Store, foreign_key="store")]
pub struct StoreVideo {
    pub id:    i32,
    pub store: i32,
    pub src:   String,
}

#[derive(Serialize, Insertable)]
#[table_name="store_videos"]
pub struct NewStoreVideo {
    pub store: i32,
    pub src:   String,
}

impl NewStoreVideo {
    pub fn from_store_videos_form (
        store_id: i32, src: String) -> Self {
        NewStoreVideo {
            store: store_id,
            src:   src,
        }
    }
}
