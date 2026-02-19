import React, { useRef, useEffect } from 'react';
import { useNavigate, useLocation } from 'react-router-dom';
import { Dropdown } from 'antd';
import { CloseOutlined, ReloadOutlined, CloseCircleOutlined, ColumnWidthOutlined } from '@ant-design/icons';
import { useTagsViewStore } from '@/stores/useTagsViewStore';

const TagsView: React.FC = () => {
  const navigate = useNavigate();
  const location = useLocation();
  const scrollRef = useRef<HTMLDivElement>(null);
  const visitedViews = useTagsViewStore((s) => s.visitedViews);
  const delView = useTagsViewStore((s) => s.delView);
  const delOthersViews = useTagsViewStore((s) => s.delOthersViews);
  const delAllViews = useTagsViewStore((s) => s.delAllViews);

  // 滚动到当前激活的 tag
  useEffect(() => {
    const container = scrollRef.current;
    if (!container) return;
    const activeTag = container.querySelector('.tags-view-item.active') as HTMLElement;
    if (activeTag) {
      const containerRect = container.getBoundingClientRect();
      const tagRect = activeTag.getBoundingClientRect();
      if (tagRect.left < containerRect.left) {
        container.scrollLeft -= containerRect.left - tagRect.left + 10;
      } else if (tagRect.right > containerRect.right) {
        container.scrollLeft += tagRect.right - containerRect.right + 10;
      }
    }
  }, [location.pathname, visitedViews]);

  // 鼠标滚轮横向滚动
  const handleWheel = (e: React.WheelEvent) => {
    if (scrollRef.current) {
      e.preventDefault();
      scrollRef.current.scrollLeft += e.deltaY;
    }
  };

  const isActive = (path: string) => path === location.pathname;
  const isAffix = (view: any) => view.meta?.affix || view.path === '/dashboard';

  const handleClose = (view: any, e?: React.MouseEvent) => {
    e?.stopPropagation();
    if (isAffix(view)) return;
    const { visitedViews: remaining } = delView(view);
    if (isActive(view.path)) {
      const latestView = remaining[remaining.length - 1];
      navigate(latestView ? latestView.path : '/dashboard');
    }
  };

  const handleCloseOthers = (view: any) => {
    if (!isActive(view.path)) navigate(view.path);
    delOthersViews(view);
  };

  const handleCloseAll = () => {
    delAllViews();
    navigate('/dashboard');
  };

  const getContextMenuItems = (view: any) => ({
    items: [
      { key: 'refresh', icon: <ReloadOutlined />, label: '刷新', onClick: () => navigate(view.path) },
      ...(!isAffix(view) ? [{ key: 'close', icon: <CloseOutlined />, label: '关闭', onClick: () => handleClose(view) }] : []),
      { key: 'closeOthers', icon: <ColumnWidthOutlined />, label: '关闭其他', onClick: () => handleCloseOthers(view) },
      { key: 'closeAll', icon: <CloseCircleOutlined />, label: '关闭全部', onClick: () => handleCloseAll() },
    ],
  });

  return (
    <div style={{
      height: 34, display: 'flex', alignItems: 'center', background: '#fff',
      borderBottom: '1px solid #f0f0f0', padding: '0 8px', boxShadow: '0 1px 3px rgba(0,0,0,0.05)',
    }}>
      <div ref={scrollRef} onWheel={handleWheel} style={{
        flex: 1, display: 'flex', alignItems: 'center', gap: 4, overflowX: 'auto', overflowY: 'hidden',
        scrollbarWidth: 'none', msOverflowStyle: 'none',
      }}>
        <style>{`.tags-scroll::-webkit-scrollbar { display: none; }`}</style>
        {visitedViews.map((view) => (
          <Dropdown key={view.path} menu={getContextMenuItems(view)} trigger={['contextMenu']}>
            <div
              className={`tags-view-item ${isActive(view.path) ? 'active' : ''}`}
              onClick={() => navigate(view.path)}
              style={{
                display: 'inline-flex', alignItems: 'center', gap: 4,
                padding: '0 8px', height: 26, lineHeight: '26px', cursor: 'pointer',
                borderRadius: 3, fontSize: 12, whiteSpace: 'nowrap', flexShrink: 0,
                background: isActive(view.path) ? '#0256FF' : '#f0f0f0',
                color: isActive(view.path) ? '#fff' : '#666',
                transition: 'all 0.2s',
              }}
            >
              <span>{view.title || view.path}</span>
              {!isAffix(view) && (
                <CloseOutlined
                  style={{ fontSize: 10, marginLeft: 2 }}
                  onClick={(e) => handleClose(view, e)}
                />
              )}
            </div>
          </Dropdown>
        ))}
      </div>
    </div>
  );
};

export default TagsView;
