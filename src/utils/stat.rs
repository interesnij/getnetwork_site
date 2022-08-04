use crate::schema;
use crate::utils::establish_connection;
use crate::diesel::{ExpressionMethods, RunQueryDsl, QueryDsl};


pub fn plus_mainpage_views() -> () {
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

pub fn plus_blog_category_views() -> () {
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
            .set(schema::stat_blog_categories::view.eq(item.view + 1))
            .get_result::<StatBlogCategorie>(&_connection)
            .expect("Error.");
    }
    else {
        use crate::models::NewStatBlogCategorie;
        let _new_item = NewStatBlogCategorie {
            view: 1,
        };
        diesel::insert_into(schema::stat_blog_categories::table)
            .values(&_new_item)
            .get_result::<StatBlogCategorie>(&_connection)
            .expect("Error.");
    }
}
pub fn plus_service_category_views() -> () {
    use schema::stat_service_categories::dsl::stat_service_categories;
    use crate::models::StatServicegCategorie;

    let _connection = establish_connection();
    let items = stat_service_categories
        .filter(schema::stat_service_categories::id.eq(1))
        .load::<StatServiceCategorie>(&_connection)
        .expect("E");

    if items.len() > 0 {
        let item = items.into_iter().nth(0).unwrap();
        diesel::update(&item)
            .set(schema::stat_service_categories::view.eq(item.view + 1))
            .get_result::<StatServiceCategorie>(&_connection)
            .expect("Error.");
    }
    else {
        use crate::models::NewStatServiceCategorie;
        let _new_item = NewStatServiceCategorie {
            view: 1,
        };
        diesel::insert_into(schema::stat_service_categories::table)
            .values(&_new_item)
            .get_result::<StatServiceCategorie>(&_connection)
            .expect("Error.");
    }
}
pub fn plus_store_category_views() -> () {
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
            .set(schema::stat_store_categories::view.eq(item.view + 1))
            .get_result::<StatStoreCategorie>(&_connection)
            .expect("Error.");
    }
    else {
        use crate::models::NewStatStoreCategorie;
        let _new_item = NewStatStoreCategorie {
            view: 1,
        };
        diesel::insert_into(schema::stat_store_categories::table)
            .values(&_new_item)
            .get_result::<StatStoreCategorie>(&_connection)
            .expect("Error.");
    }
}
pub fn plus_wiki_category_views() -> () {
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
            .set(schema::stat_wiki_categories::view.eq(item.view + 1))
            .get_result::<StatWikiCategorie>(&_connection)
            .expect("Error.");
    }
    else {
        use crate::models::NewStatWikiCategorie;
        let _new_item = NewStatWikiCategorie {
            view: 1,
        };
        diesel::insert_into(schema::stat_wiki_categories::table)
            .values(&_new_item)
            .get_result::<StatWikiCategorie>(&_connection)
            .expect("Error.");
    }
}
pub fn plus_work_category_views() -> () {
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
            .set(schema::stat_work_categories::view.eq(item.view + 1))
            .get_result::<StatWorkCategorie>(&_connection)
            .expect("Error.");
    }
    else {
        use crate::models::NewStatWorkCategorie;
        let _new_item = NewStatWorkCategorie {
            view: 1,
        };
        diesel::insert_into(schema::stat_work_categories::table)
            .values(&_new_item)
            .get_result::<StatWorkCategorie>(&_connection)
            .expect("Error.");
    }
}


pub fn plus_tags_views() -> () {
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
            .set(schema::stat_tags::view.eq(item.view + 1))
            .get_result::<StatTag>(&_connection)
            .expect("Error.");
    }
    else {
        use crate::models::NewStatTag;
        let _new_item = NewStatTag {
            view: 1,
        };
        diesel::insert_into(schema::stat_tags::table)
            .values(&_new_item)
            .get_result::<StatTag>(&_connection)
            .expect("Error.");
    }
}

pub fn plus_about_views() -> () {
    use schema::stat_abouts::dsl::stat_abouts;
    use crate::models::StatAbout;

    let _connection = establish_connection();
    let items = stat_abouts
        .filter(schema::stat_abouts::id.eq(1))
        .load::<StatAbout>(&_connection)
        .expect("E");

    if items.len() > 0 {
        let item = items.into_iter().nth(0).unwrap();
        diesel::update(&item)
            .set(schema::stat_abouts::view.eq(item.view + 1))
            .get_result::<StatAbout>(&_connection)
            .expect("Error.");
    }
    else {
        use crate::models::NewStatAbout;
        let _new_item = NewStatAbout {
            view: 1,
        };
        diesel::insert_into(schema::stat_abouts::table)
            .values(&_new_item)
            .get_result::<StatAbout>(&_connection)
            .expect("Error.");
    }
}

pub fn plus_info_views() -> () {
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
            .set(schema::stat_infos::view.eq(item.view + 1))
            .get_result::<StatInfo>(&_connection)
            .expect("Error.");
    }
    else {
        use crate::models::NewStatInfo;
        let _new_item = NewStatInfo {
            view: 1,
        };
        diesel::insert_into(schema::stat_infos::table)
            .values(&_new_item)
            .get_result::<StatInfo>(&_connection)
            .expect("Error.");
    }
}

pub fn plus_contact_views() -> () {
    use schema::stat_contacts::dsl::stat_contacts;
    use crate::models::StatContact;

    let _connection = establish_connection();
    let items = stat_contacts
        .filter(schema::stat_contacts::id.eq(1))
        .load::<StatContact>(&_connection)
        .expect("E");

    if items.len() > 0 {
        let item = items.into_iter().nth(0).unwrap();
        diesel::update(&item)
            .set(schema::stat_contacts::view.eq(item.view + 1))
            .get_result::<StatContact>(&_connection)
            .expect("Error.");
    }
    else {
        use crate::models::NewStatContact;
        let _new_item = NewStatContact {
            view: 1,
        };
        diesel::insert_into(schema::stat_contacts::table)
            .values(&_new_item)
            .get_result::<StatContact>(&_connection)
            .expect("Error.");
    }
}

pub fn plus_contact_views() -> () {
    use schema::stat_contacts::dsl::stat_contacts;
    use crate::models::StatContact;

    let _connection = establish_connection();
    let items = stat_contacts
        .filter(schema::stat_contacts::id.eq(1))
        .load::<StatContact>(&_connection)
        .expect("E");

    if items.len() > 0 {
        let item = items.into_iter().nth(0).unwrap();
        diesel::update(&item)
            .set(schema::stat_contacts::view.eq(item.view + 1))
            .get_result::<StatContact>(&_connection)
            .expect("Error.");
    }
    else {
        use crate::models::NewStatContact;
        let _new_item = NewStatContact {
            view: 1,
        };
        diesel::insert_into(schema::stat_contacts::table)
            .values(&_new_item)
            .get_result::<StatContact>(&_connection)
            .expect("Error.");
    }
}

pub fn plus_team_views() -> () {
    use schema::stat_teams::dsl::stat_teams;
    use crate::models::StatTeam;

    let _connection = establish_connection();
    let items = stat_teams
        .filter(schema::stat_teams::id.eq(1))
        .load::<StatTeam>(&_connection)
        .expect("E");

    if items.len() > 0 {
        let item = items.into_iter().nth(0).unwrap();
        diesel::update(&item)
            .set(schema::stat_teams::view.eq(item.view + 1))
            .get_result::<StatTeam>(&_connection)
            .expect("Error.");
    }
    else {
        use crate::models::NewStatTeam;
        let _new_item = NewStatTeam {
            view: 1,
        };
        diesel::insert_into(schema::stat_teams::table)
            .values(&_new_item)
            .get_result::<StatTeam>(&_connection)
            .expect("Error.");
    }
}

pub fn plus_partnership_views() -> () {
    use schema::stat_partnerships::dsl::stat_partnerships;
    use crate::models::StatPartnership;

    let _connection = establish_connection();
    let items = stat_partnerships
        .filter(schema::stat_partnerships::id.eq(1))
        .load::<StatPartnership>(&_connection)
        .expect("E");

    if items.len() > 0 {
        let item = items.into_iter().nth(0).unwrap();
        diesel::update(&item)
            .set(schema::stat_partnerships::view.eq(item.view + 1))
            .get_result::<StatPartnership>(&_connection)
            .expect("Error.");
    }
    else {
        use crate::models::NewStatPartnership;
        let _new_item = NewStatPartnership {
            view: 1,
        };
        diesel::insert_into(schema::stat_partnerships::table)
            .values(&_new_item)
            .get_result::<StatPartnership>(&_connection)
            .expect("Error.");
    }
}

pub fn plus_login_views() -> () {
    use schema::stat_logins::dsl::stat_logins;
    use crate::models::StatLogin;

    let _connection = establish_connection();
    let items = stat_logins
        .filter(schema::stat_logins::id.eq(1))
        .load::<StatLogin>(&_connection)
        .expect("E");

    if items.len() > 0 {
        let item = items.into_iter().nth(0).unwrap();
        diesel::update(&item)
            .set(schema::stat_logins::view.eq(item.view + 1))
            .get_result::<StatLogin>(&_connection)
            .expect("Error.");
    }
    else {
        use crate::models::NewStatLogin;
        let _new_item = NewStatLogin {
            view: 1,
        };
        diesel::insert_into(schema::stat_logins::table)
            .values(&_new_item)
            .get_result::<StatLogin>(&_connection)
            .expect("Error.");
    }
}

pub fn plus_logout_views() -> () {
    use schema::stat_logouts::dsl::stat_logouts;
    use crate::models::StatLogout;

    let _connection = establish_connection();
    let items = stat_logouts
        .filter(schema::stat_logouts::id.eq(1))
        .load::<StatLogout>(&_connection)
        .expect("E");

    if items.len() > 0 {
        let item = items.into_iter().nth(0).unwrap();
        diesel::update(&item)
            .set(schema::stat_logouts::view.eq(item.view + 1))
            .get_result::<StatLogout>(&_connection)
            .expect("Error.");
    }
    else {
        use crate::models::NewStatLogout;
        let _new_item = NewStatLogout {
            view: 1,
        };
        diesel::insert_into(schema::stat_logouts::table)
            .values(&_new_item)
            .get_result::<StatLogout>(&_connection)
            .expect("Error.");
    }
}

pub fn plus_logout_views() -> () {
    use schema::stat_signups::dsl::stat_signups;
    use crate::models::StatSignup;

    let _connection = establish_connection();
    let items = stat_signups
        .filter(schema::stat_signups::id.eq(1))
        .load::<StatSignup>(&_connection)
        .expect("E");

    if items.len() > 0 {
        let item = items.into_iter().nth(0).unwrap();
        diesel::update(&item)
            .set(schema::stat_signups::view.eq(item.view + 1))
            .get_result::<StatSignup>(&_connection)
            .expect("Error.");
    }
    else {
        use crate::models::NewStatSignup;
        let _new_item = NewStatSignup {
            view: 1,
        };
        diesel::insert_into(schema::stat_signups::table)
            .values(&_new_item)
            .get_result::<StatSignup>(&_connection)
            .expect("Error.");
    }
}

pub fn plus_help_views() -> () {
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
            .set(schema::stat_helps::view.eq(item.view + 1))
            .get_result::<StatHelp>(&_connection)
            .expect("Error.");
    }
    else {
        use crate::models::NewStatHelp;
        let _new_item = NewStatHelp {
            view: 1,
        };
        diesel::insert_into(schema::stat_helps::table)
            .values(&_new_item)
            .get_result::<StatHelp>(&_connection)
            .expect("Error.");
    }
}

pub fn plus_profil_views() -> () {
    use schema::stat_profils::dsl::stat_profils;
    use crate::models::StatProfil;

    let _connection = establish_connection();
    let items = stat_profils
        .filter(schema::stat_profils::id.eq(1))
        .load::<StatProfil>(&_connection)
        .expect("E");

    if items.len() > 0 {
        let item = items.into_iter().nth(0).unwrap();
        diesel::update(&item)
            .set(schema::stat_profils::view.eq(item.view + 1))
            .get_result::<StatProfil>(&_connection)
            .expect("Error.");
    }
    else {
        use crate::models::NewStatProfil;
        let _new_item = NewStatProfil {
            view: 1,
        };
        diesel::insert_into(schema::stat_profils::table)
            .values(&_new_item)
            .get_result::<StatProfil>(&_connection)
            .expect("Error.");
    }
}
