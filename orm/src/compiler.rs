use structured::*;

pub trait Compiler {
    fn visit_select(&self, Select) -> String {
        String::from("SELECT ")
    }
}

pub struct DefaultCompiler;

impl Compiler for DefaultCompiler {}
