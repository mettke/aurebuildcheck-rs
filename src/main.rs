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

use std::fs::*;
use std::process::Command;
use rayon::prelude::*;

fn check_file(path: &DirEntry) {
    let mut print_string = String::new();
    let name_string = path.path();
    print_string.push_str(&format!("checking: {}", &name_string.display()));

    match Command::new("ldd").arg(&name_string).output() {
        Ok(out) => {
            //    println!("git gc error\nstatus: {}", out.status);
            //    println!("stdout:\n {}", String::from_utf8_lossy(&out.stdout));
            //    println!("stderr:\n {}", String::from_utf8_lossy(&out.stderr));
            //if out.status.success() {}
            let output = String::from_utf8_lossy(&out.stdout);
            let output = output.into_owned();
            let mut first = true;
            for line in output.lines() {
                if line.ends_with("=> not found") {
                    if first {
                        print_string.push_str(&format!("\nbinary: {}\n", &name_string.display()));
                    }
                    print_string.push_str(&format!(
                        "\t\t is missing: {}\n",
                        line.replace("=> not found", "").trim()
                    ));
                    first = false;
                }
                //println!("{}", line);
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

fn main() {
    let list_of_packages = get_packages();

    println!("{:?}", list_of_packages);
    for pkg in list_of_packages {
        let mut files = Vec::new();
        files = get_files(&pkg);

        println!("package: {}, files: {:?}", pkg, files);
    }
    //files.par_iter().for_each(|binary| check_file(binary));
}
