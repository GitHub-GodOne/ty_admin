import React, { useState, useEffect, useCallback } from 'react';
import { Modal, Table, Input, Space, message } from 'antd';
import type { ColumnsType } from 'antd/es/table';
import { couponListApi } from '@/api/marketing';

interface CouponListProps {
  open: boolean;
  onCancel: () => void;
  onOk: (selected: any[]) => void;
  multiple?: boolean;
}

const CouponList: React.FC<CouponListProps> = ({ open, onCancel, onOk, multiple = true }) => {
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [keyword, setKeyword] = useState('');
  const [selectedRowKeys, setSelectedRowKeys] = useState<React.Key[]>([]);
  const [selectedRows, setSelectedRows] = useState<any[]>([]);
  const [pagination, setPagination] = useState({ current: 1, pageSize: 10, total: 0 });

  const fetchList = useCallback(async (page = 1) => {
    setLoading(true);
    try {
      const res = await couponListApi({ page, limit: pagination.pageSize, name: keyword, status: 1 });
      setList(res?.list || []);
      setPagination((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch {
      message.error('获取优惠券列表失败');
    } finally {
      setLoading(false);
    }
  }, [keyword, pagination.pageSize]);

  useEffect(() => {
    if (open) fetchList(1);
  }, [open]);

  const columns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 60 },
    { title: '优惠券名称', dataIndex: 'name', ellipsis: true },
    { title: '面值', dataIndex: 'couponPrice', width: 80 },
    { title: '使用门槛', dataIndex: 'useMinPrice', width: 100 },
    { title: '剩余数量', dataIndex: 'lastTotal', width: 100 },
  ];

  const handleOk = () => {
    if (!selectedRows.length) { message.warning('请选择优惠券'); return; }
    onOk(selectedRows);
    setSelectedRowKeys([]);
    setSelectedRows([]);
  };

  return (
    <Modal title="选择优惠券" open={open} onCancel={onCancel} onOk={handleOk} width={800} destroyOnClose>
      <Space style={{ marginBottom: 16 }}>
        <Input.Search placeholder="搜索优惠券" value={keyword} onChange={(e) => setKeyword(e.target.value)} onSearch={() => fetchList(1)} allowClear style={{ width: 300 }} />
      </Space>
      <Table
        rowKey="id"
        columns={columns}
        dataSource={list}
        loading={loading}
        size="small"
        rowSelection={{
          type: multiple ? 'checkbox' : 'radio',
          selectedRowKeys,
          onChange: (keys, rows) => { setSelectedRowKeys(keys); setSelectedRows(rows); },
        }}
        pagination={{ ...pagination, onChange: (p) => fetchList(p), showSizeChanger: false }}
      />
    </Modal>
  );
};

export default CouponList;
