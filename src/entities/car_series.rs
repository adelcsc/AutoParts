//! SeaORM Entity. Generated by sea-orm-codegen 0.5.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "car_series")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub name: String,
    pub year_start: i32,
    pub year_end: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::cars::Entity")]
    Cars,
}

impl Related<super::cars::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Cars.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}