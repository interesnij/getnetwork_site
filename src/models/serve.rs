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
use crate::utils::{
    establish_connection,
    get_linguage_storage,
    CategoriesForm
};
use crate::models::User;


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
    pub fn update_category_with_id(user: User, cat_id: i32, form: CategoriesForm) -> i16 {
        let _connection = establish_connection();
        let l = get_linguage_storage();
        let cat = schema::tech_categories::table
            .filter(schema::tech_categories::id.eq(cat_id))
            .first::<TechCategories>(&_connection)
            .expect("E.");
        if user.perm < 60 && cat.user_id != user.id {
            return 0;
        }
        if l == 1 { 
            diesel::update(&cat)
                .set((
                    schema::tech_categories::name.eq(&form.name),
                    schema::tech_categories::description.eq(&form.description),
                    schema::tech_categories::position.eq(form.position),
                    //schema::tech_categories::image.eq(&form.image),
                    schema::tech_categories::level.eq(form.level),
                ))
                .execute(&_connection)
                .expect("E");
        }
        else if l == 2 {
            diesel::update(&cat)
                .set((
                    schema::tech_categories::name_en.eq(&form.name),
                    schema::tech_categories::description_en.eq(&form.description),
                    schema::tech_categories::position.eq(form.position),
                    //schema::tech_categories::image.eq(&form.image),
                    schema::tech_categories::level.eq(form.level),
                ))
                .execute(&_connection)
                .expect("E");
        }
        return 1;
    }
    pub fn create(user_id: i32, form: CategoriesForm) -> i16 {
        let _connection = establish_connection();
        let l = get_linguage_storage();
        if l == 1 {
            let new_cat = NewTechCategories {
                name:           form.name.clone(),
                name_en:        "".to_string(),
                description:    Some(form.description.clone()),
                description_en: None,
                position:       form.position,
                count:          0,
                level:          form.level,
                user_id:        user_id,
                view:           0,
                height:         0.0,
                seconds:        0,
            };
            let _new_tech = diesel::insert_into(tech_categories::table)
                .values(&new_cat)
                .execute(&_connection)
                .expect("E.");
        }
        else if l == 2 {
            let new_cat = NewTechCategories {
                name:           "".to_string(),
                name_en:        form.name.clone(),
                description:    None,
                description_en: Some(form.description.clone()),
                position:       form.position,
                count:          0,
                level:          form.level,
                user_id:        user_id,
                view:           0,
                height:         0.0,
                seconds:        0,
            };
            let _new_tech = diesel::insert_into(tech_categories::table)
                .values(&new_cat)
                .execute(&_connection)
                .expect("E.");
        }
        return 1;
    }
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
    pub fn update_category_with_id(user: User, cat_id: i32, form: crate::utils::ServeCategoriesForm) -> i16 {
        let _connection = establish_connection();
        let l = get_linguage_storage();
        let cat = schema::serve_categories::table
            .filter(schema::serve_categories::id.eq(cat_id))
            .first::<ServeCategories>(&_connection)
            .expect("E.");
        if user.perm < 60 && cat.user_id != user.id {
            return 0;
        }
        if l == 1 { 
            diesel::update(&cat)
                .set((
                    schema::serve_categories::name.eq(&form.name),
                    schema::serve_categories::description.eq(&form.description),
                    schema::serve_categories::position.eq(form.position),
                    //schema::serve_categories::image.eq(&form.image),
                ))
                .execute(&_connection)
                .expect("E");
        }
        else if l == 2 {
            diesel::update(&cat)
                .set((
                    schema::serve_categories::name_en.eq(&form.name),
                    schema::serve_categories::description_en.eq(&form.description),
                    schema::serve_categories::position.eq(form.position),
                    //schema::serve_categories::image.eq(&form.image),
                )) 
                .execute(&_connection)
                .expect("E");
        }
        return 1;
    }
    pub fn create(user_id: i32, form: crate::utils::ServeCategoriesForm) -> i16 {
        let _connection = establish_connection();
        let l = get_linguage_storage();
        if l == 1 {
            let new_cat = NewServeCategories {  
                name:           form.name.clone(),
                name_en:        "".to_string(),
                description:    Some(form.description.clone()),
                description_en: None,
                category_id:    form.category_id,
                position:       form.position,
                count:          0,
                default_price:  form.default_price,
                user_id:        user_id,
                view:           0,
                height:         0.0,
                seconds:        0,
            };
            diesel::insert_into(serve_categories::table)
                .values(&new_cat)
                .execute(&_connection)
                .expect("E.");
        }
        else if l == 2 {
            let new_cat = NewServeCategories {
                name:           "".to_string(),
                name_en:        form.name.clone(),
                description:    None,
                description_en: Some(form.description.clone()),
                category_id:    form.category_id,
                position:       form.position,
                count:          0,
                default_price:  form.default_price,
                user_id:        user_id,
                view:           0,
                height:         0.0,
                seconds:        0,
            };
            diesel::insert_into(serve_categories::table)
                .values(&new_cat)
                .execute(&_connection)
                .expect("E.");
        }
        return 1;
    }
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
    pub fn get_100_description(&self, l: u8) -> String {
        if l == 1 {
            if self.description.is_some() {
                let _content = self.description.as_deref().unwrap();
                if _content.len() > 100 {
                    return _content[..100].to_string();
                }
                return _content.to_string();
            }
        }
        else if l == 2 {
            if self.description_en.is_some() {
                let _content = self.description_en.as_deref().unwrap();
                if _content.len() > 100 {
                    return _content[..100].to_string();
                }
                return _content.to_string();
            }
        }
        return "".to_string();
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
