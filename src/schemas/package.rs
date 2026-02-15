use sea_orm::entity::prelude::*;
use sea_orm::sqlx::types::chrono::NaiveDateTime;

#[derive(Clone, Debug, PartialEq, Default, DeriveEntityModel)]
#[sea_orm(table_name = "packages")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub name: String,
    pub description: String,
    pub reason: String,
    pub date_added: NaiveDateTime
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}