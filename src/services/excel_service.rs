/// Excel导出服务
///
/// Java参考: ExcelServiceImpl
use loco_rs::prelude::*;
use chrono::Local;
use rust_xlsxwriter::{Workbook, Format};
use std::collections::HashMap;

use crate::dtos::common::PageParamRequest;
use crate::dtos::product::StoreProductSearchRequest;
use crate::dtos::store_order::StoreOrderSearchRequest;
use crate::dtos::store_bargain::StoreBargainSearchRequest;
use crate::dtos::store_combination::StoreCombinationSearchRequest;
use crate::dtos::excel::*;
use crate::services::store_product_service::StoreProductService;
use crate::services::store_order_service::StoreOrderService;
use crate::services::store_bargain_service::StoreBargainService;
use crate::services::store_combination_service::StoreCombinationService;
use crate::services::category_service::CategoryService;

/// 导出最大数量 (Java: Constants.EXPORT_MAX_LIMIT)
const EXPORT_MAX_LIMIT: i32 = 99999;

pub struct ExcelService;

impl ExcelService {
    /// 导出商品Excel
    ///
    /// Java参考: ExcelServiceImpl.exportProduct()
    pub async fn export_product(
        db: &sea_orm::DatabaseConnection,
        request: &StoreProductSearchRequest,
    ) -> Result<String> {
        let page_param = PageParamRequest {
            page: Some(1),
            limit: Some(EXPORT_MAX_LIMIT),
        };
        let page_data = StoreProductService::get_admin_list(db, request, &page_param).await?;
        if page_data.list.is_empty() {
            return Err(Error::string("没有可导出的数据"));
        }

        // 收集所有分类ID，批量查询分类名称
        let mut cate_ids: Vec<i32> = Vec::new();
        for item in &page_data.list {
            if let Some(cate_id_str) = &item.cate_id {
                for id_str in cate_id_str.split(',') {
                    if let Ok(id) = id_str.trim().parse::<i32>() {
                        if !cate_ids.contains(&id) {
                            cate_ids.push(id);
                        }
                    }
                }
            }
        }
        let categories = CategoryService::get_by_ids(db, cate_ids).await.unwrap_or_default();
        let cate_map: HashMap<i32, String> = categories.into_iter()
            .map(|c| (c.id, c.name))
            .collect();

        // 转换为Excel VO
        let vo_list: Vec<ProductExcelVo> = page_data.list.iter().map(|item| {
            // 解析分类名称
            let cate_name = item.cate_id.as_ref().map(|cid| {
                cid.split(',')
                    .filter_map(|s| s.trim().parse::<i32>().ok())
                    .filter_map(|id| cate_map.get(&id))
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(",")
            }).unwrap_or_default();

            ProductExcelVo {
                store_name: item.store_name.clone(),
                cate_name,
                price: format!("￥{}", item.price),
                stock: item.stock.to_string(),
                sales: item.sales.to_string(),
                browse: item.browse.unwrap_or(0).to_string(),
            }
        }).collect();

        // 列别名映射
        let aliases = vec![
            ("storeName", "商品名称"),
            ("cateName", "商品分类"),
            ("price", "价格"),
            ("stock", "库存"),
            ("sales", "销量"),
            ("browse", "浏览量"),
        ];

        // 生成Excel
        let file_name = Self::generate_file_name("商品导出");
        Self::write_excel(&file_name, "商品导出", &aliases, &vo_list, |vo| {
            vec![
                vo.store_name.clone(),
                vo.cate_name.clone(),
                vo.price.clone(),
                vo.stock.clone(),
                vo.sales.clone(),
                vo.browse.clone(),
            ]
        })?;

        Ok(file_name)
    }

    /// 导出砍价商品Excel
    ///
    /// Java参考: ExcelServiceImpl.exportBargainProduct()
    pub async fn export_bargain_product(
        db: &sea_orm::DatabaseConnection,
        request: &StoreBargainSearchRequest,
    ) -> Result<String> {
        let page_param = PageParamRequest {
            page: Some(1),
            limit: Some(EXPORT_MAX_LIMIT),
        };
        let page_data = StoreBargainService::get_list(db, request, &page_param).await?;
        if page_data.list.is_empty() {
            return Err(Error::string("没有可导出的数据"));
        }

        let vo_list: Vec<BargainProductExcelVo> = page_data.list.iter().map(|item| {
            BargainProductExcelVo {
                title: item.title.clone(),
                price: item.price.map(|p| format!("￥{}", p)).unwrap_or_default(),
                bargain_num: item.bargain_num.to_string(),
                status: if item.status == 1 { "开启".to_string() } else { "关闭".to_string() },
                start_time: item.start_time.clone(),
                stop_time: item.stop_time.clone(),
                sales: item.sales.unwrap_or(0).to_string(),
                quota_show: item.quota_show.to_string(),
                give_integral: item.give_integral.unwrap_or(0).to_string(),
                add_time: item.add_time.clone().unwrap_or_default(),
            }
        }).collect();

        let aliases = vec![
            ("title", "砍价活动名称"),
            ("price", "砍价金额"),
            ("bargainNum", "用户每次砍价的次数"),
            ("status", "砍价状态"),
            ("startTime", "砍价开启时间"),
            ("stopTime", "砍价结束时间"),
            ("sales", "销量"),
            ("quotaShow", "库存"),
            ("giveIntegral", "返多少积分"),
            ("addTime", "添加时间"),
        ];

        let file_name = Self::generate_file_name("砍价");
        Self::write_excel(&file_name, "砍价商品", &aliases, &vo_list, |vo| {
            vec![
                vo.title.clone(),
                vo.price.clone(),
                vo.bargain_num.clone(),
                vo.status.clone(),
                vo.start_time.clone(),
                vo.stop_time.clone(),
                vo.sales.clone(),
                vo.quota_show.clone(),
                vo.give_integral.clone(),
                vo.add_time.clone(),
            ]
        })?;

        Ok(file_name)
    }

    /// 导出拼团商品Excel
    ///
    /// Java参考: ExcelServiceImpl.exportCombinationProduct()
    pub async fn export_combination_product(
        db: &sea_orm::DatabaseConnection,
        request: &StoreCombinationSearchRequest,
    ) -> Result<String> {
        let page_param = PageParamRequest {
            page: Some(1),
            limit: Some(EXPORT_MAX_LIMIT),
        };
        let page_data = StoreCombinationService::get_list(db, request, &page_param).await?;
        if page_data.list.is_empty() {
            return Err(Error::string("没有可导出的数据"));
        }

        let vo_list: Vec<CombinationProductExcelVo> = page_data.list.iter().map(|item| {
            CombinationProductExcelVo {
                id: item.id.to_string(),
                title: item.title.clone(),
                ot_price: item.ot_price.to_string(),
                price: item.price.to_string(),
                quota_show: item.quota_show.to_string(),
                count_people: item.people.to_string(),
                count_people_all: item.count_people_all.to_string(),
                count_people_pink: item.count_people_pink.to_string(),
                sales: item.sales.to_string(),
                is_show: if item.is_show { "开启".to_string() } else { "关闭".to_string() },
                stop_time: item.stop_time.clone(),
            }
        }).collect();

        let aliases = vec![
            ("id", "编号"),
            ("title", "拼团名称"),
            ("otPrice", "原价"),
            ("price", "拼团价"),
            ("quotaShow", "库存"),
            ("countPeople", "拼团人数"),
            ("countPeopleAll", "参与人数"),
            ("countPeoplePink", "成团数量"),
            ("sales", "销量"),
            ("isShow", "商品状态"),
            ("stopTime", "拼团结束时间"),
        ];

        let file_name = Self::generate_file_name("拼团");
        Self::write_excel(&file_name, "拼团商品", &aliases, &vo_list, |vo| {
            vec![
                vo.id.clone(),
                vo.title.clone(),
                vo.ot_price.clone(),
                vo.price.clone(),
                vo.quota_show.clone(),
                vo.count_people.clone(),
                vo.count_people_all.clone(),
                vo.count_people_pink.clone(),
                vo.sales.clone(),
                vo.is_show.clone(),
                vo.stop_time.clone(),
            ]
        })?;

        Ok(file_name)
    }

    /// 导出订单Excel
    ///
    /// Java参考: ExcelServiceImpl.exportOrder()
    pub async fn export_order(
        db: &sea_orm::DatabaseConnection,
        request: &StoreOrderSearchRequest,
    ) -> Result<String> {
        let page_param = PageParamRequest {
            page: Some(1),
            limit: Some(EXPORT_MAX_LIMIT),
        };
        let page_data = StoreOrderService::get_admin_list(db, request, &page_param).await?;
        if page_data.list.is_empty() {
            return Err(Error::string("没有可导出的数据"));
        }

        let vo_list: Vec<OrderExcelVo> = page_data.list.iter().map(|item| {
            // 拼接商品名称
            let product_name = item.product_list.iter()
                .filter_map(|p| {
                    p.info.as_ref().and_then(|info| {
                        info.get("productName")
                            .or_else(|| info.get("storeName"))
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string())
                    })
                })
                .collect::<Vec<_>>()
                .join(",");

            // 订单状态: statusStr是HashMap, 取value字段
            let status_str = item.status_str.get("value")
                .cloned()
                .unwrap_or_default();

            OrderExcelVo {
                order_id: item.order_id.clone(),
                pay_price: item.pay_price.to_string(),
                create_time: item.create_time.clone().unwrap_or_default(),
                product_name,
                status_str,
                pay_type_str: item.pay_type_str.clone(),
                order_type: item.order_type.clone(),
                real_name: item.real_name.clone(),
            }
        }).collect();

        let aliases = vec![
            ("orderId", "订单号"),
            ("payPrice", "实际支付金额"),
            ("createTime", "创建时间"),
            ("productName", "商品信息"),
            ("statusStr", "订单状态"),
            ("payTypeStr", "支付方式"),
            ("orderType", "订单类型"),
            ("realName", "用户姓名"),
        ];

        let file_name = Self::generate_file_name("订单导出");
        Self::write_excel(&file_name, "订单导出", &aliases, &vo_list, |vo| {
            vec![
                vo.order_id.clone(),
                vo.pay_price.clone(),
                vo.create_time.clone(),
                vo.product_name.clone(),
                vo.status_str.clone(),
                vo.pay_type_str.clone(),
                vo.order_type.clone(),
                vo.real_name.clone(),
            ]
        })?;

        Ok(file_name)
    }

    // ==================== 私有方法 ====================

    /// 生成文件名
    ///
    /// Java参考: ExcelServiceImpl中的文件名生成逻辑
    /// 格式: {prefix}_{timestamp}_{random}.xlsx
    fn generate_file_name(prefix: &str) -> String {
        let timestamp = Local::now().format("%Y%m%d%H%M%S");
        let random = uuid::Uuid::new_v4().to_string().replace("-", "");
        let random_short = &random[..8];
        format!("{}_{}{}.xlsx", prefix, timestamp, random_short)
    }

    /// 写入Excel文件
    ///
    /// 通用Excel写入方法，接受列别名和数据转换函数
    fn write_excel<T, F>(
        file_name: &str,
        sheet_name: &str,
        aliases: &[(&str, &str)],
        data: &[T],
        row_mapper: F,
    ) -> Result<()>
    where
        F: Fn(&T) -> Vec<String>,
    {
        // 确保uploads/excel目录存在
        let dir = std::path::Path::new("./uploads/excel");
        if !dir.exists() {
            std::fs::create_dir_all(dir)
                .map_err(|e| Error::string(&format!("创建目录失败: {}", e)))?;
        }

        let file_path = dir.join(file_name);

        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();
        worksheet.set_name(sheet_name)
            .map_err(|e| Error::string(&format!("设置工作表名称失败: {}", e)))?;

        // 写入表头
        let header_format = Format::new().set_bold();
        for (col, (_key, header)) in aliases.iter().enumerate() {
            worksheet.write_string_with_format(0, col as u16, *header, &header_format)
                .map_err(|e| Error::string(&format!("写入表头失败: {}", e)))?;
        }

        // 写入数据行
        for (row_idx, item) in data.iter().enumerate() {
            let row_data = row_mapper(item);
            for (col_idx, value) in row_data.iter().enumerate() {
                worksheet.write_string((row_idx + 1) as u32, col_idx as u16, value)
                    .map_err(|e| Error::string(&format!("写入数据失败: {}", e)))?;
            }
        }

        workbook.save(&file_path)
            .map_err(|e| Error::string(&format!("保存Excel文件失败: {}", e)))?;

        Ok(())
    }
}
