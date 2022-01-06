//! SeaORM Entity. Generated by sea-orm-codegen 0.5.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "alter_names")]
pub struct Model {
    pub part_id: i32,
    pub name: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::part_names::Entity",
        from = "Column::PartId",
        to = "super::part_names::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    PartNames,
}

impl Related<super::part_names::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PartNames.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
