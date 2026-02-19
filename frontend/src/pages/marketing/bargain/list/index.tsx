import React, { useState, useEffect, useCallback } from 'react';
import { Card, Table, Form, Select, Button, Image, Tag, Modal, DatePicker, message } from 'antd';
import type { ColumnsType } from 'antd/es/table';
import { bargainListListApi, bargainOrderPinkApi } from '@/api/marketing';
import dayjs from 'dayjs';

const { RangePicker } = DatePicker;

const bargainStatusMap: Record<number, { text: string; color: string }> = {
  1: { text: '进行中', color: 'processing' },
  2: { text: '未完成', color: 'warning' },
  3: { text: '已成功', color: 'success' },
};

const BargainList: React.FC = () => {
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [status, setStatus] = useState<number | undefined>(undefined);
  const [dateLimit, setDateLimit] = useState('');
  const [timeVal, setTimeVal] = useState<[dayjs.Dayjs, dayjs.Dayjs] | null>(null);
  const [pagination, setPagination] = useState({ current: 1, pageSize: 20, total: 0 });
  const [detailOpen, setDetailOpen] = useState(false);
  const [detailLoading, setDetailLoading] = useState(false);
  const [detailData, setDetailData] = useState<any[]>([]);

  const fetchList = useCallback(async (page = 1) => {
    setLoading(true);
    try {
      const res = await bargainListListApi({ page, limit: pagination.pageSize, status, dateLimit: dateLimit || undefined });
      setList(res?.list || []);
      setPagination((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch {
      message.error('获取砍价列表失败');
    } finally {
      setLoading(false);
    }
  }, [status, dateLimit, pagination.pageSize]);

  useEffect(() => { fetchList(1); }, []);

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
      const res = await bargainOrderPinkApi(id);
      setDetailData(Array.isArray(res) ? res : (res?.list || []));
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
    { title: '发起用户', dataIndex: 'nickname', width: 100 },
    { title: '开启时间', dataIndex: 'addTime', width: 150 },
    { title: '砍价商品', dataIndex: 'title', ellipsis: true, width: 300 },
    { title: '最低价', dataIndex: 'bargainPriceMin', width: 100 },
    { title: '当前价', dataIndex: 'nowPrice', width: 100 },
    { title: '总砍价次数', dataIndex: 'peopleNum', width: 100 },
    { title: '剩余砍价次数', dataIndex: 'num', width: 110 },
    { title: '结束时间', dataIndex: 'dataTime', width: 150 },
    {
      title: '砍价状态', dataIndex: 'status', width: 100,
      render: (v: number) => {
        const s = bargainStatusMap[v] || { text: '未知', color: 'default' };
        return <Tag color={s.color}>{s.text}</Tag>;
      },
    },
    {
      title: '操作', width: 100, fixed: 'right',
      render: (_: any, record: any) => <a onClick={() => handleLook(record.id)}>查看详情</a>,
    },
  ];

  const detailColumns: ColumnsType<any> = [
    { title: '用户ID', dataIndex: 'uid', width: 80 },
    {
      title: '用户头像', dataIndex: 'avatar', width: 80,
      render: (v: string) => <Image src={v} width={36} height={36} style={{ borderRadius: '50%' }} />,
    },
    { title: '用户名称', dataIndex: 'nickname', width: 120 },
    { title: '砍价金额', dataIndex: 'price', width: 100 },
    { title: '砍价时间', dataIndex: 'addTime', width: 180 },
  ];

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
      <Card bodyStyle={{ paddingBottom: 0 }}>
        <Form layout="inline" style={{ marginBottom: 16 }}>
          <Form.Item label="时间选择">
            <RangePicker value={timeVal} onChange={handleTimeChange} style={{ width: 260 }} />
          </Form.Item>
          <Form.Item label="砍价状态">
            <Select value={status} onChange={(v) => { setStatus(v); }} placeholder="请选择"
              allowClear style={{ width: 160 }}>
              <Select.Option value={1}>进行中</Select.Option>
              <Select.Option value={2}>未完成</Select.Option>
              <Select.Option value={3}>已成功</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item>
            <Button type="primary" onClick={() => fetchList(1)}>搜索</Button>
          </Form.Item>
        </Form>
      </Card>
      <Card>
        <Table rowKey="id" columns={columns} dataSource={list} loading={loading} size="small"
          scroll={{ x: 1400 }}
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

export default BargainList;
