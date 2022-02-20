#[cfg(test)]
mod get_folder_name_should {
    use super::get_folder_name;

    #[test]
    fn return_folder_name_with_application_name_hello_world_and_language_rust_and_program_type_library(
    ) {
        let expected = "a-hello-world-l-rust-p-library";

        let actual = &get_folder_name("Hello World", "Rust", "Library");

        assert_eq!(actual, expected);
    }

    #[test]
    fn return_folder_name_with_application_name_hello_world_and_language_rust_and_program_type_console(
    ) {
        let expected = "a-hello-world-l-rust-p-console";

        let actual = &get_folder_name("Hello World", "Rust", "Console");

        assert_eq!(actual, expected);
    }
}

use crate::make_name_valid;

pub fn get_folder_name<'a>(
    application_name: &'a str,
    language: &'a str,
    program_type: &'a str,
) -> String {
    let application_name = make_name_valid(application_name);
    let language = make_name_valid(language);
    let program_type = make_name_valid(program_type);
    format!("a-{application_name}-l-{language}-p-{program_type}")
}
