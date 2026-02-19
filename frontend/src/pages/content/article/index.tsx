import React, { useState, useEffect, useCallback } from 'react';
import { Card, Table, Form, Input, Button, Space, Tag, message } from 'antd';
import { SearchOutlined, PlusOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { useNavigate } from 'react-router-dom';
import { ListArticle as articleListApi, DelArticle as articleDeleteApi } from '@/api/article';
import { useModal } from '@/hooks/useModal';
import { usePagination } from '@/hooks/usePagination';

const Article: React.FC = () => {
  const navigate = useNavigate();
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [keywords, setKeywords] = useState('');
  const { deleteConfirm } = useModal();
  const { pagination, setTotal, antdPagination } = usePagination();

  const fetchList = useCallback(async () => {
    setLoading(true);
    try {
      const res: any = await articleListApi({
        keywords,
        page: pagination.page,
        limit: pagination.limit,
      });
      setList(res?.list || []);
      setTotal(res?.total || 0);
    } catch {
      /* handled by interceptor */
    } finally {
      setLoading(false);
    }
  }, [keywords, pagination.page, pagination.limit]);

  useEffect(() => { fetchList(); }, [fetchList]);

  const handleDelete = (id: number) => {
    deleteConfirm(async () => {
      await articleDeleteApi({ id });
      message.success('删除成功');
      fetchList();
    });
  };

  const columns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 80 },
    { title: '标题', dataIndex: 'title', ellipsis: true },
    { title: '作者', dataIndex: 'author', width: 120 },
    { title: '分类', dataIndex: 'cid', width: 100 },
    { title: '浏览量', dataIndex: 'visit', width: 100 },
    {
      title: '状态', dataIndex: 'status', width: 80,
      render: (val: number) => (
        <Tag color={val ? 'green' : 'default'}>{val ? '显示' : '隐藏'}</Tag>
      ),
    },
    { title: '创建时间', dataIndex: 'createTime', width: 180 },
    {
      title: '操作', width: 150, fixed: 'right',
      render: (_: any, record: any) => (
        <Space>
          <a onClick={() => navigate(`/content/article/create?id=${record.id}`)}>编辑</a>
          <a onClick={() => handleDelete(record.id)} style={{ color: '#ff4d4f' }}>删除</a>
        </Space>
      ),
    },
  ];

  return (
    <div>
      <Card bodyStyle={{ padding: '16px' }} style={{ marginBottom: 16 }}>
        <Form layout="inline">
          <Form.Item label="文章标题">
            <Input
              value={keywords}
              onChange={(e) => setKeywords(e.target.value)}
              placeholder="请输入标题搜索"
              allowClear
            />
          </Form.Item>
          <Form.Item>
            <Space>
              <Button type="primary" icon={<SearchOutlined />} onClick={fetchList}>搜索</Button>
              <Button onClick={() => { setKeywords(''); }}>重置</Button>
              <Button type="primary" icon={<PlusOutlined />} onClick={() => navigate('/content/article/create')}>
                添加文章
              </Button>
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
          pagination={antdPagination}
          scroll={{ x: 1000 }}
          size="small"
        />
      </Card>
    </div>
  );
};

export default Article;
