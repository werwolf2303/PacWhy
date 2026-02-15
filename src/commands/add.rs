use sea_orm::ColumnTrait;
use crate::schemas::{Package, PackageModel, PackageRaw};
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

pub async fn add_handler(name: String, description: String, reason: String, db: &DatabaseConnection) {
    let package = PackageModel {
        name: Set(name.clone()),
        description: Set(description.clone()),
        reason: Set(reason.clone()),
        date_added: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    };

    let package_search_result = Package::find()
        .filter(PackageRaw::Column::Name.eq(name.clone()))
        .one(db).await;

    if package_search_result.is_err() {
        println!("Could not find if package {} is already in the database", name);
        return;
    }

    let package_search_result_unwrapped = package_search_result.unwrap();

    if package_search_result_unwrapped.is_some() {
        println!("Package {} is already in the database. Skipped", name);
        return;
    }

    package.insert(db).await.expect("Unable to add package to internal database");

    println!("Package {} added successfully", name);
}