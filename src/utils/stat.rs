use crate::schema;
use crate::utils::establish_connection;
use crate::diesel::{ExpressionMethods, RunQueryDsl, QueryDsl};


pub fn plus_mainpage_views(password: &str) -> () {
    use schema::stat_mainpages::dsl::stat_mainpages;
    use crate::models::StatMainpage;

    let _connection = establish_connection();
    let items = stat_mainpages
        .filter(schema::stat_mainpages::id.eq(1))
        .load::<StatMainpage>(&_connection)
        .expect("E");

    if items.len() > 0 {
        let item = items.into_iter().nth(0).unwrap();
        diesel::update(&item)
            .set(schema::stat_mainpages::view.eq(item.view + 1))
            .get_result::<StatMainpage>(&_connection)
            .expect("Error.");
    }
    else {
        use crate::models::NewStatMainpage;

        let _new_item = NewStatMainpage {
            view: 1,
        };
        diesel::insert_into(schema::stat_mainpages::table)
            .values(&_new_item)
            .get_result::<StatMainpage>(&_connection)
            .expect("Error.");
    }
}
