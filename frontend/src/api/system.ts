import request from '@/utils/request';

export function formatLstApi(data: any) {
  return (request as any).get({
    url: '/admin/product/save',
    method: 'POST',
    data,
  });
}

export function attachmentCreateApi() {
  return (request as any).get('system/attachment/category/create/form');
}

export function attachmentUpdateApi(id: number | string) {
  return (request as any).get(`system/attachment/category/update/form/${id}`);
}

export function attachmentDeleteApi(id: number | string) {
  return (request as any).delete(`system/attachment/category/delete/${id}`);
}

export function attachmentListApi(data: any) {
  return (request as any).get('system/attachment/lst', data);
}

export function picDeleteApi(id: any) {
  return (request as any).delete('system/attachment/delete', id);
}

export function categoryApi(ids: any, attachment_category_id: any) {
  return (request as any).post('system/attachment/category', { ids, attachment_category_id });
}
