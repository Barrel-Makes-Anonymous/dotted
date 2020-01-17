use std::path::PathBuf;
use std::ffi::OsStr;
use std::env;

const MAIN_SEPARATOR:char = std::path::MAIN_SEPARATOR;
// trait to make PathBuf and String interchangable
pub trait PathConvert {
    fn name(&self) -> String;
    fn string(&self) -> String;
    fn absolute_string(&self) -> String {
        format!("{}", self.absolute_path().display())
    }
    fn path(&self) -> PathBuf;
    fn absolute_path(&self) -> PathBuf {
        make_absolute(self.as_os_str())
    }
    fn as_os_str(&self) -> &OsStr;
}
// implementation for String
impl PathConvert for String {
    fn name(&self) -> String {
        let parts:Vec<&str> = self.split(MAIN_SEPARATOR).collect();
        let name = parts
            .last()
            .expect("Path was empty")
            .to_string();
        name
    }
    fn string(&self) -> String {
        self.to_string()
    }
    fn path(&self) -> PathBuf {
        PathBuf::from(&self)
    }
    fn as_os_str(&self) -> &OsStr {
        let os_str:&OsStr = self.as_ref();
        os_str
    }
}
// implementation for PathBuf
impl PathConvert for PathBuf {
    fn name(&self) -> String {
        self.file_name()
            .expect("file_name() call failed")
            .to_str()
            .expect("conversion from &OsStr to &str failed")
            .to_string()
    }
    fn string(&self) -> String {
        format!("{}", self.display())
    }
    fn path(&self) -> PathBuf {
        self.to_path_buf()
    }
    fn as_os_str(&self) -> &OsStr {
        let os_str:&OsStr = self.as_ref();
        os_str
    }
}
// helpers for making absolute paths
fn make_absolute(path:&OsStr) -> PathBuf {
    let path = PathBuf::from(path);
    if !path.is_absolute() {
        return current_dir().join(path);
    }
    path
}
fn current_dir() -> PathBuf {
    if let Ok(dir) = env::current_dir() {
        return dir;
    }
    panic!("Could not access current directory");
}
