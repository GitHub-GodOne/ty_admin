use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyUserToken::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyUserToken::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyUserToken::Uid)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyUserToken::Token)
                            .string_len(255)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyUserToken::Type)
                            .tiny_integer()
                            .null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TyUserToken::CreateTime)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyUserToken::ExpiresTime)
                            .date_time()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyUserToken::LoginIp)
                            .string_len(32)
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_user_token_type+token")
                    .table(TyUserToken::Table)
                    .col(TyUserToken::Type)
                    .col(TyUserToken::Token)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyUserToken::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyUserToken {
    #[iden = "ty_user_token"]
    Table,
    Id,
    Uid,
    Token,
    Type,
    #[iden = "create_time"]
    CreateTime,
    #[iden = "expires_time"]
    ExpiresTime,
    #[iden = "login_ip"]
    LoginIp,
}
