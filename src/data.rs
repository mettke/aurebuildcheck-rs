use regex;
use std::{collections::HashSet, error, fmt, io, iter::FromIterator, rc::Rc};

#[derive(Debug)]
pub enum Error<'a> {
    Dependency(&'a str),
    ExecutionIO(io::Error),
    Execution(String),
    Regex(regex::Error),
}

impl fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Dependency(dep) => write!(f, "Dependency missing: {}", dep),
            Error::ExecutionIO(ref err) => write!(f, "Command ExecutionIO error: {}", err),
            Error::Execution(ref err) => write!(f, "Command ExecutionIO error: {}", err),
            Error::Regex(ref err) => write!(f, "Regex Error: {}", err),
        }
    }
}

impl error::Error for Error<'_> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::Dependency(_) | Error::Execution(_) => None,
            Error::ExecutionIO(ref err) => Some(err),
            Error::Regex(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for Error<'_> {
    fn from(err: io::Error) -> Self {
        Error::ExecutionIO(err)
    }
}

impl From<regex::Error> for Error<'_> {
    fn from(err: regex::Error) -> Self {
        Error::Regex(err)
    }
}

#[derive(Debug)]
pub struct ProcessingPackage {
    pub name: String,
    pub file_dependencies: Vec<ProcessingFileDependency>,
}

impl ProcessingPackage {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            file_dependencies: vec![],
        }
    }
}

#[derive(Debug, Default)]
pub struct ProcessingFileDependency {
    pub file_name: String,
    pub library_dependencies: HashSet<String>,
}

#[derive(Debug)]
pub struct Package {
    pub name: String,
    pub file_dependencies: Vec<FileDependency>,
    pub library_requirements: Vec<LibraryRequired>,
    pub packages_containing: Vec<PackagesContaining>,
}

impl From<ProcessingPackage> for Package {
    fn from(package: ProcessingPackage) -> Self {
        Self {
            name: package.name,
            file_dependencies: package
                .file_dependencies
                .into_iter()
                .map(|dependency| dependency.into())
                .collect(),
            library_requirements: vec![],
            packages_containing: vec![],
        }
    }
}

#[derive(Debug, Default)]
pub struct FileDependency {
    pub file_name: Rc<String>,
    pub library_dependencies: HashSet<Rc<String>>,
}

impl From<ProcessingFileDependency> for FileDependency {
    fn from(dependency: ProcessingFileDependency) -> Self {
        Self {
            file_name: Rc::new(dependency.file_name),
            library_dependencies: HashSet::from_iter(
                dependency.library_dependencies.into_iter().map(Rc::new),
            ),
        }
    }
}

#[derive(Debug, Default)]
pub struct LibraryRequired {
    pub library_name: Rc<String>,
    pub files_requiring: Vec<Rc<String>>,
}

#[derive(Debug, Default)]
pub struct PackagesContaining {
    pub library_name: Rc<String>,
    pub packages_containing: Vec<String>,
}
