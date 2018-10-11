use db::Pool;

pub struct State {
    pub secret_key: Vec<u8>,
    pub pool: Pool,
}
