#[cfg(test)]
    mod get_app_name_prog_type_should {
        use super::get_app_name_prog_type;

        #[test]
        fn return_app_name_prog_type_with_application_name_hello_world_and_program_type_library()
        {
            let expected = "a-hello-world-p-library";

            let actual = get_app_name_prog_type("Hello World", "Library");

            assert_eq!(actual, expected);
        }

        #[test]
        fn return_app_name_prog_type_with_application_name_hello_world_and_program_type_console()
        {
            let expected = "a-hello-world-p-console";

            let actual = get_app_name_prog_type("Hello World", "Console");

            assert_eq!(actual, expected);
        }
    }

use crate::make_name_valid;

pub fn get_app_name_prog_type(application_name: &str, program_type: &str) -> String {
    let application_name = make_name_valid(application_name);
    let program_type = make_name_valid(program_type);

    return format!("a-{application_name}-p-{program_type}");
}
