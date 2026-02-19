import request from '@/utils/request';

/**
 * 新增分类
 */
export function addCategroy(pram: any) {
  return request({
    url: '/admin/category/save',
    method: 'POST',
    data: {
      extra: pram.extra || '',
      name: pram.name,
      pid: Number(pram.pid ?? 0),
      sort: Number(pram.sort ?? 0),
      status: Number(pram.status ?? 1),
      type: Number(pram.type),
      url: pram.url || '',
    },
  });
}

/**
 * 分类详情
 */
export function infoCategroy(pram: any) {
  const data = {
    id: pram.id,
  };
  return request({
    url: '/admin/category/info',
    method: 'GET',
    params: data,
  });
}

/**
 * 删除分类
 */
export function deleteCategroy(pram: any) {
  const data = {
    id: pram.id,
  };
  return request({
    url: '/admin/category/delete',
    method: 'GET',
    params: data,
  });
}

/**
 * 分类列表
 */
export function listCategroy(pram: any) {
  const data = {
    limit: pram.limit,
    name: pram.name,
    page: pram.page,
    pid: pram.pid,
    status: pram.status,
    type: pram.type,
  };
  return request({
    url: '/admin/category/list',
    method: 'GET',
    params: data,
  });
}

/**
 * 分类数据tree数据
 */
export function treeCategroy(pram: any) {
  const data: any = { type: pram.type };
  if (pram.status !== undefined && pram.status !== -1) data.status = pram.status;
  if (pram.name) data.name = pram.name;
  return request({
    url: '/admin/category/list/tree',
    method: 'GET',
    params: data,
  });
}

/**
 * 更新分类
 */
export function updateCategroy(pram: any) {
  return request({
    url: '/admin/category/update',
    method: 'POST',
    params: { id: pram.id },
    data: {
      extra: pram.extra || '',
      name: pram.name,
      pid: Number(pram.pid ?? 0),
      sort: Number(pram.sort ?? 0),
      status: Number(pram.status ?? 1),
      type: Number(pram.type),
      url: pram.url || '',
    },
  });
}

/**
 * 根据id集合查询对应分类列表
 */
export function categroyByIds(pram: any) {
  const data = {
    ids: pram.ids,
  };
  return request({
    url: '/admin/category/list/ids',
    method: 'GET',
    params: data,
  });
}

/**
 * 修改 显示关闭状态
 */
export function categroyUpdateStatus(id: any) {
  return request({
    url: '/admin/category/updateStatus',
    method: 'GET',
    params: { id },
  });
}

/**
 * 文章详情
 */
export function articleInfoApi(params: any) {
  return request({
    url: '/admin/article/info',
    method: 'GET',
    params,
  });
}
