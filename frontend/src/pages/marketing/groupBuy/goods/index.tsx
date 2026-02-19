import React, { useState, useEffect, useCallback } from 'react';
import { Card, Table, Form, Input, Select, Button, Space, Image, Switch, message, Popconfirm } from 'antd';
import { PlusOutlined, SearchOutlined, ReloadOutlined, ExportOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { combinationListApi, combinationDeleteApi, combinationStatusApi, exportcombiantionApi } from '@/api/marketing';
import { useNavigate } from 'react-router-dom';

const GroupGoods: React.FC = () => {
  const navigate = useNavigate();
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [keywords, setKeywords] = useState('');
  const [isShow, setIsShow] = useState<number | undefined>(undefined);
  const [pagination, setPagination] = useState({ current: 1, pageSize: 20, total: 0 });

  const fetchList = useCallback(async (page = 1) => {
    setLoading(true);
    try {
      const res = await combinationListApi({ page, limit: pagination.pageSize, keywords: keywords || undefined, isShow });
      setList(res?.list || []);
      setPagination((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch {
      message.error('获取拼团商品列表失败');
    } finally {
      setLoading(false);
    }
  }, [keywords, isShow, pagination.pageSize]);

  useEffect(() => { fetchList(1); }, []);

  const handleReset = () => {
    setKeywords('');
    setIsShow(undefined);
  };

  const handleDelete = async (id: number) => {
    try {
      await combinationDeleteApi({ id });
      message.success('删除成功');
      fetchList(pagination.current);
    } catch { /* noop */ }
  };

  const handleStatusChange = async (row: any) => {
    try {
      await combinationStatusApi({ id: row.id, isShow: !row.isShow });
      message.success('修改成功');
      fetchList(pagination.current);
    } catch { /* noop */ }
  };

  const handleExport = async () => {
    try {
      const res = await exportcombiantionApi({ keywords, isShow });
      if (res?.fileName) window.open(res.fileName);
    } catch { /* noop */ }
  };

  const columns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 60 },
    {
      title: '拼团图片', dataIndex: 'image', width: 80,
      render: (v: string) => <Image src={v} width={36} height={36} style={{ borderRadius: 4 }} />,
    },
    { title: '拼团名称', dataIndex: 'title', ellipsis: true, width: 200 },
    { title: '原价', dataIndex: 'otPrice', width: 100 },
    { title: '拼团价', dataIndex: 'price', width: 100 },
    { title: '拼团人数', dataIndex: 'countPeople', width: 100 },
    { title: '参与人数', dataIndex: 'countPeopleAll', width: 100 },
    { title: '成团数量', dataIndex: 'countPeoplePink', width: 100 },
    { title: '限量', dataIndex: 'quotaShow', width: 80 },
    { title: '限量剩余', dataIndex: 'remainingQuota', width: 100 },
    { title: '结束时间', dataIndex: 'stopTime', width: 150 },
    {
      title: '拼团状态', width: 100, fixed: 'right',
      render: (_: any, record: any) => (
        <Switch size="small" checked={!!record.isShow}
          checkedChildren="开启" unCheckedChildren="关闭"
          onChange={() => handleStatusChange(record)} />
      ),
    },
    {
      title: '操作', width: 120, fixed: 'right',
      render: (_: any, record: any) => (
        <Space size="small">
          <a onClick={() => navigate(record.isShow
            ? `/marketing/groupBuy/creatGroup?id=${record.id}&info=1`
            : `/marketing/groupBuy/creatGroup?id=${record.id}`)}>
            {record.isShow ? '详情' : '编辑'}
          </a>
          <Popconfirm title="永久删除该商品?" onConfirm={() => handleDelete(record.id)}>
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
          <Form.Item label="拼团状态">
            <Select value={isShow} onChange={(v) => setIsShow(v)} placeholder="请选择"
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
            onClick={() => navigate('/marketing/groupBuy/creatGroup')}>添加拼团商品</Button>
        </div>
        <Table rowKey="id" columns={columns} dataSource={list} loading={loading} size="small"
          scroll={{ x: 1500 }}
          pagination={{ ...pagination, showSizeChanger: true, pageSizeOptions: ['10', '20', '30', '40'],
            showTotal: (t: number) => `共 ${t} 条`,
            onChange: (p: number, ps: number) => { setPagination((prev) => ({ ...prev, pageSize: ps })); fetchList(p); } }} />
      </Card>
    </div>
  );
};

export default GroupGoods;
