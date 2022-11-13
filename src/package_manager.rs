use std::{
    error::Error,
    fmt::Display,
    fs::{self, ReadDir},
    io::{self, Read},
};

use git::{IndexAddOption, IntoCString};
use serde::{Deserialize, Serialize};

const DATA_PATH: &str = "data.json";

pub struct Repository {
    git_repo: git::Repository,
    data: RepositoryData,
}

impl Repository {
    // creates a new repository
    pub fn new(repo_path: &str) -> RResult<Self> {
        match Self::is_empty(repo_path) {
            Err(_) => return Err(RepositoryError::NotARepository),
            Ok(false) => return Err(RepositoryError::FolderNotEmpty),
            Ok(true) => (),
        }

        let repo = git::Repository::init(repo_path).or(Err(RepositoryError::GitError))?;

        repo.config()
            .or(Err(RepositoryError::GitError))?
            .set_str("user.name", "arp")
            .or(Err(RepositoryError::GitError))?;
        repo.config()
            .or(Err(RepositoryError::GitError))?
            .set_str("user.email", "arp@github.com")
            .or(Err(RepositoryError::GitError))?;

        let data = RepositoryData {
            email: None,
            name: None,
        };

        Ok(Self {
            git_repo: repo,
            data,
        })
    }

    // opens an existing repository
    pub fn open(repo_path: &str) -> RResult<Self> {
        let repo = match git::Repository::open(repo_path) {
            Ok(repo) => repo,
            Err(_) => return Err(RepositoryError::NotARepository),
        };

        let data = Self::get_data(repo_path)?;

        Ok(Self {
            data,
            git_repo: repo,
        })
    }

    // uploads the repository to the aur
    pub fn upload(&self) -> RResult<()> {
        self.fetch()?;

        let mut index = self.git_repo.index().or(Err(RepositoryError::GitError))?;

        index
            .update_all(["*"], None)
            .or(Err(RepositoryError::GitError))?;

        index
            .add_all(["*"], IndexAddOption::DEFAULT, None)
            .or(Err(RepositoryError::GitError))?;

        self.git_repo.commit(
            None,
            &self.git_repo.signature().unwrap(),
            &self.git_repo.signature().unwrap(),
            &format!("arp commit_ {:#?}", std::time::SystemTime::now()),
            tree,
            parents,
        );

        todo!()
    }

    // returns the repository data
    fn get_data(repo_path: &str) -> RResult<RepositoryData> {
        let mut file = fs::File::open(repo_path).or(Err(RepositoryError::NotARepository))?;

        let mut string_data = String::new();
        file.read_to_string(&mut string_data)
            .or(Err(RepositoryError::NotARepository))?;

        Ok(serde_json::from_str(&string_data).or(Err(RepositoryError::NotARepository))?)
    }

    // returns tru if folder is empty
    fn is_empty(path: &str) -> io::Result<bool> {
        Ok(fs::read_dir(path)?.next().is_none())
    }

    fn fetch(&self) -> RResult<()> {
        self.git_repo
            .find_remote("origin")
            .or(Err(RepositoryError::NoRemote))?
            .fetch(&["master"], None, None)
            .or(Err(RepositoryError::GitFetchError))
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct RepositoryData {
    email: Option<String>,
    name: Option<String>,
}

pub type RResult<T> = Result<T, RepositoryError>;

#[derive(Debug)]
pub enum RepositoryError {
    NotARepository,
    FolderNotEmpty,
    GitFetchError,
    GitError,
    NoRemote,
}

impl Error for RepositoryError {}

impl Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            RepositoryError::NotARepository => "Not a Repository",
            RepositoryError::FolderNotEmpty => "Folder not empty",
            RepositoryError::GitError => "Git Error",
        };

        write!(f, "{}", text)
    }
}
