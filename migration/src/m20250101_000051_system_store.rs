use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TySystemStore::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TySystemStore::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TySystemStore::Name)
                            .string_len(100)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TySystemStore::Introduction)
                            .string_len(1000)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TySystemStore::Phone)
                            .char_len(25)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TySystemStore::Address)
                            .string_len(255)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TySystemStore::DetailedAddress)
                            .string_len(255)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TySystemStore::Image)
                            .string_len(255)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TySystemStore::Latitude)
                            .char_len(25)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TySystemStore::Longitude)
                            .char_len(25)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TySystemStore::ValidTime)
                            .string_len(100)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TySystemStore::DayTime)
                            .string_len(100)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TySystemStore::IsShow)
                            .tiny_integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TySystemStore::IsDel)
                            .tiny_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TySystemStore::CreateTime)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TySystemStore::UpdateTime)
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
                    .name("idx_ty_system_store_phone")
                    .table(TySystemStore::Table)
                    .col(TySystemStore::Phone)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TySystemStore::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TySystemStore {
    #[iden = "ty_system_store"]
    Table,
    Id,
    Name,
    Introduction,
    Phone,
    Address,
    #[iden = "detailed_address"]
    DetailedAddress,
    Image,
    Latitude,
    Longitude,
    #[iden = "valid_time"]
    ValidTime,
    #[iden = "day_time"]
    DayTime,
    #[iden = "is_show"]
    IsShow,
    #[iden = "is_del"]
    IsDel,
    #[iden = "create_time"]
    CreateTime,
    #[iden = "update_time"]
    UpdateTime,
}
