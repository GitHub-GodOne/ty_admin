import React, { useState, useEffect, useCallback } from 'react';
import { Card, Table, Form, Input, Select, Button, Space, Image, Switch, message, Popconfirm } from 'antd';
import { PlusOutlined, SearchOutlined, ReloadOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { seckillStoreListApi, seckillStoreDeleteApi, seckillStoreStatusApi, seckillListApi } from '@/api/marketing';
import { useNavigate, useSearchParams } from 'react-router-dom';

const SeckillList: React.FC = () => {
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [keywords, setKeywords] = useState('');
  const [status, setStatus] = useState<number | undefined>(undefined);
  const [timeId, setTimeId] = useState<number | undefined>(() => {
    const t = searchParams.get('timeId');
    return t ? Number(t) : undefined;
  });
  const [seckillTime, setSeckillTime] = useState<any[]>([]);
  const [pagination, setPagination] = useState({ current: 1, pageSize: 20, total: 0 });

  const fetchSeckillTime = async () => {
    try {
      const res = await seckillListApi({ page: 1, limit: 100 });
      setSeckillTime(res?.list || []);
    } catch { /* noop */ }
  };

  const fetchList = useCallback(async (page = 1) => {
    setLoading(true);
    try {
      const res = await seckillStoreListApi({
        page, limit: pagination.pageSize,
        keywords: keywords || undefined, status, timeId,
      });
      setList(res?.list || []);
      setPagination((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch {
      message.error('获取秒杀商品列表失败');
    } finally {
      setLoading(false);
    }
  }, [keywords, status, timeId, pagination.pageSize]);

  useEffect(() => { fetchSeckillTime(); fetchList(1); }, []);

  const handleReset = () => { setKeywords(''); setStatus(undefined); setTimeId(undefined); };

  const handleDelete = async (id: number) => {
    try {
      await seckillStoreDeleteApi({ id });
      message.success('删除成功');
      fetchList(pagination.current);
    } catch { /* noop */ }
  };

  const handleStatusChange = async (row: any) => {
    try {
      await seckillStoreStatusApi({ id: row.id, status: row.status === 1 ? 0 : 1 });
      message.success('修改成功');
      fetchList(pagination.current);
    } catch { /* noop */ }
  };

  const columns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 60 },
    {
      title: '配置', width: 160,
      render: (_: any, r: any) => {
        const mgr = seckillTime.find((t: any) => t.id === r.timeId);
        return (
          <div>
            <div>{mgr?.name || '-'}</div>
            <div style={{ fontSize: 12, color: '#999' }}>{r.startTime} - {r.stopTime}</div>
          </div>
        );
      },
    },
    {
      title: '秒杀时段', width: 130,
      render: (_: any, r: any) => {
        const mgr = seckillTime.find((t: any) => t.id === r.timeId);
        return mgr?.time ? mgr.time.split(',').join(' - ') : '-';
      },
    },
    {
      title: '商品图片', dataIndex: 'image', width: 80,
      render: (v: string) => <Image src={v} width={36} height={36} style={{ borderRadius: 4 }} />,
    },
    { title: '商品标题', dataIndex: 'title', ellipsis: true, width: 250 },
    { title: '原价', dataIndex: 'otPrice', width: 80 },
    { title: '秒杀价', dataIndex: 'price', width: 80 },
    { title: '限量', dataIndex: 'quotaShow', width: 70 },
    { title: '限量剩余', dataIndex: 'quota', width: 80 },
    { title: '秒杀状态', dataIndex: 'statusName', width: 100 },
    { title: '创建时间', dataIndex: 'createTime', width: 150 },
    {
      title: '状态', width: 100, fixed: 'right',
      render: (_: any, record: any) => (
        <Switch size="small" checked={record.status === 1}
          checkedChildren="开启" unCheckedChildren="关闭"
          onChange={() => handleStatusChange(record)} />
      ),
    },
    {
      title: '操作', width: 120, fixed: 'right',
      render: (_: any, record: any) => (
        <Space size="small">
          <a onClick={() => navigate(`/marketing/seckill/creatSeckill?productId=${record.productId}&id=${record.id}`)}>编辑</a>
          {record.killStatus !== 2 && (
            <Popconfirm title="永久删除该商品?" onConfirm={() => handleDelete(record.id)}>
              <a style={{ color: '#ff4d4f' }}>删除</a>
            </Popconfirm>
          )}
        </Space>
      ),
    },
  ];

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
      <Card bodyStyle={{ paddingBottom: 0 }}>
        <Form layout="inline" style={{ marginBottom: 16 }}>
          <Form.Item label="是否显示">
            <Select value={status} onChange={(v) => setStatus(v)} placeholder="请选择"
              allowClear style={{ width: 140 }}>
              <Select.Option value={0}>关闭</Select.Option>
              <Select.Option value={1}>开启</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item label="配置名称">
            <Select value={timeId} onChange={(v) => setTimeId(v)} placeholder="请选择"
              allowClear style={{ width: 200 }}>
              {seckillTime.map((item: any) => (
                <Select.Option key={item.id} value={item.id}>
                  {item.name} - {item.time}
                </Select.Option>
              ))}
            </Select>
          </Form.Item>
          <Form.Item label="商品搜索">
            <Input placeholder="请输入商品ID/名称" value={keywords} onChange={(e) => setKeywords(e.target.value)}
              onPressEnter={() => fetchList(1)} allowClear style={{ width: 200 }} />
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
        <div style={{ display: 'flex', justifyContent: 'flex-end', marginBottom: 16 }}>
          <Button type="primary" icon={<PlusOutlined />}
            onClick={() => navigate('/marketing/seckill/creatSeckill')}>添加秒杀商品</Button>
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

export default SeckillList;
