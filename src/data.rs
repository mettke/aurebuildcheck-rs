use regex;
use std::{error, fmt, io};
use std::collections::HashSet;
use std::iter::FromIterator;
use std::rc::Rc;

#[derive(Debug)]
pub enum Error<'a> {
    Dependency(&'a str),
    Execution(io::Error),
    ExecutionError(String),
    RegexError(regex::Error),
}

impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Dependency(dep) => write!(f, "Dependency missing: {}", dep),
            Error::Execution(ref err) => write!(f, "Command execution error: {}", err),
            Error::ExecutionError(ref err) => write!(f, "Command execution error: {:#?}", err),
            Error::RegexError(ref err) => write!(f, "Regex Error: {:#?}", err),
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
            Error::RegexError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Dependency(_) | Error::ExecutionError(_) => None,
            Error::Execution(ref err) => Some(err),
            Error::RegexError(ref err) => Some(err),
        }
    }
}

impl<'a> From<io::Error> for Error<'a> {
    fn from(err: io::Error) -> Self {
        Error::Execution(err)
    }
}

impl<'a> From<regex::Error> for Error<'a> {
    fn from(err: regex::Error) -> Self {
        Error::RegexError(err)
    }
}

#[derive(Debug)]
pub struct ProcessingPackage {
    pub name: String,
    pub file_dependencies: Vec<ProcessingFileDependency>,
}

impl ProcessingPackage {
    pub fn new<S: Into<String>>(name: S) -> Self {
        ProcessingPackage {
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
        Package {
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
        FileDependency {
            file_name: Rc::new(dependency.file_name),
            library_dependencies: HashSet::from_iter(
                dependency
                    .library_dependencies
                    .into_iter()
                    .map(|entry| Rc::new(entry)),
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
