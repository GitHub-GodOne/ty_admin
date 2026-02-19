import { create } from 'zustand';
import { local } from '@/utils/storage';

interface TagView {
  path: string;
  query?: Record<string, string>;
  fullPath?: string;
  meta?: { title?: string; affix?: boolean; noCache?: boolean };
  title: string;
  name?: string;
}

interface TagsViewState {
  visitedViews: TagView[];
  cachedViews: string[];
  addView: (view: TagView) => void;
  addVisitedView: (view: TagView) => void;
  addCachedView: (view: TagView) => void;
  delView: (view: TagView) => { visitedViews: TagView[]; cachedViews: string[] };
  delVisitedView: (view: TagView) => void;
  delCachedView: (view: TagView) => void;
  delOthersViews: (view: TagView) => void;
  delAllViews: () => void;
  updateVisitedView: (view: TagView) => void;
}

export const useTagsViewStore = create<TagsViewState>((set, get) => ({
  visitedViews: local.getJSON<TagView[]>('visitedViews') || [],
  cachedViews: [],

  addView: (view) => {
    get().addVisitedView(view);
    get().addCachedView(view);
  },

  addVisitedView: (view) =>
    set((state) => {
      if (state.visitedViews.some((v) => v.path === view.path)) return state;
      const newViews = [...state.visitedViews, { ...view, title: view.meta?.title || 'no-name' }];
      local.setJSON('visitedViews', newViews);
      return { visitedViews: newViews };
    }),

  addCachedView: (view) =>
    set((state) => {
      if (!view.name || state.cachedViews.includes(view.name)) return state;
      if (view.meta?.noCache) return state;
      return { cachedViews: [...state.cachedViews, view.name] };
    }),

  delView: (view) => {
    get().delVisitedView(view);
    get().delCachedView(view);
    const state = get();
    return { visitedViews: state.visitedViews, cachedViews: state.cachedViews };
  },

  delVisitedView: (view) =>
    set((state) => {
      const newViews = state.visitedViews.filter((v) => v.path !== view.path);
      local.setJSON('visitedViews', newViews);
      return { visitedViews: newViews };
    }),

  delCachedView: (view) =>
    set((state) => ({
      cachedViews: state.cachedViews.filter((name) => name !== view.name),
    })),

  delOthersViews: (view) =>
    set((state) => {
      const newVisited = state.visitedViews.filter((v) => v.meta?.affix || v.path === view.path);
      local.setJSON('visitedViews', newVisited);
      const idx = state.cachedViews.indexOf(view.name || '');
      return {
        visitedViews: newVisited,
        cachedViews: idx > -1 ? state.cachedViews.slice(idx, idx + 1) : [],
      };
    }),

  delAllViews: () => {
    set((state) => {
      const affixTags = state.visitedViews.filter((tag) => tag.meta?.affix);
      local.setJSON('visitedViews', affixTags);
      return { visitedViews: affixTags, cachedViews: [] };
    });
  },

  updateVisitedView: (view) =>
    set((state) => ({
      visitedViews: state.visitedViews.map((v) => (v.path === view.path ? { ...v, ...view } : v)),
    })),
}));
