use actix::Addr;
use serde_json::to_value;
use crate::schema;
use crate::utils::establish_connection;
use crate::diesel::{ExpressionMethods, RunQueryDsl, QueryDsl};
use schema::stat_pages::dsl::stat_pages;
use crate::models::{StatPage, NewStatPage};
use crate::websocket::{MessageToClient, Server};
use actix_web::web::Data;


pub fn plus_page_stat (
    types: i16,
    height: f64,
    seconds: i32,
    websocket_srv: Data<Addr<Server>>
) -> () {
    // статистика страницы главной
    let _connection = establish_connection();

    let _items = stat_pages
        .filter(schema::stat_pages::types.eq(types))
        .limit(1)
        .load::<StatPage>(&_connection)
        .expect("E");

    let _item: StatPage;

    if _items.len() > 0 {
        _item = _items.into_iter().nth(0).unwrap();
        let item_height = format!("{:.2}", _item.height);
        let _height: f64 = item_height.parse().unwrap();
        if _item.now_u > 0 {
            diesel::update(&_item)
                .set ((
                    schema::stat_pages::view.eq(_item.view + 1),
                    schema::stat_pages::height.eq(_height + height),
                    schema::stat_pages::seconds.eq(_item.seconds + seconds),
                    schema::stat_pages::now_u.eq(_item.now_u - 1),
                ))
                .get_result::<StatPage>(&_connection)
                .expect("Error.");
        }
        else {
            diesel::update(&_item)
                .set ((
                    schema::stat_pages::view.eq(_item.view + 1),
                    schema::stat_pages::height.eq(_height + height),
                    schema::stat_pages::seconds.eq(_item.seconds + seconds),
                ))
                .get_result::<StatPage>(&_connection)
                .expect("Error.");
        }
    }
    else {
        let _new_item = NewStatPage {
            types:   types,
            view:    1,
            height:  height,
            seconds: seconds,
            now_u:   0,
        };
        _item = diesel::insert_into(schema::stat_pages::table)
            .values(&_new_item)
            .get_result::<StatPage>(&_connection)
            .expect("Error.");
    }
    if let Ok(history_page) = to_value(_item.now_u.to_string()) {
        let msg = MessageToClient::new("end_page_view", types.into(), history_page);
        websocket_srv.do_send(msg);
    }
    println!("{:?}", _item.now_u);
}

pub fn plus_category_stat(id: i32, height: f64, seconds: i32) -> () {
    // статистика страницы категории блога
    use schema::categories::dsl::categories;
    use crate::models::Categories;

    let _connection = establish_connection();
    let _items = categories
        .filter(schema::categories::id.eq(id))
        .load::<Categories>(&_connection)
        .expect("E");

    if _items.len() > 0 {
        let _item = _items.into_iter().nth(0).unwrap();
        let item_height = format!("{:.2}", _item.height);
        let _height: f64 = item_height.parse().unwrap();
        if _item.now_u > 0 {
            diesel::update(&_item)
                .set ((
                    schema::categories::view.eq(_item.view + 1),
                    schema::categories::height.eq(_height + height),
                    schema::categories::seconds.eq(_item.seconds + seconds),
                    schema::categories::now_u.eq(_item.now_u - 1),
                ))
                .get_result::<Categories>(&_connection)
                .expect("Error.");
        } else {
            diesel::update(&_item)
                .set ((
                    schema::categories::view.eq(_item.view + 1),
                    schema::categories::height.eq(_height + height),
                    schema::categories::seconds.eq(_item.seconds + seconds),
                ))
                .get_result::<Categories>(&_connection)
                .expect("Error.");
        }
    }
}
pub fn plus_item_stat(id: i32, height: f64, seconds: i32) -> () {
    // статистика страницы блога
    use schema::items::dsl::items;
    use crate::models::Item;

    let _connection = establish_connection();
    let _items = items
        .filter(schema::items::id.eq(id))
        .load::<Item>(&_connection)
        .expect("E");

    if _items.len() > 0 {
        let _item = _items.into_iter().nth(0).unwrap();
        let item_height = format!("{:.2}", _item.height);
        let _height: f64 = item_height.parse().unwrap();
        if _item.now_u > 0 {
            diesel::update(&_item)
                .set ((
                    schema::items::view.eq(_item.view + 1),
                    schema::items::height.eq(_height + height),
                    schema::items::seconds.eq(_item.seconds + seconds),
                    schema::items::now_u.eq(_item.now_u - 1),
                ))
                .get_result::<Item>(&_connection)
                .expect("Error.");
        } else {
            diesel::update(&_item)
                .set ((
                    schema::items::view.eq(_item.view + 1),
                    schema::items::height.eq(_height + height),
                    schema::items::seconds.eq(_item.seconds + seconds),
                ))
                .get_result::<Item>(&_connection)
                .expect("Error.");
        }
    }
}

pub fn plus_tag_stat(id: i32, height: f64, seconds: i32) -> () {
    // статистика страницы работы
    use schema::tags::dsl::tags;
    use crate::models::Tag;

    let _connection = establish_connection();
    let _items = tags
        .filter(schema::tags::id.eq(id))
        .load::<Tag>(&_connection)
        .expect("E");

    if _items.len() > 0 {
        let _item = _items.into_iter().nth(0).unwrap();
        let item_height = format!("{:.2}", _item.height);
        let _height: f64 = item_height.parse().unwrap();
        if _item.now_u > 0 {
            diesel::update(&_item)
                .set ((
                    schema::tags::view.eq(_item.view + 1),
                    schema::tags::height.eq(_height + height),
                    schema::tags::seconds.eq(_item.seconds + seconds),
                    schema::tags::now_u.eq(_item.now_u - 1),
                ))
                .get_result::<Tag>(&_connection)
                .expect("Error.");
        } else {
            diesel::update(&_item)
                .set ((
                    schema::tags::view.eq(_item.view + 1),
                    schema::tags::height.eq(_height + height),
                    schema::tags::seconds.eq(_item.seconds + seconds),
                ))
                .get_result::<Tag>(&_connection)
                .expect("Error.");
        }
    }
}
