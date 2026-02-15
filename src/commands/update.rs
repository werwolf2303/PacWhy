use crate::schemas::{Package, PackageRaw};
use sea_orm::sea_query::Expr;
use sea_orm::ColumnTrait;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};

pub async fn update_handler(name: String, description: String, reason: String, db: &DatabaseConnection) {
    let mut update_query = Package::update_many()
        .filter(PackageRaw::Column::Name.eq(name.as_str()));

    if !name.is_empty() {
        update_query = update_query.col_expr(PackageRaw::Column::Name, Expr::value(name.clone()));
    }

    if !description.is_empty() {
        update_query = update_query.col_expr(PackageRaw::Column::Description, Expr::value(description));
    }

    if !reason.is_empty() {
        update_query = update_query.col_expr(PackageRaw::Column::Reason, Expr::value(reason));
    }

    let update_result = update_query.exec(db).await;

    if update_result.is_err() {
        println!("{}", update_result.err().unwrap());
        return;
    }

    let update_result_unwrapped = update_result.unwrap();

    if update_result_unwrapped.rows_affected == 0 {
        println!("Found no Packages with name {}", name);
        return;
    }

    println!("Updated {} packages with name {}", update_result_unwrapped.rows_affected, name);
}