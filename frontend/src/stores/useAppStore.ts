import { create } from 'zustand';
import Cookies from 'js-cookie';

interface AppState {
  sidebar: {
    opened: boolean;
    withoutAnimation: boolean;
  };
  device: 'desktop' | 'mobile';
  size: string;
  toggleSidebar: () => void;
  closeSidebar: (withoutAnimation: boolean) => void;
  toggleDevice: (device: 'desktop' | 'mobile') => void;
  setSize: (size: string) => void;
}

export const useAppStore = create<AppState>((set) => ({
  sidebar: {
    opened: Cookies.get('sidebarStatus') ? !!+Cookies.get('sidebarStatus')! : true,
    withoutAnimation: false,
  },
  device: 'desktop',
  size: Cookies.get('size') || 'small',

  toggleSidebar: () =>
    set((state) => {
      const opened = !state.sidebar.opened;
      Cookies.set('sidebarStatus', opened ? '1' : '0');
      return { sidebar: { opened, withoutAnimation: false } };
    }),

  closeSidebar: (withoutAnimation) => {
    Cookies.set('sidebarStatus', '0');
    set({ sidebar: { opened: false, withoutAnimation } });
  },

  toggleDevice: (device) => set({ device }),

  setSize: (size) => {
    Cookies.set('size', size);
    set({ size });
  },
}));
