use cli;
use data::{Error, FileDependency, Package};
use std::process::Command;

pub fn check_required_programs(settings: &cli::CommandLineSettings) -> Result<(), Error> {
    match check_required_program("pacman") {
        Err(_) => return Err(Error::Dependency("pacman")),
        _ => {}
    }
    match check_required_program("file") {
        Err(_) => return Err(Error::Dependency("file")),
        _ => {}
    }
    match &settings.command {
        &cli::Command::Ldd => match check_required_program("ldd") {
            Err(_) => return Err(Error::Dependency("ldd")),
            _ => {}
        },
        &cli::Command::Readelf => match check_required_program("readelf") {
            Err(_) => return Err(Error::Dependency("readelf")),
            _ => {}
        },
    }
    if settings.show_candidates {
        match check_required_program("pkgfile") {
            Err(_) => return Err(Error::Dependency("pkgfile")),
            _ => {}
        }
    }
    Ok(())
}

fn check_required_program(program: &str) -> Result<(), ()> {
    match Command::new("which").arg(program).output() {
        Err(_) => Err(()),
        _ => Ok(()),
    }
}

pub fn get_all_packages(settings: &mut cli::CommandLineSettings) -> Result<(), Error> {
    let out = Command::new("pacman").arg("-Qqm").output()?;
    let output = String::from_utf8_lossy(&out.stdout);
    let output = output.into_owned();
    for package in output.lines() {
        settings.packages.push(package.into());
    }
    Ok(())
}

pub fn get_files_for_package<'a>(package: &Package) -> Result<Vec<String>, Error<'a>> {
    let mut files = Vec::new();
    let out = Command::new("pacman")
        .arg("-Qql")
        .arg(&package.name)
        .output()?;
    let output = String::from_utf8_lossy(&out.stdout);
    let output = output.into_owned();
    for package in output.lines() {
        files.push(package.into());
    }
    Ok(files)
}

pub fn file_is_elf<'a>(file: &str) -> Result<bool, Error<'a>> {
    let out = Command::new("file").arg(&file).output()?;
    let output = String::from_utf8_lossy(&out.stdout);
    Ok(output.contains("ELF"))
}

pub fn verify_files_via_ldd<'a>(
    file: &str,
    settings: &cli::CommandLineSettings,
    filenames: &Vec<String>,
) -> Result<Option<FileDependency>, Error<'a>> {
    let mut dependency = FileDependency::default();
    dependency.file_name = String::from(file);
    let out = Command::new("ldd").arg(&file).output()?;
    let output = String::from_utf8_lossy(&out.stdout);
    for line in output.lines() {
        if line.ends_with("=> not found") {
            let library_name = String::from(line.replace("=> not found", "").trim());
            // only add if library is not in ignore
            if !settings.ignore_libraries.contains(&library_name)
                // library is not in package file
                && !filenames.contains(&library_name)
                // and was not already found
                && !dependency.library_dependencies.contains(&library_name)
            {
                dependency.library_dependencies.push(library_name);
            }
        }
    }
    if dependency.library_dependencies.len() > 0 {
        Ok(Some(dependency))
    } else {
        Ok(None)
    }
}

pub fn verify_files_via_readelf<'a>(
    file: &str,
    _settings: &cli::CommandLineSettings,
    _filenames: &Vec<String>,
) -> Result<Option<FileDependency>, Error<'a>> {
    let mut dependency = FileDependency::default();
    dependency.file_name = String::from(file);
    if dependency.library_dependencies.len() > 0 {
        Ok(Some(dependency))
    } else {
        Ok(None)
    }
}

pub fn get_packages_containing_library<'a>(library: &str) -> Result<Vec<String>, Error<'a>> {
    let mut packages = Vec::new();
    let out = Command::new("pkgfile").arg(&library).output()?;
    let output = String::from_utf8_lossy(&out.stdout);
    let output = output.into_owned();
    for package in output.lines() {
        packages.push(package.into());
    }
    Ok(packages)
}
