//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "cities")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub city_name: String,
    pub state_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::contacts_informations::Entity")]
    ContactsInformations,
    #[sea_orm(has_many = "super::districts::Entity")]
    Districts,
    #[sea_orm(
        belongs_to = "super::states::Entity",
        from = "Column::StateId",
        to = "super::states::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    States,
}

impl Related<super::contacts_informations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ContactsInformations.def()
    }
}

impl Related<super::districts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Districts.def()
    }
}

impl Related<super::states::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::States.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
