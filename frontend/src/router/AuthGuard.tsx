import React, { useEffect, useState } from 'react';
import { Navigate, useLocation } from 'react-router-dom';
import NProgress from 'nprogress';
import 'nprogress/nprogress.css';
import { getToken } from '@/utils/auth';
import { useAuthStore } from '@/stores/useAuthStore';
import getPageTitle from '@/utils/get-page-title';

NProgress.configure({ showSpinner: false });

const whiteList = ['/login', '/401', '/404'];

interface AuthGuardProps {
  children: React.ReactNode;
}

const AuthGuard: React.FC<AuthGuardProps> = ({ children }) => {
  const location = useLocation();
  const token = useAuthStore((s) => s.token);
  const roles = useAuthStore((s) => s.roles);
  const getInfoAction = useAuthStore((s) => s.getInfoAction);
  const getMenusAction = useAuthStore((s) => s.getMenusAction);
  const [loading, setLoading] = useState(false);
  const [ready, setReady] = useState(roles.length > 0);

  useEffect(() => {
    NProgress.start();
    document.title = getPageTitle();

    if (!token && !getToken()) {
      if (!whiteList.includes(location.pathname)) {
        NProgress.done();
        return;
      }
    }

    if (token && roles.length === 0 && !loading) {
      setLoading(true);
      Promise.all([getInfoAction(), getMenusAction()])
        .then(() => {
          setReady(true);
          setLoading(false);
        })
        .catch(() => {
          setLoading(false);
          // token 无效或过期，清除并跳转登录
          const { resetToken } = useAuthStore.getState();
          resetToken();
        });
    }

    NProgress.done();
  }, [location.pathname, token, roles.length, loading, getInfoAction, getMenusAction]);

  if (!token && !getToken() && !whiteList.includes(location.pathname)) {
    return <Navigate to="/login" replace />;
  }

  if (token && location.pathname === '/login') {
    return <Navigate to="/dashboard" replace />;
  }

  if (token && roles.length === 0 && !ready) {
    return null;
  }

  return <>{children}</>;
};

export default AuthGuard;
