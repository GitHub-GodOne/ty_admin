import React, { useState, useEffect, useCallback } from 'react';
import { Modal, Table, Input, Button, Space, Image, message } from 'antd';
import type { ColumnsType } from 'antd/es/table';
import { productLstApi } from '@/api/store';

interface GoodListProps {
  open: boolean;
  onCancel: () => void;
  onOk: (selected: any[]) => void;
  multiple?: boolean;
}

const GoodList: React.FC<GoodListProps> = ({ open, onCancel, onOk, multiple = true }) => {
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [keyword, setKeyword] = useState('');
  const [selectedRowKeys, setSelectedRowKeys] = useState<React.Key[]>([]);
  const [selectedRows, setSelectedRows] = useState<any[]>([]);
  const [pagination, setPagination] = useState({ current: 1, pageSize: 10, total: 0 });

  const fetchList = useCallback(async (page = 1) => {
    setLoading(true);
    try {
      const res = await productLstApi({ page, limit: pagination.pageSize, keywords: keyword, type: 1 });
      setList(res?.list || []);
      setPagination((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch {
      message.error('获取商品列表失败');
    } finally {
      setLoading(false);
    }
  }, [keyword, pagination.pageSize]);

  useEffect(() => {
    if (open) fetchList(1);
  }, [open]);

  const columns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 60 },
    {
      title: '商品图',
      dataIndex: 'image',
      width: 80,
      render: (v: string) => <Image src={v} width={40} height={40} />,
    },
    { title: '商品名称', dataIndex: 'storeName', ellipsis: true },
    { title: '价格', dataIndex: 'price', width: 80 },
    { title: '库存', dataIndex: 'stock', width: 80 },
  ];

  const handleOk = () => {
    if (!selectedRows.length) {
      message.warning('请选择商品');
      return;
    }
    onOk(selectedRows);
    setSelectedRowKeys([]);
    setSelectedRows([]);
  };

  return (
    <Modal title="选择商品" open={open} onCancel={onCancel} onOk={handleOk} width={800} destroyOnClose>
      <Space style={{ marginBottom: 16 }}>
        <Input.Search placeholder="搜索商品" value={keyword} onChange={(e) => setKeyword(e.target.value)} onSearch={() => fetchList(1)} allowClear style={{ width: 300 }} />
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

export default GoodList;
