table! {
    arts (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    fossils (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    ownedarts (id) {
        id -> Integer,
        user_id -> Integer,
        item_id -> Integer,
        extra -> Integer,
    }
}

table! {
    ownedfossils (id) {
        id -> Integer,
        user_id -> Integer,
        item_id -> Integer,
        extra -> Integer,
    }
}

table! {
    ownedrecipes (id) {
        id -> Integer,
        user_id -> Integer,
        item_id -> Integer,
        extra -> Integer,
    }
}

table! {
    recipes (id) {
        id -> Integer,
        name -> Text,
        type_id -> Integer,
    }
}

table! {
    recipetypes (id) {
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
    arts,
    fossils,
    ownedarts,
    ownedfossils,
    ownedrecipes,
    recipes,
    recipetypes,
    users,
);
