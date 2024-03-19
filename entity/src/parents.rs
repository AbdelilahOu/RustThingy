//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "parents")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub created_at: Option<DateTime>,
    pub full_name: Option<String>,
    pub person_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::persons::Entity",
        from = "Column::PersonId",
        to = "super::persons::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Persons,
    #[sea_orm(has_many = "super::pickups::Entity")]
    Pickups,
}

impl Related<super::persons::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Persons.def()
    }
}

impl Related<super::pickups::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Pickups.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
