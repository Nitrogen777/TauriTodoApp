// @generated automatically by Diesel CLI.

diesel::table! {
    todo (id) {
        id -> Int4,
        text -> Varchar,
        date -> Date,
    }
}
