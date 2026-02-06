use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TyUserExperienceRecord::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TyUserExperienceRecord::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TyUserExperienceRecord::Uid)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyUserExperienceRecord::LinkId)
                            .string_len(32)
                            .not_null()
                            .default("0"),
                    )
                    .col(
                        ColumnDef::new(TyUserExperienceRecord::LinkType)
                            .string_len(32)
                            .not_null()
                            .default("ORDER"),
                    )
                    .col(
                        ColumnDef::new(TyUserExperienceRecord::Type)
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TyUserExperienceRecord::Title)
                            .string_len(64)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyUserExperienceRecord::Experience)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyUserExperienceRecord::Balance)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TyUserExperienceRecord::Mark)
                            .string_len(512)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(TyUserExperienceRecord::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(TyUserExperienceRecord::CreateTime)
                            .timestamp()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TyUserExperienceRecord::UpdateTime)
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
                    .name("idx_ty_user_experience_record_openid")
                    .table(TyUserExperienceRecord::Table)
                    .col(TyUserExperienceRecord::Uid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_user_experience_record_status")
                    .table(TyUserExperienceRecord::Table)
                    .col(TyUserExperienceRecord::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_user_experience_record_add_time")
                    .table(TyUserExperienceRecord::Table)
                    .col(TyUserExperienceRecord::CreateTime)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_user_experience_record_type")
                    .table(TyUserExperienceRecord::Table)
                    .col(TyUserExperienceRecord::Type)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ty_user_experience_record_type_link")
                    .table(TyUserExperienceRecord::Table)
                    .col(TyUserExperienceRecord::Type)
                    .col(TyUserExperienceRecord::LinkId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TyUserExperienceRecord::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TyUserExperienceRecord {
    #[iden = "ty_user_experience_record"]
    Table,
    Id,
    Uid,
    #[iden = "link_id"]
    LinkId,
    #[iden = "link_type"]
    LinkType,
    Type,
    Title,
    Experience,
    Balance,
    Mark,
    Status,
    #[iden = "create_time"]
    CreateTime,
    #[iden = "update_time"]
    UpdateTime,
}
