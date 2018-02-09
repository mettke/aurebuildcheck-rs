use cli::{CommandLineSettings, Output};
use data::Package;
use json;

pub fn print_packages(packages: &Vec<Package>, settings: &CommandLineSettings) {
    match settings.output {
        Output::Console => print_console(packages, settings),
        Output::JSON => print_json(packages, settings),
    }
}

fn print_console(packages: &Vec<Package>, settings: &CommandLineSettings) {
    for (i, package) in packages.iter().enumerate() {
        if i != 0 {
            println!("");
        }
        println!("========================================");
        println!("Package: {}", package.name);
        println!("========================================");
        if settings.group_by_file {
            package.file_dependencies.iter().for_each(|dependency| {
                println!("\nelf file \"{}\" is missing:", dependency.file_name);
                dependency.library_dependencies.iter().for_each(|library| {
                    println!("\t{}", library);
                })
            });
        }
        if settings.group_by_library {
            package.library_requirements.iter().for_each(|library| {
                println!("\nlibrary \"{}\" is required by:", library.library_name);
                library.files_requiring.iter().for_each(|file| {
                    println!("\t{}", file);
                })
            });
        }
        if settings.group_by_containing_package {
            package
                .packages_containing
                .iter()
                .for_each(|package_entry| {
                    println!(
                        "\nlibrary \"{}\" is packaged in:",
                        package_entry.library_name
                    );
                    package_entry
                        .packages_containing
                        .iter()
                        .for_each(|package| {
                            println!("\t{}", package);
                        })
                });
        }
    }
}

fn print_json(packages: &Vec<Package>, settings: &CommandLineSettings) {
    let mut json_packages = json::JsonValue::new_array();
    for package in packages.iter() {
        let mut json_package = json::JsonValue::new_object();
        json_package["package_name"] = package.name.clone().into();
        if settings.group_by_file {
            json_package["file_dependencies"] = print_json_file_dependencies(package);
        }
        if settings.group_by_library {
            json_package["library_requirements"] = print_json_library_requirements(package);
        }
        if settings.group_by_containing_package {
            json_package["packages_containing"] = print_json_packages_containing(package);
        }
        json_packages.push(json_package).unwrap();
    }
    println!("{}", json_packages.dump());
}

fn print_json_file_dependencies(package: &Package) -> json::JsonValue {
    let mut json_file_dependencies = json::JsonValue::new_array();
    package.file_dependencies.iter().for_each(|dependency| {
        let mut json_file_dependency = json::JsonValue::new_object();
        json_file_dependency["file_name"] = dependency.file_name.clone().into();
        let mut json_file_dependencies_array = json::JsonValue::new_array();
        dependency.library_dependencies.iter().for_each(|library| {
            json_file_dependencies_array.push(library.clone()).unwrap();
        });
        json_file_dependency["library_dependencies"] = json_file_dependencies_array;
        json_file_dependencies.push(json_file_dependency).unwrap();
    });
    json_file_dependencies
}

fn print_json_library_requirements(package: &Package) -> json::JsonValue {
    let mut json_library_requirements = json::JsonValue::new_array();
    package.library_requirements.iter().for_each(|library| {
        let mut json_library_requirement = json::JsonValue::new_object();
        json_library_requirement["library_name"] = library.library_name.clone().into();
        let mut json_library_requirements_array = json::JsonValue::new_array();
        library.files_requiring.iter().for_each(|file| {
            json_library_requirements_array.push(file.clone()).unwrap();
        });
        json_library_requirement["files_requiring"] = json_library_requirements_array;
        json_library_requirements
            .push(json_library_requirement)
            .unwrap();
    });
    json_library_requirements
}

fn print_json_packages_containing(package: &Package) -> json::JsonValue {
    let mut json_packages_containing = json::JsonValue::new_array();
    package
        .packages_containing
        .iter()
        .for_each(|package_entry| {
            let mut json_package_containing = json::JsonValue::new_object();
            json_package_containing["library_name"] = package_entry.library_name.clone().into();
            let mut json_packages_containing_array = json::JsonValue::new_array();
            package_entry
                .packages_containing
                .iter()
                .for_each(|package| {
                    json_packages_containing_array
                        .push(package.clone())
                        .unwrap();
                });
            json_package_containing["packages_containing"] = json_packages_containing_array;
            json_packages_containing
                .push(json_package_containing)
                .unwrap();
        });
    json_packages_containing
}
