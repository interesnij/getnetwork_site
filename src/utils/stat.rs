use crate::schema;
use crate::utils::establish_connection;
use crate::diesel::{ExpressionMethods, RunQueryDsl, QueryDsl};


pub fn plus_mainpage_stat(height: f64, seconds: i32) -> () {
    // статистика страницы главной
    use schema::stat_mainpages::dsl::stat_mainpages;
    use crate::models::StatMainpage;

    let _connection = establish_connection();
    let format_height = format!("{:.2}", height);
    let parse_height: f64 = format_height.parse().unwrap();
    let items = stat_mainpages
        .filter(schema::stat_mainpages::id.eq(1))
        .load::<StatMainpage>(&_connection)
        .expect("E");

    if items.len() > 0 {
        let item = items.into_iter().nth(0).unwrap();
        diesel::update(&item)
            .set ((
                schema::stat_mainpages::view.eq(item.view + 1),
                schema::stat_mainpages::height.eq(item.height + parse_height),
                schema::stat_mainpages::seconds.eq(item.seconds + seconds),
            ))
            .get_result::<StatMainpage>(&_connection)
            .expect("Error.");
    }
    else {
        use crate::models::NewStatMainpage;
        let _new_item = NewStatMainpage {
            view:    1,
            height:  parse_height,
            seconds: seconds,
        };
        diesel::insert_into(schema::stat_mainpages::table)
            .values(&_new_item)
            .get_result::<StatMainpage>(&_connection)
            .expect("Error.");
    }
}

pub fn plus_blog_categories_stat(height: f64, seconds: i32) -> () {
    // статистика страницы всех категорий блога
    use schema::stat_blog_categories::dsl::stat_blog_categories;
    use crate::models::StatBlogCategorie;

    let _connection = establish_connection();
    let items = stat_blog_categories
        .filter(schema::stat_blog_categories::id.eq(1))
        .load::<StatBlogCategorie>(&_connection)
        .expect("E");

    if items.len() > 0 {
        let item = items.into_iter().nth(0).unwrap();
        diesel::update(&item)
            .set ((
                schema::stat_blog_categories::view.eq(item.view + 1),
                schema::stat_blog_categories::height.eq(item.height + height),
                schema::stat_blog_categories::seconds.eq(item.seconds + seconds),
            ))
            .get_result::<StatBlogCategorie>(&_connection)
            .expect("Error.");
    }
    else {
        use crate::models::NewStatBlogCategorie;
        let _new_item = NewStatBlogCategorie {
            view:    1,
            height:  height,
            seconds: seconds,
        };
        diesel::insert_into(schema::stat_blog_categories::table)
            .values(&_new_item)
            .get_result::<StatBlogCategorie>(&_connection)
            .expect("Error.");
    }
}
pub fn plus_blog_category_stat(id: i32, height: f64, seconds: i32) -> () {
    // статистика страницы категории блога
    use schema::blog_categories::dsl::blog_categories;
    use crate::models::BlogCategories;

    let _connection = establish_connection();
    let items = blog_categories
        .filter(schema::blog_categories::id.eq(id))
        .load::<BlogCategories>(&_connection)
        .expect("E");

    if items.len() > 0 {
        let item = items.into_iter().nth(0).unwrap();
        diesel::update(&item)
        .set ((
            schema::blog_categories::view.eq(item.view + 1),
            schema::blog_categories::height.eq(item.height + height),
            schema::blog_categories::seconds.eq(item.seconds + seconds),
        ))
        .get_result::<BlogCategories>(&_connection)
        .expect("Error.");
    }
}
pub fn plus_blog_stat(id: i32, height: f64, seconds: i32) -> () {
    // статистика страницы блога
    use schema::blogs::dsl::blogs;
    use crate::models::Blog;

    let _connection = establish_connection();
    let items = blogs
        .filter(schema::blogs::id.eq(id))
        .load::<Blog>(&_connection)
        .expect("E");

    if items.len() > 0 {
        let item = items.into_iter().nth(0).unwrap();
        diesel::update(&item)
            .set ((
                schema::blogs::view.eq(item.view + 1),
                schema::blogs::height.eq(item.height + height),
                schema::blogs::seconds.eq(item.seconds + seconds),
            ))
            .get_result::<Blog>(&_connection)
            .expect("Error.");
    }
}

pub fn plus_service_categories_stat(height: f64, seconds: i32) -> () {
    // статистика страницы всех категорий услуг
    use schema::stat_service_categories::dsl::stat_service_categories;
    use crate::models::StatServiceCategorie;

    let _connection = establish_connection();
    let items = stat_service_categories
        .filter(schema::stat_service_categories::id.eq(1))
        .load::<StatServiceCategorie>(&_connection)
        .expect("E");

    if items.len() > 0 {
        let item = items.into_iter().nth(0).unwrap();
        diesel::update(&item)
            .set ((
                schema::stat_service_categories::view.eq(item.view + 1),
                schema::stat_service_categories::height.eq(item.height + height),
                schema::stat_service_categories::seconds.eq(item.seconds + seconds),
            ))
            .get_result::<StatServiceCategorie>(&_connection)
            .expect("Error.");
    }
    else {
        use crate::models::NewStatServiceCategorie;
        let _new_item = NewStatServiceCategorie {
            view:    1,
            height:  height,
            seconds: seconds,
        };
        diesel::insert_into(schema::stat_service_categories::table)
            .values(&_new_item)
            .get_result::<StatServiceCategorie>(&_connection)
            .expect("Error.");
    }
}
pub fn plus_service_category_stat(id: i32, height: f64, seconds: i32) -> () {
    // статистика страницы категории услуг
    use schema::service_categories::dsl::service_categories;
    use crate::models::ServiceCategories;

    let _connection = establish_connection();
    let items = service_categories
        .filter(schema::service_categories::id.eq(id))
        .load::<ServiceCategories>(&_connection)
        .expect("E");

    if items.len() > 0 {
        let item = items.into_iter().nth(0).unwrap();
        diesel::update(&item)
            .set ((
                schema::service_categories::view.eq(item.view + 1),
                schema::service_categories::height.eq(item.height + height),
                schema::service_categories::seconds.eq(item.seconds + seconds),
            ))
            .get_result::<ServiceCategories>(&_connection)
            .expect("Error.");
    }
}
pub fn plus_service_stat(id: i32, height: f64, seconds: i32) -> () {
    // статистика страницы услуги
    use schema::services::dsl::services;
    use crate::models::Service;

    let _connection = establish_connection();
    let items = services
        .filter(schema::services::id.eq(id))
        .load::<Service>(&_connection)
        .expect("E");

    if items.len() > 0 {
        let item = items.into_iter().nth(0).unwrap();
        diesel::update(&item)
            .set ((
                schema::services::view.eq(item.view + 1),
                schema::services::height.eq(item.height + height),
                schema::services::seconds.eq(item.seconds + seconds),
            ))
            .get_result::<Service>(&_connection)
            .expect("Error.");
    }
}

pub fn plus_store_categories_stat(height: f64, seconds: i32) -> () {
    // статистика страницы всех категорий товаров
    use schema::stat_store_categories::dsl::stat_store_categories;
    use crate::models::StatStoreCategorie;

    let _connection = establish_connection();
    let items = stat_store_categories
        .filter(schema::stat_store_categories::id.eq(1))
        .load::<StatStoreCategorie>(&_connection)
        .expect("E");

    if items.len() > 0 {
        let item = items.into_iter().nth(0).unwrap();
        diesel::update(&item)
            .set ((
                schema::stat_store_categories::view.eq(item.view + 1),
                schema::stat_store_categories::height.eq(item.height + height),
                schema::stat_store_categories::seconds.eq(item.seconds + seconds),
            ))
            .get_result::<StatStoreCategorie>(&_connection)
            .expect("Error.");
    }
    else {
        use crate::models::NewStatStoreCategorie;
        let _new_item = NewStatStoreCategorie {
            view:    1,
            height:  height,
            seconds: seconds,
        };
        diesel::insert_into(schema::stat_store_categories::table)
            .values(&_new_item)
            .get_result::<StatStoreCategorie>(&_connection)
            .expect("Error.");
    }
}
pub fn plus_store_category_stat(id: i32, height: f64, seconds: i32) -> () {
    // статистика страницы категории товаров
    use schema::store_categories::dsl::store_categories;
    use crate::models::StoreCategories;

    let _connection = establish_connection();
    let items = store_categories
        .filter(schema::store_categories::id.eq(id))
        .load::<StoreCategories>(&_connection)
        .expect("E");

    if items.len() > 0 {
        let item = items.into_iter().nth(0).unwrap();
        diesel::update(&item)
            .set ((
                schema::store_categories::view.eq(item.view + 1),
                schema::store_categories::height.eq(item.height + height),
                schema::store_categories::seconds.eq(item.seconds + seconds),
            ))
            .get_result::<StoreCategories>(&_connection)
            .expect("Error.");
    }
}
pub fn plus_store_stat(id: i32, height: f64, seconds: i32) -> () {
    // статистика страницы товара
    use schema::stores::dsl::stores;
    use crate::models::Store;

    let _connection = establish_connection();
    let items = stores
        .filter(schema::stores::id.eq(id))
        .load::<Store>(&_connection)
        .expect("E");

    if items.len() > 0 {
        let item = items.into_iter().nth(0).unwrap();
        diesel::update(&item)
            .set ((
                schema::stores::view.eq(item.view + 1),
                schema::stores::height.eq(item.height + height),
                schema::stores::seconds.eq(item.seconds + seconds),
            ))
            .get_result::<Store>(&_connection)
            .expect("Error.");
    }
}

pub fn plus_wiki_categories_stat(height: f64, seconds: i32) -> () {
    // статистика страницы всех категорий обучающих статей
    use schema::stat_wiki_categories::dsl::stat_wiki_categories;
    use crate::models::StatWikiCategorie;

    let _connection = establish_connection();
    let items = stat_wiki_categories
        .filter(schema::stat_wiki_categories::id.eq(1))
        .load::<StatWikiCategorie>(&_connection)
        .expect("E");

    if items.len() > 0 {
        let item = items.into_iter().nth(0).unwrap();
        diesel::update(&item)
            .set ((
                schema::stat_wiki_categories::view.eq(item.view + 1),
                schema::stat_wiki_categories::height.eq(item.height + height),
                schema::stat_wiki_categories::seconds.eq(item.seconds + seconds),
            ))
            .get_result::<StatWikiCategorie>(&_connection)
            .expect("Error.");
    }
    else {
        use crate::models::NewStatWikiCategorie;
        let _new_item = NewStatWikiCategorie {
            view:    1,
            height:  height,
            seconds: seconds,
        };
        diesel::insert_into(schema::stat_wiki_categories::table)
            .values(&_new_item)
            .get_result::<StatWikiCategorie>(&_connection)
            .expect("Error.");
    }
}
pub fn plus_wiki_category_stat(id: i32, height: f64, seconds: i32) -> () {
    // статистика страницы категории товаров
    use schema::wiki_categories::dsl::wiki_categories;
    use crate::models::WikiCategories;

    let _connection = establish_connection();
    let items = wiki_categories
        .filter(schema::wiki_categories::id.eq(id))
        .load::<WikiCategories>(&_connection)
        .expect("E");

    if items.len() > 0 {
        let item = items.into_iter().nth(0).unwrap();
        diesel::update(&item)
            .set ((
                schema::wiki_categories::view.eq(item.view + 1),
                schema::wiki_categories::height.eq(item.height + height),
                schema::wiki_categories::seconds.eq(item.seconds + seconds),
            ))
            .get_result::<WikiCategories>(&_connection)
            .expect("Error.");
    }
}
pub fn plus_wiki_stat(id: i32, height: f64, seconds: i32) -> () {
    // статистика страницы товара
    use schema::wikis::dsl::wikis;
    use crate::models::Wiki;

    let _connection = establish_connection();
    let items = wikis
        .filter(schema::wikis::id.eq(id))
        .load::<Wiki>(&_connection)
        .expect("E");

    if items.len() > 0 {
        let item = items.into_iter().nth(0).unwrap();
        diesel::update(&item)
            .set ((
                schema::wikis::view.eq(item.view + 1),
                schema::wikis::height.eq(item.height + height),
                schema::wikis::seconds.eq(item.seconds + seconds),
            ))
            .get_result::<Wiki>(&_connection)
            .expect("Error.");
    }
}

pub fn plus_work_categories_stat(height: f64, seconds: i32) -> () {
    // статистика страницы всех категорий работ
    use schema::stat_work_categories::dsl::stat_work_categories;
    use crate::models::StatWorkCategorie;

    let _connection = establish_connection();
    let items = stat_work_categories
        .filter(schema::stat_work_categories::id.eq(1))
        .load::<StatWorkCategorie>(&_connection)
        .expect("E");

    if items.len() > 0 {
        let item = items.into_iter().nth(0).unwrap();
        diesel::update(&item)
            .set ((
                schema::stat_work_categories::view.eq(item.view + 1),
                schema::stat_work_categories::height.eq(item.height + height),
                schema::stat_work_categories::seconds.eq(item.seconds + seconds),
            ))
            .get_result::<StatWorkCategorie>(&_connection)
            .expect("Error.");
    }
    else {
        use crate::models::NewStatWorkCategorie;
        let _new_item = NewStatWorkCategorie {
            view:    1,
            height:  height,
            seconds: seconds,
        };
        diesel::insert_into(schema::stat_work_categories::table)
            .values(&_new_item)
            .get_result::<StatWorkCategorie>(&_connection)
            .expect("Error.");
    }
}
pub fn plus_work_category_stat(id: i32, height: f64, seconds: i32) -> () {
    // статистика страницы категории работ
    use schema::work_categories::dsl::work_categories;
    use crate::models::WorkCategories;

    let _connection = establish_connection();
    let items = work_categories
        .filter(schema::work_categories::id.eq(id))
        .load::<WorkCategories>(&_connection)
        .expect("E");

    if items.len() > 0 {
        let item = items.into_iter().nth(0).unwrap();
        diesel::update(&item)
            .set ((
                schema::work_categories::view.eq(item.view + 1),
                schema::work_categories::height.eq(item.height + height),
                schema::work_categories::seconds.eq(item.seconds + seconds),
            ))
            .get_result::<WorkCategories>(&_connection)
            .expect("Error.");
    }
}
pub fn plus_work_stat(id: i32, height: f64, seconds: i32) -> () {
    // статистика страницы работы
    use schema::works::dsl::works;
    use crate::models::Work;

    let _connection = establish_connection();
    let items = works
        .filter(schema::works::id.eq(id))
        .load::<Work>(&_connection)
        .expect("E");

    if items.len() > 0 {
        let item = items.into_iter().nth(0).unwrap();
        diesel::update(&item)
            .set ((
                schema::works::view.eq(item.view + 1),
                schema::works::height.eq(item.height + height),
                schema::works::seconds.eq(item.seconds + seconds),
            ))
            .get_result::<Work>(&_connection)
            .expect("Error.");
    }
}

pub fn plus_tags_stat(height: f64, seconds: i32) -> () {
    // статистика страницы всех тегов
    use schema::stat_tags::dsl::stat_tags;
    use crate::models::StatTag;

    let _connection = establish_connection();
    let items = stat_tags
        .filter(schema::stat_tags::id.eq(1))
        .load::<StatTag>(&_connection)
        .expect("E");

    if items.len() > 0 {
        let item = items.into_iter().nth(0).unwrap();
        diesel::update(&item)
            .set ((
                schema::stat_tags::view.eq(item.view + 1),
                schema::stat_tags::height.eq(item.height + height),
                schema::stat_tags::seconds.eq(item.seconds + seconds),
            ))
            .get_result::<StatTag>(&_connection)
            .expect("Error.");
    }
    else {
        use crate::models::NewStatTag;
        let _new_item = NewStatTag {
            view:    1,
            height:  height,
            seconds: seconds,
        };
        diesel::insert_into(schema::stat_tags::table)
            .values(&_new_item)
            .get_result::<StatTag>(&_connection)
            .expect("Error.");
    }
}
pub fn plus_tag_stat(id: i32, height: f64, seconds: i32) -> () {
    // статистика страницы работы
    use schema::tags::dsl::tags;
    use crate::models::Tag;

    let _connection = establish_connection();
    let items = tags
        .filter(schema::tags::id.eq(id))
        .load::<Tag>(&_connection)
        .expect("E");

    if items.len() > 0 {
        let item = items.into_iter().nth(0).unwrap();
        diesel::update(&item)
            .set ((
                schema::tags::view.eq(item.view + 1),
                schema::tags::height.eq(item.height + height),
                schema::tags::seconds.eq(item.seconds + seconds),
            ))
            .get_result::<Tag>(&_connection)
            .expect("Error.");
    }
}

pub fn plus_info_stat(height: f64, seconds: i32) -> () {
    // статистика страницы общей информации
    use schema::stat_infos::dsl::stat_infos;
    use crate::models::StatInfo;

    let _connection = establish_connection();
    let items = stat_infos
        .filter(schema::stat_infos::id.eq(1))
        .load::<StatInfo>(&_connection)
        .expect("E");

    if items.len() > 0 {
        let item = items.into_iter().nth(0).unwrap();
        diesel::update(&item)
            .set ((
                schema::stat_infos::view.eq(item.view + 1),
                schema::stat_infos::height.eq(item.height + height),
                schema::stat_infos::seconds.eq(item.seconds + seconds),
            ))
            .get_result::<StatInfo>(&_connection)
            .expect("Error.");
    }
    else {
        use crate::models::NewStatInfo;
        let _new_item = NewStatInfo {
            view:    1,
            height:  height,
            seconds: seconds,
        };
        diesel::insert_into(schema::stat_infos::table)
            .values(&_new_item)
            .get_result::<StatInfo>(&_connection)
            .expect("Error.");
    }
}

pub fn plus_help_stat(height: f64, seconds: i32) -> () {
    // статистика страницы помощи
    use schema::stat_helps::dsl::stat_helps;
    use crate::models::StatHelp;

    let _connection = establish_connection();
    let items = stat_helps
        .filter(schema::stat_helps::id.eq(1))
        .load::<StatHelp>(&_connection)
        .expect("E");

    if items.len() > 0 {
        let item = items.into_iter().nth(0).unwrap();
        diesel::update(&item)
            .set ((
                schema::stat_helps::view.eq(item.view + 1),
                schema::stat_helps::height.eq(item.height + height),
                schema::stat_helps::seconds.eq(item.seconds + seconds),
            ))
            .get_result::<StatHelp>(&_connection)
            .expect("Error.");
    }
    else {
        use crate::models::NewStatHelp;
        let _new_item = NewStatHelp {
            view:    1,
            height:  height,
            seconds: seconds,
        };
        diesel::insert_into(schema::stat_helps::table)
            .values(&_new_item)
            .get_result::<StatHelp>(&_connection)
            .expect("Error.");
    }
}
