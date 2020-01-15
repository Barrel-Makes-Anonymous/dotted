use crate::path_list::PathList;
use std::path::PathBuf;
use std::fs::{self, File};
use std::io::{Lines, BufReader, BufRead};

const HOME_PREFIX:&str = "%HOME_DIR%";

fn home_dir() -> PathBuf {
    match dirs::home_dir() {
        Some(dir) => dir,
        None => panic!("Could not access home dir")
    }
}
// get home dir as a string for easy file writing
fn home_dir_string() -> String {
    format!("{}", home_dir().display())
}
// make an entry for a Package's info file from a file name and
// a destination
fn make_entry(name:&String, dest:&String) -> String {
    format!("{} -> {}\n", name, dest)
}

fn entry_parts(entry:&String) -> Option<(String, String)> {
    let parts:Vec<&str> = entry.split(" -> ").collect();
    if parts.len() == 2 {
        return Some((parts[0].to_string(), parts[1].to_string()));
    }
    None
}

fn append_newline(string:String) -> String {
    format!("{}\n", string)
}

fn should_parse(line:&String) -> bool {
    !line.trim().starts_with("#") && line.trim().len() != 0
}

// PackageInfo struct
pub struct PackageInfo {
    package_path:PathBuf,
    info_path:PathBuf,
}
impl PackageInfo {
    pub fn new(package_path:PathBuf) -> Self {
        let info_path = package_path.join(".dotted");
        PackageInfo {
            package_path:package_path,
            info_path:info_path
        }
    }
    pub fn add_entries(&self,
        mut names:Vec<String>,
        mut dests:Vec<String>) {
        let mut output_string = String::new();
        let lines = self.read_info_file_lines();
        for line in lines {
            let line = line.unwrap();
            output_string.push_str(&append_newline(line));
        }
        for (name, dest) in 
            names.iter().zip(dests.iter()) {
            output_string.push_str(&make_entry(name, dest));
        }
        self.write_to_info_file(output_string);
    }
    pub fn remove_entries(&self, mut remove:Vec<String>) {
        let mut output_string = String::new();
        let lines = self.read_info_file_lines();
        for line in lines {
            let line = line.unwrap();
            if should_parse(&line) {
                match entry_parts(&line) {
                    Some((name, dest)) => {
                        let mut found_match = false;
                        for i in 0..remove.len() {
                            if name == remove[i] || dest == remove[i] {
                                remove.remove(i);
                                found_match = true;
                                break;
                            }
                        }
                        if found_match {
                            continue;
                        }
                    },
                    None => {}
                }
            }
            output_string.push_str(&append_newline(line));
        }
        self.write_to_info_file(output_string);
    }
    pub fn modify_entries(&self, 
        mut names:Vec<String>, 
        mut new_dests:Vec<String>) 
        -> (Vec<String>, Vec<String>) {
        let mut output_string = String::new();
        let lines = self.read_info_file_lines();
        for line in lines {
            let line = line.unwrap();
            if should_parse(&line) {
                match entry_parts(&line) {
                    Some((name, dest)) => {
                        let mut found_match = false;
                        for i in 0..names.len() {
                            if name == names[i] {
                                output_string.push_str(
                                    &make_entry(&name, &new_dests[i]));
                                names.remove(i);
                                new_dests.remove(i);
                                found_match = true;
                                break;
                            }
                        }
                        if found_match {
                            continue;
                        }
                    },
                    None => {}
                }
            }
            output_string.push_str(&append_newline(line));
        }
        self.write_to_info_file(output_string);
        (names, new_dests)
    }
    pub fn search_entries(&self, queries:Vec<String>) 
    -> Vec<Option<(PathBuf, PathBuf)>> {
        let (sources, dests) = self.read_info();
        let mut source_names = sources.file_names();
        let mut dest_strings = dests.path_strings();
        let source_paths = sources.file_paths();
        let dest_paths = dests.file_paths();
        let mut results = vec!();
        for query in queries.into_iter() {
            let mut entry:Option<(PathBuf, PathBuf)> = None;
            for i in 0..source_names.len() {
                if source_names[i] == query
                    || dest_strings[i] == query {
                    source_names.remove(i);
                    dest_strings.remove(i);
                    entry = Some((
                        source_paths[i].to_path_buf(), 
                        dest_paths[i].to_path_buf()));
                }
            }
            results.push(entry);
        }
        results
    }
    pub fn read_info(&self) -> (PathList, PathList) {
        let mut source_list:Vec<PathBuf> = vec!();
        let mut dest_list:Vec<PathBuf> = vec!();
        let info_file_lines = self.read_info_file_lines();
        for (line_number, line) in info_file_lines.enumerate() {
            let line = line.unwrap();
            if should_parse(&line) {
                match entry_parts(&line) {
                    Some((name, dest)) => {
                        source_list.push(self.package_path.join(name));
                        dest_list.push(PathBuf::from(
                            dest.replace(
                                HOME_PREFIX, &home_dir_string())));
                    },
                    None => {}
                }
            }
        }
        let source_list = PathList::from_vec(source_list, false);
        let dest_list = PathList::from_vec(dest_list, false);
        (source_list, dest_list)
    }
    pub fn read_info_file_lines(&self) -> Lines<BufReader<File>> {
        let info_file = match File::open(&self.info_path) {
            Ok(file) => file,
            Err(e) => panic!("{}", e)
        };
        let info_reader = BufReader::new(info_file);
        info_reader.lines()
    }
    pub fn write_to_info_file(&self, contents:String) {
        let contents = contents.replace(&home_dir_string(), HOME_PREFIX);
        match fs::write(&self.info_path, contents) {
            Ok(()) => {},
            Err(e) => panic!("{}", e)
        }
    }
}
