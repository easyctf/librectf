#[derive(Debug, StructOpt)]
pub struct WebCommand {
    /// Run the API server.
    #[structopt(long = "api")]
    api: bool,
}
