import React, { useState, useEffect, useCallback } from 'react';
import { Card, Table, Form, Input, Select, Button, Space, Image, Switch, Tag, DatePicker, message, Popconfirm } from 'antd';
import { PlusOutlined, ReloadOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { atuosphereList, atmosphereDelete, atmosphereStatusApi } from '@/api/marketing';
import { useNavigate, useLocation } from 'react-router-dom';
import dayjs from 'dayjs';

const { RangePicker } = DatePicker;

const methodMap: Record<number, string> = { 0: '全部商品', 1: '指定商品', 3: '指定分类' };
const statusMap: Record<number, { text: string; color: string }> = {
  0: { text: '未开始', color: 'default' },
  1: { text: '进行中', color: 'processing' },
  [-1]: { text: '已结束', color: 'error' },
};

const AtmosphereList: React.FC = () => {
  const navigate = useNavigate();
  const location = useLocation();
  const isBorder = location.pathname.includes('border');
  const styleType = isBorder ? 0 : 1; // 0=边框, 1=氛围图
  const typeName = isBorder ? '活动边框' : '氛围图';
  const addPath = isBorder ? '/marketing/border/add' : '/marketing/atmosphere/add';

  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [name, setName] = useState('');
  const [runningStatus, setRunningStatus] = useState<number | undefined>(undefined);
  const [timeVal, setTimeVal] = useState<[dayjs.Dayjs, dayjs.Dayjs] | null>(null);
  const [starttime, setStarttime] = useState('');
  const [endtime, setEndtime] = useState('');
  const [pagination, setPagination] = useState({ current: 1, pageSize: 20, total: 0 });

  const fetchList = useCallback(async (page = 1) => {
    setLoading(true);
    try {
      const res = await atuosphereList({
        page, limit: pagination.pageSize, type: styleType,
        name: name ? encodeURIComponent(name) : undefined,
        runningStatus, starttime: starttime || undefined, endtime: endtime || undefined,
      });
      setList(res?.list || []);
      setPagination((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch { message.error(`获取${typeName}列表失败`); }
    finally { setLoading(false); }
  }, [name, runningStatus, starttime, endtime, pagination.pageSize, styleType]);

  useEffect(() => { fetchList(1); }, [styleType]);

  const handleReset = () => {
    setName(''); setRunningStatus(undefined); setTimeVal(null); setStarttime(''); setEndtime('');
  };
  const handleTimeChange = (dates: any) => {
    setTimeVal(dates);
    setStarttime(dates ? dates[0].format('YYYY-MM-DD HH:mm:ss') : '');
    setEndtime(dates ? dates[1].format('YYYY-MM-DD HH:mm:ss') : '');
  };
  const handleDelete = async (id: number) => {
    try { await atmosphereDelete({ id }); message.success('删除成功'); fetchList(pagination.current); } catch {}
  };
  const handleStatusChange = async (row: any) => {
    try { await atmosphereStatusApi({ id: row.id, status: !row.status }); message.success('修改成功'); fetchList(pagination.current); }
    catch {}
  };
  const handleEdit = (row: any) => {
    localStorage.setItem('activitystyle', JSON.stringify(row));
    navigate(`${addPath}?id=${row.id}`);
  };

  const columns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 60 },
    { title: '活动名称', dataIndex: 'name', ellipsis: true, width: 150 },
    { title: typeName, dataIndex: 'style', width: 100,
      render: (v: string) => v ? <Image src={v} width={36} height={36} style={{ borderRadius: 4 }} /> : '-' },
    { title: '使用范围', width: 110,
      render: (_: any, r: any) => methodMap[r.method] || '-' },
    { title: '活动日期', width: 260,
      render: (_: any, r: any) => `${r.starttime || ''} - ${r.endtime || ''}` },
    { title: '活动状态', width: 90,
      render: (_: any, r: any) => {
        const s = statusMap[r.runningStatus] || { text: '未知', color: 'default' };
        return <Tag color={s.color}>{s.text}</Tag>;
      } },
    { title: '创建时间', dataIndex: 'createtime', width: 150 },
    { title: '是否开启', width: 90, fixed: 'right',
      render: (_: any, record: any) => (
        <Switch size="small" checked={!!record.status}
          checkedChildren="开启" unCheckedChildren="关闭"
          onChange={() => handleStatusChange(record)} />
      ) },
    { title: '操作', width: 100, fixed: 'right',
      render: (_: any, record: any) => (
        <Space size="small">
          <a onClick={() => handleEdit(record)}>编辑</a>
          <Popconfirm title="删除活动后将无法恢复，请谨慎操作!" onConfirm={() => handleDelete(record.id)}>
            <a style={{ color: '#ff4d4f' }}>删除</a>
          </Popconfirm>
        </Space>
      ) },
  ];

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
      <Card bodyStyle={{ paddingBottom: 0 }}>
        <Form layout="inline" style={{ marginBottom: 16 }}>
          <Form.Item label="创建时间">
            <RangePicker showTime value={timeVal} onChange={handleTimeChange} style={{ width: 360 }}
              format="YYYY-MM-DD HH:mm:ss" />
          </Form.Item>
          <Form.Item label="活动状态">
            <Select value={runningStatus} onChange={(v) => setRunningStatus(v)} placeholder="请选择"
              allowClear style={{ width: 140 }}>
              <Select.Option value={0}>未开始</Select.Option>
              <Select.Option value={1}>进行中</Select.Option>
              <Select.Option value={-1}>已结束</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item label="活动名称">
            <Input placeholder="请输入活动名称" value={name} onChange={(e) => setName(e.target.value)}
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
        <div style={{ marginBottom: 16 }}>
          <Button type="primary" icon={<PlusOutlined />} onClick={() => navigate(addPath)}>添加{typeName}</Button>
        </div>
        <Table rowKey="id" columns={columns} dataSource={list} loading={loading} size="small"
          scroll={{ x: 1300 }}
          pagination={{ ...pagination, showSizeChanger: true, pageSizeOptions: ['20', '40', '60', '80'],
            showTotal: (t: number) => `共 ${t} 条`,
            onChange: (p: number, ps: number) => { setPagination((prev) => ({ ...prev, pageSize: ps })); fetchList(p); } }} />
      </Card>
    </div>
  );
};

export default AtmosphereList;
