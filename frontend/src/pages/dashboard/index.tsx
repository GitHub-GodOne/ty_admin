import React, { useEffect, useState } from 'react';
import { Card, Col, Row, Statistic, Spin, Segmented, Space } from 'antd';
import {
  DollarOutlined, EyeOutlined, ShoppingCartOutlined, UserAddOutlined,
  ShopOutlined, SettingOutlined, OrderedListOutlined, UserOutlined,
  GiftOutlined, AccountBookOutlined, AlertOutlined, InboxOutlined,
} from '@ant-design/icons';
import EChartsComponent from '@/components/ECharts';
import { viewModelApi, chartUserApi, chartOrder30Api, chartOrderWeekApi, chartOrderMonthApi, chartOrderYearApi, businessData } from '@/api/dashboard';
import { userOverviewData, userChannelData } from '@/api/statistic';

const orderChartApis: Record<string, () => Promise<any>> = {
  '30天': chartOrder30Api,
  '周': chartOrderWeekApi,
  '月': chartOrderMonthApi,
  '年': chartOrderYearApi,
};

const Dashboard: React.FC = () => {
  const [loading, setLoading] = useState(true);
  const [baseInfo, setBaseInfo] = useState<any>({});
  const [bizData, setBizData] = useState<any>({});
  const [userChart, setUserChart] = useState<any>({ x: [], series: [] });
  const [orderChart, setOrderChart] = useState<any>({ x: [], amountSeries: [], countSeries: [] });
  const [orderPeriod, setOrderPeriod] = useState<string>('30天');
  const [channelData, setChannelData] = useState<any[]>([]);

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
      } finally {
        setLoading(false);
      }
    })();
  }, []);

  const handleOrderPeriod = async (val: string) => {
    setOrderPeriod(val as string);
    const api = orderChartApis[val as string];
    if (!api) return;
    try {
      const oc = await api();
      if (oc) setOrderChart({ x: oc.xAxis || [], amountSeries: oc.amountList || oc.yAxis || [], countSeries: oc.countList || [] });
    } catch { /* noop */ }
  };

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

  const userOption: any = {
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
        {/* Base Info Cards */}
        <Row gutter={[16, 16]}>
          <Col xs={24} sm={12} lg={6}>
            <Card hoverable><Statistic title="今日销售额" value={baseInfo.todaySales ?? 0} prefix={<DollarOutlined />} precision={2} /></Card>
          </Col>
          <Col xs={24} sm={12} lg={6}>
            <Card hoverable><Statistic title="用户访问量" value={baseInfo.todayVisits ?? 0} prefix={<EyeOutlined />} /></Card>
          </Col>
          <Col xs={24} sm={12} lg={6}>
            <Card hoverable><Statistic title="订单量" value={baseInfo.todayOrders ?? 0} prefix={<ShoppingCartOutlined />} /></Card>
          </Col>
          <Col xs={24} sm={12} lg={6}>
            <Card hoverable><Statistic title="新增用户" value={baseInfo.todayNewUsers ?? 0} prefix={<UserAddOutlined />} /></Card>
          </Col>
        </Row>

        {/* Business Data */}
        <Card title="业务数据">
          <Row gutter={[16, 16]}>
            {[
              { label: '待发货订单', key: 'notShippingOrderNum', icon: <InboxOutlined /> },
              { label: '退款中订单', key: 'refundingOrderNum', icon: <AlertOutlined /> },
              { label: '在售商品', key: 'onSaleProductNum', icon: <ShopOutlined /> },
              { label: '库存预警', key: 'stockWarningNum', icon: <AlertOutlined style={{ color: '#ff4d4f' }} /> },
            ].map((item) => (
              <Col xs={12} sm={6} key={item.key}>
                <Statistic title={item.label} value={bizData[item.key] ?? 0} prefix={item.icon} />
              </Col>
            ))}
          </Row>
        </Card>

        {/* Order Chart */}
        <Card title="订单统计" extra={
          <Segmented options={['30天', '周', '月', '年']} value={orderPeriod} onChange={handleOrderPeriod} />
        }>
          <EChartsComponent option={orderOption} style={{ height: 350 }} />
        </Card>

        {/* User Chart + Channel */}
        <Row gutter={16}>
          <Col xs={24} lg={16}>
            <Card title="用户趋势">
              <EChartsComponent option={userOption} style={{ height: 300 }} />
            </Card>
          </Col>
          <Col xs={24} lg={8}>
            <Card title="用户渠道">
              <EChartsComponent option={channelOption} style={{ height: 300 }} />
            </Card>
          </Col>
        </Row>
      </div>
    </Spin>
  );
};

export default Dashboard;
