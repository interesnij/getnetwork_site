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
use crate::schema::{
    tech_categories,
    serve_categories,
    serve,
    serve_items,
};
use crate::utils::establish_connection;


/////// TechCategories //////
#[derive(Debug, Serialize, Identifiable, Queryable, Associations)]
#[table_name="tech_categories"]
pub struct TechCategories {
    pub id:          i32,
    pub name:        String,
    pub description: Option<String>,
    pub position:    i32,
    pub count:       i32,
}
#[derive(Insertable,AsChangeset)]
#[table_name="tech_categories"]
pub struct NewTechCategories {
    pub name:        String,
    pub description: Option<String>,
    pub position:    i32,
    pub count:       i32,
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
    pub position:        i32,
    pub count:           i32,
    pub default_price:   i32,
}

#[derive(Insertable,AsChangeset)]
#[table_name="serve_categories"]
pub struct NewServeCategories {
    pub name:            String,
    pub description:     Option<String>,
    pub cat_name:        String,
    pub tech_categories: i32,
    pub position:        i32,
    pub count:           i32,
    pub default_price:   i32,
}

/////// Serve //////
#[derive(Debug, Serialize, Identifiable, Queryable, Associations)]
#[belongs_to(ServeCategories, foreign_key="serve_categories")]
#[table_name="serve"]
pub struct Serve {
    pub id:               i32,
    pub name:             String,
    pub cat_name:         String,
    pub description:      Option<String>,
    pub position:         i32,
    pub serve_categories: Option<i32>,
    pub price:            Option<i32>,
    pub price_acc:        Option<i32>,
    pub social_price:     Option<i32>,
    pub man_hours:        Option<i32>,
    pub is_default:       bool,
    pub user_id:          i32,
}

impl Serve {
    pub fn get_100_description(&self) -> String {
        let _content = self.description.clone();
        if _content.len() > 100 {
            return _content[..100].to_string();
        }
        else {
            return _content.to_string();
        }
    }
}

#[derive(Insertable,AsChangeset)]
#[table_name="serve"]
pub struct NewServe {
    pub name:             String,
    pub cat_name:         String,
    pub description:      Option<String>,
    pub position:         i32,
    pub serve_categories: Option<i32>,
    pub price:            Option<i32>,
    pub price_acc:        Option<i32>,
    pub social_price:     Option<i32>,
    pub man_hours:        Option<i32>,
    pub is_default:       bool,
    pub user_id:          i32,
}
#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="serve"]
pub struct EditServe {
    pub name:             String,
    pub cat_name:         String,
    pub description:      Option<String>,
    pub position:         i32,
    pub serve_categories: Option<i32>,
    pub price:            Option<i32>,
    pub price_acc:        Option<i32>,
    pub social_price:     Option<i32>,
    pub man_hours:        Option<i32>,
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
}
#[derive(Insertable)]
#[table_name="serve_items"]
pub struct NewServeItems {
    pub serve_id:   i32,
    pub service_id: i32,
    pub store_id:   i32,
    pub work_id:    i32,
}
