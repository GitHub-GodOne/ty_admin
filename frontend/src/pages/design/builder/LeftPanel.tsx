import React from 'react';
import { Collapse, Typography } from 'antd';
import {
  PictureOutlined, FontSizeOutlined, AppstoreOutlined, SearchOutlined, MenuOutlined,
  LayoutOutlined, BlockOutlined, BorderOutlined, LineOutlined, FileTextOutlined,
  ShoppingOutlined, ScissorOutlined, TagOutlined, ThunderboltOutlined, TeamOutlined,
  ReadOutlined, PlayCircleOutlined, SwitcherOutlined, NotificationOutlined, AimOutlined,
  PicCenterOutlined, ShopOutlined,
} from '@ant-design/icons';
import { componentList, componentCategories, createComponentInstance, getComponentDef } from './components';
import type { ComponentInstance } from './components';

const iconMap: Record<string, React.ReactNode> = {
  PictureOutlined: <PictureOutlined />,
  FontSizeOutlined: <FontSizeOutlined />,
  AppstoreOutlined: <AppstoreOutlined />,
  SearchOutlined: <SearchOutlined />,
  MenuOutlined: <MenuOutlined />,
  LayoutOutlined: <LayoutOutlined />,
  BlockOutlined: <BlockOutlined />,
  BorderOutlined: <BorderOutlined />,
  LineOutlined: <LineOutlined />,
  FileTextOutlined: <FileTextOutlined />,
  ShoppingOutlined: <ShoppingOutlined />,
  ScissorOutlined: <ScissorOutlined />,
  TagOutlined: <TagOutlined />,
  ThunderboltOutlined: <ThunderboltOutlined />,
  TeamOutlined: <TeamOutlined />,
  ReadOutlined: <ReadOutlined />,
  PlayCircleOutlined: <PlayCircleOutlined />,
  SwitcherOutlined: <SwitcherOutlined />,
  NotificationOutlined: <NotificationOutlined />,
  AimOutlined: <AimOutlined />,
  PicCenterOutlined: <PicCenterOutlined />,
  ShopOutlined: <ShopOutlined />,
};

interface LeftPanelProps {
  components: ComponentInstance[];
  onAdd: (comp: ComponentInstance) => void;
}

const LeftPanel: React.FC<LeftPanelProps> = ({ components, onAdd }) => {
  const handleAdd = (name: string) => {
    const def = getComponentDef(name);
    if (!def) return;
    // Check singleton
    if (def.singleton && components.some((c) => c.name === name)) {
      return;
    }
    // Check conflicts
    if (def.conflicts?.length) {
      const hasConflict = components.some((c) => def.conflicts!.includes(c.name));
      if (hasConflict) return;
    }
    onAdd(createComponentInstance(def));
  };

  const isDisabled = (name: string) => {
    const def = getComponentDef(name);
    if (!def) return false;
    if (def.singleton && components.some((c) => c.name === name)) return true;
    if (def.conflicts?.length && components.some((c) => def.conflicts!.includes(c.name))) return true;
    return false;
  };

  return (
    <div style={{ width: 220, height: '100%', overflow: 'auto', borderRight: '1px solid #f0f0f0', background: '#fff' }}>
      <div style={{ padding: '12px 16px', fontWeight: 600, borderBottom: '1px solid #f0f0f0' }}>组件库</div>
      <Collapse defaultActiveKey={componentCategories.map((c) => c.key)} ghost size="small">
        {componentCategories.map((cat) => (
          <Collapse.Panel header={cat.label} key={cat.key}>
            <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 8 }}>
              {componentList
                .filter((c) => c.category === cat.key)
                .map((comp) => {
                  const disabled = isDisabled(comp.name);
                  return (
                    <div
                      key={comp.name}
                      onClick={() => !disabled && handleAdd(comp.name)}
                      style={{
                        display: 'flex', flexDirection: 'column', alignItems: 'center', gap: 4,
                        padding: '8px 4px', borderRadius: 6, border: '1px solid #f0f0f0',
                        cursor: disabled ? 'not-allowed' : 'pointer',
                        opacity: disabled ? 0.4 : 1,
                        transition: 'all 0.2s',
                      }}
                      onMouseEnter={(e) => { if (!disabled) e.currentTarget.style.borderColor = '#1890ff'; }}
                      onMouseLeave={(e) => { e.currentTarget.style.borderColor = '#f0f0f0'; }}
                    >
                      <span style={{ fontSize: 20 }}>{iconMap[comp.icon] || <BlockOutlined />}</span>
                      <Typography.Text style={{ fontSize: 12 }} ellipsis>{comp.cname}</Typography.Text>
                    </div>
                  );
                })}
            </div>
          </Collapse.Panel>
        ))}
      </Collapse>
    </div>
  );
};

export default LeftPanel;
