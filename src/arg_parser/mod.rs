use std::env;

/* Collect the arguments passed to the program into a vector
(excluding the first which is the name of the executable) */
fn arg_vec() -> Option<Vec<String>> {
    let args:Vec<String> = env::args().collect();
    if args.len() > 1 {
        return Some(args[1..].to_vec());
    }
    None
}
pub fn get_args() -> (Vec<String>, Vec<Vec<String>>) {
    let mut options:Vec<String> = vec!();
    let mut params:Vec<Vec<String>> = vec!();
    let mut param_index:i32 = -1;
    match arg_vec() {
        Some(args) => {
            for mut arg in args.into_iter() {
                if arg.starts_with("-") {
                    if arg.starts_with("--") {
                        options.push(arg.replace("--", ""));
                    } else {
                        options.push(arg.replace("-", ""));
                    }
                    params.push(vec!());
                    param_index += 1;
                } else if param_index != -1 {
                    if arg.ends_with('/') || arg.ends_with('\') {
                        arg.pop();
                    }
                    params[param_index as usize].push(arg);
                }
            }
        },
        None => {}
    }
    (options, params)
}

