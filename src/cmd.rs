use cli;
use data::{Error, FileDependency, Package};
use std::process::{Command, Output};

pub fn check_required_programs(settings: &cli::CommandLineSettings) -> Result<(), Error> {
    check_required_program("pacman")?;
    check_required_program("file")?;
    match &settings.command {
        &cli::Command::Ldd => check_required_program("ldd")?,
        &cli::Command::Readelf => check_required_program("readelf")?,
    }
    if settings.show_candidates {
        check_required_program("pkgfile")?;
    }
    Ok(())
}

fn check_required_program<'a>(program: &'a str) -> Result<(), Error<'a>> {
    match execute_command(Command::new("which").arg(program)) {
        Err(_) => Err(Error::Dependency(program)),
        _ => Ok(()),
    }
}

fn execute_command<'a>(command: &mut Command) -> Result<Output, Error<'a>> {
    let out = command.output()?;
    if !out.status.success() {
        return Err(Error::ExecutionError(
            String::from_utf8_lossy(&out.stderr).into_owned(),
        ));
    }
    Ok(out)
}

pub fn get_all_packages(settings: &mut cli::CommandLineSettings) -> Result<(), Error> {
    let out = execute_command(Command::new("pacman").arg("-Qqm"))?;
    let output = String::from_utf8_lossy(&out.stdout);
    let output = output.into_owned();
    for package in output.lines() {
        settings.packages.push(package.into());
    }
    Ok(())
}

pub fn get_files_for_package<'a>(package: &Package) -> Result<Vec<String>, Error<'a>> {
    let mut files = Vec::new();
    let out = execute_command(Command::new("pacman").arg("-Qql").arg(&package.name))?;
    let output = String::from_utf8_lossy(&out.stdout);
    let output = output.into_owned();
    for file in output.lines() {
        files.push(file.into());
    }
    Ok(files)
}

pub fn file_is_elf<'a>(file: &str) -> Result<bool, Error<'a>> {
    let out = execute_command(Command::new("file").arg(&file))?;
    let output = String::from_utf8_lossy(&out.stdout);
    Ok(output.contains("ELF"))
}

pub fn verify_files_via_ldd<'a>(file: &str) -> Result<Option<FileDependency>, Error<'a>> {
    let mut dependency = FileDependency::default();
    dependency.file_name = String::from(file);
    let out = Command::new("ldd").arg(&file).output()?;
    // TODO: ldd prints warnings - should be included in verbose output
    let output = String::from_utf8_lossy(&out.stdout);
    for line in output.lines() {
        if line.ends_with(" => not found") {
            let mut library_name = String::from(line.trim());
            let new_length = library_name.len() - " => not found".len();
            library_name.truncate(new_length);
            dependency.library_dependencies.insert(library_name);
        }
    }
    if dependency.library_dependencies.is_empty() {
        Ok(None)
    } else {
        Ok(Some(dependency))        
    }
}

pub fn verify_files_via_readelf<'a>(file: &str) -> Result<Option<FileDependency>, Error<'a>> {
    let mut dependency = FileDependency::default();
    dependency.file_name = String::from(file);
    if dependency.library_dependencies.is_empty() {
        Ok(None)
    } else {
        Ok(Some(dependency))        
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
