import React, { useState, useEffect, useCallback } from 'react';
import { Card, Table, Form, Input, Select, Button, Row, Col, Statistic, message } from 'antd';
import { SearchOutlined, ReloadOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { topUpLogListApi, balanceApi } from '@/api/financial';
import { payTypeFilter } from '@/utils/formatters';

const dateOptions = [
  { label: '今天', value: 'today' },
  { label: '昨天', value: 'yesterday' },
  { label: '最近7天', value: 'lately7' },
  { label: '最近30天', value: 'lately30' },
  { label: '本月', value: 'month' },
  { label: '本年', value: 'year' },
];

const FinancialRecharge: React.FC = () => {
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [pagination, setPagination] = useState({ current: 1, pageSize: 20, total: 0 });
  const [keywords, setKeywords] = useState('');
  const [dateLimit, setDateLimit] = useState<string | undefined>(undefined);
  const [balanceData, setBalanceData] = useState<any>({});

  const fetchBalance = useCallback(async () => {
    try {
      const res = await balanceApi();
      setBalanceData(res || {});
    } catch { /* noop */ }
  }, []);

  const fetchList = useCallback(async (page = 1) => {
    setLoading(true);
    try {
      const params: any = { page, limit: pagination.pageSize };
      if (keywords) params.keywords = keywords;
      if (dateLimit) params.dateLimit = dateLimit;
      const res = await topUpLogListApi(params);
      setList(res?.list || []);
      setPagination((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch { message.error('获取充值记录失败'); }
    finally { setLoading(false); }
  }, [pagination.pageSize, keywords, dateLimit]);

  useEffect(() => { fetchList(1); fetchBalance(); }, []);

  const handleSearch = () => { fetchList(1); };
  const handleReset = () => { setKeywords(''); setDateLimit(undefined); };

  const columns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 60 },
    { title: '订单号', dataIndex: 'orderId', width: 200, ellipsis: true },
    { title: '用户昵称', dataIndex: 'nickname', width: 120, ellipsis: true },
    { title: '充值金额', dataIndex: 'price', width: 100, render: (v: any) => `¥${v || 0}` },
    { title: '赠送金额', dataIndex: 'givePrice', width: 100, render: (v: any) => `¥${v || 0}` },
    { title: '退款金额', dataIndex: 'refundPrice', width: 100, render: (v: any) => v ? `¥${v}` : '-' },
    { title: '支付方式', dataIndex: 'rechargeType', width: 100, render: (v: string) => payTypeFilter(v) },
    { title: '充值时间', dataIndex: 'createTime', width: 160 },
  ];

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
      <Row gutter={16}>
        <Col span={5}><Card><Statistic title="小程序充值" value={balanceData.routine || 0} prefix="¥" precision={2} /></Card></Col>
        <Col span={5}><Card><Statistic title="公众号充值" value={balanceData.weChat || 0} prefix="¥" precision={2} /></Card></Col>
        <Col span={5}><Card><Statistic title="总金额" value={balanceData.total || 0} prefix="¥" precision={2} /></Card></Col>
        <Col span={5}><Card><Statistic title="退款金额" value={balanceData.refund || 0} prefix="¥" precision={2} /></Card></Col>
        <Col span={4}><Card><Statistic title="其他充值" value={balanceData.other || 0} prefix="¥" precision={2} /></Card></Col>
      </Row>
      <Card bodyStyle={{ paddingBottom: 0 }}>
        <Form layout="inline" style={{ marginBottom: 16 }}>
          <Form.Item><Input placeholder="请输入订单号" value={keywords} onChange={(e) => setKeywords(e.target.value)} style={{ width: 200 }} allowClear /></Form.Item>
          <Form.Item><Select placeholder="选择时间" allowClear style={{ width: 140 }} options={dateOptions} value={dateLimit} onChange={(v) => setDateLimit(v)} /></Form.Item>
          <Form.Item><Button type="primary" icon={<SearchOutlined />} onClick={handleSearch}>搜索</Button></Form.Item>
          <Form.Item><Button icon={<ReloadOutlined />} onClick={handleReset}>重置</Button></Form.Item>
        </Form>
      </Card>
      <Card title="充值记录">
        <Table rowKey="id" columns={columns} dataSource={list} loading={loading} size="small" scroll={{ x: 900 }}
          pagination={{ ...pagination, showSizeChanger: true, showTotal: (t: number) => `共 ${t} 条`,
            onChange: (p: number, ps: number) => { setPagination((prev) => ({ ...prev, pageSize: ps })); fetchList(p); } }} />
      </Card>
    </div>
  );
};

export default FinancialRecharge;
