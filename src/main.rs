//! A program for checking ArchLinux packages for missing libraries.
//!
//! If a package is missing a library it may mean, that is necessary to rebuild that given package.
//! This binary checks every elf file in a package using either ldd or readelf and reports missing
//! libraries.

// enable additional rustc warnings
#![warn(anonymous_parameters, missing_debug_implementations, missing_docs, trivial_casts,
        trivial_numeric_casts, unsafe_code, unstable_features, unused_extern_crates,
        unused_import_braces, unused_qualifications, variant_size_differences)]
// enable additional clippy warnings
#![cfg_attr(feature = "cargo-clippy", warn(int_plus_one))]
#![cfg_attr(feature = "cargo-clippy", warn(shadow_reuse, shadow_same, shadow_unrelated))]
#![cfg_attr(feature = "cargo-clippy", warn(mut_mut))]
#![cfg_attr(feature = "cargo-clippy", warn(nonminimal_bool))]
#![cfg_attr(feature = "cargo-clippy", warn(pub_enum_variant_names))]
#![cfg_attr(feature = "cargo-clippy", warn(range_plus_one))]
#![cfg_attr(feature = "cargo-clippy", warn(string_add, string_add_assign))]
#![cfg_attr(feature = "cargo-clippy", warn(stutter))]
#![cfg_attr(feature = "cargo-clippy", warn(result_unwrap_used))]

#[macro_use]
extern crate clap;
extern crate json;
extern crate rayon;

mod cli;
mod cmd;
mod data;
mod process;
mod output;

use cli::Command;
use data::Error;
use std::process::exit;

fn main() {
    let mut settings = cli::get_command_line_settings();
    handle_error(cmd::check_required_programs(&settings), 2);

    // TODO: Implement readelf and remove following lines
    match settings.command {
        Command::Readelf => {
            println!("readelf is currently not supported but will be added shortly");
            exit(10);
        }
        _ => {}
    }

    if settings.all_packages {
        handle_error(cmd::get_all_packages(&mut settings), 3);
    }
    // TODO: Replace with verbose
    // TODO: Print more information with verbose like the file types which are checked
    if !settings.quite {
        print!("Checking Packages: ");
        for (index, package) in settings.packages.iter().enumerate() {
            if index != 0 {
                print!(", ");
            }
            print!("{}", package);
        }
        println!("");

        print!("Ignoring Libraries: ");
        for (index, package) in settings.ignore_libraries.iter().enumerate() {
            if index != 0 {
                print!(", ");
            }
            print!("{}", package);
        }
        println!("");
    }

    let packages = handle_error(process::verify_packages(&settings), 4);
    output::print_packages(&packages, &settings);
    match packages.iter().any(|package| {
        !package.file_dependencies.is_empty()
    }) {
        true => exit(1),
        false => exit(0)
    }
}

/// Error Handling for the main method. Takes a result and either
/// prints the error message or returns the value.
///
/// # Arguments
///
/// * `result` - Result to process
/// * `error_code` - Error Code to exit with
fn handle_error<T>(result: Result<T, Error>, error_code: i32) -> T {
    match result {
        Err(e) => {
            println!("{}", e);
            exit(error_code);
        }
        Ok(element) => return element,
    }
}
