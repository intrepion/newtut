use newtut::get_folder_name;
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
    println!("{folder_name}");
}
