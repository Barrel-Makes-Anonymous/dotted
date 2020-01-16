mod package;
mod path_list;
mod file_op;
mod arg_parser;

use package::Package;

fn main() {
    let (options, params) = arg_parser::get_args();
    if options.len() >= 1 {
        match options[0].as_str() {
            "a" | "add-files" => {
                add_files(options, params);
            },
            "r" | "remove-files" => {
            },
            "m" | "move-files" => {
            },
            "e" | "es" | "enable-symlink" => {
            },
            "ec" | "enable-copy" => {
            },
            "E" | "disable" => {
            },
            "R" | "remove-packages" => {
            },
            "help" => {
            },
            _ => eprintln!("Unrecognized option `{}`", options[0])
        }
    } else {
        help_prompt();
    }
}
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
        let package = Package::new(package_name);
        package.add_files_at(sources.clone(), dests.clone());
    }
}
fn remove_files(options:Vec<String>, params:Vec<Vec<String>>) {
}
fn move_files(options:Vec<String>, params:Vec<Vec<String>>) {
}
fn enable_packages(params:Vec<Vec<String>>) {
}
fn disable_packages(params:Vec<Vec<String>>) {
}
fn remove_packages(params:Vec<Vec<String>>) {
}

fn help_prompt() {
    eprintln!("Try `dotted --help` for more information`");
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
