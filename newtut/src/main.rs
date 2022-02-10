use newtut::{get_creating_gitignore_file_message, get_folder_name, get_gitignore_text};
use std::fs::File;
use std::io::Write;
use std::process::Command;
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

    println!("mkdir -p path_name: {path_name}");

    fs::create_dir_all(&path_name).expect("unable to create path name");

    println!("cd {path_name}");

    env::set_current_dir(&path_name).expect("changing directory failed");

    let folder_name = get_folder_name(application_name, language, program_type);
    let description = format!("{application_name} {program_type} written in {language}");
    let homepage = format!("https://{user_name}.github.io/{folder_name}");

    let gh_repo_create_string = format!("gh repo create {folder_name} --clone --description \"{description}\" --homepage \"{homepage}\" --license mit --public");

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

    if gh_repo_create_output.status.success() {
        println!(
            "gh repo create stdout: {}",
            String::from_utf8_lossy(&gh_repo_create_output.stdout)
        );
    } else {
        println!(
            "gh repo create stderr: {}",
            String::from_utf8_lossy(&gh_repo_create_output.stderr)
        );

        process::exit(1);
    }

    println!("cd {folder_name}");

    env::set_current_dir(&folder_name).expect("changing directory failed");

    let new_commit_message = format!(
        "{}{gh_repo_create_string}",
        r#"Initial commit

"#
    );

    println!("git commit --amend --message '{new_commit_message}'");

    let git_commit_output = Command::new("git")
        .arg("commit")
        .arg("--amend")
        .arg("--message")
        .arg(&new_commit_message)
        .output()
        .expect("git commit failed");

    if git_commit_output.status.success() {
        println!(
            "git commit stdout: {}",
            String::from_utf8_lossy(&git_commit_output.stdout)
        );
    } else {
        println!(
            "git commit stderr: {}",
            String::from_utf8_lossy(&git_commit_output.stderr)
        );

        process::exit(1);
    }

    println!("git push --force");

    let git_push_force_output = Command::new("git")
        .arg("push")
        .arg("--force")
        .output()
        .expect("git push force failed");

    if git_push_force_output.status.success() {
        println!(
            "git push force stdout: {}",
            String::from_utf8_lossy(&git_push_force_output.stdout)
        );
    } else {
        println!(
            "git push force stderr: {}",
            String::from_utf8_lossy(&git_push_force_output.stderr)
        );

        process::exit(1);
    }

    println!("creating .gitignore file");

    let mut file = File::create(".gitignore").expect("unable to create .gitignore file");
    file.write_all(get_gitignore_text(language, program_type).as_bytes())
        .expect("unable to write to .gitignore file");

    println!("git add .gitignore");

    let git_add_gitignore_output = Command::new("git")
        .arg("add")
        .arg(".gitignore")
        .output()
        .expect("git add gitignore failed");

    if git_add_gitignore_output.status.success() {
        println!(
            "git add .gitignore stdout: {}",
            String::from_utf8_lossy(&git_add_gitignore_output.stdout)
        );
    } else {
        println!(
            "git add .gitignore stderr: {}",
            String::from_utf8_lossy(&git_add_gitignore_output.stderr)
        );

        process::exit(1);
    }

    let creating_gitignore_file_message =
        get_creating_gitignore_file_message(language, program_type);

    println!("git commit --message {creating_gitignore_file_message}");

    let git_commit_output = Command::new("git")
        .arg("commit")
        .arg("--message")
        .arg(&creating_gitignore_file_message)
        .output()
        .expect("git commit failed");

    if git_commit_output.status.success() {
        println!(
            "git commit stdout: {}",
            String::from_utf8_lossy(&git_commit_output.stdout)
        );
    } else {
        println!(
            "git commit stderr: {}",
            String::from_utf8_lossy(&git_commit_output.stderr)
        );

        process::exit(1);
    }

    println!("git push");

    let git_push_output = Command::new("git")
        .arg("push")
        .output()
        .expect("git push failed");

    if git_push_output.status.success() {
        println!(
            "git push stdout: {}",
            String::from_utf8_lossy(&git_push_output.stdout)
        );
    } else {
        println!(
            "git push stderr: {}",
            String::from_utf8_lossy(&git_push_output.stderr)
        );

        process::exit(1);
    }
}
