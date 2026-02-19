import React, { useState, useEffect, useCallback } from 'react';
import {
  Tree, Table, Card, Form, Select, Input, Button, Switch, Modal, Space, message,
  InputNumber, Cascader, Image,
} from 'antd';
import { PlusOutlined, DeleteOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import * as categoryApi from '@/api/categoryApi';
import { useModal } from '@/hooks/useModal';
import MaterialPicker from '@/components/MaterialPicker';

interface CategoryProps {
  biztype: { value: number; name: string };
  pid?: number;
  selectModel?: boolean;
  selectModelKeys?: number[];
  onRulesSelect?: (keys: number[]) => void;
}

const Category: React.FC<CategoryProps> = ({ biztype, pid = 0, selectModel = false, selectModelKeys = [], onRulesSelect }) => {
  const [treeList, setTreeList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [status, setStatus] = useState<number>(-1);
  const [name, setName] = useState('');
  const { deleteConfirm } = useModal();

  // 弹窗
  const [modalOpen, setModalOpen] = useState(false);
  const [modalTitle, setModalTitle] = useState('添加分类');
  const [editId, setEditId] = useState<number | null>(null);
  const [saving, setSaving] = useState(false);
  const [form] = Form.useForm();

  // 图标选择
  const [iconUrl, setIconUrl] = useState('');
  const [iconPickerOpen, setIconPickerOpen] = useState(false);

  const fetchTree = useCallback(async () => {
    setLoading(true);
    try {
      const params: any = { type: biztype.value };
      if (status !== -1) params.status = status;
      if (name) params.name = name;
      const data = biztype.value !== 3
        ? await categoryApi.treeCategroy(params)
        : await categoryApi.listCategroy({ type: 3, status: status !== -1 ? status : undefined, pid, name: name || undefined });
      setTreeList(Array.isArray(data) ? data : []);
    } catch {
      message.error('获取分类失败');
    } finally { setLoading(false); }
  }, [biztype.value, status, name, pid]);

  useEffect(() => { fetchTree(); }, [fetchTree]);

  const handleStatusChange = async (row: any) => {
    try {
      await categoryApi.categroyUpdateStatus(row.id);
      message.success('修改成功');
      fetchTree();
    } catch { /* noop */ }
  };

  const handleDelete = (row: any) => {
    deleteConfirm(async () => {
      await categoryApi.deleteCategroy({ id: row.id });
      message.success('删除成功');
      fetchTree();
    });
  };

  // 根据 pid 构建 Cascader 路径数组
  const buildCascaderPath = (targetPid: number, list: any[]): number[] => {
    if (!targetPid || targetPid === 0) return [0];
    const findPath = (nodes: any[], target: number, path: number[]): number[] | null => {
      for (const node of nodes) {
        const cur = [...path, node.id];
        if (node.id === target) return cur;
        if (node.child?.length) {
          const found = findPath(node.child, target, cur);
          if (found) return found;
        }
      }
      return null;
    };
    return [0, ...(findPath(treeList, targetPid, []) || [targetPid])];
  };

  // 添加分类
  const handleAdd = () => {
    setEditId(null);
    setModalTitle('添加分类');
    form.resetFields();
    form.setFieldsValue({ pid: [0], sort: 0, status: 1 });
    setIconUrl('');
    setModalOpen(true);
  };

  // 添加子分类
  const handleAddChild = (row: any) => {
    setEditId(null);
    setModalTitle(`添加子分类 - ${row.name}`);
    form.resetFields();
    form.setFieldsValue({ pid: buildCascaderPath(row.id, treeList), sort: 0, status: 1 });
    setIconUrl('');
    setModalOpen(true);
  };

  // 编辑分类
  const handleEdit = async (row: any) => {
    setEditId(row.id);
    setModalTitle('编辑分类');
    form.resetFields();
    form.setFieldsValue({
      name: row.name, pid: buildCascaderPath(row.pid, treeList), sort: row.sort || 0,
      status: row.status, url: row.url || '',
    });
    setIconUrl(row.extra || '');
    setModalOpen(true);
  };

  // 保存
  const handleSave = async () => {
    try {
      const values = await form.validateFields();
      setSaving(true);
      const pidVal = Array.isArray(values.pid) ? values.pid[values.pid.length - 1] : (values.pid ?? 0);
      const submitData = {
        name: values.name,
        pid: Number(pidVal),
        sort: Number(values.sort ?? 0),
        status: Number(values.status ?? 1),
        type: Number(values.type || biztype.value),
        url: values.url || '',
        extra: iconUrl || '',
      };
      if (editId) {
        await categoryApi.updateCategroy({ ...submitData, id: editId });
        message.success('修改成功');
      } else {
        await categoryApi.addCategroy(submitData);
        message.success('添加成功');
      }
      setModalOpen(false);
      fetchTree();
    } catch (e: any) {
      if (e?.errorFields) return;
      message.error(e?.message || '操作失败');
    } finally { setSaving(false); }
  };

  if (selectModel) {
    return (
      <Tree checkable defaultCheckedKeys={selectModelKeys} treeData={treeList}
        fieldNames={{ title: 'name', key: 'id', children: 'child' }}
        onCheck={(keys) => onRulesSelect?.(keys as number[])} />
    );
  }

  const columns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 70 },
    { title: '名称', dataIndex: 'name', ellipsis: true },
    {
      title: '图标', dataIndex: 'extra', width: 80,
      render: (v: string) => v ? <Image src={v} width={36} height={36} style={{ borderRadius: 4, objectFit: 'cover' }} /> : <span style={{ color: '#ccc' }}>无</span>,
    },
    { title: '排序', dataIndex: 'sort', width: 80 },
    {
      title: '状态', width: 100,
      render: (_: any, r: any) => <Switch checked={!!r.status} onChange={() => handleStatusChange(r)} checkedChildren="显示" unCheckedChildren="隐藏" />,
    },
    {
      title: '操作', width: 220, fixed: 'right',
      render: (_: any, r: any) => (
        <Space size="small" wrap>
          <a onClick={() => handleEdit(r)}>编辑</a>
          <a onClick={() => handleAddChild(r)} style={{ color: '#0256FF' }}>添加子分类</a>
          <a onClick={() => handleDelete(r)} style={{ color: '#ff4d4f' }}>删除</a>
        </Space>
      ),
    },
  ];

  return (
    <div>
      <Card bodyStyle={{ padding: '16px' }} style={{ marginBottom: 16 }}>
        <Form layout="inline" size="small">
          <Form.Item label="分类状态">
            <Select value={status} onChange={setStatus} style={{ width: 120 }}>
              <Select.Option value={-1}>全部</Select.Option>
              <Select.Option value={1}>显示</Select.Option>
              <Select.Option value={0}>不显示</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item label="分类名称">
            <Input value={name} onChange={(e) => setName(e.target.value)} placeholder="请输入名称" allowClear />
          </Form.Item>
          <Form.Item>
            <Button type="primary" onClick={fetchTree}>搜索</Button>
            <Button style={{ marginLeft: 8 }} onClick={() => { setStatus(-1); setName(''); }}>重置</Button>
          </Form.Item>
        </Form>
      </Card>
      <Card>
        <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: 12 }}>
          <span style={{ fontWeight: 500, fontSize: 15 }}>{biztype.name}</span>
          <Button type="primary" icon={<PlusOutlined />} onClick={handleAdd}>添加分类</Button>
        </div>
        <Table rowKey="id" columns={columns} dataSource={treeList} loading={loading}
          size="small" childrenColumnName="child" pagination={false} scroll={{ x: 700 }}
          defaultExpandAllRows />
      </Card>

      {/* 添加/编辑分类弹窗 */}
      <Modal title={modalTitle} open={modalOpen} onCancel={() => setModalOpen(false)}
        onOk={handleSave} confirmLoading={saving} destroyOnClose width={600}>
        <Form form={form} labelCol={{ span: 5 }} wrapperCol={{ span: 18 }} style={{ marginTop: 16 }}>
          <Form.Item label="分类名称" name="name" rules={[{ required: true, message: '请输入分类名称' }]}>
            <Input placeholder="请输入分类名称" maxLength={50} />
          </Form.Item>
          <Form.Item label="父级分类" name="pid">
            <Cascader options={[{ id: 0, name: '顶级分类', child: treeList }]}
              fieldNames={{ label: 'name', value: 'id', children: 'child' }}
              changeOnSelect placeholder="选择父级分类" allowClear />
          </Form.Item>
          <Form.Item label="分类图标">
            <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
              {iconUrl ? (
                <div style={{ position: 'relative' }}>
                  <Image src={iconUrl} width={60} height={60} style={{ borderRadius: 4, objectFit: 'cover' }} />
                  <DeleteOutlined style={{ position: 'absolute', top: -6, right: -6, color: '#ff4d4f', cursor: 'pointer', background: '#fff', borderRadius: '50%' }}
                    onClick={() => setIconUrl('')} />
                </div>
              ) : (
                <Button icon={<PlusOutlined />} onClick={() => setIconPickerOpen(true)}>选择图标</Button>
              )}
            </div>
            <div style={{ color: '#999', fontSize: 12, marginTop: 4 }}>建议尺寸：80x80px</div>
            <MaterialPicker open={iconPickerOpen} onCancel={() => setIconPickerOpen(false)}
              onOk={(urls) => { setIconUrl(urls[0] || ''); setIconPickerOpen(false); }} />
          </Form.Item>
          <Form.Item label="排序" name="sort" initialValue={0}>
            <InputNumber min={0} style={{ width: '100%' }} placeholder="数字越大越靠前" />
          </Form.Item>
          <Form.Item label="状态" name="status" initialValue={1}>
            <Select>
              <Select.Option value={1}>显示</Select.Option>
              <Select.Option value={0}>隐藏</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item label="链接地址" name="url">
            <Input placeholder="请输入链接地址（选填）" />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
};

export default Category;
