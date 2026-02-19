import React, { useState, useEffect, useCallback } from 'react';
import { Card, Table, Form, Input, Select, Button, Tag, message } from 'antd';
import { SearchOutlined, ReloadOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { brokerageListApi } from '@/api/financial';

const typeOptions = [
  { label: '订单返佣', value: 1 },
  { label: '申请提现', value: 2 },
  { label: '提现失败', value: 3 },
  { label: '提现成功', value: 4 },
  { label: '佣金转余额', value: 5 },
];

const typeMap: Record<number, { text: string; color: string }> = {
  1: { text: '订单返佣', color: 'green' },
  2: { text: '申请提现', color: 'blue' },
  3: { text: '提现失败', color: 'red' },
  4: { text: '提现成功', color: 'cyan' },
  5: { text: '佣金转余额', color: 'orange' },
};

const FinancialCommission: React.FC = () => {
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [pagination, setPagination] = useState({ current: 1, pageSize: 20, total: 0 });
  const [keywords, setKeywords] = useState('');
  const [recordType, setRecordType] = useState<number | undefined>(undefined);

  const fetchList = useCallback(async (page = 1) => {
    setLoading(true);
    try {
      const params: any = { page, limit: pagination.pageSize };
      if (keywords) params.keywords = keywords;
      if (recordType !== undefined) params.recordType = recordType;
      const res = await brokerageListApi(params);
      setList(res?.list || []);
      setPagination((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch { message.error('获取佣金记录失败'); }
    finally { setLoading(false); }
  }, [pagination.pageSize, keywords, recordType]);

  useEffect(() => { fetchList(1); }, []);

  const handleSearch = () => { fetchList(1); };
  const handleReset = () => { setKeywords(''); setRecordType(undefined); };

  const columns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 60 },
    { title: '用户昵称', dataIndex: 'userName', width: 120, ellipsis: true },
    { title: 'UID', dataIndex: 'uid', width: 80 },
    { title: '佣金金额', dataIndex: 'price', width: 110,
      render: (v: any, r: any) => <span style={{ color: r.type === 1 ? '#52c41a' : '#ff4d4f' }}>{r.type === 1 ? '+' : '-'}¥{v || 0}</span> },
    { title: '变动后余额', dataIndex: 'balance', width: 110, render: (v: any) => `¥${v || 0}` },
    {
      title: '变动类型', dataIndex: 'recordType', width: 120,
      render: (v: number) => {
        const t = typeMap[v] || { text: '未知', color: 'default' };
        return <Tag color={t.color}>{t.text}</Tag>;
      },
    },
    { title: '变动信息', dataIndex: 'mark', width: 220, ellipsis: true, render: (v: string) => v || '-' },
    { title: '关联ID', dataIndex: 'linkId', width: 140, ellipsis: true, render: (v: any) => v || '-' },
    { title: '创建时间', dataIndex: 'createTime', width: 160 },
    { title: '变动时间', dataIndex: 'updateTime', width: 160 },
  ];

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
      <Card bodyStyle={{ paddingBottom: 0 }}>
        <Form layout="inline" style={{ marginBottom: 16 }}>
          <Form.Item><Input placeholder="用户昵称/UID" value={keywords} onChange={(e) => setKeywords(e.target.value)} style={{ width: 180 }} allowClear /></Form.Item>
          <Form.Item>
            <Select placeholder="变动类型" allowClear style={{ width: 160 }} options={typeOptions} value={recordType} onChange={(v) => setRecordType(v)} />
          </Form.Item>
          <Form.Item><Button type="primary" icon={<SearchOutlined />} onClick={handleSearch}>搜索</Button></Form.Item>
          <Form.Item><Button icon={<ReloadOutlined />} onClick={handleReset}>重置</Button></Form.Item>
        </Form>
      </Card>
      <Card title="佣金记录">
        <Table rowKey="id" columns={columns} dataSource={list} loading={loading} size="small" scroll={{ x: 1200 }}
          pagination={{ ...pagination, showSizeChanger: true, showTotal: (t: number) => `共 ${t} 条`,
            onChange: (p: number, ps: number) => { setPagination((prev) => ({ ...prev, pageSize: ps })); fetchList(p); } }} />
      </Card>
    </div>
  );
};

export default FinancialCommission;
