use sea_orm::{DatabaseConnection};
use crate::commands::find::find_handler;

pub async fn list_handler(parseable: String, db: &DatabaseConnection) {
    // When not providing a name, description and time to the find query it lists
    // every package in the Database
    find_handler(String::new(), String::new(), String::new(), parseable, true, db).await;
}