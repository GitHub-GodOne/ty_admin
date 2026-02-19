import React, { useState, useEffect, useCallback } from 'react';
import { Card, Table, Button, Space, Modal, Form, Input, message, Popconfirm } from 'antd';
import { PlusOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { groupListApi, groupSaveApi, groupUpdateApi, groupDeleteApi, groupInfoApi } from '@/api/user';

const UserGroup: React.FC = () => {
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [editId, setEditId] = useState<number | null>(null);
  const [form] = Form.useForm();
  const [pagination, setPagination] = useState({ current: 1, pageSize: 20, total: 0 });

  const fetchList = useCallback(async (page = 1) => {
    setLoading(true);
    try {
      const res = await groupListApi({ page, limit: pagination.pageSize });
      const data = Array.isArray(res) ? res : res?.list || [];
      setList(data);
      setPagination((p) => ({ ...p, current: page, total: res?.total || data.length }));
    } catch { message.error('获取分组列表失败'); }
    finally { setLoading(false); }
  }, [pagination.pageSize]);

  useEffect(() => { fetchList(1); }, []);

  const handleEdit = async (id: number) => {
    try {
      const info = await groupInfoApi({ id });
      setEditId(id);
      form.setFieldsValue(info);
      setModalOpen(true);
    } catch { message.error('获取详情失败'); }
  };

  const handleDelete = async (id: number) => {
    try {
      await groupDeleteApi({ id });
      message.success('删除成功');
      fetchList(pagination.current);
    } catch { message.error('删除失败'); }
  };

  const handleSubmit = async () => {
    const values = await form.validateFields();
    try {
      if (editId) {
        await groupUpdateApi({ id: editId }, values);
      } else {
        await groupSaveApi(values);
      }
      message.success(editId ? '编辑成功' : '添加成功');
      setModalOpen(false);
      form.resetFields();
      setEditId(null);
      fetchList(1);
    } catch { message.error('操作失败'); }
  };

  const columns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 80 },
    { title: '分组名称', dataIndex: 'groupName' },
    {
      title: '操作', width: 150,
      render: (_: any, r: any) => (
        <Space>
          <a onClick={() => handleEdit(r.id)}>编辑</a>
          <Popconfirm title="确定删除?" onConfirm={() => handleDelete(r.id)}>
            <a style={{ color: '#ff4d4f' }}>删除</a>
          </Popconfirm>
        </Space>
      ),
    },
  ];

  return (
    <Card title="用户分组" extra={
      <Button type="primary" icon={<PlusOutlined />}
        onClick={() => { setEditId(null); form.resetFields(); setModalOpen(true); }}>
        添加分组
      </Button>
    }>
      <Table rowKey="id" columns={columns} dataSource={list} loading={loading} size="small"
        pagination={{ ...pagination, showTotal: (t) => `共 ${t} 条`,
          onChange: (p) => fetchList(p) }} />
      <Modal title={editId ? '编辑分组' : '添加分组'} open={modalOpen}
        onCancel={() => setModalOpen(false)} onOk={handleSubmit} destroyOnClose>
        <Form form={form} layout="vertical">
          <Form.Item name="groupName" label="分组名称"
            rules={[{ required: true, message: '请输入分组名称' }]}>
            <Input placeholder="请输入分组名称" />
          </Form.Item>
        </Form>
      </Modal>
    </Card>
  );
};

export default UserGroup;
