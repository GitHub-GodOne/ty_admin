import React, { useState, useEffect, useRef } from 'react';
import { Card, Table, Tabs, Form, Input, DatePicker, Button, Space, Tag, message, Popconfirm, Modal, Select, Dropdown, Drawer, Timeline, Image, InputNumber } from 'antd';
import { SearchOutlined, ReloadOutlined, ExportOutlined, DownOutlined, ShoppingOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import type { MenuProps } from 'antd';
import {
  orderListApi, orderDeleteApi, orderSendApi, orderRefundApi, orderRefuseApi,
  orderDetailApi, orderStatusNumApi, orderMarkApi, companyGetListApi,
  orderLogApi, getLogisticsInfoApi,
} from '@/api/order';
import { orderExcelApi } from '@/api/store';
import dayjs from 'dayjs';

const { RangePicker } = DatePicker;
const { TextArea } = Input;

const statusTabs = [
  { key: 'all', label: '全部', status: '' },
  { key: 'unPaid', label: '待支付', status: 'unPaid' },
  { key: 'notShipped', label: '待发货', status: 'notShipped' },
  { key: 'spike', label: '待收货', status: 'spike' },
  { key: 'bargain', label: '待评价', status: 'bargain' },
  { key: 'complete', label: '已完成', status: 'complete' },
  { key: 'toBeWrittenOff', label: '待核销', status: 'toBeWrittenOff' },
  { key: 'refunding', label: '退款中', status: 'refunding' },
  { key: 'refunded', label: '已退款', status: 'refunded' },
  { key: 'deleted', label: '已删除', status: 'deleted' },
];

const statusMap: Record<string, { text: string; color: string }> = {
  '0': { text: '待支付', color: 'default' },
  '1': { text: '待发货', color: 'orange' },
  '2': { text: '待收货', color: 'blue' },
  '3': { text: '待评价', color: 'cyan' },
  '4': { text: '已完成', color: 'green' },
  '5': { text: '待核销', color: 'purple' },
  '-3': { text: '退款中', color: 'red' },
  '-4': { text: '已退款', color: 'volcano' },
  '-5': { text: '已删除', color: 'default' },
};

const payTypeMap: Record<number, string> = { 1: '微信支付', 2: '支付宝', 3: '余额支付' };
const orderTypeOptions = [
  { label: '普通订单', value: 0 },
  { label: '视频号订单', value: 1 },
  { label: '全部订单', value: 2 },
];

const payTypeFilter = (payType: string) => {
  const map: Record<string, string> = {
    weixin: '微信支付',
    alipay: '支付宝',
    yue: '余额支付',
  };
  return map[payType] || payType || '-';
};

const Order: React.FC = () => {
  const [activeTab, setActiveTab] = useState('all');
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [pagination, setPagination] = useState({ current: 1, pageSize: 20, total: 0 });
  const [orderType, setOrderType] = useState<number>(0);
  const [orderNo, setOrderNo] = useState('');
  const [dateRange, setDateRange] = useState<[dayjs.Dayjs, dayjs.Dayjs] | null>(null);
  const [tabCounts, setTabCounts] = useState<Record<string, number>>({});

  // 详情抽屉
  const [detailOpen, setDetailOpen] = useState(false);
  const [detailData, setDetailData] = useState<any>(null);
  const [detailLoading, setDetailLoading] = useState(false);
  const [detailTab, setDetailTab] = useState('detail');
  // 发货弹窗
  const [deliveryOpen, setDeliveryOpen] = useState(false);
  const [deliveryOrderNo, setDeliveryOrderNo] = useState('');
  const [deliveryForm] = Form.useForm();
  const [companyList, setCompanyList] = useState<any[]>([]);
  // 备注弹窗
  const [markOpen, setMarkOpen] = useState(false);
  const [markOrderNo, setMarkOrderNo] = useState('');
  const [markContent, setMarkContent] = useState('');
  // 物流弹窗
  const [logisticsOpen, setLogisticsOpen] = useState(false);
  const [logisticsData, setLogisticsData] = useState<any>(null);
  // 订单记录弹窗
  const [logOpen, setLogOpen] = useState(false);
  const [logList, setLogList] = useState<any[]>([]);
  // 拒绝退款弹窗
  const [refuseOpen, setRefuseOpen] = useState(false);
  const [refuseOrderNo, setRefuseOrderNo] = useState('');
  const [refuseReason, setRefuseReason] = useState('');
  // 退款弹窗
  const [refundOpen, setRefundOpen] = useState(false);
  const [refundOrderNo, setRefundOrderNo] = useState('');
  const [refundAmount, setRefundAmount] = useState<string>('');

  const filterRef = useRef({ orderType: 0, orderNo: '', dateRange: null as any, activeTab: 'all' });
  filterRef.current = { orderType, orderNo, dateRange, activeTab };

  const getStatus = (tab?: string) => {
    const currentTab = tab ?? filterRef.current.activeTab;
    return statusTabs.find((t) => t.key === currentTab)?.status ?? '';
  };

  const fetchStatusNum = async (f?: any) => {
    try {
      const p = f || filterRef.current;
      const params: any = { type: p.orderType };
      if (p.orderNo) params.orderId = p.orderNo;
      if (p.dateRange?.[0]) params.dateLimit = `${p.dateRange[0].format('YYYY-MM-DD')},${p.dateRange[1].format('YYYY-MM-DD')}`;
      const res = await orderStatusNumApi(params);
      if (res) setTabCounts(res);
    } catch {}
  };

  const fetchList = async (page = 1, overrideFilters?: any, tab?: string) => {
    setLoading(true);
    try {
      const f = overrideFilters || filterRef.current;
      const params: any = { page, limit: pagination.pageSize, type: f.orderType };
      const status = getStatus(tab);
      if (status) params.status = status;
      if (f.orderNo) params.orderNo = f.orderNo;
      if (f.dateRange?.[0]) params.dateLimit = `${f.dateRange[0].format('YYYY-MM-DD')},${f.dateRange[1].format('YYYY-MM-DD')}`;
      const res = await orderListApi(params);
      setList(res?.list || []);
      setPagination((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch { message.error('获取订单列表失败'); }
    finally { setLoading(false); }
  };

  useEffect(() => { fetchStatusNum(); fetchList(1); }, []);

  const handleTabChange = (key: string) => {
    setActiveTab(key);
    fetchList(1, undefined, key);
  };

  const handleSearch = () => { fetchStatusNum(); fetchList(1); };
  const handleReset = () => {
    setOrderType(0); setOrderNo(''); setDateRange(null);
    const empty = { orderType: 0, orderNo: '', dateRange: null, activeTab };
    fetchStatusNum(empty); fetchList(1, empty);
  };

  // ========== 操作 ==========
  const handleDetail = async (orderNo: string) => {
    setDetailLoading(true);
    setDetailOpen(true);
    setDetailTab('detail');
    try {
      const res = await orderDetailApi({ orderNo });
      setDetailData(res);
    } catch { message.error('获取订单详情失败'); }
    finally { setDetailLoading(false); }
  };

  const handleDelete = async (orderNo: string) => {
    try {
      await orderDeleteApi({ orderNo });
      message.success('删除成功');
      fetchList(pagination.current);
      fetchStatusNum();
    } catch { message.error('删除失败'); }
  };

  const openRefund = (orderNo: string, payPrice: number) => {
    setRefundOrderNo(orderNo);
    setRefundAmount(payPrice ? String(payPrice) : '');
    setRefundOpen(true);
  };

  const handleRefund = async () => {
    const amount = parseFloat(refundAmount);
    if (!refundAmount || isNaN(amount) || amount <= 0) {
      message.warning('请输入有效的退款金额');
      return;
    }
    try {
      await orderRefundApi({ orderNo: refundOrderNo, amount });
      message.success('退款成功');
      setRefundOpen(false);
      fetchList(pagination.current);
      fetchStatusNum();
    } catch {}
  };

  const openRefuseRefund = (orderNo: string) => {
    setRefuseOrderNo(orderNo);
    setRefuseReason('');
    setRefuseOpen(true);
  };

  const handleRefuseRefund = async () => {
    if (!refuseReason.trim()) {
      message.warning('请输入拒绝原因');
      return;
    }
    try {
      await orderRefuseApi({ orderNo: refuseOrderNo, reason: refuseReason });
      message.success('拒绝退款成功');
      setRefuseOpen(false);
      fetchList(pagination.current);
      fetchStatusNum();
    } catch {}
  };

  // 备注
  const openMark = (orderNo: string, remark?: string) => {
    setMarkOrderNo(orderNo);
    setMarkContent(remark || '');
    setMarkOpen(true);
  };
  const handleMark = async () => {
    if (!markContent.trim()) { message.warning('请输入备注内容'); return; }
    await orderMarkApi({ orderNo: markOrderNo, mark: markContent });
    message.success('备注成功');
    setMarkOpen(false);
    fetchList(pagination.current);
  };

  // 发货
  const openDelivery = async (orderNo: string) => {
    setDeliveryOrderNo(orderNo);
    deliveryForm.resetFields();
    if (companyList.length === 0) {
      try { const res = await companyGetListApi(); setCompanyList(Array.isArray(res) ? res : []); } catch {}
    }
    setDeliveryOpen(true);
  };
  const handleDelivery = async () => {
    const values = await deliveryForm.validateFields();
    try {
      await orderSendApi({ ...values, orderNo: deliveryOrderNo });
      message.success('发货成功');
      setDeliveryOpen(false);
      fetchList(pagination.current);
      fetchStatusNum();
    } catch { message.error('发货失败'); }
  };

  // 物流信息
  const handleLogistics = async (orderNo: string) => {
    try {
      const res = await getLogisticsInfoApi({ orderNo });
      setLogisticsData(res);
      setLogisticsOpen(true);
    } catch { message.error('获取物流信息失败'); }
  };

  // 订单记录
  const handleOrderLog = async (orderNo: string) => {
    try {
      const res = await orderLogApi({ orderNo });
      setLogList(res?.list || []);
      setLogOpen(true);
    } catch { message.error('获取订单记录失败'); }
  };

  // 导出
  const handleExport = async () => {
    try {
      const params: any = { type: orderType };
      const status = getStatus();
      if (status) params.status = status;
      if (orderNo) params.orderNo = orderNo;
      if (dateRange?.[0]) params.dateLimit = `${dateRange[0].format('YYYY-MM-DD')},${dateRange[1].format('YYYY-MM-DD')}`;
      const res = await orderExcelApi(params);
      if (res) { message.success('导出成功'); }
    } catch { message.error('导出失败'); }
  };

  // 更多操作菜单
  const getMoreMenu = (record: any): MenuProps['items'] => {
    const items: MenuProps['items'] = [];
    items.push({ key: 'mark', label: '订单备注', onClick: () => openMark(record.orderId, record.remark) });
    items.push({ key: 'log', label: '订单记录', onClick: () => handleOrderLog(record.orderId) });
    if (record.status === 1) {
      items.push({ key: 'delivery', label: '发货', onClick: () => openDelivery(record.orderId) });
    }
    if (record.status === 2 || record.status === 3 || record.status === 4) {
      items.push({ key: 'logistics', label: '物流信息', onClick: () => handleLogistics(record.orderId) });
    }
    if (record.refundStatus === 1) {
      items.push({ key: 'refund', label: '立即退款', onClick: () => openRefund(record.orderId, record.payPrice) });
      items.push({ key: 'refuse', label: '拒绝退款', onClick: () => openRefuseRefund(record.orderId) });
    }
    return items;
  };

  const columns: ColumnsType<any> = [
    { title: '订单号', dataIndex: 'orderId', width: 200 },
    { title: '收货人', dataIndex: 'realName', width: 100 },
    {
      title: '商品信息', width: 250, ellipsis: true,
      render: (_: any, record: any) => {
        const productList = record.productList || [];
        if (productList.length === 0) return '-';
        const firstProduct = productList[0]?.info;
        if (!firstProduct) return '-';
        const productName = typeof firstProduct === 'string'
          ? JSON.parse(firstProduct)?.productName
          : firstProduct?.productName;
        return productName || '-';
      },
    },
    { title: '实付金额', dataIndex: 'payPrice', width: 100, render: (v: any) => `¥${v ?? 0}` },
    {
      title: '支付方式', dataIndex: 'payTypeStr', width: 100,
      render: (v: string) => v || '-',
    },
    {
      title: '订单状态', dataIndex: 'status', width: 100,
      render: (v: number, record: any) => {
        const statusStr = record.statusStr;
        if (statusStr?.value) {
          return <Tag color={statusMap[String(v)]?.color || 'default'}>{statusStr.value}</Tag>;
        }
        const s = statusMap[String(v)] || { text: '未知', color: 'default' };
        return <Tag color={s.color}>{s.text}</Tag>;
      },
    },
    {
      title: '订单类型', dataIndex: 'orderType', width: 100,
      render: (v: string) => v || '-',
    },
    { title: '创建时间', dataIndex: 'createTime', width: 170 },
    {
      title: '操作', width: 200, fixed: 'right' as const,
      render: (_: any, record: any) => (
        <Space size="small">
          <a onClick={() => handleDetail(record.orderId)}>详情</a>
          <Popconfirm title="确定删除此订单?" onConfirm={() => handleDelete(record.orderId)}>
            <a style={{ color: '#ff4d4f' }}>删除</a>
          </Popconfirm>
          <Dropdown menu={{ items: getMoreMenu(record) }}>
            <a>更多 <DownOutlined /></a>
          </Dropdown>
        </Space>
      ),
    },
  ];

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
      <Card bodyStyle={{ paddingBottom: 0 }}>
        <div style={{ display: 'flex', flexWrap: 'wrap', gap: 12, alignItems: 'center', marginBottom: 16 }}>
          <Select
            value={orderType}
            onChange={(v) => setOrderType(v)}
            options={orderTypeOptions}
            style={{ width: 140 }}
          />
          <Input placeholder="订单号" value={orderNo} onChange={(e) => setOrderNo(e.target.value)}
            onPressEnter={handleSearch} allowClear prefix={<SearchOutlined />} style={{ width: 200 }} />
          <RangePicker value={dateRange} onChange={(v) => setDateRange(v as any)} />
          <Button type="primary" icon={<SearchOutlined />} onClick={handleSearch}>搜索</Button>
          <Button icon={<ReloadOutlined />} onClick={handleReset}>重置</Button>
          <Button icon={<ExportOutlined />} onClick={handleExport}>导出</Button>
        </div>
      </Card>
      <Card>
        <Tabs activeKey={activeTab} onChange={handleTabChange}
          items={statusTabs.map((t) => ({
            key: t.key,
            label: `${t.label}${tabCounts[t.key] !== undefined ? ` (${tabCounts[t.key]})` : ''}`,
          }))} />
        <Table rowKey="id" columns={columns} dataSource={list} loading={loading} size="small" scroll={{ x: 1400 }}
          pagination={{ ...pagination, showSizeChanger: true, showTotal: (t) => `共 ${t} 条`,
            onChange: (p, ps) => { setPagination((prev) => ({ ...prev, pageSize: ps })); fetchList(p); } }} />
      </Card>

      {/* 订单详情抽屉 */}
      <Drawer
        title={null}
        placement="right"
        width={1000}
        open={detailOpen}
        onClose={() => { setDetailOpen(false); setDetailData(null); }}
        destroyOnClose
        styles={{ header: { display: 'none' }, body: { padding: 0 } }}
      >
        {detailData && (
          <div style={{ minHeight: '100%' }}>
            {/* 头部信息 */}
            <div style={{ padding: '24px 24px 24px 35px' }}>
              <div style={{ display: 'flex', alignItems: 'center' }}>
                <div style={{
                  width: 60, height: 60, borderRadius: 6, backgroundColor: '#1890ff',
                  display: 'flex', justifyContent: 'center', alignItems: 'center'
                }}>
                  <ShoppingOutlined style={{ color: '#fff', fontSize: 35 }} />
                </div>
                <div style={{ flex: 1, paddingLeft: 12 }}>
                  <div style={{ marginBottom: 10, fontWeight: 500, fontSize: 16, color: 'rgba(0,0,0,0.85)' }}>
                    {detailData.orderTypeText || '-'}
                  </div>
                  <div style={{ fontSize: 13, color: '#606266' }}>
                    <span style={{ marginRight: 20 }}>订单号：{detailData.orderId}</span>
                  </div>
                </div>
              </div>
              <ul style={{ display: 'flex', marginTop: 20, padding: 0, listStyle: 'none' }}>
                <li style={{ width: 200 }}>
                  <div style={{ marginBottom: 12, fontSize: 13, color: '#666' }}>订单状态</div>
                  <div style={{ fontSize: 14, color: '#f56022' }}>{detailData.statusStr?.value || '-'}</div>
                </li>
                <li style={{ width: 200 }}>
                  <div style={{ marginBottom: 12, fontSize: 13, color: '#666' }}>实际支付</div>
                  <div style={{ fontSize: 14, color: 'rgba(0,0,0,0.85)' }}>¥ {detailData.payPrice || '0.0'}</div>
                </li>
                <li style={{ width: 200 }}>
                  <div style={{ marginBottom: 12, fontSize: 13, color: '#666' }}>支付方式</div>
                  <div style={{ fontSize: 14, color: 'rgba(0,0,0,0.85)' }}>{payTypeFilter(detailData.payType)}</div>
                </li>
                <li style={{ width: 200 }}>
                  <div style={{ marginBottom: 12, fontSize: 13, color: '#666' }}>创建时间</div>
                  <div style={{ fontSize: 14, color: 'rgba(0,0,0,0.85)' }}>{detailData.createTime || '-'}</div>
                </li>
              </ul>
            </div>

            {/* Tabs */}
            <Tabs
              activeKey={detailTab}
              onChange={setDetailTab}
              type="card"
              style={{ margin: '0 20px' }}
              items={[
                {
                  key: 'detail',
                  label: '订单信息',
                  children: (
                    <div style={{ padding: '0 20px' }}>
                      {/* 用户信息 */}
                      <div style={{ padding: '25px 0', borderTop: '1px dashed #eee' }}>
                        <div style={{ paddingLeft: 10, borderLeft: '3px solid #1890ff', fontSize: 15, color: '#303133', marginBottom: 16 }}>用户信息</div>
                        <ul style={{ display: 'flex', flexWrap: 'wrap', padding: 0, listStyle: 'none', margin: 0 }}>
                          <li style={{ flex: '0 0 33.33%', display: 'flex', marginTop: 16, fontSize: 13, color: '#606266' }}>
                            <div style={{ width: 80 }}>用户名称：</div>
                            <div style={{ flex: 1, color: '#303133' }}>{detailData.nikeName} | {detailData.uid}</div>
                          </li>
                          <li style={{ flex: '0 0 33.33%', display: 'flex', marginTop: 16, fontSize: 13, color: '#606266' }}>
                            <div style={{ width: 80 }}>用户电话：</div>
                            <div style={{ flex: 1, color: '#303133' }}>{detailData.phone || '-'}</div>
                          </li>
                        </ul>
                      </div>

                      {/* 收货信息 */}
                      {detailData.shippingType < 2 && (
                        <div style={{ padding: '25px 0', borderTop: '1px dashed #eee' }}>
                          <div style={{ paddingLeft: 10, borderLeft: '3px solid #1890ff', fontSize: 15, color: '#303133', marginBottom: 16 }}>收货信息</div>
                          <ul style={{ display: 'flex', flexWrap: 'wrap', padding: 0, listStyle: 'none', margin: 0 }}>
                            <li style={{ flex: '0 0 33.33%', display: 'flex', marginTop: 16, fontSize: 13, color: '#606266' }}>
                              <div style={{ width: 80 }}>收货人：</div>
                              <div style={{ flex: 1, color: '#303133' }}>{detailData.realName || '-'}</div>
                            </li>
                            <li style={{ flex: '0 0 33.33%', display: 'flex', marginTop: 16, fontSize: 13, color: '#606266' }}>
                              <div style={{ width: 80 }}>收货电话：</div>
                              <div style={{ flex: 1, color: '#303133' }}>{detailData.userPhone || '-'}</div>
                            </li>
                            <li style={{ flex: '0 0 33.33%', display: 'flex', marginTop: 16, fontSize: 13, color: '#606266' }}>
                              <div style={{ width: 80 }}>收货地址：</div>
                              <div style={{ flex: 1, color: '#303133' }}>{detailData.userAddress || '-'}</div>
                            </li>
                          </ul>
                        </div>
                      )}

                      {/* 核销信息 */}
                      {detailData.shippingType === 2 && (
                        <div style={{ padding: '25px 0', borderTop: '1px dashed #eee' }}>
                          <div style={{ paddingLeft: 10, borderLeft: '3px solid #1890ff', fontSize: 15, color: '#303133', marginBottom: 16 }}>核销信息</div>
                          <ul style={{ display: 'flex', flexWrap: 'wrap', padding: 0, listStyle: 'none', margin: 0 }}>
                            <li style={{ flex: '0 0 33.33%', display: 'flex', marginTop: 16, fontSize: 13, color: '#606266' }}>
                              <div style={{ width: 80 }}>核销姓名：</div>
                              <div style={{ flex: 1, color: '#303133' }}>{detailData.realName || '-'}</div>
                            </li>
                            <li style={{ flex: '0 0 33.33%', display: 'flex', marginTop: 16, fontSize: 13, color: '#606266' }}>
                              <div style={{ width: 80 }}>核销电话：</div>
                              <div style={{ flex: 1, color: '#303133' }}>{detailData.userPhone || '-'}</div>
                            </li>
                          </ul>
                        </div>
                      )}

                      {/* 订单信息 */}
                      <div style={{ padding: '25px 0', borderTop: '1px dashed #eee' }}>
                        <div style={{ paddingLeft: 10, borderLeft: '3px solid #1890ff', fontSize: 15, color: '#303133', marginBottom: 16 }}>订单信息</div>
                        <ul style={{ display: 'flex', flexWrap: 'wrap', padding: 0, listStyle: 'none', margin: 0 }}>
                          <li style={{ flex: '0 0 33.33%', display: 'flex', marginTop: 16, fontSize: 13, color: '#606266' }}>
                            <div style={{ width: 80 }}>商品总价：</div>
                            <div style={{ flex: 1, color: '#303133' }}>{detailData.proTotalPrice}</div>
                          </li>
                          <li style={{ flex: '0 0 33.33%', display: 'flex', marginTop: 16, fontSize: 13, color: '#606266' }}>
                            <div style={{ width: 80 }}>商品总数：</div>
                            <div style={{ flex: 1, color: '#303133' }}>{detailData.totalNum}</div>
                          </li>
                          <li style={{ flex: '0 0 33.33%', display: 'flex', marginTop: 16, fontSize: 13, color: '#606266' }}>
                            <div style={{ width: 80 }}>优惠券：</div>
                            <div style={{ flex: 1, color: '#303133' }}>{detailData.couponPrice}</div>
                          </li>
                          <li style={{ flex: '0 0 33.33%', display: 'flex', marginTop: 16, fontSize: 13, color: '#606266' }}>
                            <div style={{ width: 80 }}>实际支付：</div>
                            <div style={{ flex: 1, color: '#303133' }}>{detailData.payPrice || '0.0'}</div>
                          </li>
                          <li style={{ flex: '0 0 33.33%', display: 'flex', marginTop: 16, fontSize: 13, color: '#606266' }}>
                            <div style={{ width: 80 }}>抵扣金额：</div>
                            <div style={{ flex: 1, color: '#303133' }}>{detailData.deductionPrice || '0.0'}</div>
                          </li>
                          <li style={{ flex: '0 0 33.33%', display: 'flex', marginTop: 16, fontSize: 13, color: '#606266' }}>
                            <div style={{ width: 80 }}>退款金额：</div>
                            <div style={{ flex: 1, color: '#303133' }}>{detailData.refundPrice || '0.0'}</div>
                          </li>
                          <li style={{ flex: '0 0 33.33%', display: 'flex', marginTop: 16, fontSize: 13, color: '#606266' }}>
                            <div style={{ width: 80 }}>支付邮费：</div>
                            <div style={{ flex: 1, color: '#303133' }}>{detailData.payPostage}</div>
                          </li>
                          <li style={{ flex: '0 0 33.33%', display: 'flex', marginTop: 16, fontSize: 13, color: '#606266' }}>
                            <div style={{ width: 80 }}>支付方式：</div>
                            <div style={{ flex: 1, color: '#303133' }}>{payTypeFilter(detailData.payType)}</div>
                          </li>
                          <li style={{ flex: '0 0 33.33%', display: 'flex', marginTop: 16, fontSize: 13, color: '#606266' }}>
                            <div style={{ width: 80 }}>创建时间：</div>
                            <div style={{ flex: 1, color: '#303133' }}>{detailData.createTime || '-'}</div>
                          </li>
                          <li style={{ flex: '0 0 33.33%', display: 'flex', marginTop: 16, fontSize: 13, color: '#606266' }}>
                            <div style={{ width: 80 }}>推广人：</div>
                            <div style={{ flex: 1, color: '#303133' }}>{detailData.spreadName || '-'}</div>
                          </li>
                        </ul>
                      </div>

                      {/* 买家留言 */}
                      <div style={{ padding: '25px 0', borderTop: '1px dashed #eee' }}>
                        <div style={{ paddingLeft: 10, borderLeft: '3px solid #1890ff', fontSize: 15, color: '#303133', marginBottom: 16 }}>买家留言</div>
                        <div style={{ fontSize: 13, color: '#606266', marginTop: 16 }}>{detailData.mark || '-'}</div>
                      </div>

                      {/* 商家备注 */}
                      <div style={{ padding: '25px 0', borderTop: '1px dashed #eee' }}>
                        <div style={{ paddingLeft: 10, borderLeft: '3px solid #1890ff', fontSize: 15, color: '#303133', marginBottom: 16 }}>商家备注</div>
                        <div style={{ fontSize: 13, color: '#606266', marginTop: 16 }}>{detailData.remark || '-'}</div>
                      </div>
                    </div>
                  ),
                },
                {
                  key: 'goods',
                  label: '商品信息',
                  children: (
                    <div style={{ padding: '20px' }}>
                      <Table
                        size="small"
                        pagination={false}
                        dataSource={detailData.orderInfo || []}
                        rowKey={(_, i) => String(i)}
                        columns={[
                          {
                            title: '商品信息',
                            width: 400,
                            render: (_, record: any) => {
                              const info = record.info || {};
                              return (
                                <div style={{ display: 'flex', alignItems: 'center' }}>
                                  <div style={{ marginRight: 15 }}>
                                    <Image src={info.image} width={50} height={50} style={{ borderRadius: 4 }} />
                                  </div>
                                  <div style={{ width: 300 }}>
                                    <div style={{ marginBottom: 10, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{info.productName}</div>
                                    <div style={{ color: '#909399', lineHeight: 1 }}>规格：{info.sku || '-'}</div>
                                  </div>
                                </div>
                              );
                            },
                          },
                          {
                            title: '商品售价',
                            width: 90,
                            render: (_, record: any) => record.info?.price || '-',
                          },
                          {
                            title: '购买数量',
                            width: 90,
                            render: (_, record: any) => record.info?.payNum || '-',
                          },
                        ]}
                      />
                    </div>
                  ),
                },
                ...(detailData.status > 0 ? [{
                  key: 'delivery',
                  label: '发货记录',
                  children: (
                    <div style={{ padding: '20px' }}>
                      <Table
                        size="small"
                        pagination={false}
                        dataSource={detailData.orderInfo || []}
                        rowKey={(_, i) => String(i)}
                        columns={[
                          {
                            title: () => (
                              <div>
                                {detailData.deliveryType === 'express' && (
                                  <>
                                    <span style={{ color: '#1890ff' }}>【快递配送】</span>
                                    <span>{detailData.deliveryName}：{detailData.deliveryId}</span>
                                    <span style={{ marginLeft: 30 }}>{detailData.createTime}</span>
                                  </>
                                )}
                                {detailData.deliveryType === 'send' && (
                                  <>
                                    <span style={{ color: '#1890ff' }}>【商家送货】</span>
                                    <span>{detailData.deliveryName}：{detailData.deliveryId}</span>
                                    <span style={{ marginLeft: 30 }}>{detailData.createTime}</span>
                                  </>
                                )}
                                {detailData.deliveryType !== 'express' && detailData.deliveryType !== 'send' && (
                                  <>
                                    <span style={{ color: '#1890ff' }}>【虚拟发货】</span>
                                    <span style={{ marginLeft: 30 }}>{detailData.createTime}</span>
                                  </>
                                )}
                              </div>
                            ),
                            width: 400,
                            render: (_, record: any) => {
                              const info = record.info || {};
                              return (
                                <div style={{ display: 'flex', alignItems: 'center' }}>
                                  <div style={{ marginRight: 15 }}>
                                    <Image src={info.image} width={50} height={50} style={{ borderRadius: 4 }} />
                                  </div>
                                  <div style={{ width: 300 }}>
                                    <div style={{ marginBottom: 10, lineHeight: 1 }}>{info.productName}</div>
                                    <div style={{ color: '#909399', lineHeight: 1 }}>规格：{info.sku || '-'}</div>
                                  </div>
                                  <div style={{ marginLeft: 30, fontSize: 12, color: '#606266' }}>
                                    X {info.payNum}
                                  </div>
                                </div>
                              );
                            },
                          },
                          {
                            title: () => (
                              <div style={{ display: 'flex', justifyContent: 'flex-end', marginRight: 10 }}>
                                {detailData.deliveryType === 'express' && (
                                  <a onClick={() => handleLogistics(detailData.orderId)}>查看物流</a>
                                )}
                              </div>
                            ),
                            width: 400,
                            render: (_, record: any) => {
                              if (detailData.deliveryType === 'noNeed') {
                                return <div style={{ fontSize: 12, color: '#606266' }}>发货备注：{detailData.deliveryMark || '-'}</div>;
                              }
                              return null;
                            },
                          },
                        ]}
                      />
                    </div>
                  ),
                }] : []),
              ]}
            />
          </div>
        )}
      </Drawer>

      {/* 物流信息弹窗 */}
      <Modal title="物流信息" open={logisticsOpen} onCancel={() => setLogisticsOpen(false)} onOk={() => setLogisticsOpen(false)} destroyOnClose width={500}>
        {logisticsData && (
          <div style={{ display: 'flex', alignItems: 'center', padding: '10px 0' }}>
            <div style={{ width: 45, height: 45, marginRight: 12 }}>
              {logisticsData.logo ? (
                <img src={logisticsData.logo} alt="" style={{ width: '100%', height: '100%' }} onError={(e) => { (e.target as HTMLImageElement).style.display = 'none'; }} />
              ) : (
                <img src="/expressi.jpg" alt="" style={{ width: '100%', height: '100%' }} onError={(e) => { (e.target as HTMLImageElement).style.display = 'none'; }} />
              )}
            </div>
            <div>
              <div style={{ display: 'block', fontSize: 12, marginBottom: 10 }}>物流公司：{logisticsData.expName || '-'}</div>
              <div style={{ display: 'block', fontSize: 12 }}>物流单号：{logisticsData.number || '-'}</div>
            </div>
          </div>
        )}
        {logisticsData?.list && logisticsData.list.length > 0 && (
          <div style={{ width: '100%', height: 400, borderRadius: 4, overflow: 'hidden' }}>
            <div style={{ width: '100%', height: '100%', overflow: 'auto', marginLeft: 18, padding: '10px 0', boxSizing: 'border-box' }}>
              <Timeline
                items={logisticsData.list.map((item: any) => ({
                  children: (
                    <>
                      <p style={{ fontSize: 12, color: '#2d8cf0' }}>{item.time}</p>
                      <p style={{ fontSize: 12 }}>{item.status}</p>
                    </>
                  ),
                }))}
              />
            </div>
          </div>
        )}
        {(!logisticsData || !logisticsData.list || logisticsData.list.length === 0) && (
          <div style={{ textAlign: 'center', padding: 20, color: '#999' }}>暂无物流信息</div>
        )}
      </Modal>
      {/* 发货弹窗 */}
      <Modal title="订单发货" open={deliveryOpen} onCancel={() => setDeliveryOpen(false)} onOk={handleDelivery} destroyOnClose>
        <Form form={deliveryForm} layout="vertical">
          <Form.Item name="deliveryName" label="快递公司" rules={[{ required: true, message: '请输入快递公司' }]}>
            <Input placeholder="请输入快递公司" />
          </Form.Item>
          <Form.Item name="deliveryId" label="快递单号" rules={[{ required: true, message: '请输入快递单号' }]}>
            <Input placeholder="请输入快递单号" />
          </Form.Item>
        </Form>
      </Modal>

      {/* 备注弹窗 */}
      <Modal title="订单备注" open={markOpen} onCancel={() => setMarkOpen(false)} onOk={handleMark} destroyOnClose>
        <TextArea placeholder="请输入备注内容" rows={4} value={markContent} onChange={(e) => setMarkContent(e.target.value)} />
      </Modal>

      {/* 拒绝退款弹窗 */}
      <Modal title="拒绝退款原因" open={refuseOpen} onCancel={() => setRefuseOpen(false)} onOk={handleRefuseRefund} destroyOnClose>
        <TextArea placeholder="请输入拒绝退款原因" rows={4} value={refuseReason} onChange={(e) => setRefuseReason(e.target.value)} />
      </Modal>

      {/* 退款弹窗 */}
      <Modal title="订单退款" open={refundOpen} onCancel={() => setRefundOpen(false)} onOk={handleRefund} destroyOnClose>
        <Form layout="vertical">
          <Form.Item label="退款金额" required>
            <InputNumber
              style={{ width: '100%' }}
              placeholder="请输入退款金额"
              min={0}
              precision={2}
              value={refundAmount ? parseFloat(refundAmount) : undefined}
              onChange={(v) => setRefundAmount(v ? String(v) : '')}
              prefix="¥"
            />
          </Form.Item>
        </Form>
      </Modal>

      {/* 订单记录弹窗 */}
      <Modal title="操作记录" open={logOpen} onCancel={() => setLogOpen(false)} footer={null} destroyOnClose width={700}>
        <Table
          size="small"
          bordered
          dataSource={logList}
          rowKey={(_, i) => String(i)}
          pagination={{
            pageSize: 10,
            showSizeChanger: true,
            pageSizeOptions: ['10', '20', '30', '40'],
            showTotal: (total) => `共 ${total} 条`,
          }}
          columns={[
            { title: 'ID', dataIndex: 'oid', width: 80 },
            { title: '操作记录', dataIndex: 'changeMessage', width: 280 },
            { title: '操作时间', dataIndex: 'createTime', width: 180 },
          ]}
        />
      </Modal>
    </div>
  );
};

export default Order;
