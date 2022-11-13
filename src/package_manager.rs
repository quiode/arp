use std::{
    error::Error,
    fmt::Display,
    fs::{self},
    io::{self, Read, Write},
    process::Command,
};

use serde::{Deserialize, Serialize};

const DATA_PATH: &str = "data.json";

#[derive(Default)]
pub struct Repository {
    pub data: RepositoryData,
    path: String,
}

impl Repository {
    // creates a new repository
    pub fn new(repo_path: &str) -> RResult<Self> {
        match Self::is_empty(repo_path) {
            Err(_) => return Err(RepositoryError::NotARepository),
            Ok(false) => return Err(RepositoryError::FolderNotEmpty),
            Ok(true) => (),
        }

        Command::new(format!("git"))
            .arg("init")
            .arg(repo_path)
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
        let data = Self::read_data(repo_path)?;

        Ok(Self {
            path: repo_path.to_string(),
            data,
        })
    }

    // uploads the repository to the aur
    pub fn upload(&self) -> RResult<()> {
        self.run_command("git", vec!["fetch", "aur"])?;
        self.run_command("git", vec!["add", "."])?;
        self.run_command("git", vec!["commit", "-m", "\"commit through arp\""])?;
        self.run_command("git", vec!["push", "aur"])
    }

    // returns the repository data from a json file
    // saves repo data to a json file
    pub fn save_data(&self) -> RResult<()> {
        let json = serde_json::to_string(&self.data).or(Err(RepositoryError::SerializeError))?;

        // save file
        let mut file = fs::File::create(&format!("{}/{}", self.path, DATA_PATH))
            .or(Err(RepositoryError::SerializeError))?;

        file.write_all(json.as_bytes())
            .or(Err(RepositoryError::SerializeError))?;
        Ok(())
    }

    // loads new path
    pub fn load_path(&mut self, new_path: &str) -> RResult<()> {
        // save data
        self.save_data();
        let data = Self::read_data(new_path)?;

        self.path = new_path.to_string();
        self.data = data;
        Ok(())
    }

    fn read_data(repo_path: &str) -> RResult<RepositoryData> {
        let mut file = fs::File::open(&format!("{}/{}", repo_path, DATA_PATH))
            .or(Err(RepositoryError::NotARepository))?;

        let mut string_data = String::new();
        file.read_to_string(&mut string_data)
            .or(Err(RepositoryError::NotARepository))?;

        Ok(serde_json::from_str(&string_data).or(Err(RepositoryError::NotARepository))?)
    }

    // returns tru if folder is empty
    fn is_empty(path: &str) -> io::Result<bool> {
        Ok(fs::read_dir(path)?.next().is_none())
    }

    fn run_command(&self, command: &str, args: Vec<&str>) -> RResult<()> {
        Command::new(command)
            .args(args)
            .current_dir(&self.path)
            .output()
            .or(Err(RepositoryError::GitFetchError))?;
        Ok(())
    }

    // adds the aur as a remote and performs a fetch to register the package on the aur
    fn register_package(&self) -> RResult<()> {
        let name = match &self.data.package_name {
            Some(name) => name,
            None => return Err(RepositoryError::DataNotProvied),
        };
        self.run_command(
            "git",
            vec![
                "remote",
                "add",
                "aur",
                &format!("ssh://aur@aur.archlinux.org/{}.git", name),
            ],
        )?;
        self.run_command("git", vec!["fetch", "aur"])
    }
}

impl Drop for Repository {
    fn drop(&mut self) {
        self.save_data().ok();
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RepositoryData {
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
    DataNotProvied,
    SerializeError,
}

impl Error for RepositoryError {}

impl Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            RepositoryError::NotARepository => "Not a Repository",
            RepositoryError::FolderNotEmpty => "Folder not Empty",
            RepositoryError::GitError => "Git Error",
            RepositoryError::GitFetchError => "Git Fetch Error",
            RepositoryError::DataNotProvied => "Data not Provied",
            RepositoryError::SerializeError => "Serialize Error",
        };

        write!(f, "{}", text)
    }
}
