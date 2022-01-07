//! SeaORM Entity. Generated by sea-orm-codegen 0.5.0

use async_graphql::{Context, Object};
use async_graphql::dataloader::DataLoader;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use crate::entities::prelude::AlterNames;

pub mod prelude;

pub mod alter_names;
pub mod buy_history;
pub mod car_brands;
pub mod car_series;
pub mod cars;
pub mod credit_book;
pub mod credit_payments;
pub mod fourni;
pub mod item_car;
pub mod item_feedback;
pub mod item_order;
pub mod items;
pub mod order_credit;
pub mod orders;
pub mod part_brands;
pub mod part_names;
pub mod part_wishes;
pub mod place_history;
pub mod place_item;
pub mod place_locations;
pub mod places;
pub mod users;


pub struct Query;

pub type DLoader = DataLoader<SqliteLoader>;
#[Object]
impl Query {

    //AlterName Model
    async fn alterNames(&self,ctx : &Context<'_>,like : Option<alter_names::ModelInput>) ->Vec<alter_names::Model> {
        let loader = ctx.data_unchecked::<DLoader>();
        match like {
            None => {return alter_names::Entity::find().all(&loader.loader().pool).await.unwrap();}
            Some(like) => {return loader.load_one(like).await.unwrap().unwrap();}
        }
    }
    //

    // PartName Model
    async fn partNames(&self,ctx : &Context<'_>,like : Option<part_names::ModelInput>) ->Vec<part_names::Model> {
        let loader = ctx.data_unchecked::<DLoader>();
        match like {
            None => {return part_names::Entity::find().all(&loader.loader().pool).await.unwrap();}
            Some(like) => {return loader.load_one(like).await.unwrap().unwrap();}
        }
    }
    //

}
pub struct SqliteLoader {
    pub(crate) pool : DatabaseConnection
}
