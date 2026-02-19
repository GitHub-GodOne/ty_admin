import React, { useState, useEffect, useCallback } from 'react';
import { Card, Table, Form, Input, Button, Space, Switch, message, Modal, Select, Image } from 'antd';
import { PlusOutlined, SearchOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { useNavigate } from 'react-router-dom';
import { marketingListApi, couponDeleteApi, couponIssueStatusApi, couponUserListApi } from '@/api/marketing';

// 优惠券类型
const couponTypeMap: Record<string, string> = {
  receive: '手动领取',
  send: '后台发送',
  give: '满赠',
  new: '新人',
  buy: '买赠送',
};

// 使用类型
const useTypeMap: Record<number, string> = {
  1: '通用券',
  2: '商品券',
  3: '品类券',
};

const CouponList: React.FC = () => {
  const navigate = useNavigate();
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [searchForm] = Form.useForm();
  const [pagination, setPagination] = useState({ current: 1, pageSize: 20, total: 0 });

  // 领取记录弹窗
  const [recordModalOpen, setRecordModalOpen] = useState(false);
  const [recordLoading, setRecordLoading] = useState(false);
  const [recordList, setRecordList] = useState<any[]>([]);
  const [recordPagination, setRecordPagination] = useState({ current: 1, pageSize: 10, total: 0 });
  const [currentCouponId, setCurrentCouponId] = useState<number | null>(null);

  const fetchList = useCallback(async (page = 1, pageSize = 20) => {
    setLoading(true);
    try {
      const values = searchForm.getFieldsValue();
      const res = await marketingListApi({
        page,
        limit: pageSize,
        status: values.status,
        name: values.name || undefined,
      });
      setList(res?.list || []);
      setPagination({ current: page, pageSize, total: res?.total || 0 });
    } catch {
      message.error('获取优惠券列表失败');
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

  // 修改状态
  const handleStatusChange = async (row: any) => {
    try {
      await couponIssueStatusApi({ id: row.id, status: !row.status });
      message.success('修改成功');
      fetchList(pagination.current, pagination.pageSize);
    } catch {
      message.error('修改失败');
    }
  };

  // 删除
  const handleDelete = (row: any) => {
    Modal.confirm({
      title: '提示',
      content: '删除当前数据?',
      onOk: async () => {
        try {
          await couponDeleteApi({ id: row.id });
          message.success('删除成功');
          fetchList(1, pagination.pageSize);
        } catch {
          message.error('删除失败');
        }
      },
    });
  };

  // 领取记录
  const fetchRecordList = useCallback(async (couponId: number, page = 1, pageSize = 10) => {
    setRecordLoading(true);
    try {
      const res = await couponUserListApi({ couponId, page, limit: pageSize });
      setRecordList(res?.list || []);
      setRecordPagination({ current: page, pageSize, total: res?.total || 0 });
    } catch {
      message.error('获取领取记录失败');
    } finally {
      setRecordLoading(false);
    }
  }, []);

  const handleShowRecord = (row: any) => {
    setCurrentCouponId(row.id);
    setRecordModalOpen(true);
    fetchRecordList(row.id, 1, 10);
  };

  const columns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 60 },
    { title: '名称', dataIndex: 'name', width: 180, ellipsis: true },
    {
      title: '类型', dataIndex: 'useType', width: 80,
      render: (v: number) => useTypeMap[v] || '-',
    },
    { title: '面值', dataIndex: 'money', width: 100, render: (v: any) => `¥${v}` },
    {
      title: '领取方式', dataIndex: 'type', width: 100,
      render: (v: string) => couponTypeMap[v] || v,
    },
    {
      title: '领取日期', width: 220,
      render: (_: any, row: any) => row.receiveEndTime
        ? `${row.receiveStartTime} - ${row.receiveEndTime}`
        : '不限时',
    },
    {
      title: '使用时间', width: 220,
      render: (_: any, row: any) => row.day
        ? `${row.day}天`
        : `${row.useStartTime} - ${row.useEndTime}`,
    },
    {
      title: '发布数量', width: 100,
      render: (_: any, row: any) => !row.isLimited ? '不限量' : (
        <div>
          <div style={{ color: '#0a6aa1' }}>发布：{row.total}</div>
          <div style={{ color: '#ff0000' }}>剩余：{row.lastTotal}</div>
        </div>
      ),
    },
    {
      title: '是否开启', dataIndex: 'status', width: 100,
      render: (_: any, row: any) => (
        <Switch
          checked={row.status === true || row.status === 1}
          checkedChildren="开启"
          unCheckedChildren="关闭"
          onChange={() => handleStatusChange(row)}
        />
      ),
    },
    {
      title: '操作', width: 180, fixed: 'right',
      render: (_: any, row: any) => (
        <Space size={0} split={<span style={{ color: '#dcdfe6', margin: '0 6px' }}>|</span>}>
          <a onClick={() => handleShowRecord(row)}>领取记录</a>
          {row.status && (
            <a onClick={() => navigate(`/marketing/coupon/list/save?id=${row.id}`)}>复制</a>
          )}
          <a style={{ color: '#ff4d4f' }} onClick={() => handleDelete(row)}>删除</a>
        </Space>
      ),
    },
  ];

  // 领取记录表格列
  const recordColumns: ColumnsType<any> = [
    { title: '用户名', dataIndex: 'nickname', width: 120 },
    {
      title: '用户头像', dataIndex: 'avatar', width: 80,
      render: (v: string) => v ? <Image src={v} width={36} height={36} style={{ borderRadius: '50%' }} /> : '-',
    },
    { title: '领取时间', dataIndex: 'createTime', width: 180 },
  ];

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 14 }}>
      <Card bodyStyle={{ paddingBottom: 0 }}>
        <Form form={searchForm} layout="inline" style={{ marginBottom: 16 }}>
          <Form.Item label="状态" name="status">
            <Select placeholder="请选择" allowClear style={{ width: 150 }} onChange={handleSearch}>
              <Select.Option value={0}>未开启</Select.Option>
              <Select.Option value={1}>开启</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item label="优惠券名称" name="name">
            <Input placeholder="请输入优惠券名称" allowClear style={{ width: 200 }} />
          </Form.Item>
          <Form.Item>
            <Space>
              <Button type="primary" icon={<SearchOutlined />} onClick={handleSearch}>搜索</Button>
              <Button onClick={handleReset}>重置</Button>
            </Space>
          </Form.Item>
        </Form>
      </Card>

      <Card
        title={
          <Button type="primary" icon={<PlusOutlined />} onClick={() => navigate('/marketing/coupon/list/save')}>
            添加优惠劵
          </Button>
        }
      >
        <Table
          rowKey="id"
          columns={columns}
          dataSource={list}
          loading={loading}
          size="small"
          scroll={{ x: 1400 }}
          pagination={{
            ...pagination,
            showSizeChanger: true,
            pageSizeOptions: ['20', '40', '60', '80'],
            showTotal: (total) => `共 ${total} 条`,
            onChange: (page, pageSize) => fetchList(page, pageSize),
          }}
        />
      </Card>

      {/* 领取记录弹窗 */}
      <Modal
        title="领取记录"
        open={recordModalOpen}
        onCancel={() => setRecordModalOpen(false)}
        footer={null}
        width={700}
      >
        <Table
          rowKey="id"
          columns={recordColumns}
          dataSource={recordList}
          loading={recordLoading}
          size="small"
          pagination={{
            ...recordPagination,
            showSizeChanger: true,
            pageSizeOptions: ['10', '20', '30', '40'],
            showTotal: (total) => `共 ${total} 条`,
            onChange: (page, pageSize) => {
              if (currentCouponId) {
                fetchRecordList(currentCouponId, page, pageSize);
              }
            },
          }}
        />
      </Modal>
    </div>
  );
};

export default CouponList;
