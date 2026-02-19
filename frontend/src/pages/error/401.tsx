import React from 'react';
import { Button, Result } from 'antd';
import { useNavigate } from 'react-router-dom';

const NotAuth: React.FC = () => {
  const navigate = useNavigate();
  return (
    <div style={{ minHeight: '100vh', display: 'flex', alignItems: 'center', justifyContent: 'center' }}>
      <Result
        status="403"
        title="401"
        subTitle="抱歉，您没有权限访问此页面"
        extra={<Button type="primary" onClick={() => navigate('/dashboard')}>返回首页</Button>}
      />
    </div>
  );
};

export default NotAuth;
