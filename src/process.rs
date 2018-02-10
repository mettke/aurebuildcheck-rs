use cli::{Command, CommandLineSettings};
use cmd;
use data::{Error, LibraryRequired, Package, PackagesContaining,
           ProcessingFileDependency, ProcessingPackage};
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;

pub fn verify_packages(settings: &CommandLineSettings) -> Result<Vec<Package>, Error> {
    let mut packages = settings.packages
        .par_iter()
        .map(|package| verify_package(package, settings))
        .collect::<Result<Vec<ProcessingPackage>, Error>>()?
        .into_iter()
        .map(|package| {
            package.into()
        })
        .collect::<Vec<Package>>();
 
    packages.iter_mut().map(|mut package| {
        setup_library_requirements(&mut package)?;
        if settings.show_candidates {
            setup_packages_containing(&mut package)?;
        }
        Ok(())
    }).collect::<Result<Vec<_>, Error>>()?;

    Ok(packages)
}

fn verify_package<'a>(
    package_name: &String,
    settings: &CommandLineSettings,
) -> Result<ProcessingPackage, Error<'a>> {
    let files = cmd::get_files_for_package(package_name)?;
    let filenames = get_filenames_from_files(&files);
    let mut package = ProcessingPackage::new(package_name.clone());

    package.file_dependencies = files
        .par_iter()
        // verify files parallel - will stop if error occures
        .map(|file| verify_file(file, settings))
        // collect and abort if error
        .collect::<Result<Vec<Option<ProcessingFileDependency>>, Error>>()?
        .into_iter()
        // remove Option
        .filter_map(|element| element)
        .collect::<Vec<ProcessingFileDependency>>();
    remove_ignored_or_packaged_libraries(&mut package, filenames, settings);

    Ok(package)
}

fn get_filenames_from_files(files: &Vec<String>) -> Vec<String> {
    files
        .iter()
        .filter_map(|file| {
            if let Some(path) = PathBuf::from(file).file_name() {
                if let Some(filename) = path.to_str() {
                    return Some(String::from(filename));
                }
            }
            None
        })
        .collect::<Vec<String>>()
}

fn verify_file<'a>(
    file: &str,
    settings: &CommandLineSettings,
) -> Result<Option<ProcessingFileDependency>, Error<'a>> {
    if file_might_be_binary(file) && cmd::file_is_elf(file)? {
        let dependency = match settings.command {
            Command::Ldd => cmd::verify_files_via_ldd(file),
            Command::Readelf => cmd::verify_files_via_readelf(file),
        }?;
        return Ok(dependency);
    }
    Ok(None)
}

fn file_might_be_binary(file: &str) -> bool {
    let path = PathBuf::from(file);
    if !path.is_file() {
        return false;
    }
    if let Some(extension) = path.extension() {
        if let Some(ext) = extension.to_str() {
            return match ext {
                "a" | "png" | "la" | "ttf" | "gz" | "html" | "css" | "h" | "c" | "cxx" | "xml"
                | "rgb" | "gif" | "wav" | "ogg" | "ogv" | "avi" | "opus" | "mp3" | "po" | "txt"
                | "jpg" | "jpeg" | "bmp" | "xcf" | "mo" | "rb" | "py" | "lua" | "config"
                | "cfg" | "svg" | "desktop" | "conf" | "pdf" | "xz" => false,
                "so" | _ => true,
            };
        }
    }
    true
}

fn remove_ignored_or_packaged_libraries<'a>(
    package: &mut ProcessingPackage,
    filenames: Vec<String>,
    settings: &CommandLineSettings,
) {
    package
        .file_dependencies
        .iter_mut()
        .for_each(|file_dependency| {
            file_dependency
                .library_dependencies
                .retain(|library_dependency| {
                    !settings.ignore_libraries.contains(library_dependency)
                        && !filenames.contains(library_dependency)
                        && !settings.ignore_libraries_regex.iter().fold(
                            false,
                            |_, ignore_libraries_regex| {
                                ignore_libraries_regex.is_match(library_dependency)
                            },
                        )
                });
        });
    package
        .file_dependencies
        .retain(|file_dependency| file_dependency.library_dependencies.len() > 0);
}

fn setup_library_requirements<'a>(package: &mut Package) -> Result<(), Error<'a>> {
    let mut cache: HashMap<String, LibraryRequired> = HashMap::new();
    package
        .file_dependencies
        .iter()
        .for_each(|file_dependency| {
            file_dependency
                .library_dependencies
                .iter()
                .for_each(|library_dependency| {
                    if cache.contains_key(&**library_dependency) {
                        if let Some(value) = cache.get_mut(&**library_dependency) {
                            value
                                .files_requiring
                                .push(file_dependency.file_name.clone());
                        }
                    } else {
                        cache.insert(
                            (**library_dependency).clone(),
                            LibraryRequired {
                                library_name: library_dependency.clone(),
                                files_requiring: vec![file_dependency.file_name.clone()],
                            },
                        );
                    }
                })
        });
    package.library_requirements = cache.into_iter().map(|(_, val)| val).collect();
    Ok(())
}

fn setup_packages_containing<'a>(package: &mut Package) -> Result<(), Error<'a>> {
    package.packages_containing = package
        .library_requirements
        .iter()
        .map(|library| {
            Ok(PackagesContaining {
                library_name: library.library_name.clone(),
                packages_containing: cmd::get_packages_containing_library(&library.library_name)?,
            })
        })
        .collect::<Result<Vec<PackagesContaining>, Error>>()?;
    Ok(())
}
