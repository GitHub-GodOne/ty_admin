/// 商品评论管理服务
///
/// 实现与Java版本一致的商品评论管理业务逻辑
/// Java参考: StoreProductReplyServiceImpl
use loco_rs::prelude::*;
use sea_orm::*;
use sea_orm::sea_query::Expr;

use crate::models::_entities::{store_product_reply, store_product};
use crate::dtos::product_reply::{
    StoreProductReplySearchRequest, StoreProductReplyAddRequest,
    StoreProductReplyCommentRequest, StoreProductReplyResponse,
    StoreProductSimpleVo,
};
use crate::dtos::common::{CommonPage, PageParamRequest};

/// 商品评论服务
pub struct StoreProductReplyService;

impl StoreProductReplyService {
    /// 分页列表
    ///
    /// Java参考: StoreProductReplyServiceImpl.getList()
    /// 过滤条件: isDel=false, isReply, productSearch(商品名称), nickname, dateLimit
    /// 每条评论附带商品信息，pics字符串转数组
    pub async fn get_list(
        db: &DatabaseConnection,
        request: &StoreProductReplySearchRequest,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<StoreProductReplyResponse>> {
        let page = page_param.get_page();
        let limit = page_param.get_limit();

        // 基础条件: is_del = 0
        let mut query = store_product_reply::Entity::find()
            .filter(store_product_reply::Column::IsDel.eq(0i16));

        // 是否已回复
        if let Some(is_reply) = request.is_reply {
            let reply_val: i16 = if is_reply { 1 } else { 0 };
            query = query.filter(store_product_reply::Column::IsReply.eq(reply_val));
        }

        // 用户昵称模糊搜索
        if let Some(nickname) = &request.nickname {
            if !nickname.is_empty() {
                query = query.filter(
                    store_product_reply::Column::Nickname.contains(nickname)
                );
            }
        }

        // 时间范围过滤
        // Java: dateLimit格式 "2024-01-01,2024-01-31"
        if let Some(date_limit) = &request.date_limit {
            if !date_limit.is_empty() {
                let dates: Vec<&str> = date_limit.split(',').collect();
                if dates.len() == 2 {
                    let start_date = dates[0].trim();
                    let end_date = dates[1].trim();
                    query = query.filter(
                        Expr::cust(&format!(
                            "create_time >= '{} 00:00:00'::timestamp",
                            start_date
                        ))
                    );
                    query = query.filter(
                        Expr::cust(&format!(
                            "create_time <= '{} 23:59:59'::timestamp",
                            end_date
                        ))
                    );
                }
            }
        }

        // 商品名称搜索: 需要先查出匹配的商品ID，再过滤评论
        // Java: 通过productSearch查询store_product表的store_name，获取product_id列表
        if let Some(product_search) = &request.product_search {
            if !product_search.is_empty() {
                let product_ids = Self::get_product_ids_by_name(db, product_search).await?;
                if product_ids.is_empty() {
                    // 没有匹配的商品，返回空结果
                    return Ok(CommonPage::new(vec![], 0, page, limit));
                }
                query = query.filter(
                    store_product_reply::Column::ProductId.is_in(product_ids)
                );
            }
        }

        // 按ID降序
        query = query.order_by_desc(store_product_reply::Column::Id);

        // 分页查询
        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let replies = paginator.fetch_page((page - 1) as u64).await?;

        // 为每条评论附带商品信息
        let mut list = Vec::new();
        for reply in replies {
            let product = Self::get_product_simple(db, reply.product_id).await?;
            let response = Self::model_to_response(reply, product);
            list.push(response);
        }

        Ok(CommonPage::new(list, total as i64, page, limit))
    }

    /// 虚拟评论（新增虚拟评论）
    ///
    /// Java参考: StoreProductReplyServiceImpl.virtualCreate()
    /// 创建虚拟评论，清理pics中的JSON括号，清理avatar前缀，生成随机unique
    pub async fn virtual_create(
        db: &DatabaseConnection,
        request: &StoreProductReplyAddRequest,
    ) -> Result<bool> {
        // 验证商品是否存在
        let _product = store_product::Entity::find_by_id(request.product_id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("商品不存在"))?;

        // 验证评分范围 1-5
        if request.product_score < 1 || request.product_score > 5 {
            return Err(Error::string("商品评分范围为1-5"));
        }
        if request.service_score < 1 || request.service_score > 5 {
            return Err(Error::string("服务评分范围为1-5"));
        }

        // 验证评论内容
        if request.comment.is_empty() {
            return Err(Error::string("评论内容不能为空"));
        }
        if request.comment.len() > 512 {
            return Err(Error::string("评论内容不能超过512个字符"));
        }

        // 处理pics: 清理JSON括号
        // Java: pics = systemAttachmentService.clearPrefix(request.getPics())
        //        如果pics包含 "[" 和 "]"，去掉括号
        let pics = request.pics.clone().unwrap_or_default();
        let pics = Self::clean_pics_string(&pics);

        // 处理avatar: 清理前缀
        // Java: avatar = systemAttachmentService.clearPrefix(request.getAvatar())
        let avatar = request.avatar.clone().unwrap_or_default();

        // 生成随机unique
        // Java: CrmebUtil.randomCount(11111111, 99999999).toString()
        let unique = Self::generate_random_unique();

        let now = chrono::Local::now().naive_local();
        let uid = request.user_id.unwrap_or(0);

        let model = store_product_reply::ActiveModel {
            uid: Set(uid),
            oid: Set(0), // 虚拟评论没有订单
            unique: Set(unique),
            product_id: Set(request.product_id),
            reply_type: Set("product".to_string()),
            product_score: Set(request.product_score),
            service_score: Set(request.service_score),
            comment: Set(request.comment.clone()),
            pics: Set(pics),
            nickname: Set(request.nickname.clone().unwrap_or_default()),
            avatar: Set(avatar),
            sku: Set(request.sku.clone().unwrap_or_default()),
            is_del: Set(0),
            is_reply: Set(0),
            create_time: Set(Some(now)),
            update_time: Set(Some(now)),
            ..Default::default()
        };

        store_product_reply::Entity::insert(model).exec(db).await?;
        Ok(true)
    }

    /// 删除评论（软删除）
    ///
    /// Java参考: StoreProductReplyServiceImpl.delete()
    /// 设置 is_del = 1
    pub async fn delete(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<bool> {
        let reply = store_product_reply::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("评论不存在"))?;

        let mut model: store_product_reply::ActiveModel = reply.into();
        model.is_del = Set(1);
        model.update(db).await?;

        Ok(true)
    }

    /// 获取评论详情
    ///
    /// Java参考: Controller中调用 getById(id)
    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<StoreProductReplyResponse> {
        let reply = store_product_reply::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("评论不存在"))?;

        let product = Self::get_product_simple(db, reply.product_id).await?;
        Ok(Self::model_to_response(reply, product))
    }

    /// 管理员回复评论
    ///
    /// Java参考: StoreProductReplyServiceImpl.comment()
    /// 设置 merchantReplyContent, merchantReplyTime=当前时间戳, isReply=1
    pub async fn comment(
        db: &DatabaseConnection,
        request: &StoreProductReplyCommentRequest,
    ) -> Result<bool> {
        if request.merchant_reply_content.is_empty() {
            return Err(Error::string("回复内容不能为空"));
        }

        let reply = store_product_reply::Entity::find_by_id(request.ids)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("评论不存在"))?;

        // 检查是否已经回复过
        // Java: if (storeProductReply.getIsReply()) throw new CrmebException("已回复，不能再次回复")
        if reply.is_reply != 0 {
            return Err(Error::string("已回复，不能再次回复"));
        }

        let now_timestamp = chrono::Local::now().timestamp() as i32;

        let mut model: store_product_reply::ActiveModel = reply.into();
        model.merchant_reply_content = Set(Some(request.merchant_reply_content.clone()));
        model.merchant_reply_time = Set(Some(now_timestamp));
        model.is_reply = Set(1);
        model.update_time = Set(Some(chrono::Local::now().naive_local()));
        model.update(db).await?;

        Ok(true)
    }

    // ==================== 辅助方法 ====================

    /// 根据商品名称搜索商品ID列表
    ///
    /// Java参考: StoreProductReplyServiceImpl.getList() 中的 productSearch 逻辑
    async fn get_product_ids_by_name(
        db: &DatabaseConnection,
        product_name: &str,
    ) -> Result<Vec<i32>> {
        let products = store_product::Entity::find()
            .filter(store_product::Column::StoreName.contains(product_name))
            .filter(store_product::Column::IsDel.eq(0i16))
            .all(db)
            .await?;

        Ok(products.into_iter().map(|p| p.id).collect())
    }

    /// 获取商品简要信息
    ///
    /// Java参考: StoreProductReplyServiceImpl.getList() 中为每条评论附带 StoreProduct
    async fn get_product_simple(
        db: &DatabaseConnection,
        product_id: i32,
    ) -> Result<Option<StoreProductSimpleVo>> {
        let product = store_product::Entity::find_by_id(product_id)
            .one(db)
            .await?;

        Ok(product.map(|p| StoreProductSimpleVo {
            id: p.id,
            image: p.image,
            store_name: p.store_name,
        }))
    }

    /// 清理pics字符串
    ///
    /// Java参考: 去掉JSON数组的括号和多余引号
    /// 输入: "[\"url1\",\"url2\"]" 或 "url1,url2"
    /// 输出: "url1,url2"
    fn clean_pics_string(pics: &str) -> String {
        let mut result = pics.to_string();
        // 去掉JSON数组括号
        result = result.replace("[", "").replace("]", "");
        // 去掉多余的引号
        result = result.replace("\"", "");
        result.trim().to_string()
    }

    /// 解析pics字符串为数组
    ///
    /// Java参考: StoreProductReplyServiceImpl.getList() 中 pics 转 List<String>
    fn parse_pics_to_vec(pics: &str) -> Vec<String> {
        if pics.is_empty() {
            return vec![];
        }

        // 尝试JSON解析
        if let Ok(arr) = serde_json::from_str::<Vec<String>>(pics) {
            return arr;
        }

        // 逗号分隔
        pics.split(',')
            .map(|s| s.trim().replace("\"", "").to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    /// 生成随机unique标识
    ///
    /// Java参考: CrmebUtil.randomCount(11111111, 99999999).toString()
    fn generate_random_unique() -> String {
        // 使用uuid生成随机数，取前8位数字
        // Java: CrmebUtil.randomCount(11111111, 99999999).toString()
        let uuid = uuid::Uuid::new_v4().to_string().replace("-", "");
        // 从uuid的hex字符中取8个字符转为数字
        let hash = uuid.as_bytes().iter().fold(0u64, |acc, &b| acc.wrapping_mul(31).wrapping_add(b as u64));
        let num = (hash % 88888889) + 11111111;
        num.to_string()
    }

    /// Model转Response
    fn model_to_response(
        model: store_product_reply::Model,
        product: Option<StoreProductSimpleVo>,
    ) -> StoreProductReplyResponse {
        let pics = Self::parse_pics_to_vec(&model.pics);
        let create_time = model.create_time.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string());
        let update_time = model.update_time.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string());

        StoreProductReplyResponse {
            id: model.id,
            uid: model.uid,
            oid: model.oid,
            unique: model.unique,
            product_id: model.product_id,
            reply_type: model.reply_type,
            product_score: model.product_score,
            service_score: model.service_score,
            comment: model.comment,
            pics,
            merchant_reply_content: model.merchant_reply_content,
            merchant_reply_time: model.merchant_reply_time,
            is_del: model.is_del != 0,
            is_reply: model.is_reply != 0,
            nickname: model.nickname,
            avatar: model.avatar,
            create_time,
            update_time,
            sku: model.sku,
            store_product: product,
        }
    }
}
