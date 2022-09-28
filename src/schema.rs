table! {
    categories (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        position -> Int2,
        image -> Nullable<Varchar>,
        count -> Int2,
        view -> Int4,
        height -> Float8,
        seconds -> Int4,
        types -> Int2,
        slug -> Varchar,
    }
}

table! {
    category (id) {
        id -> Int4,
        categories_id -> Int4,
        item_id -> Int4,
        types -> Int2,
    }
}

table! {
    chats (id) {
        id -> Int4,
        user_id -> Int4,
        created -> Timestamp,
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
    files (id) {
        id -> Int4,
        user_id -> Int4,
        item_id -> Int4,
        item_types -> Int2,
        types -> Int2,
        src -> Varchar,
        description -> Nullable<Varchar>,
        position -> Int2,
        view -> Int4,
        seconds -> Int4,
    }
}

table! {
    item_comments (id) {
        id -> Int4,
        comment -> Varchar,
        item_id -> Int4,
        user_id -> Int4,
        parent_id -> Nullable<Int4>,
        created -> Timestamp,
    }
}

table! {
    items (id) {
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
        types -> Int2,
        slug -> Varchar,
    }
}

table! {
    messages (id) {
        id -> Int4,
        user_id -> Int4,
        chat_id -> Int4,
        created -> Timestamp,
        content -> Nullable<Varchar>,
        view -> Int2,
        types -> Int2,
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
        price -> Int4,
        price_acc -> Nullable<Int4>,
    }
}

table! {
    serve (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        position -> Int2,
        serve_categories -> Int4,
        price -> Int4,
        man_hours -> Int2,
        is_default -> Bool,
        user_id -> Int4,
        tech_cat_id -> Int4,
        height -> Float8,
        seconds -> Int4,
        serve_id -> Nullable<Int4>,
        view -> Int4,
    }
}

table! {
    serve_categories (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
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
        item_id -> Int4,
        types -> Int2,
    }
}

table! {
    stat_pages (id) {
        id -> Int4,
        types -> Int2,
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
        item_id -> Int4,
        types -> Int2,
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
        item_id -> Int4,
        types -> Int2,
        is_active -> Int2,
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

joinable!(category -> categories (categories_id));
joinable!(category -> items (item_id));
joinable!(chats -> users (user_id));
joinable!(cookie_stats -> cookie_users (user_id));
joinable!(item_comments -> items (item_id));
joinable!(item_comments -> users (user_id));
joinable!(items -> users (user_id));
joinable!(messages -> users (user_id));
joinable!(order_files -> orders (order_id));
joinable!(serve -> serve_categories (serve_categories));
joinable!(serve -> users (user_id));
joinable!(serve_categories -> tech_categories (tech_categories));
joinable!(tags -> users (user_id));

allow_tables_to_appear_in_same_query!(
    categories,
    category,
    chats,
    cookie_stats,
    cookie_users,
    feedbacks,
    files,
    item_comments,
    items,
    messages,
    order_files,
    orders,
    serve,
    serve_categories,
    serve_items,
    stat_pages,
    tags,
    tags_items,
    tech_categories,
    tech_categories_items,
    users,
);
