use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyUserVisitRecord::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyUserVisitRecord::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyUserVisitRecord::Date)
                            .string_len(20)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyUserVisitRecord::Uid)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyUserVisitRecord::VisitType)
                            .integer()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_user_visit_record_date")
                    .table(TyUserVisitRecord::Table)
                    .col(TyUserVisitRecord::Date)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyUserVisitRecord::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyUserVisitRecord {
    #[iden = "ty_user_visit_record"]
    Table,
    Id,
    Date,
    Uid,
    #[iden = "visit_type"]
    VisitType,
}
