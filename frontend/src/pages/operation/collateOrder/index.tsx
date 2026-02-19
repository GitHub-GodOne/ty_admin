import React, { useState, useEffect, useCallback } from 'react';
import { Card, Table, Form, Input, Button, Select, DatePicker, Space, Statistic, Row, Col, message } from 'antd';
import { SearchOutlined, ReloadOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { orderListApi, storeListApi } from '@/api/storePoint';
import dayjs from 'dayjs';

const { RangePicker } = DatePicker;

const CollateOrder: React.FC = () => {
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [storeOptions, setStoreOptions] = useState<any[]>([]);
  const [keywords, setKeywords] = useState('');
  const [storeId, setStoreId] = useState<number | string>('');
  const [dateLimit, setDateLimit] = useState('');
  const [timeVal, setTimeVal] = useState<any>(null);
  const [pagination, setPagination] = useState({ current: 1, pageSize: 20, total: 0 });
  const [stats, setStats] = useState<any>({});

  const fetchStores = useCallback(async () => {
    try {
      const res: any = await storeListApi({ page: 1, limit: 999, status: 1 });
      setStoreOptions(res?.list || []);
    } catch { /* */ }
  }, []);

  const fetchList = useCallback(async (page = 1) => {
    setLoading(true);
    try {
      const params: any = { page, limit: pagination.pageSize };
      if (keywords) params.keywords = keywords;
      if (storeId) params.storeId = storeId;
      if (dateLimit) params.dateLimit = dateLimit;
      const res: any = await orderListApi(params);
      setList(res?.list?.list || []);
      setPagination((p) => ({ ...p, current: page, total: res?.list?.total || 0 }));
      setStats({
        total: res?.total ?? 0,
        orderTotalPrice: res?.orderTotalPrice ?? 0,
        refundTotal: res?.refundTotal ?? 0,
        refundTotalPrice: res?.refundTotalPrice ?? 0,
      });
    } catch { /* */ }
    finally { setLoading(false); }
  }, [keywords, storeId, dateLimit, pagination.pageSize]);

  useEffect(() => { fetchStores(); }, []);
  useEffect(() => { fetchList(1); }, [storeId, dateLimit]);

  const handleReset = () => {
    setKeywords(''); setStoreId(''); setDateLimit(''); setTimeVal(null);
  };

  const handleDateChange = (_: any, dateStrings: [string, string]) => {
    setTimeVal(_);
    setDateLimit(dateStrings[0] && dateStrings[1] ? `${dateStrings[0]},${dateStrings[1]}` : '');
  };

  const columns: ColumnsType<any> = [
    { title: '订单号', dataIndex: 'orderId', width: 200 },
    { title: '用户信息', dataIndex: 'realName', width: 100 },
    { title: '推荐人信息', width: 100, render: (_: any, r: any) => r.spreadInfo?.name || '-' },
    {
      title: '商品信息', width: 360, ellipsis: true,
      render: (_: any, record: any) => (
        <div>
          {record.productList?.map((item: any, i: number) => (
            <div key={i} style={{ display: 'flex', alignItems: 'center', marginBottom: 4 }}>
              {item.info?.image && <img src={item.info.image} alt="" style={{ width: 36, height: 36, marginRight: 8, objectFit: 'cover', borderRadius: 4 }} />}
              <div style={{ overflow: 'hidden', whiteSpace: 'nowrap', textOverflow: 'ellipsis' }}>
                <span>{item.info?.productName} | {item.info?.sku || '-'}</span>
                <span style={{ marginLeft: 8 }}>￥{item.info?.price} x {item.info?.payNum}</span>
              </div>
            </div>
          ))}
        </div>
      ),
    },
    { title: '实际支付', dataIndex: 'payPrice', width: 100 },
    { title: '核销员', width: 100, render: (_: any, r: any) => r.clerkName || '-' },
    { title: '核销门店', dataIndex: 'storeName', width: 150 },
    { title: '支付状态', width: 80, render: (_: any, r: any) => r.paid ? '已支付' : '未支付' },
    { title: '订单状态', width: 100, render: (_: any, r: any) => r.statusStr?.value || '-' },
    { title: '下单时间', dataIndex: 'createTime', width: 150 },
  ];

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
      <Card bodyStyle={{ paddingBottom: 0 }}>
        <Form layout="inline" style={{ marginBottom: 16 }}>
          <Form.Item label="时间选择">
            <RangePicker value={timeVal} onChange={handleDateChange} style={{ width: 250 }} />
          </Form.Item>
          <Form.Item label="选择门店">
            <Select value={storeId || undefined} onChange={(v) => setStoreId(v)} allowClear placeholder="请选择" style={{ width: 200 }}>
              {storeOptions.map((s: any) => <Select.Option key={s.id} value={s.id}>{s.name}</Select.Option>)}
            </Select>
          </Form.Item>
          <Form.Item label="关键字">
            <Input value={keywords} onChange={(e) => setKeywords(e.target.value)} placeholder="姓名、电话、订单ID"
              onPressEnter={() => fetchList(1)} allowClear style={{ width: 220 }} />
          </Form.Item>
          <Form.Item>
            <Space>
              <Button type="primary" icon={<SearchOutlined />} onClick={() => fetchList(1)}>搜索</Button>
              <Button icon={<ReloadOutlined />} onClick={handleReset}>重置</Button>
            </Space>
          </Form.Item>
        </Form>
      </Card>

      <Row gutter={16}>
        <Col span={6}><Card><Statistic title="订单数量" value={stats.total} valueStyle={{ color: '#1890FF' }} /></Card></Col>
        <Col span={6}><Card><Statistic title="订单金额" value={stats.orderTotalPrice} prefix="￥" valueStyle={{ color: '#A277FF' }} /></Card></Col>
        <Col span={6}><Card><Statistic title="退款总单数" value={stats.refundTotal} valueStyle={{ color: '#EF9C20' }} /></Card></Col>
        <Col span={6}><Card><Statistic title="退款总金额" value={stats.refundTotalPrice} prefix="￥" valueStyle={{ color: '#1BBE6B' }} /></Card></Col>
      </Row>

      <Card>
        <Table rowKey="orderId" columns={columns} dataSource={list} loading={loading} size="small"
          scroll={{ x: 1400 }}
          pagination={{
            current: pagination.current, pageSize: pagination.pageSize, total: pagination.total,
            showSizeChanger: true, pageSizeOptions: ['20', '40', '60', '80'],
            showTotal: (t: number) => `共 ${t} 条`,
            onChange: (p, ps) => { setPagination((prev) => ({ ...prev, pageSize: ps || 20 })); fetchList(p); },
          }}
        />
      </Card>
    </div>
  );
};

export default CollateOrder;
