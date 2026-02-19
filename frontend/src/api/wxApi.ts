import request from '@/utils/request';

export function menuCreate(data: any) {
  return request({
    url: '/admin/wechat/menu/public/create',
    method: 'post',
    params: data,
  });
}

export function menuDelete(data: any) {
  return request({
    url: '/admin/wechat/menu/public/delete',
    method: 'post',
    params: data,
  });
}

export function menuInfo(pram: any) {
  const data = {
    isAsync: pram.isAsync,
  };
  return request({
    url: '/admin/wechat/menu/public/get',
    method: 'post',
    params: data,
  });
}

export function wechatTemplateListApi(params: any) {
  return request({
    url: '/admin/wechat/template/list',
    method: 'get',
    params,
  });
}

export function wechatTemplateSaveApi(data: any) {
  return request({
    url: '/admin/wechat/template/save',
    method: 'post',
    data,
  });
}

export function wechatTemplateUpdateApi(id: number | string, data: any) {
  return request({
    url: `/admin/wechat/template/update/${id}`,
    method: 'post',
    data,
  });
}

export function wechatTemplateInfoApi(id: number | string) {
  return request({
    url: `/admin/wechat/template/info/${id}`,
    method: 'get',
  });
}

export function wechatTemplateStatusApi(id: number | string, params: any) {
  return request({
    url: `/admin/wechat/template/update/status/${id}`,
    method: 'post',
    params,
  });
}

export function wechatTemplateDeleteApi(id: number | string) {
  return request({
    url: `/admin/wechat/template/delete/${id}`,
    method: 'get',
  });
}

export function replyListApi(params: any) {
  return request({
    url: '/admin/wechat/keywords/reply/list',
    method: 'get',
    params,
  });
}

export function replySaveApi(data: any) {
  return request({
    url: '/admin/wechat/keywords/reply/save',
    method: 'post',
    data,
  });
}

export function replyStatusApi(params: any) {
  return request({
    url: '/admin/wechat/keywords/reply/status',
    method: 'post',
    params,
  });
}

export function replyUpdateApi(params: any, data: any) {
  return request({
    url: '/admin/wechat/keywords/reply/update',
    method: 'post',
    params,
    data,
  });
}

export function replyInfoApi(params: any) {
  return request({
    url: '/admin/wechat/keywords/reply/info',
    method: 'get',
    params,
  });
}

export function replyDeleteApi(params: any) {
  return request({
    url: '/admin/wechat/keywords/reply/delete',
    method: 'get',
    params,
  });
}

export function keywordsInfoApi(params: any) {
  return request({
    url: '/admin/wechat/keywords/reply/info/keywords',
    method: 'get',
    params,
  });
}

export function wechatMenuApi(params: any) {
  return request({
    url: '/admin/wechat/menu/public/get',
    method: 'get',
    params,
  });
}

export function wechatMenuAddApi(data: any) {
  return request({
    url: '/admin/wechat/menu/public/create',
    method: 'post',
    data,
  });
}

export function publicTempListApi(params: any) {
  return request({
    url: '/admin/wechat/program/public/temp/list',
    method: 'get',
    params,
  });
}

export function categoryApi() {
  return request({
    url: '/admin/wechat/program/category',
    method: 'get',
  });
}

export function getWeChatKeywordsByTidApi(params: any) {
  return request({
    url: '/admin/wechat/program/getWeChatKeywordsByTid',
    method: 'get',
    params,
  });
}

export function publicTempInfoApi(params: any) {
  return request({
    url: '/admin/wechat/program/public/temp/info',
    method: 'get',
    params,
  });
}

export function myTempListApi(params: any) {
  return request({
    url: '/admin/wechat/program/my/temp/list',
    method: 'get',
    params,
  });
}

export function myTempInfoApi(params: any) {
  return request({
    url: '/admin/wechat/program/my/temp/info',
    method: 'get',
    params,
  });
}

export function myTempSaveApi(data: any) {
  return request({
    url: '/admin/wechat/program/my/temp/save',
    method: 'post',
    data,
  });
}

export function myTempUpdateApi(params: any, data: any) {
  return request({
    url: '/admin/wechat/program/my/temp/update',
    method: 'post',
    params,
    data,
  });
}

export function myTempStatusApi(params: any) {
  return request({
    url: '/admin/wechat/program/my/temp/update/status',
    method: 'get',
    params,
  });
}

export function myTempTypeApi(params: any) {
  return request({
    url: '/admin/wechat/program/my/temp/update/type',
    method: 'get',
    params,
  });
}

export function getWechatConfig() {
  return request({
    url: '/admin/wechat/config',
    method: 'get',
    params: { url: encodeURIComponent(location.href.split('#')[0]) },
  });
}

export function wechatAuth(code: string) {
  return request({
    url: '/admin/authorize/login',
    method: 'get',
    params: { code },
  });
}

export function unbindApi() {
  return request({
    url: '/admin/unbind',
    method: 'get',
  });
}

export function tempAsyncApi() {
  return request({
    url: '/admin/wechat/program/my/temp/async',
    method: 'get',
  });
}

export function wechatAsyncApi() {
  return request({
    url: '/admin/wechat/template/whcbqhn/sync',
    method: 'post',
  });
}

export function routineAsyncApi() {
  return request({
    url: '/admin/wechat/template/routine/sync',
    method: 'post',
  });
}

export function wechatCodeDownload() {
  return request({
    url: '/admin/wechat/code/download',
    method: 'get',
  });
}

export function wechatGetShippingSwitchApi() {
  return request({
    url: '/admin/wechat/menu/get/shipping/switch',
    method: 'get',
  });
}

export function wechatUpdateShippingSwitchApi(data: any) {
  return request({
    url: '/admin/wechat/menu/update/shipping/switch',
    method: 'post',
    data,
  });
}
