import request from '@/utils/request';

export function groupDataDelete(pram: any) {
  const data = {
    id: pram.id,
  };
  return request({
    url: '/admin/system/group/data/delete',
    method: 'GET',
    params: data,
  });
}

export function groupDataInfo(pram: any) {
  const data = {
    id: pram.id,
  };
  return request({
    url: '/admin/system/group/data/info',
    method: 'GET',
    params: data,
  });
}

export function groupDataList(pram: any) {
  const data = {
    gid: pram.gid,
    keywords: pram.keywords,
    status: pram.status,
    page: pram.page,
    limit: pram.limit,
  };
  return request({
    url: '/admin/system/group/data/list',
    method: 'GET',
    params: data,
  });
}

export function groupDataSave(pram: any) {
  return request({
    url: '/admin/system/group/data/save',
    method: 'POST',
    data: pram,
  });
}

export function groupDataEdit(pram: any, id: number | string) {
  return request({
    url: '/admin/system/group/data/update',
    method: 'POST',
    data: pram,
    params: { id: id },
  });
}
