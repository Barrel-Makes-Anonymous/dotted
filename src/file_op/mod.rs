use crate::package::Package;
use std::io;
use std::fs;
use std::path::PathBuf;
use std::os::unix;

pub fn copy_files(from_paths:&Vec<PathBuf>, to_paths:&Vec<PathBuf>) 
    -> Vec<bool> {
    let mut successful:Vec<bool> = vec!();
    for (from_path, to_path) 
    in from_paths.into_iter().zip(to_paths.into_iter()) {
        successful.push(copy_file(from_path, to_path, false))
    }
    successful
}
pub fn copy_file(from_path:&PathBuf, to_path:&PathBuf, auto_confirm:bool) 
-> bool {
    if from_path == to_path {
        eprintln!("Will not copy `{}` onto itself.", from_path.display());
        return false;
    } else if !from_path.exists() {
        eprintln!("Cannot copy `{}` because it does not exist", 
            from_path.display());
        return false;
    }
    if to_path.exists() {
        if !auto_confirm {
            let prompt = format!("`{}` already exists. Overwrite it with `{}`? [y/N]",
                to_path.display(), from_path.display());
            if !prompt_user(prompt, true) {
                return false;
            }
        }
    } else {
        create_dir_for(to_path);
    }
    if from_path.is_dir() {
        remove_file(to_path);
        return copy_dir(from_path, to_path);
    } else {
        match fs::copy(from_path, to_path) {
            Ok(_b) => true,
            Err(e) => {
                eprintln!("Error copying file: {}", e);
                return false;
            }
        }
    }
}
fn copy_dir(from_path:&PathBuf, to_path:&PathBuf) 
-> bool {
    let successfully_created_dir = create_dir(to_path);
    if !successfully_created_dir {
        println!("Could not create dir `{}`", to_path.display());
        return false;
    }
    let entries = match from_path.read_dir() {
        Ok(entries) => entries,
        Err(e) => {
            println!("Error: {}", e);
            return false;
        }
    };
    for entry in entries {
        if let Ok(entry) = entry {
            let entry = entry.path();
            let entry_name = match entry.file_name() {
                Some(name) => {
                    match name.to_str() {
                        Some(name_str) => name_str,
                        None => break
                    }
                },
                None => break
            };
            copy_file(&entry, &to_path.join(entry_name), true);
        }
    }
    true 
}
pub fn remove_files(paths:&Vec<PathBuf>) {
    for path in paths.into_iter() {
        if let Some(package) = Package::find_at(path.to_path_buf()) {
            package.disable();
        }
        remove_file(path);
    }
}
pub fn remove_file(path:&PathBuf) {
    if path.exists() {
        match fs::remove_file(path) {
            Ok(()) => {},
            Err(_e) => {
                match fs::remove_dir_all(path) {
                    Ok(()) => {},
                    Err(e) => eprintln!("Error removing `{}`: {}", 
                        path.display(), e)
                }
            }
        }
    }
}
pub fn move_files(from_paths:&Vec<PathBuf>, to_paths:&Vec<PathBuf>) 
    -> Vec<bool> {
    let successful = copy_files(from_paths, to_paths);
    let remove_paths:Vec<PathBuf> = from_paths
        .into_iter()
        .zip(successful.iter())
        .filter(|(from_path, success)| **success)
        .map(|(from_path, success)| from_path.to_path_buf())
        .collect();
    remove_files(&remove_paths);
    successful
}
pub fn move_file(from_path:&PathBuf, to_path:&PathBuf) 
-> bool {
    let success = copy_file(from_path, to_path, false);
    if success {
        remove_file(from_path);
    }
    success
}
pub fn symlink_files(from_paths:&Vec<PathBuf>, to_paths:&Vec<PathBuf>) {
    for (from_path, to_path) 
    in from_paths.into_iter().zip(to_paths.into_iter()) {
        symlink_file(from_path, to_path);
    }
}
pub fn symlink_file(from_path:&PathBuf, to_path:&PathBuf) -> bool {
    if to_path.exists() {
        let prompt = format!("`{}` already exists. Overwrite it with `{}`? [y/N]",
            to_path.display(), from_path.display());
        if !prompt_user(prompt, true) {
            return false;
        } else {
            remove_files(&vec![to_path.to_path_buf()]);
        }
    } else {
        create_dir_for(to_path);
    }
    match unix::fs::symlink(from_path, to_path) {
        Ok(()) => return true,
        Err(e) => eprintln!("Error creating symlink: {}", e)
    }
    false
}
fn prompt_user(prompt:String, default_no:bool) -> bool {
    println!("{}", prompt);
    let (letter, word) = if default_no {
        ("y", "yes")
    } else {
        ("n", "no")
    };
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read from stdin");
    let input = input.to_ascii_lowercase();
    let input = input.trim();
    if input == letter || input == word {
        return default_no;
    }
    println!("Skipping operation.");
    !default_no
}
pub fn create_dir_for(path:&PathBuf) {
    let parent = path_parent(path);
    create_dir(&parent);
}
fn create_dir(path:&PathBuf) -> bool {
    let mut created_dir = true;
    if !path.exists() {
        match fs::create_dir_all(path) {
            Ok(()) => {},
            Err(e) => {
                eprintln!("Error creating directory: {}", e);
                created_dir = false;
            }
        }
    }
    created_dir
}
pub fn path_parent(path:&PathBuf) -> PathBuf {
    match path.parent() {
        Some(parent) => parent.to_path_buf(),
        None => path.to_path_buf()
    }
}
