use std::env;
use std::collections::VecDeque;


pub(crate) fn pwd_builtin() -> String {
    format!("{}\n", env::current_dir().unwrap().to_string_lossy())
}

pub(crate) fn cd_builtin(args: &mut VecDeque<&str>) -> String {
    if let Some(dir) = args.pop_front() {
        match env::set_current_dir(dir) {
            Ok(_) => "".to_string(),
            Err(_) => format!("cd: {}: No such file or directory\n", dir)
        }
    } else {
        "cd: missing argument\n".to_string()
    }
}
