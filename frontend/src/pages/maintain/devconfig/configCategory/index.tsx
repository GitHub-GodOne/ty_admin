import React, { useState, useEffect, useCallback } from 'react';
import { Card, Table, Button, Space, Tag, Modal, Form, Input, InputNumber, Radio, Cascader, message, Popconfirm } from 'antd';
import { PlusOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { treeCategroy, addCategroy, updateCategroy, deleteCategroy, infoCategroy } from '@/api/categoryApi';

const CATEGORY_TYPE = 10; // config category type

const ConfigCategory: React.FC = () => {
  const [treeData, setTreeData] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [editId, setEditId] = useState<number | null>(null);
  const [form] = Form.useForm();
  const [cascaderOptions, setCascaderOptions] = useState<any[]>([]);

  const fetchTree = useCallback(async () => {
    setLoading(true);
    try {
      const res: any = await treeCategroy({ type: CATEGORY_TYPE, status: -1 });
      const list = Array.isArray(res) ? res : [];
      setTreeData(addChildrenKey(list));
      setCascaderOptions([{ id: 0, name: '顶级分类', child: list }]);
    } catch { message.error('获取配置分类失败'); }
    finally { setLoading(false); }
  }, []);

  useEffect(() => { fetchTree(); }, []);

  const addChildrenKey = (list: any[]): any[] =>
    list.map((item: any) => ({
      ...item, key: item.id,
      children: item.child?.length ? addChildrenKey(item.child) : undefined,
    }));

  const handleAdd = (pid = 0) => {
    setEditId(null);
    form.setFieldsValue({ name: '', url: '', sort: 0, status: 1, pid });
    setModalVisible(true);
  };

  const handleEdit = async (id: number) => {
    try {
      const res: any = await infoCategroy({ id });
      setEditId(id);
      form.setFieldsValue({ name: res.name, url: res.url, sort: res.sort, status: res.status, pid: res.pid || 0 });
      setModalVisible(true);
    } catch { message.error('获取分类详情失败'); }
  };

  const handleDelete = async (id: number) => {
    try { await deleteCategroy({ id }); message.success('删除成功'); fetchTree(); } catch { /* */ }
  };

  const handleSave = async () => {
    try {
      const values = await form.validateFields();
      const params = { ...values, type: CATEGORY_TYPE };
      if (editId) { await updateCategroy({ ...params, id: editId }); message.success('编辑成功'); }
      else { await addCategroy(params); message.success('添加成功'); }
      setModalVisible(false); fetchTree();
    } catch { /* */ }
  };

  const columns: ColumnsType<any> = [
    { title: '分类名称', dataIndex: 'name', width: 200 },
    { title: '英文名称', dataIndex: 'url', width: 150 },
    { title: '关联表单', dataIndex: 'extra', width: 100 },
    { title: '状态', dataIndex: 'status', width: 80,
      render: (v: number) => v === 1 ? <Tag color="green">启用</Tag> : <Tag color="default">禁用</Tag> },
    {
      title: '操作', width: 250,
      render: (_: any, record: any) => (
        <Space size="small">
          <a onClick={() => handleAdd(record.id)}>添加子分类</a>
          <a onClick={() => handleEdit(record.id)}>编辑</a>
          <Popconfirm title="确定删除?" onConfirm={() => handleDelete(record.id)}>
            <a style={{ color: '#ff4d4f' }}>删除</a>
          </Popconfirm>
        </Space>
      ),
    },
  ];

  return (
    <Card>
      <div style={{ marginBottom: 16 }}>
        <Button type="primary" icon={<PlusOutlined />} onClick={() => handleAdd(0)}>添加配置分类</Button>
      </div>
      <Table rowKey="id" columns={columns} dataSource={treeData} loading={loading}
        size="small" pagination={false} expandable={{ defaultExpandAllRows: true }} />
      <Modal title={editId ? '编辑分类' : '添加分类'} open={modalVisible} onOk={handleSave}
        onCancel={() => setModalVisible(false)} destroyOnClose width={500}>
        <Form form={form} labelCol={{ span: 5 }} wrapperCol={{ span: 17 }}>
          <Form.Item label="分类名称" name="name" rules={[{ required: true, message: '请输入分类名称' }]}><Input placeholder="请输入分类名称" /></Form.Item>
          <Form.Item label="英文名称" name="url"><Input placeholder="请输入英文名称" /></Form.Item>
          <Form.Item label="排序" name="sort"><InputNumber min={0} style={{ width: '100%' }} /></Form.Item>
          <Form.Item label="状态" name="status"><Radio.Group><Radio value={1}>启用</Radio><Radio value={0}>禁用</Radio></Radio.Group></Form.Item>
          <Form.Item name="pid" hidden><Input /></Form.Item>
        </Form>
      </Modal>
    </Card>
  );
};

function addChildrenKey(list: any[]): any[] {
  return list.map((item: any) => ({
    ...item, key: item.id,
    children: item.child?.length ? addChildrenKey(item.child) : undefined,
  }));
}

export default ConfigCategory;
