use crate::schema;
use crate::utils::establish_connection;
use crate::diesel::{ExpressionMethods, RunQueryDsl, QueryDsl};
use schema::{items::dsl::items,categories::dsl::categories};
use crate::models::{Item, Categories};


pub fn get_categories_for_types_obj(types: i16) -> Vec<Categories> {
    use crate::schema::categories::dsl::categories;
    // name, slug, count
    let _connection = establish_connection();
    return categories
        .filter(schema::categories::types.eq(types))
        .load::<Categories>(&_connection)
        .expect("E");
}
// cat_slug, object.slug, object.get_image(), object.is_active,
// object.title, object.created, object.get_100_description(),
