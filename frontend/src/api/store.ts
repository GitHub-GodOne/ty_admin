import request from '@/utils/request';

export function productCreateApi(data: any) {
  return request({
    url: '/admin/store/product/save',
    method: 'POST',
    data,
  });
}

export function productUpdateApi(data: any) {
  return request({
    url: '/admin/store/product/update',
    method: 'POST',
    data,
  });
}

export function productDetailApi(id: number | string) {
  return request({
    url: '/admin/store/product/info',
    method: 'GET',
    params: { id },
  });
}

export function productDeleteApi(id: number | string, type: number | string) {
  return request({
    url: '/admin/store/product/delete',
    method: 'get',
    params: { id, type },
  });
}

export function productHeadersApi(params: any) {
  return request({
    url: '/admin/store/product/tabs/headers',
    method: 'GET',
    params,
  });
}

export function productLstApi(params: any) {
  return request({
    url: '/admin/store/product/list',
    method: 'GET',
    params,
  });
}

export function categoryApi(params: any) {
  return request({
    url: '/admin/category/list/tree',
    method: 'GET',
    params,
  });
}

export function putOnShellApi(id: number | string) {
  return request({
    url: '/admin/store/product/putOnShell',
    method: 'GET',
    params: { id },
  });
}

export function offShellApi(id: number | string) {
  return request({
    url: '/admin/store/product/offShell',
    method: 'GET',
    params: { id },
  });
}

export function templateListApi(params: any) {
  return request({
    url: '/admin/store/product/rule/list',
    method: 'GET',
    params,
  });
}

export function stockAddApi(data: any) {
  return request({
    url: '/admin/store/product/quick/stock/add',
    method: 'POST',
    data,
  });
}

export function attrDeleteApi(ids: number | string) {
  return request({
    url: '/admin/store/product/rule/delete',
    method: 'get',
    params: { ids: String(ids) },
  });
}

export function attrCreatApi(data: any) {
  return request({
    url: '/admin/store/product/rule/save',
    method: 'POST',
    data,
  });
}

export function attrEditApi(data: any) {
  return request({
    url: '/admin/store/product/rule/update',
    method: 'POST',
    data,
  });
}

export function attrInfoApi(id: number | string) {
  return request({
    url: '/admin/store/product/rule/info',
    method: 'GET',
    params: { id },
  });
}

export function replyListApi(params: any) {
  return request({
    url: '/admin/store/product/reply/list',
    method: 'GET',
    params,
  });
}

export function replyCreatApi(data: any) {
  return request({
    url: '/admin/store/product/reply/save',
    method: 'POST',
    data,
  });
}

export function replyEditApi(data: any) {
  return request({
    url: '/admin/store/product/reply/update',
    method: 'POST',
    data,
  });
}

export function replyInfoApi(id: number | string) {
  return request({
    url: '/admin/store/product/reply/info',
    method: 'GET',
    params: { id },
  });
}

export function replyDeleteApi(id: number | string) {
  return request({
    url: '/admin/store/product/reply/delete',
    method: 'GET',
    params: { id },
  });
}

export function replyCommentApi(data: any) {
  return request({
    url: '/admin/store/product/reply/comment',
    method: 'post',
    data,
  });
}

export function productExportApi(params: any) {
  return request({
    url: '/admin/export/excel/product',
    method: 'get',
    params,
  });
}

export function importProductApi(params: any) {
  return request({
    url: '/admin/store/product/importProduct',
    method: 'post',
    params,
  });
}

export function copyProductApi(data: any) {
  return request({
    url: '/admin/store/product/copy/product',
    method: 'post',
    data,
  });
}

export function restoreApi(id: number | string) {
  return request({
    url: '/admin/store/product/restore',
    method: 'get',
    params: { id },
  });
}

export function productExcelApi(params: any) {
  return request({
    url: '/admin/export/excel/product',
    method: 'get',
    params,
  });
}

export function copyConfigApi() {
  return request({
    url: '/admin/store/product/copy/config',
    method: 'post',
  });
}

export function orderExcelApi(params: any) {
  return request({
    url: '/admin/export/excel/order',
    method: 'get',
    params,
  });
}

export function productListbyidsApi(ids: string) {
  return request({
    url: '/admin/store/product/listids',
    method: 'get',
    params: { ids },
  });
}
