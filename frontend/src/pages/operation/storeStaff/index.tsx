import React, { useState, useEffect, useCallback } from 'react';
import { Card, Table, Form, Input, Button, Space, Tag, Modal, Switch, Select, message } from 'antd';
import { PlusOutlined, SearchOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import {
  storeStaffListApi, storeStaffSaveApi, storeStaffDeleteApi,
  storeStaffUpdateApi, storeStaffInfoApi, storeListApi,
} from '@/api/storePoint';
import { adminList } from '@/api/systemAdmin';
import { useModal } from '@/hooks/useModal';
import { usePagination } from '@/hooks/usePagination';

const OperationStoreStaff: React.FC = () => {
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [storeId, setStoreId] = useState<number | undefined>(undefined);
  const [storeOptions, setStoreOptions] = useState<any[]>([]);
  const [modalVisible, setModalVisible] = useState(false);
  const [editingId, setEditingId] = useState<number | null>(null);
  const [confirmLoading, setConfirmLoading] = useState(false);
  const [form] = Form.useForm();
  const { deleteConfirm } = useModal();
  const { pagination, setTotal, antdPagination } = usePagination();

  // Admin picker state
  const [adminPickerVisible, setAdminPickerVisible] = useState(false);
  const [adminListData, setAdminListData] = useState<any[]>([]);
  const [adminLoading, setAdminLoading] = useState(false);
  const [adminKeywords, setAdminKeywords] = useState('');
  const [selectedAdmin, setSelectedAdmin] = useState<string>('');

  const fetchStores = useCallback(async () => {
    try { const res: any = await storeListApi({ page: 1, limit: 100, status: 1 }); setStoreOptions(res?.list || []); } catch { /* */ }
  }, []);

  const fetchList = useCallback(async () => {
    setLoading(true);
    try {
      const params: any = { page: pagination.page, limit: pagination.limit };
      if (storeId) params.storeId = storeId;
      const res: any = await storeStaffListApi(params);
      setList(res?.list || []);
      setTotal(res?.total || 0);
    } catch { /* */ }
    finally { setLoading(false); }
  }, [storeId, pagination.page, pagination.limit]);

  useEffect(() => { fetchStores(); }, []);
  useEffect(() => { fetchList(); }, [fetchList]);

  const handleAdd = () => {
    setEditingId(null); form.resetFields(); setSelectedAdmin(''); setModalVisible(true);
  };

  const handleEdit = async (id: number) => {
    try {
      const res: any = await storeStaffInfoApi({ id });
      setEditingId(id);
      setSelectedAdmin(res?.avatar || '');
      form.setFieldsValue({ uid: res?.uid, storeId: res?.storeId, staffName: res?.staffName, phone: res?.phone });
      setModalVisible(true);
    } catch { message.error('获取核销员详情失败'); }
  };

  const handleDelete = (id: number) => {
    deleteConfirm(async () => { await storeStaffDeleteApi({ id }); message.success('删除成功'); fetchList(); });
  };

  const handleModalOk = async () => {
    try {
      const values = await form.validateFields();
      if (values.phone && !/^1[3456789]\d{9}$/.test(values.phone)) {
        message.error('手机号格式不正确'); return;
      }
      setConfirmLoading(true);
      const params = { ...values, avatar: selectedAdmin };
      if (editingId) { await storeStaffUpdateApi({ ...params, id: editingId }); message.success('编辑成功'); }
      else { await storeStaffSaveApi(params); message.success('添加成功'); }
      setModalVisible(false); fetchList();
    } catch { /* validation */ }
    finally { setConfirmLoading(false); }
  };

  // Admin picker
  const openAdminPicker = async () => {
    setAdminPickerVisible(true);
    fetchAdminList();
  };

  const fetchAdminList = async (kw?: string) => {
    setAdminLoading(true);
    try {
      const res: any = await adminList({ page: 1, limit: 50, name: kw || undefined });
      setAdminListData(res?.list || []);
    } catch { /* */ }
    finally { setAdminLoading(false); }
  };

  const selectAdmin = (record: any) => {
    setSelectedAdmin(record.account || record.realName || '');
    form.setFieldsValue({ uid: record.id });
    setAdminPickerVisible(false);
  };

  const adminColumns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 60 },
    { title: '账号', dataIndex: 'account', width: 120 },
    { title: '姓名', dataIndex: 'realName', width: 120 },
    { title: '操作', width: 80, render: (_: any, record: any) => <a onClick={() => selectAdmin(record)}>选择</a> },
  ];

  const columns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 60 },
    { title: '核销员名称', dataIndex: 'staffName', width: 120 },
    { title: '账号', dataIndex: 'avatar', width: 120 },
    { title: '手机号码', dataIndex: 'phone', width: 120 },
    { title: '所属提货点', dataIndex: 'systemStore', width: 150, render: (v: any) => v?.name || '-' },
    { title: '添加时间', dataIndex: 'createTime', width: 160 },
    {
      title: '操作', width: 120, fixed: 'right',
      render: (_: any, record: any) => (
        <Space size="small">
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
          <Form.Item label="提货点名称">
            <Select placeholder="请选择" allowClear style={{ width: 200 }} value={storeId} onChange={(v) => setStoreId(v)}>
              {storeOptions.map((s: any) => <Select.Option key={s.id} value={s.id}>{s.name}</Select.Option>)}
            </Select>
          </Form.Item>
          <Form.Item><Button type="primary" icon={<SearchOutlined />} onClick={fetchList}>搜索</Button></Form.Item>
          <Form.Item><Button onClick={() => setStoreId(undefined)}>重置</Button></Form.Item>
          <Form.Item><Button type="primary" icon={<PlusOutlined />} onClick={handleAdd}>添加核销员</Button></Form.Item>
        </Form>
      </Card>
      <Card>
        <Table rowKey="id" columns={columns} dataSource={list} loading={loading} pagination={antdPagination} scroll={{ x: 900 }} size="small" />
      </Card>

      {/* 添加/编辑核销员 */}
      <Modal title={editingId ? '修改核销员' : '添加核销员'} open={modalVisible} onOk={handleModalOk}
        onCancel={() => setModalVisible(false)} confirmLoading={confirmLoading} destroyOnClose width={540}>
        <Form form={form} labelCol={{ span: 5 }} wrapperCol={{ span: 17 }}>
          <Form.Item label="管理员" name="uid" rules={[{ required: true, message: '请选择管理员' }]}>
            <Space>
              <span>{selectedAdmin || ''}</span>
              <Button type="primary" size="small" onClick={openAdminPicker}>选择管理员</Button>
            </Space>
          </Form.Item>
          <Form.Item label="所属提货点" name="storeId" rules={[{ required: true, message: '请选择提货点' }]}>
            <Select placeholder="请选择" allowClear>
              {storeOptions.map((s: any) => <Select.Option key={s.id} value={s.id}>{s.name}</Select.Option>)}
            </Select>
          </Form.Item>
          <Form.Item label="核销员名称" name="staffName">
            <Input placeholder="请输入核销员名称" />
          </Form.Item>
          <Form.Item label="手机号码" name="phone">
            <Input placeholder="请输入手机号码" />
          </Form.Item>
        </Form>
      </Modal>

      {/* 选择管理员弹窗 */}
      <Modal title="选择管理员" open={adminPickerVisible} onCancel={() => setAdminPickerVisible(false)} footer={null} width={500} destroyOnClose>
        <div style={{ marginBottom: 12 }}>
          <Input.Search placeholder="搜索账号/姓名" allowClear value={adminKeywords}
            onChange={(e) => setAdminKeywords(e.target.value)}
            onSearch={(v) => fetchAdminList(v)} enterButton />
        </div>
        <Table rowKey="id" columns={adminColumns} dataSource={adminListData} loading={adminLoading}
          size="small" pagination={false} scroll={{ y: 300 }} />
      </Modal>
    </div>
  );
};

export default OperationStoreStaff;
