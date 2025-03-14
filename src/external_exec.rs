use std::env;
use std::collections::VecDeque;
use std::path::Path;
use crate::StringExt;

pub(crate) fn run_external_executable(command: &str, args: VecDeque<&str>) -> String {
    // Get the PATH environment variable
    let path = match env::var("PATH") {
        Ok(val) => val,
        Err(e) => panic!("could not interpret PATH: {}", e),
    };

    // Split PATH using the appropriate separator for the OS
    let path_separator = if cfg!(windows) { ';' } else { ':' };
    let path_entries: Vec<&str> = path.split(path_separator).collect();
    // println!("PATH entries: {:?}", path_entries);

    // Check if the command exists in any of the PATH entries
    let mut res = format!("{}: not found\n", command);
    for path_entry in path_entries {
        if Path::new(path_entry).join(command).exists() {
            let output = std::process::Command::new(command).args(args).output().expect("Failed to execute process").stdout;
            res = String::from_utf8(output).expect("Failed to parse output");
            res.trim_newline();
            res.push('\n');
            break;
        }
    }
    res
}
