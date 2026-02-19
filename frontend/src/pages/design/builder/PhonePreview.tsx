import React from 'react';
import { Button, Space, Tooltip } from 'antd';
import { DeleteOutlined, EyeOutlined, EyeInvisibleOutlined, CopyOutlined, ArrowUpOutlined, ArrowDownOutlined } from '@ant-design/icons';
import { DndContext, closestCenter, PointerSensor, useSensor, useSensors } from '@dnd-kit/core';
import { SortableContext, verticalListSortingStrategy, useSortable } from '@dnd-kit/sortable';
import { CSS } from '@dnd-kit/utilities';
import type { ComponentInstance } from './components';
import ComponentPreview from './PreviewComponents';

interface SortableItemProps {
  comp: ComponentInstance;
  index: number;
  isActive: boolean;
  onClick: () => void;
  onDelete: () => void;
  onToggle: () => void;
  onDuplicate: () => void;
  onMoveUp: () => void;
  onMoveDown: () => void;
  isFirst: boolean;
  isLast: boolean;
}

const SortableItem: React.FC<SortableItemProps> = ({ comp, index, isActive, onClick, onDelete, onToggle, onDuplicate, onMoveUp, onMoveDown, isFirst, isLast }) => {
  const { attributes, listeners, setNodeRef, transform, transition } = useSortable({
    id: comp.id,
    disabled: !!comp.fixed,
  });
  const style: React.CSSProperties = {
    transform: CSS.Transform.toString(transform),
    transition,
    position: 'relative',
    cursor: 'pointer',
    opacity: comp.isHide ? 0.4 : 1,
    outline: isActive ? '2px solid #1890ff' : '2px solid transparent',
    outlineOffset: -2,
  };

  return (
    <div ref={setNodeRef} style={style} {...attributes} {...listeners} onClick={onClick}>
      <ComponentPreview comp={comp} />
      {isActive && (
        <div style={{ position: 'absolute', top: -28, right: 0, zIndex: 10 }} onClick={(e) => e.stopPropagation()}>
          <Space size={2}>
            {!comp.fixed && !isFirst && (
              <Tooltip title="上移"><Button size="small" icon={<ArrowUpOutlined />} onClick={onMoveUp} /></Tooltip>
            )}
            {!comp.fixed && !isLast && (
              <Tooltip title="下移"><Button size="small" icon={<ArrowDownOutlined />} onClick={onMoveDown} /></Tooltip>
            )}
            <Tooltip title={comp.isHide ? '显示' : '隐藏'}>
              <Button size="small" icon={comp.isHide ? <EyeInvisibleOutlined /> : <EyeOutlined />} onClick={onToggle} />
            </Tooltip>
            {!comp.singleton && (
              <Tooltip title="复制"><Button size="small" icon={<CopyOutlined />} onClick={onDuplicate} /></Tooltip>
            )}
            <Tooltip title="删除"><Button size="small" danger icon={<DeleteOutlined />} onClick={onDelete} /></Tooltip>
          </Space>
        </div>
      )}
      {isActive && (
        <div style={{ position: 'absolute', top: 0, left: -60, background: '#1890ff', color: '#fff', padding: '2px 8px', borderRadius: '4px 0 0 4px', fontSize: 12, whiteSpace: 'nowrap' }}>
          {comp.cname}
        </div>
      )}
    </div>
  );
};

interface PhonePreviewProps {
  components: ComponentInstance[];
  activeIndex: number;
  bgColor: string;
  onSelect: (index: number) => void;
  onDelete: (index: number) => void;
  onToggle: (index: number) => void;
  onDuplicate: (index: number) => void;
  onMove: (fromIndex: number, toIndex: number) => void;
}

const PhonePreview: React.FC<PhonePreviewProps> = ({ components, activeIndex, bgColor, onSelect, onDelete, onToggle, onDuplicate, onMove }) => {
  const sensors = useSensors(useSensor(PointerSensor, { activationConstraint: { distance: 5 } }));

  const handleDragEnd = (event: any) => {
    const { active, over } = event;
    if (!over || active.id === over.id) return;
    const oldIndex = components.findIndex((c) => c.id === active.id);
    const newIndex = components.findIndex((c) => c.id === over.id);
    if (oldIndex !== -1 && newIndex !== -1) onMove(oldIndex, newIndex);
  };

  return (
    <div style={{ flex: 1, display: 'flex', justifyContent: 'center', alignItems: 'flex-start', padding: '20px 0', background: '#f0f2f5', overflow: 'auto' }}>
      <div style={{ width: 375, minHeight: 667, background: bgColor || '#f5f5f5', borderRadius: 30, boxShadow: '0 8px 40px rgba(0,0,0,0.12)', position: 'relative', overflow: 'hidden', border: '8px solid #222' }}>
        {/* Status bar */}
        <div style={{ height: 24, background: '#000', borderRadius: '22px 22px 0 0' }} />
        {/* Content */}
        <div style={{ minHeight: 619, overflow: 'auto', position: 'relative' }}>
          {components.length === 0 && (
            <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'center', height: 400, color: '#bbb', fontSize: 14 }}>
              点击左侧组件添加到页面
            </div>
          )}
          <DndContext sensors={sensors} collisionDetection={closestCenter} onDragEnd={handleDragEnd}>
            <SortableContext items={components.map((c) => c.id)} strategy={verticalListSortingStrategy}>
              {components.map((comp, idx) => (
                <SortableItem
                  key={comp.id}
                  comp={comp}
                  index={idx}
                  isActive={activeIndex === idx}
                  onClick={() => onSelect(idx)}
                  onDelete={() => onDelete(idx)}
                  onToggle={() => onToggle(idx)}
                  onDuplicate={() => onDuplicate(idx)}
                  onMoveUp={() => onMove(idx, idx - 1)}
                  onMoveDown={() => onMove(idx, idx + 1)}
                  isFirst={idx === 0}
                  isLast={idx === components.length - 1}
                />
              ))}
            </SortableContext>
          </DndContext>
        </div>
        {/* Bottom bar */}
        <div style={{ height: 24, background: '#000', borderRadius: '0 0 22px 22px', display: 'flex', justifyContent: 'center', alignItems: 'center' }}>
          <div style={{ width: 40, height: 4, background: '#555', borderRadius: 2 }} />
        </div>
      </div>
    </div>
  );
};

export default PhonePreview;
