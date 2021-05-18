table! {
    addresses (id) {
        id -> Int4,
        street -> Varchar,
        addr_number -> Varchar,
        city -> Varchar,
        client_id -> Int4,
    }
}

table! {
    clients (id) {
        id -> Int4,
        name -> Varchar,
    }
}

joinable!(addresses -> clients (client_id));

allow_tables_to_appear_in_same_query!(
    addresses,
    clients,
);
