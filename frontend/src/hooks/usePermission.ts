import { useAuthStore } from '@/stores/useAuthStore';

export function usePermission() {
  const permissions = useAuthStore((s) => s.permissions);

  const hasPermi = (value: string | string[]): boolean => {
    const perms = Array.isArray(value) ? value : [value];
    if (perms.length === 0) return false;
    const allPermission = '*:*:*';
    return permissions.some(
      (p) => allPermission === p || perms.includes(p),
    );
  };

  return { hasPermi };
}
