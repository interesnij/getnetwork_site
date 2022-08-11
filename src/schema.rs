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
        types -> Nullable<Varchar>,
        view -> Int4,
        height -> Float8,
        seconds -> Int4,
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
    }
}
