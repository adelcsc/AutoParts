//! SeaORM Entity. Generated by sea-orm-codegen 0.5.0

use sea_orm::entity::prelude::*;
use async_graphql::{ComplexObject, InputObject, SimpleObject};
use macros::FilterQueryBuilder;
use std::collections::HashMap;
use crate::entities::Paging;
use async_graphql::dataloader::{DataLoader, Loader};
use async_graphql::*;
use itertools::Itertools;
use sea_orm::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel,Eq,Hash,SimpleObject,Default)]
#[graphql(name="CarBrand",complex)]
#[sea_orm(table_name = "car_brands")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub name: String,
}
#[derive(Clone, Debug, PartialEq,Eq,Hash,Default,InputObject,FilterQueryBuilder)]
#[graphql(name="CarBrandInput")]
pub struct ModelInput {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub page : Option<Paging>
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
