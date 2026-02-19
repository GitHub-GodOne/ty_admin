import React, { useState, useEffect, useCallback, useRef } from 'react';
import { Card, Table, Button, Space, Modal, Form, Input, message, Popconfirm, Tag } from 'antd';
import { PlusOutlined, DeleteOutlined, SearchOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { templateListApi, attrCreatApi, attrEditApi, attrDeleteApi, attrInfoApi } from '@/api/store';

interface RuleItem {
  value: string;
  detail: string[];
  inputVisible: boolean;
}

const TAG_COLORS = ['blue', 'green', 'orange', 'purple', 'cyan', 'magenta', 'red', 'gold', 'lime'];

const StoreAttr: React.FC = () => {
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [editId, setEditId] = useState<number | null>(null);
  const [form] = Form.useForm();
  const [pagination, setPagination] = useState({ current: 1, pageSize: 20, total: 0 });
  const [selectedRowKeys, setSelectedRowKeys] = useState<React.Key[]>([]);
  const [keyword, setKeyword] = useState('');
  const [ruleList, setRuleList] = useState<RuleItem[]>([]);
  const inputRefs = useRef<Record<string, any>>({});

  const fetchList = useCallback(async (page = 1) => {
    setLoading(true);
    try {
      const res = await templateListApi({ page, limit: pagination.pageSize, keywords: keyword || undefined });
      setList(res?.list || []);
      setPagination((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch { message.error('获取规格列表失败'); }
    finally { setLoading(false); }
  }, [pagination.pageSize, keyword]);

  useEffect(() => { fetchList(1); }, []);

  const parseRuleValue = (val: any): RuleItem[] => {
    if (!val) return [];
    try {
      const arr = typeof val === 'string' ? JSON.parse(val) : val;
      if (Array.isArray(arr)) return arr.map((item: any) => ({ value: item.value || '', detail: item.detail || [], inputVisible: false }));
    } catch {}
    return [];
  };

  const handleEdit = async (id: number) => {
    try {
      const info = await attrInfoApi(id);
      setEditId(id);
      form.setFieldsValue({ ruleName: info.ruleName });
      setRuleList(parseRuleValue(info.ruleValue));
      setModalOpen(true);
    } catch { message.error('获取详情失败'); }
  };

  const handleDelete = async (id: number) => {
    await attrDeleteApi(id);
    message.success('删除成功');
    setSelectedRowKeys((keys) => keys.filter((k) => k !== id));
    fetchList(pagination.current);
  };

  const handleBatchDelete = async () => {
    const ids = selectedRowKeys.join(',');
    await attrDeleteApi(ids);
    message.success('批量删除成功');
    setSelectedRowKeys([]);
    fetchList(1);
  };

  const handleSubmit = async () => {
    const values = await form.validateFields();
    if (ruleList.length === 0) { message.warning('请至少添加一个规格'); return; }
    for (const item of ruleList) {
      if (!item.value.trim()) { message.warning('规格属性名不能为空'); return; }
      if (item.detail.length === 0) { message.warning(`规格"${item.value}"至少需要一个规格值`); return; }
    }
    const ruleValue = JSON.stringify(ruleList.map(({ value, detail }) => ({ value, detail })));
    const payload = { ...values, ruleValue };
    editId ? await attrEditApi({ ...payload, id: editId }) : await attrCreatApi(payload);
    message.success(editId ? '编辑成功' : '添加成功');
    setModalOpen(false);
    form.resetFields();
    setRuleList([]);
    setEditId(null);
    fetchList(1);
  };

  // 规格编辑器操作
  const addRule = () => setRuleList([...ruleList, { value: '', detail: [], inputVisible: false }]);

  const removeRule = (idx: number) => setRuleList(ruleList.filter((_, i) => i !== idx));

  const updateRuleName = (idx: number, name: string) => {
    const next = [...ruleList];
    next[idx] = { ...next[idx], value: name };
    setRuleList(next);
  };

  const removeDetail = (ruleIdx: number, detailIdx: number) => {
    const next = [...ruleList];
    next[ruleIdx] = { ...next[ruleIdx], detail: next[ruleIdx].detail.filter((_, i) => i !== detailIdx) };
    setRuleList(next);
  };

  const showDetailInput = (ruleIdx: number) => {
    const next = [...ruleList];
    next[ruleIdx] = { ...next[ruleIdx], inputVisible: true };
    setRuleList(next);
    setTimeout(() => inputRefs.current[`detail-${ruleIdx}`]?.focus(), 50);
  };

  const handleDetailInputConfirm = (ruleIdx: number, val: string) => {
    const next = [...ruleList];
    const trimmed = val.trim();
    if (trimmed && !next[ruleIdx].detail.includes(trimmed)) {
      next[ruleIdx] = { ...next[ruleIdx], detail: [...next[ruleIdx].detail, trimmed], inputVisible: false };
    } else {
      next[ruleIdx] = { ...next[ruleIdx], inputVisible: false };
    }
    setRuleList(next);
  };

  const columns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 60 },
    { title: '规格名称', dataIndex: 'ruleName', width: 150 },
    {
      title: '规格值', dataIndex: 'ruleValue',
      render: (val: any) => {
        const items = parseRuleValue(val);
        if (items.length === 0) return '-';
        return (
          <Space direction="vertical" size={2}>
            {items.map((item, i) => (
              <div key={i}>
                <span style={{ marginRight: 4, fontWeight: 500 }}>{item.value}:</span>
                {item.detail.map((d, j) => (
                  <Tag color={TAG_COLORS[i % TAG_COLORS.length]} key={j} style={{ marginBottom: 2 }}>{d}</Tag>
                ))}
              </div>
            ))}
          </Space>
        );
      },
    },
    {
      title: '操作', width: 150,
      render: (_: any, r: any) => (
        <Space>
          <a onClick={() => handleEdit(r.id)}>编辑</a>
          <Popconfirm title="确定删除?" onConfirm={() => handleDelete(r.id)}>
            <a style={{ color: '#ff4d4f' }}>删除</a>
          </Popconfirm>
        </Space>
      ),
    },
  ];

  const openAdd = () => {
    setEditId(null);
    form.resetFields();
    setRuleList([{ value: '', detail: [], inputVisible: false }]);
    setModalOpen(true);
  };

  return (
    <Card
      title="商品规格"
      extra={
        <Space>
          <Input.Search
            placeholder="搜索规格名称"
            allowClear
            style={{ width: 200 }}
            onSearch={(v) => { setKeyword(v); fetchList(1); }}
            enterButton={<SearchOutlined />}
          />
          <Button type="primary" icon={<PlusOutlined />} onClick={openAdd}>添加规格</Button>
        </Space>
      }
    >
      {selectedRowKeys.length > 0 && (
        <div style={{ marginBottom: 12 }}>
          <Popconfirm title={`确定删除选中的 ${selectedRowKeys.length} 项？`} onConfirm={handleBatchDelete}>
            <Button danger icon={<DeleteOutlined />}>批量删除 ({selectedRowKeys.length})</Button>
          </Popconfirm>
        </div>
      )}
      <Table
        rowKey="id"
        columns={columns}
        dataSource={list}
        loading={loading}
        size="small"
        rowSelection={{ selectedRowKeys, onChange: setSelectedRowKeys }}
        pagination={{ ...pagination, showTotal: (t) => `共 ${t} 条`, onChange: (p) => fetchList(p) }}
      />

      <Modal title={editId ? '编辑规格' : '添加规格'} open={modalOpen} onCancel={() => setModalOpen(false)} onOk={handleSubmit} destroyOnClose width={600}>
        <Form form={form} layout="vertical">
          <Form.Item name="ruleName" label="规格名称" rules={[{ required: true, message: '请输入规格名称' }]}>
            <Input placeholder="如：颜色" />
          </Form.Item>
        </Form>
        <div style={{ marginBottom: 8, fontWeight: 500 }}>规格属性：</div>
        {ruleList.map((rule, ruleIdx) => (
          <div key={ruleIdx} style={{ border: '1px solid #f0f0f0', borderRadius: 6, padding: 12, marginBottom: 10, background: '#fafafa' }}>
            <div style={{ display: 'flex', alignItems: 'center', marginBottom: 8 }}>
              <Input
                style={{ width: 160 }}
                placeholder="规格名，如：颜色"
                value={rule.value}
                onChange={(e) => updateRuleName(ruleIdx, e.target.value)}
              />
              <Button type="link" danger onClick={() => removeRule(ruleIdx)} style={{ marginLeft: 'auto' }}>删除</Button>
            </div>
            <div style={{ display: 'flex', flexWrap: 'wrap', gap: 4, alignItems: 'center' }}>
              {rule.detail.map((d, dIdx) => (
                <Tag key={dIdx} closable color={TAG_COLORS[ruleIdx % TAG_COLORS.length]} onClose={() => removeDetail(ruleIdx, dIdx)}>{d}</Tag>
              ))}
              {rule.inputVisible ? (
                <Input
                  ref={(el) => { inputRefs.current[`detail-${ruleIdx}`] = el; }}
                  size="small"
                  style={{ width: 100 }}
                  placeholder="规格值"
                  onBlur={(e) => handleDetailInputConfirm(ruleIdx, e.target.value)}
                  onPressEnter={(e) => handleDetailInputConfirm(ruleIdx, (e.target as HTMLInputElement).value)}
                />
              ) : (
                <Tag onClick={() => showDetailInput(ruleIdx)} style={{ borderStyle: 'dashed', cursor: 'pointer' }}>
                  <PlusOutlined /> 添加
                </Tag>
              )}
            </div>
          </div>
        ))}
        <Button type="dashed" block icon={<PlusOutlined />} onClick={addRule}>添加新规格</Button>
      </Modal>
    </Card>
  );
};

export default StoreAttr;
