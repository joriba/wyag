use std::{io, path::Path};
use crate::repository::Repository;

pub fn add() -> Result<(), io::Error> { todo!() }

pub fn cat_file() -> Result<(), io::Error> { todo!() }

pub fn check_ignore() -> Result<(), io::Error> { todo!() }

pub fn checkout() -> Result<(), io::Error> { todo!() }

pub fn commit() -> Result<(), io::Error> { todo!() }

pub fn hash_object() -> Result<(), io::Error> { todo!() }

pub fn init() -> Result<(), io::Error> { 
    let path = Path::new("");
    match Repository::new(path.to_path_buf()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

pub fn log() -> Result<(), io::Error> { todo!() }

pub fn ls_files() -> Result<(), io::Error> { todo!() }

pub fn ls_tree() -> Result<(), io::Error> { todo!() }

pub fn rev_parse() -> Result<(), io::Error> { todo!() }

pub fn rm() -> Result<(), io::Error> { todo!() }

pub fn show_ref() -> Result<(), io::Error> { todo!() }

pub fn status() -> Result<(), io::Error> { todo!() }

pub fn tag() -> Result<(), io::Error> { todo!() }