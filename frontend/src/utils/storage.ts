const prefix = 'ty_admin_';

export const local = {
  set(key: string, value: string): void {
    localStorage.setItem(prefix + key, value);
  },
  get(key: string): string | null {
    return localStorage.getItem(prefix + key);
  },
  setJSON(key: string, value: unknown): void {
    localStorage.setItem(prefix + key, JSON.stringify(value));
  },
  getJSON<T = unknown>(key: string): T | null {
    const val = localStorage.getItem(prefix + key);
    if (!val) return null;
    try { return JSON.parse(val) as T; } catch { return null; }
  },
  remove(key: string): void {
    localStorage.removeItem(prefix + key);
  },
  has(key: string): boolean {
    return localStorage.getItem(prefix + key) !== null;
  },
};

export const session = {
  set(key: string, value: string): void {
    sessionStorage.setItem(prefix + key, value);
  },
  get(key: string): string | null {
    return sessionStorage.getItem(prefix + key);
  },
  setJSON(key: string, value: unknown): void {
    sessionStorage.setItem(prefix + key, JSON.stringify(value));
  },
  getJSON<T = unknown>(key: string): T | null {
    const val = sessionStorage.getItem(prefix + key);
    if (!val) return null;
    try { return JSON.parse(val) as T; } catch { return null; }
  },
  remove(key: string): void {
    sessionStorage.removeItem(prefix + key);
  },
};
