import React, { useEffect, useState } from 'react';
import { Card, Col, Row, Statistic, Spin, Segmented, DatePicker, Tag } from 'antd';
import {
  DollarOutlined, EyeOutlined, ShoppingCartOutlined, UserAddOutlined,
  ShopOutlined, SettingOutlined, OrderedListOutlined, UserOutlined,
  GiftOutlined, InboxOutlined, AlertOutlined, MoneyCollectOutlined,
  FileTextOutlined, ShareAltOutlined, ArrowUpOutlined, ArrowDownOutlined,
} from '@ant-design/icons';
import { useNavigate } from 'react-router-dom';
import EChartsComponent from '@/components/ECharts';
import {
  viewModelApi, chartUserApi, chartOrder30Api, chartOrderWeekApi,
  chartOrderMonthApi, chartOrderYearApi, businessData,
} from '@/api/dashboard';
import { userOverviewData, userChannelData } from '@/api/statistic';
import dayjs from 'dayjs';

const { RangePicker } = DatePicker;

const orderChartApis: Record<string, () => Promise<any>> = {
  '30天': chartOrder30Api, '周': chartOrderWeekApi,
  '月': chartOrderMonthApi, '年': chartOrderYearApi,
};

// 快捷入口配置
const quickEntries = [
  { title: '用户管理', icon: <UserOutlined style={{ fontSize: 24, color: '#1890ff' }} />, path: '/user/index', bg: '#e6f7ff' },
  { title: '系统设置', icon: <SettingOutlined style={{ fontSize: 24, color: '#722ed1' }} />, path: '/operation/config', bg: '#f9f0ff' },
  { title: '商品管理', icon: <ShopOutlined style={{ fontSize: 24, color: '#52c41a' }} />, path: '/store/index', bg: '#f6ffed' },
  { title: '订单管理', icon: <OrderedListOutlined style={{ fontSize: 24, color: '#fa8c16' }} />, path: '/order/index', bg: '#fff7e6' },
  { title: '文章管理', icon: <FileTextOutlined style={{ fontSize: 24, color: '#13c2c2' }} />, path: '/content/article', bg: '#e6fffb' },
  { title: '分销管理', icon: <ShareAltOutlined style={{ fontSize: 24, color: '#eb2f96' }} />, path: '/distribution/index', bg: '#fff0f6' },
  { title: '优惠券', icon: <GiftOutlined style={{ fontSize: 24, color: '#f5222d' }} />, path: '/marketing/coupon/list', bg: '#fff1f0' },
  { title: '充值记录', icon: <MoneyCollectOutlined style={{ fontSize: 24, color: '#2f54eb' }} />, path: '/financial/record/charge', bg: '#f0f5ff' },
];

// 经营数据配置
const bizMetrics = [
  { title: '待发货订单', key: 'notShippingOrderNum', icon: <InboxOutlined style={{ color: '#1890ff' }} />, path: '/order/index' },
  { title: '退款中订单', key: 'refundingOrderNum', icon: <AlertOutlined style={{ color: '#ff4d4f' }} />, path: '/order/index' },
  { title: '待核销订单', key: 'notWriteOffOrderNum', icon: <OrderedListOutlined style={{ color: '#fa8c16' }} />, path: '/order/index' },
  { title: '库存预警', key: 'vigilanceInventoryNum', icon: <AlertOutlined style={{ color: '#ff4d4f' }} />, path: '/store/index' },
  { title: '在售商品', key: 'onSaleProductNum', icon: <ShopOutlined style={{ color: '#52c41a' }} />, path: '/store/index' },
  { title: '仓库商品', key: 'notSaleProductNum', icon: <ShopOutlined style={{ color: '#8c8c8c' }} />, path: '/store/index' },
  { title: '待审核提现', key: 'notAuditNum', icon: <MoneyCollectOutlined style={{ color: '#722ed1' }} />, path: '/financial/commission/template' },
  { title: '账户充值金额', key: 'totalRechargeAmount', icon: <DollarOutlined style={{ color: '#2f54eb' }} />, path: '/financial/record/charge' },
];

const Dashboard: React.FC = () => {
  const navigate = useNavigate();
  const [loading, setLoading] = useState(true);
  const [baseInfo, setBaseInfo] = useState<any>({});
  const [bizData, setBizData] = useState<any>({});
  const [userOverview, setUserOverview] = useState<any>({});
  const [userChart, setUserChart] = useState<any>({ x: [], series: [] });
  const [orderChart, setOrderChart] = useState<any>({ x: [], amountSeries: [], countSeries: [] });
  const [orderPeriod, setOrderPeriod] = useState<string>('30天');
  const [channelData, setChannelData] = useState<any[]>([]);
  const [dateRange, setDateRange] = useState<[dayjs.Dayjs, dayjs.Dayjs]>([dayjs().subtract(7, 'day'), dayjs()]);

  useEffect(() => {
    (async () => {
      setLoading(true);
      try {
        const [info, biz, uc, oc, ch] = await Promise.all([
          viewModelApi().catch(() => ({})),
          businessData().catch(() => ({})),
          chartUserApi().catch(() => ({})),
          chartOrder30Api().catch(() => ({})),
          userChannelData().catch(() => ([])),
        ]);
        setBaseInfo(info || {});
        setBizData(biz || {});
        if (uc) setUserChart({ x: uc.xAxis || [], series: uc.yAxis || uc.series || [] });
        if (oc) setOrderChart({ x: oc.xAxis || [], amountSeries: oc.amountList || oc.yAxis || [], countSeries: oc.countList || [] });
        if (Array.isArray(ch)) setChannelData(ch);
      } finally { setLoading(false); }
    })();
  }, []);

  // 获取用户概览
  const fetchUserOverview = async (start: dayjs.Dayjs, end: dayjs.Dayjs) => {
    try {
      const dateLimit = `${start.format('YYYY/MM/DD')}-${end.format('YYYY/MM/DD')}`;
      const res = await userOverviewData({ dateLimit });
      setUserOverview(res || {});
    } catch { /* */ }
  };

  useEffect(() => { fetchUserOverview(dateRange[0], dateRange[1]); }, []);

  const handleDateChange = (dates: any) => {
    if (dates && dates[0] && dates[1]) {
      setDateRange(dates);
      fetchUserOverview(dates[0], dates[1]);
    }
  };

  const handleQuickDate = (days: number) => {
    const range: [dayjs.Dayjs, dayjs.Dayjs] = [dayjs().subtract(days, 'day'), dayjs()];
    setDateRange(range);
    fetchUserOverview(range[0], range[1]);
  };

  const handleOrderPeriod = async (val: string) => {
    setOrderPeriod(val as string);
    const api = orderChartApis[val as string];
    if (!api) return;
    try {
      const oc = await api();
      if (oc) setOrderChart({ x: oc.xAxis || [], amountSeries: oc.amountList || oc.yAxis || [], countSeries: oc.countList || [] });
    } catch { /* */ }
  };

  // 比率渲染
  const renderRatio = (ratio: string | number | undefined) => {
    if (!ratio && ratio !== 0) return null;
    const val = typeof ratio === 'string' ? parseFloat(ratio) : ratio;
    if (isNaN(val)) return <span style={{ fontSize: 12, color: '#999' }}>{ratio}</span>;
    const color = val >= 0 ? '#52c41a' : '#ff4d4f';
    const icon = val >= 0 ? <ArrowUpOutlined /> : <ArrowDownOutlined />;
    return <span style={{ fontSize: 12, color }}>{icon} {Math.abs(val)}%</span>;
  };

  // ECharts 配置
  const orderOption: any = {
    tooltip: { trigger: 'axis' },
    legend: { data: ['订单金额', '订单数'] },
    xAxis: { type: 'category', data: orderChart.x },
    yAxis: [{ type: 'value', name: '金额' }, { type: 'value', name: '数量' }],
    series: [
      { name: '订单金额', type: 'bar', data: orderChart.amountSeries, itemStyle: { color: '#0256FF' } },
      { name: '订单数', type: 'line', yAxisIndex: 1, data: orderChart.countSeries, itemStyle: { color: '#ff9900' } },
    ],
  };

  const userChartOption: any = {
    tooltip: { trigger: 'axis' },
    xAxis: { type: 'category', data: userChart.x },
    yAxis: { type: 'value' },
    series: [{ name: '用户数', type: 'line', data: userChart.series, areaStyle: { color: 'rgba(2,86,255,0.1)' }, itemStyle: { color: '#0256FF' } }],
  };

  const channelOption: any = {
    tooltip: { trigger: 'item' },
    legend: { bottom: 0 },
    series: [{
      type: 'pie', radius: ['40%', '70%'],
      data: channelData.map((c: any) => ({ name: c.name || c.channel, value: c.value || c.count })),
    }],
  };

  return (
    <Spin spinning={loading}>
      <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
        {/* 顶部数据卡片 - 今日/昨日对比 */}
        <Row gutter={[16, 16]}>
          {[
            { title: '今日销售额', value: baseInfo.sales, yesterday: baseInfo.yesterdaySales, prefix: '¥', precision: 2, icon: <DollarOutlined />, color: '#1890ff' },
            { title: '今日访问量', value: baseInfo.pageviews, yesterday: baseInfo.yesterdayPageviews, icon: <EyeOutlined />, color: '#52c41a' },
            { title: '今日订单量', value: baseInfo.orderNum, yesterday: baseInfo.yesterdayOrderNum, icon: <ShoppingCartOutlined />, color: '#fa8c16' },
            { title: '今日新增用户', value: baseInfo.newUserNum, yesterday: baseInfo.yesterdayNewUserNum, icon: <UserAddOutlined />, color: '#722ed1' },
          ].map((item, idx) => (
            <Col xs={24} sm={12} lg={6} key={idx}>
              <Card hoverable>
                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start' }}>
                  <div>
                    <div style={{ color: '#999', fontSize: 14, marginBottom: 8 }}>{item.title}</div>
                    <div style={{ fontSize: 28, fontWeight: 600 }}>
                      {item.prefix || ''}{item.value ?? 0}
                    </div>
                    <div style={{ color: '#999', fontSize: 12, marginTop: 8 }}>
                      昨日: {item.prefix || ''}{item.yesterday ?? 0}
                    </div>
                  </div>
                  <div style={{ fontSize: 40, color: item.color, opacity: 0.2 }}>{item.icon}</div>
                </div>
              </Card>
            </Col>
          ))}
        </Row>

        {/* 快捷入口 */}
        <Card title="快捷入口" size="small">
          <Row gutter={[16, 16]}>
            {quickEntries.map((entry, idx) => (
              <Col xs={12} sm={8} md={6} lg={3} key={idx}>
                <div onClick={() => navigate(entry.path)}
                  style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', gap: 8,
                    padding: '16px 0', borderRadius: 8, cursor: 'pointer', background: entry.bg,
                    transition: 'transform 0.2s' }}
                  onMouseEnter={(e) => (e.currentTarget.style.transform = 'scale(1.05)')}
                  onMouseLeave={(e) => (e.currentTarget.style.transform = 'scale(1)')}>
                  {entry.icon}
                  <span style={{ fontSize: 13 }}>{entry.title}</span>
                </div>
              </Col>
            ))}
          </Row>
        </Card>

        {/* 经营数据 */}
        <Card title="经营数据" size="small">
          <Row gutter={[16, 16]}>
            {bizMetrics.map((m, idx) => (
              <Col xs={12} sm={8} md={6} lg={3} key={idx}>
                <div onClick={() => navigate(m.path)}
                  style={{ display: 'flex', alignItems: 'center', gap: 12, padding: 12,
                    borderRadius: 8, cursor: 'pointer', background: '#fafafa' }}>
                  <div style={{ fontSize: 28 }}>{m.icon}</div>
                  <div>
                    <div style={{ fontSize: 20, fontWeight: 600 }}>{bizData[m.key] ?? 0}</div>
                    <div style={{ color: '#999', fontSize: 12 }}>{m.title}</div>
                  </div>
                </div>
              </Col>
            ))}
          </Row>
        </Card>

        {/* 用户概览 */}
        <Card title="用户概览" size="small"
          extra={
            <div style={{ display: 'flex', gap: 8, alignItems: 'center', flexWrap: 'wrap' }}>
              {[{ label: '近7天', days: 7 }, { label: '近30天', days: 30 }, { label: '本月', days: 0 }].map((btn) => (
                <Tag key={btn.label} color="blue" style={{ cursor: 'pointer' }}
                  onClick={() => {
                    if (btn.days > 0) handleQuickDate(btn.days);
                    else {
                      const range: [dayjs.Dayjs, dayjs.Dayjs] = [dayjs().startOf('month'), dayjs()];
                      setDateRange(range);
                      fetchUserOverview(range[0], range[1]);
                    }
                  }}>{btn.label}</Tag>
              ))}
              <RangePicker size="small" value={dateRange} onChange={handleDateChange} />
            </div>
          }>
          <Row gutter={[16, 16]}>
            <Col xs={24} lg={16}>
              <Row gutter={[12, 12]}>
                {[
                  { title: '新增用户', value: userOverview.registerNum, ratio: userOverview.registerNumRatio },
                  { title: '活跃用户', value: userOverview.activeUserNum, ratio: userOverview.activeUserNumRatio },
                  { title: '充值用户', value: userOverview.rechargeUserNum, ratio: userOverview.rechargeUserNumRatio },
                  { title: '访客量', value: userOverview.pageviews },
                  { title: '下单用户', value: userOverview.orderUserNum },
                  { title: '付费用户', value: userOverview.orderPayUserNum },
                  { title: '付费金额', value: userOverview.payOrderAmount, prefix: '¥' },
                  { title: '客单价', value: userOverview.customerPrice, prefix: '¥' },
                ].map((item, idx) => (
                  <Col xs={12} sm={8} md={6} key={idx}>
                    <div style={{ padding: 12, background: '#fafafa', borderRadius: 8 }}>
                      <div style={{ color: '#999', fontSize: 12, marginBottom: 4 }}>{item.title}</div>
                      <div style={{ fontSize: 20, fontWeight: 600 }}>
                        {item.prefix || ''}{item.value ?? 0}
                      </div>
                      {item.ratio !== undefined && <div style={{ marginTop: 4 }}>{renderRatio(item.ratio)}</div>}
                    </div>
                  </Col>
                ))}
              </Row>
            </Col>
            <Col xs={24} lg={8}>
              <div style={{ textAlign: 'center', fontWeight: 500, marginBottom: 8 }}>用户渠道分布</div>
              <EChartsComponent option={channelOption} style={{ height: 260 }} />
            </Col>
          </Row>
        </Card>

        {/* 订单统计 */}
        <Card title="订单统计" size="small"
          extra={<Segmented options={['30天', '周', '月', '年']} value={orderPeriod} onChange={handleOrderPeriod} />}>
          <EChartsComponent option={orderOption} style={{ height: 350 }} />
        </Card>

        {/* 用户趋势 */}
        <Card title="用户增长趋势" size="small">
          <EChartsComponent option={userChartOption} style={{ height: 350 }} />
        </Card>
      </div>
    </Spin>
  );
};

export default Dashboard;
