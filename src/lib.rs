use std::{collections::HashMap, fs, io::{self, Error, ErrorKind}, path::PathBuf};


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

pub fn invalid_argument(message: &'static str) -> io::Error {
    Error::new(ErrorKind::InvalidInput, message)
}

pub struct Repository {
    worktree: PathBuf,
    gitdir: PathBuf,
    conf: HashMap<String, String>,
}

impl Repository {
    pub fn open(path: PathBuf) -> Result<Self, io::Error> {
        let gitdir = path.join(".git");
        if !gitdir.exists() {
            return Err(invalid_argument("Not a Git repository"));
        }

        let result = Self {
            worktree: path,
            gitdir: gitdir,
            conf: HashMap::new()
        };
        
        Ok(result)
    }

    pub fn new(path: PathBuf) -> Result<Self, io::Error> {
        todo!()
    }

    /// Builds a path object from the repo's git directory and the supplied
    /// strings. Might return an invalid path! If the path should be created,
    /// use [`Self::repo_path_create()`] instead
    /// 
    /// # Examples
    /// ```
    /// use wyag::Repository;
    /// use std::path::PathBuf;
    /// 
    /// let repo = Repository::new(PathBuf::from("."), false).unwrap();
    /// let path = repo.repo_path(vec!("refs", "remotes", "origin"));
    /// assert_eq!(path, PathBuf::from("./.git/refs/remotes/origin"));
    /// ```
    pub fn repo_path(&self, paths: Vec<&str>) -> PathBuf {
        paths.iter().fold(
            self.gitdir.clone(), 
            |acc, x| acc.join(x)
        )
    }

    /// Builds a path object from the repo's git directory and the supplied
    /// strings. Creates the path if ti does not exist, so this always 
    /// returns either a (now) valid directory or an error.
    pub fn repo_path_create(&self, paths: Vec<&str>) -> Result<PathBuf, io::Error> {
        let path = self.repo_path(paths);
        if path.exists() {
            if path.is_dir() { return Ok(path); }
            else { return Err(invalid_argument("Not a directory!"))}
        } 
        
        fs::create_dir_all(path.as_os_str())?;
        Ok(path)
    }
}