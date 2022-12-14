pub fn get_env_var(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|e| format!("{}: {}", name, e))
}

pub fn get_bool_of_env_var(name: &str) -> bool {
    get_env_var(name).unwrap().to_uppercase() == "TRUE"
}
