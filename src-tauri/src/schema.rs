// @generated automatically by Diesel CLI.

diesel::table! {
    expenses (id) {
        id -> Integer,
        name -> Text,
        amount -> Float,
        flexible -> Bool,
    }
}
