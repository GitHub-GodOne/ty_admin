import axios from 'axios';
import { message } from 'antd';
import { getToken } from '@/utils/auth';
import SettingMer from '@/utils/settingMer';

const service = axios.create({
  baseURL: SettingMer.apiBaseURL,
  timeout: 60000,
});

service.interceptors.request.use(
  (config) => {
    const token = getToken();
    if (token) {
      config.headers['Authori-zation'] = token;
    }
    if (/get/i.test(config.method || '')) {
      config.params = config.params || {};
      config.params.temp = Math.floor(Date.now() / 1000);
    }
    return config;
  },
  (error) => Promise.reject(error),
);

service.interceptors.response.use(
  (response) => {
    const res = response.data;
    if (res.code === 401) {
      message.error('无效的会话，或者登录已过期，请重新登录。');
      if (window.location.pathname !== '/login') {
        window.location.href = '/login';
      }
      return Promise.reject(res);
    }
    if (res.code === 403) {
      message.error('没有权限访问。');
      return Promise.reject(res);
    }
    if (res.code !== 200) {
      message.error(res.message || 'Error');
      return Promise.reject(res);
    }
    return res.data;
  },
  (error) => {
    if (error.response && error.response.status === 401) {
      message.error('无效的会话，或者登录已过期，请重新登录。');
      if (window.location.pathname !== '/login') {
        window.location.href = '/login';
      }
      return Promise.reject(error);
    }
    message.error(error.message || '网络错误');
    return Promise.reject(error);
  },
);

export default service;
