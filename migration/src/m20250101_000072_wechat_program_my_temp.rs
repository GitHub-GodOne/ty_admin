use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyWechatProgramMyTemp::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyWechatProgramMyTemp::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyWechatProgramMyTemp::Tid)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatProgramMyTemp::Title)
                            .string_len(200)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyWechatProgramMyTemp::Kid)
                            .string_len(200)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyWechatProgramMyTemp::SceneDesc)
                            .string_len(500)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyWechatProgramMyTemp::TempId)
                            .string_len(50)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatProgramMyTemp::Extra)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatProgramMyTemp::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TyWechatProgramMyTemp::Type)
                            .string_len(100)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyWechatProgramMyTemp::CreateTime)
                            .timestamp()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyWechatProgramMyTemp::UpdateTime)
                            .timestamp()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyWechatProgramMyTemp::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyWechatProgramMyTemp {
    #[iden = "ty_wechat_program_my_temp"]
    Table,
    Id,
    Tid,
    Title,
    Kid,
    #[iden = "scene_desc"]
    SceneDesc,
    #[iden = "temp_id"]
    TempId,
    Extra,
    Status,
    Type,
    #[iden = "create_time"]
    CreateTime,
    #[iden = "update_time"]
    UpdateTime,
}
