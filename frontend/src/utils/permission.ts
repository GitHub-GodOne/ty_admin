import { useAuthStore } from '@/stores/useAuthStore';

export function checkPermi(value: string[]): boolean {
  if (!value || value.length === 0) return false;
  const permissions = useAuthStore.getState().permissions;
  const allPermission = '*:*:*';
  return permissions.some(
    (p) => allPermission === p || value.includes(p),
  );
}

export function checkRole(value: string[]): boolean {
  if (!value || value.length === 0) return false;
  const roles = useAuthStore.getState().roles;
  const superAdmin = 'admin';
  return roles.some(
    (r) => superAdmin === r || value.includes(r),
  );
}
