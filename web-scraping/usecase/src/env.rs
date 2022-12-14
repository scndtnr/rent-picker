pub fn get_env_var(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|e| format!("{}: {}", name, e))
}

pub fn get_usize_of_env_var(name: &str) -> usize {
    get_env_var(name)
        .unwrap()
        .parse()
        .unwrap_or_else(|_| panic!("Fail to parse {} into usize.", name))
}

pub fn get_bool_of_env_var(name: &str) -> bool {
    get_env_var(name).unwrap().to_uppercase() == "TRUE"
}
