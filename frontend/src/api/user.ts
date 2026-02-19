import request from '@/utils/request';

export function login(data: any) {
  return request({
    url: '/admin/login',
    method: 'post',
    data,
  });
}

export function getInfo(token: string) {
  return request({
    url: '/admin/getAdminInfoByToken',
    method: 'get',
    params: { token },
  });
}

export function logout() {
  return request({
    url: '/admin/logout',
    method: 'get',
  });
}

export function userListApi(params: any) {
  return request({
    url: '/admin/user/list',
    method: 'get',
    params,
  });
}

export function userUpdateApi(params: any, data: any) {
  return request({
    url: '/admin/user/update',
    method: 'post',
    params,
    data,
  });
}

export function userLevelUpdateApi(data: any) {
  return request({
    url: '/admin/user/update/level',
    method: 'post',
    data,
  });
}

export function userInfoApi(params: any) {
  return request({
    url: '/admin/user/info',
    method: 'get',
    params,
  });
}

export function infobyconditionApi(params: any) {
  return request({
    url: '/admin/user/infobycondition',
    method: 'get',
    params,
  });
}

export function topdetailApi(params: any) {
  return request({
    url: '/admin/user/topdetail',
    method: 'get',
    params,
  });
}

export function groupPiApi(params: any) {
  return request({
    url: '/admin/user/group',
    method: 'post',
    params,
  });
}

export function tagPiApi(params: any) {
  return request({
    url: '/admin/user/tag',
    method: 'post',
    params,
  });
}

export function foundsApi(params: any) {
  return request({
    url: '/admin/user/operate/founds',
    method: 'get',
    params,
  });
}

export function userDeleteApi(params: any) {
  return request({
    url: '/admin/user/delete',
    method: 'get',
    params,
  });
}

export function levelListApi() {
  return request({
    url: '/admin/system/user/level/list',
    method: 'get',
  });
}

export function levelSaveApi(data: any) {
  return request({
    url: '/admin/system/user/level/save',
    method: 'post',
    data,
  });
}

export function levelUpdateApi(id: number | string, data: any) {
  return request({
    url: '/admin/system/user/level/update',
    method: 'post',
    params: { id },
    data,
  });
}

export function levelInfoApi(params: any) {
  return request({
    url: '/admin/system/user/level/info',
    method: 'get',
    params,
  });
}

export function levelDeleteApi(id: number | string) {
  return request({
    url: '/admin/system/user/level/delete',
    method: 'post',
    params: { id },
  });
}

export function levelUseApi(data: any) {
  return request({
    url: '/admin/system/user/level/use',
    method: 'post',
    data,
  });
}

export function tagListApi(params: any) {
  return request({
    url: '/admin/user/tag/list',
    method: 'get',
    params,
  });
}

export function tagSaveApi(data: any) {
  return request({
    url: '/admin/user/tag/save',
    method: 'post',
    data,
  });
}

export function tagUpdateApi(params: any, data: any) {
  return request({
    url: '/admin/user/tag/update',
    method: 'post',
    params,
    data,
  });
}

export function tagInfoApi(params: any) {
  return request({
    url: '/admin/user/tag/info',
    method: 'get',
    params,
  });
}

export function tagDeleteApi(params: any) {
  return request({
    url: '/admin/user/tag/delete',
    method: 'get',
    params,
  });
}

export function groupListApi(params: any) {
  return request({
    url: '/admin/user/group/list',
    method: 'get',
    params,
  });
}

export function groupSaveApi(data: any) {
  return request({
    url: '/admin/user/group/save',
    method: 'post',
    data,
  });
}

export function groupUpdateApi(params: any, data: any) {
  return request({
    url: '/admin/user/group/update',
    method: 'post',
    params,
    data,
  });
}

export function groupInfoApi(params: any) {
  return request({
    url: '/admin/user/group/info',
    method: 'get',
    params,
  });
}

export function groupDeleteApi(params: any) {
  return request({
    url: '/admin/user/group/delete',
    method: 'get',
    params,
  });
}

export function getLoginPicApi() {
  return request({
    url: '/admin/getLoginPic',
    method: 'get',
  });
}

export function captchaApi() {
  return request({
    url: '/admin/validate/code/get',
    method: 'get',
  });
}

export function updateSpreadApi(data: any) {
  return request({
    url: '/admin/user/update/spread',
    method: 'post',
    data,
  });
}

export function updatePhoneApi(params: any) {
  return request({
    url: '/admin/user/update/phone',
    method: 'get',
    params,
  });
}

export function captchaconfigApi() {
  return request({
    url: '/admin/validate/code/getcaptchaconfig',
    method: 'get',
  });
}
