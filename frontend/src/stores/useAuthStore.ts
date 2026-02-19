import { create } from 'zustand';
import { getToken, setToken, removeToken } from '@/utils/auth';
import { login, getInfo, logout } from '@/api/user';
import Cookies from 'js-cookie';

interface AuthState {
  token: string;
  name: string;
  avatar: string;
  roles: string[];
  permissions: string[];
  menuList: unknown[];
  oneLvRoutes: unknown[];

  setToken: (token: string) => void;
  loginAction: (data: { account: string; pwd: string }) => Promise<void>;
  getInfoAction: () => Promise<unknown>;
  getMenusAction: () => Promise<void>;
  logoutAction: () => Promise<void>;
  resetToken: () => void;
}

export const useAuthStore = create<AuthState>((set, get) => ({
  token: getToken() || '',
  name: '',
  avatar: '',
  roles: [],
  permissions: [],
  menuList: JSON.parse(localStorage.getItem('TyAdmin_MenuList') || '[]'),
  oneLvRoutes: JSON.parse(localStorage.getItem('TyAdmin_oneLvRoutes') || '[]'),

  setToken: (token: string) => {
    set({ token });
    setToken(token);
  },

  loginAction: async (data) => {
    const res: any = await login(data);
    set({ token: res.token });
    Cookies.set('JavaInfo', JSON.stringify(res));
    setToken(res.token);
  },

  getInfoAction: async () => {
    const token = get().token;
    const data: any = await getInfo(token);
    if (!data) throw new Error('Verification failed');
    const { roles, account, permissionsList } = data;
    set({
      roles,
      name: account,
      avatar: '',
      permissions: permissionsList || [],
    });
    return data;
  },

  getMenusAction: async () => {
    const { menuListApi } = await import('@/api/roleApi');
    let menus: any[] = await menuListApi();
    menus = replaceChildListWithChildren(menus);
    set({ menuList: menus });
    localStorage.setItem('TyAdmin_MenuList', JSON.stringify(menus));
    const routes = flattenMenus(menus);
    set({ oneLvRoutes: routes });
    localStorage.setItem('TyAdmin_oneLvRoutes', JSON.stringify(routes));
  },

  logoutAction: async () => {
    await logout();
    set({ token: '', roles: [], permissions: [], menuList: [] });
    removeToken();
    Cookies.remove('JavaInfo');
    sessionStorage.removeItem('token');
    localStorage.removeItem('TyAdmin_MenuList');
    localStorage.removeItem('TyAdmin_oneLvRoutes');
  },

  resetToken: () => {
    set({ token: '', roles: [] });
    removeToken();
  },
}));

function replaceChildListWithChildren(data: any[]): any[] {
  return data.map((item) => {
    const result: any = {
      ...item,
      title: item.name,
      path: item.component || item.url,
    };
    if (item.childList) {
      result.children = replaceChildListWithChildren(item.childList);
      delete result.childList;
    }
    return result;
  });
}

function flattenMenus(menus: any[]): any[] {
  const result: any[] = [];
  for (const menu of menus) {
    result.push(menu);
    if (menu.children?.length) {
      result.push(...flattenMenus(menu.children));
    }
  }
  return result;
}
