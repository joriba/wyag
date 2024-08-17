use defaults::GIT_SUBDIR;

use crate::wyag_error::Error;
use crate::InvalidArgument;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub struct Repository {
    pub worktree: PathBuf,
    pub gitdir: PathBuf,
    conf: HashMap<String, String>,
}

pub mod defaults {
    pub const INIT_DIRECTORY: &'static str = ".";
    pub const GIT_SUBDIR: &'static str = ".wyag";
}

impl Repository {
    pub fn open(path: PathBuf) -> Result<Self, Error> {
        let gitdir = path.join(GIT_SUBDIR);
        if !gitdir.exists() {
            return Err(InvalidArgument("Not a Git repository"));
        }

        let result = Self {
            worktree: path,
            gitdir: gitdir,
            conf: HashMap::new(), // todo: read config file here
        };

        Ok(result)
    }

    fn populate_new_repo(repo: &Repository) -> Result<(), Error> {
        fs::create_dir_all(&repo.worktree)?;

        let subdirs = vec![
            vec!["branches"],
            vec!["objects"],
            vec!["refs", "tags"],
            vec!["refs", "heads"],
        ];

        let errors: Result<Vec<_>, _> = subdirs
            .iter()
            .map(|x| repo.repo_path_create(x.to_vec()))
            .collect();
        errors?;

        let description =
            "Unnamed repository; edit this file 'description' to name the repository.\n";
        fs::write(repo.gitdir.join("description"), description)?;
        fs::write(repo.gitdir.join("HEAD"), "ref: refs/head/master")?;

        Ok(())
    }

    pub fn new(path: PathBuf) -> Result<Self, Error> {
        let gitdir = path.join(GIT_SUBDIR);
        let result = Self {
            worktree: path,
            gitdir: gitdir.clone(),
            conf: HashMap::new(), // todo: get the HashMap from the config file
        };
        if result.worktree.exists() {
            if !result.worktree.is_dir() {
                return Err(InvalidArgument("Not a Directory"));
            }
            if result.gitdir.exists() && fs::read_dir(&result.gitdir)?.next().is_some() {
                return Err(InvalidArgument("Directory is not empty"));
            }
        }

        match Self::populate_new_repo(&result) {
            Ok(()) => (),
            Err(error) => {
                // todo: rollback changes made to the filesystem
                return Err(error);
            }
        }

        Ok(result)
    }

    /// Builds a path object from the repo's git directory and the supplied
    /// strings. Might return an invalid path! If the path should be created,
    /// use [`Self::repo_path_create()`] instead
    ///
    /// # Examples
    /// ```
    /// use wyag::repository::Repository;
    /// use wyag::repository::defaults;
    /// use std::path::PathBuf;
    ///
    /// let repo = Repository::new(PathBuf::from("test_repo")).unwrap();
    /// 
    /// let expected = format!("test_repo/{}/refs/remotes/origin", defaults::GIT_SUBDIR);
    /// let actual = repo.repo_path(vec!("refs", "remotes", "origin"));
    /// assert_eq!(PathBuf::from(expected), actual);
    /// std::fs::remove_dir_all(repo.gitdir);
    /// ```
    pub fn repo_path(&self, paths: Vec<&str>) -> PathBuf {
        paths.iter().fold(self.gitdir.clone(), |acc, x| acc.join(x))
    }

    /// Builds a path object from the repo's git directory and the supplied
    /// strings. Creates the path if ti does not exist, so this always
    /// returns either a (now) valid directory or an error.
    pub fn repo_path_create(&self, paths: Vec<&str>) -> Result<PathBuf, Error> {
        let path = self.repo_path(paths);
        if path.exists() {
            if path.is_dir() {
                return Ok(path);
            } else {
                return Err(InvalidArgument("Not a directory!"));
            }
        }

        fs::create_dir_all(path.as_os_str())?;
        Ok(path)
    }
}
