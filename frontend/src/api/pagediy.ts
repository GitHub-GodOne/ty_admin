import request from '@/utils/request';

export function pagediyListApi(params: any) {
  return request({
    url: '/admin/pagediy/list',
    method: 'get',
    params,
  });
}

export function pagediySaveApi(data: any) {
  return request({
    url: '/admin/pagediy/save',
    method: 'post',
    data,
  });
}

export function pagediyUpdateApi(data: any) {
  return request({
    url: '/admin/pagediy/update',
    method: 'post',
    data,
  });
}

export function pagediyInfoApi(id: number | string) {
  return request({
    url: '/admin/pagediy/info',
    method: 'get',
    params: { id },
  });
}

export function pagediyDeleteApi(params: any) {
  return request({
    url: '/admin/pagediy/delete',
    method: 'get',
    params,
  });
}

export function pagediySetdefaultApi(id: number | string) {
  return request({
    url: '/admin/pagediy/setdefault',
    method: 'get',
    params: { id },
  });
}

export function pagediyGetSetHome() {
  return request({
    url: '/admin/pagediy/getdefault',
    method: 'get',
  });
}

export function wechatQrcodeApi(data: any) {
  return request({
    url: '/public/wechat/mini/get/qrcode',
    method: 'post',
    data,
  });
}
