import request from '@/utils/request';

export function groupDelete(pram: any) {
  const data = {
    id: pram.id,
  };
  return request({
    url: '/admin/system/group/delete',
    method: 'GET',
    params: data,
  });
}

export function groupInfo(pram: any) {
  const data = {
    id: pram.id,
  };
  return request({
    url: '/admin/system/group/info',
    method: 'GET',
    params: data,
  });
}

export function groupList(pram: any) {
  const data = {
    keywords: pram.keywords,
    page: pram.page,
    limit: pram.limit,
  };
  return request({
    url: '/admin/system/group/list',
    method: 'GET',
    params: data,
  });
}

export function groupDataList(pram: any) {
  const data = {
    gid: pram.gid,
    keywords: pram.keywords,
    page: pram.page,
    limit: pram.limit,
  };
  return request({
    url: '/admin/system/group/data/list',
    method: 'GET',
    params: data,
  });
}

export function groupSave(pram: any) {
  const data = {
    formId: pram.formId,
    info: pram.info,
    name: pram.name,
  };
  return request({
    url: '/admin/system/group/save',
    method: 'POST',
    params: data,
  });
}

export function groupEdit(pram: any) {
  const data = {
    formId: pram.formId,
    info: pram.info,
    name: pram.name,
    id: pram.id,
  };
  return request({
    url: '/admin/system/group/update',
    method: 'POST',
    params: data,
  });
}

export function designListApi() {
  return (request as any).get('/admin/page/layout/index');
}

export function goodDesignList(pram: any) {
  const data = {
    gid: pram.gid,
  };
  return request({
    url: '/admin/system/group/data/list',
    method: 'GET',
    params: data,
  });
}

export function SaveDataApi(data: any, url: string) {
  return request({
    url: url,
    method: 'POST',
    data,
  });
}

export function getDataApi(data: any) {
  return request({
    url: '/admin/page/layout/category/config',
    method: 'GET',
    data,
  });
}

export function themeSave(params: any) {
  return request({
    url: '/admin/system/config/saveuniq',
    method: 'post',
    params,
  });
}

export function getBottomNavigationApi() {
  return request({
    url: '/admin/page/layout/bottom/navigation/get',
    method: 'GET',
  });
}
