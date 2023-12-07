//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "states")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub state_name: String,
    pub state_initials: Option<String>,
    pub state_code: Option<i32>,
    pub country_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::cities::Entity")]
    Cities,
    #[sea_orm(
        belongs_to = "super::countries::Entity",
        from = "Column::CountryId",
        to = "super::countries::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Countries,
    #[sea_orm(has_many = "super::person_details::Entity")]
    PersonDetails,
}

impl Related<super::cities::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Cities.def()
    }
}

impl Related<super::countries::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Countries.def()
    }
}

impl Related<super::person_details::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PersonDetails.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
