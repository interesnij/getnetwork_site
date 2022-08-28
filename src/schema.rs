table! {
    blog_categories (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        position -> Int2,
        image -> Nullable<Varchar>,
        count -> Int2,
        view -> Int4,
        height -> Float8,
        seconds -> Int4,
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
    blog_comments (id) {
        id -> Int4,
        comment -> Varchar,
        blog_id -> Int4,
        user_id -> Int4,
        parent_id -> Nullable<Int4>,
        created -> Timestamp,
    }
}

table! {
    blog_images (id) {
        id -> Int4,
        blog -> Int4,
        src -> Varchar,
    }
}

table! {
    blog_videos (id) {
        id -> Int4,
        blog -> Int4,
        src -> Varchar,
    }
}

table! {
    blogs (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Varchar>,
        content -> Nullable<Varchar>,
        link -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        is_active -> Bool,
        user_id -> Int4,
        created -> Timestamp,
        view -> Int4,
        height -> Float8,
        seconds -> Int4,
    }
}

table! {
    cookie_stats (id) {
        id -> Int4,
        user_id -> Int4,
        page -> Int2,
        link -> Varchar,
        title -> Varchar,
        height -> Float8,
        seconds -> Int4,
        created -> Timestamp,
    }
}

table! {
    cookie_users (id) {
        id -> Int4,
        ip -> Varchar,
        device -> Int2,
        city_ru -> Nullable<Varchar>,
        city_en -> Nullable<Varchar>,
        region_ru -> Nullable<Varchar>,
        region_en -> Nullable<Varchar>,
        country_ru -> Nullable<Varchar>,
        country_en -> Nullable<Varchar>,
        height -> Float8,
        seconds -> Int4,
        created -> Timestamp,
    }
}

table! {
    feedbacks (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        message -> Varchar,
    }
}

table! {
    help_item_categories (id) {
        id -> Int4,
        title -> Varchar,
    }
}

table! {
    help_items (id) {
        id -> Int4,
        category_id -> Int4,
        title -> Varchar,
        content -> Varchar,
    }
}

table! {
    order_files (id) {
        id -> Int4,
        order_id -> Int4,
        src -> Varchar,
    }
}

table! {
    orders (id) {
        id -> Int4,
        title -> Varchar,
        types -> Int2,
        object_id -> Int4,
        username -> Varchar,
        email -> Varchar,
        description -> Nullable<Varchar>,
        created -> Timestamp,
        user_id -> Int4,
    }
}

table! {
    serve (id) {
        id -> Int4,
        name -> Varchar,
        cat_name -> Varchar,
        description -> Nullable<Varchar>,
        position -> Int2,
        serve_categories -> Int4,
        price -> Int4,
        man_hours -> Int2,
        is_default -> Bool,
        user_id -> Int4,
        tech_cat_id -> Int4,
        view -> Int4,
        height -> Float8,
        seconds -> Int4,
        serve_id -> Nullable<Int4>,
    }
}

table! {
    serve_categories (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        cat_name -> Varchar,
        tech_categories -> Int4,
        position -> Int2,
        count -> Int2,
        default_price -> Int4,
        user_id -> Int4,
        view -> Int4,
        height -> Float8,
        seconds -> Int4,
    }
}

table! {
    serve_items (id) {
        id -> Int4,
        serve_id -> Int4,
        service_id -> Int4,
        store_id -> Int4,
        work_id -> Int4,
        orders_id -> Nullable<Int4>,
    }
}

table! {
    service_categories (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        position -> Int2,
        image -> Nullable<Varchar>,
        count -> Int2,
        view -> Int4,
        height -> Float8,
        seconds -> Int4,
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
        src -> Varchar,
    }
}

table! {
    service_videos (id) {
        id -> Int4,
        service -> Int4,
        src -> Varchar,
    }
}

table! {
    services (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Varchar>,
        content -> Nullable<Varchar>,
        link -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        is_active -> Bool,
        price -> Int4,
        user_id -> Int4,
        created -> Timestamp,
        position -> Int2,
        view -> Int4,
        height -> Float8,
        seconds -> Int4,
        price_acc -> Nullable<Int4>,
    }
}

table! {
    stat_blog_categories (id) {
        id -> Int4,
        view -> Int4,
        height -> Float8,
        seconds -> Int4,
    }
}

table! {
    stat_helps (id) {
        id -> Int4,
        view -> Int4,
        height -> Float8,
        seconds -> Int4,
    }
}

table! {
    stat_infos (id) {
        id -> Int4,
        view -> Int4,
        height -> Float8,
        seconds -> Int4,
    }
}

table! {
    stat_mainpages (id) {
        id -> Int4,
        view -> Int4,
        height -> Float8,
        seconds -> Int4,
    }
}

table! {
    stat_service_categories (id) {
        id -> Int4,
        view -> Int4,
        height -> Float8,
        seconds -> Int4,
    }
}

table! {
    stat_store_categories (id) {
        id -> Int4,
        view -> Int4,
        height -> Float8,
        seconds -> Int4,
    }
}

table! {
    stat_tags (id) {
        id -> Int4,
        view -> Int4,
        height -> Float8,
        seconds -> Int4,
    }
}

table! {
    stat_wiki_categories (id) {
        id -> Int4,
        view -> Int4,
        height -> Float8,
        seconds -> Int4,
    }
}

table! {
    stat_work_categories (id) {
        id -> Int4,
        view -> Int4,
        height -> Float8,
        seconds -> Int4,
    }
}

table! {
    store_categories (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        position -> Int2,
        image -> Nullable<Varchar>,
        count -> Int2,
        view -> Int4,
        height -> Float8,
        seconds -> Int4,
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
        src -> Varchar,
    }
}

table! {
    store_videos (id) {
        id -> Int4,
        store -> Int4,
        src -> Varchar,
    }
}

table! {
    stores (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Varchar>,
        content -> Nullable<Varchar>,
        link -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        is_active -> Bool,
        price -> Int4,
        user_id -> Int4,
        created -> Timestamp,
        position -> Int2,
        view -> Int4,
        height -> Float8,
        seconds -> Int4,
        price_acc -> Nullable<Int4>,
    }
}

table! {
    tags (id) {
        id -> Int4,
        name -> Varchar,
        position -> Int2,
        count -> Int2,
        blog_count -> Int2,
        service_count -> Int2,
        store_count -> Int2,
        wiki_count -> Int2,
        work_count -> Int2,
        user_id -> Int4,
        view -> Int4,
        height -> Float8,
        seconds -> Int4,
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
        created -> Timestamp,
    }
}

table! {
    tech_categories (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        position -> Int2,
        count -> Int2,
        level -> Int2,
        user_id -> Int4,
        view -> Int4,
        height -> Float8,
        seconds -> Int4,
    }
}

table! {
    tech_categories_items (id) {
        id -> Int4,
        category_id -> Int4,
        service_id -> Int4,
        store_id -> Int4,
        work_id -> Int4,
        types -> Int2,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        bio -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        perm -> Int2,
    }
}

table! {
    wiki_categories (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        position -> Int2,
        image -> Nullable<Varchar>,
        count -> Int2,
        view -> Int4,
        height -> Float8,
        seconds -> Int4,
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
        src -> Varchar,
    }
}

table! {
    wiki_videos (id) {
        id -> Int4,
        wiki -> Int4,
        src -> Varchar,
    }
}

table! {
    wikis (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Varchar>,
        content -> Nullable<Varchar>,
        link -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        is_active -> Bool,
        user_id -> Int4,
        created -> Timestamp,
        view -> Int4,
        height -> Float8,
        seconds -> Int4,
    }
}

table! {
    work_categories (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        position -> Int2,
        image -> Nullable<Varchar>,
        count -> Int2,
        view -> Int4,
        height -> Float8,
        seconds -> Int4,
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
        src -> Varchar,
    }
}

table! {
    work_videos (id) {
        id -> Int4,
        work -> Int4,
        src -> Varchar,
    }
}

table! {
    works (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Varchar>,
        content -> Nullable<Varchar>,
        link -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        is_active -> Bool,
        price -> Int4,
        user_id -> Int4,
        created -> Timestamp,
        position -> Int2,
        view -> Int4,
        height -> Float8,
        seconds -> Int4,
        price_acc -> Nullable<Int4>,
    }
}

joinable!(blog_category -> blog_categories (blog_categories_id));
joinable!(blog_category -> blogs (blog_id));
joinable!(blog_comments -> blogs (blog_id));
joinable!(blog_comments -> users (user_id));
joinable!(blog_images -> blogs (blog));
joinable!(blog_videos -> blogs (blog));
joinable!(blogs -> users (user_id));
joinable!(cookie_stats -> cookie_users (user_id));
joinable!(order_files -> orders (order_id));
joinable!(serve -> serve_categories (serve_categories));
joinable!(serve -> users (user_id));
joinable!(serve_categories -> tech_categories (tech_categories));
joinable!(service_category -> service_categories (service_categories_id));
joinable!(service_category -> services (service_id));
joinable!(service_images -> services (service));
joinable!(service_videos -> services (service));
joinable!(services -> users (user_id));
joinable!(store_category -> store_categories (store_categories_id));
joinable!(store_category -> stores (store_id));
joinable!(store_images -> stores (store));
joinable!(store_videos -> stores (store));
joinable!(stores -> users (user_id));
joinable!(tags -> users (user_id));
joinable!(wiki_category -> wiki_categories (wiki_categories_id));
joinable!(wiki_category -> wikis (wiki_id));
joinable!(wiki_images -> wikis (wiki));
joinable!(wiki_videos -> wikis (wiki));
joinable!(wikis -> users (user_id));
joinable!(work_category -> work_categories (work_categories_id));
joinable!(work_category -> works (work_id));
joinable!(work_images -> works (work));
joinable!(work_videos -> works (work));
joinable!(works -> users (user_id));

allow_tables_to_appear_in_same_query!(
    blog_categories,
    blog_category,
    blog_comments,
    blog_images,
    blog_videos,
    blogs,
    cookie_stats,
    cookie_users,
    feedbacks,
    help_item_categories,
    help_items,
    order_files,
    orders,
    serve,
    serve_categories,
    serve_items,
    service_categories,
    service_category,
    service_images,
    service_videos,
    services,
    stat_blog_categories,
    stat_helps,
    stat_infos,
    stat_mainpages,
    stat_service_categories,
    stat_store_categories,
    stat_tags,
    stat_wiki_categories,
    stat_work_categories,
    store_categories,
    store_category,
    store_images,
    store_videos,
    stores,
    tags,
    tags_items,
    tech_categories,
    tech_categories_items,
    users,
    wiki_categories,
    wiki_category,
    wiki_images,
    wiki_videos,
    wikis,
    work_categories,
    work_category,
    work_images,
    work_videos,
    works,
);
