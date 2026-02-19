import request from '@/utils/request';

export function productDataApi(params: any) {
  return request({
    url: '/admin/statistics/product/data',
    method: 'GET',
    params,
  });
}

export function productRankApi(params: any) {
  return request({
    url: '/admin/statistics/product/ranking',
    method: 'GET',
    params,
  });
}

export function productTrendApi(params: any) {
  return request({
    url: '/admin/statistics/product/trend',
    method: 'GET',
    params,
  });
}

export function tradeDataApi() {
  return request({
    url: '/admin/statistics/trade/data',
    method: 'GET',
  });
}

export function tradeOverviewApi(params: any) {
  return request({
    url: '/admin/statistics/trade/overview',
    method: 'GET',
    params,
  });
}

export function tradeTrendApi(params: any) {
  return request({
    url: '/admin/statistics/trade/trend',
    method: 'GET',
    params,
  });
}

export function userTotalData() {
  return request({
    url: '/admin/statistics/user/total/data',
    method: 'GET',
  });
}

export function userAreaData() {
  return request({
    url: '/admin/statistics/user/area',
    method: 'GET',
  });
}

export function userChannelData() {
  return request({
    url: '/admin/statistics/user/channel',
    method: 'GET',
  });
}

export function userOverviewData(params: any) {
  return request({
    url: '/admin/statistics/user/overview',
    method: 'GET',
    params,
  });
}

export function userSexData() {
  return request({
    url: '/admin/statistics/user/sex',
    method: 'GET',
  });
}

export function userOverviewListApi(params: any) {
  return request({
    url: '/admin/statistics/user/overview/list',
    method: 'GET',
    params,
  });
}
