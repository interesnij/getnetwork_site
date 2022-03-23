
//////////////////// users //////////////
table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        bio -> Nullable<Text>,
        image -> Nullable<Text>,
    }
}

//////////////////// users //////////////
table! {
    feedbacks (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        message -> Text,
    }
}

//////////////////// blog //////////////

// Таблица блога
table! {
    blog_categories (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        blog_position -> Int4,
        image -> Nullable<Text>,
        blog_count -> Int4,
    }
}
table! {
    blogs (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Varchar>,
        content -> Nullable<Text>,
        link -> Nullable<Text>,
        image -> Nullable<Text>,
        is_blog_active -> Bool,
        creator -> Int4,
        blog_created -> Timestamp,
    }
}
table! {
    blog_category (id) {
        id -> Int4,
        blog_categories_id -> Int4,
        blog_id -> Int4,
    }
}

table! {
    blog_images (id) {
        id -> Int4,
        blog -> Int4,
        src -> Text,
    }
}
table! {
    blog_videos (id) {
        id -> Int4,
        blog -> Int4,
        src -> Text,
    }
}

table! {
    blog_comments (id) {
        id -> Int4,
        comment -> Varchar,
        blog_id -> Int4,
        user_id -> Int4,
        parent_comment_id -> Nullable<Int4>,
        created_at -> Timestamp,
    }
}

//////////////////// work //////////////

table! {
    work_categories (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        work_position -> Int4,
        image -> Nullable<Text>,
        work_count -> Int4,
    }
}

table! {
    works (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Varchar>,
        content -> Nullable<Text>,
        link -> Nullable<Text>,
        image -> Nullable<Text>,
        is_work_active -> Bool,
        creator -> Int4,
        work_created -> Timestamp,
    }
}


table! {
    work_category (id) {
        id -> Int4,
        work_categories_id -> Int4,
        work_id -> Int4,
    }
}

table! {
    work_images (id) {
        id -> Int4,
        work -> Int4,
        src -> Text,
    }
}
table! {
    work_videos (id) {
        id -> Int4,
        work -> Int4,
        src -> Text,
    }
}

//////////////////// store //////////////
table! {
    store_categories (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        store_position -> Int4,
        image -> Nullable<Text>,
        store_count -> Int4,
    }
}
table! {
    stores (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Varchar>,
        content -> Nullable<Text>,
        link -> Nullable<Text>,
        image -> Nullable<Text>,
        is_store_active -> Bool,
        price -> Int4,
        price_acc -> Nullable<Int4>,
        social_price -> Nullable<Int4>,
        creator -> Int4,
        store_created -> Timestamp,
    }
}
table! {
    store_category (id) {
        id -> Int4,
        store_categories_id -> Int4,
        store_id -> Int4,
    }
}

table! {
    store_images (id) {
        id -> Int4,
        store -> Int4,
        src -> Text,
    }
}
table! {
    store_videos (id) {
        id -> Int4,
        store -> Int4,
        src -> Text,
    }
}

//////////////////// wiki //////////////
table! {
    wiki_categories (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        wiki_position -> Int4,
        image -> Nullable<Text>,
        wiki_count -> Int4,
    }
}
table! {
    wikis (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Varchar>,
        content -> Nullable<Text>,
        link -> Nullable<Text>,
        image -> Nullable<Text>,
        is_wiki_active -> Bool,
        creator -> Int4,
        wiki_created -> Timestamp,
    }
}
table! {
    wiki_category (id) {
        id -> Int4,
        wiki_categories_id -> Int4,
        wiki_id -> Int4,
    }
}

table! {
    wiki_images (id) {
        id -> Int4,
        wiki -> Int4,
        src -> Text,
    }
}
table! {
    wiki_videos (id) {
        id -> Int4,
        wiki -> Int4,
        src -> Text,
    }
}

//////////////////// service //////////////
table! {
    service_categories (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        service_position -> Int4,
        image -> Nullable<Text>,
        service_count -> Int4,
    }
}
table! {
    services (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Varchar>,
        content -> Nullable<Text>,
        link -> Nullable<Text>,
        image -> Nullable<Text>,
        is_service_active -> Bool,
        creator -> Int4,
        service_created -> Timestamp,
    }
}
table! {
    service_category (id) {
        id -> Int4,
        service_categories_id -> Int4,
        service_id -> Int4,
    }
}

table! {
    service_images (id) {
        id -> Int4,
        service -> Int4,
        src -> Text,
    }
}
table! {
    service_videos (id) {
        id -> Int4,
        service -> Int4,
        src -> Text,
    }
}

//////////////////// serve //////////////

table! {
    tech_categories (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        tech_position -> Int4,
        tech_count -> Int4,
    }
}

table! {
    serve_categories (id) {
        id -> Int4,
        name -> Varchar,
        cat_name -> Varchar,
        tech_categories -> Int4,
        serve_position -> Int4,
        serve_count -> Int4,
    }
}

table! {
    serve (id) {
        id -> Int4,
        name -> Varchar,
        cat_name -> Varchar,
        description -> Text,
        serve_position -> Int4,
        serve_categories -> Int4,
        price -> Int4,
        price_acc -> Nullable<Int4>,
        social_price -> Nullable<Int4>,
        man_hours -> Int4,
    }
}

table! {
    serve_items (id) {
        id -> Int4,
        serve_id -> Int4,
        service_id -> Int4,
        store_id -> Int4,
        work_id -> Int4,
    }
}

//////////////////// tags //////////////
table! {
    tags (id) {
        id -> Int4,
        name -> Varchar,
        tag_position -> Int4,
        tag_count -> Int4,
        blog_count -> Int4,
        service_count -> Int4,
        store_count -> Int4,
        wiki_count -> Int4,
        work_count -> Int4,
    }
}
table! {
    tags_items (id) {
        id -> Int4,
        tag_id -> Int4,
        service_id -> Int4,
        store_id -> Int4,
        blog_id -> Int4,
        wiki_id -> Int4,
        work_id -> Int4,
        tag_created -> Timestamp,
    }
}

//////////////////// end tables //////////////

joinable!(wikis -> users (creator));
joinable!(wiki_images -> wikis (wiki));
joinable!(wiki_videos -> wikis (wiki));
joinable!(wiki_category -> wiki_categories (wiki_categories_id));
joinable!(wiki_category -> wikis (wiki_id));

joinable!(works -> users (creator));
joinable!(work_images -> works (work));
joinable!(work_videos -> works (work));
joinable!(work_category -> work_categories (work_categories_id));
joinable!(work_category -> works (work_id));

joinable!(services -> users (creator));
joinable!(service_images -> services (service));
joinable!(service_videos -> services (service));
joinable!(service_category -> service_categories (service_categories_id));
joinable!(service_category -> services (service_id));

joinable!(stores -> users (creator));
joinable!(store_images -> stores (store));
joinable!(store_videos -> stores (store));
joinable!(store_category -> store_categories (store_categories_id));
joinable!(store_category -> stores (store_id));

joinable!(blogs -> users (creator));
joinable!(blog_images -> blogs (blog));
joinable!(blog_videos -> blogs (blog));
joinable!(blog_category -> blog_categories (blog_categories_id));
joinable!(blog_category -> blogs (blog_id));
joinable!(blog_comments -> blogs (blog_id));
joinable!(blog_comments -> users (user_id));

allow_tables_to_appear_in_same_query!(
    users,

    blog_categories,
    blog_category,
    blogs,
    blog_images,
    blog_videos,
    blog_comments,

    works,
    work_categories,
    work_category,
    work_images,
    work_videos,

    stores,
    store_categories,
    store_category,
    store_images,
    store_videos,

    wikis,
    wiki_categories,
    wiki_category,
    wiki_images,
    wiki_videos,

    services,
    service_categories,
    service_category,
    service_images,
    service_videos,
);
