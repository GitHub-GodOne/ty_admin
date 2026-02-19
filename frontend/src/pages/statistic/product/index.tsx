import React, { useState, useEffect, useCallback } from 'react';
import { Card, Row, Col, Table, Statistic, Spin, DatePicker } from 'antd';
import { EyeOutlined, HeartOutlined, ShoppingCartOutlined, OrderedListOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import EChartsComponent from '@/components/ECharts';
import { productDataApi, productRankApi, productTrendApi } from '@/api/statistic';
import dayjs from 'dayjs';

const { RangePicker } = DatePicker;

const StatisticProduct: React.FC = () => {
  const [loading, setLoading] = useState(true);
  const [statData, setStatData] = useState<any>({});
  const [trendData, setTrendData] = useState<any>({ x: [], views: [], collects: [], carts: [], orders: [] });
  const [rankList, setRankList] = useState<any[]>([]);
  const [dateRange, setDateRange] = useState<[string, string]>([
    dayjs().subtract(30, 'day').format('YYYY-MM-DD'),
    dayjs().format('YYYY-MM-DD'),
  ]);

  const fetchData = useCallback(async () => {
    setLoading(true);
    try {
      const [data, trend, rank] = await Promise.all([
        productDataApi({}).catch(() => ({})),
        productTrendApi({ dateLimit: dateRange.join(',') }).catch(() => ({})),
        productRankApi({}).catch(() => ([])),
      ]);
      setStatData(data || {});
      if (trend) {
        setTrendData({
          x: trend.xAxis || trend.dates || [],
          views: trend.pageviews || trend.views || [],
          collects: trend.collectNum || trend.collects || [],
          carts: trend.addCartNum || trend.carts || [],
          orders: trend.orderNum || trend.orders || [],
        });
      }
      setRankList(Array.isArray(rank) ? rank : rank?.list || []);
    } finally { setLoading(false); }
  }, [dateRange]);

  useEffect(() => { fetchData(); }, [fetchData]);

  const trendOption: any = {
    tooltip: { trigger: 'axis' },
    legend: { data: ['浏览量', '收藏量', '加购量', '下单量'] },
    xAxis: { type: 'category', data: trendData.x },
    yAxis: { type: 'value' },
    series: [
      { name: '浏览量', type: 'line', data: trendData.views, smooth: true },
      { name: '收藏量', type: 'line', data: trendData.collects, smooth: true },
      { name: '加购量', type: 'line', data: trendData.carts, smooth: true },
      { name: '下单量', type: 'line', data: trendData.orders, smooth: true },
    ],
  };

  const rankColumns: ColumnsType<any> = [
    { title: '排名', width: 60, render: (_: any, __: any, i: number) => i + 1 },
    { title: '商品名称', dataIndex: 'storeName', ellipsis: true },
    { title: '浏览量', dataIndex: 'pageviews', width: 100 },
    { title: '收藏量', dataIndex: 'collectNum', width: 100 },
    { title: '加购量', dataIndex: 'addCartNum', width: 100 },
    { title: '下单量', dataIndex: 'orderNum', width: 100 },
  ];

  const handleDateChange = (dates: any) => {
    if (dates) {
      setDateRange([dates[0].format('YYYY-MM-DD'), dates[1].format('YYYY-MM-DD')]);
    }
  };

  const statCards = [
    { title: '浏览量', value: statData.pageviews ?? statData.views ?? 0, icon: <EyeOutlined />, color: '#1890ff' },
    { title: '收藏量', value: statData.collectNum ?? statData.collects ?? 0, icon: <HeartOutlined />, color: '#eb2f96' },
    { title: '加购量', value: statData.addCartNum ?? statData.carts ?? 0, icon: <ShoppingCartOutlined />, color: '#faad14' },
    { title: '下单量', value: statData.orderNum ?? statData.orders ?? 0, icon: <OrderedListOutlined />, color: '#52c41a' },
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
        <Card title="商品趋势" extra={<RangePicker onChange={handleDateChange} defaultValue={[dayjs(dateRange[0]), dayjs(dateRange[1])]} />}>
          <EChartsComponent option={trendOption} style={{ height: 350 }} />
        </Card>
        <Card title="商品排行">
          <Table rowKey={(r, i) => r.id || i} columns={rankColumns} dataSource={rankList} size="small" pagination={false} />
        </Card>
      </div>
    </Spin>
  );
};

export default StatisticProduct;
