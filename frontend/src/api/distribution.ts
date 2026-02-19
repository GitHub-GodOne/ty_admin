import request from '@/utils/request';

/**
 * 分销设置 -- 详情
 */
export function configApi() {
  return request({
    url: '/admin/store/retail/spread/manage/get',
    method: 'get',
  });
}

/**
 * 分销设置 -- 表单提交
 */
export function configUpdateApi(data: any) {
  return request({
    url: '/admin/store/retail/spread/manage/set',
    method: 'post',
    data,
  });
}

/**
 * 分销员 -- 列表
 */
export function promoterListApi(params: any) {
  return request({
    url: '/admin/store/retail/list',
    method: 'get',
    params,
  });
}

/**
 * 推广人 -- 列表
 */
export function spreadListApi(params: any, data: any) {
  return request({
    url: '/admin/store/retail/spread/userlist',
    method: 'post',
    params,
    data,
  });
}

/**
 * 推广人订单 -- 列表
 */
export function spreadOrderListApi(params: any, data: any) {
  return request({
    url: '/admin/store/retail/spread/orderlist',
    method: 'post',
    params,
    data,
  });
}

/**
 * 推广人 -- 清除上级推广人
 */
export function spreadClearApi(id: any) {
  return request({
    url: `/admin/store/retail/spread/clean/${id}`,
    method: 'get',
  });
}

/**
 * 分销统计
 */
export function spreadStatisticsApi(params: any) {
  return request({
    url: '/admin/store/retail/statistics',
    method: 'get',
    params,
  });
}
