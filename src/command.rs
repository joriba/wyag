use crate::{repository::Repository, wyag_error::Error};
use std::path::Path;

pub fn add() -> Result<(), Error> {
    todo!()
}

pub fn cat_file() -> Result<(), Error> {
    todo!()
}

pub fn check_ignore() -> Result<(), Error> {
    todo!()
}

pub fn checkout() -> Result<(), Error> {
    todo!()
}

pub fn commit() -> Result<(), Error> {
    todo!()
}

pub fn hash_object() -> Result<(), Error> {
    todo!()
}

pub fn init() -> Result<(), Error> {
    let path = Path::new("");
    match Repository::new(path.to_path_buf()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn log() -> Result<(), Error> {
    todo!()
}

pub fn ls_files() -> Result<(), Error> {
    todo!()
}

pub fn ls_tree() -> Result<(), Error> {
    todo!()
}

pub fn rev_parse() -> Result<(), Error> {
    todo!()
}

pub fn rm() -> Result<(), Error> {
    todo!()
}

pub fn show_ref() -> Result<(), Error> {
    todo!()
}

pub fn status() -> Result<(), Error> {
    todo!()
}

pub fn tag() -> Result<(), Error> {
    todo!()
}
