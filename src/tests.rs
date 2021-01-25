#[cfg(test)]
pub mod tests {
    // Fix the tests here
    use super::super::EnvLoader;

    #[test]
    fn load_default() {
        EnvLoader::new().activate();

        let variable = std::env::var("SUPER_SECRET").unwrap();

        assert_eq!(variable, String::from("I_ate_the_chocolate"))
    }

    #[test]
    #[should_panic]
    fn load_default_wrong() {
        EnvLoader::new().activate();

        std::env::var("Doesn't exist").unwrap();
    }


    #[test]
    fn load_external_file() {
        EnvLoader::new()
        .change_comment(String::from("#"))
        .change_delimiter(String::from("--"))
        .change_file(String::from("./.env"))
        .activate();

        let new_delimiter = std::env::var("NEW_DELIMITER").unwrap();

        assert_eq!(new_delimiter, String::from("is_working"))
    }
}