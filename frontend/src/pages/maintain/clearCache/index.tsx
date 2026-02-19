import React, { useState } from 'react';
import { Card, Button, message } from 'antd';
import { ClearOutlined } from '@ant-design/icons';
import { clearCacheApi } from '@/api/systemConfig';

const ClearCache: React.FC = () => {
  const [loading, setLoading] = useState(false);

  const handleClear = async () => {
    setLoading(true);
    try {
      await clearCacheApi();
      message.success('清除成功');
    } catch {
      message.error('清除缓存失败');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
      <Card>
        <Button type="primary" icon={<ClearOutlined />} onClick={handleClear} loading={loading}>
          清除缓存
        </Button>
      </Card>
    </div>
  );
};

export default ClearCache;
