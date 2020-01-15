use std::path::PathBuf;
use std::ffi::OsStr;
use std::env;

pub fn convert_vec<O>(input_vec:Vec<O>, make_absolute:bool) -> Vec<PathBuf>
where O:AsRef<OsStr> {
    input_vec
        .into_iter()
        .map(|path|
            if make_absolute {
                make_path_absolute(PathBuf::from(path.as_ref()))
            } else {
                PathBuf::from(path.as_ref())
            }
        )
        .collect()
}

fn make_path_absolute(path:PathBuf) -> PathBuf {
    if path.is_relative() {
        return current_dir().join(path);
    }
    path
}

fn current_dir() -> PathBuf {
    match env::current_dir() {
        Ok(dir) => dir,
        Err(e) => panic!("{}", e),
    }
}
// PathList struct
pub struct PathList {
    paths:Vec<PathBuf>
}
impl PathList {
    pub fn from_vec<O>(input_vec:Vec<O>, make_absolute:bool) -> Self
    where O:AsRef<OsStr> {
        PathList {
            paths:convert_vec(input_vec, make_absolute),
        }
    }
    pub fn file_paths(&self) -> Vec<PathBuf> {
        self.paths.clone()
    }
    pub fn path_strings(&self) -> Vec<String> {
        self.paths
            .iter()
            .map(|path| format!("{}", path.display()))
            .collect()
    }
    pub fn file_names(&self) -> Vec<String> {
        self.paths
            .iter()
            .map(|path| 
                match path.file_name() {
                    Some(name) => {
                        match name.to_str() {
                            Some(string) => String::from(string),
                            None => panic!("Could not convert OsStr to &str")
                        }
                    },
                    None => panic!("Could not get file name as OsStr")
                }
            )
            .collect()
    }
    pub fn symlink_sources(&self) -> Vec<Option<(PathBuf, PathBuf)>> {
        self.paths
            .clone()
            .into_iter()
            .map(|link_dest| 
                match link_dest.read_link() {
                    Ok(link_source) => Some((link_dest, link_source)),
                    Err(_e) => None
                }
            )
            .collect()
    }
}
