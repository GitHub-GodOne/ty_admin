use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TySystemAttachment::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TySystemAttachment::AttId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TySystemAttachment::Name)
                            .string_len(100)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TySystemAttachment::AttDir)
                            .string_len(200)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TySystemAttachment::SattDir)
                            .string_len(200)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TySystemAttachment::AttSize)
                            .char_len(30)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TySystemAttachment::AttType)
                            .char_len(30)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TySystemAttachment::Pid)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TySystemAttachment::ImageType)
                            .tiny_unsigned()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TySystemAttachment::CreateTime)
                            .timestamp()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TySystemAttachment::UpdateTime)
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
            .drop_table(Table::drop().table(TySystemAttachment::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TySystemAttachment {
    #[iden = "ty_system_attachment"]
    Table,
    #[iden = "att_id"]
    AttId,
    Name,
    #[iden = "att_dir"]
    AttDir,
    #[iden = "satt_dir"]
    SattDir,
    #[iden = "att_size"]
    AttSize,
    #[iden = "att_type"]
    AttType,
    Pid,
    #[iden = "image_type"]
    ImageType,
    #[iden = "create_time"]
    CreateTime,
    #[iden = "update_time"]
    UpdateTime,
}
