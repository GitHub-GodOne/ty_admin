use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyUserSign::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyUserSign::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyUserSign::Uid)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyUserSign::Title)
                            .string_len(255)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyUserSign::Number)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyUserSign::Balance)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyUserSign::Type)
                            .tiny_integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TyUserSign::CreateDay)
                            .date()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyUserSign::CreateTime)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_user_sign_uid")
                    .table(TyUserSign::Table)
                    .col(TyUserSign::Uid)
                    .col(TyUserSign::Type)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyUserSign::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyUserSign {
    #[iden = "ty_user_sign"]
    Table,
    Id,
    Uid,
    Title,
    Number,
    Balance,
    Type,
    #[iden = "create_day"]
    CreateDay,
    #[iden = "create_time"]
    CreateTime,
}
