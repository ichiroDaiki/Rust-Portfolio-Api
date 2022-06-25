table! {
    projects (id) {
        id -> Int4,
        title -> Text,
        body -> Text,
        gallery_name -> Text,
        name_tech -> Array<Text>,
    }
}

table! {
    skills (id) {
        id -> Int4,
        title -> Text,
        body -> Text,
        name_tech -> Text,
        expe -> Text,
    }
}

/* allow_tables_to_appear_in_same_query!(
    projects,
    skills,
);
 */