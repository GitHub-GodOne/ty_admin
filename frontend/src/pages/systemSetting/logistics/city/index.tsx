import React, { useState, useEffect, useCallback } from 'react';
import { Card, Table, Form, Input, Button, Space, Switch, Modal, message } from 'antd';
import { SearchOutlined, ReloadOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { cityList, cityUpdate, cityInfo, updateStatus } from '@/api/logistics';

const CityList: React.FC = () => {
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [keywords, setKeywords] = useState('');
  const [pagination, setPagination] = useState({ current: 1, pageSize: 20, total: 0 });
  const [modalOpen, setModalOpen] = useState(false);
  const [editData, setEditData] = useState<any>(null);
  const [form] = Form.useForm();

  const fetchList = useCallback(async (page = 1) => {
    setLoading(true);
    try {
      const res = await cityList({ page, limit: pagination.pageSize, keywords: keywords || undefined });
      setList(res?.list || []);
      setPagination((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch {
      message.error('获取城市列表失败');
    } finally {
      setLoading(false);
    }
  }, [keywords, pagination.pageSize]);

  useEffect(() => { fetchList(1); }, []);

  const handleReset = () => { setKeywords(''); };

  const handleStatusChange = async (record: any, checked: boolean) => {
    try {
      await updateStatus({ id: record.cityId, status: checked ? 1 : 0 });
      message.success('修改成功');
      fetchList(pagination.current);
    } catch { /* noop */ }
  };

  const handleEdit = async (id: number) => {
    try {
      const res = await cityInfo({ cityId: id });
      setEditData(res);
      form.setFieldsValue(res);
      setModalOpen(true);
    } catch { /* noop */ }
  };

  const handleSave = async () => {
    try {
      const values = await form.validateFields();
      await cityUpdate({ ...values, cityId: editData?.cityId });
      message.success('修改成功');
      setModalOpen(false);
      fetchList(pagination.current);
    } catch { /* noop */ }
  };

  const columns: ColumnsType<any> = [
    { title: '城市ID', dataIndex: 'cityId', width: 80 },
    { title: '城市名称', dataIndex: 'name', width: 150 },
    { title: '上级城市', dataIndex: 'parentName', width: 150 },
    { title: '级别', dataIndex: 'level', width: 80 },
    {
      title: '状态', width: 100,
      render: (_: any, record: any) => (
        <Switch size="small" checked={!!record.isShow}
          onChange={(checked) => handleStatusChange(record, checked)} />
      ),
    },
    {
      title: '操作', width: 100, fixed: 'right',
      render: (_: any, record: any) => (
        <a onClick={() => handleEdit(record.cityId)}>编辑</a>
      ),
    },
  ];

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
      <Card bodyStyle={{ paddingBottom: 0 }}>
        <Form layout="inline" style={{ marginBottom: 16 }}>
          <Form.Item>
            <Input placeholder="城市名称" value={keywords} onChange={(e) => setKeywords(e.target.value)}
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
        <Table rowKey="cityId" columns={columns} dataSource={list} loading={loading} size="small"
          scroll={{ x: 700 }}
          pagination={{ ...pagination, showSizeChanger: true, pageSizeOptions: ['10', '20', '30', '40'],
            showTotal: (t: number) => `共 ${t} 条`,
            onChange: (p: number, ps: number) => { setPagination((prev) => ({ ...prev, pageSize: ps })); fetchList(p); } }} />
      </Card>

      <Modal title="编辑城市" open={modalOpen} onOk={handleSave}
        onCancel={() => setModalOpen(false)} width={500} destroyOnClose>
        <Form form={form} labelCol={{ span: 6 }} wrapperCol={{ span: 16 }} preserve={false}>
          <Form.Item label="城市名称" name="name" rules={[{ required: true, message: '请输入城市名称' }]}>
            <Input placeholder="请输入城市名称" />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
};

export default CityList;
