//! SeaORM Entity. Generated by sea-orm-codegen 0.5.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "cars")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub car_name: String,
    pub brand_id: i32,
    pub serie: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::car_brands::Entity",
        from = "Column::BrandId",
        to = "super::car_brands::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    CarBrands,
    #[sea_orm(
        belongs_to = "super::car_series::Entity",
        from = "Column::Serie",
        to = "super::car_series::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    CarSeries,
    #[sea_orm(has_many = "super::item_car::Entity")]
    ItemCar,
}

impl Related<super::car_brands::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CarBrands.def()
    }
}

impl Related<super::car_series::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CarSeries.def()
    }
}

impl Related<super::item_car::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ItemCar.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}