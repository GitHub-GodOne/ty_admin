import request from '@/utils/request';

export function diyListApi(params: any) {
  return request({
    url: '/admin/pagediy/list',
    method: 'get',
    params,
  });
}
