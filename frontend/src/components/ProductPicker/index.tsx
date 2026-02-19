import React, { useState, useEffect, useCallback } from 'react';
import { Modal, Table, Input, Button, Space, Image, message } from 'antd';
import { SearchOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { productLstApi as productListApi } from '@/api/store';

interface ProductPickerProps {
  open: boolean;
  onCancel: () => void;
  onOk: (products: any[]) => void;
  multiple?: boolean;
  selectedProducts?: any[];
}

const ProductPicker: React.FC<ProductPickerProps> = ({
  open,
  onCancel,
  onOk,
  multiple = false,
  selectedProducts = [],
}) => {
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [keywords, setKeywords] = useState('');
  const [pagination, setPagination] = useState({ current: 1, pageSize: 10, total: 0 });
  const [selectedRowKeys, setSelectedRowKeys] = useState<React.Key[]>([]);
  const [selectedRows, setSelectedRows] = useState<any[]>([]);

  // 初始化已选商品
  useEffect(() => {
    if (open && selectedProducts.length > 0) {
      setSelectedRowKeys(selectedProducts.map((p) => p.id));
      setSelectedRows(selectedProducts);
    } else if (!open) {
      setSelectedRowKeys([]);
      setSelectedRows([]);
    }
  }, [open, selectedProducts]);

  const fetchList = useCallback(async (page = 1) => {
    setLoading(true);
    try {
      const params: any = { page, limit: pagination.pageSize, type: 1 };
      if (keywords) params.keywords = keywords;
      const res = await productListApi(params);
      setList(res?.list || []);
      setPagination((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch {
      message.error('获取商品列表失败');
    } finally {
      setLoading(false);
    }
  }, [keywords, pagination.pageSize]);

  useEffect(() => {
    if (open) {
      fetchList(1);
    }
  }, [open]);

  const handleSearch = () => {
    fetchList(1);
  };

  const handleOk = () => {
    if (selectedRows.length === 0) {
      message.warning('请选择商品');
      return;
    }
    onOk(selectedRows);
  };

  const columns: ColumnsType<any> = [
    {
      title: '商品图片',
      dataIndex: 'image',
      width: 80,
      render: (v: string) => <Image src={v} width={50} height={50} style={{ borderRadius: 4 }} />,
    },
    {
      title: '商品名称',
      dataIndex: 'storeName',
      ellipsis: true,
    },
    {
      title: '商品售价',
      dataIndex: 'price',
      width: 100,
      render: (v: number) => `¥${v}`,
    },
    {
      title: '库存',
      dataIndex: 'stock',
      width: 80,
    },
  ];

  return (
    <Modal
      title="选择商品"
      open={open}
      onCancel={onCancel}
      onOk={handleOk}
      width={800}
      destroyOnClose
    >
      <div style={{ marginBottom: 16 }}>
        <Space>
          <Input
            placeholder="请输入商品名称"
            value={keywords}
            onChange={(e) => setKeywords(e.target.value)}
            onPressEnter={handleSearch}
            style={{ width: 250 }}
            allowClear
          />
          <Button type="primary" icon={<SearchOutlined />} onClick={handleSearch}>
            搜索
          </Button>
        </Space>
      </div>
      <Table
        rowKey="id"
        columns={columns}
        dataSource={list}
        loading={loading}
        size="small"
        rowSelection={{
          type: multiple ? 'checkbox' : 'radio',
          selectedRowKeys,
          onChange: (keys, rows) => {
            setSelectedRowKeys(keys);
            // 合并已选和新选的商品
            if (multiple) {
              const existingMap = new Map(selectedRows.map((r) => [r.id, r]));
              rows.forEach((r) => existingMap.set(r.id, r));
              // 移除取消选择的
              const newRows = Array.from(existingMap.values()).filter((r) => keys.includes(r.id));
              setSelectedRows(newRows);
            } else {
              setSelectedRows(rows);
            }
          },
        }}
        pagination={{
          ...pagination,
          showSizeChanger: true,
          showTotal: (t) => `共 ${t} 条`,
          onChange: (p, ps) => {
            setPagination((prev) => ({ ...prev, pageSize: ps || 10 }));
            fetchList(p);
          },
        }}
      />
    </Modal>
  );
};

export default ProductPicker;
