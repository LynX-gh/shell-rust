use std::env;
use std::path::Path;
use std::collections::VecDeque;


pub(crate) fn type_builtin(inputs: &mut VecDeque<&str>) -> String {
    let command = if let Some(command) = inputs.pop_front() { command } else { "" };
    match command {
        "echo" | "exit" | "type" | "pwd" => format!("{} is a shell builtin\n", command),
        _ => {
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
                    res = format!("{} is {}\n", command, Path::new(path_entry).join(command).to_string_lossy());
                    break;
                }
            }
            res
        }
    }
}
