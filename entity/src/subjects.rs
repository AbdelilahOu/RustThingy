//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "subjects")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub subject_name: String,
    pub subject_description: Option<String>,
    pub level_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::classes::Entity")]
    Classes,
    #[sea_orm(
        belongs_to = "super::levels::Entity",
        from = "Column::LevelId",
        to = "super::levels::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Levels,
    #[sea_orm(has_many = "super::teacher_subjects::Entity")]
    TeacherSubjects,
}

impl Related<super::classes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Classes.def()
    }
}

impl Related<super::levels::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Levels.def()
    }
}

impl Related<super::teacher_subjects::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TeacherSubjects.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
