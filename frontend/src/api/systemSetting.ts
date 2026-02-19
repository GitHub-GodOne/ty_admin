import request from '@/utils/request';

export function systemConfigCheck(pram: any) {
  const data = {
    name: pram.name,
  };
  return request({
    url: '/admin/system/config/check',
    method: 'GET',
    params: data,
  });
}

export function systemConfigInfo(pram: any) {
  const data = {
    formId: pram.id,
  };
  return request({
    url: '/admin/system/config/info',
    method: 'GET',
    params: data,
  });
}

export function systemConfigSave(pram: any) {
  return request({
    url: '/admin/system/config/save/form',
    method: 'POST',
    data: pram,
  });
}

export function fileFileApi(data: any, params: any) {
  return request({
    url: '/admin/upload/file',
    method: 'POST',
    params,
    data,
  });
}

export function fileImageApi(data: any, params: any) {
  return request({
    url: '/admin/upload/image',
    method: 'POST',
    params,
    data,
  });
}

export function fileListApi(params: any) {
  return request({
    url: '/admin/system/attachment/list',
    method: 'get',
    params,
  });
}

export function fileDeleteApi(id: number | string) {
  return request({
    url: '/admin/system/attachment/delete',
    method: 'get',
    params: { id },
  });
}

export function attachmentMoveApi(data: any) {
  return request({
    url: '/admin/system/attachment/move',
    method: 'post',
    data,
  });
}

export function wechatUploadApi(data: any, params: any) {
  return request({
    url: '/admin/wechat/media/upload',
    method: 'post',
    data,
    params,
  });
}
