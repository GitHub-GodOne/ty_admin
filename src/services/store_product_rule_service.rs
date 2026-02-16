/// 商品规格管理服务
///
/// 实现与Java版本一致的商品规格管理业务逻辑
/// Java参考: StoreProductRuleServiceImpl
use loco_rs::prelude::*;
use sea_orm::*;

use crate::models::_entities::store_product_rule;
use crate::dtos::product_rule::{
    StoreProductRuleRequest, StoreProductRuleSearchRequest, StoreProductRuleResponse,
};
use crate::dtos::common::{CommonPage, PageParamRequest};

/// 商品规格服务
pub struct StoreProductRuleService;

impl StoreProductRuleService {
    /// 分页列表
    ///
    /// Java参考: StoreProductRuleServiceImpl.getList()
    /// 按关键字搜索ruleName或ruleValue，按id降序
    pub async fn get_list(
        db: &DatabaseConnection,
        request: &StoreProductRuleSearchRequest,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<StoreProductRuleResponse>> {
        let page = page_param.get_page();
        let limit = page_param.get_limit();

        let mut query = store_product_rule::Entity::find();

        // 关键字搜索：匹配ruleName或ruleValue
        if let Some(keywords) = &request.keywords {
            if !keywords.is_empty() {
                query = query.filter(
                    Condition::any()
                        .add(store_product_rule::Column::RuleName.contains(keywords))
                        .add(store_product_rule::Column::RuleValue.contains(keywords))
                );
            }
        }

        // 按ID降序
        query = query.order_by_desc(store_product_rule::Column::Id);

        // 分页查询
        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let list = paginator.fetch_page((page - 1) as u64).await?;

        let list: Vec<StoreProductRuleResponse> = list
            .into_iter()
            .map(Self::model_to_response)
            .collect();

        Ok(CommonPage {
            list,
            total: total as i64,
            page_number: page,
            page_size: limit,
        })
    }

    /// 新增商品规格
    ///
    /// Java参考: StoreProductRuleServiceImpl.save()
    /// 检查规格名称是否已存在，存在则抛出异常
    pub async fn save(
        db: &DatabaseConnection,
        request: &StoreProductRuleRequest,
    ) -> Result<bool> {
        // 验证
        if request.rule_name.is_empty() {
            return Err(Error::string("规格名称不能为空"));
        }
        if request.rule_name.len() > 32 {
            return Err(Error::string("规格名称长度不能超过32个字符"));
        }
        if request.rule_value.is_empty() {
            return Err(Error::string("规格值不能为空"));
        }

        // 检查规格名称是否已存在
        let existing = Self::get_list_by_rule_name(db, &request.rule_name).await?;
        if !existing.is_empty() {
            return Err(Error::string("此规格值已经存在"));
        }

        let model = store_product_rule::ActiveModel {
            rule_name: Set(request.rule_name.clone()),
            rule_value: Set(request.rule_value.clone()),
            ..Default::default()
        };

        store_product_rule::Entity::insert(model).exec(db).await?;
        Ok(true)
    }

    /// 修改规格
    ///
    /// Java参考: StoreProductRuleServiceImpl.updateRule()
    pub async fn update_rule(
        db: &DatabaseConnection,
        request: &StoreProductRuleRequest,
    ) -> Result<bool> {
        let id = request.id
            .ok_or_else(|| Error::string("规格ID不能为空"))?;

        let existing = store_product_rule::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("规格不存在"))?;

        let mut model: store_product_rule::ActiveModel = existing.into();
        model.rule_name = Set(request.rule_name.clone());
        model.rule_value = Set(request.rule_value.clone());
        model.update(db).await?;

        Ok(true)
    }

    /// 批量删除规格
    ///
    /// Java参考: Controller中调用 removeByIds(CrmebUtil.stringToArray(ids))
    pub async fn delete_by_ids(
        db: &DatabaseConnection,
        ids: Vec<i32>,
    ) -> Result<bool> {
        if ids.is_empty() {
            return Err(Error::string("请选择要删除的规格"));
        }

        store_product_rule::Entity::delete_many()
            .filter(store_product_rule::Column::Id.is_in(ids))
            .exec(db)
            .await?;

        Ok(true)
    }

    /// 获取规格详情
    ///
    /// Java参考: Controller中调用 getById(id)
    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<StoreProductRuleResponse> {
        let model = store_product_rule::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("规格不存在"))?;

        Ok(Self::model_to_response(model))
    }

    // ==================== 辅助方法 ====================

    /// 根据规格名称查询同名规格
    ///
    /// Java参考: StoreProductRuleServiceImpl.getListByRuleName()
    async fn get_list_by_rule_name(
        db: &DatabaseConnection,
        rule_name: &str,
    ) -> Result<Vec<store_product_rule::Model>> {
        if rule_name.is_empty() {
            return Ok(vec![]);
        }

        let list = store_product_rule::Entity::find()
            .filter(store_product_rule::Column::RuleName.eq(rule_name))
            .all(db)
            .await?;

        Ok(list)
    }

    /// Model转Response
    fn model_to_response(model: store_product_rule::Model) -> StoreProductRuleResponse {
        StoreProductRuleResponse {
            id: model.id,
            rule_name: model.rule_name,
            rule_value: model.rule_value,
        }
    }
}
