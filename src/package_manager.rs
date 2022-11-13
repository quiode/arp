use std::{
    error::Error,
    fmt::Display,
    fs::{self, ReadDir},
    io::{self, Read},
    process::Command,
};

use serde::{Deserialize, Serialize};

const DATA_PATH: &str = "data.json";

pub struct Repository {
    path: String,
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

        Command::new(format!("git init {}", repo_path))
            .output()
            .or(Err(RepositoryError::GitError))?;

        let data = RepositoryData {
            email: None,
            name: None,
        };

        Ok(Self {
            path: repo_path.to_string(),
            data,
        })
    }

    // opens an existing repository
    pub fn open(repo_path: &str) -> RResult<Self> {
        let data = Self::get_data(repo_path)?;

        Ok(Self {
            path: repo_path.to_string(),
            data,
        })
    }

    // uploads the repository to the aur
    pub fn upload(&self) -> RResult<()> {
        self.run_command("git fetch")?;
        self.run_command("git add .")?;

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

    fn run_command(&self, command: &str) -> RResult<()> {
        Command::new(command)
            .current_dir(&self.path)
            .output()
            .or(Err(RepositoryError::GitFetchError))?;
        Ok(())
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
