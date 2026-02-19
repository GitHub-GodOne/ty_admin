import React, { useState, useEffect, useCallback } from 'react';
import { Card, Table, Form, Input, Button, Space, message, Popconfirm } from 'antd';
import { PlusOutlined, SearchOutlined, ReloadOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { shippingTemplatesList, shippingDetete } from '@/api/logistics';
import EditTemplate from './editTemplate';

const FreightTemplate: React.FC = () => {
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [keywords, setKeywords] = useState('');
  const [pagination, setPagination] = useState({ current: 1, pageSize: 20, total: 0 });
  const [drawerVisible, setDrawerVisible] = useState(false);
  const [editId, setEditId] = useState<number | null>(null);

  const fetchList = useCallback(async (page = 1) => {
    setLoading(true);
    try {
      const res = await shippingTemplatesList({ page, limit: pagination.pageSize, keywords: keywords || undefined });
      setList(res?.list || []);
      setPagination((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch {
      message.error('获取运费模板列表失败');
    } finally {
      setLoading(false);
    }
  }, [keywords, pagination.pageSize]);

  useEffect(() => { fetchList(1); }, []);

  const handleReset = () => { setKeywords(''); };

  const handleDelete = async (id: number) => {
    try {
      await shippingDetete({ id });
      message.success('删除成功');
      fetchList(pagination.current);
    } catch { /* noop */ }
  };

  const handleEdit = (id: number) => { setEditId(id); setDrawerVisible(true); };
  const handleAdd = () => { setEditId(null); setDrawerVisible(true); };

  const columns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 60 },
    { title: '模板名称', dataIndex: 'name', width: 200 },
    { title: '计费方式', dataIndex: 'type', width: 100,
      render: (v: number) => v === 1 ? '按件数' : v === 2 ? '按重量' : v === 3 ? '按体积' : '无',
    },
    { title: '包邮方式', dataIndex: 'appoint', width: 100,
      render: (v: number) => v === 0 ? '全国包邮' : v === 1 ? '部分包邮' : '自定义',
    },
    { title: '排序', dataIndex: 'sort', width: 80 },
    { title: '创建时间', dataIndex: 'createTime', width: 170 },
    {
      title: '操作', width: 150, fixed: 'right',
      render: (_: any, record: any) => (
        <Space size="small">
          <a onClick={() => handleEdit(record.id)}>编辑</a>
          <Popconfirm title="确定删除?" onConfirm={() => handleDelete(record.id)}>
            <a style={{ color: '#ff4d4f' }}>删除</a>
          </Popconfirm>
        </Space>
      ),
    },
  ];

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
      <Card bodyStyle={{ paddingBottom: 0 }}>
        <Form layout="inline" style={{ marginBottom: 16 }}>
          <Form.Item>
            <Input placeholder="模板名称" value={keywords} onChange={(e) => setKeywords(e.target.value)}
              onPressEnter={() => fetchList(1)} allowClear prefix={<SearchOutlined />} style={{ width: 240 }} />
          </Form.Item>
          <Form.Item>
            <Button type="primary" onClick={() => fetchList(1)}>搜索</Button>
          </Form.Item>
          <Form.Item>
            <Button icon={<ReloadOutlined />} onClick={handleReset}>重置</Button>
          </Form.Item>
        </Form>
      </Card>
      <Card>
        <div style={{ display: 'flex', justifyContent: 'flex-end', marginBottom: 16 }}>
          <Button type="primary" icon={<PlusOutlined />} onClick={handleAdd}>添加运费模板</Button>
        </div>
        <Table rowKey="id" columns={columns} dataSource={list} loading={loading} size="small"
          scroll={{ x: 900 }}
          pagination={{ ...pagination, showSizeChanger: true, pageSizeOptions: ['20', '40', '60', '80'],
            showTotal: (t: number) => `共 ${t} 条`,
            onChange: (p: number, ps: number) => { setPagination((prev) => ({ ...prev, pageSize: ps })); fetchList(p); } }} />
      </Card>
      <EditTemplate
        visible={drawerVisible}
        editId={editId}
        onClose={() => setDrawerVisible(false)}
        onSuccess={() => fetchList(pagination.current)}
      />
    </div>
  );
};

export default FreightTemplate;
