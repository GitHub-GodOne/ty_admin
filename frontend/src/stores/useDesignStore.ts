import { create } from 'zustand';
import type { ComponentInstance } from '@/pages/design/builder/components';

interface PageConfig {
  id?: number;
  name: string;
  title: string;
  bgColor: string;
  bgPic: string;
  bgTabVal: number; // 0=color, 1=repeat, 2=stretch
  isBgColor: boolean;
  isBgPic: boolean;
  returnAddress: string;
  titleColor: string;
  titleBgColor: string;
  isDefault: boolean;
}

interface DesignState {
  pageConfig: PageConfig;
  components: ComponentInstance[];
  activeIndex: number;

  setPageConfig: (config: Partial<PageConfig>) => void;
  setComponents: (components: ComponentInstance[]) => void;
  setActiveIndex: (index: number) => void;
  addComponent: (comp: ComponentInstance, index?: number) => void;
  removeComponent: (index: number) => void;
  moveComponent: (fromIndex: number, toIndex: number) => void;
  updateComponentConfig: (index: number, config: Record<string, any>) => void;
  toggleComponentVisibility: (index: number) => void;
  duplicateComponent: (index: number) => void;
  resetAll: () => void;
  loadFromData: (data: any) => void;
  toSaveData: () => any;
}

const defaultPageConfig: PageConfig = {
  name: '',
  title: '',
  bgColor: '#f5f5f5',
  bgPic: '',
  bgTabVal: 0,
  isBgColor: true,
  isBgPic: false,
  returnAddress: '',
  titleColor: '#000000',
  titleBgColor: '#ffffff',
  isDefault: false,
};

export const useDesignStore = create<DesignState>((set, get) => ({
  pageConfig: { ...defaultPageConfig },
  components: [],
  activeIndex: -1,

  setPageConfig: (config) =>
    set((s) => ({ pageConfig: { ...s.pageConfig, ...config } })),

  setComponents: (components) => set({ components }),

  setActiveIndex: (activeIndex) => set({ activeIndex }),

  addComponent: (comp, index) =>
    set((s) => {
      const list = [...s.components];
      if (index !== undefined) {
        list.splice(index, 0, comp);
      } else {
        // Insert before footer if exists
        const footerIdx = list.findIndex((c) => c.fixed === 'bottom');
        if (footerIdx >= 0) {
          list.splice(footerIdx, 0, comp);
        } else {
          list.push(comp);
        }
      }
      return { components: list, activeIndex: index ?? list.length - 1 };
    }),

  removeComponent: (index) =>
    set((s) => {
      const list = s.components.filter((_, i) => i !== index);
      return {
        components: list,
        activeIndex: s.activeIndex >= list.length ? list.length - 1 : s.activeIndex === index ? -1 : s.activeIndex,
      };
    }),

  moveComponent: (fromIndex, toIndex) =>
    set((s) => {
      const list = [...s.components];
      const [item] = list.splice(fromIndex, 1);
      list.splice(toIndex, 0, item);
      return { components: list, activeIndex: toIndex };
    }),

  updateComponentConfig: (index, config) =>
    set((s) => {
      const list = [...s.components];
      if (list[index]) {
        list[index] = {
          ...list[index],
          defaultConfig: { ...list[index].defaultConfig, ...config },
        };
      }
      return { components: list };
    }),

  toggleComponentVisibility: (index) =>
    set((s) => {
      const list = [...s.components];
      if (list[index]) {
        list[index] = { ...list[index], isHide: !list[index].isHide };
      }
      return { components: list };
    }),

  duplicateComponent: (index) =>
    set((s) => {
      const list = [...s.components];
      const src = list[index];
      if (!src || src.singleton) return s;
      const timestamp = Date.now();
      const copy = {
        ...JSON.parse(JSON.stringify(src)),
        timestamp,
        id: `id${timestamp}`,
      };
      list.splice(index + 1, 0, copy);
      return { components: list, activeIndex: index + 1 };
    }),

  resetAll: () =>
    set({ pageConfig: { ...defaultPageConfig }, components: [], activeIndex: -1 }),

  loadFromData: (data) => {
    if (!data) return;
    const pageConfig: PageConfig = {
      id: data.id,
      name: data.name || '',
      title: data.title || '',
      bgColor: data.colorPicker || '#f5f5f5',
      bgPic: data.bgPic || '',
      bgTabVal: data.bgTabVal || 0,
      isBgColor: !!data.isBgColor,
      isBgPic: !!data.isBgPic,
      returnAddress: data.returnAddress || '',
      titleColor: data.titleColor || '#000000',
      titleBgColor: data.titleBgColor || '#ffffff',
      isDefault: !!data.isDefault,
    };
    // Parse value (component array)
    let components: ComponentInstance[] = [];
    const value = typeof data.value === 'string' ? JSON.parse(data.value || '{}') : (data.value || {});
    if (typeof value === 'object' && !Array.isArray(value)) {
      components = Object.values(value).map((v: any) => ({
        name: v.name,
        cname: v.cname,
        timestamp: v.timestamp || v.num || Date.now(),
        id: v.id || `id${v.timestamp}`,
        isHide: !!v.isHide,
        fixed: v.fixed,
        singleton: v.singleton,
        conflicts: v.conflicts,
        defaultConfig: v.defaultConfig || {},
      }));
    } else if (Array.isArray(value)) {
      components = value.map((v: any) => ({
        name: v.name,
        cname: v.cname,
        timestamp: v.timestamp || Date.now(),
        id: v.id || `id${v.timestamp}`,
        isHide: !!v.isHide,
        fixed: v.fixed,
        singleton: v.singleton,
        conflicts: v.conflicts,
        defaultConfig: v.defaultConfig || {},
      }));
    }
    set({ pageConfig, components, activeIndex: -1 });
  },

  toSaveData: () => {
    const { pageConfig, components } = get();
    const value: Record<string, any> = {};
    components.forEach((c) => {
      value[String(c.timestamp)] = {
        name: c.name,
        cname: c.cname,
        timestamp: c.timestamp,
        id: c.id,
        num: c.timestamp,
        isHide: c.isHide,
        defaultConfig: c.defaultConfig,
      };
    });
    return {
      id: pageConfig.id,
      name: pageConfig.name,
      title: pageConfig.title,
      colorPicker: pageConfig.bgColor,
      bgPic: pageConfig.bgPic,
      bgTabVal: pageConfig.bgTabVal,
      isBgColor: pageConfig.isBgColor ? 1 : 0,
      isBgPic: pageConfig.isBgPic ? 1 : 0,
      returnAddress: pageConfig.returnAddress,
      titleColor: pageConfig.titleColor,
      titleBgColor: pageConfig.titleBgColor,
      value: JSON.stringify(value),
    };
  },
}));
