import request from '@/utils/request';

export function storeListApi(data: any) {
  return request({
    url: '/admin/system/store/list',
    method: 'get',
    params: data,
  });
}

export function storeGetCountApi(params: any) {
  return request({
    url: '/admin/system/store/getCount',
    method: 'get',
    params,
  });
}

export function storeUpdateStatusApi(data: any) {
  return request({
    url: '/admin/system/store/update/status',
    method: 'get',
    params: data,
  });
}

export function storeDeleteApi(data: any) {
  return request({
    url: '/admin/system/store/delete',
    method: 'get',
    params: data,
  });
}

export function allDeleteApi(params: any) {
  return request({
    url: '/admin/system/store/completely/delete',
    method: 'get',
    params,
  });
}

export function storeSaveApi(data: any) {
  return request({
    url: '/admin/system/store/save',
    method: 'post',
    data,
  });
}

export function storeInfoApi(data: any) {
  return request({
    url: '/admin/system/store/info',
    method: 'get',
    params: data,
  });
}

export function storeUpdateApi(data: any, id: number | string) {
  return request({
    url: '/admin/system/store/update',
    method: 'post',
    params: { id: id },
    data,
  });
}

export function storeRecoveryApi(params: any) {
  return request({
    url: '/admin/system/store/recovery',
    method: 'get',
    params,
  });
}

export function storeStaffListApi(data: any) {
  return request({
    url: '/admin/system/store/staff/list',
    method: 'get',
    params: data,
  });
}

export function storeStaffSaveApi(data: any) {
  return request({
    url: '/admin/system/store/staff/save',
    method: 'POST',
    params: data,
  });
}

export function storeStaffDeleteApi(data: any) {
  return request({
    url: '/admin/system/store/staff/delete',
    method: 'get',
    params: data,
  });
}

export function storeStaffUpdateApi(data: any) {
  return request({
    url: '/admin/system/store/staff/update',
    method: 'POST',
    params: data,
  });
}

export function storeStaffInfoApi(id: any) {
  return request({
    url: '/admin/system/store/staff/info',
    method: 'get',
    params: id,
  });
}

export function storeStaffUpdateStatusApi(data: any) {
  return request({
    url: '/admin/system/store/staff/update/status',
    method: 'get',
    params: data,
  });
}

export function userListApi(data: any) {
  return request({
    url: '/admin/wechat/user/list',
    method: 'get',
    params: data,
  });
}

export function orderListApi(params: any) {
  return request({
    url: '/admin/system/store/order/list',
    method: 'post',
    params,
  });
}
