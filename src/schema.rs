// @generated automatically by Diesel CLI.

diesel::table! {
    msg (id) {
        id -> Int4,
        sender -> Varchar,
        recipient -> Varchar,
        body -> Text,
        sent_at -> Timestamp,
    }
}

diesel::table! {
    thread (id) {
        id -> Int4,
        peer1 -> Varchar,
        peer2 -> Varchar,
        messages -> Nullable<Array<Nullable<Int4>>>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    msg,
    thread,
);
