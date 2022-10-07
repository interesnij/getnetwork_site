use crate::schema;
use crate::diesel::{
    Queryable,
    Insertable,
    QueryDsl,
    RunQueryDsl,
    ExpressionMethods,
    NullableExpressionMethods,
    PgTextExpressionMethods,
};
use serde::{Serialize,Deserialize};
use crate::models::{
    User,
    Tag,
    TechCategories,
    Serve,
    SmallTag,
    SmallFile,
};

use crate::schema::{
    categories,
    items,
    category,
    item_comments,
};
use crate::utils::establish_connection;
use crate::errors::Error;


///////////
// types:
// 1. блог
// 2. услуга
// 3. товар
// 4. wiki
// 5. работа
// 6. помощь
// 7. заказ

#[derive(Serialize, Queryable)]
pub struct CatDetail {
    pub name:    String,
    pub slug:    String,
    pub count:   i16,
    pub id:      i32,
    pub image:   Option<String>,
    pub view:    i32,
    pub height:  f64,
    pub seconds: i32,
    pub now_u:   i16,
}
#[derive(Serialize, Queryable)]
pub struct Cat {
    pub name:  String,
    pub slug:  String,
    pub count: i16,
    pub id:    i32,
    pub image: Option<String>,
}

#[derive(Serialize, Queryable)]
pub struct SmallCat {
    pub name:  String,
    pub slug:  String,
    pub count: i16,
}
#[derive(Serialize, Queryable)]
pub struct Blog {
    pub slug:        String,
    pub image:       Option<String>,
    pub is_active:   bool,
    pub title:       String,
    pub created:     chrono::NaiveDateTime,
    pub description: Option<String>,
}
#[derive(Serialize, Queryable)]
pub struct Service {
    pub slug:        String,
    pub image:       Option<String>,
    pub is_active:   bool,
    pub title:       String,
    pub description: Option<String>,
}
#[derive(Serialize, Queryable)]
pub struct Store {
    pub slug:        String,
    pub image:       Option<String>,
    pub is_active:   bool,
    pub title:       String,
    pub description: Option<String>,
    pub price:       i32,
    pub price_acc:   Option<i32>,
}
#[derive(Serialize, Queryable)]
pub struct Wiki {
    pub slug:        String,
    pub image:       Option<String>,
    pub is_active:   bool,
    pub title:       String,
    pub description: Option<String>,
    pub created:     chrono::NaiveDateTime,
}
#[derive(Serialize, Queryable)]
pub struct Work {
    pub slug:        String,
    pub image:       Option<String>,
    pub is_active:   bool,
    pub title:       String,
    pub description: Option<String>,
}
#[derive(Serialize, Queryable)]
pub struct Help {
    pub id:        i32,
    pub is_active: bool,
    pub title:     String,
    pub content:   Option<String>,
}
#[derive(Serialize, Queryable)]
pub struct FeaturedItem {
    pub slug:  String,
    pub title: String,
}

#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
#[table_name="categories"]
pub struct Categories {
    pub id:          i32,
    pub name:        String,
    pub description: Option<String>,
    pub position:    i16,
    pub image:       Option<String>,
    pub count:       i16,
    pub view:        i32,
    pub height:      f64,
    pub seconds:     i32,
    pub types:       i16,
    pub slug:        String,
    pub now_u:       i16,
}

impl Categories {
    pub fn get_featured_items (&self, types: i16, id: i32) -> (Option<FeaturedItem>, Option<FeaturedItem>) {
        use crate::schema::{
            category::dsl::category,
            items::dsl::items,
        };

        let _connection = establish_connection();

        let mut prev: Option<FeaturedItem> = None;
        let mut next: Option<FeaturedItem> = None;

        let _category_items = category
            .filter(schema::category::categories_id.eq(self.id))
            .filter(schema::category::types.eq(types))
            .select(schema::category::item_id)
            .load::<i32>(&_connection)
            .expect("E");
        let _category_items_len = _category_items.len();
        for (i, item) in _category_items.iter().enumerate().rev() {
            if item == &id {
                if (i + 1) != _category_items_len {
                    let _next = Some(&_category_items[i + 1]);
                    next = items
                        .filter(schema::items::id.eq(_next.unwrap()))
                        .filter(schema::items::types.eq(types))
                        .filter(schema::items::is_active.eq(true))
                        .select((
                            schema::items::slug,
                            schema::items::title,
                        ))
                        .load::<FeaturedItem>(&_connection)
                        .expect("E")
                        .into_iter()
                        .nth(0);
                };
                if i != 0 {
                    let _prev = Some(&_category_items[i - 1]);
                    prev = items
                        .filter(schema::items::id.eq(_prev.unwrap()))
                        .filter(schema::items::types.eq(types))
                        .filter(schema::items::is_active.eq(true))
                        .select((
                            schema::items::slug,
                            schema::items::title,
                        ))
                        .load::<FeaturedItem>(&_connection)
                        .expect("E")
                        .into_iter()
                        .nth(0);
                };
                break;
            }
        };
        return (prev, next);
    }
    pub fn get_type(&self) -> String {
        return match self.types {
            1 => "блог".to_string(),
            2 => "услуга".to_string(),
            3 => "товар".to_string(),
            4 => "wiki".to_string(),
            5 => "работа".to_string(),
            6 => "помощь".to_string(),
            _ => "Непонятно".to_string(),
        };
    }

    pub fn get_blogs_list (
        cat_id:   i32,
        page:     i32,
        limit:    i32,
        is_admin: bool
    ) -> (Vec<Blog>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Blog>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Categories::get_blogs(cat_id, limit.into(), step.into(), is_admin);
        }
        else {
            have_next = limit + 1;
            object_list = Categories::get_blogs(cat_id, limit.into(), 0, is_admin);
        }
        if Categories::get_blogs(cat_id, 1, have_next.into(), is_admin).len() > 0 {
            next_page_number = page + 1;
        }

        return (object_list, next_page_number);
    }
    pub fn get_blogs (
        cat_id:   i32,
        limit:    i64,
        offset:   i64,
        is_admin: bool
    ) -> Vec<Blog> {
        // 0 object.slug, 1 object.image, 2 object.is_active
        // 3 object.title, 4 object.created,
        // 5 object.get_100_description()
        use crate::schema::{
            items::dsl::items,
            category::dsl::category,
        };

        let _connection = establish_connection();
        let ids = category
            .filter(schema::category::categories_id.eq(cat_id))
            .filter(schema::category::types.eq(1))
            .select(schema::category::item_id)
            .load::<i32>(&_connection)
            .expect("E");
        if is_admin {
             return items
                .filter(schema::items::id.eq_any(ids))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::created,
                    schema::items::description.nullable(),
                ))
                .load::<Blog>(&_connection)
                .expect("E.");
        } else {
            return items
                .filter(schema::items::id.eq_any(ids))
                .filter(schema::items::is_active.eq(true))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::created,
                    schema::items::description.nullable(),
                ))
                .load::<Blog>(&_connection)
                .expect("E.");
        }
    }
    pub fn get_services_list (
        cat_id:   i32,
        page:     i32,
        limit:    i32,
        is_admin: bool
    ) -> (Vec<Service>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Service>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Categories::get_services(cat_id, limit.into(), step.into(), is_admin);
        }
        else {
            have_next = limit + 1;
            object_list = Categories::get_services(cat_id, limit.into(), 0, is_admin);
        }
        if Categories::get_services(cat_id, 1, have_next.into(), is_admin).len() > 0 {
            next_page_number = page + 1;
        }

        return (object_list, next_page_number);
    }
    pub fn get_services (
        cat_id:   i32,
        limit:    i64,
        offset:   i64,
        is_admin: bool
    ) -> Vec<Service> {
        // 0 object.slug, 1 object.image, 2 object.is_active
        // 3 object.title, 4 object.get_100_description()
        use crate::schema::{
            items::dsl::items,
            category::dsl::category,
        };

        let _connection = establish_connection();
        let ids = category
            .filter(schema::category::categories_id.eq(cat_id))
            .filter(schema::category::types.eq(2))
            .select(schema::category::item_id)
            .load::<i32>(&_connection)
            .expect("E");
        if is_admin {
             return items
                .filter(schema::items::id.eq_any(ids))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::description.nullable(),
                ))
                .load::<Service>(&_connection)
                .expect("E.");
        } else {
            return items
                .filter(schema::items::id.eq_any(ids))
                .filter(schema::items::is_active.eq(true))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::description.nullable(),
                ))
                .load::<Service>(&_connection)
                .expect("E.");
        }
    }

    pub fn get_stores_list (
        cat_id:   i32,
        page:     i32,
        limit:    i32,
        is_admin: bool
    ) -> (Vec<Store>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Store>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Categories::get_stores(cat_id, limit.into(), step.into(), is_admin);
        }
        else {
            have_next = limit + 1;
            object_list = Categories::get_stores(cat_id, limit.into(), 0, is_admin);
        }
        if Categories::get_stores(cat_id, 1, have_next.into(), is_admin).len() > 0 {
            next_page_number = page + 1;
        }

        return (object_list, next_page_number);
    }
    pub fn get_stores (
        cat_id:   i32,
        limit:    i64,
        offset:   i64,
        is_admin: bool
    ) -> Vec<Store> {
        // 0 object.slug, 1 object.image, 2 object.is_active
        // 3 object.title, 4 object.get_100_description()
        // 5 object.price, 6 object.price_acc
        use crate::schema::{
            items::dsl::items,
            category::dsl::category,
        };

        let _connection = establish_connection();
        let ids = category
            .filter(schema::category::categories_id.eq(cat_id))
            .filter(schema::category::types.eq(3))
            .select(schema::category::item_id)
            .load::<i32>(&_connection)
            .expect("E");
        if is_admin {
             return items
                .filter(schema::items::id.eq_any(ids))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::description.nullable(),
                    schema::items::price,
                    schema::items::price_acc.nullable(),
                ))
                .load::<Store>(&_connection)
                .expect("E.");
        } else {
            return items
                .filter(schema::items::id.eq_any(ids))
                .filter(schema::items::is_active.eq(true))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::description.nullable(),
                    schema::items::price,
                    schema::items::price_acc.nullable(),
                ))
                .load::<Store>(&_connection)
                .expect("E.");
        }
    }

    pub fn get_wikis_list (
        cat_id:   i32,
        page:     i32,
        limit:    i32,
        is_admin: bool
    ) -> (Vec<Wiki>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Wiki>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Categories::get_wikis(cat_id, limit.into(), step.into(), is_admin);
        }
        else {
            have_next = limit + 1;
            object_list = Categories::get_wikis(cat_id, limit.into(), 0, is_admin);
        }
        if Categories::get_wikis(cat_id, 1, have_next.into(), is_admin).len() > 0 {
            next_page_number = page + 1;
        }

        return (object_list, next_page_number);
    }
    pub fn get_wikis (
        cat_id:   i32,
        limit:    i64,
        offset:   i64,
        is_admin: bool
    ) -> Vec<Wiki> {
        // 0 object.slug, 1 object.image, 2 object.is_active
        // 3 object.title, 4 object.get_100_description()
        // 5 object.created
        use crate::schema::{
            items::dsl::items,
            category::dsl::category,
        };

        let _connection = establish_connection();
        let ids = category
            .filter(schema::category::categories_id.eq(cat_id))
            .filter(schema::category::types.eq(4))
            .select(schema::category::item_id)
            .load::<i32>(&_connection)
            .expect("E");
        if is_admin {
             return items
                .filter(schema::items::id.eq_any(ids))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::description.nullable(),
                    schema::items::created
                ))
                .load::<Wiki>(&_connection)
                .expect("E.");
        } else {
            return items
                .filter(schema::items::id.eq_any(ids))
                .filter(schema::items::is_active.eq(true))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::description.nullable(),
                    schema::items::created
                ))
                .load::<Wiki>(&_connection)
                .expect("E.");
        }
    }

    pub fn get_works_list (
        cat_id:   i32,
        page:     i32,
        limit:    i32,
        is_admin: bool
    ) -> (Vec<Work>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Work>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Categories::get_works(cat_id, limit.into(), step.into(), is_admin);
        }
        else {
            have_next = limit + 1;
            object_list = Categories::get_works(cat_id, limit.into(), 0, is_admin);
        }
        if Categories::get_works(cat_id, 1, have_next.into(), is_admin).len() > 0 {
            next_page_number = page + 1;
        }

        return (object_list, next_page_number);
    }
    pub fn get_works (
        cat_id:   i32,
        limit:    i64,
        offset:   i64,
        is_admin: bool
    ) -> Vec<Work> {
        // 0 object.slug, 1 object.image, 2 object.is_active
        // 3 object.title, 4 object.get_100_description()
        use crate::schema::{
            items::dsl::items,
            category::dsl::category,
        };

        let _connection = establish_connection();
        let ids = category
            .filter(schema::category::categories_id.eq(cat_id))
            .filter(schema::category::types.eq(5))
            .select(schema::category::item_id)
            .load::<i32>(&_connection)
            .expect("E");
        if is_admin {
             return items
                .filter(schema::items::id.eq_any(ids))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::description.nullable()
                ))
                .load::<Work>(&_connection)
                .expect("E.");
        } else {
            return items
                .filter(schema::items::id.eq_any(ids))
                .filter(schema::items::is_active.eq(true))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::description.nullable()
                ))
                .load::<Work>(&_connection)
                .expect("E.");
        }
    }

    pub fn get_helps_list (
        cat_id:   i32,
        page:     i32,
        limit:    i32,
        is_admin: bool
    ) -> (Vec<Help>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Help>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Categories::get_helps(cat_id, limit.into(), step.into(), is_admin);
        }
        else {
            have_next = limit + 1;
            object_list = Categories::get_helps(cat_id, limit.into(), 0, is_admin);
        }
        if Categories::get_helps(cat_id, 1, have_next.into(), is_admin).len() > 0 {
            next_page_number = page + 1;
        }

        return (object_list, next_page_number);
    }
    pub fn get_helps (
        cat_id:   i32,
        limit:    i64,
        offset:   i64,
        is_admin: bool
    ) -> Vec<Help> {
        // 0 object.slug, 1 object.is_active
        // 2 object.title 3 object.content
        use crate::schema::{
            items::dsl::items,
            category::dsl::category,
        };

        let _connection = establish_connection();
        let ids = category
            .filter(schema::category::categories_id.eq(cat_id))
            .filter(schema::category::types.eq(6))
            .select(schema::category::item_id)
            .load::<i32>(&_connection)
            .expect("E");
        if is_admin {
             return items
                .filter(schema::items::id.eq_any(ids))
                .order(schema::items::position.asc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::content
                ))
                .load::<Help>(&_connection)
                .expect("E.");
        } else {
            return items
                .filter(schema::items::id.eq_any(ids))
                .filter(schema::items::is_active.eq(true))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::content
                ))
                .load::<Help>(&_connection)
                .expect("E.");
        }
    }

    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/img.jpg".to_string();
        }
    }
    pub fn get_categories_for_types(types: i16) -> Vec<Cat> {
        use crate::schema::categories::dsl::categories;
        let _connection = establish_connection();
        return categories
            .filter(schema::categories::types.eq(types))
            .select((
                schema::categories::name,
                schema::categories::slug,
                schema::categories::count,
                schema::categories::id,
                schema::categories::image
            ))
            .load::<Cat>(&_connection)
            .expect("E");
    }
}

#[derive(Insertable)]
#[table_name="categories"]
pub struct NewCategories {
    pub name:        String,
    pub description: Option<String>,
    pub position:    i16,
    pub image:       Option<String>,
    pub count:       i16,
    pub view:        i32,
    pub height:      f64,
    pub seconds:     i32,
    pub types:       i16,
    pub slug:        String,
    pub now_u:       i16,
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="categories"]
pub struct EditCategories {
    pub name:        String,
    pub description: Option<String>,
    pub position:    i16,
    pub image:       Option<String>,
    pub slug:        String,
}

#[derive(Debug, Serialize, Clone, Queryable, Identifiable, Associations)]
#[belongs_to(User)]
pub struct Item {
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
    pub types:       i16,
    pub slug:        String,
    pub now_u:       i16,
}

impl Item {
    pub fn get_type(&self) -> String {
        return match self.types {
            1 => "блог".to_string(),
            2 => "услуга".to_string(),
            3 => "товар".to_string(),
            4 => "wiki".to_string(),
            5 => "работа".to_string(),
            6 => "помощь".to_string(),
            _ => "Непонятно".to_string(),
        };
    }
    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/img.jpg".to_string();
        }
    }
    pub fn get_files(&self) -> (
        Vec<SmallFile>, // photos id, src, description
        Vec<SmallFile>, // videos id, src, description
        Vec<SmallFile>, // audios id, src, description
        Vec<SmallFile>  // docs id, src, description
    ) {
        use schema::files::dsl::files;

        let _connection = establish_connection();
        let photos = files
            .filter(schema::files::item_id.eq(self.id))
            .filter(schema::files::item_types.eq(self.types))
            .filter(schema::files::types.eq(1))
            .select((schema::files::id, schema::files::src, schema::files::description.nullable()))
            .load::<SmallFile>(&_connection)
            .expect("E");
        let videos = files
            .filter(schema::files::item_id.eq(self.id))
            .filter(schema::files::item_types.eq(self.types))
            .filter(schema::files::types.eq(2))
            .select((schema::files::id, schema::files::src, schema::files::description.nullable()))
            .load::<SmallFile>(&_connection)
            .expect("E");
        let audios = files
            .filter(schema::files::item_id.eq(self.id))
            .filter(schema::files::item_types.eq(self.types))
            .filter(schema::files::types.eq(3))
            .select((schema::files::id, schema::files::src, schema::files::description.nullable()))
            .load::<SmallFile>(&_connection)
            .expect("E");
        let docs = files
            .filter(schema::files::item_id.eq(self.id))
            .filter(schema::files::item_types.eq(self.types))
            .filter(schema::files::types.eq(4))
            .select((schema::files::id, schema::files::src, schema::files::description.nullable()))
            .load::<SmallFile>(&_connection)
            .expect("E");

        return (photos, videos, audios, docs);
    }
    pub fn get_images_ids(&self) -> Vec<i32> {
        use schema::files::dsl::files;

        let _connection = establish_connection();
        return files
            .filter(schema::files::item_id.eq(self.id))
            .filter(schema::files::types.eq(1))
            .select(schema::files::id)
            .load::<i32>(&_connection)
            .expect("E");
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

    pub fn get_categories(&self) -> Vec<SmallCat> {
        use crate::schema::{
            category::dsl::category,
            categories::dsl::categories,
        };
        // name, slug, count
        let _connection = establish_connection();
        let ids = category
            .filter(schema::category::item_id.eq(self.id))
            .filter(schema::category::types.eq(self.types))
            .select(schema::category::categories_id)
            .load::<i32>(&_connection)
            .expect("E");

        return categories
            .filter(schema::categories::id.eq_any(ids))
            .select((
                schema::categories::name,
                schema::categories::slug,
                schema::categories::count
            ))
            .load::<SmallCat>(&_connection)
            .expect("E");
    }
    pub fn get_categories_obj(&self) -> Vec<Categories> {
        use crate::schema::{
            category::dsl::category,
            categories::dsl::categories,
        };

        let _connection = establish_connection();
        let ids = category
            .filter(schema::category::item_id.eq(self.id))
            .filter(schema::category::types.eq(self.types))
            .select(schema::category::categories_id)
            .load::<i32>(&_connection)
            .expect("E");

        return categories
            .filter(schema::categories::id.eq_any(ids))
            .load::<Categories>(&_connection)
            .expect("E");
    }

    pub fn get_tags(&self) -> Vec<SmallTag> {
        use crate::schema::{
            tags_items::dsl::tags_items,
            tags::dsl::tags,
        };
        let _connection = establish_connection();

        let _tag_items = tags_items
            .filter(schema::tags_items::item_id.eq(&self.id))
            .filter(schema::tags_items::types.eq(self.types))
            .select(schema::tags_items::tag_id)
            .load::<i32>(&_connection)
            .expect("E");
        return tags
            .filter(schema::tags::id.eq_any(_tag_items))
            .select((schema::tags::name, schema::tags::count))
            .load::<SmallTag>(&_connection)
            .expect("E");
    }
    pub fn get_tags_obj(&self) -> Vec<Tag> {
        use crate::schema::{
            tags_items::dsl::tags_items,
            tags::dsl::tags,
        };
        let _connection = establish_connection();

        let _tag_items = tags_items
            .filter(schema::tags_items::item_id.eq(&self.id))
            .filter(schema::tags_items::types.eq(self.types))
            .select(schema::tags_items::tag_id)
            .load::<i32>(&_connection)
            .expect("E");
        return tags
            .filter(schema::tags::id.eq_any(_tag_items))
            .load::<Tag>(&_connection)
            .expect("E");
    }

    pub fn get_blogs (
        limit:    i64,
        offset:   i64,
        is_admin: bool
    ) -> Vec<Blog> {
        use crate::schema::items::dsl::items;

        let _connection = establish_connection();
        if is_admin {
             return items
                .filter(schema::items::types.eq(1))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::created,
                    schema::items::description.nullable(),
                ))
                .load::<Blog>(&_connection)
                .expect("E.");
        } else {
            return items
                .filter(schema::items::types.eq(1))
                .filter(schema::items::is_active.eq(true))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::created,
                    schema::items::description.nullable(),
                ))
                .load::<Blog>(&_connection)
                .expect("E.");
        }
    }
    pub fn search_blogs (
        q:        &String,
        limit:    i64,
        offset:   i64,
        is_admin: bool
    ) -> Vec<Blog> {
        use crate::schema::items::dsl::items;

        let _connection = establish_connection();
        if is_admin {
             return items
                .filter(schema::items::title.ilike(&q))
                .or_filter(schema::items::description.ilike(&q))
                .or_filter(schema::items::content.ilike(&q))
                .filter(schema::items::types.eq(1))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::created,
                    schema::items::description.nullable(),
                ))
                .load::<Blog>(&_connection)
                .expect("E.");
        } else {
            return items
                .filter(schema::items::title.ilike(&q))
                .or_filter(schema::items::description.ilike(&q))
                .or_filter(schema::items::content.ilike(&q))
                .filter(schema::items::types.eq(1))
                .filter(schema::items::is_active.eq(true))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::created,
                    schema::items::description.nullable(),
                ))
                .load::<Blog>(&_connection)
                .expect("E.");
        }
    }

    pub fn get_services (
        limit:    i64,
        offset:   i64,
        is_admin: bool
    ) -> Vec<Service> {
        use crate::schema::items::dsl::items;

        let _connection = establish_connection();
        if is_admin {
             return items
                .filter(schema::items::types.eq(2))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::description.nullable(),
                ))
                .load::<Service>(&_connection)
                .expect("E.");
        } else {
            return items
                .filter(schema::items::types.eq(2))
                .filter(schema::items::is_active.eq(true))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::description.nullable(),
                ))
                .load::<Service>(&_connection)
                .expect("E.");
        }
    }
    pub fn search_services (
        q:        &String,
        limit:    i64,
        offset:   i64,
        is_admin: bool
    ) -> Vec<Service> {
        use crate::schema::items::dsl::items;

        let _connection = establish_connection();
        if is_admin {
             return items
                .filter(schema::items::title.ilike(&q))
                .or_filter(schema::items::description.ilike(&q))
                .or_filter(schema::items::content.ilike(&q))
                .filter(schema::items::types.eq(2))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::description.nullable(),
                ))
                .load::<Service>(&_connection)
                .expect("E.");
        } else {
            return items
                .filter(schema::items::title.ilike(&q))
                .or_filter(schema::items::description.ilike(&q))
                .or_filter(schema::items::content.ilike(&q))
                .filter(schema::items::types.eq(2))
                .filter(schema::items::is_active.eq(true))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::description.nullable(),
                ))
                .load::<Service>(&_connection)
                .expect("E.");
        }
    }

    pub fn get_stores (
          limit:    i64,
          offset:   i64,
          is_admin: bool
      ) -> Vec<Store> {
          use crate::schema::items::dsl::items;

          let _connection = establish_connection();
          if is_admin {
               return items
                  .filter(schema::items::types.eq(3))
                  .order(schema::items::created.desc())
                  .limit(limit)
                  .offset(offset)
                  .select((
                      schema::items::slug,
                      schema::items::image.nullable(),
                      schema::items::is_active,
                      schema::items::title,
                      schema::items::description.nullable(),
                      schema::items::price,
                      schema::items::price_acc.nullable(),
                  ))
                  .load::<Store>(&_connection)
                  .expect("E.");
          } else {
              return items
                  .filter(schema::items::types.eq(3))
                  .filter(schema::items::is_active.eq(true))
                  .order(schema::items::created.desc())
                  .limit(limit)
                  .offset(offset)
                  .select((
                      schema::items::slug,
                      schema::items::image.nullable(),
                      schema::items::is_active,
                      schema::items::title,
                      schema::items::description.nullable(),
                      schema::items::price,
                      schema::items::price_acc.nullable(),
                  ))
                  .load::<Store>(&_connection)
                  .expect("E.");
          }
    }
    pub fn search_stores (
          q:        &String,
          limit:    i64,
          offset:   i64,
          is_admin: bool
      ) -> Vec<Store> {
          use crate::schema::items::dsl::items;

          let _connection = establish_connection();
          if is_admin {
               return items
                  .filter(schema::items::title.ilike(&q))
                  .or_filter(schema::items::description.ilike(&q))
                  .or_filter(schema::items::content.ilike(&q))
                  .filter(schema::items::types.eq(3))
                  .order(schema::items::created.desc())
                  .limit(limit)
                  .offset(offset)
                  .select((
                      schema::items::slug,
                      schema::items::image.nullable(),
                      schema::items::is_active,
                      schema::items::title,
                      schema::items::description.nullable(),
                      schema::items::price,
                      schema::items::price_acc.nullable(),
                  ))
                  .load::<Store>(&_connection)
                  .expect("E.");
          } else {
              return items
                  .filter(schema::items::title.ilike(&q))
                  .or_filter(schema::items::description.ilike(&q))
                  .or_filter(schema::items::content.ilike(&q))
                  .filter(schema::items::types.eq(3))
                  .filter(schema::items::is_active.eq(true))
                  .order(schema::items::created.desc())
                  .limit(limit)
                  .offset(offset)
                  .select((
                      schema::items::slug,
                      schema::items::image.nullable(),
                      schema::items::is_active,
                      schema::items::title,
                      schema::items::description.nullable(),
                      schema::items::price,
                      schema::items::price_acc.nullable(),
                  ))
                  .load::<Store>(&_connection)
                  .expect("E.");
          }
    }

    pub fn get_works (
          limit:    i64,
          offset:   i64,
          is_admin: bool
      ) -> Vec<Work> {
          use crate::schema::items::dsl::items;

          let _connection = establish_connection();
          if is_admin {
               return items
                  .filter(schema::items::types.eq(5))
                  .order(schema::items::created.desc())
                  .limit(limit)
                  .offset(offset)
                  .select((
                      schema::items::slug,
                      schema::items::image.nullable(),
                      schema::items::is_active,
                      schema::items::title,
                      schema::items::description.nullable()
                  ))
                  .load::<Work>(&_connection)
                  .expect("E.");
          } else {
              return items
                  .filter(schema::items::types.eq(5))
                  .filter(schema::items::is_active.eq(true))
                  .order(schema::items::created.desc())
                  .limit(limit)
                  .offset(offset)
                  .select((
                      schema::items::slug,
                      schema::items::image.nullable(),
                      schema::items::is_active,
                      schema::items::title,
                      schema::items::description.nullable()
                  ))
                  .load::<Work>(&_connection)
                  .expect("E.");
        }
    }
    pub fn search_works (
          q:        &String,
          limit:    i64,
          offset:   i64,
          is_admin: bool
      ) -> Vec<Work> {
          use crate::schema::items::dsl::items;

          let _connection = establish_connection();
          if is_admin {
               return items
                  .filter(schema::items::title.ilike(&q))
                  .or_filter(schema::items::description.ilike(&q))
                  .or_filter(schema::items::content.ilike(&q))
                  .filter(schema::items::types.eq(5))
                  .order(schema::items::created.desc())
                  .limit(limit)
                  .offset(offset)
                  .select((
                      schema::items::slug,
                      schema::items::image.nullable(),
                      schema::items::is_active,
                      schema::items::title,
                      schema::items::description.nullable()
                  ))
                  .load::<Work>(&_connection)
                  .expect("E.");
          } else {
              return items
                  .filter(schema::items::title.ilike(&q))
                  .or_filter(schema::items::description.ilike(&q))
                  .or_filter(schema::items::content.ilike(&q))
                  .filter(schema::items::types.eq(5))
                  .filter(schema::items::is_active.eq(true))
                  .order(schema::items::created.desc())
                  .limit(limit)
                  .offset(offset)
                  .select((
                      schema::items::slug,
                      schema::items::image.nullable(),
                      schema::items::is_active,
                      schema::items::title,
                      schema::items::description.nullable()
                  ))
                  .load::<Work>(&_connection)
                  .expect("E.");
        }
    }

    pub fn get_wikis (
          limit:    i64,
          offset:   i64,
          is_admin: bool
      ) -> Vec<Wiki> {
          use crate::schema::items::dsl::items;

          let _connection = establish_connection();
          if is_admin {
               return items
                  .filter(schema::items::types.eq(4))
                  .order(schema::items::created.desc())
                  .limit(limit)
                  .offset(offset)
                  .select((
                      schema::items::slug,
                      schema::items::image.nullable(),
                      schema::items::is_active,
                      schema::items::title,
                      schema::items::description.nullable(),
                      schema::items::created,
                  ))
                  .load::<Wiki>(&_connection)
                  .expect("E.");
          } else {
              return items
                  .filter(schema::items::types.eq(4))
                  .filter(schema::items::is_active.eq(true))
                  .order(schema::items::created.desc())
                  .limit(limit)
                  .offset(offset)
                  .select((
                      schema::items::slug,
                      schema::items::image.nullable(),
                      schema::items::is_active,
                      schema::items::title,
                      schema::items::description.nullable(),
                      schema::items::created
                  ))
                  .load::<Wiki>(&_connection)
                  .expect("E.");
        }
    }
    pub fn search_wikis (
          q:        &String,
          limit:    i64,
          offset:   i64,
          is_admin: bool
      ) -> Vec<Wiki> {
          use crate::schema::items::dsl::items;

          let _connection = establish_connection();
          if is_admin {
               return items
                  .filter(schema::items::title.ilike(&q))
                  .or_filter(schema::items::description.ilike(&q))
                  .or_filter(schema::items::content.ilike(&q))
                  .filter(schema::items::types.eq(4))
                  .order(schema::items::created.desc())
                  .limit(limit)
                  .offset(offset)
                  .select((
                      schema::items::slug,
                      schema::items::image.nullable(),
                      schema::items::is_active,
                      schema::items::title,
                      schema::items::description.nullable(),
                      schema::items::created
                  ))
                  .load::<Wiki>(&_connection)
                  .expect("E.");
          } else {
              return items
                  .filter(schema::items::title.ilike(&q))
                  .or_filter(schema::items::description.ilike(&q))
                  .or_filter(schema::items::content.ilike(&q))
                  .filter(schema::items::types.eq(4))
                  .filter(schema::items::is_active.eq(true))
                  .order(schema::items::created.desc())
                  .limit(limit)
                  .offset(offset)
                  .select((
                      schema::items::slug,
                      schema::items::image.nullable(),
                      schema::items::is_active,
                      schema::items::title,
                      schema::items::description.nullable(),
                      schema::items::created,
                  ))
                  .load::<Wiki>(&_connection)
                  .expect("E.");
        }
    }

    pub fn get_helps (
        limit:    i64,
        offset:   i64,
        is_admin: bool
    ) -> Vec<Help> {
        use crate::schema::items::dsl::items;

        let _connection = establish_connection();
        if is_admin {
             return items
                .filter(schema::items::types.eq(6))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::content
                ))
                .load::<Help>(&_connection)
                .expect("E.");
        } else {
            return items
                .filter(schema::items::types.eq(6))
                .filter(schema::items::is_active.eq(true))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::content
                ))
                .load::<Help>(&_connection)
                .expect("E.");
        }
    }

    pub fn search_helps (
        q:        &String,
        limit:    i64,
        offset:   i64,
        is_admin: bool
    ) -> Vec<Help> {
        // 0 object.slug, 1 object.is_active
        // 2 object.title 3 object.content
        use crate::schema::items::dsl::items;

        let _connection = establish_connection();
        if is_admin {
             return items
                .filter(schema::items::title.ilike(&q))
                .or_filter(schema::items::description.ilike(&q))
                .or_filter(schema::items::content.ilike(&q))
                .filter(schema::items::types.eq(6))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::content,
                ))
                .load::<Help>(&_connection)
                .expect("E.");
        } else {
            return items
                .filter(schema::items::title.ilike(&q))
                .or_filter(schema::items::description.ilike(&q))
                .or_filter(schema::items::content.ilike(&q))
                .filter(schema::items::is_active.eq(true))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::content
                ))
                .load::<Help>(&_connection)
                .expect("E.");
        }
    }

    pub fn get_blogs_list_for_ids (
        page:  i32,
        limit: i32,
        ids:   &Vec<i32>,
        is_admin: bool
    ) -> (Vec<Blog>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Blog>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Item::get_blogs_for_ids(limit.into(), step.into(), &ids, is_admin);
        }
        else {
            have_next = limit + 1;
            object_list = Item::get_blogs_for_ids(limit.into(), 0, &ids, is_admin);
        }
        if Item::get_blogs_for_ids(1, have_next.into(), &ids, is_admin).len() > 0 {
            next_page_number = page + 1;
        }
        return (object_list, next_page_number);
    }

    pub fn get_blogs_for_ids (
        limit:  i64,
        offset: i64,
        ids:    &Vec<i32>,
        is_admin: bool
    ) -> Vec<Blog> {
        use crate::schema::items::dsl::items;

        let _connection = establish_connection();
        if is_admin {
            return items
                .filter(schema::items::id.eq_any(ids))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::created,
                    schema::items::description.nullable()
                ))
                .load::<Blog>(&_connection)
                .expect("E.");
        }
        else {
            return items
                .filter(schema::items::id.eq_any(ids))
                .filter(schema::items::is_active.eq(true))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::created,
                    schema::items::description.nullable()
                ))
                .load::<Blog>(&_connection)
                .expect("E.");
        }
    }

    pub fn get_services_list_for_ids (
        page:  i32,
        limit: i32,
        ids:   &Vec<i32>,
        is_admin: bool
    ) -> (Vec<Service>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Service>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Item::get_services_for_ids(limit.into(), step.into(), &ids, is_admin);
        }
        else {
            have_next = limit + 1;
            object_list = Item::get_services_for_ids(limit.into(), 0, &ids, is_admin);
        }
        if Item::get_services_for_ids(1, have_next.into(), &ids, is_admin).len() > 0 {
            next_page_number = page + 1;
        }
        return (object_list, next_page_number);
    }

    pub fn get_services_for_ids (
        limit:  i64,
        offset: i64,
        ids:    &Vec<i32>,
        is_admin: bool
    ) -> Vec<Service> {
        use crate::schema::items::dsl::items;

        let _connection = establish_connection();
        if is_admin {
            return items
                .filter(schema::items::id.eq_any(ids))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::description.nullable()
                ))
                .load::<Service>(&_connection)
                .expect("E.");
        }
        else {
            return items
                .filter(schema::items::id.eq_any(ids))
                .filter(schema::items::is_active.eq(true))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::description.nullable()
                ))
                .load::<Service>(&_connection)
                .expect("E.");
        }
    }

    pub fn get_stores_list_for_ids (
        page:  i32,
        limit: i32,
        ids:   &Vec<i32>,
        is_admin: bool
    ) -> (Vec<Store>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Store>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Item::get_stores_for_ids(limit.into(), step.into(), &ids, is_admin);
        }
        else {
            have_next = limit + 1;
            object_list = Item::get_stores_for_ids(limit.into(), 0, &ids, is_admin);
        }
        if Item::get_stores_for_ids(1, have_next.into(), &ids, is_admin).len() > 0 {
            next_page_number = page + 1;
        }
        return (object_list, next_page_number);
    }

    pub fn get_stores_for_ids (
        limit:  i64,
        offset: i64,
        ids:    &Vec<i32>,
        is_admin: bool
    ) -> Vec<Store> {
        use crate::schema::items::dsl::items;

        let _connection = establish_connection();
        if is_admin {
            return items
                .filter(schema::items::id.eq_any(ids))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::description.nullable(),
                    schema::items::price,
                    schema::items::price_acc.nullable()
                ))
                .load::<Store>(&_connection)
                .expect("E.");
        }
        else {
            return items
                .filter(schema::items::id.eq_any(ids))
                .filter(schema::items::is_active.eq(true))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::description.nullable(),
                    schema::items::price,
                    schema::items::price_acc.nullable()
                ))
                .load::<Store>(&_connection)
                .expect("E.");
        }
    }

    pub fn get_wikis_list_for_ids (
        page:  i32,
        limit: i32,
        ids:   &Vec<i32>,
        is_admin: bool
    ) -> (Vec<Wiki>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Wiki>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Item::get_wikis_for_ids(limit.into(), step.into(), &ids, is_admin);
        }
        else {
            have_next = limit + 1;
            object_list = Item::get_wikis_for_ids(limit.into(), 0, &ids, is_admin);
        }
        if Item::get_wikis_for_ids(1, have_next.into(), &ids, is_admin).len() > 0 {
            next_page_number = page + 1;
        }
        return (object_list, next_page_number);
    }

    pub fn get_wikis_for_ids (
        limit:  i64,
        offset: i64,
        ids:    &Vec<i32>,
        is_admin: bool
    ) -> Vec<Wiki> {
        use crate::schema::items::dsl::items;

        let _connection = establish_connection();
        if is_admin {
            return items
                .filter(schema::items::id.eq_any(ids))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::description.nullable(),
                    schema::items::created,
                ))
                .load::<Wiki>(&_connection)
                .expect("E.");
        }
        else {
            return items
                .filter(schema::items::id.eq_any(ids))
                .filter(schema::items::is_active.eq(true))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::description.nullable(),
                    schema::items::created,
                ))
                .load::<Wiki>(&_connection)
                .expect("E.");
        }
    }

    pub fn get_works_list_for_ids (
        page:  i32,
        limit: i32,
        ids:   &Vec<i32>,
        is_admin: bool
    ) -> (Vec<Work>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Work>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Item::get_works_for_ids(limit.into(), step.into(), &ids, is_admin);
        }
        else {
            have_next = limit + 1;
            object_list = Item::get_works_for_ids(limit.into(), 0, &ids, is_admin);
        }
        if Item::get_works_for_ids(1, have_next.into(), &ids, is_admin).len() > 0 {
            next_page_number = page + 1;
        }
        return (object_list, next_page_number);
    }

    pub fn get_works_for_ids (
        limit:  i64,
        offset: i64,
        ids:    &Vec<i32>,
        is_admin: bool
    ) -> Vec<Work> {
        use crate::schema::items::dsl::items;

        let _connection = establish_connection();
        if is_admin {
            return items
                .filter(schema::items::id.eq_any(ids))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::description.nullable()
                ))
                .load::<Work>(&_connection)
                .expect("E.");
        }
        else {
            return items
                .filter(schema::items::id.eq_any(ids))
                .filter(schema::items::is_active.eq(true))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::description.nullable()
                ))
                .load::<Work>(&_connection)
                .expect("E.");
        }
    }

    pub fn get_helps_list_for_ids (
        page:  i32,
        limit: i32,
        ids:   &Vec<i32>,
        is_admin: bool
    ) -> (Vec<Help>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Help>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Item::get_helps_for_ids(limit.into(), step.into(), &ids, is_admin);
        }
        else {
            have_next = limit + 1;
            object_list = Item::get_helps_for_ids(limit.into(), 0, &ids, is_admin);
        }
        if Item::get_helps_for_ids(1, have_next.into(), &ids, is_admin).len() > 0 {
            next_page_number = page + 1;
        }
        return (object_list, next_page_number);
    }

    pub fn get_helps_for_ids (
        limit:  i64,
        offset: i64,
        ids:    &Vec<i32>,
        is_admin: bool
    ) -> Vec<Help> {
        use crate::schema::items::dsl::items;

        let _connection = establish_connection();
        if is_admin {
            return items
                .filter(schema::items::id.eq_any(ids))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::description.nullable()
                ))
                .load::<Help>(&_connection)
                .expect("E.");
        }
        else {
            return items
                .filter(schema::items::id.eq_any(ids))
                .filter(schema::items::is_active.eq(true))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::description.nullable()
                ))
                .load::<Help>(&_connection)
                .expect("E.");
        }
    }

    pub fn get_serves_ids(&self) -> Vec<i32> {
        use schema::serve_items::dsl::serve_items;

        let _connection = establish_connection();
        return serve_items
            .filter(schema::serve_items::item_id.eq(&self.id))
            .filter(schema::serve_items::types.eq(self.types))
            .select(schema::serve_items::serve_id)
            .load::<i32>(&_connection)
            .expect("E");
    }
    pub fn get_serves(&self) -> Vec<Serve> {
        use schema::{
            serve_items::dsl::serve_items,
            serve::dsl::serve,
        };

        let _connection = establish_connection();
        let _items = serve_items
            .filter(schema::serve_items::item_id.eq(&self.id))
            .filter(schema::serve_items::types.eq(self.types))
            .select(schema::serve_items::serve_id)
            .load::<i32>(&_connection)
            .expect("E");

        return serve
            .filter(schema::serve::id.eq_any(_items))
            .load::<Serve>(&_connection)
            .expect("E");
    }
    pub fn get_open_tech_categories(&self, types: i16) -> Vec<TechCategories> {
        // получаем открытые тех.категории товара
        use schema::{
            tech_categories_items::dsl::tech_categories_items,
            tech_categories::dsl::tech_categories,
        };

        let _connection = establish_connection();
        let ids = tech_categories_items
            .filter(schema::tech_categories_items::item_id.eq(&self.id))
            .filter(schema::tech_categories_items::types.eq(types))
            .filter(schema::tech_categories_items::is_active.eq(1))
            .select(schema::tech_categories_items::category_id)
            .load::<i32>(&_connection)
            .expect("E");

        return tech_categories
            .filter(schema::tech_categories::id.eq_any(ids))
            .order(schema::tech_categories::position.desc())
            .load::<TechCategories>(&_connection)
            .expect("E");
    }
    pub fn get_close_tech_categories(&self, types: i16) -> Vec<TechCategories> {
        // получаем закрытые тех.категории товара
        use schema::{
            tech_categories_items::dsl::tech_categories_items,
            tech_categories::dsl::tech_categories,
        };

        let _connection = establish_connection();
        let ids = tech_categories_items
            .filter(schema::tech_categories_items::item_id.eq(&self.id))
            .filter(schema::tech_categories_items::types.eq(types))
            .filter(schema::tech_categories_items::is_active.eq(2))
            .select(schema::tech_categories_items::category_id)
            .load::<i32>(&_connection)
            .expect("E");

        return tech_categories
            .filter(schema::tech_categories::id.eq_any(ids))
            .order(schema::tech_categories::position.desc())
            .load::<TechCategories>(&_connection)
            .expect("E");
    }
    pub fn get_close_tech_cats_ids(&self, types: i16) -> Vec<i32> {
        use schema::tech_categories_items::dsl::tech_categories_items;

        let _connection = establish_connection();
        return tech_categories_items
            .filter(schema::tech_categories_items::item_id.eq(&self.id))
            .filter(schema::tech_categories_items::types.eq(types))
            .filter(schema::tech_categories_items::is_active.eq(2))
            .select(schema::tech_categories_items::category_id)
            .load::<i32>(&_connection)
            .expect("E");
    }
}

#[derive(Serialize, Insertable)]
#[table_name="items"]
pub struct NewItem {
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
    pub types:       i16,
    pub slug:        String,
    pub now_u:       i16,
}

impl NewItem {
    pub fn create (
        title:       String,
        description: Option<String>,
        link:        Option<String>,
        image:       Option<String>,
        user_id:     i32,
        position:    i16,
        types:       i16,
        slug:        String
    ) -> Self {
        use chrono::Duration;

        NewItem {
            title:       title,
            description: description,
            content:     None,
            link:        link,
            image:       image,
            is_active:   false,
            price:       0,
            user_id:     user_id,
            created:     chrono::Local::now().naive_utc() + Duration::hours(3),
            position:    position,
            view:        0,
            height:      0.0,
            seconds:     0,
            price_acc:   None,
            types:       types,
            slug:        slug,
            now_u:       0,
        }
    }
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="items"]
pub struct EditItem {
    pub title:       String,
    pub description: Option<String>,
    pub link:        Option<String>,
    pub image:       Option<String>,
    pub position:    i16,
    pub slug:        String,
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
#[derive(Identifiable, Queryable, Associations)]
#[table_name="category"]
pub struct Category {
    pub id:            i32,
    pub categories_id: i32,
    pub item_id:       i32,
    pub types:         i16,
}

#[derive(Insertable)]
#[table_name="category"]
pub struct NewCategory {
    pub categories_id: i32,
    pub item_id:       i32,
    pub types:         i16,
}

#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
pub struct ItemComment {
    pub id:        i32,
    pub comment:   String,
    pub blog_id:   i32,
    pub user_id:   i32,
    pub parent_id: Option<i32>,
    pub created:   chrono::NaiveDateTime,
}

#[derive(Serialize, Insertable)]
#[table_name="item_comments"]
pub struct NewItemComment {
    pub comment:   String,
    pub item_id:   i32,
    pub user_id:   i32,
    pub parent_id: Option<i32>,
    pub created:   chrono::NaiveDateTime,
}

impl NewItemComment {
    pub fn new (comment: String, item_id: i32,
        user_id: i32, parent_id: Option<i32>) -> Self {
        use chrono::Duration;

        NewItemComment {
            comment:   comment,
            item_id:   item_id,
            user_id:   user_id,
            parent_id: parent_id,
            created:   chrono::Local::now().naive_utc() + Duration::hours(3),
        }
    }
}
