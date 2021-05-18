table! {
    addresses (id) {
        id -> Int4,
        street -> Varchar,
        addr_number -> Varchar,
        city -> Varchar,
    }
}

table! {
    clients (id) {
        id -> Int4,
        name -> Varchar,
        address_id -> Int4,
    }
}

joinable!(clients -> addresses (address_id));

allow_tables_to_appear_in_same_query!(
    addresses,
    clients,
);
