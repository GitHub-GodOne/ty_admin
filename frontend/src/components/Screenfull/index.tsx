import React, { useState, useCallback } from 'react';
import { useFullscreen } from '@/hooks/useFullscreen';
import { FullscreenOutlined, FullscreenExitOutlined } from '@ant-design/icons';
import { Tooltip } from 'antd';

const Screenfull: React.FC = () => {
  const [isFullscreen, setIsFullscreen] = useState(false);

  const toggle = useCallback(() => {
    if (!document.fullscreenElement) {
      document.documentElement.requestFullscreen().then(() => setIsFullscreen(true));
    } else {
      document.exitFullscreen().then(() => setIsFullscreen(false));
    }
  }, []);

  return (
    <Tooltip title={isFullscreen ? '退出全屏' : '全屏'}>
      <span onClick={toggle} style={{ cursor: 'pointer', fontSize: 18 }}>
        {isFullscreen ? <FullscreenExitOutlined /> : <FullscreenOutlined />}
      </span>
    </Tooltip>
  );
};

export default Screenfull;
