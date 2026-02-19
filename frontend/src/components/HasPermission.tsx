import React from 'react';
import { usePermission } from '@/hooks/usePermission';

interface HasPermissionProps {
  permi: string | string[];
  children: React.ReactNode;
}

const HasPermission: React.FC<HasPermissionProps> = ({ permi, children }) => {
  const { hasPermi } = usePermission();
  if (!hasPermi(permi)) return null;
  return <>{children}</>;
};

export default HasPermission;
