//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::{entity::prelude::*, Set};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "events")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(column_type = "Text", nullable)]
    pub event_title: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub event_description: Option<String>,
    pub time_table_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::time_table::Entity",
        from = "Column::TimeTableId",
        to = "super::time_table::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    TimeTable,
}

impl Related<super::time_table::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TimeTable.def()
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
