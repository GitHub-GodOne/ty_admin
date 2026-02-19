import React, { useState, useEffect, useCallback } from 'react';
import { Card, Table, Tabs, Button, Space, Tag, message, Modal, Input, Form, Select, Row, Col, Statistic, DatePicker } from 'antd';
import { SearchOutlined, ReloadOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { applyListApi, applyStatusApi, applyBalanceApi, applyUpdateApi } from '@/api/financial';
import { extractTypeFilter } from '@/utils/formatters';
import dayjs from 'dayjs';

const { RangePicker } = DatePicker;

const statusTabs = [
  { key: '', label: '全部' },
  { key: '0', label: '审核中' },
  { key: '1', label: '已提现' },
  { key: '-1', label: '已拒绝' },
];

const statusMap: Record<string, { text: string; color: string }> = {
  '0': { text: '审核中', color: 'processing' },
  '1': { text: '已提现', color: 'success' },
  '-1': { text: '已拒绝', color: 'error' },
};

const dateOptions = [
  { label: '今天', value: 'today' },
  { label: '昨天', value: 'yesterday' },
  { label: '最近7天', value: 'lately7' },
  { label: '最近30天', value: 'lately30' },
  { label: '本月', value: 'month' },
  { label: '本年', value: 'year' },
];

const FinancialExtract: React.FC = () => {
  const [activeTab, setActiveTab] = useState('');
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [pagination, setPagination] = useState({ current: 1, pageSize: 20, total: 0 });
  const [keywords, setKeywords] = useState('');
  const [extractType, setExtractType] = useState<string | undefined>(undefined);
  const [dateLimit, setDateLimit] = useState<string | undefined>(undefined);
  const [balance, setBalance] = useState<any>({});
  const [refuseModal, setRefuseModal] = useState<{ open: boolean; id: number | null }>({ open: false, id: null });
  const [refuseReason, setRefuseReason] = useState('');
  const [editModal, setEditModal] = useState(false);
  const [editForm] = Form.useForm();
  const [editId, setEditId] = useState<number | null>(null);

  const fetchBalance = useCallback(async () => {
    try {
      const res = await applyBalanceApi({ dateLimit: dateLimit || '' });
      setBalance(res || {});
    } catch { /* noop */ }
  }, [dateLimit]);

  const fetchList = useCallback(async (page = 1) => {
    setLoading(true);
    try {
      const params: any = { page, limit: pagination.pageSize };
      if (activeTab !== '') params.status = Number(activeTab);
      if (keywords) params.keywords = keywords;
      if (extractType) params.extractType = extractType;
      if (dateLimit) params.dateLimit = dateLimit;
      const res = await applyListApi(params);
      setList(res?.list || []);
      setPagination((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch { message.error('获取提现列表失败'); }
    finally { setLoading(false); }
  }, [activeTab, pagination.pageSize, keywords, extractType, dateLimit]);

  useEffect(() => { fetchList(1); fetchBalance(); }, [activeTab]);

  const handleSearch = () => { fetchList(1); fetchBalance(); };
  const handleReset = () => { setKeywords(''); setExtractType(undefined); setDateLimit(undefined); };

  const handleAgree = async (id: number) => {
    try {
      await applyStatusApi({ id }, { status: 1 });
      message.success('已通过');
      fetchList(pagination.current);
      fetchBalance();
    } catch { /* noop */ }
  };

  const handleRefuse = async () => {
    if (!refuseModal.id) return;
    if (!refuseReason) { message.warning('请输入拒绝原因'); return; }
    try {
      await applyStatusApi({ id: refuseModal.id }, { status: -1, backMessage: refuseReason });
      message.success('已拒绝');
      setRefuseModal({ open: false, id: null });
      setRefuseReason('');
      fetchList(pagination.current);
      fetchBalance();
    } catch { /* noop */ }
  };

  const handleEdit = (record: any) => {
    setEditId(record.id);
    editForm.setFieldsValue({
      realName: record.realName,
      extractType: record.extractType,
      bankCode: record.bankCode,
      bankName: record.bankName,
      alipayCode: record.alipayCode,
      wechat: record.wechat,
      extractPrice: record.extractPrice,
      mark: record.mark,
      qrcodeUrl: record.qrcodeUrl,
    });
    setEditModal(true);
  };

  const handleEditSubmit = async () => {
    const values = await editForm.validateFields();
    try {
      await applyUpdateApi({ id: editId, ...values });
      message.success('修改成功');
      setEditModal(false);
      editForm.resetFields();
      setEditId(null);
      fetchList(pagination.current);
    } catch { message.error('修改失败'); }
  };

  const columns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 60 },
    { title: '用户昵称', dataIndex: 'nickname', width: 120, ellipsis: true },
    { title: '提现金额', dataIndex: 'extractPrice', width: 100, render: (v: any) => `¥${v || 0}` },
    { title: '提现方式', dataIndex: 'extractType', width: 100, render: (v: string) => extractTypeFilter(v) },
    {
      title: '账号信息', width: 180, ellipsis: true,
      render: (_: any, r: any) => {
        if (r.extractType === 'bank') return `${r.bankName || ''} ${r.bankCode || ''}`;
        if (r.extractType === 'alipay') return r.alipayCode || '-';
        if (r.extractType === 'weixin') return r.wechat || '-';
        return '-';
      },
    },
    { title: '备注', dataIndex: 'mark', width: 140, ellipsis: true, render: (v: string) => v || '-' },
    { title: '失败原因', dataIndex: 'failMsg', width: 140, ellipsis: true, render: (v: string) => v || '-' },
    { title: '申请时间', dataIndex: 'createTime', width: 160 },
    {
      title: '状态', dataIndex: 'status', width: 90,
      render: (v: number) => {
        const s = statusMap[String(v)] || { text: '未知', color: 'default' };
        return <Tag color={s.color}>{s.text}</Tag>;
      },
    },
    {
      title: '操作', width: 160, fixed: 'right',
      render: (_: any, record: any) => record.status === 0 ? (
        <Space size="small">
          <a onClick={() => handleAgree(record.id)}>通过</a>
          <a style={{ color: '#ff4d4f' }} onClick={() => { setRefuseModal({ open: true, id: record.id }); setRefuseReason(''); }}>拒绝</a>
          <a onClick={() => handleEdit(record)}>编辑</a>
        </Space>
      ) : '-',
    },
  ];

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
      <Row gutter={16}>
        <Col span={6}><Card><Statistic title="已提现金额" value={balance.withdrawn || 0} prefix="¥" precision={2} /></Card></Col>
        <Col span={6}><Card><Statistic title="待提现金额(审核中)" value={balance.toBeWithdrawn || 0} prefix="¥" precision={2} /></Card></Col>
        <Col span={6}><Card><Statistic title="佣金总金额" value={balance.commissionTotal || 0} prefix="¥" precision={2} /></Card></Col>
        <Col span={6}><Card><Statistic title="未提现金额" value={balance.unDrawn || 0} prefix="¥" precision={2} /></Card></Col>
      </Row>
      <Card bodyStyle={{ paddingBottom: 0 }}>
        <Form layout="inline" style={{ marginBottom: 16 }}>
          <Form.Item><Input placeholder="姓名/银行卡/微信号/支付宝" value={keywords} onChange={(e) => setKeywords(e.target.value)} style={{ width: 200 }} allowClear /></Form.Item>
          <Form.Item>
            <Select placeholder="提现方式" allowClear style={{ width: 140 }} value={extractType} onChange={(v) => setExtractType(v)}>
              <Select.Option value="bank">银行卡</Select.Option>
              <Select.Option value="alipay">支付宝</Select.Option>
              <Select.Option value="weixin">微信</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item><Select placeholder="选择时间" allowClear style={{ width: 140 }} options={dateOptions} value={dateLimit} onChange={(v) => setDateLimit(v)} /></Form.Item>
          <Form.Item><Button type="primary" icon={<SearchOutlined />} onClick={handleSearch}>搜索</Button></Form.Item>
          <Form.Item><Button icon={<ReloadOutlined />} onClick={handleReset}>重置</Button></Form.Item>
        </Form>
      </Card>
      <Card>
        <Tabs activeKey={activeTab} onChange={(k) => setActiveTab(k)} items={statusTabs.map((t) => ({ key: t.key, label: t.label }))} />
        <Table rowKey="id" columns={columns} dataSource={list} loading={loading} size="small" scroll={{ x: 1100 }}
          pagination={{ ...pagination, showSizeChanger: true, showTotal: (t: number) => `共 ${t} 条`,
            onChange: (p: number, ps: number) => { setPagination((prev) => ({ ...prev, pageSize: ps })); fetchList(p); } }} />
      </Card>

      <Modal title="拒绝提现" open={refuseModal.open} onOk={handleRefuse}
        onCancel={() => setRefuseModal({ open: false, id: null })}>
        <Input.TextArea rows={4} placeholder="请输入拒绝原因" value={refuseReason}
          onChange={(e) => setRefuseReason(e.target.value)} />
      </Modal>

      <Modal title="编辑提现申请" open={editModal} onOk={handleEditSubmit}
        onCancel={() => { setEditModal(false); editForm.resetFields(); }} destroyOnClose width={600}>
        <Form form={editForm} layout="vertical">
          <Form.Item name="realName" label="真实姓名"><Input placeholder="请输入真实姓名" /></Form.Item>
          <Form.Item name="extractType" label="提现方式">
            <Select placeholder="请选择提现方式">
              <Select.Option value="bank">银行卡</Select.Option>
              <Select.Option value="alipay">支付宝</Select.Option>
              <Select.Option value="weixin">微信</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item name="bankName" label="开户行"><Input placeholder="请输入开户行" /></Form.Item>
          <Form.Item name="bankCode" label="银行卡号"><Input placeholder="请输入银行卡号" /></Form.Item>
          <Form.Item name="alipayCode" label="支付宝账号"><Input placeholder="请输入支付宝账号" /></Form.Item>
          <Form.Item name="wechat" label="微信号"><Input placeholder="请输入微信号" /></Form.Item>
          <Form.Item name="extractPrice" label="提现金额"><Input placeholder="请输入提现金额" /></Form.Item>
          <Form.Item name="mark" label="备注"><Input.TextArea rows={3} placeholder="请输入备注" /></Form.Item>
        </Form>
      </Modal>
    </div>
  );
};

export default FinancialExtract;
