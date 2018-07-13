use std::collections::{btree_map, BTreeMap};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use either::Either;
use failure::{err_msg, Error};
use walkdir::WalkDir;

pub type Binfile<T> = Either<T, PathBuf>;
pub struct Bindata<T>(pub BTreeMap<String, Binfile<T>>);

pub trait Storage: Clone + Sized {
    fn read_from(f: &mut File) -> Result<Self, Error>;
}

impl Storage for Vec<u8> {
    fn read_from(f: &mut File) -> Result<Self, Error> {
        let mut data = Vec::new();
        f.read_to_end(&mut data)?;
        Ok(data)
    }
}

impl Storage for String {
    fn read_from(f: &mut File) -> Result<Self, Error> {
        let mut data = String::new();
        f.read_to_string(&mut data)?;
        Ok(data)
    }
}

cfg_if! {
    if #[cfg(debug_assertions)] {
        fn load_entry<T>(path: &PathBuf) -> Result<Binfile<T>, Error> {
            Ok(Either::Right(path.clone()))
        }
    } else {
        fn load_entry<T>(path: &PathBuf) -> Result<Binfile<T>, Error>{
            let mut file = File::open(path)?;
            let mut data: Vec<u8> = Vec::new();
            match file.read_to_end(&mut data) {
                Ok(_) => Ok(Either::Left(data)),
                Err(err) => err,
            }
        }
    }
}

pub(crate) fn resolve<T: Storage>(v: &Binfile<T>) -> Result<T, Error> {
    match v {
        &Either::Left(ref data) => Ok(data.clone()),
        &Either::Right(ref path) => {
            let mut file = File::open(path)?;
            T::read_from(&mut file)
        }
    }
}

impl<T: Storage> Bindata<T> {
    pub fn new(folder: PathBuf) -> Result<Self, Error> {
        let mut map = BTreeMap::new();
        for entry in WalkDir::new(folder.clone())
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry
                .path()
                .to_path_buf();
            let entry = load_entry(&path)?;
            let name = path
                .as_os_str()
                .to_str()
                .ok_or(err_msg("could not load template"))?;
            map.insert(name.to_owned(), entry);
        }
        Ok(Bindata(map))
    }
    pub fn get(&self, name: &str) -> Option<T> {
        if let Some(ref split) = self.0.get(name) {
            resolve(split).ok()
        } else {
            None
        }
    }
    pub fn iter(&self) -> btree_map::Iter<'_, String, Binfile<T>> {
        self.0.iter()
    }
}
