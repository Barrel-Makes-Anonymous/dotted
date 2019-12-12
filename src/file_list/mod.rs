use std::path::PathBuf;
use std::ffi::OsStr;

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
            path_list.push(PathBuf::from(path));
        }
        FileList {
            files:path_list
        }
    }
    // return a Vec containing the names of the files
    // at the end of the elements in the 'files' field
    pub fn file_names(&self) -> Vec<String> {
        let mut file_names:Vec<String> = vec!();
        for file in self.files.iter() {
            let file_name = match file.file_name() {
                Some(os_str) => {
                    match os_str.to_str() {
                        Some(str_ref) => {
                            String::from(str_ref)
                        },
                        None => panic!("couldn't change OsStr to &str \
                                       for file: {}", file.display()),
                    }
                },
                None => panic!("file_name() failed for {}", 
                               file.display()),
            };
            file_names.push(file_name);
        }
        file_names
    }
}
