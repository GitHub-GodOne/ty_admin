import React, { useState, useEffect, useCallback } from 'react';
import { Card, Table, Form, Input, Button, Space, Tag, Modal, Switch, Select, message } from 'antd';
import { PlusOutlined, SearchOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { adminList as adminListApi, adminAdd as adminSaveApi, adminUpdate as adminEditApi, adminDel as adminDeleteApi, adminInfo as adminInfoApi } from '@/api/systemAdmin';
import { getRoleList } from '@/api/role';
import { useModal } from '@/hooks/useModal';
import { usePagination } from '@/hooks/usePagination';

const OperationAdmin: React.FC = () => {
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [keywords, setKeywords] = useState('');
  const [modalVisible, setModalVisible] = useState(false);
  const [editingId, setEditingId] = useState<number | null>(null);
  const [confirmLoading, setConfirmLoading] = useState(false);
  const [roleOptions, setRoleOptions] = useState<any[]>([]);
  const [form] = Form.useForm();
  const { deleteConfirm } = useModal();
  const { pagination, setTotal, antdPagination } = usePagination();

  const fetchRoles = useCallback(async () => {
    try {
      const res: any = await getRoleList({ page: 1, limit: 100 });
      setRoleOptions(res?.list || []);
    } catch { /* noop */ }
  }, []);

  const fetchList = useCallback(async () => {
    setLoading(true);
    try {
      const res: any = await adminListApi({
        realName: keywords,
        page: pagination.page,
        limit: pagination.limit,
      });
      setList(res?.list || []);
      setTotal(res?.total || 0);
    } catch {
      /* handled by interceptor */
    } finally {
      setLoading(false);
    }
  }, [keywords, pagination.page, pagination.limit]);

  useEffect(() => { fetchRoles(); }, [fetchRoles]);
  useEffect(() => { fetchList(); }, [fetchList]);

  const handleAdd = () => {
    setEditingId(null);
    form.resetFields();
    setModalVisible(true);
  };

  const handleEdit = async (id: number) => {
    try {
      const res: any = await adminInfoApi({ id });
      setEditingId(id);
      form.setFieldsValue({
        account: res?.account,
        realName: res?.realName,
        phone: res?.phone,
        roles: res?.roles ? (typeof res.roles === 'string' ? res.roles.split(',').map(Number) : res.roles) : [],
        status: res?.status === 1,
      });
      setModalVisible(true);
    } catch {
      message.error('获取管理员详情失败');
    }
  };

  const handleDelete = (id: number) => {
    deleteConfirm(async () => {
      await adminDeleteApi({ id });
      message.success('删除成功');
      fetchList();
    });
  };

  const handleModalOk = async () => {
    try {
      const values = await form.validateFields();
      setConfirmLoading(true);
      const params = { ...values, status: values.status ? 1 : 0 };
      if (editingId) {
        await adminEditApi({ ...params, id: editingId });
        message.success('编辑成功');
      } else {
        await adminSaveApi(params);
        message.success('添加成功');
      }
      setModalVisible(false);
      fetchList();
    } catch {
      /* validation or api error */
    } finally {
      setConfirmLoading(false);
    }
  };

  const columns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 80 },
    { title: '管理员名称', dataIndex: 'realName', width: 130 },
    { title: '角色', dataIndex: 'roles', width: 120 },
    { title: '手机号', dataIndex: 'phone', width: 130 },
    {
      title: '状态', dataIndex: 'status', width: 80,
      render: (val: number) => <Tag color={val ? 'green' : 'default'}>{val ? '启用' : '禁用'}</Tag>,
    },
    { title: '最后登录时间', dataIndex: 'lastLoginTime', width: 180 },
    {
      title: '操作', width: 150, fixed: 'right',
      render: (_: any, record: any) => (
        <Space>
          <a onClick={() => handleEdit(record.id)}>编辑</a>
          <a onClick={() => handleDelete(record.id)} style={{ color: '#ff4d4f' }}>删除</a>
        </Space>
      ),
    },
  ];

  return (
    <div>
      <Card bodyStyle={{ padding: '16px' }} style={{ marginBottom: 16 }}>
        <Form layout="inline">
          <Form.Item label="管理员名称">
            <Input value={keywords} onChange={(e) => setKeywords(e.target.value)} placeholder="请输入管理员名称" allowClear />
          </Form.Item>
          <Form.Item>
            <Space>
              <Button type="primary" icon={<SearchOutlined />} onClick={fetchList}>搜索</Button>
              <Button onClick={() => setKeywords('')}>重置</Button>
              <Button type="primary" icon={<PlusOutlined />} onClick={handleAdd}>添加管理员</Button>
            </Space>
          </Form.Item>
        </Form>
      </Card>
      <Card>
        <Table rowKey="id" columns={columns} dataSource={list} loading={loading} pagination={antdPagination} scroll={{ x: 900 }} size="small" />
      </Card>
      <Modal
        title={editingId ? '编辑管理员' : '添加管理员'}
        open={modalVisible}
        onOk={handleModalOk}
        onCancel={() => setModalVisible(false)}
        confirmLoading={confirmLoading}
        destroyOnClose
      >
        <Form form={form} labelCol={{ span: 6 }} wrapperCol={{ span: 16 }} initialValues={{ status: true, roles: [] }}>
          <Form.Item label="账号" name="account" rules={[{ required: true, message: '请输入账号' }]}>
            <Input placeholder="请输入账号" />
          </Form.Item>
          <Form.Item label="姓名" name="realName" rules={[{ required: true, message: '请输入姓名' }]}>
            <Input placeholder="请输入姓名" />
          </Form.Item>
          <Form.Item label="手机号" name="phone">
            <Input placeholder="请输入手机号" />
          </Form.Item>
          {!editingId && (
            <Form.Item label="密码" name="pwd" rules={[{ required: true, message: '请输入密码' }]}>
              <Input.Password placeholder="请输入密码" />
            </Form.Item>
          )}
          <Form.Item label="角色" name="roles" rules={[{ required: true, message: '请选择角色' }]}>
            <Select mode="multiple" placeholder="请选择角色">
              {roleOptions.map((r: any) => (
                <Select.Option key={r.id} value={r.id}>{r.roleName}</Select.Option>
              ))}
            </Select>
          </Form.Item>
          <Form.Item label="状态" name="status" valuePropName="checked">
            <Switch checkedChildren="启用" unCheckedChildren="禁用" />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
};

export default OperationAdmin;
