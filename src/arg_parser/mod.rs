use std::env;
use std::vec::IntoIter;

/* Collect the arguments passed to the program into a vector
(excluding the first which is the name of the executable) */
fn arg_vec() -> Option<Vec<String>> {
    let args:Vec<String> = env::args().collect();
    if args.len() > 1 {
        return Some(args[1..].to_vec());
    }
    None
}
// return an iterator over the args passed
fn arg_iter() -> Option<IntoIter<String>> {
    match arg_vec() {
        Some(vec) => Some(vec.into_iter()),
        None => None
    }
}
// get the parameters passed for an option
fn parameters(arg_iter: &mut IntoIter<String>) -> Vec<String> {
    let mut parameters:Vec<String> = vec!();
    for param in arg_iter {
        if param.starts_with('-') {
            break;
        } else {
            parameters.push(param);
        }
    }
    parameters
}
// get the options in the order they were passed
fn options() -> Vec<String> {
    let mut options:Vec<String> = vec!();
    match arg_iter() {
        Some(args) => {
            for option in args {
                if option.starts_with('-') {
                    let mut option_chars = option.chars();
                    option_chars.next();
                    let option_string = String::from(option_chars.as_str());
                    if !option_string.is_empty() {
                        options.push(option_string);
                    }
                }
            }
        },
        None => {}
    }
    options
}
