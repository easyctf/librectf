use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use either::Either;
use failure::Error;
use walkdir::WalkDir;

pub type TemplateData = Either<Vec<u8>, PathBuf>;
pub struct Templates(BTreeMap<String, TemplateData>);

cfg_if! {
    if #[cfg(debug_assertions)] {
        fn load_entry(path: PathBuf) -> Result<TemplateData, Error> {
            Ok(Either::Right(path))
        }
    } else {
        fn load_entry(path: PathBuf) -> Result<TemplateData ,Error>{
            let mut file = match File::open(path) {
                Ok(mut file) => file,
                Err(err) => return err,
            };
            let mut data: Vec<u8> = Vec::new();
            match file.read_to_end(&mut data) {
                Ok(_) => Ok(Either::Left(data)),
                Err(err) => err,
            }
        }
    }
}

impl Templates {
    pub fn new(folder: PathBuf) -> Result<Self, Error> {
        let mut map = BTreeMap::new();
        for entry in WalkDir::new(folder.clone())
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path().to_path_buf();
            map.insert("a".to_owned(), load_entry(path)?);
        }
        Ok(Templates(map))
    }
    pub fn get(&self, name: &str) -> Option<Vec<u8>> {
        if let Some(ref split) = self.0.get(name) {
            match split {
                &Either::Left(ref data) => Some(data.clone()),
                &Either::Right(ref path) => {
                    let mut file = match File::open(path) {
                        Ok(mut file) => file,
                        Err(_) => return None,
                    };
                    let mut data: Vec<u8> = Vec::new();
                    match file.read_to_end(&mut data) {
                        Ok(_) => Some(data),
                        Err(_) => return None,
                    }
                }
            }
        } else {
            None
        }
    }
}
