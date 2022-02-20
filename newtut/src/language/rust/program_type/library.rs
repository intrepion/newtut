use crate::make_name_valid;

pub fn get_generate_application_message(application_name: &str) -> String {
    let application_name = make_name_valid(application_name);

    format!(
        r#"Generating application

cargo new --lib {application_name};
"#
    )
}

pub fn get_creating_gitignore_file_message() -> String {
    r#"Creating .gitignore file

Here, we are creating a .gitignore file to let git know that we do not want to track any Rust backup files or anything in the build directory.
"#
        .to_owned()
}
