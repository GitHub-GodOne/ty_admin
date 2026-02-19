import dayjs from 'dayjs';

export function parseTime(time: number | string | Date, pattern = 'YYYY-MM-DD HH:mm:ss'): string {
  if (!time) return '';
  return dayjs(typeof time === 'number' && time < 1e12 ? time * 1000 : time).format(pattern);
}

export interface MenuItem {
  id: number;
  pid: number;
  name: string;
  url: string;
  icon?: string;
  childList?: MenuItem[];
  [key: string]: unknown;
}

export function handleTree<T extends { id: number; pid: number; children?: T[] }>(
  data: T[],
  idField = 'id',
  parentField = 'pid',
): T[] {
  const map = new Map<number, T>();
  const roots: T[] = [];
  for (const item of data) {
    map.set((item as Record<string, unknown>)[idField] as number, { ...item, children: [] });
  }
  for (const item of map.values()) {
    const parentId = (item as Record<string, unknown>)[parentField] as number;
    if (parentId === 0 || !map.has(parentId)) {
      roots.push(item);
    } else {
      map.get(parentId)!.children!.push(item);
    }
  }
  return roots;
}

export interface RouteMenu {
  id: number;
  pid: number;
  name: string;
  url: string;
  icon?: string;
  childList?: RouteMenu[];
  path?: string;
  title?: string;
  children?: RouteMenu[];
}

export function formatRoutes(menus: RouteMenu[]): RouteMenu[] {
  return menus.map((item) => ({
    ...item,
    path: item.url,
    title: item.name,
    children: item.childList ? formatRoutes(item.childList) : undefined,
    childList: undefined,
  }));
}

export function flattenRoutes(routes: RouteMenu[]): RouteMenu[] {
  const result: RouteMenu[] = [];
  for (const route of routes) {
    result.push(route);
    if (route.children?.length) {
      result.push(...flattenRoutes(route.children));
    }
  }
  return result;
}
