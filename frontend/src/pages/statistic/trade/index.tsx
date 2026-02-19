import React, { useState, useEffect, useCallback } from 'react';
import { Card, Row, Col, Statistic, Spin, DatePicker } from 'antd';
import { DollarOutlined, ShoppingCartOutlined, AccountBookOutlined, RollbackOutlined } from '@ant-design/icons';
import EChartsComponent from '@/components/ECharts';
import { tradeDataApi, tradeOverviewApi, tradeTrendApi } from '@/api/statistic';
import dayjs from 'dayjs';

const { RangePicker } = DatePicker;

const StatisticTrade: React.FC = () => {
  const [loading, setLoading] = useState(true);
  const [statData, setStatData] = useState<any>({});
  const [trendData, setTrendData] = useState<any>({ x: [], amount: [], count: [] });
  const [dateRange, setDateRange] = useState<[string, string]>([
    dayjs().subtract(30, 'day').format('YYYY-MM-DD'),
    dayjs().format('YYYY-MM-DD'),
  ]);

  const fetchData = useCallback(async () => {
    setLoading(true);
    try {
      const [data, trend] = await Promise.all([
        tradeDataApi().catch(() => ({})),
        tradeTrendApi({ dateLimit: dateRange.join(',') }).catch(() => ({})),
      ]);
      setStatData(data || {});
      if (trend) {
        setTrendData({
          x: trend.xAxis || trend.dates || [],
          amount: trend.amountList || trend.amount || [],
          count: trend.countList || trend.count || [],
        });
      }
    } finally { setLoading(false); }
  }, [dateRange]);

  useEffect(() => { fetchData(); }, [fetchData]);

  const handleDateChange = (dates: any) => {
    if (dates) {
      setDateRange([dates[0].format('YYYY-MM-DD'), dates[1].format('YYYY-MM-DD')]);
    }
  };

  const trendOption: any = {
    tooltip: { trigger: 'axis' },
    legend: { data: ['营业额', '订单量'] },
    xAxis: { type: 'category', data: trendData.x },
    yAxis: [
      { type: 'value', name: '金额(元)' },
      { type: 'value', name: '订单量' },
    ],
    series: [
      { name: '营业额', type: 'line', data: trendData.amount, smooth: true, areaStyle: { color: 'rgba(24,144,255,0.15)' }, itemStyle: { color: '#1890ff' } },
      { name: '订单量', type: 'line', yAxisIndex: 1, data: trendData.count, smooth: true, itemStyle: { color: '#faad14' } },
    ],
  };

  const statCards = [
    { title: '营业额', value: statData.turnover ?? statData.totalAmount ?? 0, icon: <DollarOutlined />, color: '#1890ff', prefix: '¥', precision: 2 },
    { title: '订单量', value: statData.orderCount ?? statData.totalOrder ?? 0, icon: <ShoppingCartOutlined />, color: '#52c41a' },
    { title: '客单价', value: statData.avgPrice ?? statData.perOrder ?? 0, icon: <AccountBookOutlined />, color: '#faad14', prefix: '¥', precision: 2 },
    { title: '退款金额', value: statData.refundAmount ?? statData.refund ?? 0, icon: <RollbackOutlined />, color: '#ff4d4f', prefix: '¥', precision: 2 },
  ];

  return (
    <Spin spinning={loading}>
      <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
        <Row gutter={[16, 16]}>
          {statCards.map((item) => (
            <Col xs={24} sm={12} lg={6} key={item.title}>
              <Card hoverable>
                <Statistic
                  title={item.title}
                  value={item.value}
                  prefix={item.icon}
                  suffix={item.prefix ? undefined : undefined}
                  precision={item.precision}
                  valueStyle={{ color: item.color }}
                />
              </Card>
            </Col>
          ))}
        </Row>
        <Card title="交易趋势" extra={<RangePicker onChange={handleDateChange} defaultValue={[dayjs(dateRange[0]), dayjs(dateRange[1])]} />}>
          <EChartsComponent option={trendOption} style={{ height: 400 }} />
        </Card>
      </div>
    </Spin>
  );
};

export default StatisticTrade;
