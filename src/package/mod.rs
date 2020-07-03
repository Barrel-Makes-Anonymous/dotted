mod info;

use info::PackageInfo;
use std::fs::File;
use crate::{file_op, path_list::PathList};
use std::path::PathBuf;

pub struct Package {
    pub package_path:PathBuf,
    package_info:PackageInfo
}
impl Package {
    pub fn new(name:String) -> Self {
        let package_path = data_local_dir().join("dotted").join(name);
        let info_path = package_path.join(".dotted");
        if !info_path.exists() {
            file_op::create_dir_for(&info_path);
            match File::create(&info_path) {
                Ok(_file) => {},
                Err(e) => panic!("{}", e)
            }
        }
        let package_info = PackageInfo::new(package_path.to_path_buf());
        Package {
            package_path:package_path,
            package_info:package_info
        }
    }
    pub fn find(name:String) -> Option<Self> {
        let package_path = data_local_dir().join("dotted").join(name);
        let info_path = package_path.join(".dotted");
        if info_path.exists() {
            let package_info = PackageInfo::new(package_path.to_path_buf());
            return Some(Package {
                package_path:package_path,
                package_info:package_info
            });
        }
        None
    }
    pub fn new_at(package_path:PathBuf) -> Self {
        let info_path = package_path.join(".dotted");
        if !info_path.exists() {
            file_op::create_dir_for(&info_path);
            match File::create(&info_path) {
                Ok(_file) => {},
                Err(e) => panic!("{}", e)
            }
        }
        let package_info = PackageInfo::new(package_path.to_path_buf());
        Package {
            package_path:package_path,
            package_info:package_info
        }
    }
    pub fn find_at(package_path:PathBuf) -> Option<Self> {
        let info_path = package_path.join(".dotted");
        if info_path.exists() {
            let package_info = PackageInfo::new(package_path.to_path_buf());
            return Some(Package {
                package_path:package_path,
                package_info:package_info
            });
        }
        None
    }
    pub fn install(&self) {
        self.disable();
        let name = match self.package_path.file_name() {
            Some(os_str) => {
                match os_str.to_str() {
                    Some(name_str) => name_str,
                    None => panic!("Could not convert &OsStr to &str")
                }
            },
            None => panic!("file_name() call failed")
        };
        file_op::move_file(&self.package_path, &data_local_dir().join("dotted").join(name));
    }
    pub fn add_files_at(
        &self, 
        source_files:Vec<String>, 
        dest_files:Vec<String>) {
        let source_list = PathList::from_vec(source_files, true);
        let dest_list = PathList::from_vec(dest_files, true);
        let copy_from = source_list.file_paths();
        let copy_to:Vec<PathBuf> = source_list.file_names()
            .into_iter()
            .map(|name| self.package_path.join(name))
            .collect();
        let mut successful_names:Vec<String> = vec!();
        let mut successful_dests:Vec<String> = vec!();
        for ((from, to), (name, dest)) in
            copy_from.iter().zip(copy_to.iter()).zip(
            source_list.file_names().into_iter().zip(
            dest_list.path_strings().into_iter())) {
            if file_op::copy_file(from, to, false) {
                successful_names.push(name);
                successful_dests.push(dest);
            }
        }
        let entries = 
            self.package_info.search_entries(successful_names.clone());
        let mut modify_names:Vec<String> = vec!();
        let mut modify_dests:Vec<String> = vec!();
        let mut add_names:Vec<String> = vec!();
        let mut add_dests:Vec<String> = vec!();
        for (entry, (name, dest)) in
            entries.iter().zip(
            successful_names.into_iter().zip(
            successful_dests.into_iter())) {
            if entry.is_some() {
                modify_names.push(name);
                modify_dests.push(dest);
            } else {
                add_names.push(name);
                add_dests.push(dest);
            }
        }
        self.move_files(modify_names, modify_dests);
        self.package_info.add_entries(add_names, add_dests);
    }
    pub fn remove_files(&self, files:Vec<String>) {
        let remove_entries = self.package_info.search_entries(files);
        let remove_entries = remove_entries
            .into_iter()
            .filter(|entry| entry.is_some())
            .map(|entry| entry.unwrap());
        let remove_dests:Vec<PathBuf> = remove_entries
            .clone()
            .map(|(_source, dest)| dest)
            .collect();
        let remove_sources:Vec<PathBuf> = remove_entries
            .map(|(source, _dest)| source.to_path_buf())
            .collect();
        let remove_source_names = 
            PathList::from_vec(remove_sources.clone(), false).file_names();
        let remove_dests = PathList::from_vec(remove_dests, false)
            .symlink_sources()
            .into_iter()
            .filter(|link| link.is_some())
            .map(|link| link.unwrap())
            .filter(|(_link_dest, link_source)| 
                file_op::path_parent(&link_source) == self.package_path)
            .map(|(link_dest, _link_source)| link_dest)
            .collect();
        file_op::remove_files(&remove_sources);
        file_op::remove_files(&remove_dests);
        self.package_info.remove_entries(remove_source_names);
    }
    pub fn move_files (
        &self,
        change_files:Vec<String>,
        to_files:Vec<String>) {
        let change_list = PathList::from_vec(change_files, false);
        let to_list = PathList::from_vec(to_files, true);
        let entries = 
            self.package_info.search_entries(change_list.path_strings());
        let change_entries = entries.into_iter().zip(
            change_list.file_names().into_iter().zip(
            to_list.path_strings().into_iter()))
            .filter(|(entry, (_source, _dest))| entry.is_some())
            .map(|(entry, (source, dest))| (entry.unwrap(), (source, dest)));
        let mut change_names:Vec<String> = vec!();
        let mut change_paths:Vec<String> = vec!();
        for ((entry_source, entry_dest), (source, dest)) in change_entries {
            match entry_dest.read_link() {
                Ok(link_source) => {
                    if file_op::path_parent(&link_source) == self.package_path {
                        if entry_dest != PathBuf::from(&dest) 
                        && file_op::symlink_file(
                            &entry_source, &PathBuf::from(&dest)) {
                            change_names.push(source);
                            change_paths.push(dest);
                            file_op::remove_file(&entry_dest);
                        }
                        continue;
                    }
                },
                Err(_e) => {}
            }
            change_names.push(source);
            change_paths.push(dest);
        }
        self.package_info.modify_entries(change_names, change_paths);
    }
    pub fn enable_symlink(&self) {
        let (source_list, dest_list) = self.package_info.read_info();
        let source_paths = source_list.file_paths();
        let dest_paths = dest_list.file_paths();
        file_op::symlink_files(&source_paths, &dest_paths);
    }
    pub fn enable_copy(&self) {
        let (source_list, dest_list) = self.package_info.read_info();
        let source_paths = source_list.file_paths();
        let dest_paths = dest_list.file_paths();
        file_op::copy_files(&source_paths, &dest_paths);
    }
    pub fn disable(&self) {
        let (_source_list, dest_list) = self.package_info.read_info();
        for link in dest_list.symlink_sources() {
            match link {
                Some((link_dest, link_source)) => {
                    if file_op::path_parent(&link_source) == self.package_path {
                        if let Some(package) = 
                            Package::find_at(link_dest.clone()) {
                            package.disable();
                        }
                        file_op::remove_file(&link_dest);
                    }
                },
                None => {}
            }
        }
    }
}
fn data_local_dir() -> PathBuf {
    match dirs::data_local_dir() {
        Some(dir) => dir,
        None => panic!("Could not access local data dir")
    }
}
