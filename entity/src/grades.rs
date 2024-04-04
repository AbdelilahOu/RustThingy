//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "grades")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub student_id: Uuid,
    pub assignment_id: Uuid,
    #[sea_orm(column_type = "Float")]
    pub score: f32,
    pub feedback: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::assignments::Entity",
        from = "Column::AssignmentId",
        to = "super::assignments::Column::Id",
        on_update = "NoAction",
        on_delete = "Restrict"
    )]
    Assignments,
    #[sea_orm(
        belongs_to = "super::students::Entity",
        from = "Column::StudentId",
        to = "super::students::Column::Id",
        on_update = "NoAction",
        on_delete = "Restrict"
    )]
    Students,
}

impl Related<super::assignments::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Assignments.def()
    }
}

impl Related<super::students::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Students.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}