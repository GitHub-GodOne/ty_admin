import React, { useState, useEffect, useCallback } from 'react';
import { Card, Table, Button, Space, Modal, Form, Input, InputNumber, Switch, message, Image } from 'antd';
import { PlusOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { levelListApi, levelSaveApi, levelUpdateApi, levelDeleteApi, levelUseApi } from '@/api/user';
import UploadPicture from '@/components/UploadPicture';

const UserGrade: React.FC = () => {
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [editData, setEditData] = useState<any>(null);
  const [submitLoading, setSubmitLoading] = useState(false);
  const [form] = Form.useForm();

  const fetchList = useCallback(async () => {
    setLoading(true);
    try {
      const res = await levelListApi();
      setList(Array.isArray(res) ? res : res?.list || []);
    } catch { message.error('获取等级列表失败'); }
    finally { setLoading(false); }
  }, []);

  useEffect(() => { fetchList(); }, []);

  const handleAdd = () => {
    setEditData(null);
    form.resetFields();
    form.setFieldsValue({ grade: 1, discount: 100, experience: 0, isShow: true });
    setModalOpen(true);
  };

  const handleEdit = (record: any) => {
    setEditData(record);
    form.setFieldsValue({
      name: record.name,
      grade: record.grade,
      discount: record.discount,
      experience: record.experience,
      icon: record.icon,
      isShow: record.isShow === 1 || record.isShow === true,
    });
    setModalOpen(true);
  };

  const handleDelete = (id: number) => {
    Modal.confirm({
      title: '提示',
      content: '删除会导致对应用户等级数据清空，请谨慎操作！',
      onOk: async () => {
        try {
          await levelDeleteApi(id);
          message.success('删除成功');
          fetchList();
        } catch { message.error('删除失败'); }
      },
    });
  };

  // 状态开关切换
  const handleStatusChange = (record: any) => {
    const newIsShow = !record.isShow;
    if (record.isShow) {
      // 当前是开启状态，要关闭
      Modal.confirm({
        title: '提示',
        content: '该操作会导致对应用户等级隐藏，请谨慎操作',
        onOk: async () => {
          try {
            await levelUseApi({ id: record.id, isShow: newIsShow });
            message.success('修改成功');
            fetchList();
          } catch { message.error('修改失败'); }
        },
      });
    } else {
      // 当前是关闭状态，直接开启
      levelUseApi({ id: record.id, isShow: newIsShow })
        .then(() => {
          message.success('修改成功');
          fetchList();
        })
        .catch(() => { message.error('修改失败'); });
    }
  };

  const handleSubmit = async () => {
    try {
      const values = await form.validateFields();
      const data = {
        ...values,
        isShow: values.isShow ? true : false,
      };
      setSubmitLoading(true);
      if (editData?.id) {
        await levelUpdateApi(editData.id, data);
        message.success('编辑成功');
      } else {
        await levelSaveApi(data);
        message.success('添加成功');
      }
      setModalOpen(false);
      form.resetFields();
      setEditData(null);
      fetchList();
    } catch (e: any) {
      if (e?.errorFields) return; // 表单验证错误
      message.error('操作失败');
    } finally {
      setSubmitLoading(false);
    }
  };

  const columns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 60 },
    {
      title: '等级图标', dataIndex: 'icon', width: 100,
      render: (v: string) => v ? (
        <Image src={v} width={36} height={36} style={{ objectFit: 'cover' }} />
      ) : '-',
    },
    { title: '等级名称', dataIndex: 'name', width: 150 },
    { title: '经验', dataIndex: 'experience', width: 100 },
    { title: '享受折扣(%)', dataIndex: 'discount', width: 120 },
    {
      title: '状态', dataIndex: 'isShow', width: 120,
      render: (_: any, record: any) => (
        <Switch
          checked={record.isShow === 1 || record.isShow === true}
          checkedChildren="开启"
          unCheckedChildren="关闭"
          onChange={() => handleStatusChange(record)}
        />
      ),
    },
    {
      title: '操作', width: 120, fixed: 'right' as const,
      render: (_: any, record: any) => (
        <Space size={0} split={<span style={{ color: '#dcdfe6', margin: '0 6px' }}>|</span>}>
          <a onClick={() => handleEdit(record)}>编辑</a>
          <a style={{ color: '#ff4d4f' }} onClick={() => handleDelete(record.id)}>删除</a>
        </Space>
      ),
    },
  ];

  return (
    <Card
      title="用户等级"
      extra={
        <Button type="primary" icon={<PlusOutlined />} onClick={handleAdd}>
          添加用户等级
        </Button>
      }
    >
      <Table
        rowKey="id"
        columns={columns}
        dataSource={list}
        loading={loading}
        size="small"
        pagination={false}
      />

      <Modal
        title={editData ? '编辑用户等级' : '添加用户等级'}
        open={modalOpen}
        onCancel={() => setModalOpen(false)}
        onOk={handleSubmit}
        confirmLoading={submitLoading}
        destroyOnClose
        width={540}
      >
        <Form form={form} labelCol={{ span: 6 }} wrapperCol={{ span: 16 }}>
          <Form.Item name="name" label="等级名称" rules={[{ required: true, message: '请输入等级名称' }]}>
            <Input placeholder="请输入等级名称" />
          </Form.Item>
          <Form.Item name="grade" label="等级" rules={[{ required: true, message: '请输入等级' }]}>
            <InputNumber min={0} placeholder="请输入等级" style={{ width: '100%' }} />
          </Form.Item>
          <Form.Item name="discount" label="享受折扣(%)" rules={[{ required: true, message: '请输入折扣' }]}>
            <InputNumber min={0} max={100} placeholder="请输入享受折扣" style={{ width: '100%' }} />
          </Form.Item>
          <Form.Item name="experience" label="经验" rules={[{ required: true, message: '请输入经验' }]}>
            <InputNumber min={0} placeholder="请输入经验" style={{ width: '100%' }} />
          </Form.Item>
          <Form.Item name="icon" label="图标" rules={[{ required: true, message: '请上传图标' }]}>
            <UploadPicture limit={1} />
          </Form.Item>
        </Form>
      </Modal>
    </Card>
  );
};

export default UserGrade;
