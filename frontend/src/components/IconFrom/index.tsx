import React, { useState } from 'react';
import { Modal, Input, Row, Col } from 'antd';
import * as icons from '@ant-design/icons';

// 收集所有 Outlined 风格图标
const iconEntries: { name: string; component: React.FC }[] = [];
Object.keys(icons).forEach((key) => {
  if (key.endsWith('Outlined') && typeof (icons as any)[key] === 'object') {
    iconEntries.push({ name: key, component: (icons as any)[key] });
  }
});

interface IconFromProps {
  open: boolean;
  onCancel: () => void;
  onOk: (icon: string) => void;
}

const IconFrom: React.FC<IconFromProps> = ({ open, onCancel, onOk }) => {
  const [search, setSearch] = useState('');
  const [selected, setSelected] = useState('');

  const filtered = search
    ? iconEntries.filter((i) => i.name.toLowerCase().includes(search.toLowerCase()))
    : iconEntries;

  return (
    <Modal title="选择图标" open={open} onCancel={onCancel}
      onOk={() => { if (selected) onOk(selected); }} width={660} destroyOnClose>
      <Input.Search placeholder="搜索图标" value={search}
        onChange={(e) => setSearch(e.target.value)} style={{ marginBottom: 16 }} allowClear />
      <div style={{ maxHeight: 400, overflow: 'auto' }}>
        <Row gutter={[8, 8]}>
          {filtered.map(({ name, component: Icon }) => (
            <Col key={name} span={3}>
              <div
                onClick={() => setSelected(name)}
                style={{
                  textAlign: 'center', padding: '8px 0', cursor: 'pointer', borderRadius: 4,
                  border: selected === name ? '2px solid #0256FF' : '1px solid #d9d9d9',
                  background: selected === name ? '#e6f0ff' : '#fff',
                }}
              >
                {React.createElement(Icon as any, { style: { fontSize: 22 } })}
                <div style={{ fontSize: 9, marginTop: 4, color: '#666', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap', padding: '0 2px' }}>
                  {name.replace('Outlined', '')}
                </div>
              </div>
            </Col>
          ))}
        </Row>
      </div>
    </Modal>
  );
};

export default IconFrom;
