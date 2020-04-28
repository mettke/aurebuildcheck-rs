//! A program for checking `ArchLinux` packages for missing libraries.
//!
//! If a package is missing a library it may mean, that is necessary to rebuild that given package.
//! This binary checks every elf file in a package using either ldd or readelf and reports missing
//! libraries.

// enable additional rustc warnings
#![warn(
    absolute_paths_not_starting_with_crate,
    anonymous_parameters,
    box_pointers,
    deprecated_in_future,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    indirect_structural_match,
    keyword_idents,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    missing_doc_code_examples,
    non_ascii_idents,
    private_doc_tests,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]
// enable additional clippy warnings
#![warn(
    clippy::correctness,
    clippy::restriction,
    clippy::style,
    clippy::pedantic,
    clippy::complexity,
    clippy::perf,
    clippy::cargo,
    clippy::nursery
)]
#![allow(
    clippy::implicit_return,
    clippy::missing_docs_in_private_items,
    clippy::result_expect_used,
    clippy::shadow_reuse,
    clippy::option_expect_used,
    clippy::wildcard_enum_match_arm,
    clippy::exit,
    clippy::module_name_repetitions,
    clippy::similar_names,
    clippy::else_if_without_else,
    clippy::multiple_crate_versions,
    clippy::print_stdout
)]

mod cli;
mod cmd;
mod data;
mod output;
mod process;

use crate::{cli::Command, data::Error};
use std::process::exit;

fn main() {
    let mut settings = handle_error(cli::get_command_line_settings(), 2);
    handle_error(cmd::check_required_programs(&settings), 3);

    // TODO: Implement readelf and remove following lines
    if let Command::Readelf = settings.command {
        println!("readelf is currently not supported but will be added shortly");
        exit(10);
    }

    if settings.all_packages {
        handle_error(cmd::get_all_packages(&mut settings), 4);
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
        println!();
    }

    let packages = handle_error(process::verify_packages(&settings), 5);
    output::print_packages(&packages, &settings);
    if packages
        .iter()
        .any(|package| !package.file_dependencies.is_empty())
    {
        exit(1)
    } else {
        exit(0)
    }
}

/// Error Handling for the main method. Takes a result and either
/// prints the error message or returns the value.
///
/// # Arguments
///
/// * `result` - Result to process
/// * `error_code` - Error Code to exit with
fn handle_error<T>(result: Result<T, Error<'_>>, error_code: i32) -> T {
    match result {
        Err(e) => {
            println!("{}", e);
            exit(error_code);
        }
        Ok(element) => element,
    }
}
