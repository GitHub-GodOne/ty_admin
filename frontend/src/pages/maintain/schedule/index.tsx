import React, { useState, useEffect, useCallback } from 'react';
import { Card, Table, Form, Button, Space, Tag, Modal, Input, message, Popconfirm, Tabs } from 'antd';
import { PlusOutlined, PlayCircleOutlined, PauseCircleOutlined, ThunderboltOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { jobList, jobLogList, scheduleJobAdd, scheduleJobDelete, scheduleJobStart, scheduleJobSuspend, scheduleJobTrig, scheduleJobUpdate } from '@/api/schedule';

const ScheduleList: React.FC = () => {
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [logList, setLogList] = useState<any[]>([]);
  const [logLoading, setLogLoading] = useState(false);
  const [logPagination, setLogPagination] = useState({ current: 1, pageSize: 20, total: 0 });
  const [activeTab, setActiveTab] = useState('job');
  const [modalOpen, setModalOpen] = useState(false);
  const [editData, setEditData] = useState<any>(null);
  const [form] = Form.useForm();

  const fetchList = async () => {
    setLoading(true);
    try {
      const res = await jobList();
      setList(Array.isArray(res) ? res : res?.list || []);
    } catch {
      message.error('获取定时任务列表失败');
    } finally {
      setLoading(false);
    }
  };

  const fetchLogList = useCallback(async (page = 1) => {
    setLogLoading(true);
    try {
      const res = await jobLogList({ page, limit: logPagination.pageSize });
      setLogList(res?.list || []);
      setLogPagination((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch {
      message.error('获取任务日志失败');
    } finally {
      setLogLoading(false);
    }
  }, [logPagination.pageSize]);

  useEffect(() => { fetchList(); }, []);
  useEffect(() => { if (activeTab === 'log') fetchLogList(1); }, [activeTab]);

  const handleDelete = async (id: number) => {
    try {
      await scheduleJobDelete(id);
      message.success('删除成功');
      fetchList();
    } catch { /* noop */ }
  };

  const handleStart = async (id: number) => {
    try {
      await scheduleJobStart(id);
      message.success('启动成功');
      fetchList();
    } catch { /* noop */ }
  };

  const handleSuspend = async (id: number) => {
    try {
      await scheduleJobSuspend(id);
      message.success('暂停成功');
      fetchList();
    } catch { /* noop */ }
  };

  const handleTrig = async (id: number) => {
    try {
      await scheduleJobTrig(id);
      message.success('执行成功');
      fetchList();
    } catch { /* noop */ }
  };

  const handleAdd = () => {
    setEditData(null);
    form.resetFields();
    setModalOpen(true);
  };

  const handleEdit = (record: any) => {
    setEditData(record);
    form.setFieldsValue(record);
    setModalOpen(true);
  };

  const handleSave = async () => {
    try {
      const values = await form.validateFields();
      if (editData) {
        await scheduleJobUpdate({ ...values, jobId: editData.jobId });
      } else {
        await scheduleJobAdd(values);
      }
      message.success('操作成功');
      setModalOpen(false);
      fetchList();
    } catch { /* noop */ }
  };

  const jobColumns: ColumnsType<any> = [
    { title: '任务ID', dataIndex: 'jobId', width: 80 },
    { title: 'Bean名称', dataIndex: 'beanName', width: 150 },
    { title: '方法名称', dataIndex: 'methodName', width: 150 },
    { title: '参数', dataIndex: 'params', ellipsis: true, width: 150 },
    { title: 'Cron表达式', dataIndex: 'cronExpression', width: 150 },
    { title: '备注', dataIndex: 'remark', ellipsis: true, width: 150 },
    {
      title: '状态', dataIndex: 'status', width: 80,
      render: (v: number) => v === 1
        ? <Tag color="green">运行中</Tag>
        : <Tag color="default">已暂停</Tag>,
    },
    {
      title: '操作', width: 250, fixed: 'right',
      render: (_: any, record: any) => (
        <Space size="small">
          <a onClick={() => handleEdit(record)}>编辑</a>
          {record.status === 1
            ? <a onClick={() => handleSuspend(record.jobId)}><PauseCircleOutlined /> 暂停</a>
            : <a onClick={() => handleStart(record.jobId)}><PlayCircleOutlined /> 启动</a>
          }
          <a onClick={() => handleTrig(record.jobId)}><ThunderboltOutlined /> 执行</a>
          <Popconfirm title="确定删除?" onConfirm={() => handleDelete(record.jobId)}>
            <a style={{ color: '#ff4d4f' }}>删除</a>
          </Popconfirm>
        </Space>
      ),
    },
  ];

  const logColumns: ColumnsType<any> = [
    { title: '日志ID', dataIndex: 'logId', width: 80 },
    { title: '任务ID', dataIndex: 'jobId', width: 80 },
    { title: 'Bean名称', dataIndex: 'beanName', width: 150 },
    { title: '方法名称', dataIndex: 'methodName', width: 150 },
    { title: '参数', dataIndex: 'params', ellipsis: true, width: 150 },
    {
      title: '状态', dataIndex: 'status', width: 80,
      render: (v: number) => v === 0
        ? <Tag color="green">成功</Tag>
        : <Tag color="red">失败</Tag>,
    },
    { title: '异常信息', dataIndex: 'error', ellipsis: true, width: 200 },
    { title: '耗时(ms)', dataIndex: 'times', width: 100 },
    { title: '创建时间', dataIndex: 'createTime', width: 170 },
  ];

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
      <Card>
        <Tabs activeKey={activeTab} onChange={setActiveTab}
          items={[
            { key: 'job', label: '定时任务' },
            { key: 'log', label: '任务日志' },
          ]} />

        {activeTab === 'job' && (
          <>
            <div style={{ display: 'flex', justifyContent: 'flex-end', marginBottom: 16 }}>
              <Button type="primary" icon={<PlusOutlined />} onClick={handleAdd}>添加任务</Button>
            </div>
            <Table rowKey="jobId" columns={jobColumns} dataSource={list} loading={loading}
              size="small" scroll={{ x: 1100 }} pagination={false} />
          </>
        )}

        {activeTab === 'log' && (
          <Table rowKey="logId" columns={logColumns} dataSource={logList} loading={logLoading}
            size="small" scroll={{ x: 1100 }}
            pagination={{ ...logPagination, showSizeChanger: true, pageSizeOptions: ['10', '20', '30', '40'],
              showTotal: (t: number) => `共 ${t} 条`,
              onChange: (p: number, ps: number) => { setLogPagination((prev) => ({ ...prev, pageSize: ps })); fetchLogList(p); } }} />
        )}
      </Card>

      <Modal title={editData ? '编辑任务' : '添加任务'} open={modalOpen} onOk={handleSave}
        onCancel={() => setModalOpen(false)} width={600} destroyOnClose>
        <Form form={form} labelCol={{ span: 6 }} wrapperCol={{ span: 16 }} preserve={false}>
          <Form.Item label="Bean名称" name="beanName" rules={[{ required: true, message: '请输入Bean名称' }]}>
            <Input placeholder="请输入Bean名称" />
          </Form.Item>
          <Form.Item label="方法名称" name="methodName" rules={[{ required: true, message: '请输入方法名称' }]}>
            <Input placeholder="请输入方法名称" />
          </Form.Item>
          <Form.Item label="Cron表达式" name="cronExpression" rules={[{ required: true, message: '请输入Cron表达式' }]}>
            <Input placeholder="请输入Cron表达式" />
          </Form.Item>
          <Form.Item label="参数" name="params">
            <Input placeholder="请输入参数" />
          </Form.Item>
          <Form.Item label="备注" name="remark">
            <Input.TextArea placeholder="请输入备注" rows={3} />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
};

export default ScheduleList;
