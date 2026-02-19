import React, { useState, useEffect, useCallback } from 'react';
import { Card, Table, Button, Space, message, Popconfirm, Modal, Input, Typography } from 'antd';
import { SearchOutlined, ReloadOutlined, EyeOutlined, DeleteOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { getFormConfigList, getFormConfigInfo } from '@/api/systemFormConfig';

const MaintainFormConfig: React.FC = () => {
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [keyword, setKeyword] = useState('');
  const [pagination, setPagination] = useState({ current: 1, pageSize: 20, total: 0 });
  const [viewOpen, setViewOpen] = useState(false);
  const [viewData, setViewData] = useState<any>(null);
  const [viewLoading, setViewLoading] = useState(false);

  const fetchList = useCallback(async (page = 1) => {
    setLoading(true);
    try {
      const res = await getFormConfigList({ page, limit: pagination.pageSize, keywords: keyword || undefined });
      setList(res?.list || []);
      setPagination((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch { message.error('获取表单配置列表失败'); }
    finally { setLoading(false); }
  }, [keyword, pagination.pageSize]);

  useEffect(() => { fetchList(1); }, []);

  const handleView = async (id: number) => {
    setViewOpen(true);
    setViewLoading(true);
    try {
      const res = await getFormConfigInfo({ id });
      setViewData(res);
    } catch { message.error('获取表单详情失败'); }
    finally { setViewLoading(false); }
  };

  const columns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 80 },
    { title: '表单名称', dataIndex: 'name', ellipsis: true, width: 200 },
    { title: '表单信息', dataIndex: 'info', ellipsis: true },
    { title: '创建时间', dataIndex: 'createTime', width: 180 },
    {
      title: '操作', width: 150, fixed: 'right',
      render: (_: any, record: any) => (
        <Space size="small">
          <Button type="link" size="small" icon={<EyeOutlined />} onClick={() => handleView(record.id)}>查看</Button>
        </Space>
      ),
    },
  ];

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
      <Card bodyStyle={{ paddingBottom: 0 }}>
        <Space style={{ marginBottom: 16 }}>
          <Input
            placeholder="表单名称"
            value={keyword}
            onChange={(e) => setKeyword(e.target.value)}
            onPressEnter={() => fetchList(1)}
            allowClear
            prefix={<SearchOutlined />}
            style={{ width: 240 }}
          />
          <Button type="primary" onClick={() => fetchList(1)}>搜索</Button>
          <Button icon={<ReloadOutlined />} onClick={() => { setKeyword(''); fetchList(1); }}>重置</Button>
        </Space>
      </Card>
      <Card>
        <Table
          rowKey="id"
          columns={columns}
          dataSource={list}
          loading={loading}
          size="small"
          scroll={{ x: 700 }}
          pagination={{
            ...pagination,
            showSizeChanger: true,
            showTotal: (t) => `共 ${t} 条`,
            onChange: (p, ps) => { setPagination((prev) => ({ ...prev, pageSize: ps })); fetchList(p); },
          }}
        />
      </Card>
      <Modal
        title="表单信息"
        open={viewOpen}
        onCancel={() => { setViewOpen(false); setViewData(null); }}
        footer={null}
        width={700}
      >
        {viewLoading ? (
          <div style={{ textAlign: 'center', padding: 40 }}>加载中...</div>
        ) : viewData ? (
          <pre style={{ maxHeight: 500, overflow: 'auto', background: '#f5f5f5', padding: 16, borderRadius: 8, fontSize: 13 }}>
            {JSON.stringify(viewData, null, 2)}
          </pre>
        ) : (
          <Typography.Text type="secondary">暂无数据</Typography.Text>
        )}
      </Modal>
    </div>
  );
};

export default MaintainFormConfig;
