//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "rooms")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub room_name: String,
    pub room_description: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::classes::Entity")]
    Classes,
}

impl Related<super::classes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Classes.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
