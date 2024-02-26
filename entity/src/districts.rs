//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::{entity::prelude::*, Set};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "districts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub district_name: String,
    pub city_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::cities::Entity",
        from = "Column::CityId",
        to = "super::cities::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Cities,
    #[sea_orm(has_many = "super::person_details::Entity")]
    PersonDetails,
    #[sea_orm(has_many = "super::streets::Entity")]
    Streets,
}

impl Related<super::cities::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Cities.def()
    }
}

impl Related<super::person_details::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PersonDetails.def()
    }
}

impl Related<super::streets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Streets.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(Uuid::new_v4()),
            ..ActiveModelTrait::default()
        }
    }
}
