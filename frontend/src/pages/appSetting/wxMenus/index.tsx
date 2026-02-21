import React, { useState, useEffect, useCallback } from 'react';
import { Card, Row, Col, Button, Input, Select, Form, Space, Tooltip, message } from 'antd';
import { PlusOutlined, DeleteOutlined } from '@ant-design/icons';
import { wechatMenuApi, wechatMenuAddApi } from '@/api/wxApi';

interface MenuItem {
  name: string;
  type: string;
  url?: string;
  key?: string;
  appid?: string;
  pagepath?: string;
  sub_button: MenuItem[];
}

const WxMenus: React.FC = () => {
  const [list, setList] = useState<MenuItem[]>([]);
  const [checkedIndex, setCheckedIndex] = useState<number | null>(null);
  const [checkedSubIndex, setCheckedSubIndex] = useState<number | null>(null);
  const [saving, setSaving] = useState(false);
  const [form] = Form.useForm();

  const currentItem = checkedIndex !== null
    ? (checkedSubIndex !== null ? list[checkedIndex]?.sub_button?.[checkedSubIndex] : list[checkedIndex])
    : null;

  const fetchMenu = useCallback(async () => {
    try {
      const res: any = await wechatMenuApi({});
      if (res?.button) setList(res.button);
    } catch { /* */ }
  }, []);

  useEffect(() => { fetchMenu(); }, []);

  useEffect(() => {
    if (currentItem) {
      form.setFieldsValue({
        name: currentItem.name || '',
        type: currentItem.type || 'click',
        url: currentItem.url || '',
        key: currentItem.key || '',
        appid: currentItem.appid || '',
        pagepath: currentItem.pagepath || '',
      });
    } else {
      form.resetFields();
    }
  }, [checkedIndex, checkedSubIndex, currentItem]);
  const updateCurrent = (field: string, value: string) => {
    if (checkedIndex === null) return;
    const newList = [...list];
    const item = checkedSubIndex !== null
      ? newList[checkedIndex].sub_button[checkedSubIndex]
      : newList[checkedIndex];
    (item as any)[field] = value;
    setList(newList);
  };

  const selectItem = (idx: number, subIdx: number | null) => {
    setCheckedIndex(idx);
    setCheckedSubIndex(subIdx);
  };

  const addTopMenu = () => {
    if (list.length >= 3) { message.warning('最多添加3个一级菜单'); return; }
    setList([...list, { name: '', type: 'click', sub_button: [] }]);
    setCheckedIndex(list.length);
    setCheckedSubIndex(null);
  };

  const addSubMenu = (idx: number) => {
    const newList = [...list];
    if (newList[idx].sub_button.length >= 5) { message.warning('最多添加5个二级菜单'); return; }
    newList[idx].sub_button.push({ name: '', type: 'click', sub_button: [] });
    setList(newList);
    setCheckedIndex(idx);
    setCheckedSubIndex(newList[idx].sub_button.length - 1);
  };

  const deleteCurrent = () => {
    if (checkedIndex === null) return;
    const newList = [...list];
    if (checkedSubIndex !== null) {
      newList[checkedIndex].sub_button.splice(checkedSubIndex, 1);
    } else {
      newList.splice(checkedIndex, 1);
    }
    setList(newList);
    setCheckedIndex(null);
    setCheckedSubIndex(null);
  };
  const handleSave = async () => {
    setSaving(true);
    try {
      await wechatMenuAddApi({ button: list });
      message.success('保存并发布成功');
    } catch { message.error('保存失败'); }
    finally { setSaving(false); }
  };

  const menuType = currentItem?.type || 'click';

  return (
    <Card>
      <Row gutter={24}>
        <Col xs={24} sm={24} md={10} lg={8}>
          <div style={{ border: '1px solid #e8e8e8', borderRadius: 8, padding: 16, minHeight: 400, background: '#fafafa' }}>
            <div style={{ textAlign: 'center', fontWeight: 'bold', marginBottom: 16, fontSize: 16 }}>菜单预览</div>
            <div style={{ display: 'flex', justifyContent: 'center', gap: 4, borderTop: '1px solid #e8e8e8', paddingTop: 12 }}>
              {list.map((item, idx) => (
                <div key={idx} style={{ flex: 1, textAlign: 'center' }}>
                  <div style={{ marginBottom: 8 }}>
                    {item.sub_button.map((sub, sIdx) => (
                      <Tooltip key={sIdx} title={sub.name || '二级菜单'}>
                        <Button size="small" block type={checkedIndex === idx && checkedSubIndex === sIdx ? 'primary' : 'default'}
                          style={{ marginBottom: 4, overflow: 'hidden', textOverflow: 'ellipsis' }}
                          onClick={() => selectItem(idx, sIdx)}>
                          {sub.name || '二级菜单'}
                        </Button>
                      </Tooltip>
                    ))}
                    {item.sub_button.length < 5 && (
                      <Button size="small" block type="dashed" icon={<PlusOutlined />}
                        onClick={() => addSubMenu(idx)} style={{ marginBottom: 4 }} />
                    )}
                  </div>
                  <Tooltip title={item.name || '一级菜单'}>
                    <Button block type={checkedIndex === idx && checkedSubIndex === null ? 'primary' : 'default'}
                      onClick={() => selectItem(idx, null)}
                      style={{ overflow: 'hidden', textOverflow: 'ellipsis' }}>
                      {item.name || '一级菜单'}
                    </Button>
                  </Tooltip>
                </div>
              ))}
              {list.length < 3 && (
                <div style={{ flex: 1, textAlign: 'center' }}>
                  <Button block icon={<PlusOutlined />} onClick={addTopMenu} />
                </div>
              )}
            </div>
          </div>
        </Col>
        <Col xs={24} sm={24} md={14} lg={16}>
          {currentItem ? (
            <Form form={form} labelCol={{ span: 4 }} wrapperCol={{ span: 18 }}>
              <Form.Item label="菜单名称" name="name">
                <Input maxLength={checkedSubIndex !== null ? 40 : 8}
                  placeholder={checkedSubIndex !== null ? '请输入二级菜单名称(最多40字)' : '请输入一级菜单名称(最多8字)'}
                  onChange={(e) => updateCurrent('name', e.target.value)} />
              </Form.Item>
              <Form.Item label="菜单类型" name="type">
                <Select onChange={(v) => updateCurrent('type', v)}
                  options={[
                    { value: 'click', label: '发送消息' },
                    { value: 'view', label: '跳转网页' },
                    { value: 'miniprogram', label: '跳转小程序' },
                  ]} />
              </Form.Item>
              {menuType === 'view' && (
                <Form.Item label="网页地址" name="url">
                  <Input placeholder="请输入网页链接" onChange={(e) => updateCurrent('url', e.target.value)} />
                </Form.Item>
              )}
              {menuType === 'click' && (
                <Form.Item label="消息Key" name="key">
                  <Input placeholder="请输入消息Key值" onChange={(e) => updateCurrent('key', e.target.value)} />
                </Form.Item>
              )}
              {menuType === 'miniprogram' && (
                <>
                  <Form.Item label="小程序AppId" name="appid">
                    <Input placeholder="请输入小程序AppId" onChange={(e) => updateCurrent('appid', e.target.value)} />
                  </Form.Item>
                  <Form.Item label="小程序路径" name="pagepath">
                    <Input placeholder="请输入小程序页面路径" onChange={(e) => updateCurrent('pagepath', e.target.value)} />
                  </Form.Item>
                  <Form.Item label="备用网页" name="url">
                    <Input placeholder="不支持小程序的老版本客户端将打开此网页" onChange={(e) => updateCurrent('url', e.target.value)} />
                  </Form.Item>
                </>
              )}
              <Form.Item wrapperCol={{ offset: 4 }}>
                <Button danger icon={<DeleteOutlined />} onClick={deleteCurrent}>删除菜单</Button>
              </Form.Item>
            </Form>
          ) : (
            <div style={{ textAlign: 'center', color: '#999', paddingTop: 80 }}>请选择左侧菜单进行编辑</div>
          )}
        </Col>
      </Row>
      <div style={{ textAlign: 'center', marginTop: 24 }}>
        <Button type="primary" loading={saving} onClick={handleSave}>保存并发布</Button>
      </div>
    </Card>
  );
};

export default WxMenus;
