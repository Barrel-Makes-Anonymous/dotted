mod package;
mod path_list;
mod file_op;
mod arg_parser;

use package::Package;

fn main() {
   test(); 
}

fn test() {
    let test_package = Package::new("test_pack".to_string());
    let add_files = vec!["/home/dexter/test".to_string()];
    let add_names = vec!["test".to_string()];
    let at_files = vec!["fhsakjdfhsahdf".to_string()];
    //test_package.move_files(add_files, at_files);
    //test_package.add_files(add_files);
    //test_package.enable();
    test_package.remove_files(add_names);
    //test_package.package_info.modify_entries(add_files, at_files);
    //test_package.enable();
}
