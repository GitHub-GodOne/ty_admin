import request from '@/utils/request';

export function getFormConfigInfo(pram: any) {
  const data = {
    id: pram.id,
  };
  return request({
    url: '/admin/system/form/temp/info',
    method: 'GET',
    params: data,
  });
}

export function getFormConfigList(pram: any) {
  const data = {
    keywords: pram.keywords,
    page: pram.page,
    limit: pram.limit,
  };
  return request({
    url: '/admin/system/form/temp/list',
    method: 'GET',
    params: data,
  });
}

export function getFormConfigSave(pram: any) {
  const data = {
    content: pram.content,
    info: pram.info,
    name: pram.name,
  };
  return request({
    url: '/admin/system/form/temp/save',
    method: 'POST',
    data: data,
  });
}

export function getFormConfigEdit(pram: any) {
  const params = { id: pram.id };
  const data = {
    content: pram.content,
    info: pram.info,
    name: pram.name,
  };
  return request({
    url: '/admin/system/form/temp/update',
    method: 'POST',
    params: params,
    data: data,
  });
}

export function notificationListApi(pram: any) {
  const data = {
    sendType: pram.sendType,
  };
  return request({
    url: '/admin/system/notification/list',
    method: 'GET',
    params: data,
  });
}

export function notificationRoutine(id: number | string) {
  return request({
    url: `/admin/system/notification/routine/switch/${id}`,
    method: 'post',
  });
}

export function notificationWechat(id: number | string) {
  return request({
    url: `/admin/system/notification/wechat/switch/${id}`,
    method: 'post',
  });
}

export function notificationSms(id: number | string) {
  return request({
    url: `/admin/system/notification/sms/switch/${id}`,
    method: 'post',
  });
}

export function notificationDetail(param: any) {
  const data = {
    detailType: param.type,
    id: param.id,
  };
  return request({
    url: '/admin/system/notification/detail',
    method: 'get',
    params: data,
  });
}

export function notificationUpdate(param: any) {
  const data = {
    detailType: param.type,
    id: param.id,
    status: param.status,
    tempId: param.tempId,
  };
  return request({
    url: '/admin/system/notification/update',
    method: 'post',
    data,
  });
}

export function knowUserCaptchaApi(data: any) {
  return request({
    url: '/public/safety/get',
    method: 'post',
    data,
  });
}

export function knowUserSmsCaptchaApi(data: any) {
  return request({
    url: '/public/safety/check',
    method: 'post',
    data,
  });
}
