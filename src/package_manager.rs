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

    // gets a clone of the path
    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    // exports everything to the package build
    pub fn export_to_pkgbuild(&self) -> RResult<()> {
        let mut pkgbuild = fs::File::create(format!("{}/PKGBUILD", self.path))
            .or(Err(RepositoryError::NotARepository))?;

        let string = format!(
            "
# Maintainer: {username} <{email}>
pkgname={name}
pkgver={version}
pkgrel={rel}
epoch={epoch}
pkgdesc=\"{desc}\"
arch=({arch})
url=\"{url}\"
license=({license})
groups=({groups})
depends=({depends})
makedepends=({makedepends})
checkdepends=({checkdepends})
optdepends=({optdepends})
provides=({provides})
conflicts=({conflicts})
replaces=({replaces})
backup=({backup})
options=({options})
install={install}
changelog={changelog}
source=({source})
noextract=({noextract})
md5sums=({md5sums})
validpgpkeys=({gpgkeys})

prepare() {{
        {prepare}
}}

build() {{
        {build}
}}

check() {{
        {check}
}}

package() {{
        {package}
}}
",
            name = self.data.name.clone().unwrap_or(String::new()),
            email = self.data.email.clone().unwrap_or(String::new()),
            username = self.data.username.clone().unwrap_or(String::new()),
            version = self.data.version.clone().unwrap_or(String::new()),
            rel = self.data.rel.clone().unwrap_or(String::new()),
            epoch = self.data.epoch.clone().unwrap_or(String::new()),
            desc = self.data.desc.clone().unwrap_or(String::new()),
            arch = self
                .data
                .arch
                .iter()
                .map(|val| format!("'{}'", val))
                .collect::<Vec<String>>()
                .join(","),
            url = self.data.url.clone().unwrap_or(String::new()),
            license = self
                .data
                .license
                .iter()
                .map(|val| format!("'{}'", val))
                .collect::<Vec<String>>()
                .join(","),
            groups = self
                .data
                .groups
                .iter()
                .map(|val| format!("'{}'", val))
                .collect::<Vec<String>>()
                .join(","),
            depends = self
                .data
                .depends
                .iter()
                .map(|val| format!("'{}'", val))
                .collect::<Vec<String>>()
                .join(","),
            makedepends = self
                .data
                .makedepends
                .iter()
                .map(|val| format!("'{}'", val))
                .collect::<Vec<String>>()
                .join(","),
            checkdepends = self
                .data
                .checkdepends
                .iter()
                .map(|val| format!("'{}'", val))
                .collect::<Vec<String>>()
                .join(","),
            optdepends = self
                .data
                .optdepends
                .iter()
                .map(|val| format!("'{}'", val))
                .collect::<Vec<String>>()
                .join(","),
            provides = self
                .data
                .provides
                .iter()
                .map(|val| format!("'{}'", val))
                .collect::<Vec<String>>()
                .join(","),
            conflicts = self
                .data
                .conflicts
                .iter()
                .map(|val| format!("'{}'", val))
                .collect::<Vec<String>>()
                .join(","),
            replaces = self
                .data
                .replaces
                .iter()
                .map(|val| format!("'{}'", val))
                .collect::<Vec<String>>()
                .join(","),
            backup = self
                .data
                .backup
                .iter()
                .map(|val| format!("'{}'", val))
                .collect::<Vec<String>>()
                .join(","),
            options = self
                .data
                .options
                .iter()
                .map(|val| format!("'{}'", val))
                .collect::<Vec<String>>()
                .join(","),
            install = self.data.install.clone().unwrap_or(String::new()),
            changelog = self.data.changelog.clone().unwrap_or(String::new()),
            source = self
                .data
                .source
                .iter()
                .map(|val| format!("'{}'", val))
                .collect::<Vec<String>>()
                .join(","),
            noextract = self
                .data
                .noextract
                .iter()
                .map(|val| format!("'{}'", val))
                .collect::<Vec<String>>()
                .join(","),
            md5sums = self
                .data
                .md5sums
                .iter()
                .map(|val| format!("'{}'", val))
                .collect::<Vec<String>>()
                .join(","),
            gpgkeys = self
                .data
                .pgpkeys
                .iter()
                .map(|val| format!("'{}'", val))
                .collect::<Vec<String>>()
                .join(","),
            prepare = self.data.prepare.clone().unwrap_or(String::new()),
            build = self.data.build.clone().unwrap_or(String::new()),
            check = self.data.check.clone().unwrap_or(String::new()),
            package = self.data.package.clone().unwrap_or(String::new()),
        );

        pkgbuild
            .write_all(string.as_bytes())
            .or(Err(RepositoryError::PKGBUILDError))
    }

    // removes everything stored at path, doesn't do checks
    // # USE CAREFULLY!
    pub fn delete(&self) {
        fs::remove_dir_all(self.path.clone());
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
        let name = match &self.data.name {
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
    pub username: Option<String>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub version: Option<String>,
    pub rel: Option<String>,
    pub epoch: Option<String>,
    pub desc: Option<String>,
    pub arch: Vec<String>,
    pub url: Option<String>,
    pub license: Vec<String>,
    pub groups: Vec<String>,
    pub depends: Vec<String>,
    pub makedepends: Vec<String>,
    pub checkdepends: Vec<String>,
    pub optdepends: Vec<String>,
    pub provides: Vec<String>,
    pub conflicts: Vec<String>,
    pub replaces: Vec<String>,
    pub backup: Vec<String>,
    pub options: Vec<String>,
    pub install: Option<String>,
    pub changelog: Option<String>,
    pub source: Vec<String>,
    pub noextract: Vec<String>,
    pub pgpkeys: Vec<String>,
    pub md5sums: Vec<String>,
    pub prepare: Option<String>,
    pub build: Option<String>,
    pub check: Option<String>,
    pub package: Option<String>,
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
    PKGBUILDError,
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
            RepositoryError::PKGBUILDError => "PKGBUILD Error",
        };

        write!(f, "{}", text)
    }
}
