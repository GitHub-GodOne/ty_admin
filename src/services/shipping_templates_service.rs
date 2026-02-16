/// 运费模板 -- 服务层
///
/// Java参考: ShippingTemplatesServiceImpl, ShippingTemplatesRegionServiceImpl, ShippingTemplatesFreeServiceImpl
use loco_rs::prelude::*;
use sea_orm::*;
use chrono::Local;
use indexmap::IndexMap;

use crate::dtos::shipping_templates::*;
use crate::dtos::common::PageParamRequest;
use crate::common::pagination::PageResponse;
use crate::models::_entities::{
    shipping_templates, shipping_templates_free, shipping_templates_region, store_product,
};

pub struct ShippingTemplatesService;

impl ShippingTemplatesService {
    /// 分页列表
    /// Java: ShippingTemplatesServiceImpl.getList()
    pub async fn get_list(
        db: &DatabaseConnection,
        request: &ShippingTemplatesSearchRequest,
        page_param: &PageParamRequest,
    ) -> Result<PageResponse<ShippingTemplatesListResponse>> {
        let page = page_param.get_page();
        let limit = page_param.get_limit();

        let mut query = shipping_templates::Entity::find();

        if let Some(keywords) = &request.keywords {
            if !keywords.trim().is_empty() {
                query = query.filter(shipping_templates::Column::Name.contains(keywords.trim()));
            }
        }

        query = query
            .order_by_desc(shipping_templates::Column::Sort)
            .order_by_desc(shipping_templates::Column::Id);

        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let records = paginator.fetch_page((page - 1) as u64).await?;

        let list = records.into_iter().map(Self::model_to_list_response).collect();
        Ok(PageResponse::new(list, total, page as u64, limit as u64))
    }

    /// 新增运费模板
    /// Java: ShippingTemplatesServiceImpl.create()
    pub async fn create(
        db: &DatabaseConnection,
        request: &ShippingTemplatesRequest,
    ) -> Result<bool> {
        // 判断模板名称是否重复
        if Self::is_exist_name(db, &request.name, None).await? {
            return Err(Error::BadRequest("模板名称已存在,请更换模板名称!".to_string()));
        }
        if request.appoint == 2 && request.region_list.is_empty() {
            return Err(Error::BadRequest("不包邮，最少需要一条公共区域运费数据".to_string()));
        }

        let now = Local::now().naive_local();
        let model = shipping_templates::ActiveModel {
            name: Set(request.name.clone()),
            r#type: Set(request.template_type as i16),
            appoint: Set(request.appoint as i16),
            sort: Set(request.sort),
            create_time: Set(now),
            update_time: Set(now),
            ..Default::default()
        };
        let inserted = model.insert(db).await?;
        let temp_id = inserted.id;

        // 保存区域运费
        if request.appoint != 0 && !request.region_list.is_empty() {
            Self::save_region_all(db, &request.region_list, request.template_type as i16, temp_id).await?;
        }
        // 保存包邮区域
        if request.appoint == 2 && !request.free_list.is_empty() {
            Self::save_free_all(db, &request.free_list, request.template_type as i16, temp_id).await?;
        }

        Ok(true)
    }

    /// 修改运费模板
    /// Java: ShippingTemplatesServiceImpl.update()
    pub async fn update(
        db: &DatabaseConnection,
        id: i32,
        request: &ShippingTemplatesRequest,
    ) -> Result<bool> {
        let template = Self::get_by_id(db, id).await?;

        if request.appoint == 2 && request.region_list.is_empty() {
            return Err(Error::BadRequest("不包邮，最少需要一条公共区域运费数据".to_string()));
        }
        if template.name != request.name {
            if Self::is_exist_name(db, &request.name, None).await? {
                return Err(Error::BadRequest("模板名称已存在,请更换模板名称!".to_string()));
            }
        }

        // 删除旧的区域和包邮数据
        if template.appoint != 0 {
            Self::delete_region_by_temp_id(db, id).await?;
        }
        if template.appoint == 2 {
            Self::delete_free_by_temp_id(db, id).await?;
        }

        let now = Local::now().naive_local();
        let mut active: shipping_templates::ActiveModel = template.into();
        active.name = Set(request.name.clone());
        active.sort = Set(request.sort);
        active.r#type = Set(request.template_type as i16);
        active.appoint = Set(request.appoint as i16);
        active.update_time = Set(now);
        active.update(db).await?;

        // 保存新的区域运费
        if !request.region_list.is_empty() {
            Self::save_region_all(db, &request.region_list, request.template_type as i16, id).await?;
        }
        // 保存新的包邮区域
        if request.appoint == 2 && !request.free_list.is_empty() {
            Self::save_free_all(db, &request.free_list, request.template_type as i16, id).await?;
        }

        Ok(true)
    }

    /// 删除运费模板
    /// Java: ShippingTemplatesServiceImpl.remove()
    pub async fn remove(db: &DatabaseConnection, id: i32) -> Result<bool> {
        Self::get_by_id(db, id).await?;
        // 检查是否有商品使用此模板
        let count = store_product::Entity::find()
            .filter(store_product::Column::TempId.eq(id))
            .filter(store_product::Column::IsDel.eq(0i16))
            .count(db)
            .await?;
        if count > 0 {
            return Err(Error::BadRequest("有商品使用此运费模板，无法删除".to_string()));
        }
        Self::delete_region_by_temp_id(db, id).await?;
        Self::delete_free_by_temp_id(db, id).await?;
        shipping_templates::Entity::delete_by_id(id).exec(db).await?;
        Ok(true)
    }

    /// 获取模板详情
    /// Java: ShippingTemplatesServiceImpl.getInfo()
    pub async fn get_info(db: &DatabaseConnection, id: i32) -> Result<ShippingTemplatesInfoResponse> {
        let template = Self::get_by_id(db, id).await?;
        let mut response = ShippingTemplatesInfoResponse {
            id: template.id,
            name: template.name.clone(),
            template_type: template.r#type as i32,
            appoint: template.appoint as i32,
            sort: template.sort,
            region_list: vec![],
            free_list: vec![],
        };
        if template.appoint == 0 {
            return Ok(response);
        }
        let mut region_list = Self::get_region_list_group(db, id).await?;
        if !region_list.is_empty() {
            for r in &mut region_list {
                r.title = format!("[{}]", r.title);
            }
            response.region_list = region_list;
        }
        if template.appoint == 2 {
            let mut free_list = Self::get_free_list_group(db, id).await?;
            for f in &mut free_list {
                f.title = format!("[{}]", f.title);
            }
            response.free_list = free_list;
        }
        Ok(response)
    }

    /// 区域运费分组查询
    /// Java: ShippingTemplatesRegionServiceImpl.getListGroup()
    pub async fn get_region_list_group(
        db: &DatabaseConnection,
        temp_id: i32,
    ) -> Result<Vec<ShippingTemplatesRegionResponse>> {
        let list = shipping_templates_region::Entity::find()
            .filter(shipping_templates_region::Column::TempId.eq(temp_id))
            .all(db)
            .await?;
        if list.is_empty() {
            return Ok(vec![]);
        }
        let mut map: IndexMap<String, Vec<shipping_templates_region::Model>> = IndexMap::new();
        for item in list {
            map.entry(item.uniqid.clone()).or_default().push(item);
        }
        let mut result = vec![];
        for (uniqid, items) in map {
            let title = items.iter()
                .map(|i| i.title.clone().unwrap_or_default())
                .collect::<Vec<_>>()
                .join(",");
            let first_item = &items[0];
            result.push(ShippingTemplatesRegionResponse {
                title,
                first: first_item.first,
                first_price: first_item.first_price,
                renewal: first_item.renewal,
                renewal_price: first_item.renewal_price,
                uniqid,
            });
        }
        Ok(result)
    }

    /// 包邮区域分组查询
    /// Java: ShippingTemplatesFreeServiceImpl.getListGroup()
    pub async fn get_free_list_group(
        db: &DatabaseConnection,
        temp_id: i32,
    ) -> Result<Vec<ShippingTemplatesFreeResponse>> {
        let list = shipping_templates_free::Entity::find()
            .filter(shipping_templates_free::Column::TempId.eq(temp_id))
            .all(db)
            .await?;
        if list.is_empty() {
            return Ok(vec![]);
        }
        let mut map: IndexMap<String, Vec<shipping_templates_free::Model>> = IndexMap::new();
        for item in list {
            map.entry(item.uniqid.clone()).or_default().push(item);
        }
        let mut result = vec![];
        for (uniqid, items) in map {
            let title = items.iter()
                .map(|i| i.title.clone().unwrap_or_default())
                .collect::<Vec<_>>()
                .join(",");
            let first_item = &items[0];
            result.push(ShippingTemplatesFreeResponse {
                title,
                number: first_item.number,
                price: first_item.price,
                uniqid,
            });
        }
        Ok(result)
    }

    /// 批量保存区域运费
    /// Java: ShippingTemplatesRegionServiceImpl.saveAll()
    async fn save_region_all(
        db: &DatabaseConnection,
        region_list: &[ShippingTemplatesRegionRequest],
        template_type: i16,
        temp_id: i32,
    ) -> Result<()> {
        let now = Local::now().naive_local();
        for req in region_list {
            let unique_key = format!("{:x}", md5::compute(format!("{:?}", req)));
            let title_array: Vec<String> = serde_json::from_str(&req.title)
                .unwrap_or_else(|_| vec![req.title.clone()]);

            if req.city_id == "all" || req.city_id == "0" {
                let model = shipping_templates_region::ActiveModel {
                    city_id: Set(0),
                    title: Set(Some(title_array.first().cloned().unwrap_or_default())),
                    uniqid: Set(unique_key),
                    renewal: Set(req.renewal),
                    renewal_price: Set(req.renewal_price),
                    first: Set(req.first),
                    first_price: Set(req.first_price),
                    temp_id: Set(temp_id),
                    r#type: Set(template_type),
                    status: Set(Some(1i16)),
                    create_time: Set(now),
                    update_time: Set(now),
                    ..Default::default()
                };
                model.insert(db).await?;
            } else {
                let city_ids: Vec<i32> = req.city_id.split(',')
                    .filter_map(|s| s.trim().parse::<i32>().ok())
                    .collect();
                // 解析title JSON数组，建立cityId -> title的映射
                let mut city_map: std::collections::HashMap<i32, String> = std::collections::HashMap::new();
                for t in &title_array {
                    if let Ok(arr) = serde_json::from_str::<Vec<serde_json::Value>>(t) {
                        if arr.len() > 2 {
                            if let Some(cid) = arr[2].as_i64() {
                                city_map.insert(cid as i32, t.clone());
                            }
                        }
                    }
                }
                for city_id in &city_ids {
                    let title = city_map.get(city_id).cloned().unwrap_or_default();
                    let model = shipping_templates_region::ActiveModel {
                        city_id: Set(*city_id),
                        title: Set(Some(title)),
                        uniqid: Set(unique_key.clone()),
                        renewal: Set(req.renewal),
                        renewal_price: Set(req.renewal_price),
                        first: Set(req.first),
                        first_price: Set(req.first_price),
                        temp_id: Set(temp_id),
                        r#type: Set(template_type),
                        status: Set(Some(1i16)),
                        create_time: Set(now),
                        update_time: Set(now),
                        ..Default::default()
                    };
                    model.insert(db).await?;
                }
            }
        }
        Ok(())
    }

    /// 批量保存包邮区域
    /// Java: ShippingTemplatesFreeServiceImpl.saveAll()
    async fn save_free_all(
        db: &DatabaseConnection,
        free_list: &[ShippingTemplatesFreeRequest],
        template_type: i16,
        temp_id: i32,
    ) -> Result<()> {
        let now = Local::now().naive_local();
        for req in free_list {
            let unique_key = format!("{:x}", md5::compute(format!("{:?}", req)));
            let title_array: Vec<String> = serde_json::from_str(&req.title)
                .unwrap_or_else(|_| vec![req.title.clone()]);

            if req.city_id == "all" || req.city_id == "0" {
                let model = shipping_templates_free::ActiveModel {
                    city_id: Set(0),
                    title: Set(Some(title_array.first().cloned().unwrap_or_default())),
                    uniqid: Set(unique_key),
                    temp_id: Set(temp_id),
                    r#type: Set(template_type),
                    status: Set(Some(1i16)),
                    number: Set(req.number),
                    price: Set(req.price),
                    create_time: Set(now),
                    update_time: Set(now),
                    ..Default::default()
                };
                model.insert(db).await?;
            } else {
                let city_ids: Vec<i32> = req.city_id.split(',')
                    .filter_map(|s| s.trim().parse::<i32>().ok())
                    .collect();
                let mut city_map: std::collections::HashMap<i32, String> = std::collections::HashMap::new();
                for t in &title_array {
                    if let Ok(arr) = serde_json::from_str::<Vec<serde_json::Value>>(t) {
                        if arr.len() > 2 {
                            if let Some(cid) = arr[2].as_i64() {
                                city_map.insert(cid as i32, t.clone());
                            }
                        }
                    }
                }
                for city_id in &city_ids {
                    let title = city_map.get(city_id).cloned().unwrap_or_default();
                    let model = shipping_templates_free::ActiveModel {
                        city_id: Set(*city_id),
                        title: Set(Some(title)),
                        uniqid: Set(unique_key.clone()),
                        temp_id: Set(temp_id),
                        r#type: Set(template_type),
                        number: Set(req.number),
                        price: Set(req.price),
                        status: Set(Some(1i16)),
                        create_time: Set(now),
                        update_time: Set(now),
                        ..Default::default()
                    };
                    model.insert(db).await?;
                }
            }
        }
        Ok(())
    }

    /// 删除模板下所有区域运费
    async fn delete_region_by_temp_id(db: &DatabaseConnection, temp_id: i32) -> Result<()> {
        shipping_templates_region::Entity::delete_many()
            .filter(shipping_templates_region::Column::TempId.eq(temp_id))
            .exec(db)
            .await?;
        Ok(())
    }

    /// 删除模板下所有包邮区域
    async fn delete_free_by_temp_id(db: &DatabaseConnection, temp_id: i32) -> Result<()> {
        shipping_templates_free::Entity::delete_many()
            .filter(shipping_templates_free::Column::TempId.eq(temp_id))
            .exec(db)
            .await?;
        Ok(())
    }

    /// 检查模板名称是否存在
    async fn is_exist_name(db: &DatabaseConnection, name: &str, _exclude_id: Option<i32>) -> Result<bool> {
        let count = shipping_templates::Entity::find()
            .filter(shipping_templates::Column::Name.eq(name))
            .count(db)
            .await?;
        Ok(count > 0)
    }

    /// 根据ID获取模板
    async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<shipping_templates::Model> {
        shipping_templates::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::BadRequest("运费模板不存在".to_string()))
    }

    /// Model -> ListResponse
    fn model_to_list_response(model: shipping_templates::Model) -> ShippingTemplatesListResponse {
        ShippingTemplatesListResponse {
            id: model.id,
            name: model.name,
            template_type: model.r#type as i32,
            appoint: model.appoint as i32,
            sort: model.sort,
            create_time: Some(model.create_time.format("%Y-%m-%d %H:%M:%S").to_string()),
            update_time: Some(model.update_time.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}
