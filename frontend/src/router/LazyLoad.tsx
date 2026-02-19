import React, { Suspense } from 'react';
import { Spin } from 'antd';

interface LazyLoadProps {
  children: React.ReactNode;
}

const LazyLoad: React.FC<LazyLoadProps> = ({ children }) => (
  <Suspense
    fallback={
      <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', height: '100%', minHeight: 200 }}>
        <Spin size="large" />
      </div>
    }
  >
    {children}
  </Suspense>
);

export default LazyLoad;
