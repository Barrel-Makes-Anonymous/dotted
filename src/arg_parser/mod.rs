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
// get the parameters between options
fn get_parameters<I>(arg_iter: I) -> Vec<String> 
where
    I:IntoIterator<Item = String>
{
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
fn get_options<I>(arg_iter: I) -> Vec<String>
where
    I:IntoIterator<Item=String>
{
    let mut options:Vec<String> = vec!();
    for option in arg_iter {
        if option.starts_with('-') {
            let mut option_chars = option.chars();
            option_chars.next();
            let option_string = String::from(option_chars.as_str());
            if !option_string.is_empty() {
                options.push(option_string);
            }
        }
    }
    options
}
