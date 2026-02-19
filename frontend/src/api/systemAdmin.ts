import request from '@/utils/request';

export function getMenu() {
  return request({
    url: '/admin/system/role/testMenu',
    method: 'GET',
  });
}

export function adminDel(pram: { id: number | string }) {
  const data = {
    id: pram.id,
  };
  return request({
    url: '/admin/system/admin/delete',
    method: 'GET',
    params: data,
  });
}

export function adminInfo(pram: { id: number | string }) {
  const data = {
    id: pram.id,
  };
  return request({
    url: '/admin/system/admin/info',
    method: 'GET',
    params: data,
  });
}

export function adminList(params: any) {
  return request({
    url: '/admin/system/admin/list',
    method: 'GET',
    params,
  });
}

export function adminAdd(pram: any) {
  const data = {
    account: pram.account,
    level: pram.level,
    pwd: pram.pwd,
    realName: pram.realName,
    roles: pram.roles.join(','),
    status: pram.status,
    phone: pram.phone,
  };
  return request({
    url: '/admin/system/admin/save',
    method: 'POST',
    data: data,
  });
}

export function adminUpdate(pram: any) {
  const data = {
    account: pram.account,
    phone: pram.phone,
    pwd: pram.pwd,
    roles: pram.roles,
    realName: pram.realName,
    status: pram.status,
    id: pram.id,
    isDel: pram.isDel,
  };
  return request({
    url: '/admin/system/admin/update',
    method: 'POST',
    data,
  });
}

export function updateStatusApi(params: any) {
  return request({
    url: '/admin/system/admin/updateStatus',
    method: 'get',
    params,
  });
}

export function updateIsSmsApi(params: any) {
  return request({
    url: '/admin/system/admin/update/isSms',
    method: 'get',
    params,
  });
}

export function menuListApi(params: { menuType?: string; name?: string }) {
  return request({
    url: '/admin/system/menu/list',
    method: 'get',
    params,
  });
}

export function menuAdd(data: any) {
  return request({
    url: '/admin/system/menu/save',
    method: 'post',
    data,
  });
}

export function menuDelete(id: number | string) {
  return request({
    url: '/admin/system/menu/delete',
    method: 'get',
    params: { id },
  });
}

export function menuInfo(id: number | string) {
  return request({
    url: '/admin/system/menu/info',
    method: 'get',
    params: { id },
  });
}

export function menuUpdate(data: any) {
  return request({
    url: '/admin/system/menu/update',
    method: 'post',
    data,
  });
}

export function menuUpdateShowStatus(params: any) {
  return request({
    url: '/admin/system/menu/updateShowStatus',
    method: 'post',
    params,
  });
}

export function sensitiveListApi(params: any) {
  return request({
    url: '/admin/log/sensitive/list',
    method: 'get',
    params,
  });
}

export function adminNameUpdateApi(data: any) {
  return request({
    url: '/admin/login/admin/update',
    method: 'post',
    data,
  });
}

export function adminPwdUpdateApi(data: any) {
  return request({
    url: '/admin/login/update/password',
    method: 'post',
    data,
  });
}
