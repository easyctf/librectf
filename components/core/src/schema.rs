table! {
    chals (id) {
        id -> Integer,
        title -> Varchar,
        enabled -> Bool,
        description -> Text,
        correct_flag -> Text,
        regex -> Bool,
        value -> Integer,
    }
}

table! {
    files (id) {
        id -> Integer,
        name -> Varchar,
        url -> Text,
        chal_id -> Integer,
        team_id -> Nullable<Integer>,
    }
}

table! {
    solves (id) {
        id -> Integer,
        timestamp -> Datetime,
        flag -> Text,
        chal_id -> Integer,
        team_id -> Integer,
        user_id -> Integer,
    }
}

table! {
    tasks (id) {
        id -> Integer,
        created -> Datetime,
        claimed -> Nullable<Datetime>,
        completed -> Nullable<Datetime>,
        name -> Varchar,
        payload -> Nullable<Text>,
    }
}

table! {
    teams (id) {
        id -> Integer,
        name -> Varchar,
        affiliation -> Nullable<Varchar>,
        banned -> Bool,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Varchar,
        email -> Varchar,
        email_verified -> Bool,
        password -> Varchar,
        admin -> Bool,
        team_id -> Nullable<Integer>,
    }
}

joinable!(files -> chals (chal_id));
joinable!(files -> teams (team_id));
joinable!(solves -> chals (chal_id));
joinable!(solves -> teams (team_id));
joinable!(solves -> users (user_id));
joinable!(users -> teams (team_id));

allow_tables_to_appear_in_same_query!(chals, files, solves, tasks, teams, users,);
