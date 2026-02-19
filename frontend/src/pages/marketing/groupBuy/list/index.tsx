import React, { useState, useEffect, useCallback } from 'react';
import { Card, Table, Form, Select, Button, Image, Tag, Modal, DatePicker, Statistic, Row, Col, message } from 'antd';
import type { ColumnsType } from 'antd/es/table';
import { combineListApi, combineStatisticsApi, combineOrderPinkApi } from '@/api/marketing';
import dayjs from 'dayjs';

const { RangePicker } = DatePicker;

const groupStatusMap: Record<number, { text: string; color: string }> = {
  1: { text: '进行中', color: 'processing' },
  2: { text: '已成功', color: 'success' },
  3: { text: '未完成', color: 'warning' },
};

const orderStatusMap: Record<number, string> = {
  0: '待支付', 1: '待发货', 2: '待收货', 3: '待评价', 4: '已完成',
};
const refundStatusMap: Record<number, string> = {
  1: '申请退款中', 2: '退款中', 3: '已退款',
};

const GroupList: React.FC = () => {
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [status, setStatus] = useState<number | undefined>(undefined);
  const [dateLimit, setDateLimit] = useState('');
  const [timeVal, setTimeVal] = useState<[dayjs.Dayjs, dayjs.Dayjs] | null>(null);
  const [pagination, setPagination] = useState({ current: 1, pageSize: 20, total: 0 });
  const [detailOpen, setDetailOpen] = useState(false);
  const [detailLoading, setDetailLoading] = useState(false);
  const [detailData, setDetailData] = useState<any[]>([]);
  const [statistics, setStatistics] = useState<{ countPeople: number; countTeam: number }>({ countPeople: 0, countTeam: 0 });

  const fetchList = useCallback(async (page = 1) => {
    setLoading(true);
    try {
      const res = await combineListApi({ page, limit: pagination.pageSize, status, dateLimit: dateLimit || undefined });
      setList(res?.list || []);
      setPagination((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch {
      message.error('获取拼团列表失败');
    } finally {
      setLoading(false);
    }
  }, [status, dateLimit, pagination.pageSize]);

  const fetchStatistics = async () => {
    try {
      const res = await combineStatisticsApi({});
      setStatistics({ countPeople: res?.countPeople || 0, countTeam: res?.countTeam || 0 });
    } catch { /* noop */ }
  };

  useEffect(() => { fetchStatistics(); fetchList(1); }, []);

  const handleTimeChange = (dates: any) => {
    setTimeVal(dates);
    if (dates) {
      setDateLimit(`${dates[0].format('YYYY-MM-DD')},${dates[1].format('YYYY-MM-DD')}`);
    } else {
      setDateLimit('');
    }
  };

  const handleLook = async (id: number) => {
    setDetailOpen(true);
    setDetailLoading(true);
    try {
      const res = await combineOrderPinkApi(id);
      setDetailData(Array.isArray(res) ? res : []);
    } catch { /* noop */ } finally {
      setDetailLoading(false);
    }
  };

  const columns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 60 },
    {
      title: '头像', dataIndex: 'avatar', width: 80,
      render: (v: string) => <Image src={v} width={36} height={36} style={{ borderRadius: '50%' }} />,
    },
    { title: '开团团长', dataIndex: 'nickname', width: 100 },
    { title: '开团时间', dataIndex: 'addTime', width: 150 },
    { title: '拼团商品', dataIndex: 'title', ellipsis: true, width: 300 },
    { title: '几人团', dataIndex: 'people', width: 80 },
    { title: '几人参加', dataIndex: 'countPeople', width: 90 },
    { title: '结束时间', dataIndex: 'stopTime', width: 150 },
    {
      title: '拼团状态', dataIndex: 'status', width: 100,
      render: (v: number) => {
        const s = groupStatusMap[v] || { text: '未知', color: 'default' };
        return <Tag color={s.color}>{s.text}</Tag>;
      },
    },
    {
      title: '操作', width: 100, fixed: 'right',
      render: (_: any, record: any) => <a onClick={() => handleLook(record.id)}>查看详情</a>,
    },
  ];

  const detailColumns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 60 },
    {
      title: '头像', dataIndex: 'avatar', width: 80,
      render: (v: string) => <Image src={v} width={36} height={36} style={{ borderRadius: '50%' }} />,
    },
    { title: '用户名称', dataIndex: 'nickname', width: 100 },
    { title: '订单编号', dataIndex: 'orderId', width: 180 },
    { title: '金额', dataIndex: 'totalPrice', width: 100 },
    {
      title: '订单状态', width: 100,
      render: (_: any, r: any) => r.refundStatus === 0
        ? (orderStatusMap[r.orderStatus] || '未知')
        : (refundStatusMap[r.refundStatus] || '未知'),
    },
  ];

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
      <Card bodyStyle={{ paddingBottom: 0 }}>
        <Form layout="inline" style={{ marginBottom: 16 }}>
          <Form.Item label="时间选择">
            <RangePicker value={timeVal} onChange={handleTimeChange} style={{ width: 260 }} />
          </Form.Item>
          <Form.Item label="拼团状态">
            <Select value={status} onChange={(v) => setStatus(v)} placeholder="请选择"
              allowClear style={{ width: 160 }}>
              <Select.Option value={1}>进行中</Select.Option>
              <Select.Option value={2}>已成功</Select.Option>
              <Select.Option value={3}>未完成</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item>
            <Button type="primary" onClick={() => fetchList(1)}>搜索</Button>
          </Form.Item>
        </Form>
      </Card>
      <Row gutter={16}>
        <Col span={12}>
          <Card><Statistic title="参与人数(人)" value={statistics.countPeople} valueStyle={{ color: '#1890FF' }} /></Card>
        </Col>
        <Col span={12}>
          <Card><Statistic title="成团数量(个)" value={statistics.countTeam} valueStyle={{ color: '#A277FF' }} /></Card>
        </Col>
      </Row>
      <Card>
        <Table rowKey="id" columns={columns} dataSource={list} loading={loading} size="small"
          scroll={{ x: 1300 }}
          pagination={{ ...pagination, showSizeChanger: true, pageSizeOptions: ['10', '20', '30', '40'],
            showTotal: (t: number) => `共 ${t} 条`,
            onChange: (p: number, ps: number) => { setPagination((prev) => ({ ...prev, pageSize: ps })); fetchList(p); } }} />
      </Card>
      <Modal title="查看详情" open={detailOpen} onCancel={() => setDetailOpen(false)}
        footer={null} width={650} destroyOnClose>
        <Table rowKey="id" columns={detailColumns} dataSource={detailData}
          loading={detailLoading} size="small" pagination={false} />
      </Modal>
    </div>
  );
};

export default GroupList;
