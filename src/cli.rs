use crate::data::Error;
use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, AppSettings, Arg,
    ArgMatches, SubCommand,
};
use regex::RegexSet;

/// Specifies the various ways to check elf files for missing libraries
#[derive(Debug)]
pub enum Command {
    Ldd,
    Readelf,
}

// Specifies the various ways to output the missing library information
#[derive(Debug)]
pub enum Output {
    Console,
    JSON,
}

/// These Settings define how the program operates and are used everywhere
#[derive(Debug)]
pub struct CommandLineSettings {
    pub command: Command,
    pub packages: Vec<String>,
    pub all_packages: bool,
    pub ignore_libraries: Vec<String>,
    pub ignore_libraries_regex: Option<RegexSet>,
    pub show_candidates: bool,
    pub output: Output,
    pub quite: bool,
    pub group_by_file: bool,
    pub group_by_library: bool,
    pub group_by_containing_package: bool,
}

impl Default for CommandLineSettings {
    fn default() -> Self {
        Self {
            command: Command::Ldd,
            packages: vec![],
            all_packages: false,
            ignore_libraries: vec![],
            ignore_libraries_regex: None,
            show_candidates: false,
            output: Output::Console,
            quite: false,
            group_by_file: false,
            group_by_library: false,
            group_by_containing_package: false,
        }
    }
}

pub fn get_command_line_settings<'a>() -> Result<CommandLineSettings, Error<'a>> {
    let mut settings = CommandLineSettings::default();
    let parser = setup_command_line_parser();

    if let Some(subcommand) = parser.subcommand_matches("ldd") {
        settings.command = Command::Ldd;
        get_subcommand_line_settings(subcommand, &mut settings)?;
    }
    if let Some(subcommand) = parser.subcommand_matches("readelf") {
        settings.command = Command::Readelf;
        get_subcommand_line_settings(subcommand, &mut settings)?;
    }

    if parser.is_present("show candidates") {
        settings.show_candidates = true;
    }
    if parser.is_present("output json") {
        settings.output = Output::JSON;
    }
    if parser.is_present("quite") {
        settings.quite = true;
    }
    if parser.is_present("group by file") {
        settings.group_by_file = true;
    }
    if parser.is_present("group by library") {
        settings.group_by_library = true;
    }
    if parser.is_present("group by containing package") {
        settings.group_by_containing_package = true;
    }

    // by default (if not specified otherwise) only files
    // and libraries are printed. Packages are printed only
    // if specified or by default if `show_candidates` is set
    if !settings.group_by_file
        && !settings.group_by_library
        && !settings.group_by_containing_package
    {
        settings.group_by_file = true;
        settings.group_by_library = true;
        settings.group_by_containing_package = settings.show_candidates;
    }
    Ok(settings)
}

fn get_subcommand_line_settings<'a>(
    parser: &ArgMatches<'_>,
    settings: &mut CommandLineSettings,
) -> Result<(), Error<'a>> {
    if let Some(packages) = parser.values_of_lossy("packages") {
        settings.packages = packages;
        settings.packages.sort();
    }
    if parser.is_present("all packages") {
        settings.all_packages = true;
    }
    if let Some(ignore_libraries) = parser.values_of_lossy("ignore libraries") {
        settings.ignore_libraries = ignore_libraries;
        if !settings.quite {
            print!("Ignoring Libraries: ");
            for (index, package) in settings.ignore_libraries.iter().enumerate() {
                if index != 0 {
                    print!(", ");
                }
                print!("{}", package);
            }
            println!();
        }
    }
    if let Some(ignore_libraries_regex) = parser.values_of_lossy("ignore libraries via regex") {
        print!("Ignoring Libraries via Regex: ");
        for (index, package) in ignore_libraries_regex.iter().enumerate() {
            if index != 0 {
                print!(", ");
            }
            print!("{}", package);
        }
        println!();
        settings.ignore_libraries_regex = Some(RegexSet::new(ignore_libraries_regex)?);
    }
    Ok(())
}

#[allow(clippy::too_many_lines)]
fn setup_command_line_parser<'a>() -> ArgMatches<'a> {
    app_from_crate!()
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("ldd")
                .about("Checks packages using ldd")
                .arg(
                    Arg::with_name("packages")
                        .multiple(true)
                        .use_delimiter(true)
                        .number_of_values(1)
                        .help("List of packages to check (eg package1,package2)")
                        .required_unless("all_packages")
                        .conflicts_with("all_packages"),
                )
                .arg(
                    Arg::with_name("all packages")
                        .short("a")
                        .long("all_packages")
                        .help("Checks all installed packages marked as local")
                        .conflicts_with("packages"),
                )
                .arg(
                    Arg::with_name("ignore libraries")
                        .short("i")
                        .long("ignore_libs")
                        .multiple(true)
                        .use_delimiter(true)
                        .number_of_values(1)
                        .help("List of libraries to ignore (eg lib1,lib2)"),
                )
                .arg(
                    Arg::with_name("ignore libraries via regex")
                        .short("r")
                        .long("ignore_libs_regex")
                        .multiple(true)
                        .use_delimiter(true)
                        .number_of_values(1)
                        .help("List of libraries to ignore (eg lib1,lib2) via regex")
                        .long_help(
                            "
List of libraries to ignore (eg lib1,lib2) via regex.
More information about how to define regex at
https://docs.rs/regex/#syntax
                        ",
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name("readelf")
                .about("Checks packages using readelf")
                .arg(
                    Arg::with_name("packages")
                        .multiple(true)
                        .use_delimiter(true)
                        .number_of_values(1)
                        .help("List of packages to check (eg package1,package2)")
                        .required_unless("all_packages")
                        .conflicts_with("all_packages"),
                )
                .arg(
                    Arg::with_name("all packages")
                        .short("a")
                        .long("all_packages")
                        .help("Checks all installed packages marked as local")
                        .conflicts_with("packages"),
                )
                .arg(
                    Arg::with_name("ignore libraries")
                        .short("i")
                        .long("ignore_libs")
                        .multiple(true)
                        .use_delimiter(true)
                        .number_of_values(1)
                        .help("List of libraries to ignore (eg lib1,lib2)"),
                )
                .arg(
                    Arg::with_name("ignore libraries via regex")
                        .short("r")
                        .long("ignore_libs_regex")
                        .multiple(true)
                        .use_delimiter(true)
                        .number_of_values(1)
                        .help("List of libraries to ignore (eg lib1,lib2) via regex")
                        .long_help(
                            "
List of libraries to ignore (eg lib1,lib2) via regex.
More information about how to define regex at
https://docs.rs/regex/#syntax
                        ",
                        ),
                ),
        )
        .arg(
            Arg::with_name("show candidates")
                .short("c")
                .long("show_candidates")
                .help("Prints a list of packages containing the missing library")
                .long_help(
                    "Prints a list of packages containing the missing library.
The listed packages may or may not add the library to the
system path. Therefore just because a package is listed
doesn't mean it will satisfy the library requirement.
Requires pkgfile",
                ),
        )
        .arg(
            Arg::with_name("output json")
                .short("j")
                .long("output_json")
                .help("Uses json for the list of missing libraries"),
        )
        .arg(
            // TODO: Replace with verbose
            Arg::with_name("quite")
                .short("q")
                .long("quite")
                .visible_alias("s")
                .visible_alias("silent")
                .help("Hides all messages"),
        )
        .arg(
            Arg::with_name("group by file")
                .long("group_by_file")
                .help("groups output by files missing libraries"),
        )
        .arg(
            Arg::with_name("group by library")
                .long("group_by_library")
                .help("groups output by libraries required in files"),
        )
        .arg(
            Arg::with_name("group by containing package")
                .long("group_by_containing_package")
                .help("groups output by packages containing libraries"),
        )
        .get_matches()
}
