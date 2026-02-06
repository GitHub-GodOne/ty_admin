use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyUserAddress::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyUserAddress::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyUserAddress::Uid)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TyUserAddress::RealName)
                            .string_len(32)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyUserAddress::Phone)
                            .string_len(16)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyUserAddress::Province)
                            .string_len(64)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyUserAddress::City)
                            .string_len(64)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyUserAddress::CityId)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyUserAddress::District)
                            .string_len(64)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyUserAddress::Detail)
                            .string_len(256)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyUserAddress::PostCode)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyUserAddress::Longitude)
                            .string_len(16)
                            .not_null()
                            .default("0"),
                    )
                    .col(
                        ColumnDef::new(TyUserAddress::Latitude)
                            .string_len(16)
                            .not_null()
                            .default("0"),
                    )
                    .col(
                        ColumnDef::new(TyUserAddress::IsDefault)
                            .tiny_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyUserAddress::IsDel)
                            .tiny_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyUserAddress::CreateTime)
                            .timestamp()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyUserAddress::UpdateTime)
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
                    .name("idx_ty_user_address_uid")
                    .table(TyUserAddress::Table)
                    .col(TyUserAddress::Uid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_user_address_is_default")
                    .table(TyUserAddress::Table)
                    .col(TyUserAddress::IsDefault)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_user_address_is_del")
                    .table(TyUserAddress::Table)
                    .col(TyUserAddress::IsDel)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyUserAddress::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyUserAddress {
    #[iden = "ty_user_address"]
    Table,
    Id,
    Uid,
    #[iden = "real_name"]
    RealName,
    Phone,
    Province,
    City,
    #[iden = "city_id"]
    CityId,
    District,
    Detail,
    #[iden = "post_code"]
    PostCode,
    Longitude,
    Latitude,
    #[iden = "is_default"]
    IsDefault,
    #[iden = "is_del"]
    IsDel,
    #[iden = "create_time"]
    CreateTime,
    #[iden = "update_time"]
    UpdateTime,
}
