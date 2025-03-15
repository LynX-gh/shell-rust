use std::env;
use std::collections::VecDeque;
use std::path::PathBuf;


pub(crate) fn pwd_builtin() -> String {
    format!("{}\n", env::current_dir().unwrap().to_string_lossy())
}

pub(crate) fn cd_builtin(args: &mut VecDeque<&str>) -> String {
    if let Some(dir) = args.pop_front() {
        let mut target = PathBuf::new();

        if dir.starts_with('~') {
            target.push(home_dir());
            if dir.len() > 1 {
                target.push(&dir[2..]);
            }
        } else {
            target.push(dir);
        }

        match env::set_current_dir(target) {
            Ok(_) => "".to_string(),
            Err(_) => format!("cd: {}: No such file or directory\n", dir),
        }
    } else {
        // No arguments - go to home directory
        match env::set_current_dir(home_dir()) {
            Ok(_) => "".to_string(),
            Err(e) => format!("cd: {}: {}\n", home_dir().to_string_lossy(), e),
        }
    }
}


// Helper function to get home directory
fn home_dir() -> PathBuf {
    // For Unix
    if let Ok(home) = env::var("HOME") {
        return PathBuf::from(home);
    }

    // For Windows
    if let Ok(userprofile) = env::var("USERPROFILE") {
        return PathBuf::from(userprofile);
    }
    if let (Ok(homedrive), Ok(homepath)) = (env::var("HOMEDRIVE"), env::var("HOMEPATH")) {
        return PathBuf::from(format!("{}{}", homedrive, homepath));
    }

    panic!("Could not determine home directory");
}