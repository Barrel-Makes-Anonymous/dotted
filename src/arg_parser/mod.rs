use std::env;
use std::vec::IntoIter;

// collect arguments into vec and drop the first one
fn arg_vec() -> Option<Vec<String>> {
    let args:Vec<String> = env::args().collect();
    if args.len() > 1 {
        return Some(args[1..].to_vec());
    }
    None
}

fn arg_iter() -> Option<IntoIter<String>> {
    match arg_vec() {
        Some(vec) => Some(vec.into_iter()),
        None => None
    }
}

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
