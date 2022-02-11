use newtut::{
    get_creating_gitignore_file_message, get_folder_name, get_full_application_name,
    get_generate_application_message, get_gitignore_text,
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

    println!("git add .gitignore;");

    let git_add_gitignore_output = Command::new("git")
        .arg("add")
        .arg(".gitignore")
        .output()
        .expect("git add gitignore failed");

    display_output_or_exit("git add gitignore", git_add_gitignore_output);

    let creating_gitignore_file_message =
        get_creating_gitignore_file_message(language, program_type);

    println!("git commit --message {creating_gitignore_file_message};");

    let git_commit_output = Command::new("git")
        .arg("commit")
        .arg("--message")
        .arg(&creating_gitignore_file_message)
        .output()
        .expect("git commit failed");

    display_output_or_exit("git commit", git_commit_output);

    git_push();

    let full_application_name = get_full_application_name(application_name, program_type);

    println!("generating application {full_application_name};");

    let cargo_new_output = Command::new("cargo")
        .arg("new")
        .arg("--lib")
        .arg(&full_application_name)
        .output()
        .expect("cargo new failed");

    display_output_or_exit("cargo new", cargo_new_output);

    println!("git add {full_application_name}");

    let git_add_full_application_name_output = Command::new("git")
        .arg("add")
        .arg(&full_application_name)
        .output()
        .expect("git add failed");

    display_output_or_exit("git add", git_add_full_application_name_output);

    let generate_application_message =
        get_generate_application_message(application_name, language, program_type);

    println!("git commit --message '{generate_application_message}';");

    let git_commit_output = Command::new("git")
        .arg("commit")
        .arg("--message")
        .arg(&generate_application_message)
        .output()
        .expect("git commit failed");

    display_output_or_exit("git commit", git_commit_output);

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

fn git_push() {
    println!("git push;");

    let git_push_output = Command::new("git")
        .arg("push")
        .output()
        .expect("git push failed");

    display_output_or_exit("git push", git_push_output);
}
