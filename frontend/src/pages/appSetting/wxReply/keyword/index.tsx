import React, { useState, useEffect, useCallback } from 'react';
import { Card, Table, Button, Input, Select, Switch, Space, Modal, message } from 'antd';
import { PlusOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { useNavigate } from 'react-router-dom';
import { replyListApi, replyDeleteApi, replyStatusApi } from '@/api/wxApi';

const typeOptions = [
  { value: '', label: '全部' },
  { value: 'text', label: '文字消息' },
  { value: 'image', label: '图片消息' },
  { value: 'news', label: '图文消息' },
  { value: 'voice', label: '声音消息' },
];

const typeMap: Record<string, string> = { text: '文字消息', image: '图片消息', news: '图文消息', voice: '声音消息' };

const KeywordReply: React.FC = () => {
  const navigate = useNavigate();
  const [list, setList] = useState<any[]>([]);
  const [total, setTotal] = useState(0);
  const [loading, setLoading] = useState(false);
  const [page, setPage] = useState(1);
  const [limit, setLimit] = useState(20);
  const [type, setType] = useState('');
  const [keywords, setKeywords] = useState('');

  const fetchList = useCallback(async () => {
    setLoading(true);
    try {
      const res: any = await replyListApi({ page, limit, type: type || undefined, keywords: keywords || undefined });
      setList(res?.list || []);
      setTotal(res?.total || 0);
    } catch { /* */ }
    finally { setLoading(false); }
  }, [page, limit, type, keywords]);

  useEffect(() => { fetchList(); }, [fetchList]);

  const handleDelete = (id: number) => {
    Modal.confirm({
      title: '确认删除该回复？',
      onOk: async () => {
        await replyDeleteApi({ id });
        message.success('删除成功');
        fetchList();
      },
    });
  };

  const handleStatus = async (id: number, status: boolean) => {
    await replyStatusApi({ id, status: status ? 1 : 0 });
    message.success('状态修改成功');
    fetchList();
  };

  const columns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 60 },
    { title: '关键字', dataIndex: 'keywords', width: 200 },
    { title: '回复类型', dataIndex: 'type', width: 120, render: (v: string) => typeMap[v] || v },
    {
      title: '状态', dataIndex: 'status', width: 100,
      render: (v: any, record: any) => (
        <Switch checked={!!v} onChange={(c) => handleStatus(record.id, c)}
          disabled={record.keywords === 'subscribe' || record.keywords === 'default'} />
      ),
    },
    {
      title: '操作', width: 150, fixed: 'right',
      render: (_: any, record: any) => {
        const disabled = record.keywords === 'subscribe' || record.keywords === 'default';
        return (
          <Space>
            <a onClick={() => navigate(`/appSetting/publicAccount/wxReply/follow?id=${record.id}`)}>编辑</a>
            {!disabled && <a style={{ color: '#ff4d4f' }} onClick={() => handleDelete(record.id)}>删除</a>}
          </Space>
        );
      },
    },
  ];

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
      <Card>
        <div style={{ display: 'flex', gap: 12, marginBottom: 16, flexWrap: 'wrap', alignItems: 'center' }}>
          <Select value={type} onChange={setType} options={typeOptions} style={{ width: 140 }} placeholder="回复类型" />
          <Input value={keywords} onChange={(e) => setKeywords(e.target.value)} placeholder="请输入关键字"
            style={{ width: 200 }} onPressEnter={() => { setPage(1); fetchList(); }} />
          <Button type="primary" onClick={() => { setPage(1); fetchList(); }}>搜索</Button>
          <Button type="primary" icon={<PlusOutlined />}
            onClick={() => navigate('/appSetting/publicAccount/wxReply/follow')}>添加关键字</Button>
        </div>
        <Table rowKey="id" columns={columns} dataSource={list} loading={loading}
          size="small" scroll={{ x: 700 }}
          pagination={{ current: page, pageSize: limit, total, showSizeChanger: true,
            pageSizeOptions: ['20', '40', '60', '80'],
            onChange: (p, s) => { setPage(p); setLimit(s); } }} />
      </Card>
    </div>
  );
};

export default KeywordReply;
