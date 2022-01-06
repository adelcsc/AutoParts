//! SeaORM Entity. Generated by sea-orm-codegen 0.5.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "fourni")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub name: String,
    pub phone: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::buy_history::Entity")]
    BuyHistory,
}

impl Related<super::buy_history::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BuyHistory.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}