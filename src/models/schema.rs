// @generated automatically by Diesel CLI.

diesel::table! {
    brand (id) {
        id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        is_active -> Bool,
    }
}

diesel::table! {
    brand_model (id) {
        id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        is_active -> Bool,
        brand_id -> Uuid,
    }
}

diesel::joinable!(brand_model -> brand (brand_id));

diesel::allow_tables_to_appear_in_same_query!(
    brand,
    brand_model,
);
