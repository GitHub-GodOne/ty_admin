import React, { useState, useEffect, useCallback } from 'react';
import { Card, Table, Form, Input, Button, Space, Tag, Modal, Select, Switch, message } from 'antd';
import { PlusOutlined, SearchOutlined } from '@ant-design/icons';
import * as allIcons from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { menuListApi, menuAdd, menuDelete, menuInfo, menuUpdate, menuUpdateShowStatus } from '@/api/systemAdmin';
import { useModal } from '@/hooks/useModal';
import IconFrom from '@/components/IconFrom';

const menuTypeMap: Record<string, { text: string; color: string }> = {
  M: { text: '目录', color: 'blue' },
  C: { text: '菜单', color: 'green' },
  A: { text: '按钮', color: 'orange' },
};

const OperationMenu: React.FC = () => {
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [name, setName] = useState('');
  const [menuType, setMenuType] = useState<string | undefined>(undefined);
  const [modalVisible, setModalVisible] = useState(false);
  const [editingId, setEditingId] = useState<number | null>(null);
  const [confirmLoading, setConfirmLoading] = useState(false);
  const [form] = Form.useForm();
  const { deleteConfirm } = useModal();
  const [iconModalVisible, setIconModalVisible] = useState(false);
  const [selectedIcon, setSelectedIcon] = useState('');

  const renderIcon = (name: string) => {
    const IconComp = (allIcons as any)[name];
    return IconComp ? React.createElement(IconComp, { style: { fontSize: 16 } }) : null;
  };

  const fetchList = useCallback(async () => {
    setLoading(true);
    try {
      const params: any = {};
      if (name) params.name = name;
      if (menuType) params.menuType = menuType;
      const res: any = await menuListApi(params);
      setList(res || []);
    } catch { /* handled */ }
    finally { setLoading(false); }
  }, [name, menuType]);

  useEffect(() => { fetchList(); }, [fetchList]);

  const handleAdd = (pid = 0) => {
    setEditingId(null);
    form.resetFields();
    form.setFieldsValue({ pid, menuType: 'C', sort: 0, isShow: true });
    setSelectedIcon('');
    setModalVisible(true);
  };

  const handleEdit = async (id: number) => {
    try {
      const res: any = await menuInfo(id);
      setEditingId(id);
      form.setFieldsValue({ ...res, isShow: res?.isShow === 1 || res?.isShow === true });
      setSelectedIcon(res?.icon || '');
      setModalVisible(true);
    } catch { message.error('获取菜单详情失败'); }
  };

  const handleDelete = (id: number) => {
    deleteConfirm(async () => {
      await menuDelete(id);
      message.success('删除成功');
      fetchList();
    });
  };

  const handleToggleShow = async (record: any) => {
    try {
      await menuUpdateShowStatus({ id: record.id });
      message.success('状态已更新');
      fetchList();
    } catch { /* handled */ }
  };

  const handleModalOk = async () => {
    try {
      const values = await form.validateFields();
      setConfirmLoading(true);
      const params = { ...values, isShow: values.isShow ? 1 : 0 };
      if (editingId) {
        await menuUpdate({ ...params, id: editingId });
        message.success('编辑成功');
      } else {
        await menuAdd(params);
        message.success('添加成功');
      }
      setModalVisible(false);
      fetchList();
    } catch { /* validation */ }
    finally { setConfirmLoading(false); }
  };

  const columns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 70 },
    { title: '菜单名称', dataIndex: 'name', width: 160 },
    { title: '类型', dataIndex: 'menuType', width: 80, render: (v: string) => { const t = menuTypeMap[v]; return t ? <Tag color={t.color}>{t.text}</Tag> : v; } },
    { title: '权限标识', dataIndex: 'perms', ellipsis: true, width: 200 },
    { title: '组件路径', dataIndex: 'component', ellipsis: true, width: 160 },
    { title: '排序', dataIndex: 'sort', width: 70 },
    { title: '显示', dataIndex: 'isShow', width: 80, render: (v: any, r: any) => <Switch size="small" checked={v === 1 || v === true} onChange={() => handleToggleShow(r)} /> },
    {
      title: '操作', width: 180, fixed: 'right',
      render: (_: any, record: any) => (
        <Space size="small">
          {record.menuType !== 'A' && <a onClick={() => handleAdd(record.id)}>添加子菜单</a>}
          <a onClick={() => handleEdit(record.id)}>编辑</a>
          <a style={{ color: '#ff4d4f' }} onClick={() => handleDelete(record.id)}>删除</a>
        </Space>
      ),
    },
  ];

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
      <Card bodyStyle={{ paddingBottom: 0 }}>
        <Form layout="inline" style={{ marginBottom: 16 }}>
          <Form.Item><Input placeholder="菜单名称" value={name} onChange={(e) => setName(e.target.value)} allowClear style={{ width: 180 }} /></Form.Item>
          <Form.Item>
            <Select placeholder="菜单类型" allowClear style={{ width: 120 }} value={menuType} onChange={(v) => setMenuType(v)}>
              <Select.Option value="M">目录</Select.Option>
              <Select.Option value="C">菜单</Select.Option>
              <Select.Option value="A">按钮</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item><Button type="primary" icon={<SearchOutlined />} onClick={fetchList}>搜索</Button></Form.Item>
          <Form.Item><Button onClick={() => { setName(''); setMenuType(undefined); }}>重置</Button></Form.Item>
          <Form.Item><Button type="primary" icon={<PlusOutlined />} onClick={() => handleAdd(0)}>添加菜单</Button></Form.Item>
        </Form>
      </Card>
      <Card>
        <Table rowKey="id" columns={columns} dataSource={list} loading={loading} size="small"
          scroll={{ x: 1000 }} pagination={false} />
      </Card>
      <Modal title={editingId ? '编辑菜单' : '添加菜单'} open={modalVisible} onOk={handleModalOk}
        onCancel={() => setModalVisible(false)} confirmLoading={confirmLoading} destroyOnClose width={600}>
        <Form form={form} labelCol={{ span: 5 }} wrapperCol={{ span: 17 }} initialValues={{ pid: 0, menuType: 'C', sort: 0, isShow: true }}>
          <Form.Item label="上级菜单" name="pid"><Input placeholder="上级菜单ID，0为顶级" /></Form.Item>
          <Form.Item label="菜单类型" name="menuType" rules={[{ required: true }]}>
            <Select><Select.Option value="M">目录</Select.Option><Select.Option value="C">菜单</Select.Option><Select.Option value="A">按钮</Select.Option></Select>
          </Form.Item>
          <Form.Item label="菜单名称" name="name" rules={[{ required: true, message: '请输入菜单名称' }]}><Input placeholder="请输入菜单名称" /></Form.Item>
          <Form.Item label="图标" name="icon">
            <Input placeholder="请选择图标" readOnly onClick={() => setIconModalVisible(true)}
              prefix={selectedIcon ? renderIcon(selectedIcon) : undefined}
              style={{ cursor: 'pointer' }} />
          </Form.Item>
          <Form.Item label="权限标识" name="perms"><Input placeholder="如 admin:system:menu:list" /></Form.Item>
          <Form.Item label="组件路径" name="component"><Input placeholder="请输入组件路径" /></Form.Item>
          <Form.Item label="排序" name="sort"><Input type="number" placeholder="排序值" /></Form.Item>
          <Form.Item label="是否显示" name="isShow" valuePropName="checked"><Switch checkedChildren="显示" unCheckedChildren="隐藏" /></Form.Item>
        </Form>
      </Modal>
      <IconFrom open={iconModalVisible} onCancel={() => setIconModalVisible(false)}
        onOk={(icon) => { form.setFieldsValue({ icon }); setSelectedIcon(icon); setIconModalVisible(false); }} />
    </div>
  );
};

export default OperationMenu;
