import React, { useState, useEffect, useCallback } from 'react';
import { Card, Row, Col, Statistic, Spin, DatePicker } from 'antd';
import { UserOutlined, ThunderboltOutlined, DollarOutlined, UserAddOutlined } from '@ant-design/icons';
import EChartsComponent from '@/components/ECharts';
import { userTotalData, userChannelData, userOverviewData } from '@/api/statistic';
import dayjs from 'dayjs';

const { RangePicker } = DatePicker;

const StatisticUser: React.FC = () => {
  const [loading, setLoading] = useState(true);
  const [totalData, setTotalData] = useState<any>({});
  const [channelList, setChannelList] = useState<any[]>([]);
  const [overviewData, setOverviewData] = useState<any>({ x: [], newUser: [], activeUser: [] });
  const [dateRange, setDateRange] = useState<[string, string]>([
    dayjs().subtract(30, 'day').format('YYYY-MM-DD'),
    dayjs().format('YYYY-MM-DD'),
  ]);

  const fetchData = useCallback(async () => {
    setLoading(true);
    try {
      const [total, channel, overview] = await Promise.all([
        userTotalData().catch(() => ({})),
        userChannelData().catch(() => ([])),
        userOverviewData({ dateLimit: dateRange.join(',') }).catch(() => ({})),
      ]);
      setTotalData(total || {});
      setChannelList(Array.isArray(channel) ? channel : []);
      if (overview) {
        setOverviewData({
          x: overview.xAxis || overview.dates || [],
          newUser: overview.newUserList || overview.newUser || [],
          activeUser: overview.activeUserList || overview.activeUser || [],
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
    legend: { data: ['新增用户', '活跃用户'] },
    xAxis: { type: 'category', data: overviewData.x },
    yAxis: { type: 'value' },
    series: [
      { name: '新增用户', type: 'line', data: overviewData.newUser, smooth: true, areaStyle: { color: 'rgba(24,144,255,0.1)' }, itemStyle: { color: '#1890ff' } },
      { name: '活跃用户', type: 'line', data: overviewData.activeUser, smooth: true, areaStyle: { color: 'rgba(82,196,26,0.1)' }, itemStyle: { color: '#52c41a' } },
    ],
  };

  const channelOption: any = {
    tooltip: { trigger: 'item', formatter: '{b}: {c} ({d}%)' },
    legend: { bottom: 0 },
    series: [{
      type: 'pie', radius: ['40%', '70%'],
      data: channelList.map((c: any) => ({ name: c.name || c.channel, value: c.value || c.count || 0 })),
      emphasis: { itemStyle: { shadowBlur: 10, shadowOffsetX: 0, shadowColor: 'rgba(0,0,0,0.5)' } },
    }],
  };

  const statCards = [
    { title: '累计用户', value: totalData.totalUser ?? totalData.total ?? 0, icon: <UserOutlined />, color: '#1890ff' },
    { title: '活跃用户', value: totalData.activeUser ?? totalData.active ?? 0, icon: <ThunderboltOutlined />, color: '#52c41a' },
    { title: '付费用户', value: totalData.payUser ?? totalData.pay ?? 0, icon: <DollarOutlined />, color: '#faad14' },
    { title: '新增用户', value: totalData.newUser ?? totalData.newCount ?? 0, icon: <UserAddOutlined />, color: '#eb2f96' },
  ];

  return (
    <Spin spinning={loading}>
      <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
        <Row gutter={[16, 16]}>
          {statCards.map((item) => (
            <Col xs={24} sm={12} lg={6} key={item.title}>
              <Card hoverable>
                <Statistic title={item.title} value={item.value} prefix={item.icon} valueStyle={{ color: item.color }} />
              </Card>
            </Col>
          ))}
        </Row>
        <Card title="用户趋势" extra={<RangePicker onChange={handleDateChange} defaultValue={[dayjs(dateRange[0]), dayjs(dateRange[1])]} />}>
          <EChartsComponent option={trendOption} style={{ height: 350 }} />
        </Card>
        <Card title="用户渠道">
          <EChartsComponent option={channelOption} style={{ height: 350 }} />
        </Card>
      </div>
    </Spin>
  );
};

export default StatisticUser;
