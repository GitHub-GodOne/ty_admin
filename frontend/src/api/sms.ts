import request from '@/utils/request';

/**
 * 短信发送记录 -- 列表
 */
export function smsLstApi(params: any) {
  return request({
    url: '/admin/pass/user/record',
    method: 'get',
    params,
  });
}

/**
 * 短信账户 -- 登录
 */
export function configApi(data: any) {
  return request({
    url: '/admin/pass/login',
    method: 'post',
    data,
  });
}

/**
 * 短信账户 -- 获取验证码
 */
export function captchaApi(params: any) {
  return request({
    url: '/admin/pass/sendUserCode',
    method: 'get',
    params,
  });
}

/**
 * 短信账户 -- 注册
 */
export function registerApi(data: any) {
  return request({
    url: '/admin/pass/register',
    method: 'post',
    data,
  });
}

/**
 * 短信账户 -- 是否登录
 */
export function isLoginApi() {
  return request({
    url: '/admin/pass/isLogin',
    method: 'get',
  });
}

/**
 * 短信账户 -- 退出登录
 */
export function logoutApi() {
  return request({
    url: '/admin/pass/logout',
    method: 'get',
  });
}

/**
 * 短信账户 -- 剩余条数
 */
export function smsNumberApi() {
  return request({
    url: '/admin/sms/logout',
    method: 'get',
  });
}

/**
 * 短信模板 -- 列表
 */
export function smsTempLstApi(params: any) {
  return request({
    url: '/admin/sms/temps',
    method: 'get',
    params,
  });
}

/**
 * 短信购买 -- 支付套餐
 */
export function smsPriceApi(params: any) {
  return request({
    url: '/admin/pass/meal/list',
    method: 'get',
    params,
  });
}

/**
 * 短信购买 -- 支付码
 */
export function payCodeApi(data: any) {
  return request({
    url: '/admin/pass/meal/code',
    method: 'post',
    data,
  });
}

/**
 * 短信模板 -- 添加表单
 */
export function tempCreateApi(data: any) {
  return request({
    url: '/admin/sms/temp/apply',
    method: 'post',
    data,
  });
}

/**
 * 短信 -- 用户信息
 */
export function smsInfoApi() {
  return request({
    url: '/admin/pass/info',
    method: 'get',
  });
}

/**
 * 短信 -- 短信提醒开关保存
 */
export function smsSaveApi(params: any) {
  return request({
    url: '/admin/sms/config/save',
    method: 'post',
    params,
  });
}

/**
 * 短信 -- 修改密码
 */
export function updatePasswordApi(data: any) {
  return request({
    url: '/admin/pass/update/password',
    method: 'post',
    data,
  });
}

/**
 * 短信 -- 修改手机号
 */
export function updateHoneApi(data: any) {
  return request({
    url: '/admin/pass/update/phone',
    method: 'post',
    data,
  });
}

/**
 * 一号通 -- 服务开通
 */
export function serviceOpenApi(data: any) {
  return request({
    url: '/admin/pass/service/open',
    method: 'post',
    data,
  });
}

/**
 * 一号通 -- 电子面单模板
 */
export function exportTempApi(params: any) {
  return request({
    url: '/admin/express/template',
    method: 'get',
    params,
  });
}

/**
 * 全部物流公司
 */
export function expressAllApi(params: any) {
  return request({
    url: 'admin/express/all',
    method: 'get',
    params,
  });
}

/**
 * 修改签名
 */
export function smsSignApi(data: any) {
  return request({
    url: 'admin/sms/modify/sign',
    method: 'post',
    data,
  });
}

/**
 * 修改手机号验证账号密码
 */
export function phoneValidatorApi(data: any) {
  return request({
    url: 'admin/pass/update/phone/validator',
    method: 'post',
    data,
  });
}

/**
 * 一号通 商家寄件 快递列表
 */
export function shipmentExpressApi() {
  return request({
    url: '/admin/pass/shipment/express',
    method: 'get',
  });
}
