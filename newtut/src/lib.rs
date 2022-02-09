#[cfg(test)]
mod get_folder_name_should {
    use super::*;

    #[test]
    fn return_folder_name_with_app_name_hello_world_and_language_rust_and_program_type_library() {
        let expected = "a-hello-world-l-rust-p-library";

        let actual = &get_folder_name("hello world", "rust", "library");

        assert_eq!(actual, expected);
    }
}

pub fn get_folder_name<'a>(app_name: &'a str, language: &'a str, program_type: &'a str) -> String {
    let app_name = app_name.replace(" ", "-");
    format!("a-{app_name}-l-{language}-p-{program_type}")
}
