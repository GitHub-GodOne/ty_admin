import request from '@/utils/request';

export function jobList() {
  return request({
    url: '/admin/schedule/job/list',
    method: 'get',
  });
}

export function jobLogList(data: any) {
  return request({
    url: '/admin/schedule/job/log/list',
    method: 'get',
    params: { ...data },
  });
}

export function scheduleJobAdd(pram: any) {
  const data = {
    jobId: pram.jobId,
    beanName: pram.beanName,
    cronExpression: pram.cronExpression,
    methodName: pram.methodName,
    params: pram.params,
    remark: pram.remark,
  };
  return request({
    url: '/admin/schedule/job/add',
    method: 'post',
    data: data,
  });
}

export function scheduleJobDelete(id: number | string) {
  return request({
    url: `/admin/schedule/job/delete/${id}`,
    method: 'post',
  });
}

export function scheduleJobStart(id: number | string) {
  return request({
    url: `/admin/schedule/job/start/${id}`,
    method: 'post',
  });
}

export function scheduleJobSuspend(id: number | string) {
  return request({
    url: `/admin/schedule/job/suspend/${id}`,
    method: 'post',
  });
}

export function scheduleJobTrig(id: number | string) {
  return request({
    url: `/admin/schedule/job/trig/${id}`,
    method: 'post',
  });
}

export function scheduleJobUpdate(pram: any) {
  const data = {
    jobId: pram.jobId,
    beanName: pram.beanName,
    cronExpression: pram.cronExpression,
    methodName: pram.methodName,
    params: pram.params,
    remark: pram.remark,
  };
  return request({
    url: '/admin/schedule/job/update',
    method: 'post',
    data: { ...data },
  });
}
