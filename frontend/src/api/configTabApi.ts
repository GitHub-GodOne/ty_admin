import request from '@/utils/request';

export function configTabDelete(pram: any) {
  const data = {
    id: pram.id,
  };
  return request({
    url: 'admin/system/config/tab/delete',
    method: 'GET',
    params: data,
  });
}

export function configTabInfo(pram: any) {
  const data = {
    id: pram.id,
  };
  return request({
    url: 'admin/system/config/tab/info',
    method: 'GET',
    params: data,
  });
}

export function configTabList(pram: any) {
  const data = {
    id: pram.id,
    engTitle: pram.engTitle,
    icon: pram.icon,
    info: pram.info,
    title: pram.title,
    parameter: pram.parameter,
    status: pram.status,
    page: pram.page,
    limit: pram.limit,
  };
  return request({
    url: 'admin/system/config/tab/list',
    method: 'POST',
    params: data,
  });
}

export function configSave(pram: any) {
  const data = {
    id: pram.id,
    configTabId: pram.configTabId,
    desc: pram.desc,
    high: pram.high,
    info: pram.info,
    inputType: pram.inputType,
    menuName: pram.menuName,
    parameter: pram.parameter,
    required: pram.required,
    sort: pram.sort,
    status: pram.status,
    type: pram.type,
    updateType: pram.updateType,
    value: pram.value,
    width: pram.width,
  };
  return request({
    url: 'admin/system/config/save',
    method: 'POST',
    params: data,
  });
}

export function configUpdate(pram: any) {
  const data = {
    id: pram.id,
    configTabId: pram.configTabId,
    desc: pram.desc,
    high: pram.high,
    info: pram.info,
    inputType: pram.inputType,
    menuName: pram.menuName,
    parameter: pram.parameter,
    required: pram.required,
    sort: pram.sort,
    status: pram.status,
    type: pram.type,
    updateType: pram.updateType,
    value: pram.value,
    width: pram.width,
  };
  return request({
    url: 'admin/system/config/update',
    method: 'POST',
    params: data,
  });
}
