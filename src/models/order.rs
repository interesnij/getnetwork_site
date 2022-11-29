use crate::schema;
use crate::diesel::{
    Queryable,
    Insertable,
    QueryDsl,
    RunQueryDsl,
    ExpressionMethods,
};
use serde::{Serialize, Deserialize};
use crate::models::{Serve, TechCategories};
use crate::schema::{
    orders,
    order_files,
};
use crate::utils::establish_connection;


#[derive(Debug, Serialize, Identifiable, Queryable, Associations)]
#[table_name="orders"]
pub struct Order {
    pub id:          i32,
    pub title:       String,
    pub types:       i16,
    pub object_id:   i32,
    pub username:    String,
    pub email:       String,
    pub description: Option<String>,
    pub created:     chrono::NaiveDateTime,
    pub user_id:     i32,
    pub price:       i32,
    pub price_acc:   Option<i32>,
}

impl Order {
    pub fn get_orders_list(page: i32, limit: i32) -> (Vec<Order>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Order>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Order::get_orders(limit.into(), step.into());
        }
        else {
            have_next = limit + 1;
            object_list = Order::get_orders(limit.into(), 0);
        }
        if Order::get_orders(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        return (object_list, next_page_number);
    }
    pub fn get_orders(limit: i64, offset: i64) -> Vec<Order> {
        use crate::schema::orders::dsl::orders;

        let _connection = establish_connection();
        return orders
            .order(schema::orders::created.desc())
            .limit(limit)
            .offset(offset)
            .load::<Order>(&_connection)
            .expect("E.");
    }
    pub fn get_user_orders_list(user_id: i32, page: i32, limit: i32) -> (Vec<Order>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Order>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Order::get_user_orders(user_id, limit.into(), step.into());
        }
        else {
            have_next = limit + 1;
            object_list = Order::get_user_orders(user_id, limit.into(), 0);
        }
        if Order::get_user_orders(user_id, 1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        return (object_list, next_page_number);
    }
    pub fn get_user_orders(user_id: i32, limit: i64, offset: i64) -> Vec<Order> {
        use crate::schema::orders::dsl::orders;

        let _connection = establish_connection();
        return orders
            .filter(schema::orders::user_id.eq(user_id))
            .order(schema::orders::created.desc())
            .limit(limit)
            .offset(offset)
            .load::<Order>(&_connection)
            .expect("E.");
    }
    pub fn get_serves(&self) -> Vec<Serve> {
        use schema::serve_items::dsl::serve_items;
        use schema::serve::dsl::serve;

        let _connection = establish_connection();
        let _serve_items = serve_items
            .filter(schema::serve_items::item_id.eq(&self.id))
            .filter(schema::serve_items::types.eq(7))
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
            .filter(schema::serve_items::item_id.eq(&self.id))
            .filter(schema::serve_items::types.eq(7))
            .select(schema::serve_items::serve_id)
            .load::<i32>(&_connection)
            .expect("E");
    }
    pub fn get_open_tech_categories(&self) -> Vec<TechCategories> {
        // получаем открытые тех.категории элемента
        use schema::{
            tech_categories_items::dsl::tech_categories_items,
            tech_categories::dsl::tech_categories,
        };

        let _connection = establish_connection();
        let ids = tech_categories_items
            .filter(schema::tech_categories_items::item_id.eq(&self.id))
            .filter(schema::tech_categories_items::types.eq(7))
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
}

#[derive(Insertable)]
#[table_name="orders"]
pub struct NewOrder {
    pub title:       String,
    pub types:       i16,
    pub object_id:   i32,
    pub username:    String,
    pub email:       String,
    pub description: Option<String>,
    pub created:     chrono::NaiveDateTime,
    pub user_id:     i32,
    pub price:       i32,
}
impl NewOrder {
    pub fn create (
        title:       String,
        types:       i16,
        object_id:   i32,
        username:    String,
        email:       String,
        description: Option<String>,
        user_id:     i32,
    ) -> Self {
        use chrono::Duration;

        NewOrder {
            title:       title,
            types:       types,
            object_id:   object_id,
            username:    username,
            email:       email,
            description: description,
            created:     chrono::Local::now().naive_utc() + Duration::hours(3),
            user_id:     user_id,
            price:       0,
        }
    }
}
#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="orders"]
pub struct EditOrder {
    pub username:    String,
    pub email:       String,
    pub description: Option<String>,
}


#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
pub struct OrderFile {
    pub id:       i32,
    pub order_id: i32,
    pub src:      String,
}

#[derive(Serialize, Insertable)]
#[table_name="order_files"]
pub struct NewOrderFile {
    pub order_id: i32,
    pub src:      String,
}

impl NewOrderFile {
    pub fn create (order_id: i32, src: String) -> Self {
        NewOrderFile {
            order_id: order_id,
            src:      src,
        }
    }
}
