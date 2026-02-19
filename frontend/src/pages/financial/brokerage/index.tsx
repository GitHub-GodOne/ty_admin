import React, { useState, useEffect, useCallback } from 'react';
import { Card, Table, Form, Input, Select, Button, Tag, message } from 'antd';
import { SearchOutlined, ReloadOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { brokerageListApi } from '@/api/financial';

const typeMap: Record<number, { text: string; color: string }> = {
  1: { text: '订单返佣', color: 'green' },
  2: { text: '申请提现', color: 'blue' },
  3: { text: '提现失败', color: 'red' },
  4: { text: '提现成功', color: 'cyan' },
  5: { text: '佣金转余额', color: 'orange' },
};

const dateOptions = [
  { label: '今天', value: 'today' },
  { label: '昨天', value: 'yesterday' },
  { label: '最近7天', value: 'lately7' },
  { label: '最近30天', value: 'lately30' },
  { label: '本月', value: 'month' },
  { label: '本年', value: 'year' },
];

const Brokerage: React.FC = () => {
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [recordType, setRecordType] = useState<number | undefined>(undefined);
  const [keywords, setKeywords] = useState('');
  const [dateLimit, setDateLimit] = useState<string | undefined>(undefined);
  const [pagination, setPagination] = useState({ current: 1, pageSize: 20, total: 0 });

  const fetchList = useCallback(async (page = 1) => {
    setLoading(true);
    try {
      const params: any = { page, limit: pagination.pageSize };
      if (keywords) params.keywords = keywords;
      if (recordType !== undefined) params.recordType = recordType;
      if (dateLimit) params.dateLimit = dateLimit;
      const res = await brokerageListApi(params);
      setList(res?.list || []);
      setPagination((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch { message.error('获取佣金记录失败'); }
    finally { setLoading(false); }
  }, [pagination.pageSize, keywords, recordType, dateLimit]);

  useEffect(() => { fetchList(1); }, []);

  const handleSearch = () => { fetchList(1); };
  const handleReset = () => { setKeywords(''); setRecordType(undefined); setDateLimit(undefined); };

  const columns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 60 },
    { title: '用户昵称', dataIndex: 'userName', width: 120, ellipsis: true },
    { title: 'UID', dataIndex: 'uid', width: 80 },
    { title: '变动金额', dataIndex: 'price', width: 110,
      render: (v: any, r: any) => <span style={{ color: r.type === 1 ? '#52c41a' : '#ff4d4f' }}>{r.type === 1 ? '+' : '-'}¥{v || 0}</span> },
    { title: '变动后余额', dataIndex: 'balance', width: 110, render: (v: any) => `¥${v || 0}` },
    {
      title: '变动类型', dataIndex: 'recordType', width: 120,
      render: (v: number) => {
        const t = typeMap[v] || { text: '未知', color: 'default' };
        return <Tag color={t.color}>{t.text}</Tag>;
      },
    },
    { title: '变动信息', dataIndex: 'mark', ellipsis: true, width: 220 },
    { title: '创建时间', dataIndex: 'createTime', width: 160 },
    { title: '变动时间', dataIndex: 'updateTime', width: 160 },
  ];

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
      <Card bodyStyle={{ paddingBottom: 0 }}>
        <Form layout="inline" style={{ marginBottom: 16 }}>
          <Form.Item><Input placeholder="用户昵称/UID" value={keywords} onChange={(e) => setKeywords(e.target.value)} style={{ width: 180 }} allowClear /></Form.Item>
          <Form.Item>
            <Select value={recordType} onChange={(v) => setRecordType(v)} placeholder="变动类型" allowClear style={{ width: 160 }}>
              <Select.Option value={1}>订单返佣</Select.Option>
              <Select.Option value={2}>申请提现</Select.Option>
              <Select.Option value={3}>提现失败</Select.Option>
              <Select.Option value={4}>提现成功</Select.Option>
              <Select.Option value={5}>佣金转余额</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item><Select placeholder="选择时间" allowClear style={{ width: 140 }} options={dateOptions} value={dateLimit} onChange={(v) => setDateLimit(v)} /></Form.Item>
          <Form.Item><Button type="primary" icon={<SearchOutlined />} onClick={handleSearch}>搜索</Button></Form.Item>
          <Form.Item><Button icon={<ReloadOutlined />} onClick={handleReset}>重置</Button></Form.Item>
        </Form>
      </Card>
      <Card title="佣金变动">
        <Table rowKey="id" columns={columns} dataSource={list} loading={loading} size="small"
          scroll={{ x: 1200 }}
          pagination={{ ...pagination, showSizeChanger: true, pageSizeOptions: ['10', '20', '30', '40'],
            showTotal: (t: number) => `共 ${t} 条`,
            onChange: (p: number, ps: number) => { setPagination((prev) => ({ ...prev, pageSize: ps })); fetchList(p); } }} />
      </Card>
    </div>
  );
};

export default Brokerage;
