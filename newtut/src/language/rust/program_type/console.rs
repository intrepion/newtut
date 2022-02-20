use crate::make_name_valid;

pub fn get_generate_application_message(
    application_name: &str,
) -> String {
    let application_name = make_name_valid(application_name);
    format!(
            r#"Generating application

cargo new {application_name};
"#)
}
