use std::{
    error::Error,
    fmt::Display,
    fs::{self, ReadDir},
    io::{self, Read, Write},
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

        let data = RepositoryData::default();

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
        self.run_command("git fetch aur")?;
        self.run_command("git add .")?;
        self.run_command("git commit -m \"commit through arp\"")?;
        self.run_command("git push aur")
    }

    pub fn get_data(&self) -> RepositoryData {
        self.data.clone()
    }

    // returns the repository data from a json file
    fn read_data(repo_path: &str) -> RResult<RepositoryData> {
        let mut file = fs::File::open(&format!("{}/{}", repo_path, DATA_PATH))
            .or(Err(RepositoryError::NotARepository))?;

        let mut string_data = String::new();
        file.read_to_string(&mut string_data)
            .or(Err(RepositoryError::NotARepository))?;

        Ok(serde_json::from_str(&string_data).or(Err(RepositoryError::NotARepository))?)
    }

    // saves repo data to a json file
    fn save_data(&self, repo_path: &str) -> RResult<()> {
        let json = serde_json::to_string(&self.data).or(Err(RepositoryError::SerializeError))?;

        // save file
        let mut file = fs::File::create(&format!("{}/{}", repo_path, DATA_PATH))
            .or(Err(RepositoryError::SerializeError))?;

        file.write_all(json.as_bytes())
            .or(Err(RepositoryError::SerializeError))?;
        Ok(())
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

    // adds the aur as a remote and performs a fetch to register the package on the aur
    fn register_package(&self) -> RResult<()> {
        let name = match self.data.package_name {
            Some(name) => name,
            None => return Err(RepositoryError::DataNotProvied),
        };
        self.run_command(&format!(
            "git remote add aur ssh://aur@aur.archlinux.org/{}.git",
            name
        ))?;
        self.run_command("git fetch aur")
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct RepositoryData {
    package_name: Option<String>,
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
    DataNotProvied,
    SerializeError,
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
