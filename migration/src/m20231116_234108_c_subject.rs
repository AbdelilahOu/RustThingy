use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Subject::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Subject::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Subject::Name).string().not_null())
                    .col(ColumnDef::new(Subject::Description).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Subject::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Subject {
    #[sea_orm(iden = "subjects")]
    Table,
    Id,
    #[sea_orm(iden = "subject_name")]
    Name,
    #[sea_orm(iden = "description")]
    Description,
}
