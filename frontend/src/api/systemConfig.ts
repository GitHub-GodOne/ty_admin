import request from '@/utils/request';

export function configCheckUnique(pram: any) {
  const data = {
    name: pram.name,
  };
  return request({
    url: '/admin/system/config/check',
    method: 'GET',
    params: data,
  });
}

export function configDelete(pram: any) {
  const data = {
    id: pram.id,
  };
  return request({
    url: '/admin/system/config/delete',
    method: 'GET',
    params: data,
  });
}

export function configInfo(pram: any) {
  const data = {
    formId: pram.id,
  };
  return request({
    url: '/admin/system/config/info',
    method: 'GET',
    params: data,
  });
}

export function configList(pram: any) {
  const data = {
    page: pram.page,
    limit: pram.limit,
  };
  return request({
    url: '/admin/system/config/list',
    method: 'GET',
    params: data,
  });
}

export function configSave(pram: any) {
  const data = {
    systemConfigRequest: {
      desc: pram.desc,
      groupId: pram.groupId,
      info: pram.info,
      name: pram.name,
      pid: pram.pid,
      status: pram.status,
      type: pram.type,
      value: pram.value,
    },
  };
  return request({
    url: '/admin/system/config/save',
    method: 'POST',
    params: data,
  });
}

export function configSaveForm(pram: any) {
  return request({
    url: '/admin/system/config/save/form',
    method: 'POST',
    data: pram,
  });
}

export function configUpdate(pram: any) {
  const data = {
    id: pram.id,
    systemConfigRequest: pram.systemConfigRequest,
  };
  return request({
    url: '/admin/system/config/update',
    method: 'POST',
    params: data,
  });
}

export function configSaveUniq(pram: any) {
  const data = {
    key: pram.key,
    value: pram.value,
  };
  return request({
    url: '/admin/system/config/saveuniq',
    method: 'POST',
    params: data,
  });
}

export function getSiteLogoApi() {
  return request({
    url: '/admin/system/config/get/site/logo',
    method: 'GET',
  });
}

export function getUploadTypeApi() {
  return request({
    url: '/admin/system/config/get/upload/type',
    method: 'GET',
  });
}

export function getMiniDownloadUrlApi() {
  return request({
    url: '/admin/system/config/get/mini/download/url',
    method: 'GET',
  });
}

export function getTxMapKeyApi() {
  return request({
    url: '/admin/system/config/get/tx/map/key',
    method: 'GET',
  });
}

export function getHomeStyleApi() {
  return request({
    url: '/admin/system/config/get/home/page/list/style',
    method: 'GET',
  });
}

export function getAuthHostApi() {
  return request({
    url: '/admin/system/config/get/auth/host',
    method: 'GET',
  });
}

export function clearCacheApi() {
  return request({
    url: '/admin/system/config/clear/cache',
    method: 'post',
  });
}

export function changeColorApi() {
  return request({
    url: '/admin/system/config/get/change/color',
    method: 'get',
  });
}

export function saveColorApi(data: any) {
  return request({
    url: '/admin/system/config/save/change/color',
    method: 'post',
    data,
  });
}

export function savehomeStyleApi(data: any) {
  return request({
    url: '/admin/system/config/save/home/page/list/style',
    method: 'post',
    data,
  });
}

export function passAppSaveApi(data: any) {
  return request({
    url: '/admin/pass/appsave',
    method: 'post',
    data,
  });
}

export function passAppInfoApi() {
  return request({
    url: '/admin/pass/appget',
    method: 'get',
  });
}

export function frontDomainApi() {
  return request({
    url: '/public/jsconfig/get/front/domain',
    method: 'GET',
  });
}

export function mediaDomainApi() {
  return request({
    url: '/public/jsconfig/get/admin/mediadomain',
    method: 'GET',
  });
}

export function systemStateInfoApi() {
  return request({
    url: '/admin/system/status/info',
    method: 'get',
  });
}
