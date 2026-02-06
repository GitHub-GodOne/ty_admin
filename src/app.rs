use async_trait::async_trait;
use loco_rs::{
    app::{AppContext, Hooks, Initializer},
    bgworker::{BackgroundWorker, Queue},
    boot::{create_app, BootResult, StartMode},
    config::Config,
    controller::AppRoutes,
    db::{self, truncate_table},
    environment::Environment,
    task::Tasks,
    Result,
};
use migration::Migrator;
use std::path::Path;

#[allow(unused_imports)]
use crate::{
    controllers,
    models::_entities::{
        users,
        activity_style,
        article,
        category,
        express,
        page_diy,
        schedule_job,
        schedule_job_log,
        shipping_templates,
        shipping_templates_region,
        sms_template,
        store_bargain,
        store_bargain_user,
        store_bargain_user_help,
        store_combination,
        store_coupon,
        store_coupon_user,
        store_order,
        store_order_info,
        store_order_status,
        store_pink,
        store_product,
        store_product_attr,
        store_product_attr_value,
        store_product_description,
        store_product_reply,
        store_product_rule,
        store_seckill,
        store_seckill_manger,
        system_admin,
        system_attachment,
        system_city,
        system_config,
        system_form_temp,
        system_group,
        system_group_data,
        system_menu,
        system_notification,
        system_role,
        system_role_menu,
        system_store,
        system_user_level,
        template_message,
        user,
        user_address,
        user_bill,
        user_experience_record,
        user_group,
        user_integral_record,
        user_level,
        user_sign,
        user_tag,
        wechat_program_public_temp,
        wechat_reply,
    },
    tasks,
    workers::downloader::DownloadWorker,
};

pub struct App;
#[async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    async fn boot(
        mode: StartMode,
        environment: &Environment,
        config: Config,
    ) -> Result<BootResult> {
        create_app::<Self, Migrator>(mode, environment, config).await
    }

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![])
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes() // controller routes below
            .add_route(controllers::auth::routes())
    }
    async fn connect_workers(ctx: &AppContext, queue: &Queue) -> Result<()> {
        queue.register(DownloadWorker::build(ctx)).await?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn register_tasks(tasks: &mut Tasks) {
        // tasks-inject (do not remove)
    }
    async fn truncate(ctx: &AppContext) -> Result<()> {
        truncate_table(&ctx.db, users::Entity).await?;
        Ok(())
    }
    async fn seed(ctx: &AppContext, base: &Path) -> Result<()> {
        db::seed::<users::ActiveModel>(&ctx.db, &base.join("users.yaml").display().to_string())
            .await?;
        db::seed::<activity_style::ActiveModel>(&ctx.db, &base.join("activity_style.yaml").display().to_string())
            .await?;
        db::seed::<article::ActiveModel>(&ctx.db, &base.join("article.yaml").display().to_string())
            .await?;
        db::seed::<category::ActiveModel>(&ctx.db, &base.join("category.yaml").display().to_string())
            .await?;
        db::seed::<express::ActiveModel>(&ctx.db, &base.join("express.yaml").display().to_string())
            .await?;
        db::seed::<page_diy::ActiveModel>(&ctx.db, &base.join("page_diy.yaml").display().to_string())
            .await?;
        db::seed::<schedule_job::ActiveModel>(&ctx.db, &base.join("schedule_job.yaml").display().to_string())
            .await?;
        db::seed::<schedule_job_log::ActiveModel>(&ctx.db, &base.join("schedule_job_log.yaml").display().to_string())
            .await?;
        db::seed::<shipping_templates::ActiveModel>(&ctx.db, &base.join("shipping_templates.yaml").display().to_string())
            .await?;
        db::seed::<shipping_templates_region::ActiveModel>(&ctx.db, &base.join("shipping_templates_region.yaml").display().to_string())
            .await?;
        db::seed::<sms_template::ActiveModel>(&ctx.db, &base.join("sms_template.yaml").display().to_string())
            .await?;
        db::seed::<store_bargain::ActiveModel>(&ctx.db, &base.join("store_bargain.yaml").display().to_string())
            .await?;
        db::seed::<store_bargain_user::ActiveModel>(&ctx.db, &base.join("store_bargain_user.yaml").display().to_string())
            .await?;
        db::seed::<store_bargain_user_help::ActiveModel>(&ctx.db, &base.join("store_bargain_user_help.yaml").display().to_string())
            .await?;
        db::seed::<store_combination::ActiveModel>(&ctx.db, &base.join("store_combination.yaml").display().to_string())
            .await?;
        db::seed::<store_coupon::ActiveModel>(&ctx.db, &base.join("store_coupon.yaml").display().to_string())
            .await?;
        db::seed::<store_coupon_user::ActiveModel>(&ctx.db, &base.join("store_coupon_user.yaml").display().to_string())
            .await?;
        db::seed::<store_order::ActiveModel>(&ctx.db, &base.join("store_order.yaml").display().to_string())
            .await?;
        db::seed::<store_order_info::ActiveModel>(&ctx.db, &base.join("store_order_info.yaml").display().to_string())
            .await?;
        db::seed::<store_order_status::ActiveModel>(&ctx.db, &base.join("store_order_status.yaml").display().to_string())
            .await?;
        db::seed::<store_pink::ActiveModel>(&ctx.db, &base.join("store_pink.yaml").display().to_string())
            .await?;
        db::seed::<store_product::ActiveModel>(&ctx.db, &base.join("store_product.yaml").display().to_string())
            .await?;
        db::seed::<store_product_attr::ActiveModel>(&ctx.db, &base.join("store_product_attr.yaml").display().to_string())
            .await?;
        db::seed::<store_product_attr_value::ActiveModel>(&ctx.db, &base.join("store_product_attr_value.yaml").display().to_string())
            .await?;
        db::seed::<store_product_description::ActiveModel>(&ctx.db, &base.join("store_product_description.yaml").display().to_string())
            .await?;
        db::seed::<store_product_reply::ActiveModel>(&ctx.db, &base.join("store_product_reply.yaml").display().to_string())
            .await?;
        db::seed::<store_product_rule::ActiveModel>(&ctx.db, &base.join("store_product_rule.yaml").display().to_string())
            .await?;
        db::seed::<store_seckill::ActiveModel>(&ctx.db, &base.join("store_seckill.yaml").display().to_string())
            .await?;
        db::seed::<store_seckill_manger::ActiveModel>(&ctx.db, &base.join("store_seckill_manger.yaml").display().to_string())
            .await?;
        // db::seed::<system_admin::ActiveModel>(&ctx.db, &base.join("system_admin.yaml").display().to_string())
        //     .await?;
        db::seed::<system_attachment::ActiveModel>(&ctx.db, &base.join("system_attachment.yaml").display().to_string())
            .await?;
        db::seed::<system_city::ActiveModel>(&ctx.db, &base.join("system_city.yaml").display().to_string())
            .await?;
        db::seed::<system_config::ActiveModel>(&ctx.db, &base.join("system_config.yaml").display().to_string())
            .await?;
        db::seed::<system_form_temp::ActiveModel>(&ctx.db, &base.join("system_form_temp.yaml").display().to_string())
            .await?;
        db::seed::<system_group::ActiveModel>(&ctx.db, &base.join("system_group.yaml").display().to_string())
            .await?;
        db::seed::<system_group_data::ActiveModel>(&ctx.db, &base.join("system_group_data.yaml").display().to_string())
            .await?;
        db::seed::<system_menu::ActiveModel>(&ctx.db, &base.join("system_menu.yaml").display().to_string())
            .await?;
        db::seed::<system_notification::ActiveModel>(&ctx.db, &base.join("system_notification.yaml").display().to_string())
            .await?;
        db::seed::<system_role::ActiveModel>(&ctx.db, &base.join("system_role.yaml").display().to_string())
            .await?;
        db::seed::<system_role_menu::ActiveModel>(&ctx.db, &base.join("system_role_menu.yaml").display().to_string())
            .await?;
        db::seed::<system_store::ActiveModel>(&ctx.db, &base.join("system_store.yaml").display().to_string())
            .await?;
        // db::seed::<system_user_level::ActiveModel>(&ctx.db, &base.join("system_user_level.yaml").display().to_string())
        //     .await?;
        db::seed::<template_message::ActiveModel>(&ctx.db, &base.join("template_message.yaml").display().to_string())
            .await?;
        // db::seed::<user::ActiveModel>(&ctx.db, &base.join("user.yaml").display().to_string())
        //     .await?;
        db::seed::<user_address::ActiveModel>(&ctx.db, &base.join("user_address.yaml").display().to_string())
            .await?;
        db::seed::<user_bill::ActiveModel>(&ctx.db, &base.join("user_bill.yaml").display().to_string())
            .await?;
        db::seed::<user_experience_record::ActiveModel>(&ctx.db, &base.join("user_experience_record.yaml").display().to_string())
            .await?;
        // db::seed::<user_group::ActiveModel>(&ctx.db, &base.join("user_group.yaml").display().to_string())
        //     .await?;
        db::seed::<user_integral_record::ActiveModel>(&ctx.db, &base.join("user_integral_record.yaml").display().to_string())
            .await?;
        db::seed::<user_level::ActiveModel>(&ctx.db, &base.join("user_level.yaml").display().to_string())
            .await?;
        db::seed::<user_sign::ActiveModel>(&ctx.db, &base.join("user_sign.yaml").display().to_string())
            .await?;
        // db::seed::<user_tag::ActiveModel>(&ctx.db, &base.join("user_tag.yaml").display().to_string())
        //     .await?;
        db::seed::<wechat_program_public_temp::ActiveModel>(&ctx.db, &base.join("wechat_program_public_temp.yaml").display().to_string())
            .await?;
        db::seed::<wechat_reply::ActiveModel>(&ctx.db, &base.join("wechat_reply.yaml").display().to_string())
            .await?;
        Ok(())
    }
}
