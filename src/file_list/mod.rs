use std::path::PathBuf;
use std::path::Path;
use std::ffi::OsStr;
use std::convert::AsRef;

const HOME_PREFIX:&str = "~";

pub struct FileList {
    files:Vec<PathBuf>
}

impl FileList {
    // make new FileList containing empty Vec
    pub fn new() -> Self {
        FileList {
            files:vec!()
        }
    }
    // make a FileList from a Vec of
    // type implementing AsRef<OsStr>
    pub fn from_vec<P>(input_list:Vec<P>) -> Self
    where P:AsRef<OsStr> {
        let mut path_list:Vec<PathBuf> = vec!();
        for path in input_list.iter() {
            let path = path.as_ref();
            path_list.push(PathBuf::from(path));
        }
        FileList {
            files:path_list
        }
    }
    // replace instances of home dir with "~"
    pub fn use_placeholder_home_dir(&mut self) {
        self.replace_prefix(home_dir(), HOME_PREFIX);
    }
    // replace instances of "~" with home dir
    pub fn use_real_home_dir(&mut self) {
        self.replace_prefix(HOME_PREFIX, home_dir());
    }
    // replace the beginning of a path with something else
    fn replace_prefix<A, B>(&mut self, replace:A, with:B)
    where A:AsRef<Path>,
          B:AsRef<Path> {
        let replace = replace.as_ref();
        let with = with.as_ref();
        for(index, file_path) in self.files.clone().iter().enumerate() {
            if file_path.starts_with(&replace) {
                let mut path_components = file_path.components();
                path_components.next();
                if file_name(replace) != HOME_PREFIX {
                    path_components.next();
                }
                let final_path = with.join(path_components);
                self.files[index] = final_path;
            }
        }
    }
    // return a Vec containing the names of the files
    // at the end of the elements in the 'files' field.
    // fails if a given file's name can't be converted to
    // string to ensure returned Vec "lines up"
    pub fn file_names(&self) -> Vec<String> {
        let mut file_names:Vec<String> = vec!();
        for file in self.files.iter() {
            let file_name = file_name(file);
            file_names.push(file_name);
        }
        file_names
    }
}
// return the name of a file and panic if it can't be done
fn file_name<P>(path:P) -> String
where P:AsRef<Path> {
    let path = path.as_ref();
    match path.file_name() {
        Some(os_str) => {
            match os_str.to_str() {
                Some(str_ref) => String::from(str_ref),
                None => panic!("Couldn't convert OsStr to &str")
            }
        },
        None => panic!("file_name() call failed")
    }
}

// helper method to return home directory and panic
// if it can't be accessed for whatever reason
fn home_dir() -> PathBuf {
    match dirs::home_dir() {
        Some(dir) => dir,
        None => panic!("Couldn't access home dir")
    }
}
