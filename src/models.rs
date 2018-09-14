schema! {
    schema Schema;

    #[derive(Model)]
    #[table_name = "teams"]
    pub struct Team {
        #[column(primary_key)]
        pub id: i32,
    }

    #[derive(Model)]
    #[table_name = "users"]
    pub struct User {
        #[column(primary_key)]
        pub id: i32,

        #[column(foreign_key = "Team::id")]
        pub tid: i32,

        #[column]
        pub email: String,
    }
}
