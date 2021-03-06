pub mod language;
pub mod utilities;

use crate::utilities::make_name_valid::make_name_valid;

pub fn get_generate_application_message(
    application_name: &str,
    language: &str,
    program_type: &str,
) -> String {
    let application_name = make_name_valid(application_name);
    let language = make_name_valid(language);
    let program_type = make_name_valid(program_type);

    if language == "rust" {
        if program_type == "library" {
            return language::rust::program_type::library::get_generate_application_message(
                &application_name,
            );
        }
        return language::rust::program_type::console::get_generate_application_message(
            &application_name,
        );
    }
    "".to_owned()
}

pub fn get_creating_gitignore_file_message(language: &str, program_type: &str) -> String {
    let language = make_name_valid(language);
    let program_type = make_name_valid(program_type);

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

pub fn get_creating_workflow_file_message(language: &str) -> String {
    let language = make_name_valid(language);

    if language == "rust" {
        return r#"Creating workflow file

cargo fmt - makes sure rust code is formatted properly
cargo check - quick check for any errors
cargo clippy - linter
cargo test - runs all tests
cargo audit - checks for security vulnerabilities
cargo udeps - checks for unused dependencies
"#
        .to_owned();
    }

    "".to_owned()
}

pub fn get_gitignore_text(language: &str, program_type: &str) -> String {
    let language = make_name_valid(language);
    let program_type = make_name_valid(program_type);
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

pub fn get_workflow_file_text(app_name_prog_type: &str, language: &str) -> String {
    let app_name_prog_type = make_name_valid(app_name_prog_type);
    let language = make_name_valid(language);

    if language == "rust" {
        return format!(
            r#"name: main

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always

jobs:
  format:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2
    - name: Format
      run: cd {app_name_prog_type} && cargo fmt -- --check
  check:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2
    - name: Check
      run: cd {app_name_prog_type} && cargo check
  lint:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2
    - name: Lint
      run: cd {app_name_prog_type} && cargo clippy -- -D warnings
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2
    - name: Test
      run: cd {app_name_prog_type} && cargo test --verbose
  audit:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2
    - name: Audit
      run: cd {app_name_prog_type} && cargo audit
  unused:
    runs-on: ubuntu-latest
    steps:
    - name: Install latest nightly
      uses: actions-rs/toolchain@v1
      with:
        toolchain: nightly
    - name: "Udeps Installation"
      uses: actions-rs/cargo@v1
      with:
        command: install
        args: cargo-udeps --locked
    - uses: actions/checkout@v2
    - name: Unused
      run: cd {app_name_prog_type} && cargo +nightly udeps
"#
        );
    }

    "".to_owned()
}
