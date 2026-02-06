use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyActivityStyle::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyActivityStyle::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyActivityStyle::Name)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyActivityStyle::Type)
                            .boolean()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyActivityStyle::Starttime)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyActivityStyle::Endtime)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyActivityStyle::Style)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyActivityStyle::Status)
                            .boolean()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyActivityStyle::Method)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyActivityStyle::Products)
                            .string_len(500)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyActivityStyle::Createtime)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyActivityStyle::Updatetime)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyActivityStyle::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyActivityStyle {
    #[iden = "ty_activity_style"]
    Table,
    Id,
    Name,
    Type,
    Starttime,
    Endtime,
    Style,
    Status,
    Method,
    Products,
    Createtime,
    Updatetime,
}
