use std::{ error::Error, fmt::Display, fs::{ self }, io::{ self, Read, Write }, process::Command };

use num_derive::FromPrimitive;
use serde::{ Deserialize, Serialize };

const DATA_PATH: &str = "data.json";

#[derive(Default)]
pub struct Repository {
    pub data: RepositoryData,
    path: String,
}

impl Repository {
    // creates a new repository
    pub fn new(repo_path: &str) -> RResult<Self> {
        // check that directory is empty
        match Self::is_empty(repo_path) {
            Err(_) => {
                return Err(RepositoryError::NotARepository);
            }
            Ok(false) => {
                return Err(RepositoryError::FolderNotEmpty);
            }
            Ok(true) => (),
        }

        // init git repo
        Command::new(format!("git"))
            .arg("init")
            .arg(repo_path)
            .output()
            .or(Err(RepositoryError::GitError))?;

        // create new data for repo
        let data = RepositoryData::default();

        // add gitignore
        let mut gitignore = fs::File
            ::create(format!("{}/.gitignore", repo_path))
            .or(Err(RepositoryError::FileError))?;

        gitignore
            .write_all(
                b"
#Ignore everything
*

# But not these files...
!.gitignore
!PKGBUILD
!.SRCINFO
!data.json
!.git/*
!package.install
!package.changelog

# ...even if they are in subdirectories
!*/
"
            )
            .or(Err(RepositoryError::FileError))?;

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

    // checks if all required fields are set, exports to pkgbuild, builds package and uploads package
    pub fn publish(&self) -> RResult<()> {
        // check if all values are set
        if !self.required_set() {
            return Err(RepositoryError::DataNotProvied);
        }
        // export to pkgbuild
        self.export_to_pkgbuild()?;
        // build package
        self.build_package()
        // set remote, is expected to fail if remote is already set
        // self.register_package()?;
        // publish package
        // self.upload()
    }

    // deletes every data point except path
    pub fn clear(&mut self) {
        let new_data = RepositoryData::default();
        self.data = new_data;
    }

    // builds the package
    fn build_package(&self) -> RResult<()> {
        // get srcinfo
        let output = Command::new("makepkg")
            .current_dir(self.path.clone())
            .arg("--printsrcinfo")
            .output()
            .or(Err(RepositoryError::MAKEPKGError))?;
        let srcinfo = output.stdout.as_slice();

        // save srcinfo
        let mut srcinfo_file = fs::File
            ::create(format!("{}/.SRCINFO", self.path))
            .or(Err(RepositoryError::FileError))?;

        srcinfo_file.write_all(srcinfo).or(Err(RepositoryError::FileError))
    }

    // uploads the repository to the aur
    fn upload(&self) -> RResult<()> {
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
        let mut file = fs::File
            ::create(&format!("{}/{}", self.path, DATA_PATH))
            .or(Err(RepositoryError::SerializeError))?;

        file.write_all(json.as_bytes()).or(Err(RepositoryError::SerializeError))?;
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
    fn export_to_pkgbuild(&self) -> RResult<()> {
        struct CalculatedData {
            prepare: Option<String>,
            build: Option<String>,
            check: Option<String>,
            package: Option<String>,
            source: Vec<String>,
        }

        let calc_data: CalculatedData = match self.data.package_type {
            PackageType::Binary => {
                todo!()
                // let source = self.data.source;
                // if source.len() != 1 {
                //     return Err(RepositoryError::DataNotProvied);
                // }
                // let url = format!("$pkgname-$pkgver::{}", source.get(0).unwrap());
                // CalculatedData {
                //     prepare: None,
                //     build: None,
                //     check: None,
                //     package: Some("cd $pkgname-$pkgver

                //     ".to_string()),
                //     source: vec![url],
                // }
            }
            PackageType::Make => todo!(),
            PackageType::Cargo => todo!(),
            PackageType::Custom =>
                CalculatedData {
                    prepare: self.data.prepare.clone(),
                    build: self.data.build.clone(),
                    check: self.data.check.clone(),
                    package: self.data.package.clone(),
                    source: self.data.source.clone(),
                },
        };

        let mut pkgbuild = fs::File
            ::create(format!("{}/PKGBUILD", self.path))
            .or(Err(RepositoryError::NotARepository))?;

        let string = format!(
            "# Maintainer: {username} <{email}>
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
            name = self.data.name.clone().unwrap_or(String::new()).trim(),
            email = self.data.email.clone().unwrap_or(String::new()).trim(),
            username = self.data.username.clone().unwrap_or(String::new()).trim(),
            version = self.data.version.clone().unwrap_or(String::new()).trim(),
            rel = self.data.rel.clone().unwrap_or(String::new()).trim(),
            epoch = self.data.epoch.clone().unwrap_or(String::new()).trim(),
            desc = self.data.desc.clone().unwrap_or(String::new()).trim(),
            arch = self.data.arch
                .iter()
                .map(|val| format!("'{}'", val.trim()))
                .collect::<Vec<String>>()
                .join(" "),
            url = self.data.url.clone().unwrap_or(String::new()).trim(),
            license = self.data.license
                .iter()
                .map(|val| format!("'{}'", val.trim()))
                .collect::<Vec<String>>()
                .join(" "),
            groups = self.data.groups
                .iter()
                .map(|val| format!("'{}'", val.trim()))
                .collect::<Vec<String>>()
                .join(" "),
            depends = self.data.depends
                .iter()
                .map(|val| format!("'{}'", val.trim()))
                .collect::<Vec<String>>()
                .join(" "),
            makedepends = self.data.makedepends
                .iter()
                .map(|val| format!("'{}'", val.trim()))
                .collect::<Vec<String>>()
                .join(" "),
            checkdepends = self.data.checkdepends
                .iter()
                .map(|val| format!("'{}'", val.trim()))
                .collect::<Vec<String>>()
                .join(" "),
            optdepends = self.data.optdepends
                .iter()
                .map(|val| format!("'{}'", val.trim()))
                .collect::<Vec<String>>()
                .join(" "),
            provides = self.data.provides
                .iter()
                .map(|val| format!("'{}'", val.trim()))
                .collect::<Vec<String>>()
                .join(" "),
            conflicts = self.data.conflicts
                .iter()
                .map(|val| format!("'{}'", val.trim()))
                .collect::<Vec<String>>()
                .join(" "),
            replaces = self.data.replaces
                .iter()
                .map(|val| format!("'{}'", val.trim()))
                .collect::<Vec<String>>()
                .join(" "),
            backup = self.data.backup
                .iter()
                .map(|val| format!("'{}'", val.trim()))
                .collect::<Vec<String>>()
                .join(" "),
            options = self.data.options
                .iter()
                .map(|val| format!("'{}'", val.trim()))
                .collect::<Vec<String>>()
                .join(" "),
            install = self.data.install.clone().unwrap_or(String::new()).trim(),
            changelog = self.data.changelog.clone().unwrap_or(String::new()).trim(),
            source = calc_data.source
                .iter()
                .map(|val| format!("'{}'", val.trim()))
                .collect::<Vec<String>>()
                .join(" "),
            noextract = self.data.noextract
                .iter()
                .map(|val| format!("'{}'", val.trim()))
                .collect::<Vec<String>>()
                .join(" "),
            md5sums = self.data.md5sums
                .iter()
                .map(|val| format!("'{}'", val.trim()))
                .collect::<Vec<String>>()
                .join(" "),
            gpgkeys = self.data.pgpkeys
                .iter()
                .map(|val| format!("'{}'", val.trim()))
                .collect::<Vec<String>>()
                .join(" "),
            prepare = calc_data.prepare.clone().unwrap_or("echo ''".to_string()),
            build = self.data.build.clone().unwrap_or("echo ''".to_string()),
            check = self.data.check.clone().unwrap_or("echo ''".to_string()),
            package = self.data.package.clone().unwrap_or("echo ''".to_string())
        );

        pkgbuild.write_all(string.as_bytes()).or(Err(RepositoryError::PKGBUILDError))
    }

    // removes everything stored at path, doesn't do checks
    // # USE CAREFULLY!
    pub fn delete(&self) {
        fs::remove_dir_all(self.path.clone());
    }

    // checks if the required options are set
    fn required_set(&self) -> bool {
        if let Some(name) = &self.data.name {
            if name.is_empty() {
                return false;
            }
        } else {
            return false;
        }
        if let Some(version) = &self.data.version {
            if version.is_empty() {
                return false;
            }
        } else {
            return false;
        }
        if let Some(rel) = &self.data.rel {
            if rel.is_empty() {
                return false;
            }
        } else {
            return false;
        }

        !self.data.arch.is_empty()
    }

    fn read_data(repo_path: &str) -> RResult<RepositoryData> {
        let mut file = fs::File
            ::open(&format!("{}/{}", repo_path, DATA_PATH))
            .or(Err(RepositoryError::NotARepository))?;

        let mut string_data = String::new();
        file.read_to_string(&mut string_data).or(Err(RepositoryError::NotARepository))?;

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
            None => {
                return Err(RepositoryError::DataNotProvied);
            }
        };
        // remove repo, can cause error if remote doesn't exist so just ignore
        self.run_command("git", vec!["remote", "remove", "aur"]).ok();
        // add repo
        self.run_command(
            "git",
            vec!["remote", "add", "aur", &format!("ssh://aur@aur.archlinux.org/{}.git", name)]
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
    pub package_type: PackageType,
}

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize, FromPrimitive)]
pub enum PackageType {
    #[default]
    Binary,
    Make,
    Cargo,
    Custom,
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
    FileError,
    MAKEPKGError,
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
            RepositoryError::FileError => "File Error",
            RepositoryError::MAKEPKGError => "makepkg Error",
        };

        write!(f, "{}", text)
    }
}