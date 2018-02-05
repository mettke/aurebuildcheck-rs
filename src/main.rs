// enable additional rustc warnings
#![warn(trivial_casts, trivial_numeric_casts, unsafe_code)]
// enable additional clippy warnings
#![cfg_attr(feature = "cargo-clippy", warn(int_plus_one))]
#![cfg_attr(feature = "cargo-clippy", warn(shadow_reuse, shadow_same, shadow_unrelated))]
#![cfg_attr(feature = "cargo-clippy", warn(mut_mut))]
#![cfg_attr(feature = "cargo-clippy", warn(nonminimal_bool))]
#![cfg_attr(feature = "cargo-clippy", warn(pub_enum_variant_names))]
#![cfg_attr(feature = "cargo-clippy", warn(range_plus_one))]
#![cfg_attr(feature = "cargo-clippy", warn(string_add, string_add_assign))]
#![cfg_attr(feature = "cargo-clippy", warn(stutter))]
//#![cfg_attr(feature = "cargo-clippy", warn(result_unwrap_used))]

extern crate rayon;

use std::process::Command;
use rayon::prelude::*;
use std::path::*;

fn check_binary(filename: &str) {
    let mut print_string = String::new();

    match Command::new("ldd").arg(&filename).output() {
        Ok(out) => {
            let output = String::from_utf8_lossy(&out.stdout);
            let output = output.into_owned();
            let mut first = true;
            for line in output.lines() {
                if line.ends_with("=> not found") {
                    if first {
                        print_string.push_str(&format!("\n\tbinary: {}\n", &filename));
                    }
                    print_string.push_str(&format!(
                        "\t\t is missing: {}\n",
                        line.replace("=> not found", "").trim()
                    ));
                    first = false;
                }
            }
        }
        Err(e) => panic!("ERROR '{}'", e),
    }
    if print_string.len() > 1 {
        println!("{}", print_string.trim());
    }
}

fn get_packages() -> Vec<String> {
    let mut packages = Vec::new();
    match Command::new("rpm").arg("-q").arg("-a").output() {
        Ok(out) => {
            let output = String::from_utf8_lossy(&out.stdout);
            let output = output.into_owned();
            for package in output.lines() {
                packages.push(package.into());
            }
        }
        Err(e) => panic!("ERROR '{}'", e),
    }
    packages
}

fn get_files(package: &str) -> Vec<String> {
    let mut files = Vec::new();
    match Command::new("rpm")
        .arg("-q")
        .arg("-l")
        .arg(&package)
        .output()
    {
        Ok(out) => {
            let output = String::from_utf8_lossy(&out.stdout);
            let output = output.into_owned();
            for package in output.lines() {
                files.push(package.into());
            }
        }
        Err(e) => panic!("ERROR '{}'", e),
    }
    files
}

fn file_might_be_binary(file: &str) -> bool {
    let path = PathBuf::from(file);
    if !path.is_file() {
        return false;
    }

    let ext = file.split('.').last().unwrap();

    match ext {
        "a" | "png" | "la" | "ttf" | "gz" | "html" | "css" | "h" | "c" | "cxx" | "xml" | "rgb"
        | "gif" | "wav" | "ogg" | "ogv" | "avi" | "opus" | "mp3" | "po" | "txt" | "jpg"
        | "jpeg" | "bmp" | "xcf" | "mo" | "rb" | "py" | "lua" | "config" | "cfg" | "svg"
        | "desktop" | "conf" | "pdf" | "xz" => false,
        "" | "so" | _ => true,
    }
}

fn is_elf(file: &str) -> bool {
    // check if file is elf via "file"
    let mut file_output: Vec<String> = Vec::new();
    match Command::new("file").arg(&file).output() {
        Ok(out) => {
            let output = String::from_utf8_lossy(&out.stdout);
            let output = output.into_owned();
            for line in output.split(' ') {
                file_output.push(line.into());
            }
        }
        Err(e) => panic!("ERROR '{}'", e),
    }

    file_output.len() > 2 && file_output[1] == "ELF" // ret bool
}

fn check_file(file: &str) {
    if !file_might_be_binary(file) || !is_elf(file) {
        return;
    }
    check_binary(file);
}

fn main() {
    let mut list_of_packages = get_packages();
    list_of_packages.sort();
    for pkg in list_of_packages {
        println!("Checking package: {}", pkg);
        let files = get_files(&pkg);
        files.par_iter().for_each(|file| check_file(file));
    }
}
