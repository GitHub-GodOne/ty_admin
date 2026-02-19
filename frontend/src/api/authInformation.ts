import request from '@/utils/request';

export function copyrightInfoApi() {
  return request({
    url: '/admin/copyright/get/info',
    method: 'get',
  });
}

export function accountDetectionApi(data: any) {
  return request({
    url: '/admin/login/account/detection',
    method: 'post',
    data,
  });
}
