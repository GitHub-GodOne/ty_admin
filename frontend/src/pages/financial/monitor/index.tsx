import React, { useState, useEffect, useCallback } from 'react';
import { Card, Table, Form, Input, Select, Button, message } from 'antd';
import { SearchOutlined, ReloadOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { monitorListApi } from '@/api/financial';

const titleOptions = [
  { label: '充值支付', value: 'recharge' },
  { label: '后台操作', value: 'admin' },
  { label: '商品退款', value: 'productRefund' },
  { label: '购买商品', value: 'payProduct' },
];

const dateOptions = [
  { label: '今天', value: 'today' },
  { label: '昨天', value: 'yesterday' },
  { label: '最近7天', value: 'lately7' },
  { label: '最近30天', value: 'lately30' },
  { label: '本月', value: 'month' },
  { label: '本年', value: 'year' },
];

const FinancialMonitor: React.FC = () => {
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [pagination, setPagination] = useState({ current: 1, pageSize: 20, total: 0 });
  const [keywords, setKeywords] = useState('');
  const [title, setTitle] = useState<string | undefined>(undefined);
  const [dateLimit, setDateLimit] = useState<string | undefined>(undefined);

  const fetchList = useCallback(async (page = 1) => {
    setLoading(true);
    try {
      const params: any = { page, limit: pagination.pageSize };
      if (keywords) params.keywords = keywords;
      if (title) params.title = title;
      if (dateLimit) params.dateLimit = dateLimit;
      const res = await monitorListApi(params);
      setList(res?.list || []);
      setPagination((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch { message.error('获取资金监控列表失败'); }
    finally { setLoading(false); }
  }, [pagination.pageSize, keywords, title, dateLimit]);

  useEffect(() => { fetchList(1); }, []);

  const handleSearch = () => { fetchList(1); };
  const handleReset = () => { setKeywords(''); setTitle(undefined); setDateLimit(undefined); };

  const columns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 60 },
    { title: '用户昵称', dataIndex: 'nickName', width: 120, ellipsis: true },
    { title: 'UID', dataIndex: 'uid', width: 80 },
    { title: '变动金额', dataIndex: 'number', width: 110,
      render: (v: any, r: any) => <span style={{ color: r.pm === 1 ? '#52c41a' : '#ff4d4f' }}>{r.pm === 1 ? '+' : '-'}¥{v || 0}</span> },
    { title: '类型', dataIndex: 'title', width: 120 },
    { title: '备注', dataIndex: 'mark', width: 200, ellipsis: true, render: (v: string) => v || '-' },
    { title: '时间', dataIndex: 'createTime', width: 160 },
  ];

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
      <Card bodyStyle={{ paddingBottom: 0 }}>
        <Form layout="inline" style={{ marginBottom: 16 }}>
          <Form.Item><Input placeholder="UID/用户昵称" value={keywords} onChange={(e) => setKeywords(e.target.value)} style={{ width: 180 }} allowClear /></Form.Item>
          <Form.Item><Select placeholder="明细类型" allowClear style={{ width: 160 }} options={titleOptions} value={title} onChange={(v) => setTitle(v)} /></Form.Item>
          <Form.Item><Select placeholder="选择时间" allowClear style={{ width: 140 }} options={dateOptions} value={dateLimit} onChange={(v) => setDateLimit(v)} /></Form.Item>
          <Form.Item><Button type="primary" icon={<SearchOutlined />} onClick={handleSearch}>搜索</Button></Form.Item>
          <Form.Item><Button icon={<ReloadOutlined />} onClick={handleReset}>重置</Button></Form.Item>
        </Form>
      </Card>
      <Card title="资金监控">
        <Table rowKey="id" columns={columns} dataSource={list} loading={loading} size="small" scroll={{ x: 800 }}
          pagination={{ ...pagination, showSizeChanger: true, showTotal: (t: number) => `共 ${t} 条`,
            onChange: (p: number, ps: number) => { setPagination((prev) => ({ ...prev, pageSize: ps })); fetchList(p); } }} />
      </Card>
    </div>
  );
};

export default FinancialMonitor;
