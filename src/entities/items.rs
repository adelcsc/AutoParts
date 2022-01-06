//! SeaORM Entity. Generated by sea-orm-codegen 0.5.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "items")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub part_id: i32,
    pub c_bar: i32,
    pub price: i32,
    pub buy_id: i32,
    pub part_brand_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::part_brands::Entity",
        from = "Column::PartBrandId",
        to = "super::part_brands::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    PartBrands,
    #[sea_orm(
        belongs_to = "super::buy_history::Entity",
        from = "Column::BuyId",
        to = "super::buy_history::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    BuyHistory,
    #[sea_orm(
        belongs_to = "super::part_names::Entity",
        from = "Column::PartId",
        to = "super::part_names::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    PartNames,
    #[sea_orm(has_many = "super::place_item::Entity")]
    PlaceItem,
    #[sea_orm(has_many = "super::item_car::Entity")]
    ItemCar,
    #[sea_orm(has_many = "super::item_feedback::Entity")]
    ItemFeedback,
    #[sea_orm(has_many = "super::item_order::Entity")]
    ItemOrder,
}

impl Related<super::part_brands::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PartBrands.def()
    }
}

impl Related<super::buy_history::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BuyHistory.def()
    }
}

impl Related<super::part_names::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PartNames.def()
    }
}

impl Related<super::place_item::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PlaceItem.def()
    }
}

impl Related<super::item_car::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ItemCar.def()
    }
}

impl Related<super::item_feedback::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ItemFeedback.def()
    }
}

impl Related<super::item_order::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ItemOrder.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
