import request from '@/utils/request';

export function treeCategoryApi(params: { type: number; status: number }) {
  return request({
    url: '/admin/category/list/tree',
    method: 'GET',
    params,
  });
}
