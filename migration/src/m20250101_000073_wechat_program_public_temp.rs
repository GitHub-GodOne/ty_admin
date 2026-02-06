use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyWechatProgramPublicTemp::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyWechatProgramPublicTemp::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyWechatProgramPublicTemp::Tid)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatProgramPublicTemp::Title)
                            .string_len(200)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyWechatProgramPublicTemp::Type)
                            .tiny_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatProgramPublicTemp::CategoryId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatProgramPublicTemp::CreateTime)
                            .timestamp()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyWechatProgramPublicTemp::UpdateTime)
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
                    .name("idx_ty_wechat_program_public_temp_tid")
                    .table(TyWechatProgramPublicTemp::Table)
                    .col(TyWechatProgramPublicTemp::Tid)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyWechatProgramPublicTemp::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyWechatProgramPublicTemp {
    #[iden = "ty_wechat_program_public_temp"]
    Table,
    Id,
    Tid,
    Title,
    Type,
    #[iden = "category_id"]
    CategoryId,
    #[iden = "create_time"]
    CreateTime,
    #[iden = "update_time"]
    UpdateTime,
}
