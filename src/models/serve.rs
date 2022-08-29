use crate::schema;
use crate::diesel::{
    Queryable,
    Insertable,
    QueryDsl,
    RunQueryDsl,
    ExpressionMethods,
};
use serde::{Serialize, Deserialize};
use crate::schema::{
    tech_categories,
    serve_categories,
    serve,
    serve_items,
    tech_categories_items,
};
use crate::utils::establish_connection;


/////// TechCategories //////
#[derive(Debug, Serialize, Identifiable, Queryable, Associations)]
#[table_name="tech_categories"]
pub struct TechCategories {
    pub id:          i32,
    pub name:        String,
    pub description: Option<String>,
    pub position:    i16,
    pub count:       i16,
    pub level:       i16,
    pub user_id:     i32,
    pub view:        i32,
    pub height:      f64,
    pub seconds:     i32,
}

impl TechCategories {
    pub fn get_serve_categories(&self) -> Vec<ServeCategories> {
        use crate::schema::serve_categories::dsl::serve_categories;

        let _connection = establish_connection();
        return serve_categories
            .filter(schema::serve_categories::tech_categories.eq(self.id))
            .order(schema::serve_categories::position)
            .load::<ServeCategories>(&_connection)
            .expect("E");
    }
}
#[derive(Insertable,AsChangeset)]
#[table_name="tech_categories"]
pub struct NewTechCategories {
    pub name:        String,
    pub description: Option<String>,
    pub position:    i16,
    pub count:       i16,
    pub level:       i16,
    pub user_id:     i32,
    pub view:        i32,
    pub height:      f64,
    pub seconds:     i32,
}

/////// ServeCategories //////
#[derive(Debug, Serialize, Identifiable, Queryable, Associations)]
#[table_name="serve_categories"]
pub struct ServeCategories {
    pub id:              i32,
    pub name:            String,
    pub description:     Option<String>,
    pub cat_name:        String,
    pub tech_categories: i32,
    pub position:        i16,
    pub count:           i16,
    pub default_price:   i32,
    pub user_id:         i32,
    pub view:            i32,
    pub height:          f64,
    pub seconds:         i32,
}
impl ServeCategories {
    pub fn get_categories_from_level(level: &i16) -> Vec<ServeCategories> {
        use crate::schema::{
            serve_categories::dsl::serve_categories,
            tech_categories::dsl::tech_categories,
        };

        let _connection = establish_connection();
        let tech_cats_ids = tech_categories
            .filter(schema::tech_categories::level.eq(level))
            .select(schema::tech_categories::id)
            .load::<i32>(&_connection)
            .expect("E");

        return serve_categories
            .filter(schema::serve_categories::tech_categories.eq_any(tech_cats_ids))
            .load::<ServeCategories>(&_connection)
            .expect("E");
    }

    pub fn get_serves(&self) -> Vec<Serve> {
        use crate::schema::serve::dsl::serve;

        let _connection = establish_connection();
        return serve
            .filter(schema::serve::serve_categories.eq(self.id))
            .filter(schema::serve::serve_id.is_null())
            .order(schema::serve::position)
            .load::<Serve>(&_connection)
            .expect("E");
    }
}

#[derive(Insertable,AsChangeset)]
#[table_name="serve_categories"]
pub struct NewServeCategories {
    pub name:            String,
    pub description:     Option<String>,
    pub cat_name:        String,
    pub tech_categories: i32,
    pub position:        i16,
    pub count:           i16,
    pub default_price:   i32,
    pub user_id:         i32,
    pub view:            i32,
    pub height:          f64,
    pub seconds:         i32,
}

/////// Serve //////
#[derive(Debug, Serialize, Clone, Identifiable, Queryable, Associations)]
#[belongs_to(ServeCategories, foreign_key="serve_categories")]
#[table_name="serve"]
pub struct Serve {
    pub id:               i32,
    pub name:             String,
    pub cat_name:         String,
    pub description:      Option<String>,
    pub position:         i16,
    pub serve_categories: i32,
    pub price:            i32,
    pub man_hours:        i16,
    pub is_default:       bool,
    pub user_id:          i32,
    pub tech_cat_id:      i32,
    pub view:             i32,
    pub height:           f64,
    pub seconds:          i32,
    pub serve_id:         Option<i32>,
}

impl Serve {
    pub fn get_variables(&self) -> Vec<Serve> {
        use crate::schema::serve::dsl::serve;

        let _connection = establish_connection();
        return serve
            .filter(schema::serve::serve_id.eq(self.id))
            .order(schema::serve::position)
            .load::<Serve>(&_connection)
            .expect("E");
    }
    pub fn get_variables_exclude_id(&self, id: i32) -> Vec<Serve> {
        use crate::schema::serve::dsl::serve;

        let _connection = establish_connection();
        return serve
            .filter(schema::serve::serve_id.eq(self.id))
            .filter(schema::serve::id.ne(id))
            .order(schema::serve::position)
            .load::<Serve>(&_connection)
            .expect("E");
    }
    pub fn get_first_variable(&self) -> Serve {
        use crate::schema::serve::dsl::serve;

        let _connection = establish_connection();
        let _serves = serve
            .filter(schema::serve::serve_id.eq(self.id))
            .order(schema::serve::position.asc())
            .limit(1)
            .load::<Serve>(&_connection)
            .expect("E");
        if _serves.len() > 0 {
            return _serves.into_iter().nth(0).unwrap();
        }
        else {
            return serve
                .limit(1)
                .load::<Serve>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();
        }
    }
    pub fn is_parent(&self) -> bool {
        use crate::schema::serve::dsl::serve;

        let _connection = establish_connection();
        return serve
            .filter(schema::serve::serve_id.eq(self.id))
            .select(schema::serve::id)
            .load::<i32>(&_connection)
            .expect("E")
            .len() > 0;
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
}

#[derive(Insertable,AsChangeset)]
#[table_name="serve"]
pub struct NewServe {
    pub name:             String,
    pub cat_name:         String,
    pub description:      Option<String>,
    pub position:         i16,
    pub serve_categories: i32,
    pub price:            i32,
    pub man_hours:        i16,
    pub is_default:       bool,
    pub user_id:          i32,
    pub tech_cat_id:      i32,
    pub view:             i32,
    pub height:           f64,
    pub seconds:          i32,
    pub serve_id:         Option<i32>,
}
#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="serve"]
pub struct EditServe {
    pub name:             String,
    pub cat_name:         String,
    pub description:      Option<String>,
    pub position:         i16,
    pub serve_categories: i32,
    pub price:            i32,
    pub man_hours:        i16,
    pub is_default:       bool,
}

/////// ServeItems //////
#[derive(Identifiable, Queryable, Associations)]
#[table_name="serve_items"]
pub struct ServeItems {
    pub id:         i32,
    pub serve_id:   i32,
    pub service_id: i32,
    pub store_id:   i32,
    pub work_id:    i32,
    pub orders_id:  Option<i32>,
}
#[derive(Insertable)]
#[table_name="serve_items"]
pub struct NewServeItems {
    pub serve_id:   i32,
    pub service_id: i32,
    pub store_id:   i32,
    pub work_id:    i32,
    pub orders_id:  Option<i32>,
}

/////// ServeItems //////
#[derive(Identifiable, Queryable, Associations)]
#[table_name="tech_categories_items"]
pub struct TechCategoriesItem {
    pub id:          i32,
    pub category_id: i32,
    pub service_id:  i32,
    pub store_id:    i32,
    pub work_id:     i32,
    pub types:       i16, // 1 активно, 2 неактивно
    pub orders_id:   Option<i32>,
}
#[derive(Insertable)]
#[table_name="tech_categories_items"]
pub struct NewTechCategoriesItem {
    pub category_id: i32,
    pub service_id:  i32,
    pub store_id:    i32,
    pub work_id:     i32,
    pub types:       i16,
    pub orders_id:   Option<i32>,
}
