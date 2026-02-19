import React, { useState, useEffect, useCallback } from 'react';
import { Card, Table, Button, Space, Input, Modal, Form, message, Popconfirm } from 'antd';
import { PlusOutlined, SearchOutlined, ReloadOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { groupList, groupSave, groupEdit, groupDelete, groupDataList } from '@/api/systemGroup';

const CombinedData: React.FC = () => {
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [keywords, setKeywords] = useState('');
  const [pagination, setPagination] = useState({ current: 1, pageSize: 20, total: 0 });
  const [modalVisible, setModalVisible] = useState(false);
  const [editId, setEditId] = useState<number | null>(null);
  const [form] = Form.useForm();
  // Data list modal
  const [dataModalVisible, setDataModalVisible] = useState(false);
  const [dataList, setDataList] = useState<any[]>([]);
  const [dataLoading, setDataLoading] = useState(false);
  const [currentGroup, setCurrentGroup] = useState<any>(null);

  const fetchList = useCallback(async (page = 1) => {
    setLoading(true);
    try {
      const res: any = await groupList({ page, limit: pagination.pageSize, keywords: keywords || undefined });
      setList(res?.list || []);
      setPagination((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch { message.error('获取组合数据失败'); }
    finally { setLoading(false); }
  }, [keywords, pagination.pageSize]);

  useEffect(() => { fetchList(1); }, []);

  const handleAdd = () => { setEditId(null); form.resetFields(); setModalVisible(true); };

  const handleEdit = (record: any) => {
    setEditId(record.id);
    form.setFieldsValue({ name: record.name, info: record.info, formId: record.formId });
    setModalVisible(true);
  };

  const handleDelete = async (id: number) => {
    try { await groupDelete({ id }); message.success('删除成功'); fetchList(pagination.current); } catch { /* */ }
  };

  const handleSave = async () => {
    try {
      const values = await form.validateFields();
      if (editId) { await groupEdit({ ...values, id: editId }); message.success('编辑成功'); }
      else { await groupSave(values); message.success('添加成功'); }
      setModalVisible(false); fetchList(pagination.current);
    } catch { /* */ }
  };

  const openDataList = async (record: any) => {
    setCurrentGroup(record);
    setDataModalVisible(true);
    setDataLoading(true);
    try {
      const res: any = await groupDataList({ gid: record.id, page: 1, limit: 100 });
      setDataList(res?.list || []);
    } catch { message.error('获取数据列表失败'); }
    finally { setDataLoading(false); }
  };

  const columns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 60 },
    { title: '数据组名称', dataIndex: 'name', width: 200 },
    { title: '简介', dataIndex: 'info', ellipsis: true },
    {
      title: '操作', width: 220,
      render: (_: any, record: any) => (
        <Space size="small">
          <a onClick={() => openDataList(record)}>数据列表</a>
          <a onClick={() => handleEdit(record)}>编辑</a>
          <Popconfirm title="确定删除?" onConfirm={() => handleDelete(record.id)}>
            <a style={{ color: '#ff4d4f' }}>删除</a>
          </Popconfirm>
        </Space>
      ),
    },
  ];

  const dataColumns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 60 },
    { title: '数据值', dataIndex: 'value', ellipsis: true },
    { title: '排序', dataIndex: 'sort', width: 80 },
    { title: '状态', dataIndex: 'status', width: 80,
      render: (v: number) => v === 1 ? '启用' : '禁用' },
  ];

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
      <Card bodyStyle={{ paddingBottom: 0 }}>
        <Space style={{ marginBottom: 16 }}>
          <Input placeholder="数据组名称" value={keywords} onChange={(e) => setKeywords(e.target.value)}
            onPressEnter={() => fetchList(1)} allowClear prefix={<SearchOutlined />} style={{ width: 240 }} />
          <Button type="primary" onClick={() => fetchList(1)}>搜索</Button>
          <Button icon={<ReloadOutlined />} onClick={() => { setKeywords(''); fetchList(1); }}>重置</Button>
        </Space>
      </Card>
      <Card>
        <div style={{ marginBottom: 16 }}>
          <Button type="primary" icon={<PlusOutlined />} onClick={handleAdd}>添加组合数据</Button>
        </div>
        <Table rowKey="id" columns={columns} dataSource={list} loading={loading} size="small"
          pagination={{
            ...pagination, showSizeChanger: true, showTotal: (t) => `共 ${t} 条`,
            onChange: (p, ps) => { setPagination((prev) => ({ ...prev, pageSize: ps })); fetchList(p); },
          }} />
      </Card>

      <Modal title={editId ? '编辑组合数据' : '添加组合数据'} open={modalVisible} onOk={handleSave}
        onCancel={() => setModalVisible(false)} destroyOnClose width={500}>
        <Form form={form} labelCol={{ span: 5 }} wrapperCol={{ span: 17 }}>
          <Form.Item label="数据组名" name="name" rules={[{ required: true, message: '请输入数据组名称' }]}>
            <Input placeholder="请输入数据组名称" />
          </Form.Item>
          <Form.Item label="简介" name="info"><Input.TextArea placeholder="请输入简介" rows={3} /></Form.Item>
          <Form.Item label="关联表单ID" name="formId"><Input placeholder="请输入关联表单ID" /></Form.Item>
        </Form>
      </Modal>

      <Modal title={`数据列表 - ${currentGroup?.name || ''}`} open={dataModalVisible}
        onCancel={() => setDataModalVisible(false)} footer={null} width={700}>
        <Table rowKey="id" columns={dataColumns} dataSource={dataList} loading={dataLoading}
          size="small" pagination={false} />
      </Modal>
    </div>
  );
};

export default CombinedData;
