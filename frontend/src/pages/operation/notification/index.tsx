import React, { useState, useEffect, useCallback } from 'react';
import { Card, Table, Tabs, Button, Switch, Modal, Form, Input, Radio, Space, message } from 'antd';
import type { ColumnsType } from 'antd/es/table';
import {
  notificationListApi,
  notificationDetail,
  notificationUpdate,
  notificationWechat,
  notificationRoutine,
  notificationSms,
} from '@/api/systemFormConfig';
import { wechatAsyncApi, routineAsyncApi } from '@/api/wxApi';

const OperationNotification: React.FC = () => {
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [currentTab, setCurrentTab] = useState('1');

  // detail modal
  const [detailModal, setDetailModal] = useState(false);
  const [detailRecord, setDetailRecord] = useState<any>(null);
  const [detailTabKey, setDetailTabKey] = useState('');
  const [detailTabs, setDetailTabs] = useState<{ key: string; label: string }[]>([]);
  const [detailData, setDetailData] = useState<any>(null);
  const [detailLoading, setDetailLoading] = useState(false);
  const [editForm] = Form.useForm();
  const [editLoading, setEditLoading] = useState(false);
  const [syncLoading, setSyncLoading] = useState<string>('');

  const fetchList = useCallback(async (sendType: string) => {
    setLoading(true);
    try {
      const res: any = await notificationListApi({ sendType });
      setList(res || []);
    } catch { /* handled */ }
    finally { setLoading(false); }
  }, []);

  useEffect(() => { fetchList(currentTab); }, [currentTab, fetchList]);

  const handleTabChange = (key: string) => {
    setCurrentTab(key);
  };

  const handleWechatSwitch = async (id: number) => {
    try { await notificationWechat(id); message.success('已切换'); fetchList(currentTab); } catch { /* */ }
  };
  const handleRoutineSwitch = async (id: number) => {
    try { await notificationRoutine(id); message.success('已切换'); fetchList(currentTab); } catch { /* */ }
  };
  const handleSmsSwitch = async (id: number) => {
    try { await notificationSms(id); message.success('已切换'); fetchList(currentTab); } catch { /* */ }
  };

  const handleSyncRoutine = async () => {
    setSyncLoading('routine');
    try { await routineAsyncApi(); message.success('同步成功'); fetchList(currentTab); }
    catch { /* */ }
    finally { setSyncLoading(''); }
  };
  const handleSyncWechat = async () => {
    setSyncLoading('wechat');
    try { await wechatAsyncApi(); message.success('同步成功'); fetchList(currentTab); }
    catch { /* */ }
    finally { setSyncLoading(''); }
  };

  // Open detail modal: build tabs based on record's enabled channels
  const openDetail = async (record: any) => {
    setDetailRecord(record);
    const tabs: { key: string; label: string }[] = [];
    if (record.isWechat !== 0) tabs.push({ key: 'wechat', label: '公众号模板消息' });
    if (record.isRoutine !== 0) tabs.push({ key: 'routine', label: '小程序订阅消息' });
    tabs.push({ key: 'sms', label: '短信' });
    setDetailTabs(tabs);
    setDetailModal(true);
    // Load first tab
    const firstKey = tabs[0].key;
    setDetailTabKey(firstKey);
    loadDetail(record.id, firstKey);
  };

  const loadDetail = async (id: number, type: string) => {
    setDetailLoading(true);
    setDetailData(null);
    editForm.resetFields();
    try {
      const res: any = await notificationDetail({ id, type });
      setDetailData(res);
      editForm.setFieldsValue({
        tempId: res?.tempId || '',
        status: res?.status === 1 ? '1' : '2',
      });
    } catch { message.error('获取详情失败'); }
    finally { setDetailLoading(false); }
  };

  const handleDetailTabChange = (key: string) => {
    setDetailTabKey(key);
    if (detailRecord) loadDetail(detailRecord.id, key);
  };

  const handleDetailSave = async () => {
    const values = await editForm.validateFields();
    if (!detailRecord) return;
    setEditLoading(true);
    try {
      await notificationUpdate({
        id: detailRecord.id,
        type: detailTabKey,
        tempId: values.tempId,
        status: Number(values.status),
      });
      message.success('保存成功');
      setDetailModal(false);
      fetchList(currentTab);
    } catch { /* */ }
    finally { setEditLoading(false); }
  };

  const columns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 60 },
    { title: '通知类型', dataIndex: 'type', width: 120 },
    { title: '通知场景说明', dataIndex: 'description', ellipsis: true },
    { title: '标识', dataIndex: 'mark', width: 180, ellipsis: true },
    ...(currentTab === '1'
      ? [
          {
            title: '公众号模板', dataIndex: 'isWechat', width: 110,
            render: (v: number, r: any) =>
              v !== 0 ? <Switch size="small" checked={v === 1} onChange={() => handleWechatSwitch(r.id)} /> : '-',
          } as any,
          {
            title: '小程序订阅', dataIndex: 'isRoutine', width: 110,
            render: (v: number, r: any) =>
              v !== 0 ? <Switch size="small" checked={v === 1} onChange={() => handleRoutineSwitch(r.id)} /> : '-',
          } as any,
        ]
      : []),
    {
      title: '发送短信', dataIndex: 'isSms', width: 100,
      render: (v: number, r: any) =>
        v !== 0 ? <Switch size="small" checked={v === 1} onChange={() => handleSmsSwitch(r.id)} /> : '-',
    },
    {
      title: '设置', width: 80, fixed: 'right',
      render: (_: any, record: any) => <a onClick={() => openDetail(record)}>详情</a>,
    },
  ];

  return (
    <Card>
      <Tabs activeKey={currentTab} onChange={handleTabChange} items={[
        { key: '1', label: '通知会员' },
        { key: '2', label: '通知平台' },
      ]} />
      <Space style={{ marginBottom: 16 }}>
        <Button loading={syncLoading === 'routine'} onClick={handleSyncRoutine}>同步小程序订阅消息</Button>
        <Button loading={syncLoading === 'wechat'} onClick={handleSyncWechat}>同步微信模版消息</Button>
      </Space>
      <Table
        rowKey="id"
        columns={columns}
        dataSource={list}
        loading={loading}
        size="small"
        scroll={{ x: 1000 }}
        pagination={false}
      />
      <Modal
        title="详情"
        open={detailModal}
        onCancel={() => setDetailModal(false)}
        onOk={handleDetailSave}
        confirmLoading={editLoading}
        destroyOnClose
        width={600}
      >
        <Tabs activeKey={detailTabKey} onChange={handleDetailTabChange}
          items={detailTabs.map((t) => ({ key: t.key, label: t.label }))}
        />
        {detailData && !detailLoading ? (
          <Form form={editForm} labelCol={{ span: 6 }} wrapperCol={{ span: 16 }}>
            <Form.Item label="ID"><Input disabled value={detailData.id} /></Form.Item>
            {detailData.name && <Form.Item label="模板名"><Input disabled value={detailData.name} /></Form.Item>}
            {detailData.tempId !== undefined && <Form.Item label="模板ID" name="tempId"><Input placeholder="请输入模板ID" /></Form.Item>}
            {detailData.tempKey && <Form.Item label="模板编号"><Input disabled value={detailData.tempKey} /></Form.Item>}
            {detailData.title && <Form.Item label="模板说明"><Input disabled value={detailData.title} /></Form.Item>}
            {detailData.content && <Form.Item label="模板内容"><Input disabled value={detailData.content} /></Form.Item>}
            <Form.Item label="状态" name="status">
              <Radio.Group>
                <Radio value="1">开启</Radio>
                <Radio value="2">关闭</Radio>
              </Radio.Group>
            </Form.Item>
          </Form>
        ) : null}
      </Modal>
    </Card>
  );
};

export default OperationNotification;
