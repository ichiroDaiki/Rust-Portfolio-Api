diesel::table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        published -> Bool,
    }
}

diesel::table!{
    usuarios (nombre) {
        nombre -> Text,
    }
}