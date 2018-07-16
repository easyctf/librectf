use wtforms::Field;

#[derive(Form)]
struct LoginForm {
    #[field(name = "username")]
    username: Field<String>,
}
