use sea_orm_migration::prelude::*;

use crate::{
    m20231113_170500_c_teachers::Teacher, m20231223_094755_c_classes::Class,
    m20240314_135418_c_grading_rubric::GradingRubrics,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Assignment::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Assignment::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Assignment::Title).string().not_null())
                    .col(ColumnDef::new(Assignment::Description).string().not_null())
                    .col(ColumnDef::new(Assignment::DueDate).timestamp().not_null())
                    .col(
                        ColumnDef::new(Assignment::SubmissionType)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Assignment::GradinRubricId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_assignment_rubric_id")
                            .from(Assignment::Table, Assignment::GradinRubricId)
                            .to(GradingRubrics::Table, GradingRubrics::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Assignment::File).string())
                    .col(ColumnDef::new(Assignment::TeacherId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_assignment_teacher_id")
                            .from(Assignment::Table, Assignment::GradinRubricId)
                            .to(Teacher::Table, Teacher::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ClassAssignment::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ClassAssignment::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ClassAssignment::ClassId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_class_assignments_class_id")
                            .from(ClassAssignment::Table, ClassAssignment::ClassId)
                            .to(Class::Table, Class::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .col(ColumnDef::new(ClassAssignment::AssignmentId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_class_assignments_assignment_id")
                            .from(ClassAssignment::Table, ClassAssignment::ClassId)
                            .to(Class::Table, Class::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                sea_query::ForeignKey::drop()
                    .name("fk_assignment_rubric_id")
                    .table(Assignment::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                sea_query::ForeignKey::drop()
                    .name("fk_class_assignments_class_id")
                    .table(ClassAssignment::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                sea_query::ForeignKey::drop()
                    .name("fk_class_assignments_assignment_id")
                    .table(ClassAssignment::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                sea_query::ForeignKey::drop()
                    .name("fk_assignment_teacher_id")
                    .table(Assignment::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(ClassAssignment::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Assignment::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Assignment {
    #[sea_orm(iden = "assignments")]
    Table,
    Id,
    Title,
    Description,
    DueDate,
    SubmissionType,
    GradinRubricId,
    File,
    TeacherId,
}

#[derive(DeriveIden)]
pub enum ClassAssignment {
    #[sea_orm(iden = "class_assignments")]
    Table,
    Id,
    ClassId,
    AssignmentId,
}
