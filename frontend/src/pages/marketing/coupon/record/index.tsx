import React, { useState, useEffect, useCallback } from 'react';
import { Card, Table, Form, Input, Button, Space, Select, message } from 'antd';
import { SearchOutlined, CheckOutlined, CloseOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { couponUserListApi } from '@/api/marketing';
import { userListApi } from '@/api/user';

// 获取方式
const typeMap: Record<string, string> = {
  receive: '自己领取',
  send: '后台发送',
  give: '满赠',
  new: '新人',
  buy: '买赠送',
};

// 使用状态
const statusMap: Record<number, string> = {
  0: '未使用',
  1: '已使用',
  2: '已过期',
};

const CouponRecord: React.FC = () => {
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [searchForm] = Form.useForm();
  const [pagination, setPagination] = useState({ current: 1, pageSize: 20, total: 0 });

  // 用户搜索
  const [userOptions, setUserOptions] = useState<any[]>([]);
  const [userLoading, setUserLoading] = useState(false);

  const fetchList = useCallback(async (page = 1, pageSize = 20) => {
    setLoading(true);
    try {
      const values = searchForm.getFieldsValue();
      const res = await couponUserListApi({
        page,
        limit: pageSize,
        status: values.status,
        uid: values.uid || undefined,
        name: values.name || undefined,
      });
      setList(res?.list || []);
      setPagination({ current: page, pageSize, total: res?.total || 0 });
    } catch {
      message.error('获取领取记录失败');
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
    setUserOptions([]);
    fetchList(1, pagination.pageSize);
  };

  // 远程搜索用户
  const handleUserSearch = async (keyword: string) => {
    if (!keyword) {
      setUserOptions([]);
      return;
    }
    setUserLoading(true);
    try {
      const res = await userListApi({ keywords: keyword, page: 1, limit: 10 });
      setUserOptions(res?.list || []);
    } catch {
      setUserOptions([]);
    } finally {
      setUserLoading(false);
    }
  };

  const columns: ColumnsType<any> = [
    { title: '优惠券ID', dataIndex: 'couponId', width: 80 },
    { title: '优惠券名称', dataIndex: 'name', width: 150, ellipsis: true },
    { title: '领取人', dataIndex: 'nickname', width: 130 },
    { title: '面值', dataIndex: 'money', width: 100, render: (v: any) => `¥${v}` },
    { title: '最低消费额', dataIndex: 'minPrice', width: 120, render: (v: any) => v > 0 ? `¥${v}` : '无门槛' },
    { title: '开始使用时间', dataIndex: 'startTime', width: 150 },
    { title: '结束使用时间', dataIndex: 'endTime', width: 150 },
    {
      title: '获取方式', dataIndex: 'type', width: 100,
      render: (v: string) => typeMap[v] || v,
    },
    {
      title: '是否可用', dataIndex: 'status', width: 80,
      render: (v: number) => v === 0
        ? <CheckOutlined style={{ fontSize: 14, color: '#0092dc' }} />
        : <CloseOutlined style={{ fontSize: 14, color: '#ed5565' }} />,
    },
    {
      title: '使用状态', dataIndex: 'status', width: 100,
      render: (v: number) => statusMap[v] || '-',
    },
  ];

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 14 }}>
      <Card bodyStyle={{ paddingBottom: 0 }}>
        <Form form={searchForm} layout="inline" style={{ marginBottom: 16 }}>
          <Form.Item label="使用状态" name="status">
            <Select
              placeholder="请选择使用状态"
              allowClear
              style={{ width: 150 }}
              onChange={handleSearch}
            >
              <Select.Option value="1">已使用</Select.Option>
              <Select.Option value="0">未使用</Select.Option>
              <Select.Option value="2">已过期</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item label="领取人" name="uid">
            <Select
              placeholder="请输入领取人"
              allowClear
              showSearch
              filterOption={false}
              onSearch={handleUserSearch}
              loading={userLoading}
              style={{ width: 180 }}
              onChange={handleSearch}
            >
              {userOptions.map((item: any) => (
                <Select.Option key={item.uid} value={item.uid}>
                  {item.nickname}
                </Select.Option>
              ))}
            </Select>
          </Form.Item>
          <Form.Item label="优惠劵" name="name">
            <Input placeholder="请输入优惠劵" allowClear style={{ width: 180 }} />
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
          scroll={{ x: 1200 }}
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

export default CouponRecord;
