use std::path::PathBuf;
use std::process::{exit, Command, Stdio};
use sea_orm::DatabaseConnection;

pub async fn scripts_handler(args: Vec<String>, db: &DatabaseConnection) {
    let current_exe_path = std::env::current_exe().unwrap();
    let scripts_path = PathBuf::new().join(current_exe_path.parent().unwrap()).join("scripts");
    let mut scripts: Vec<String> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(scripts_path.clone()) {
        for entry in entries {
            let path = entry.unwrap().path();
            if path.is_file() {
                scripts.push(path.file_name().unwrap().to_string_lossy().to_string());
            }
        }
    }

    if args.is_empty() {
        print_help(scripts);
        return;
    }

    if args.contains(&String::from("--help")) || args.contains(&String::from("-h")) || args.contains(&String::from("help")) {
        print_help(scripts);
        return;
    }

    let script_name = args.get(0).unwrap();

    if !scripts.contains(&script_name.to_string()) {
        eprintln!("Script not found: {}\n", script_name);
        print_help(scripts);
        return;
    }

    db.close_by_ref().await.expect("Failed to close database connection before running a script");

    let command = Command::new("bash")
        .arg(scripts_path.join(args.get(0).unwrap()))
        .args(args.iter().skip(1))
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()
        .unwrap().wait()
        .expect("Failed to execute script");

    exit(command.code().unwrap_or(0));
}

fn print_help(scripts: Vec<String>) {
    println!("PacWhy scripts <NAME> <ARGUMENTS>\n\nAvailable Scripts: \n{}", scripts.join("\n"));
}