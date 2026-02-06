use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyExpress::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyExpress::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyExpress::Code)
                            .string_len(50)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyExpress::Name)
                            .string_len(50)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyExpress::PartnerId)
                            .tiny_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyExpress::PartnerKey)
                            .tiny_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyExpress::Net)
                            .tiny_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyExpress::Account)
                            .string_len(100)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyExpress::Password)
                            .string_len(100)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyExpress::NetName)
                            .string_len(100)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyExpress::Sort)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyExpress::IsShow)
                            .tiny_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyExpress::Status)
                            .tiny_integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_express_is_show")
                    .table(TyExpress::Table)
                    .col(TyExpress::IsShow)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_express_code")
                    .table(TyExpress::Table)
                    .col(TyExpress::Code)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyExpress::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyExpress {
    #[iden = "ty_express"]
    Table,
    Id,
    Code,
    Name,
    #[iden = "partner_id"]
    PartnerId,
    #[iden = "partner_key"]
    PartnerKey,
    Net,
    Account,
    Password,
    #[iden = "net_name"]
    NetName,
    Sort,
    #[iden = "is_show"]
    IsShow,
    Status,
}
