use failure::Error;

#[derive(Debug, StructOpt)]
pub(crate) struct Web {
    #[structopt(short = "p", long = "port", default_value = "4401", help = "The port the application should run on (overrides existing configuration)")]
    port: u32,
}

impl Web {
    pub fn run(self) -> Result<(), Error> {
        Ok(())
    }
}
