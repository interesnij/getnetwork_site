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
#[derive(Debug, Serialize, Identifiable, Queryable)]
#[table_name="tech_categories"]
pub struct TechCategories {
    pub id:             i32,
    pub name:           String,
    pub name_en:        String,
    pub description:    Option<String>,
    pub description_en: Option<String>,
    pub position:       i16,
    pub count:          i16,
    pub level:          i16,
    pub user_id:        i32,
    pub view:           i32,
    pub height:         f64,
    pub seconds:        i32,
}

impl TechCategories {
    pub fn get_serve_categories(&self) -> Vec<ServeCategories> {
        use crate::schema::serve_categories::dsl::serve_categories;

        let _connection = establish_connection();
        return serve_categories
            .filter(schema::serve_categories::category_id.eq(self.id))
            .order(schema::serve_categories::position.asc())
            .load::<ServeCategories>(&_connection)
            .expect("E");
    }
    pub fn get_level_ru(&self) -> String {
        return match self.level {
            0 => "Бюджетно".to_string(),
            1 => "Обычно".to_string(),
            2 => "Средне".to_string(),
            3 => "Сложно".to_string(),
            4 => "Экспертно".to_string(),
            _ => "Непонятно".to_string(),
        };
    }
}
#[derive(Insertable,AsChangeset)]
#[table_name="tech_categories"]
pub struct NewTechCategories {
    pub name:           String,
    pub name_en:        String,
    pub description:    Option<String>,
    pub description_en: Option<String>,
    pub position:       i16,
    pub count:          i16,
    pub level:          i16,
    pub user_id:        i32,
    pub view:           i32,
    pub height:         f64,
    pub seconds:        i32,
}

/////// ServeCategories //////
#[derive(Debug, Serialize, Identifiable, Queryable)]
#[table_name="serve_categories"]
pub struct ServeCategories {
    pub id:             i32,
    pub name:           String,
    pub name_en:        String,
    pub description:    Option<String>,
    pub description_en: Option<String>,
    pub category_id:    i32,
    pub position:       i16,
    pub count:          i16,
    pub default_price:  i32,
    pub user_id:        i32,
    pub view:           i32,
    pub height:         f64,
    pub seconds:        i32,
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
            .filter(schema::serve_categories::category_id.eq_any(tech_cats_ids))
            .load::<ServeCategories>(&_connection)
            .expect("E");
    }

    pub fn get_serves(&self) -> Vec<Serve> {
        use crate::schema::serve::dsl::serve;

        let _connection = establish_connection();
        return serve
            .filter(schema::serve::category_id.eq(self.id))
            .filter(schema::serve::serve_id.is_null())
            .order(schema::serve::position)
            .load::<Serve>(&_connection)
            .expect("E");
    }
    pub fn get_serves_2(&self) -> Vec<Serve> {
        use crate::schema::serve::dsl::serve;

        let _connection = establish_connection();
        return serve
            .filter(schema::serve::category_id.eq(self.id))
            .order(schema::serve::position)
            .load::<Serve>(&_connection)
            .expect("E");
    }
    pub fn get_category(&self) -> TechCategories {
        use crate::schema::tech_categories::dsl::tech_categories;

        let _connection = establish_connection();
        return tech_categories
            .filter(schema::tech_categories::id.eq(self.category_id))
            .first::<TechCategories>(&_connection)
            .expect("E");
    }
}

#[derive(Insertable,AsChangeset)]
#[table_name="serve_categories"]
pub struct NewServeCategories {
    pub name:           String,
    pub name_en:        String,
    pub description:    Option<String>,
    pub description_en: Option<String>,
    pub category_id:    i32,
    pub position:       i16,
    pub count:          i16,
    pub default_price:  i32,
    pub user_id:        i32,
    pub view:           i32,
    pub height:         f64,
    pub seconds:        i32,
}

/////// Serve //////
#[derive(Serialize, Clone, Identifiable, Queryable)]
#[table_name="serve"]
pub struct Serve { 
    pub id:             i32,
    pub name:           String,
    pub name_en:        String,
    pub description:    Option<String>,
    pub description_en: Option<String>,
    pub position:       i16,
    pub category_id:    i32,
    pub price:          i32,
    pub man_hours:      i16, 
    pub is_default:     bool,
    pub user_id:        i32,
    pub tech_cat_id:    i32,
    pub height:         f64,
    pub seconds:        i32,
    pub serve_id:       Option<i32>,
    pub view:           i32,
}
#[derive(Serialize, Queryable)]
pub struct ServeVar {
    pub id:         i32,
    pub name:       String,
    pub name_en:    String,
    pub price:      i32,
    pub man_hours:  i16,
    pub is_default: bool,
}

impl Serve {
    pub fn get_hours(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.man_hours,
            " час".to_string(),
            " часа".to_string(),
            " часов".to_string(),
        );
    }
    pub fn get_variables(&self) -> Vec<ServeVar> {
        use crate::schema::serve::dsl::serve;

        let _connection = establish_connection();
        return serve
            .filter(schema::serve::serve_id.eq(self.id))
            .order(schema::serve::position)
            .select((
                schema::serve::id,
                schema::serve::name,
                schema::serve::name_en,
                schema::serve::price,
                schema::serve::man_hours,
                schema::serve::is_default,
            ))
            .load::<ServeVar>(&_connection)
            .expect("E");
    }
    pub fn get_variables_exclude_id(&self, id: i32) -> Vec<ServeVar> {
        use crate::schema::serve::dsl::serve;

        let _connection = establish_connection();
        return serve
            .filter(schema::serve::serve_id.eq(self.id))
            .filter(schema::serve::id.ne(id))
            .order(schema::serve::position)
            .select((
                schema::serve::id,
                schema::serve::name,
                schema::serve::name_en,
                schema::serve::price,
                schema::serve::man_hours,
                schema::serve::is_default,
            ))
            .load::<ServeVar>(&_connection)
            .expect("E");
    }
    pub fn get_first_variable(&self) -> Serve {
        use crate::schema::serve::dsl::serve;

        let _connection = establish_connection();
        let _serves = serve
            .filter(schema::serve::serve_id.eq(self.id))
            .filter(schema::serve::is_default.eq(true))
            .first::<Serve>(&_connection);
        if _serves.is_ok() {
            return _serves.expect("E");
        }
        else {
            return serve
                .first::<Serve>(&_connection)
                .expect("E");
        }
    }
    pub fn is_parent(&self) -> bool {
        use crate::schema::serve::dsl::serve;

        let _connection = establish_connection();
        return serve
            .filter(schema::serve::serve_id.eq(self.id))
            .select(schema::serve::id)
            .first::<i32>(&_connection)
            .is_ok();
    }
    pub fn get_parent(&self) -> Serve {
        use crate::schema::serve::dsl::serve;

        let _connection = establish_connection();
        return serve
            .filter(schema::serve::id.eq(self.serve_id.unwrap()))
            .first::<Serve>(&_connection)
            .expect("E");  
    }
    pub fn get_category(&self) -> ServeCategories {
        use crate::schema::serve_categories::dsl::serve_categories;

        let _connection = establish_connection();
        return serve_categories
            .filter(schema::serve_categories::id.eq(self.category_id))
            .first::<ServeCategories>(&_connection)
            .expect("E");
    } 
    pub fn get_100_description(&self) -> (String, String) {
        let description: String;
        let description_en: String;
        if self.description.is_some() {
            let _content = self.description.as_deref().unwrap();
            if _content.len() > 100 {
                description = _content[..100].to_string();
            }
            else {
                description = _content.to_string();
            }
        }
        else {
            description = "".to_string();
        }
        if self.description_en.is_some() {
            let _content = self.description_en.as_deref().unwrap();
            if _content.len() > 100 {
                description_en = _content[..100].to_string();
            }
            else {
                description_en = _content.to_string();
            }
        }
        else {
            description_en = "".to_string();
        }
        return (description, description_en);
    }
}

#[derive(Insertable, AsChangeset)]
#[table_name="serve"]
pub struct NewServe {
    pub name:           String,
    pub name_en:        String,
    pub description:    Option<String>,
    pub description_en: Option<String>,
    pub position:       i16,
    pub category_id:    i32,
    pub price:          i32,
    pub man_hours:      i16,
    pub is_default:     bool,
    pub user_id:        i32,
    pub tech_cat_id:    i32,
    pub height:         f64,
    pub seconds:        i32,
    pub serve_id:       Option<i32>,
    pub view:           i32,
}
#[derive(Queryable, Serialize, Deserialize, AsChangeset)]
#[table_name="serve"]
pub struct EditServe {
    pub name:           String,
    pub name_en:        String,
    pub description:    Option<String>,
    pub description_en: Option<String>,
    pub position:       i16,
    pub category_id:    i32,
    pub price:          i32,
    pub man_hours:      i16,
    pub is_default:     bool,
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
/////// ServeItems //////
#[derive(Identifiable, Queryable)]
#[table_name="serve_items"]
pub struct ServeItems {
    pub id:       i32,
    pub serve_id: i32,
    pub item_id:  i32,
    pub types:    i16,
}
#[derive(Insertable)]
#[table_name="serve_items"]
pub struct NewServeItems {
    pub serve_id: i32,
    pub item_id:  i32,
    pub types:    i16,
}

/////// TechCategoriesItem //////
#[derive(Identifiable, Queryable)]
#[table_name="tech_categories_items"]
pub struct TechCategoriesItem {
    pub id:          i32,
    pub category_id: i32,
    pub item_id:     i32,
    pub types:       i16,
    pub is_active:   i16,
}
#[derive(Insertable)]
#[table_name="tech_categories_items"]
pub struct NewTechCategoriesItem {
    pub category_id: i32,
    pub item_id:     i32,
    pub types:       i16,
    pub is_active:   i16,
}
