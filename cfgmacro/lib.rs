#[allow(unused_imports)]
#[macro_use]
extern crate cfgmacro_derive;
extern crate clap as _clap;
extern crate toml as _toml;

// Re-export toml
pub mod toml {
    pub use _toml::*;
}

// Re-export clap
pub mod clap {
    pub use _clap::*;
}

pub use cfgmacro_derive::*;
