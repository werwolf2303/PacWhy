use std::collections::HashMap;
use std::io;
use sea_orm::{DatabaseConnection, EntityTrait};
use crate::schemas::{Package};

pub async fn sync_handler(db: &DatabaseConnection) {
    let packages_output = std::process::Command::new("/usr/bin/pacman")
        .arg("-Q")
        .output().unwrap();

    let mut packages: Vec<String> = Vec::new();

    for line in str::from_utf8(packages_output.stdout.as_slice()).unwrap().split('\n') {
        packages.push(line.split(' ').next().unwrap().to_string());
    };

    packages.remove(packages.len() - 1);

    let packages_in_database = Package::find()
        .all(db).await;

    if packages_in_database.is_err() {
        println!("Failed to find packages in database {:?}", packages_in_database.err());
        return;
    }

    let packages_in_database = packages_in_database.unwrap();

    let mut not_found_packages: Vec<String> = Vec::new();

    for package in packages_in_database.as_slice() {
        if !packages.contains(&package.name) {
            not_found_packages.push(package.name.clone());
        }
    }

    if not_found_packages.is_empty() {
        println!("No sync was needed");
        return;
    }

    println!("Found packages that are not present on the system");

    // Database is out of sync
    for (i, package) in not_found_packages.iter().enumerate() {
        println!("[{i}] {}", package);
    }

    let length_not_found = not_found_packages.len();
    println!("\nSelect packages you want to remove [0-{}, a/all]:", length_not_found - 1);
    let mut selection = String::new();
    io::stdin().read_line(&mut selection).expect("Failed to read line");

    let mut to_remove: HashMap<i32, String> = HashMap::new();
    for (i, entry) in selection.trim().split(',').enumerate() {
        if i == 0 && (entry.eq("a") || entry.eq("all")) {
            let mut iter = 0;
            while iter < length_not_found {
                let package = packages_in_database.get(iter).unwrap();
                to_remove.insert(package.id, package.name.clone());
                iter += 1;
            }
            break;
        }

        let package = packages_in_database.get(entry.parse::<usize>().unwrap() + 1).unwrap();
        to_remove.insert(package.id, package.name.clone());
    }

    if to_remove.is_empty() {
        println!("No packages removed");
        return;
    }

    for i in to_remove {
        let result = Package::delete_by_id(i.0)
            .exec(db).await;

        if result.is_err() {
            println!("Failed to delete package {}: {}", i.1, result.err().unwrap());
        }

        println!("Removed package {}", i.1);
    }
}