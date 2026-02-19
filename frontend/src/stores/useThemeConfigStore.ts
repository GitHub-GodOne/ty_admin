import { create } from 'zustand';

interface ThemeConfig {
  isDrawer: boolean;
  primary: string;
  menuBgColor: string;
  isIsDark: boolean;
  themeStyle: string;
  topBar: string;
  topBarColor: string;
  menuBar: string;
  menuBarColor: string;
  isCollapse: boolean;
  isUniqueOpened: boolean;
  isFixedHeader: boolean;
  isShowLogo: boolean;
  isBreadcrumb: boolean;
  isTagsview: boolean;
  isFooter: boolean;
  isGrayscale: boolean;
  isInvert: boolean;
  tagsStyle: string;
  animation: string;
  layout: string;
  globalTitle: string;
  globalViceTitle: string;
  globalViceDes: string;
}

interface ThemeConfigState {
  themeConfig: ThemeConfig;
  setThemeConfig: (config: Partial<ThemeConfig>) => void;
}

const defaultConfig: ThemeConfig = {
  isDrawer: false,
  primary: '#0256FF',
  menuBgColor: '#282c34',
  isIsDark: false,
  themeStyle: 'theme-2',
  topBar: '#ffffff',
  topBarColor: '#606266',
  menuBar: '#282c34',
  menuBarColor: '#eaeaea',
  isCollapse: false,
  isUniqueOpened: true,
  isFixedHeader: true,
  isShowLogo: true,
  isBreadcrumb: true,
  isTagsview: true,
  isFooter: true,
  isGrayscale: false,
  isInvert: false,
  tagsStyle: 'tags-style-five',
  animation: 'opacitys',
  layout: 'defaults',
  globalTitle: 'TyAdmin',
  globalViceTitle: '',
  globalViceDes: '',
};

const STORAGE_KEY = 'TyAdmin_ThemeConfig';

function loadConfig(): ThemeConfig {
  try {
    const saved = localStorage.getItem(STORAGE_KEY);
    if (saved) return { ...defaultConfig, ...JSON.parse(saved), isDrawer: false };
  } catch { /* noop */ }
  return defaultConfig;
}

function saveConfig(config: ThemeConfig) {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(config));
  } catch { /* noop */ }
}

export const useThemeConfigStore = create<ThemeConfigState>((set) => ({
  themeConfig: loadConfig(),
  setThemeConfig: (config) =>
    set((state) => {
      const newConfig = { ...state.themeConfig, ...config };
      saveConfig(newConfig);
      return { themeConfig: newConfig };
    }),
}));
