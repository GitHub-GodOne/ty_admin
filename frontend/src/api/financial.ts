import request from '@/utils/request';

/**
 * 提现申请 列表
 */
export function applyListApi(params: any) {
  return request({
    url: '/admin/finance/apply/list',
    method: 'get',
    params,
  });
}

/**
 * 提现申请 金额
 */
export function applyBalanceApi(params: any) {
  return request({
    url: '/admin/finance/apply/balance',
    method: 'post',
    params,
  });
}

/**
 * 提现申请 修改
 */
export function applyUpdateApi(params: any) {
  return request({
    url: '/admin/finance/apply/update',
    method: 'post',
    params,
  });
}

/**
 * 提现申请 审核
 */
export function applyStatusApi(params: any, data: any) {
  return request({
    url: '/admin/finance/apply/apply',
    method: 'post',
    params,
    data,
  });
}

/**
 * 充值 列表
 */
export function topUpLogListApi(params: any) {
  return request({
    url: '/admin/user/topUpLog/list',
    method: 'get',
    params,
  });
}

/**
 * 充值 金额
 */
export function balanceApi() {
  return request({
    url: '/admin/user/topUpLog/balance',
    method: 'post',
  });
}

/**
 * 充值 删除
 */
export function topUpLogDeleteApi(params: any) {
  return request({
    url: '/admin/user/topUpLog/delete',
    method: 'get',
    params,
  });
}

/**
 * 充值 退款
 */
export function refundApi(data: any) {
  return request({
    url: '/admin/user/topUpLog/refund',
    method: 'post',
    data,
  });
}

/**
 * 资金监控 列表
 */
export function monitorListApi(params: any) {
  return request({
    url: '/admin/finance/founds/monitor/list',
    method: 'get',
    params,
  });
}

/**
 * 资金监控 明细类型
 */
export function monitorListOptionApi() {
  return request({
    url: '/admin/finance/founds/monitor/list/option',
    method: 'get',
  });
}

/**
 * 佣金记录 列表
 */
export function brokerageListApi(params: any) {
  return request({
    url: '/admin/finance/founds/monitor/brokerage/record',
    method: 'get',
    params,
  });
}
