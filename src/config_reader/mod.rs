extern crate dirs;

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::io::{BufRead, BufReader, Lines};

// read settings from config file into HashMap with setting names as keys
fn read_config() -> HashMap<String, String> {
    let mut config_settings = HashMap::new();
    let config_dir = match dirs::config_dir() {
        Some(dir) => dir,
        None => panic!("Couldn't find a config dir")
    };
    let config_path = config_dir.join(PathBuf::from("dotted.conf"));
    if !config_path.exists() {
        if !config_dir.exists() {
            match fs::create_dir_all(config_dir) {
                Ok(()) => {},
                Err(e) => panic!("{}", e)
            }
        }
        match fs::write(config_path, "") {
            Ok(()) => {},
            Err(e) => panic!("{}", e)
        }
    }
    for line in config_file_lines() {
        let line = match line {
            Ok(string) => string,
            // this should never happen
            Err(e) => panic!("{}", e)
        };
        if !line.trim().is_empty() {
            let line_slices:Vec<&str> = line.split('=').collect();
            if line_slices.len() >= 2 {
                let setting_key = String::from(line_slices[0].trim());
                let mut setting_value = String::new();
                for (index, slice) in line_slices[1..]
                    .to_vec()
                    .iter()
                    .enumerate() {
                        // put back all but the first '=' character
                        if index > 0 {
                            setting_value.push('=');
                        }
                        setting_value.push_str(slice);
                }
                setting_value = String::from(setting_value.trim());
                config_settings.insert(setting_key, setting_value);
            }
        }
    }
    config_settings
}
// get an iterator over the lines in the config file
fn config_file_lines() -> Lines<BufReader<File>> {
    let config_path = match dirs::config_dir() {
        Some(dir) => dir,
        None => panic!("Couldn't find a config dir")
    };
    let config_file = match File::open(
        config_path.join(PathBuf::from("dotted.conf"))) {
        Ok(file) => file,
        Err(e) => panic!("{}", e)
    };
    let config_reader = BufReader::new(config_file);
    config_reader.lines()
}   
