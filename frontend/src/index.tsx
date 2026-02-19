import React from 'react';
import ReactDOM from 'react-dom/client';
import { RouterProvider } from 'react-router-dom';
import { ConfigProvider } from 'antd';
import zhCN from 'antd/locale/zh_CN';
import { router } from '@/router';
import { useThemeConfigStore } from '@/stores/useThemeConfigStore';
import './index.css';

const App: React.FC = () => {
  const primary = useThemeConfigStore((s) => s.themeConfig.primary);
  return (
    <ConfigProvider locale={zhCN} theme={{ token: { colorPrimary: primary } }}>
      <RouterProvider router={router} />
    </ConfigProvider>
  );
};

const root = document.getElementById('root');
if (!root) throw new Error('No root element found');

ReactDOM.createRoot(root).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
