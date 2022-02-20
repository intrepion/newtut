use newtut::utilities::get_app_name_prog_type::get_app_name_prog_type;
use newtut::utilities::get_folder_name::get_folder_name;
use newtut::utilities::make_name_valid::make_name_valid;
use newtut::{
    get_creating_gitignore_file_message, get_creating_workflow_file_message,
    get_generate_application_message, get_gitignore_text, get_workflow_file_text,
};
use std::fs::File;
use std::io::Write;
use std::process::{Command, Output};
use std::{env, fs, process};

fn main() {
    let usage = r#"Usage: newtut <username> <program-name> <language> <program-type>

    <language> can only be the following:

    - rust

    <program-type> can only be the following:

    - library
"#;

    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        println!("{usage}");

        process::exit(1);
    }

    let user_name = &args[1];
    let application_name = &args[2];
    let language = &args[3];
    let program_type = &args[4];

    let path_name = format!("repos/github/{user_name}");

    println!("mkdir -p path_name: {path_name};");

    fs::create_dir_all(&path_name).expect("unable to create path name");

    println!("cd {path_name};");

    env::set_current_dir(&path_name).expect("changing directory failed");

    let folder_name = get_folder_name(application_name, language, program_type);
    let description = format!("{application_name} {program_type} written in {language}");
    let homepage = format!("https://{user_name}.github.io/{folder_name}");

    let gh_repo_create_string = format!("gh repo create {folder_name} --clone --description \"{description}\" --homepage \"{homepage}\" --license mit --public;");

    println!("{gh_repo_create_string}");

    let gh_repo_create_output = Command::new("gh")
        .arg("repo")
        .arg("create")
        .arg(&folder_name)
        .arg("--clone")
        .arg("--description")
        .arg(&description)
        .arg("--homepage")
        .arg(&homepage)
        .arg("--license")
        .arg("mit")
        .arg("--public")
        .output()
        .expect("gh repo create failed");

    display_output_or_exit("gh repo create", gh_repo_create_output);

    println!("cd {folder_name};");

    env::set_current_dir(&folder_name).expect("changing directory failed");

    let new_commit_message = format!(
        "{}{gh_repo_create_string}",
        r#"Initial commit

"#
    );

    println!("git commit --amend --message '{new_commit_message}';");

    let git_commit_amend_output = Command::new("git")
        .arg("commit")
        .arg("--amend")
        .arg("--message")
        .arg(&new_commit_message)
        .output()
        .expect("git commit failed");

    display_output_or_exit("git commit", git_commit_amend_output);

    println!("git push --force;");

    let git_push_force_output = Command::new("git")
        .arg("push")
        .arg("--force")
        .output()
        .expect("git push force failed");

    display_output_or_exit("git push force", git_push_force_output);

    println!("creating .gitignore file");

    let mut file = File::create(".gitignore").expect("unable to create .gitignore file");
    file.write_all(get_gitignore_text(language, program_type).as_bytes())
        .expect("unable to write to .gitignore file");
    git_add(".gitignore");
    git_commit(&get_creating_gitignore_file_message(language, program_type));
    git_push();

    let full_application_name = get_app_name_prog_type(application_name, program_type);
    let valid_language = make_name_valid(language);

    generate_application(&valid_language, &full_application_name);
    git_add(&full_application_name);
    git_commit(&get_generate_application_message(
        application_name,
        language,
        program_type,
    ));
    git_push();

    create_workflow_file(&full_application_name, &valid_language);
    git_add(".github/workflows/main.yml");
    git_commit(&get_creating_workflow_file_message(&valid_language));
    git_push();
}

fn display_output_or_exit(output_name: &str, output: Output) {
    if output.status.success() {
        println!(
            "{} stdout: {}",
            output_name,
            String::from_utf8(output.stdout).unwrap()
        );
    } else {
        println!(
            "{} stderr: {}",
            output_name,
            String::from_utf8(output.stderr).unwrap()
        );

        process::exit(1);
    }
}

fn create_workflow_file(full_application_name: &str, valid_language: &str) {
    if valid_language == "rust" {
        fs::create_dir_all(".github/workflows").expect("unable to create workflows path name");
        let mut file =
            File::create(".github/workflows/main.yml").expect("unable to create workflow file");
        let text = get_workflow_file_text(full_application_name, valid_language);
        file.write_all(text.as_bytes())
            .expect("unable to write to workflow file");
    }
}

fn generate_application(valid_language: &str, full_application_name: &str) {
    println!("generating application {full_application_name};");

    if valid_language == "rust" {
        let cargo_new_output = Command::new("cargo")
            .arg("new")
            .arg("--lib")
            .arg(&full_application_name)
            .output()
            .expect("cargo new failed");

        display_output_or_exit("cargo new", cargo_new_output);
    }
}

fn git_add(files_to_add: &str) {
    println!("git add {files_to_add};");

    let git_add_output = Command::new("git")
        .arg("add")
        .arg(files_to_add)
        .output()
        .expect("git add failed");

    display_output_or_exit("git add", git_add_output);
}

fn git_commit(message: &str) {
    println!("git commit --message {message};");

    let git_commit_output = Command::new("git")
        .arg("commit")
        .arg("--message")
        .arg(&message)
        .output()
        .expect("git commit failed");

    display_output_or_exit("git commit", git_commit_output);
}

fn git_push() {
    println!("git push;");

    let git_push_output = Command::new("git")
        .arg("push")
        .output()
        .expect("git push failed");

    display_output_or_exit("git push", git_push_output);
}
