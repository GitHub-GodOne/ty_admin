use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyStoreSeckillManger::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyStoreSeckillManger::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckillManger::Name)
                            .string_len(255)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckillManger::StartTime)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckillManger::EndTime)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckillManger::Img)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckillManger::SilderImgs)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckillManger::Sort)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckillManger::Status)
                            .string_len(11)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckillManger::CreateTime)
                            .date_time()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckillManger::UpdateTime)
                            .date_time()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyStoreSeckillManger::IsDel)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyStoreSeckillManger::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyStoreSeckillManger {
    #[iden = "ty_store_seckill_manger"]
    Table,
    Id,
    Name,
    #[iden = "start_time"]
    StartTime,
    #[iden = "end_time"]
    EndTime,
    Img,
    #[iden = "silder_imgs"]
    SilderImgs,
    Sort,
    Status,
    #[iden = "create_time"]
    CreateTime,
    #[iden = "update_time"]
    UpdateTime,
    #[iden = "is_del"]
    IsDel,
}
