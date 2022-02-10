#[cfg(test)]
mod test {
    mod get_folder_name_should {
        use super::super::get_folder_name;

        #[test]
        fn return_folder_name_with_application_name_hello_world_and_language_rust_and_program_type_library(
        ) {
            let expected = "a-hello-world-l-rust-p-library";

            let actual = &get_folder_name("Hello World", "Rust", "Library");

            assert_eq!(actual, expected);
        }
    }

    mod make_text_valid_for_repository_should {
        use super::super::make_text_valid_for_repository;

        #[test]
        fn return_valid_text_for_repository_with_valid_text() {
            let expected = "todo";

            let actual = &make_text_valid_for_repository("todo");

            assert_eq!(actual, expected);
        }

        #[test]
        fn return_valid_text_for_repository_with_spaceful_text() {
            let expected = "fizz-buzz";

            let actual = &make_text_valid_for_repository("  fizz buzz   \n");

            assert_eq!(actual, expected);
        }

        #[test]
        fn return_valid_text_for_repository_with_capitalized_text() {
            let expected = "hello-world";

            let actual = &make_text_valid_for_repository("Hello World");

            assert_eq!(actual, expected);
        }
    }

    mod get_gitignore_text_should {
        use super::super::get_gitignore_text;

        #[test]
        fn return_gitignore_text_with_language_rust_and_program_type_library() {
            let expected = r#"**/*.rs.bk
**/target/
Cargo.lock
"#;

            let actual = get_gitignore_text("Rust", "Library");

            assert_eq!(actual, expected);
        }

        #[test]
        fn return_gitignore_text_with_language_rust_and_program_type_console() {
            let expected = r#"**/*.rs.bk
**/target/
"#;

            let actual = get_gitignore_text("Rust", "Console");

            assert_eq!(actual, expected);
        }
    }

    mod get_creating_gitignore_file_message_should {
        use super::super::get_creating_gitignore_file_message;

        #[test]
        fn return_creating_gitignore_file_message_with_language_rust_and_program_type_library() {
            let expected = r#"Creating .gitignore file

Here, we are creating a .gitignore file to let git know that we do not want to track any Rust backup files or anything in the build directory.
Because we are creating a library, we also want to ignore the Cargo.lock file.
"#;

            let actual = get_creating_gitignore_file_message("Rust", "Library");

            assert_eq!(actual, expected);
        }

        #[test]
        fn return_creating_gitignore_file_message_with_language_rust_and_program_type_console() {
            let expected = r#"Creating .gitignore file

Here, we are creating a .gitignore file to let git know that we do not want to track any Rust backup files or anything in the build directory.
"#;

            let actual = get_creating_gitignore_file_message("Rust", "Console");

            assert_eq!(actual, expected);
        }
    }

    mod get_generate_application_message_should {
        use super::super::get_generate_application_message;

        #[test]
        fn return_generate_application_message_with_application_name_hello_world_and_program_type_library(
        ) {
            let expected = r#"Generating application

cargo new --lib hello-world;
"#;

            let actual = get_generate_application_message("Hello World", "Rust", "Library");

            assert_eq!(actual, expected);
        }
    }
}

pub fn get_generate_application_message(
    application_name: &str,
    language: &str,
    program_type: &str,
) -> String {
    let application_name = make_text_valid_for_repository(application_name);
    let language = make_text_valid_for_repository(language);
    let program_type = make_text_valid_for_repository(program_type);

    if language == "rust" {
        if program_type == "library" {
            return format!(
                r#"Generating application

cargo new --lib {application_name};
"#
            );
        }
        return format!(
            r#"Generating application

cargo new {application_name};
"#
        );
    }
    "".to_owned()
}

pub fn get_creating_gitignore_file_message(language: &str, program_type: &str) -> String {
    let language = make_text_valid_for_repository(language);
    let program_type = make_text_valid_for_repository(program_type);

    if language == "rust" {
        if program_type == "library" {
            return r#"Creating .gitignore file

Here, we are creating a .gitignore file to let git know that we do not want to track any Rust backup files or anything in the build directory.
Because we are creating a library, we also want to ignore the Cargo.lock file.
"#
            .to_owned();
        }
        return r#"Creating .gitignore file

Here, we are creating a .gitignore file to let git know that we do not want to track any Rust backup files or anything in the build directory.
"#
        .to_owned();
    }
    "".to_owned()
}

pub fn get_folder_name<'a>(
    application_name: &'a str,
    language: &'a str,
    program_type: &'a str,
) -> String {
    let application_name = make_text_valid_for_repository(application_name);
    let language = make_text_valid_for_repository(language);
    let program_type = make_text_valid_for_repository(program_type);
    format!("a-{application_name}-l-{language}-p-{program_type}")
}

pub fn get_gitignore_text(language: &str, program_type: &str) -> String {
    let language = make_text_valid_for_repository(language);
    let program_type = make_text_valid_for_repository(program_type);
    if language == "rust" {
        if program_type == "library" {
            return r#"**/*.rs.bk
**/target/
Cargo.lock
"#
            .to_owned();
        }
        return r#"**/*.rs.bk
**/target/
"#
        .to_owned();
    }
    "".to_owned()
}

pub fn make_text_valid_for_repository(text: &str) -> String {
    text.trim().to_lowercase().replace(" ", "-")
}
