use std::env;


pub(crate) fn pwd_builtin() -> String {
    env::current_dir().unwrap().to_string_lossy().to_string()
}