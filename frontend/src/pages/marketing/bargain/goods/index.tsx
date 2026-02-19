import React, { useState, useEffect, useCallback } from 'react';
import { Card, Table, Form, Input, Select, Button, Space, Image, Switch, message, Popconfirm } from 'antd';
import { PlusOutlined, SearchOutlined, ReloadOutlined, ExportOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { bargainListApi, bargainDeleteApi, bargainStatusApi, exportBargainApi } from '@/api/marketing';
import { useNavigate } from 'react-router-dom';

const BargainGoods: React.FC = () => {
  const navigate = useNavigate();
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [keywords, setKeywords] = useState('');
  const [status, setStatus] = useState<number | undefined>(undefined);
  const [pagination, setPagination] = useState({ current: 1, pageSize: 20, total: 0 });

  const fetchList = useCallback(async (page = 1) => {
    setLoading(true);
    try {
      const res = await bargainListApi({ page, limit: pagination.pageSize, keywords: keywords || undefined, status });
      setList(res?.list || []);
      setPagination((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch {
      message.error('获取砍价商品列表失败');
    } finally {
      setLoading(false);
    }
  }, [keywords, status, pagination.pageSize]);

  useEffect(() => { fetchList(1); }, []);

  const handleReset = () => {
    setKeywords('');
    setStatus(undefined);
  };

  const handleDelete = async (id: number) => {
    try {
      await bargainDeleteApi({ id });
      message.success('删除成功');
      fetchList(pagination.current);
    } catch { /* noop */ }
  };

  const handleStatusChange = async (row: any) => {
    try {
      await bargainStatusApi({ id: row.id, status: !row.status });
      message.success('修改成功');
      fetchList(pagination.current);
    } catch { /* noop */ }
  };

  const handleExport = async () => {
    try {
      const res = await exportBargainApi({ keywords, status });
      if (res?.fileName) window.open(res.fileName);
    } catch { /* noop */ }
  };

  const columns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 60 },
    {
      title: '砍价图片', dataIndex: 'image', width: 80,
      render: (v: string) => <Image src={v} width={36} height={36} style={{ borderRadius: 4 }} />,
    },
    { title: '砍价名称', dataIndex: 'title', ellipsis: true, width: 200 },
    { title: '砍价价格', dataIndex: 'price', width: 100 },
    { title: '最低价', dataIndex: 'minPrice', width: 100 },
    { title: '参与人数', dataIndex: 'countPeopleAll', width: 100 },
    { title: '帮忙砍价人数', dataIndex: 'countPeopleHelp', width: 110 },
    { title: '砍价成功人数', dataIndex: 'countPeopleSuccess', width: 110 },
    { title: '限量', dataIndex: 'quotaShow', width: 80 },
    { title: '限量剩余', dataIndex: 'surplusQuota', width: 100 },
    {
      title: '活动时间', width: 200,
      render: (_: any, r: any) => `${r.startTime} ~ ${r.stopTime}`,
    },
    {
      title: '砍价状态', width: 100, fixed: 'right',
      render: (_: any, record: any) => (
        <Switch size="small" checked={!!record.status}
          checkedChildren="开启" unCheckedChildren="关闭"
          onChange={() => handleStatusChange(record)} />
      ),
    },
    {
      title: '操作', width: 120, fixed: 'right',
      render: (_: any, record: any) => (
        <Space size="small">
          <a onClick={() => navigate(`/marketing/bargain/creatBargain?id=${record.id}`)}>编辑</a>
          <Popconfirm title="删除该商品吗?" onConfirm={() => handleDelete(record.id)}>
            <a style={{ color: '#ff4d4f' }}>删除</a>
          </Popconfirm>
        </Space>
      ),
    },
  ];

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
      <Card bodyStyle={{ paddingBottom: 0 }}>
        <Form layout="inline" style={{ marginBottom: 16 }}>
          <Form.Item label="砍价状态">
            <Select value={status} onChange={(v) => setStatus(v)} placeholder="请选择"
              allowClear style={{ width: 160 }}>
              <Select.Option value={0}>关闭</Select.Option>
              <Select.Option value={1}>开启</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item label="商品搜索">
            <Input placeholder="请输入商品名称、ID" value={keywords} onChange={(e) => setKeywords(e.target.value)}
              onPressEnter={() => fetchList(1)} allowClear style={{ width: 240 }} />
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
        <div style={{ display: 'flex', justifyContent: 'flex-end', marginBottom: 16, gap: 8 }}>
          <Button onClick={handleExport} icon={<ExportOutlined />}>导出</Button>
          <Button type="primary" icon={<PlusOutlined />}
            onClick={() => navigate('/marketing/bargain/creatBargain')}>添加砍价商品</Button>
        </div>
        <Table rowKey="id" columns={columns} dataSource={list} loading={loading} size="small"
          scroll={{ x: 1600 }}
          pagination={{ ...pagination, showSizeChanger: true, pageSizeOptions: ['10', '20', '30', '40'],
            showTotal: (t: number) => `共 ${t} 条`,
            onChange: (p: number, ps: number) => { setPagination((prev) => ({ ...prev, pageSize: ps })); fetchList(p); } }} />
      </Card>
    </div>
  );
};

export default BargainGoods;
