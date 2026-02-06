use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyPageDiy::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyPageDiy::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyPageDiy::Version)
                            .string_len(255)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyPageDiy::Name)
                            .string_len(255)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyPageDiy::Title)
                            .string_len(100)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyPageDiy::CoverImage)
                            .string_len(255)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyPageDiy::TemplateName)
                            .string_len(255)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyPageDiy::Value)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TyPageDiy::AddTime)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyPageDiy::UpdateTime)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyPageDiy::Status)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyPageDiy::Type)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyPageDiy::IsShow)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyPageDiy::IsBgColor)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyPageDiy::IsBgPic)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyPageDiy::IsDiy)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyPageDiy::ColorPicker)
                            .string_len(50)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyPageDiy::BgPic)
                            .string_len(256)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyPageDiy::BgTabVal)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyPageDiy::IsDel)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyPageDiy::ReturnAddress)
                            .string_len(255)
                            .not_null()
                            .default("0"),
                    )
                    .col(
                        ColumnDef::new(TyPageDiy::TitleBgColor)
                            .string_len(255)
                            .not_null()
                            .default("1"),
                    )
                    .col(
                        ColumnDef::new(TyPageDiy::TitleColor)
                            .string_len(255)
                            .not_null()
                            .default("1"),
                    )
                    .col(
                        ColumnDef::new(TyPageDiy::ServiceStatus)
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TyPageDiy::MerId)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyPageDiy::IsDefault)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyPageDiy::TextPosition)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_page_diy_template_name")
                    .table(TyPageDiy::Table)
                    .col(TyPageDiy::TemplateName)
                    .col(TyPageDiy::Type)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_page_diy_status_type")
                    .table(TyPageDiy::Table)
                    .col(TyPageDiy::Status)
                    .col(TyPageDiy::Type)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_page_diy_mer_id")
                    .table(TyPageDiy::Table)
                    .col(TyPageDiy::MerId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyPageDiy::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyPageDiy {
    #[iden = "ty_page_diy"]
    Table,
    Id,
    Version,
    Name,
    Title,
    #[iden = "cover_image"]
    CoverImage,
    #[iden = "template_name"]
    TemplateName,
    Value,
    #[iden = "add_time"]
    AddTime,
    #[iden = "update_time"]
    UpdateTime,
    Status,
    Type,
    #[iden = "is_show"]
    IsShow,
    #[iden = "is_bg_color"]
    IsBgColor,
    #[iden = "is_bg_pic"]
    IsBgPic,
    #[iden = "is_diy"]
    IsDiy,
    #[iden = "color_picker"]
    ColorPicker,
    #[iden = "bg_pic"]
    BgPic,
    #[iden = "bg_tab_val"]
    BgTabVal,
    #[iden = "is_del"]
    IsDel,
    #[iden = "return_address"]
    ReturnAddress,
    #[iden = "title_bg_color"]
    TitleBgColor,
    #[iden = "title_color"]
    TitleColor,
    #[iden = "service_status"]
    ServiceStatus,
    #[iden = "mer_id"]
    MerId,
    #[iden = "is_default"]
    IsDefault,
    #[iden = "text_position"]
    TextPosition,
}
