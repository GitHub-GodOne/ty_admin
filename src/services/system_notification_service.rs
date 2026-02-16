/// 系统通知 -- 服务层
///
/// 实现与Java版本一致的通知管理逻辑
/// Java代码参考: com.zbkj.service.service.impl.SystemNotificationServiceImpl
use loco_rs::prelude::*;
use sea_orm::prelude::Expr;
use sea_orm::Set;
use chrono::Local;

use crate::dtos::system_notification::*;
use crate::models::_entities::{system_notification, template_message, sms_template};

pub struct SystemNotificationService;

impl SystemNotificationService {
    /// 系统通知列表
    ///
    /// Java: SystemNotificationServiceImpl.getList()
    /// 逻辑：按 sendType 过滤（可选）
    pub async fn get_list(
        db: &DatabaseConnection,
        request: &NotificationSearchRequest,
    ) -> Result<Vec<SystemNotificationResponse>> {
        let mut query = system_notification::Entity::find();

        if let Some(send_type) = request.send_type {
            query = query.filter(system_notification::Column::SendType.eq(send_type));
        }

        let records = query.all(db).await?;

        let list = records
            .into_iter()
            .map(Self::model_to_response)
            .collect();

        Ok(list)
    }

    /// 公众号模板开关
    ///
    /// Java: SystemNotificationServiceImpl.wechatSwitch()
    /// 逻辑：
    /// 1. 查找通知
    /// 2. is_wechat == 0 则报错（未配置）
    /// 3. 切换: 1 -> 2, 2 -> 1
    pub async fn wechat_switch(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<bool> {
        let notification = Self::get_by_id_exception(db, id).await?;

        if notification.is_wechat == 0 {
            return Err(Error::BadRequest("通知没有配置公众号模板".to_string()));
        }

        let new_val: i16 = if notification.is_wechat == 1 { 2 } else { 1 };

        system_notification::Entity::update_many()
            .col_expr(system_notification::Column::IsWechat, Expr::value(new_val))
            .filter(system_notification::Column::Id.eq(id))
            .exec(db)
            .await?;

        Ok(true)
    }

    /// 小程序订阅模板开关
    ///
    /// Java: SystemNotificationServiceImpl.routineSwitch()
    /// 逻辑：
    /// 1. 查找通知
    /// 2. is_routine == 0 则报错（未配置）
    /// 3. 切换: 1 -> 2, 2 -> 1
    pub async fn routine_switch(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<bool> {
        let notification = Self::get_by_id_exception(db, id).await?;

        if notification.is_routine == 0 {
            return Err(Error::BadRequest("通知没有配置小程序订阅模板".to_string()));
        }

        let new_val: i16 = if notification.is_routine == 1 { 2 } else { 1 };

        system_notification::Entity::update_many()
            .col_expr(system_notification::Column::IsRoutine, Expr::value(new_val))
            .filter(system_notification::Column::Id.eq(id))
            .exec(db)
            .await?;

        Ok(true)
    }

    /// 发送短信开关
    ///
    /// Java: SystemNotificationServiceImpl.smsSwitch()
    /// 逻辑：
    /// 1. 查找通知
    /// 2. is_sms == 0 则报错（未配置）
    /// 3. 切换: 1 -> 2, 2 -> 1
    pub async fn sms_switch(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<bool> {
        let notification = Self::get_by_id_exception(db, id).await?;

        if notification.is_sms == 0 {
            return Err(Error::BadRequest("通知没有配置短信".to_string()));
        }

        let new_val: i16 = if notification.is_sms == 1 { 2 } else { 1 };

        system_notification::Entity::update_many()
            .col_expr(system_notification::Column::IsSms, Expr::value(new_val))
            .filter(system_notification::Column::Id.eq(id))
            .exec(db)
            .await?;

        Ok(true)
    }

    /// 通知详情
    ///
    /// Java: SystemNotificationServiceImpl.getDetail()
    /// 逻辑：
    /// 1. 查找通知
    /// 2. 根据 detailType 查找对应模板
    ///    - wechat: 查 template_message 表（wechat_id）
    ///    - routine: 查 template_message 表（routine_id）
    ///    - sms: 查 sms_template 表（sms_id）
    /// 3. 组装响应
    pub async fn get_detail(
        db: &DatabaseConnection,
        request: &NotificationInfoRequest,
    ) -> Result<NotificationInfoResponse> {
        let notification = Self::get_by_id_exception(db, request.id).await?;

        match request.detail_type.as_str() {
            "wechat" => {
                if notification.is_wechat == 0 {
                    return Err(Error::BadRequest("请先配置公众号模板消息".to_string()));
                }
                let tmpl = template_message::Entity::find_by_id(notification.wechat_id)
                    .one(db)
                    .await?
                    .ok_or_else(|| Error::NotFound)?;

                Ok(NotificationInfoResponse {
                    id: tmpl.id,
                    temp_id: tmpl.temp_id.clone(),
                    title: None,
                    temp_key: Some(tmpl.temp_key),
                    content: Some(tmpl.content),
                    name: Some(tmpl.name),
                    status: notification.is_wechat,
                })
            }
            "routine" => {
                if notification.is_routine == 0 {
                    return Err(Error::BadRequest("请先配置小程序订阅消息".to_string()));
                }
                let tmpl = template_message::Entity::find_by_id(notification.routine_id)
                    .one(db)
                    .await?
                    .ok_or_else(|| Error::NotFound)?;

                Ok(NotificationInfoResponse {
                    id: tmpl.id,
                    temp_id: tmpl.temp_id.clone(),
                    title: None,
                    temp_key: Some(tmpl.temp_key),
                    content: Some(tmpl.content),
                    name: Some(tmpl.name),
                    status: notification.is_routine,
                })
            }
            "sms" => {
                if notification.is_sms == 0 {
                    return Err(Error::BadRequest("请先配置短信模板".to_string()));
                }
                let sms = sms_template::Entity::find_by_id(notification.sms_id)
                    .one(db)
                    .await?
                    .ok_or_else(|| Error::NotFound)?;

                Ok(NotificationInfoResponse {
                    id: sms.id,
                    temp_id: Some(sms.temp_id),
                    title: Some(sms.title),
                    temp_key: Some(sms.temp_key),
                    content: Some(sms.content),
                    name: None,
                    status: notification.is_sms,
                })
            }
            _ => Err(Error::BadRequest("detailType参数错误，应为wechat/routine/sms".to_string())),
        }
    }
    /// 修改通知
    ///
    /// Java: SystemNotificationServiceImpl.modify()
    /// 逻辑：
    /// 1. 非sms类型时，tempId不能为空
    /// 2. 根据 detailType 分别处理:
    ///    - wechat: 更新 template_message.temp_id + notification.is_wechat
    ///    - routine: 更新 template_message.temp_id + notification.is_routine
    ///    - sms: 只更新 notification.is_sms
    pub async fn modify(
        db: &DatabaseConnection,
        request: &NotificationUpdateRequest,
    ) -> Result<bool> {
        // 非sms类型时，tempId不能为空
        if request.detail_type != "sms" {
            if request.temp_id.as_ref().map_or(true, |t| t.trim().is_empty()) {
                return Err(Error::BadRequest("模板id不能为空".to_string()));
            }
        }

        let notification = Self::get_by_id_exception(db, request.id).await?;

        match request.detail_type.as_str() {
            "wechat" => {
                if notification.is_wechat == 0 {
                    return Err(Error::BadRequest("请先为通知配置公众号模板".to_string()));
                }
                let tmpl = template_message::Entity::find_by_id(notification.wechat_id)
                    .one(db)
                    .await?
                    .ok_or_else(|| Error::NotFound)?;

                let req_temp_id = request.temp_id.as_deref().unwrap_or("");

                // 如果tempId和status都没变，直接返回
                if tmpl.temp_id.as_deref() == Some(req_temp_id) && notification.is_wechat == request.status {
                    return Ok(true);
                }

                // 更新 template_message.temp_id
                if tmpl.temp_id.as_deref() != Some(req_temp_id) {
                    let now = Local::now().naive_local();
                    let mut active: template_message::ActiveModel = tmpl.into();
                    active.temp_id = Set(Some(req_temp_id.to_string()));
                    active.update_time = Set(now);
                    active.update(db).await?;
                }

                // 更新 notification.is_wechat
                if notification.is_wechat != request.status {
                    system_notification::Entity::update_many()
                        .col_expr(system_notification::Column::IsWechat, Expr::value(request.status))
                        .filter(system_notification::Column::Id.eq(request.id))
                        .exec(db)
                        .await?;
                }
            }
            "routine" => {
                if notification.is_routine == 0 {
                    return Err(Error::BadRequest("请先为通知配置小程序订阅模板".to_string()));
                }
                let tmpl = template_message::Entity::find_by_id(notification.routine_id)
                    .one(db)
                    .await?
                    .ok_or_else(|| Error::NotFound)?;

                let req_temp_id = request.temp_id.as_deref().unwrap_or("");

                // 如果tempId和status都没变，直接返回
                if tmpl.temp_id.as_deref() == Some(req_temp_id) && notification.is_routine == request.status {
                    return Ok(true);
                }

                // 更新 template_message.temp_id
                if tmpl.temp_id.as_deref() != Some(req_temp_id) {
                    let now = Local::now().naive_local();
                    let mut active: template_message::ActiveModel = tmpl.into();
                    active.temp_id = Set(Some(req_temp_id.to_string()));
                    active.update_time = Set(now);
                    active.update(db).await?;
                }

                // 更新 notification.is_routine
                if notification.is_routine != request.status {
                    system_notification::Entity::update_many()
                        .col_expr(system_notification::Column::IsRoutine, Expr::value(request.status))
                        .filter(system_notification::Column::Id.eq(request.id))
                        .exec(db)
                        .await?;
                }
            }
            "sms" => {
                // sms只更新状态
                if notification.is_sms != request.status {
                    system_notification::Entity::update_many()
                        .col_expr(system_notification::Column::IsSms, Expr::value(request.status))
                        .filter(system_notification::Column::Id.eq(request.id))
                        .exec(db)
                        .await?;
                }
            }
            _ => {
                return Err(Error::BadRequest("detailType参数错误，应为wechat/routine/sms".to_string()));
            }
        }

        Ok(true)
    }

    // ==================== 辅助方法 ====================

    /// 根据id查找通知，不存在则报错
    async fn get_by_id_exception(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<system_notification::Model> {
        system_notification::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::BadRequest("系统通知不存在".to_string()))
    }

    /// Model 转 Response
    fn model_to_response(model: system_notification::Model) -> SystemNotificationResponse {
        SystemNotificationResponse {
            id: model.id,
            mark: model.mark,
            r#type: model.r#type,
            description: model.description,
            is_wechat: model.is_wechat,
            wechat_id: model.wechat_id,
            is_routine: model.is_routine,
            routine_id: model.routine_id,
            is_sms: model.is_sms,
            sms_id: model.sms_id,
            send_type: model.send_type,
            create_time: Some(model.create_time.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}
