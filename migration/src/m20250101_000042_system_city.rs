use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TySystemCity::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TySystemCity::Id)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TySystemCity::CityId)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TySystemCity::Level)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TySystemCity::ParentId)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TySystemCity::AreaCode)
                            .string_len(30)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TySystemCity::Name)
                            .string_len(100)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TySystemCity::MergerName)
                            .string_len(255)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TySystemCity::Lng)
                            .string_len(50)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TySystemCity::Lat)
                            .string_len(50)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TySystemCity::IsShow)
                            .tiny_integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TySystemCity::CreateTime)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TySystemCity::UpdateTime)
                            .timestamp()
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
            .drop_table(Table::drop().table(TySystemCity::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TySystemCity {
    #[iden = "ty_system_city"]
    Table,
    Id,
    #[iden = "city_id"]
    CityId,
    Level,
    #[iden = "parent_id"]
    ParentId,
    #[iden = "area_code"]
    AreaCode,
    Name,
    #[iden = "merger_name"]
    MergerName,
    Lng,
    Lat,
    #[iden = "is_show"]
    IsShow,
    #[iden = "create_time"]
    CreateTime,
    #[iden = "update_time"]
    UpdateTime,
}
