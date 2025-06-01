// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "WebsiteStatus"))]
    pub struct WebsiteStatus;
}

diesel::table! {
    Region (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::table! {
    Website (id) {
        id -> Text,
        url -> Text,
        #[sql_name="timeAdded"]
        time_added -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::WebsiteStatus;

    WebsiteTick (id) {
        id -> Text,
        response_time_ms -> Int4,
        status -> WebsiteStatus,
        region_id -> Text,
        website_id -> Text,
    }
}

diesel::joinable!(WebsiteTick -> Region (region_id));
diesel::joinable!(WebsiteTick -> Website (website_id));

diesel::allow_tables_to_appear_in_same_query!(
    Region,
    Website,
    WebsiteTick,
);
