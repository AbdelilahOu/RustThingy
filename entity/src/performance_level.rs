//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use super::sea_orm_active_enums::PerformanceLevelTypeEnum;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "performance_level")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub level_name: PerformanceLevelTypeEnum,
    pub grading_criteria_id: Uuid,
    pub description: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::grading_criteria::Entity",
        from = "Column::GradingCriteriaId",
        to = "super::grading_criteria::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    GradingCriteria,
}

impl Related<super::grading_criteria::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GradingCriteria.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
