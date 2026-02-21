import React, { useState, useEffect, useCallback } from 'react';
import { Card, Table, Button, Modal, Form, Input, message } from 'antd';
import type { ColumnsType } from 'antd/es/table';
import { cityList, cityUpdate, cityInfo } from '@/api/logistics';

const CityList: React.FC = () => {
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [parentId, setParentId] = useState(0);
  const [parentName, setParentName] = useState('中国');
  const [modalOpen, setModalOpen] = useState(false);
  const [editId, setEditId] = useState<number | null>(null);
  const [confirmLoading, setConfirmLoading] = useState(false);
  const [form] = Form.useForm();

  const fetchList = useCallback(async (pid: number) => {
    setLoading(true);
    try {
      const res = await cityList({ parentId: pid });
      setList(Array.isArray(res) ? res : res?.list || []);
    } catch {
      message.error('获取城市列表失败');
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => { fetchList(parentId); }, [parentId]);

  const handleDrillDown = (record: any) => {
    setParentId(record.cityId);
    setParentName(record.name);
  };

  const handleBack = () => {
    setParentId(0);
    setParentName('中国');
  };

  const handleEdit = async (record: any) => {
    setEditId(record.id);
    try {
      const res = await cityInfo({ id: record.id });
      form.setFieldsValue({ name: res?.name || record.name });
      setModalOpen(true);
    } catch {
      // fallback: use table data
      form.setFieldsValue({ name: record.name });
      setModalOpen(true);
    }
  };
  const handleSave = async () => {
    try {
      const values = await form.validateFields();
      setConfirmLoading(true);
      await cityUpdate(editId!, { parentId, name: values.name });
      message.success('修改成功');
      setModalOpen(false);
      fetchList(parentId);
    } catch { /* validation */ }
    finally { setConfirmLoading(false); }
  };

  const columns: ColumnsType<any> = [
    { title: '编号', dataIndex: 'cityId', width: 100 },
    { title: '上级名称', width: 100, render: () => parentName },
    {
      title: '地区名称', width: 250,
      render: (_: any, record: any) => (
        <a onClick={() => handleDrillDown(record)}>{record.name}</a>
      ),
    },
    {
      title: '操作', width: 80, fixed: 'right',
      render: (_: any, record: any) => (
        <a onClick={() => handleEdit(record)}>编辑</a>
      ),
    },
  ];

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
      <Card>
        {parentId > 0 && (
          <div style={{ marginBottom: 16 }}>
            <Button type="primary" onClick={handleBack}>返回</Button>
          </div>
        )}
        <Table rowKey="cityId" columns={columns} dataSource={list} loading={loading}
          size="small" scroll={{ x: 600 }} pagination={false} />
      </Card>

      <Modal title="编辑城市" open={modalOpen} onOk={handleSave}
        onCancel={() => setModalOpen(false)} confirmLoading={confirmLoading}
        width={500} destroyOnClose>
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
