use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyUserBill::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyUserBill::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyUserBill::Uid)
                            .unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyUserBill::LinkId)
                            .string_len(32)
                            .not_null()
                            .default("0"),
                    )
                    .col(
                        ColumnDef::new(TyUserBill::Pm)
                            .tiny_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyUserBill::Title)
                            .string_len(64)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyUserBill::Category)
                            .string_len(64)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyUserBill::Type)
                            .string_len(64)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyUserBill::Number)
                            .decimal_len(8, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyUserBill::Balance)
                            .decimal_len(16, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .col(
                        ColumnDef::new(TyUserBill::Mark)
                            .string_len(512)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyUserBill::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TyUserBill::CreateTime)
                            .timestamp()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyUserBill::UpdateTime)
                            .timestamp()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_user_bill_openid")
                    .table(TyUserBill::Table)
                    .col(TyUserBill::Uid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_user_bill_status")
                    .table(TyUserBill::Table)
                    .col(TyUserBill::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_user_bill_add_time")
                    .table(TyUserBill::Table)
                    .col(TyUserBill::CreateTime)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_user_bill_pm")
                    .table(TyUserBill::Table)
                    .col(TyUserBill::Pm)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_user_bill_type")
                    .table(TyUserBill::Table)
                    .col(TyUserBill::Category)
                    .col(TyUserBill::Type)
                    .col(TyUserBill::LinkId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyUserBill::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyUserBill {
    #[iden = "ty_user_bill"]
    Table,
    Id,
    Uid,
    #[iden = "link_id"]
    LinkId,
    Pm,
    Title,
    Category,
    Type,
    Number,
    Balance,
    Mark,
    Status,
    #[iden = "create_time"]
    CreateTime,
    #[iden = "update_time"]
    UpdateTime,
}
