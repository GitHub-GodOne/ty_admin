import request from '@/utils/request';

export function getRoleById(pram: { roles: string | number }) {
  return request({
    url: `/admin/system/role/info/${pram.roles}`,
    method: 'GET',
  });
}

export function menuListApi() {
  return request({
    url: '/admin/getMenus',
    method: 'GET',
  });
}
