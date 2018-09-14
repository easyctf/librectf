schema! {
    pub schema Schema;

    #[model(table_name = "teams")]
    pub struct Team {
        #[column(primary_key)]
        pub id: i32,
    }

    #[model(table_name = "users")]
    pub struct User {
        #[column(primary_key)]
        pub id: i32,

        #[column(foreign_key = "Team::id")]
        pub tid: i32,

        #[column]
        pub email: String,
    }
}
