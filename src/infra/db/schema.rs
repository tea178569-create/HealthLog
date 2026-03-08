// @generated automatically by Diesel CLI.

diesel::table! {
    locations (id) {
        id -> Nullable<Integer>,
        timestamp -> Text,
        prefecture -> Text,
        venue_name -> Text,
        donation_type -> Text,
    }
}

diesel::table! {
    measurements (id) {
        id -> Nullable<Integer>,
        metric_type -> Text,
        timestamp -> Text,
        value -> Float,
        source -> Text,
        is_outlier -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        birth_date -> Nullable<Text>,
        gender -> Nullable<Text>,
        height_cm -> Nullable<Float>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(locations, measurements, users,);
