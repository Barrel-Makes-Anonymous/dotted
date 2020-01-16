mod package;
mod path_list;
mod file_op;
mod arg_parser;

use package::Package;
use std::path::PathBuf;
use path_list::PathList;

fn main() {
    let (options, params) = arg_parser::get_args();
    if options.len() >= 1 {
        match options[0].as_str() {
            "a" | "add-files" => {
                add_files(options, params);
            },
            "r" | "remove-files" => {
                remove_files(options, params);
            },
            "m" | "move-files" => {
                move_files(options, params);
            },
            "e" | "es" | "enable-symlink" => {
                enable_packages_symlink(params[0].clone());
            },
            "ec" | "enable-copy" => {
                enable_packages_copy(params[0].clone());
            },
            "E" | "disable" => {
                disable_packages(params[0].clone());
            },
            "R" | "remove-packages" => {
                remove_packages(params[0].clone());
            },
            "i" | "install-packages" => {
                install_packages(params[0].clone());
            },
            "help" => {
            },
            _ => eprintln!("Unrecognized option `{}`", options[0])
        }
    } else {
        help_prompt();
    }
}
// add files to package
fn add_files(options:Vec<String>, params:Vec<Vec<String>>) {
    let sources = params[0].clone();
    let mut dests = vec!();
    let mut packages = vec!();
    if options.len() > 2 && params.len() > 2 {
        if options[1] == "d" || options[1] == "at-destinations" {
            dests = params[1].clone();
        } else {
            invalid_input(options[1].clone());
        }
        if options[2] == "p" || options[2] == "in-packages" {
            packages = params[2].clone();
        } else {
            invalid_input(options[2].clone());
        }
    } else if options.len() > 1 && params.len() > 1 {
        dests = sources.clone();
        if options[1] == "p" || options[1] == "in-packages" {
            packages = params[1].clone();
        } else {
            invalid_input(options[1].clone());
        }
    } else {
        not_enough_input();
    }
    for package_name in packages.into_iter() {
        let path = PathBuf::from(&package_name);
        if path.is_absolute() {
            let package = Package::new_at(path);
            package.add_files_at(sources.clone(), dests.clone());
        } else {
            let package = Package::new(package_name);
            package.add_files_at(sources.clone(), dests.clone());
        }
    }
}
// remove files from package
fn remove_files(options:Vec<String>, params:Vec<Vec<String>>) {
    let remove_files = params[0].clone();
    let mut packages = vec!();
    if options.len() >= 2 && params.len() >= 2 {
        if options[1] == "p" || options[1] == "in-packages" {
            packages = params[1].clone();
        } else {
            invalid_input(options[1].clone());
        }
    } else {
        not_enough_input();
    }
    for package_name in packages.into_iter() {
        let path = PathBuf::from(&package_name);
        if path.is_absolute() {
            if let Some(package) = Package::find_at(path) {
                package.remove_files(remove_files.clone());
            }
        } else {
            if let Some(package) = Package::find(package_name) {
                package.remove_files(remove_files.clone());
            }
        }
    }
}
// move files in package
fn move_files(options:Vec<String>, params:Vec<Vec<String>>) {
    let move_from = params[0].clone();
    let mut move_to = vec!();
    let mut packages = vec!();
    if options.len() >= 3 && params.len() >= 3 {
        if options[1] == "d" || options[1] == "at-destinations" {
            move_to = params[1].clone();
        } else {
            invalid_input(options[1].clone());
        }
        if options[2] == "p" || options[2] == "in-packages" {
            packages = params[2].clone();
        } else {
            invalid_input(options[1].clone());
        }
    } else {
        not_enough_input();
    }
    for package_name in packages.into_iter() {
        let path = PathBuf::from(&package_name);
        if path.is_absolute() {
            if let Some(package) = Package::find_at(path) {
                package.move_files(move_from.clone(), move_to.clone());
            }
        } else {
            if let Some(package) = Package::find(package_name) {
                package.move_files(move_from.clone(), move_to.clone());
            }
        }
    }
}
// enable packages with symlink
fn enable_packages_symlink(params:Vec<String>) {
    for package_name in params.into_iter() {
        let path = PathBuf::from(&package_name);
        if path.is_absolute() {
            if let Some(package) = Package::find_at(path) {
                package.enable_symlink();
            }
        } else {
            if let Some(package) = Package::find(package_name) {
                package.enable_symlink();
            }
        }
    }
}
// enable packages with copy
fn enable_packages_copy(params:Vec<String>) {
    for package_name in params.into_iter() {
        let path = PathBuf::from(&package_name);
        if path.is_absolute() {
            if let Some(package) = Package::find_at(path) {
                package.enable_copy();
            }
        } else {
            if let Some(package) = Package::find(package_name) {
                package.enable_copy();
            }
        }
    }
}
// disable packages
fn disable_packages(params:Vec<String>) {
    for package_name in params.into_iter() {
        let path = PathBuf::from(&package_name);
        if path.is_absolute() {
            if let Some(package) = Package::find_at(path) {
                package.disable();
            }
        } else {
            if let Some(package) = Package::find(package_name) {
                package.disable();
            }
        }
    }
}
// delete packages
fn remove_packages(params:Vec<String>) {
    for package_name in params.into_iter() {
        let path = PathBuf::from(&package_name);
        if path.is_absolute() {
            if let Some(package) = Package::find_at(path) {
                package.disable();
                file_op::remove_file(&package.package_path);
            }
        } else {
            if let Some(package) = Package::find(package_name) {
                package.disable();
                file_op::remove_file(&package.package_path);
            }
        }
    }
}
fn install_packages(params:Vec<String>) {
    let package_list = PathList::from_vec(params, true);
    for path in package_list.file_paths().into_iter() {
        if let Some(package) = Package::find_at(path) {
            package.install();
        }
    }
}

fn help_prompt() {
    eprintln!("Try `dotted --help` for more information");
}
fn unexpected_option(option:String) {
    eprintln!("Unexpected option `{}`", option);
}
fn invalid_input(input:String) {
    unexpected_option(input);
    help_prompt();
    return;
}
fn not_enough_input() {
    eprintln!("Too few arguments passed");
    help_prompt();
    return;
}
