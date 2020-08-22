table! {
    items (id) {
        id -> Integer,
        name_en -> Text,
        name_jp -> Text,
        pri_type -> Integer,
        sub_type -> Integer,
    }
}

table! {
    owneditems (id) {
        id -> Integer,
        user_id -> Integer,
        item_id -> Integer,
        extra -> Integer,
    }
}

table! {
    primarytypes (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    subtypes (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Text,
        passwd -> Text,
        seen -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    items,
    owneditems,
    primarytypes,
    subtypes,
    users,
);
