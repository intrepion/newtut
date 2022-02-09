#[cfg(test)]
mod test {
    mod get_folder_name_should {
        use super::super::*;

        #[test]
        fn return_folder_name_with_app_name_hello_world_and_language_rust_and_program_type_library()
        {
            let expected = "a-hello-world-l-rust-p-library";

            let actual = &get_folder_name("hello world", "rust", "library");

            assert_eq!(actual, expected);
        }
    }

    mod make_text_valid_for_repository_should {
        use super::super::*;

        #[test]
        fn return_valid_text_for_repository_with_valid_text()
        {
            let expected = "todo";

            let actual = &make_text_valid_for_repository("todo");

            assert_eq!(actual, expected);
        }

        #[test]
        fn return_valid_text_for_repository_with_spaceful_text()
        {
            let expected = "fizz-buzz";

            let actual = &make_text_valid_for_repository("  fizz buzz   \n");

            assert_eq!(actual, expected);
        }

        #[test]
        fn return_valid_text_for_repository_with_capitalized_text()
        {
            let expected = "hello-world";

            let actual = &make_text_valid_for_repository("Hello World");

            assert_eq!(actual, expected);
        }
    }
}

pub fn get_folder_name<'a>(app_name: &'a str, language: &'a str, program_type: &'a str) -> String {
    let app_name = make_text_valid_for_repository(app_name);
    format!("a-{app_name}-l-{language}-p-{program_type}")
}

fn make_text_valid_for_repository(text: &str) -> String {
    text.trim().to_lowercase().replace(" ", "-")
}
