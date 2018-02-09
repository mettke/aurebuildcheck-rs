use std::{error, fmt, io};
use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(Debug)]
pub enum Error<'a> {
    Dependency(&'a str),
    Execution(io::Error),
    ExecutionError(String),
}

impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Dependency(ref dep) => write!(f, "Dependency missing: {}", dep),
            Error::Execution(ref err) => write!(f, "Command execution error: {}", err),
            Error::ExecutionError(ref err) => write!(f, "Command execution error: {:#?}", err),
        }
    }
}

impl<'a> error::Error for Error<'a> {
    fn description(&self) -> &str {
        match *self {
            Error::Dependency(_) => {
                "Dependency is missing and must be installed before running this command"
            }
            Error::Execution(ref err) => err.description(),
            Error::ExecutionError(_) => "Execution of program failed with non zero",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Dependency(_) => None,
            Error::Execution(ref err) => Some(err),
            Error::ExecutionError(_) => None,
        }
    }
}

impl<'a> From<io::Error> for Error<'a> {
    fn from(err: io::Error) -> Self {
        Error::Execution(err)
    }
}

#[derive(Debug)]
pub struct Package {
    pub name: String,
    //#[serde(skip_serializing_if = "path")]
    pub file_dependencies: Vec<FileDependency>,
    //#[serde(skip_serializing_if = "path")]
    pub library_requirements: Vec<LibraryRequired>,
    // #[serde(skip_serializing_if = "path")]
    pub packages_containing: Vec<PackagesContaining>,
}

impl Package {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Package {
            name: name.into(),
            file_dependencies: vec![],
            library_requirements: vec![],
            packages_containing: vec![],
        }
    }
}

impl Ord for Package {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Package {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Package {}

impl PartialEq for Package {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Debug, Default)]
pub struct FileDependency {
    pub file_name: String,
    pub library_dependencies: HashSet<String>,
}

#[derive(Debug, Default)]
pub struct LibraryRequired {
    pub library_name: String,
    pub files_requiring: Vec<String>,
}

#[derive(Debug, Default)]
pub struct PackagesContaining {
    pub library_name: String,
    pub packages_containing: Vec<String>,
}
