//! SeaORM Entity. Generated by sea-orm-codegen 0.5.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "credit_payments")]
pub struct Model {
    pub credit_holder: i32,
    pub post_paid: Option<i32>,
    pub time: DateTime,
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::credit_book::Entity",
        from = "Column::CreditHolder",
        to = "super::credit_book::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    CreditBook,
}

impl Related<super::credit_book::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CreditBook.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}