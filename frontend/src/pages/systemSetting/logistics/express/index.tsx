import React, { useState, useEffect, useCallback } from 'react';
import { Card, Table, Form, Input, Select, Button, Space, Switch, Modal, message, Popconfirm } from 'antd';
import { PlusOutlined, SearchOutlined, ReloadOutlined, SyncOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { expressList, expressDelete, expressUpdateShow, expressSave, expressUpdate, expressInfo, expressSyncApi } from '@/api/logistics';

const ExpressList: React.FC = () => {
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [keywords, setKeywords] = useState('');
  const [pagination, setPagination] = useState({ current: 1, pageSize: 20, total: 0 });
  const [modalOpen, setModalOpen] = useState(false);
  const [editId, setEditId] = useState<number | null>(null);
  const [form] = Form.useForm();

  const fetchList = useCallback(async (page = 1) => {
    setLoading(true);
    try {
      const res = await expressList({ page, limit: pagination.pageSize, keywords: keywords || undefined });
      setList(res?.list || []);
      setPagination((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch {
      message.error('获取物流公司列表失败');
    } finally {
      setLoading(false);
    }
  }, [keywords, pagination.pageSize]);

  useEffect(() => { fetchList(1); }, []);

  const handleReset = () => { setKeywords(''); };

  const handleDelete = async (id: number) => {
    try {
      await expressDelete({ id });
      message.success('删除成功');
      fetchList(pagination.current);
    } catch { /* noop */ }
  };

  const handleStatusChange = async (record: any, checked: boolean) => {
    try {
      await expressUpdateShow({ id: record.id, isShow: checked ? 1 : 0 });
      message.success('修改成功');
      fetchList(pagination.current);
    } catch { /* noop */ }
  };

  const handleSync = async () => {
    try {
      await expressSyncApi();
      message.success('同步成功');
      fetchList(1);
    } catch { /* noop */ }
  };

  const handleEdit = async (id: number) => {
    setEditId(id);
    try {
      const res = await expressInfo({ id });
      form.setFieldsValue(res);
      setModalOpen(true);
    } catch { /* noop */ }
  };

  const handleAdd = () => {
    setEditId(null);
    form.resetFields();
    setModalOpen(true);
  };

  const handleSave = async () => {
    try {
      const values = await form.validateFields();
      if (editId) {
        await expressUpdate({ ...values, id: editId });
      } else {
        await expressSave(values);
      }
      message.success('操作成功');
      setModalOpen(false);
      fetchList(pagination.current);
    } catch { /* noop */ }
  };

  const columns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 60 },
    { title: '公司名称', dataIndex: 'name', width: 150 },
    { title: '编码', dataIndex: 'code', width: 120 },
    { title: '排序', dataIndex: 'sort', width: 80 },
    {
      title: '是否显示', width: 100,
      render: (_: any, record: any) => (
        <Switch size="small" checked={!!record.isShow}
          onChange={(checked) => handleStatusChange(record, checked)} />
      ),
    },
    {
      title: '操作', width: 150, fixed: 'right',
      render: (_: any, record: any) => (
        <Space size="small">
          <a onClick={() => handleEdit(record.id)}>编辑</a>
          <Popconfirm title="确定删除?" onConfirm={() => handleDelete(record.id)}>
            <a style={{ color: '#ff4d4f' }}>删除</a>
          </Popconfirm>
        </Space>
      ),
    },
  ];

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
      <Card bodyStyle={{ paddingBottom: 0 }}>
        <Form layout="inline" style={{ marginBottom: 16 }}>
          <Form.Item>
            <Input placeholder="物流公司名称" value={keywords} onChange={(e) => setKeywords(e.target.value)}
              onPressEnter={() => fetchList(1)} allowClear prefix={<SearchOutlined />} style={{ width: 240 }} />
          </Form.Item>
          <Form.Item>
            <Button type="primary" onClick={() => fetchList(1)}>搜索</Button>
          </Form.Item>
          <Form.Item>
            <Button icon={<ReloadOutlined />} onClick={handleReset}>重置</Button>
          </Form.Item>
        </Form>
      </Card>
      <Card>
        <div style={{ display: 'flex', justifyContent: 'flex-end', marginBottom: 16, gap: 8 }}>
          <Button icon={<SyncOutlined />} onClick={handleSync}>同步物流公司</Button>
          <Button type="primary" icon={<PlusOutlined />} onClick={handleAdd}>添加物流公司</Button>
        </div>
        <Table rowKey="id" columns={columns} dataSource={list} loading={loading} size="small"
          scroll={{ x: 700 }}
          pagination={{ ...pagination, showSizeChanger: true, pageSizeOptions: ['10', '20', '30', '40'],
            showTotal: (t: number) => `共 ${t} 条`,
            onChange: (p: number, ps: number) => { setPagination((prev) => ({ ...prev, pageSize: ps })); fetchList(p); } }} />
      </Card>

      <Modal title={editId ? '编辑物流公司' : '添加物流公司'} open={modalOpen} onOk={handleSave}
        onCancel={() => setModalOpen(false)} width={500} destroyOnClose>
        <Form form={form} labelCol={{ span: 6 }} wrapperCol={{ span: 16 }} preserve={false}>
          <Form.Item label="公司名称" name="name" rules={[{ required: true, message: '请输入公司名称' }]}>
            <Input placeholder="请输入公司名称" />
          </Form.Item>
          <Form.Item label="编码" name="code" rules={[{ required: true, message: '请输入编码' }]}>
            <Input placeholder="请输入编码" />
          </Form.Item>
          <Form.Item label="排序" name="sort" initialValue={0}>
            <Input type="number" />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
};

export default ExpressList;
