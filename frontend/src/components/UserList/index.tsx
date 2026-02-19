import React, { useState, useEffect, useCallback } from 'react';
import { Modal, Table, Input, Space, message } from 'antd';
import type { ColumnsType } from 'antd/es/table';
import { userListApi } from '@/api/user';

interface UserListProps {
  open: boolean;
  onCancel: () => void;
  onOk: (selected: any[]) => void;
  multiple?: boolean;
}

const UserList: React.FC<UserListProps> = ({ open, onCancel, onOk, multiple = true }) => {
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [keyword, setKeyword] = useState('');
  const [selectedRowKeys, setSelectedRowKeys] = useState<React.Key[]>([]);
  const [selectedRows, setSelectedRows] = useState<any[]>([]);
  const [pagination, setPagination] = useState({ current: 1, pageSize: 10, total: 0 });

  const fetchList = useCallback(async (page = 1) => {
    setLoading(true);
    try {
      const res = await userListApi({ page, limit: pagination.pageSize, keywords: keyword });
      setList(res?.list || []);
      setPagination((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch {
      message.error('获取用户列表失败');
    } finally {
      setLoading(false);
    }
  }, [keyword, pagination.pageSize]);

  useEffect(() => {
    if (open) fetchList(1);
  }, [open]);

  const columns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'uid', width: 60 },
    { title: '昵称', dataIndex: 'nickname', ellipsis: true },
    { title: '手机号', dataIndex: 'phone', width: 120 },
    { title: '余额', dataIndex: 'nowMoney', width: 80 },
  ];

  const handleOk = () => {
    if (!selectedRows.length) { message.warning('请选择用户'); return; }
    onOk(selectedRows);
    setSelectedRowKeys([]);
    setSelectedRows([]);
  };

  return (
    <Modal title="选择用户" open={open} onCancel={onCancel} onOk={handleOk} width={700} destroyOnClose>
      <Space style={{ marginBottom: 16 }}>
        <Input.Search placeholder="搜索用户" value={keyword} onChange={(e) => setKeyword(e.target.value)} onSearch={() => fetchList(1)} allowClear style={{ width: 300 }} />
      </Space>
      <Table
        rowKey="uid"
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

export default UserList;
