//! SeaORM Entity. Generated by sea-orm-codegen 0.5.0

use std::cmp::Ordering;
use std::collections::HashMap;
use async_graphql::{ComplexObject, Context, InputObject, SimpleObject};
use async_graphql::async_trait::async_trait;
use itertools::Itertools;
use sea_orm::{Condition, DbBackend, JoinType, QueryOrder, QuerySelect, QueryTrait};
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::UnionType;
use macros::FilterQueryBuilder;
use crate::entities::Paging;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel,Eq,Hash,SimpleObject,Default)]
#[graphql(name="PartName",complex)]
#[sea_orm(table_name = "part_names")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub name: String,
}
#[derive(Clone, Debug, PartialEq,Eq,Hash,Default,InputObject,FilterQueryBuilder)]
#[graphql(name="PartNameInput")]
pub struct ModelInput {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub page : Option<Paging>,
    #[join(Relation::AlterNames.def())]
    pub alter_name: Option<super::alter_names::ModelInput>
}

#[ComplexObject]
impl Model {
    async fn alterNames(&self,ctx:&Context<'_>,mut like : Option<super::alter_names::ModelInput>) -> Vec<super::alter_names::Model>{
        let loader = ctx.data_unchecked::<super::DLoader>();
        match like {
            None => {return loader.load_one(
                super::alter_names::ModelInput{part_id:Some(self.id),..super::alter_names::ModelInput::default()}
            ).await.unwrap().unwrap()}
            Some(mut like) => {
                like.part_id = Some(self.id);
                return loader.load_one(like).await.unwrap().unwrap();
            }
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::alter_names::Entity")]
    AlterNames,
    #[sea_orm(has_many = "super::items::Entity")]
    Items,
}

impl Related<super::alter_names::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AlterNames.def()
    }
}

impl Related<super::items::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Items.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
