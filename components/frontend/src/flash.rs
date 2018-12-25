use actix_web::{middleware::session::RequestSession, HttpRequest};
use failure::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Flash {
    pub message: String,
    pub category: String,
}

impl<A: AsRef<str>, B: AsRef<str>> From<(A, B)> for Flash {
    fn from((a, b): (A, B)) -> Self {
        Flash {
            message: a.as_ref().to_owned(),
            category: b.as_ref().to_owned(),
        }
    }
}

pub trait RequestFlash {
    fn flash(&self, flash: impl Into<Flash>) -> Result<(), Error>;
    fn flashes(&self) -> Result<Vec<Flash>, Error>;
}

impl<S> RequestFlash for HttpRequest<S> {
    fn flash(&self, flash: impl Into<Flash>) -> Result<(), Error> {
        let mut flashes = self.flashes()?;
        flashes.push(flash.into());
        self.session().set("flashes", flashes).map_err(|err| format_err!("{}", err))?;
        Ok(())
    }

    fn flashes(&self) -> Result<Vec<Flash>, Error> {
        match self
            .session()
            .get::<Vec<Flash>>("flashes")
            .map_err(|err| format_err!("Failed to retrieve flashes"))?
        {
            Some(flashes) => Ok(flashes),
            None => Ok(Vec::new()),
        }
    }
}
