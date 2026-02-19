import request from '@/utils/request';

export function viewModelApi() {
  return request({
    url: '/admin/statistics/home/index',
    method: 'GET',
  });
}

export function chartUserApi() {
  return request({
    url: '/admin/statistics/home/chart/user',
    method: 'get',
  });
}

export function chartBuyApi() {
  return request({
    url: '/admin/statistics/home/chart/user/buy',
    method: 'get',
  });
}

export function chartOrder30Api() {
  return request({
    url: '/admin/statistics/home/chart/order',
    method: 'get',
  });
}

export function chartOrderMonthApi() {
  return request({
    url: '/admin/statistics/home/chart/order/month',
    method: 'get',
  });
}

export function chartOrderWeekApi() {
  return request({
    url: '/admin/statistics/home/chart/order/week',
    method: 'get',
  });
}

export function chartOrderYearApi() {
  return request({
    url: '/admin/statistics/home/chart/order/year',
    method: 'get',
  });
}

export function businessData() {
  return request({
    url: '/admin/statistics/home/operating/data',
    method: 'get',
  });
}
