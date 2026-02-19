import request from '@/utils/request';

export function addRole(pram: any) {
  const data = {
    level: pram.level,
    roleName: pram.roleName,
    status: pram.status,
    rules: pram.rules,
  };
  return request({
    url: '/admin/system/role/save',
    method: 'POST',
    data: data,
  });
}

export function delRole(pram: { id: number | string }) {
  const data = {
    id: pram.id,
  };
  return request({
    url: '/admin/system/role/delete',
    method: 'GET',
    params: data,
  });
}

export function getInfo(id: number | string) {
  return request({
    url: '/admin/system/role/info',
    method: 'GET',
    params: { id },
  });
}

export function getRoleList(pram: any) {
  const data = {
    createTime: pram.createTime,
    updateTime: pram.updateTime,
    level: pram.level,
    page: pram.page,
    limit: pram.limit,
    roleName: pram.roleName,
    rules: pram.rules,
    status: pram.status,
  };
  return request({
    url: '/admin/system/role/list',
    method: 'get',
    params: data,
  });
}

export function updateRole(pram: any) {
  const data = {
    id: pram.id,
    roleName: pram.roleName,
    rules: pram.rules,
    status: pram.status,
  };
  return request({
    url: '/admin/system/role/update',
    method: 'post',
    params: { id: pram.id },
    data: data,
  });
}

export function updateRoleStatus(pram: { id: number | string; status: number | string }) {
  return request({
    url: '/admin/system/role/updateStatus',
    method: 'get',
    params: { id: pram.id, status: pram.status },
  });
}

export function menuCacheList() {
  return request({
    url: '/admin/system/menu/cache/tree',
    method: 'get',
  });
}
