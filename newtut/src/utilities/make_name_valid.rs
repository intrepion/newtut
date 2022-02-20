#[cfg(test)]
mod make_name_valid_should {
    use super::make_name_valid;

    #[test]
    fn return_valid_text_for_repository_with_valid_text() {
        let expected = "todo";

        let actual = &make_name_valid("todo");

        assert_eq!(actual, expected);
    }

    #[test]
    fn return_valid_text_for_repository_with_spaceful_text() {
        let expected = "fizz-buzz";

        let actual = &make_name_valid("  fizz buzz   \n");

        assert_eq!(actual, expected);
    }

    #[test]
    fn return_valid_text_for_repository_with_capitalized_text() {
        let expected = "hello-world";

        let actual = &make_name_valid("Hello World");

        assert_eq!(actual, expected);
    }
}

pub fn make_name_valid(name: &str) -> String {
    name.trim().to_lowercase().replace(" ", "-")
}
