#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users;
mod m20250101_000001_activity_style;
mod m20250101_000002_article;
mod m20250101_000003_category;
mod m20250101_000004_exception_log;
mod m20250101_000005_express;
mod m20250101_000006_page_category;
mod m20250101_000007_page_diy;
mod m20250101_000008_page_link;
mod m20250101_000009_schedule_job;
mod m20250101_000010_schedule_job_log;
mod m20250101_000011_shipping_templates;
mod m20250101_000012_shipping_templates_free;
mod m20250101_000013_shipping_templates_region;
mod m20250101_000014_sms_record;
mod m20250101_000015_sms_template;
mod m20250101_000016_store_bargain;
mod m20250101_000017_store_bargain_user;
mod m20250101_000018_store_bargain_user_help;
mod m20250101_000019_store_cart;
mod m20250101_000020_store_combination;
mod m20250101_000021_store_coupon;
mod m20250101_000022_store_coupon_user;
mod m20250101_000023_store_order;
mod m20250101_000024_store_order_info;
mod m20250101_000025_store_order_status;
mod m20250101_000026_store_pink;
mod m20250101_000027_store_product;
mod m20250101_000028_store_product_attr;
mod m20250101_000029_store_product_attr_result;
mod m20250101_000030_store_product_attr_value;
mod m20250101_000031_store_product_cate;
mod m20250101_000032_store_product_coupon;
mod m20250101_000033_store_product_description;
mod m20250101_000034_store_product_log;
mod m20250101_000035_store_product_relation;
mod m20250101_000036_store_product_reply;
mod m20250101_000037_store_product_rule;
mod m20250101_000038_store_seckill;
mod m20250101_000039_store_seckill_manger;
mod m20250101_000040_system_admin;
mod m20250101_000041_system_attachment;
mod m20250101_000042_system_city;
mod m20250101_000043_system_config;
mod m20250101_000044_system_form_temp;
mod m20250101_000045_system_group;
mod m20250101_000046_system_group_data;
mod m20250101_000047_system_menu;
mod m20250101_000048_system_notification;
mod m20250101_000049_system_role;
mod m20250101_000050_system_role_menu;
mod m20250101_000051_system_store;
mod m20250101_000052_system_store_staff;
mod m20250101_000053_system_user_level;
mod m20250101_000054_template_message;
mod m20250101_000055_user;
mod m20250101_000056_user_address;
mod m20250101_000057_user_bill;
mod m20250101_000058_user_brokerage_record;
mod m20250101_000059_user_experience_record;
mod m20250101_000060_user_extract;
mod m20250101_000061_user_group;
mod m20250101_000062_user_integral_record;
mod m20250101_000063_user_level;
mod m20250101_000064_user_recharge;
mod m20250101_000065_user_sign;
mod m20250101_000066_user_tag;
mod m20250101_000067_user_token;
mod m20250101_000068_user_visit_record;
mod m20250101_000069_wechat_callback;
mod m20250101_000070_wechat_exceptions;
mod m20250101_000071_wechat_pay_info;
mod m20250101_000072_wechat_program_my_temp;
mod m20250101_000073_wechat_program_public_temp;
mod m20250101_000074_wechat_reply;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20250101_000001_activity_style::Migration),
            Box::new(m20250101_000002_article::Migration),
            Box::new(m20250101_000003_category::Migration),
            Box::new(m20250101_000004_exception_log::Migration),
            Box::new(m20250101_000005_express::Migration),
            Box::new(m20250101_000006_page_category::Migration),
            Box::new(m20250101_000007_page_diy::Migration),
            Box::new(m20250101_000008_page_link::Migration),
            Box::new(m20250101_000009_schedule_job::Migration),
            Box::new(m20250101_000010_schedule_job_log::Migration),
            Box::new(m20250101_000011_shipping_templates::Migration),
            Box::new(m20250101_000012_shipping_templates_free::Migration),
            Box::new(m20250101_000013_shipping_templates_region::Migration),
            Box::new(m20250101_000014_sms_record::Migration),
            Box::new(m20250101_000015_sms_template::Migration),
            Box::new(m20250101_000016_store_bargain::Migration),
            Box::new(m20250101_000017_store_bargain_user::Migration),
            Box::new(m20250101_000018_store_bargain_user_help::Migration),
            Box::new(m20250101_000019_store_cart::Migration),
            Box::new(m20250101_000020_store_combination::Migration),
            Box::new(m20250101_000021_store_coupon::Migration),
            Box::new(m20250101_000022_store_coupon_user::Migration),
            Box::new(m20250101_000023_store_order::Migration),
            Box::new(m20250101_000024_store_order_info::Migration),
            Box::new(m20250101_000025_store_order_status::Migration),
            Box::new(m20250101_000026_store_pink::Migration),
            Box::new(m20250101_000027_store_product::Migration),
            Box::new(m20250101_000028_store_product_attr::Migration),
            Box::new(m20250101_000029_store_product_attr_result::Migration),
            Box::new(m20250101_000030_store_product_attr_value::Migration),
            Box::new(m20250101_000031_store_product_cate::Migration),
            Box::new(m20250101_000032_store_product_coupon::Migration),
            Box::new(m20250101_000033_store_product_description::Migration),
            Box::new(m20250101_000034_store_product_log::Migration),
            Box::new(m20250101_000035_store_product_relation::Migration),
            Box::new(m20250101_000036_store_product_reply::Migration),
            Box::new(m20250101_000037_store_product_rule::Migration),
            Box::new(m20250101_000038_store_seckill::Migration),
            Box::new(m20250101_000039_store_seckill_manger::Migration),
            Box::new(m20250101_000040_system_admin::Migration),
            Box::new(m20250101_000041_system_attachment::Migration),
            Box::new(m20250101_000042_system_city::Migration),
            Box::new(m20250101_000043_system_config::Migration),
            Box::new(m20250101_000044_system_form_temp::Migration),
            Box::new(m20250101_000045_system_group::Migration),
            Box::new(m20250101_000046_system_group_data::Migration),
            Box::new(m20250101_000047_system_menu::Migration),
            Box::new(m20250101_000048_system_notification::Migration),
            Box::new(m20250101_000049_system_role::Migration),
            Box::new(m20250101_000050_system_role_menu::Migration),
            Box::new(m20250101_000051_system_store::Migration),
            Box::new(m20250101_000052_system_store_staff::Migration),
            Box::new(m20250101_000053_system_user_level::Migration),
            Box::new(m20250101_000054_template_message::Migration),
            Box::new(m20250101_000055_user::Migration),
            Box::new(m20250101_000056_user_address::Migration),
            Box::new(m20250101_000057_user_bill::Migration),
            Box::new(m20250101_000058_user_brokerage_record::Migration),
            Box::new(m20250101_000059_user_experience_record::Migration),
            Box::new(m20250101_000060_user_extract::Migration),
            Box::new(m20250101_000061_user_group::Migration),
            Box::new(m20250101_000062_user_integral_record::Migration),
            Box::new(m20250101_000063_user_level::Migration),
            Box::new(m20250101_000064_user_recharge::Migration),
            Box::new(m20250101_000065_user_sign::Migration),
            Box::new(m20250101_000066_user_tag::Migration),
            Box::new(m20250101_000067_user_token::Migration),
            Box::new(m20250101_000068_user_visit_record::Migration),
            Box::new(m20250101_000069_wechat_callback::Migration),
            Box::new(m20250101_000070_wechat_exceptions::Migration),
            Box::new(m20250101_000071_wechat_pay_info::Migration),
            Box::new(m20250101_000072_wechat_program_my_temp::Migration),
            Box::new(m20250101_000073_wechat_program_public_temp::Migration),
            Box::new(m20250101_000074_wechat_reply::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}
