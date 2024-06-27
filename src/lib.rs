use std::{collections::HashMap, io, io::{Error, ErrorKind}, path::{Path, PathBuf}};


pub mod command {
    use std::io;

    pub fn add() -> Result<(), io::Error> { todo!() }

    pub fn cat_file() -> Result<(), io::Error> { todo!() }

    pub fn check_ignore() -> Result<(), io::Error> { todo!() }

    pub fn checkout() -> Result<(), io::Error> { todo!() }

    pub fn commit() -> Result<(), io::Error> { todo!() }

    pub fn hash_object() -> Result<(), io::Error> { todo!() }

    pub fn init() -> Result<(), io::Error> { todo!() }

    pub fn log() -> Result<(), io::Error> { todo!() }

    pub fn ls_files() -> Result<(), io::Error> { todo!() }

    pub fn ls_tree() -> Result<(), io::Error> { todo!() }

    pub fn rev_parse() -> Result<(), io::Error> { todo!() }

    pub fn rm() -> Result<(), io::Error> { todo!() }

    pub fn show_ref() -> Result<(), io::Error> { todo!() }

    pub fn status() -> Result<(), io::Error> { todo!() }

    pub fn tag() -> Result<(), io::Error> { todo!() }
}

pub struct Repository {
    worktree: PathBuf,
    gitdir: PathBuf,
    conf: HashMap<String, String>,
}

impl Repository {
    pub fn new(path: PathBuf, create: bool) -> Result<Self, io::Error> {
        let gitdir = path.join(".git");
        if !gitdir.exists() || create {
            return Err(Error::new(ErrorKind::NotFound, "Not a valid Git repo"));
        }

        let result = Self {
            worktree: path,
            gitdir: gitdir,
            conf: HashMap::new()
        };
        
        Ok(result)
    }
}