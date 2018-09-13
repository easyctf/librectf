use std::borrow::Cow;

use rocket::{
    http::{ContentType, Status},
    response::{self, Content},
    Request,
};
use serde::Serialize;
use tera::Tera;

lazy_static! {
    static ref RENDERER: Tera = {
        // TODO: clean this up
        // maybe actually use a for loop instead of pure combinators
        // also check the unwraps maybe, although it should be fine as long as list() works
        let mut tera = Tera::default();
        let tmpls = Template::list()
            .map(|file| {
                (
                    String::from(file),
                    String::from_utf8(Template::get(file).unwrap()).unwrap(),
                )
            }).collect::<Vec<_>>();
        tera.add_raw_templates(
            tmpls
                .iter()
                .map(|(a, b)| (a.as_ref(), b.as_ref()))
                .collect::<Vec<_>>(),
        ).unwrap();
        tera
    };
}

#[derive(Embed)]
#[folder = "templates"]
pub struct Template {
    contents: Option<String>,
}

impl Template {
    pub fn render<S, C>(name: S, context: C) -> Self
    where
        S: Into<Cow<'static, str>>,
        C: Serialize,
    {
        // TODO: log the error sometime
        let name = name.into();
        let contents = RENDERER.render(&name, &context).ok();
        Template { contents }
    }
    fn finalize(self) -> Result<(String, ContentType), Status> {
        match self.contents {
            Some(contents) => Ok((contents, ContentType::HTML)),
            None => Err(Status::InternalServerError),
        }
    }
}

impl response::Responder<'static> for Template {
    fn respond_to(self, req: &Request) -> response::Result<'static> {
        let (render, content_type) = self.finalize()?;
        Content(content_type, render).respond_to(req)
    }
}
