table! {
    fossils (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    ownedfossils (id) {
        id -> Integer,
        user_id -> Integer,
        fossil_id -> Integer,
        owned -> Bool,
        extra -> Integer,
    }
}

table! {
    ownedrecipes (id) {
        id -> Integer,
        user_id -> Integer,
        recipe_id -> Integer,
        owned -> Bool,
        extra -> Integer,
    }
}

table! {
    recipes (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Text,
        alias -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    fossils,
    ownedfossils,
    ownedrecipes,
    recipes,
    users,
);
