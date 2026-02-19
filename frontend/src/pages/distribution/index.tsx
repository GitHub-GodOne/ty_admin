import React, { useState, useEffect, useCallback } from 'react';
import { Card, Table, Form, Input, Button, Space, Tag, Select, message, Drawer, Tabs, Avatar, Descriptions, DatePicker } from 'antd';
import { SearchOutlined, ReloadOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { promoterListApi, spreadListApi, spreadOrderListApi, spreadClearApi } from '@/api/distribution';
import { usePagination } from '@/hooks/usePagination';
import { useModal } from '@/hooks/useModal';
import dayjs from 'dayjs';

const { RangePicker } = DatePicker;

const Distribution: React.FC = () => {
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [keywords, setKeywords] = useState('');
  const [dateRange, setDateRange] = useState<[dayjs.Dayjs, dayjs.Dayjs] | null>(null);
  const { pagination, setTotal, antdPagination } = usePagination();
  const { deleteConfirm } = useModal();

  // 详情抽屉
  const [drawerOpen, setDrawerOpen] = useState(false);
  const [currentUser, setCurrentUser] = useState<any>(null);
  const [spreadTab, setSpreadTab] = useState('user');
  const [spreadType, setSpreadType] = useState(0);
  const [spreadList, setSpreadList] = useState<any[]>([]);
  const [spreadLoading, setSpreadLoading] = useState(false);
  const [spreadPag, setSpreadPag] = useState({ current: 1, pageSize: 10, total: 0 });
  const [orderList, setOrderList] = useState<any[]>([]);
  const [orderLoading, setOrderLoading] = useState(false);
  const [orderPag, setOrderPag] = useState({ current: 1, pageSize: 10, total: 0 });

  const fetchList = useCallback(async () => {
    setLoading(true);
    try {
      const params: any = { page: pagination.page, limit: pagination.limit };
      if (keywords) params.keywords = keywords;
      if (dateRange) params.dateLimit = `${dateRange[0].format('YYYY-MM-DD')},${dateRange[1].format('YYYY-MM-DD')}`;
      const res: any = await promoterListApi(params);
      setList(res?.list || []);
      setTotal(res?.total || 0);
    } catch { /* handled */ }
    finally { setLoading(false); }
  }, [keywords, dateRange, pagination.page, pagination.limit]);

  useEffect(() => { fetchList(); }, [fetchList]);

  const handleReset = () => { setKeywords(''); setDateRange(null); };

  // 推广人列表
  const fetchSpreadUsers = useCallback(async (uid: number, type: number, page = 1) => {
    setSpreadLoading(true);
    try {
      const res: any = await spreadListApi({ page, limit: spreadPag.pageSize }, { uid, type });
      setSpreadList(res?.list || []);
      setSpreadPag((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch { /* handled */ }
    finally { setSpreadLoading(false); }
  }, [spreadPag.pageSize]);

  // 推广订单列表
  const fetchSpreadOrders = useCallback(async (uid: number, type: number, page = 1) => {
    setOrderLoading(true);
    try {
      const res: any = await spreadOrderListApi({ page, limit: orderPag.pageSize }, { uid, type });
      setOrderList(res?.list || []);
      setOrderPag((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch { /* handled */ }
    finally { setOrderLoading(false); }
  }, [orderPag.pageSize]);

  const openDrawer = (record: any) => {
    setCurrentUser(record);
    setDrawerOpen(true);
    setSpreadTab('user');
    setSpreadType(0);
    fetchSpreadUsers(record.uid, 0, 1);
    fetchSpreadOrders(record.uid, 0, 1);
  };

  const handleSpreadTypeChange = (val: number) => {
    setSpreadType(val);
    if (currentUser) {
      fetchSpreadUsers(currentUser.uid, val, 1);
      fetchSpreadOrders(currentUser.uid, val, 1);
    }
  };

  const handleClearSpread = (record: any) => {
    deleteConfirm(async () => {
      await spreadClearApi(record.uid);
      message.success('已清除上级推广人');
      fetchList();
    }, '确定清除该用户的上级推广人关系吗？');
  };

  const columns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'uid', width: 70 },
    { title: '头像', dataIndex: 'avatar', width: 60, render: (v: string) => <Avatar src={v} size="small" /> },
    { title: '昵称', dataIndex: 'nickname', width: 110, ellipsis: true },
    { title: '手机号', dataIndex: 'phone', width: 120 },
    { title: '推广人', dataIndex: 'spreadNickname', width: 110, ellipsis: true, render: (v: string) => v || '-' },
    { title: '累计佣金', dataIndex: 'totalBrokeragePrice', width: 100, render: (v: any) => `¥${v ?? 0}` },
    { title: '已提现', dataIndex: 'extractCountPrice', width: 100, render: (v: any) => `¥${v ?? 0}` },
    { title: '未提现', dataIndex: 'brokeragePrice', width: 100, render: (v: any) => `¥${v ?? 0}` },
    { title: '冻结佣金', dataIndex: 'freezeBrokeragePrice', width: 100, render: (v: any) => `¥${v ?? 0}` },
    { title: '推广人数', dataIndex: 'spreadCount', width: 90 },
    { title: '推广订单', dataIndex: 'spreadOrderNum', width: 90 },
    { title: '推广订单额', dataIndex: 'spreadOrderTotalPrice', width: 110, render: (v: any) => `¥${v ?? 0}` },
    {
      title: '操作', width: 130, fixed: 'right',
      render: (_: any, record: any) => (
        <Space size="small">
          <a onClick={() => openDrawer(record)}>更多</a>
          {record.spreadUid ? <a style={{ color: '#ff4d4f' }} onClick={() => handleClearSpread(record)}>清除推广人</a> : null}
        </Space>
      ),
    },
  ];

  const spreadColumns: ColumnsType<any> = [
    { title: '头像', dataIndex: 'avatar', width: 60, render: (v: string) => <Avatar src={v} size="small" /> },
    { title: '昵称', dataIndex: 'nickname', width: 120, ellipsis: true },
    { title: '是否推广员', dataIndex: 'isPromoter', width: 100, render: (v: boolean) => <Tag color={v ? 'green' : 'default'}>{v ? '是' : '否'}</Tag> },
    { title: '推广人数', dataIndex: 'spreadCount', width: 90 },
    { title: '购买次数', dataIndex: 'payCount', width: 90 },
  ];

  const orderColumns: ColumnsType<any> = [
    { title: '订单号', dataIndex: 'orderId', width: 200, ellipsis: true },
    { title: '收货人', dataIndex: 'realName', width: 100 },
    { title: '电话', dataIndex: 'userPhone', width: 120 },
    { title: '金额', dataIndex: 'price', width: 100, render: (v: any) => `¥${v ?? 0}` },
    { title: '时间', dataIndex: 'updateTime', width: 160 },
  ];

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
      <Card bodyStyle={{ paddingBottom: 0 }}>
        <Form layout="inline" style={{ marginBottom: 16 }}>
          <Form.Item><Input placeholder="昵称/手机号/UID" value={keywords} onChange={(e) => setKeywords(e.target.value)} style={{ width: 200 }} allowClear /></Form.Item>
          <Form.Item><RangePicker value={dateRange} onChange={(dates) => setDateRange(dates as [dayjs.Dayjs, dayjs.Dayjs] | null)} /></Form.Item>
          <Form.Item><Button type="primary" icon={<SearchOutlined />} onClick={fetchList}>搜索</Button></Form.Item>
          <Form.Item><Button icon={<ReloadOutlined />} onClick={handleReset}>重置</Button></Form.Item>
        </Form>
      </Card>
      <Card>
        <Table rowKey="uid" columns={columns} dataSource={list} loading={loading}
          pagination={antdPagination} scroll={{ x: 1400 }} size="small" />
      </Card>

      <Drawer title="分销员详情" open={drawerOpen} onClose={() => setDrawerOpen(false)} width={700} destroyOnClose>
        {currentUser && (
          <>
            <Descriptions column={2} size="small" bordered style={{ marginBottom: 16 }}>
              <Descriptions.Item label="昵称">{currentUser.nickname || '-'}</Descriptions.Item>
              <Descriptions.Item label="手机号">{currentUser.phone || '-'}</Descriptions.Item>
              <Descriptions.Item label="上级推广人">{currentUser.spreadNickname || '-'}</Descriptions.Item>
              <Descriptions.Item label="成为分销员时间">{currentUser.promoterTime || '-'}</Descriptions.Item>
              <Descriptions.Item label="累计佣金">¥{currentUser.totalBrokeragePrice ?? 0}</Descriptions.Item>
              <Descriptions.Item label="已提现">¥{currentUser.extractCountPrice ?? 0}（{currentUser.extractCountNum ?? 0}次）</Descriptions.Item>
              <Descriptions.Item label="未提现">¥{currentUser.brokeragePrice ?? 0}</Descriptions.Item>
              <Descriptions.Item label="冻结佣金">¥{currentUser.freezeBrokeragePrice ?? 0}</Descriptions.Item>
              <Descriptions.Item label="推广人数">{currentUser.spreadCount ?? 0}</Descriptions.Item>
              <Descriptions.Item label="推广订单">{currentUser.spreadOrderNum ?? 0}单 / ¥{currentUser.spreadOrderTotalPrice ?? 0}</Descriptions.Item>
            </Descriptions>

            <div style={{ marginBottom: 12 }}>
              <span style={{ marginRight: 8 }}>推广等级：</span>
              <Select value={spreadType} onChange={handleSpreadTypeChange} style={{ width: 140 }} size="small">
                <Select.Option value={0}>全部</Select.Option>
                <Select.Option value={1}>一级推广</Select.Option>
                <Select.Option value={2}>二级推广</Select.Option>
              </Select>
            </div>

            <Tabs activeKey={spreadTab} onChange={setSpreadTab} items={[
              { key: 'user', label: '推广人列表', children: (
                <Table rowKey="uid" columns={spreadColumns} dataSource={spreadList} loading={spreadLoading} size="small"
                  pagination={{ ...spreadPag, showSizeChanger: false, showTotal: (t: number) => `共 ${t} 条`,
                    onChange: (p: number) => currentUser && fetchSpreadUsers(currentUser.uid, spreadType, p) }} />
              )},
              { key: 'order', label: '推广订单', children: (
                <Table rowKey="id" columns={orderColumns} dataSource={orderList} loading={orderLoading} size="small"
                  pagination={{ ...orderPag, showSizeChanger: false, showTotal: (t: number) => `共 ${t} 条`,
                    onChange: (p: number) => currentUser && fetchSpreadOrders(currentUser.uid, spreadType, p) }} />
              )},
            ]} />
          </>
        )}
      </Drawer>
    </div>
  );
};

export default Distribution;
