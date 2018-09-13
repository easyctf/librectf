table! {
    challenges (id) {
        id -> Integer,
        title -> Varchar,
        description -> Text,
        hint -> Nullable<Text>,
        value -> Integer,
    }
}

table! {
    config (id) {
        id -> Integer,
    }
}

table! {
    teams (id) {
        id -> Integer,
        teamname -> Varchar,
        affiliation -> Nullable<Varchar>,
        banned -> Bool,
    }
}

table! {
    users (id) {
        id -> Integer,
        tid -> Nullable<Integer>,
        admin -> Bool,
        email -> Varchar,
        password -> Varchar,
        date_created -> Datetime,
    }
}

joinable!(users -> teams (tid));

allow_tables_to_appear_in_same_query!(
    challenges,
    config,
    teams,
    users,
);
