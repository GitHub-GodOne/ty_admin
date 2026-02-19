import { create } from 'zustand';
import { local } from '@/utils/storage';

interface SettingsState {
  theme: string;
  showSettings: boolean;
  tagsView: boolean;
  fixedHeader: boolean;
  sidebarLogo: boolean;
  frontDomain: string;
  mediaDomain: string;
  changeSetting: (key: string, value: unknown) => void;
  setFrontDomain: (domain: string) => void;
  setMediaDomain: (domain: string) => void;
}

const storageSetting = local.getJSON<Record<string, unknown>>('layout-setting') || {};

export const useSettingsStore = create<SettingsState>((set) => ({
  theme: (storageSetting.theme as string) || '#1890ff',
  showSettings: true,
  tagsView: storageSetting.tagsView !== undefined ? (storageSetting.tagsView as boolean) : true,
  fixedHeader: storageSetting.fixedHeader !== undefined ? (storageSetting.fixedHeader as boolean) : true,
  sidebarLogo: storageSetting.sidebarLogo !== undefined ? (storageSetting.sidebarLogo as boolean) : true,
  frontDomain: localStorage.getItem('frontDomain') || '',
  mediaDomain: localStorage.getItem('mediaDomain') || '',

  changeSetting: (key, value) =>
    set((state) => ({ ...state, [key]: value })),

  setFrontDomain: (domain) => {
    localStorage.setItem('frontDomain', domain);
    set({ frontDomain: domain });
  },

  setMediaDomain: (domain) => {
    localStorage.setItem('mediaDomain', domain);
    set({ mediaDomain: domain });
  },
}));
