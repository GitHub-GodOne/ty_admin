import request from '@/utils/request';

export function orderListApi(params: any) {
  return request({
    url: '/admin/store/order/list',
    method: 'get',
    params,
  });
}

export function orderStatusNumApi(params: any) {
  return request({
    url: '/admin/store/order/status/num',
    method: 'get',
    params,
  });
}

export function orderListDataApi(params: any) {
  return request({
    url: '/admin/store/order/list/data',
    method: 'get',
    params,
  });
}

export function orderDeleteApi(params: any) {
  return request({
    url: '/admin/store/order/delete',
    method: 'get',
    params,
  });
}

export function orderUpdateApi(data: any, params: any) {
  return request({
    url: '/admin/store/order/update',
    method: 'post',
    data,
    params,
  });
}

export function orderLogApi(params: any) {
  return request({
    url: '/admin/store/order/status/list',
    method: 'get',
    params,
  });
}

export function orderDetailApi(params: any) {
  return request({
    url: '/admin/store/order/info',
    method: 'get',
    params,
  });
}

export function orderMarkApi(params: any) {
  return request({
    url: '/admin/store/order/mark',
    method: 'post',
    params,
  });
}

export function orderSendApi(data: any) {
  return request({
    url: '/admin/store/order/send',
    method: 'post',
    data,
  });
}

export function orderRefuseApi(params: any) {
  return request({
    url: '/admin/store/order/refund/refuse',
    method: 'get',
    params,
  });
}

export function orderRefundApi(params: any) {
  return request({
    url: '/admin/store/order/refund',
    method: 'get',
    params,
  });
}

export function writeUpdateApi(vCode: string) {
  return request({
    url: `/admin/store/order/writeUpdate/${vCode}`,
    method: 'get',
  });
}

export function writeConfirmApi(vCode: string) {
  return request({
    url: `/admin/store/order/writeConfirm/${vCode}`,
    method: 'get',
  });
}

export function orderStatisticsApi() {
  return request({
    url: '/admin/store/order/statistics',
    method: 'get',
  });
}

export function statisticsDataApi(params: any) {
  return request({
    url: '/admin/store/order/statisticsData',
    method: 'get',
    params,
  });
}

export function updatePriceApi(data: any) {
  return request({
    url: 'admin/store/order/update/price',
    method: 'post',
    data,
  });
}

export function orderTimeApi(params: any) {
  return request({
    url: '/admin/store/order/time',
    method: 'get',
    params,
  });
}

export function sheetInfoApi() {
  return request({
    url: '/admin/store/order/sheet/info',
    method: 'get',
  });
}

export function getLogisticsInfoApi(params: any) {
  return request({
    url: '/admin/store/order/getLogisticsInfo',
    method: 'get',
    params,
  });
}

export function companyGetListApi() {
  return request({
    url: '/admin/pay/component/delivery/company/get/list',
    method: 'get',
  });
}

export function videoSendApi(data: any) {
  return request({
    url: '/admin/store/order/video/send',
    method: 'post',
    data,
  });
}

export function orderPrint(id: number | string) {
  return request({
    url: `/admin/yly/print/${id}`,
    method: 'get',
  });
}

export function updateTrackingNumberApi(data: any) {
  return request({
    url: '/admin/store/order/update/tracking/number',
    method: 'post',
    data,
  });
}
