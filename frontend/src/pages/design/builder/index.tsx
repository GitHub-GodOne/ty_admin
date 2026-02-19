import React, { useEffect, useState } from 'react';
import { Button, Space, Input, message } from 'antd';
import { ArrowLeftOutlined, SaveOutlined } from '@ant-design/icons';
import { useNavigate, useSearchParams } from 'react-router-dom';
import { useDesignStore } from '@/stores/useDesignStore';
import { pagediyInfoApi, pagediySaveApi, pagediyUpdateApi } from '@/api/pagediy';
import LeftPanel from './LeftPanel';
import PhonePreview from './PhonePreview';
import RightPanel from './RightPanel';

const PageBuilder: React.FC = () => {
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const editId = searchParams.get('id');
  const [saving, setSaving] = useState(false);

  const {
    pageConfig, components, activeIndex,
    setPageConfig, setActiveIndex, addComponent, removeComponent,
    moveComponent, updateComponentConfig, toggleComponentVisibility,
    duplicateComponent, resetAll, loadFromData, toSaveData,
  } = useDesignStore();

  useEffect(() => {
    resetAll();
    if (editId) {
      pagediyInfoApi(editId).then((res: any) => {
        if (res) loadFromData(res);
      }).catch(() => message.error('加载页面数据失败'));
    }
    return () => resetAll();
  }, [editId]);

  const handleSave = async () => {
    if (!pageConfig.name) {
      message.warning('请输入页面名称');
      return;
    }
    setSaving(true);
    try {
      const data = toSaveData();
      if (editId || pageConfig.id) {
        await pagediyUpdateApi({ ...data, id: editId || pageConfig.id });
      } else {
        const res = await pagediySaveApi(data);
        if (res?.id) setPageConfig({ id: res.id });
      }
      message.success('保存成功');
    } catch {
      message.error('保存失败');
    } finally {
      setSaving(false);
    }
  };

  // Ctrl+S to save
  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key === 's') {
        e.preventDefault();
        handleSave();
      }
    };
    window.addEventListener('keydown', handler);
    return () => window.removeEventListener('keydown', handler);
  }, [pageConfig, components]);

  return (
    <div style={{ display: 'flex', flexDirection: 'column', height: 'calc(100vh - 48px)', margin: '-24px', background: '#f0f2f5' }}>
      {/* Header */}
      <div style={{ height: 48, background: '#fff', borderBottom: '1px solid #f0f0f0', display: 'flex', alignItems: 'center', justifyContent: 'space-between', padding: '0 16px', flexShrink: 0 }}>
        <Space>
          <Button icon={<ArrowLeftOutlined />} onClick={() => navigate('/design/index')}>返回</Button>
          <Input
            value={pageConfig.name}
            onChange={(e) => setPageConfig({ name: e.target.value })}
            placeholder="页面名称"
            style={{ width: 200 }}
            maxLength={15}
          />
        </Space>
        <Space>
          <Button type="primary" icon={<SaveOutlined />} loading={saving} onClick={handleSave}>保存</Button>
          <Button onClick={() => { handleSave().then(() => navigate('/design/index')); }}>保存并关闭</Button>
        </Space>
      </div>
      {/* Body: 3-column layout */}
      <div style={{ flex: 1, display: 'flex', overflow: 'hidden' }}>
        <LeftPanel components={components} onAdd={addComponent} />
        <PhonePreview
          components={components}
          activeIndex={activeIndex}
          bgColor={pageConfig.bgColor}
          onSelect={setActiveIndex}
          onDelete={removeComponent}
          onToggle={toggleComponentVisibility}
          onDuplicate={duplicateComponent}
          onMove={moveComponent}
        />
        <RightPanel
          activeComponent={activeIndex >= 0 ? components[activeIndex] : null}
          pageConfig={pageConfig}
          onUpdateConfig={(config) => activeIndex >= 0 && updateComponentConfig(activeIndex, config)}
          onUpdatePage={setPageConfig}
        />
      </div>
    </div>
  );
};

export default PageBuilder;
