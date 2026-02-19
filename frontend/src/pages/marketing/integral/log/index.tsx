import React, { useState, useEffect, useCallback } from 'react';
import { Card, Table, Form, Input, Button, Space, DatePicker, message } from 'antd';
import { SearchOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { integralListApi } from '@/api/marketing';
import dayjs from 'dayjs';

const { RangePicker } = DatePicker;

const IntegralLog: React.FC = () => {
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [searchForm] = Form.useForm();
  const [pagination, setPagination] = useState({ current: 1, pageSize: 20, total: 0 });

  const fetchList = useCallback(async (page = 1, pageSize = 20) => {
    setLoading(true);
    try {
      const values = searchForm.getFieldsValue();
      let dateLimit = '';
      if (values.dateRange && values.dateRange.length === 2) {
        dateLimit = `${values.dateRange[0].format('YYYY-MM-DD')},${values.dateRange[1].format('YYYY-MM-DD')}`;
      }
      const res = await integralListApi(
        { page, limit: pageSize },
        { dateLimit, keywords: values.keywords || '' }
      );
      setList(res?.list || []);
      setPagination({ current: page, pageSize, total: res?.total || 0 });
    } catch {
      message.error('获取积分日志失败');
    } finally {
      setLoading(false);
    }
  }, [searchForm]);

  useEffect(() => { fetchList(1, pagination.pageSize); }, []);

  const handleSearch = () => {
    fetchList(1, pagination.pageSize);
  };

  const handleReset = () => {
    searchForm.resetFields();
    fetchList(1, pagination.pageSize);
  };

  const columns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 60 },
    { title: '标题', dataIndex: 'title', width: 130, ellipsis: true },
    {
      title: '积分余量', dataIndex: 'balance', width: 120,
      sorter: (a, b) => a.balance - b.balance,
    },
    {
      title: '明细数字', dataIndex: 'integral', width: 120,
      sorter: (a, b) => a.integral - b.integral,
    },
    { title: '备注', dataIndex: 'mark', width: 120, ellipsis: true },
    { title: '用户昵称', dataIndex: 'nickName', width: 120 },
    { title: '添加时间', dataIndex: 'updateTime', width: 170 },
  ];

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 14 }}>
      <Card bodyStyle={{ paddingBottom: 0 }}>
        <Form form={searchForm} layout="inline" style={{ marginBottom: 16 }}>
          <Form.Item label="时间选择" name="dateRange">
            <RangePicker
              format="YYYY-MM-DD"
              placeholder={['开始时间', '结束时间']}
              style={{ width: 260 }}
              onChange={() => handleSearch()}
            />
          </Form.Item>
          <Form.Item label="微信昵称" name="keywords">
            <Input placeholder="请输入用户昵称" allowClear style={{ width: 180 }} />
          </Form.Item>
          <Form.Item>
            <Space>
              <Button type="primary" icon={<SearchOutlined />} onClick={handleSearch}>搜索</Button>
              <Button onClick={handleReset}>重置</Button>
            </Space>
          </Form.Item>
        </Form>
      </Card>

      <Card>
        <Table
          rowKey="id"
          columns={columns}
          dataSource={list}
          loading={loading}
          size="small"
          pagination={{
            ...pagination,
            showSizeChanger: true,
            pageSizeOptions: ['20', '40', '60', '80'],
            showTotal: (total) => `共 ${total} 条`,
            onChange: (page, pageSize) => fetchList(page, pageSize),
          }}
        />
      </Card>
    </div>
  );
};

export default IntegralLog;
