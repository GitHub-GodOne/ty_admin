import request from '@/utils/request';

/**
 * 绑定产品
 */
export function bindProduct(pram: any) {
  const data = {
    id: pram.id,
    productId: pram.productId,
  };
  return request({
    url: '/admin/article/bind/product',
    method: 'POST',
    params: data,
  });
}

/**
 * 删除文章
 */
export function DelArticle(pram: any) {
  const data = {
    id: pram.id,
  };
  return request({
    url: '/admin/article/delete',
    method: 'GET',
    params: data,
  });
}

/**
 * 文章详情
 */
export function InfoArticle(id: any) {
  const data = {
    id: id,
  };
  return request({
    url: '/admin/article/info',
    method: 'get',
    params: data,
  });
}

/**
 * 文章列表
 */
export function ListArticle(pram: any) {
  const data = {
    keywords: pram.keywords,
    cid: pram.cid,
    page: pram.page,
    limit: pram.limit,
  };
  return request({
    url: '/admin/article/list',
    method: 'GET',
    params: data,
  });
}

/**
 * 新增文章
 */
export function AddArticle(pram: any) {
  const data = {
    author: pram.author,
    cid: pram.cid,
    content: pram.content,
    imageInput: pram.imageInput,
    isBanner: pram.isBanner,
    isHot: pram.isHot,
    shareSynopsis: pram.shareSynopsis,
    shareTitle: pram.shareTitle,
    sort: pram.sort,
    synopsis: pram.synopsis,
    title: pram.title,
    url: pram.url,
  };
  return request({
    url: '/admin/article/save',
    method: 'post',
    data: data,
  });
}

/**
 * 更新文章
 */
export function UpdateArticle(pram: any) {
  const data = {
    author: pram.author,
    cid: pram.cid,
    content: pram.content,
    imageInput: pram.imageInput,
    isBanner: pram.isBanner,
    isHot: pram.isHot,
    shareSynopsis: pram.shareSynopsis,
    shareTitle: pram.shareTitle,
    sort: pram.sort,
    synopsis: pram.synopsis,
    title: pram.title,
    url: pram.url,
  };
  return request({
    url: '/admin/article/update',
    method: 'post',
    params: { id: pram.id },
    data: data,
  });
}
