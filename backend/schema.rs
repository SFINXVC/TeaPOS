// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_role"))]
    pub struct UserRole;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserRole;

    users (id) {
        id -> BigSerial,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        fullname -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 15]
        whatsapp -> Varchar,
        role -> UserRole,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
