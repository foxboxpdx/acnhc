table! {
    arts (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    fossils (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    ownedarts (id) {
        id -> Int4,
        user_id -> Int4,
        item_id -> Int4,
        extra -> Int4,
    }
}

table! {
    ownedfossils (id) {
        id -> Int4,
        user_id -> Int4,
        item_id -> Int4,
        extra -> Int4,
    }
}

table! {
    ownedrecipes (id) {
        id -> Int4,
        user_id -> Int4,
        item_id -> Int4,
        extra -> Int4,
    }
}

table! {
    recipes (id) {
        id -> Int4,
        name -> Varchar,
        type_id -> Int4,
    }
}

table! {
    recipetypes (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        alias -> Varchar,
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
