use actix_web::App;
use failure::Error;
use tera::Tera;

use Config;

pub struct OpenCTF {
    pub config: Config,
}

pub struct AppState {
    templates: Tera,
}

impl OpenCTF {
    pub fn new(config: Config) -> Result<Self, Error> {
        Ok(OpenCTF { config })
    }
    pub fn app(&self) -> Result<App<AppState>, Error> {
        let tera = compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"));
        // load all templates

        let app = App::with_state(AppState { templates: tera });
        Ok(app)
    }
    pub fn bind_address(&self) -> (&str, u16) {
        (self.config.host.as_ref(), self.config.port)
    }
}
