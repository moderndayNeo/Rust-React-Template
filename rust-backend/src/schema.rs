// @generated automatically by Diesel CLI.

diesel::table! {
    ai_tools (id) {
        id -> Integer,
        name -> Text,
        company -> Text,
        description -> Text,
        image_url -> Nullable<Text>,
    }
}

diesel::table! {
    martial_arts (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        image_url -> Nullable<Text>,
        country_of_origin -> Text,
        monthly_price_usd -> Float,
    }
}

diesel::table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        published -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    ai_tools,
    martial_arts,
    posts,
);
