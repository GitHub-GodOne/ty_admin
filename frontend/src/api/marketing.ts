import request from '@/utils/request';

/**
 * 优惠券 列表
 */
export function marketingListApi(params: any) {
  return request({
    url: '/admin/marketing/coupon/list',
    method: 'get',
    params,
  });
}

// alias for CouponList component
export const couponListApi = marketingListApi;

/**
 * 用户管理 优惠券列表
 */
export function marketingSendApi(params: any) {
  return request({
    url: '/admin/marketing/coupon/send/list',
    method: 'get',
    params,
  });
}

/**
 * 优惠券 详情
 */
export function couponInfoApi(params: any) {
  return request({
    url: '/admin/marketing/coupon/info',
    method: 'post',
    params,
  });
}

/**
 * 优惠券 发送
 */
export function couponUserApi(params: any) {
  return request({
    url: '/admin/marketing/coupon/user/receive',
    method: 'post',
    params,
  });
}

/**
 * 优惠券 保存
 */
export function couponSaveApi(data: any) {
  return request({
    url: '/admin/marketing/coupon/save',
    method: 'post',
    data,
  });
}

/**
 * 优惠券 修改状态
 */
export function couponIssueStatusApi(params: any) {
  return request({
    url: '/admin/marketing/coupon/update/status',
    method: 'post',
    params,
  });
}

/**
 * 优惠券 删除
 */
export function couponDeleteApi(params: any) {
  return request({
    url: '/admin/marketing/coupon/delete',
    method: 'post',
    params,
  });
}

/**
 * 会员领取记录 列表
 */
export function couponUserListApi(params: any) {
  return request({
    url: '/admin/marketing/coupon/user/list',
    method: 'get',
    params,
  });
}

/**
 * 积分日志 列表
 */
export function integralListApi(params: any, data: any) {
  return request({
    url: '/admin/user/integral/list',
    method: 'post',
    params,
    data,
  });
}

/**
 * 秒杀配置 列表
 */
export function seckillListApi(params: any) {
  return request({
    url: '/admin/store/seckill/manger/list',
    method: 'get',
    params,
  });
}

/**
 * 秒杀配置 详情
 */
export function seckillInfoApi(params: any) {
  return request({
    url: '/admin/store/seckill/manger/info',
    method: 'get',
    params,
  });
}

/**
 * 秒杀配置 新增
 */
export function seckillSaveApi(data: any) {
  return request({
    url: '/admin/store/seckill/manger/save',
    method: 'post',
    data,
  });
}

/**
 * 秒杀配置 修改
 */
export function seckillUpdateApi(params: any, data: any) {
  return request({
    url: '/admin/store/seckill/manger/update',
    method: 'post',
    params,
    data,
  });
}

/**
 * 秒杀配置 删除
 */
export function seckillDeleteApi(params: any) {
  return request({
    url: '/admin/store/seckill/manger/delete',
    method: 'get',
    params,
  });
}

/**
 * 秒杀商品 列表
 */
export function seckillStoreListApi(params: any) {
  return request({
    url: '/admin/store/seckill/list',
    method: 'get',
    params,
  });
}

/**
 * 秒杀商品 详情
 */
export function seckillStoreInfoApi(params: any) {
  return request({
    url: '/admin/store/seckill/info',
    method: 'get',
    params,
  });
}

/**
 * 秒杀商品 新增
 */
export function seckillStoreSaveApi(data: any) {
  return request({
    url: '/admin/store/seckill/save',
    method: 'post',
    data,
  });
}

/**
 * 秒杀商品 修改
 */
export function seckillStoreUpdateApi(params: any, data: any) {
  return request({
    url: '/admin/store/seckill/update',
    method: 'post',
    params,
    data,
  });
}

/**
 * 秒杀商品 删除
 */
export function seckillStoreDeleteApi(params: any) {
  return request({
    url: '/admin/store/seckill/delete',
    method: 'get',
    params,
  });
}

/**
 * 秒杀商品 修改状态
 */
export function seckillStoreStatusApi(params: any) {
  return request({
    url: '/admin/store/seckill/update/status',
    method: 'post',
    params,
  });
}

/**
 * 秒杀配置 修改状态
 */
export function seckillConfigStatusApi(id: any, params: any) {
  return request({
    url: '/admin/store/seckill/manger/update/status',
    method: 'post',
    params: { id, ...params },
  });
}

/**
 * 砍价商品 列表
 */
export function bargainListApi(params: any) {
  return request({
    url: '/admin/store/bargain/list',
    method: 'get',
    params,
  });
}

/**
 * 砍价商品 新增
 */
export function bargainSaveApi(data: any) {
  return request({
    url: '/admin/store/bargain/save',
    method: 'POST',
    data,
  });
}

/**
 * 砍价商品 详情
 */
export function bargainInfoApi(params: any) {
  return request({
    url: '/admin/store/bargain/info',
    method: 'get',
    params,
  });
}

/**
 * 砍价商品 编辑
 */
export function bargainUpdateApi(params: any, data: any) {
  return request({
    url: '/admin/store/bargain/update',
    method: 'post',
    params,
    data,
  });
}

/**
 * 砍价商品 删除
 */
export function bargainDeleteApi(params: any) {
  return request({
    url: '/admin/store/bargain/delete',
    method: 'get',
    params,
  });
}

/**
 * 砍价列表 详情
 */
export function bargainOrderPinkApi(id: any) {
  return request({
    url: '/admin/store/bargain/bargain_list_info',
    method: 'get',
    params: { id, page: 1, limit: 100 },
  });
}

/**
 * 砍价列表 列表
 */
export function bargainListListApi(params: any) {
  return request({
    url: '/admin/store/bargain/bargain_list',
    method: 'get',
    params,
  });
}

/**
 * 砍价商品 修改状态
 */
export function bargainStatusApi(data: any) {
  return request({
    url: '/admin/store/bargain/update/status',
    method: 'post',
    data,
  });
}

/**
 * 拼团商品 列表
 */
export function combinationListApi(params: any) {
  return request({
    url: '/admin/store/combination/list',
    method: 'get',
    params,
  });
}

/**
 * 拼团商品 删除
 */
export function combinationDeleteApi(params: any) {
  return request({
    url: '/admin/store/combination/delete',
    method: 'get',
    params,
  });
}

/**
 * 拼团商品 新增
 */
export function combinationSaveApi(data: any) {
  return request({
    url: '/admin/store/combination/save',
    method: 'post',
    data,
  });
}

/**
 * 拼团商品 修改
 */
export function combinationUpdateApi(params: any, data: any) {
  return request({
    url: '/admin/store/combination/update',
    method: 'post',
    params,
    data,
  });
}

/**
 * 拼团商品 详情
 */
export function combinationInfoApi(params: any) {
  return request({
    url: '/admin/store/combination/info',
    method: 'get',
    params,
  });
}

/**
 * 拼团商品 修改拼团状态
 */
export function combinationStatusApi(data: any) {
  return request({
    url: '/admin/store/combination/update/status',
    method: 'post',
    data,
  });
}

/**
 * 拼团列表 列表
 */
export function combineListApi(params: any) {
  return request({
    url: '/admin/store/combination/combine/list',
    method: 'get',
    params,
  });
}

/**
 * 拼团列表 统计
 */
export function combineStatisticsApi(params: any) {
  return request({
    url: '/admin/store/combination/statistics',
    method: 'get',
    params,
  });
}

/**
 * 拼团列表 详情
 */
export function combineOrderPinkApi(id: any) {
  return request({
    url: `/admin/store/combination/order_pink/${id}`,
    method: 'get',
  });
}

/**
 * 砍价 导出
 */
export function exportBargainApi(params: any) {
  return request({
    url: '/admin/export/excel/bargain/product',
    method: 'get',
    params,
  });
}

/**
 * 拼团 导出
 */
export function exportcombiantionApi(params: any) {
  return request({
    url: '/admin/export/excel/combiantion/product',
    method: 'get',
    params,
  });
}

/**
 * 活动样式 列表
 */
export function atuosphereList(params: any) {
  return request({
    url: '/admin/activitystyle/list',
    method: 'get',
    params,
  });
}

/**
 * 活动样式 状态
 */
export function atmosphereStatusApi(data: any) {
  return request({
    url: '/admin/activitystyle/status',
    method: 'post',
    data,
  });
}

/**
 * 活动样式 删除
 */
export function atmosphereDelete(params: any) {
  return request({
    url: '/admin/activitystyle/delete',
    method: 'get',
    params,
  });
}

/**
 * 氛围图 -- 选择商品列表
 */
export function selectProductList(data: any) {
  return request.get('marketing/spu/lst', data);
}

/**
 * 氛围图 -- 创建氛围图
 */
export function createAtuosphere(data: any) {
  return request.post('admin/activitystyle/save', data);
}

/**
 * 氛围图 -- 编辑氛围图
 */
export function atuosphereUpdateApi(data: any) {
  return request.post('admin/activitystyle/update', data);
}
