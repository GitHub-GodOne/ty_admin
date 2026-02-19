import { create } from 'zustand';

interface PermissionState {
  routes: unknown[];
  sidebarRouters: unknown[];
  setRoutes: (routes: unknown[]) => void;
  setSidebarRouters: (routes: unknown[]) => void;
}

export const usePermissionStore = create<PermissionState>((set) => ({
  routes: [],
  sidebarRouters: [],
  setRoutes: (routes) => set({ routes }),
  setSidebarRouters: (routes) => set({ sidebarRouters: routes }),
}));
