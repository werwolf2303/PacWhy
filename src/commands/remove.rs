use crate::schemas::{Package, PackageRaw};
use sea_orm::ColumnTrait;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};

pub async fn remove_handler(name: String, db: &DatabaseConnection) {
    let result = Package::delete_many()
        .filter(PackageRaw::Column::Name.eq(name.as_str()))
        .exec(db)
        .await;

    if result.is_err() {
        println!("{}", result.unwrap_err());
        return;
    }

    let result_unwrapped = result.unwrap();

    println!("Removed {} packages", result_unwrapped.rows_affected);
}