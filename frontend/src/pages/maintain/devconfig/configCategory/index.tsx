import React, { useState, useEffect, useCallback } from 'react';
import {
  Card, Table, Button, Space, Modal, Form, Input, InputNumber, Switch,
  Cascader, message, Alert,
} from 'antd';
import { PlusOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { treeCategroy, addCategroy, updateCategroy, deleteCategroy } from '@/api/categoryApi';
import { getFormConfigList } from '@/api/systemFormConfig';
import { useModal } from '@/hooks/useModal';

const CATEGORY_TYPE = 6;

const ConfigCategory: React.FC = () => {
  const [treeList, setTreeList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const { deleteConfirm } = useModal();

  // edit dialog
  const [modalOpen, setModalOpen] = useState(false);
  const [modalTitle, setModalTitle] = useState('添加分类');
  const [editId, setEditId] = useState<number | null>(null);
  const [saving, setSaving] = useState(false);
  const [form] = Form.useForm();
  const [parentRow, setParentRow] = useState<any>(null);

  // form config select dialog
  const [formDialogOpen, setFormDialogOpen] = useState(false);
  const [formList, setFormList] = useState<any[]>([]);
  const [formLoading, setFormLoading] = useState(false);
  const [formPagination, setFormPagination] = useState({ current: 1, pageSize: 20, total: 0 });
  const [selectedFormId, setSelectedFormId] = useState<number | null>(null);
  const [currentRow, setCurrentRow] = useState<any>(null);

  const fetchTree = useCallback(async () => {
    setLoading(true);
    try {
      const data: any = await treeCategroy({ type: CATEGORY_TYPE, status: -1 });
      setTreeList(Array.isArray(data) ? data : []);
    } catch { message.error('获取配置分类失败'); }
    finally { setLoading(false); }
  }, []);

  useEffect(() => { fetchTree(); }, []);

  // cascader path helper
  const buildCascaderPath = (targetPid: number): number[] => {
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

  // add category (parent = row)
  const handleAdd = (row: any = { id: 0, name: '顶层目录' }) => {
    setEditId(null);
    setParentRow(row);
    setModalTitle('添加分类');
    form.resetFields();
    form.setFieldsValue({ pid: buildCascaderPath(row.id), sort: 0, status: true });
    setModalOpen(true);
  };

  // edit category
  const handleEdit = (row: any) => {
    setEditId(row.id);
    setParentRow(row);
    setModalTitle('编辑分类');
    form.resetFields();
    form.setFieldsValue({
      name: row.name, url: row.url || '', sort: row.sort || 0,
      status: !!row.status, pid: buildCascaderPath(row.pid),
    });
    setModalOpen(true);
  };

  // save
  const handleSave = async () => {
    try {
      const values = await form.validateFields();
      setSaving(true);
      const pidVal = Array.isArray(values.pid) ? values.pid[values.pid.length - 1] : (values.pid ?? 0);
      const submitData = {
        name: values.name, url: values.url || '',
        pid: Number(pidVal), sort: Number(values.sort ?? 0),
        status: values.status ? 1 : 0, type: CATEGORY_TYPE, extra: '',
      };
      if (editId) {
        await updateCategroy({ ...submitData, id: editId });
        message.success('更新分类成功');
      } else {
        await addCategroy(submitData);
        message.success('创建分类成功');
      }
      setModalOpen(false);
      fetchTree();
    } catch (e: any) {
      if (e?.errorFields) return;
      message.error(e?.message || '操作失败');
    } finally { setSaving(false); }
  };

  // delete
  const handleDelete = (row: any) => {
    deleteConfirm(async () => {
      await deleteCategroy({ id: row.id });
      message.success('删除成功');
      fetchTree();
    });
  };

  // open form config select dialog
  const handleOpenFormConfig = async (row: any) => {
    setCurrentRow(row);
    setSelectedFormId(row.extra ? Number(row.extra) : null);
    setFormDialogOpen(true);
    fetchFormList(1);
  };

  const fetchFormList = async (page = 1) => {
    setFormLoading(true);
    try {
      const res: any = await getFormConfigList({ page, limit: formPagination.pageSize });
      setFormList(res?.list || []);
      setFormPagination((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch { message.error('获取表单列表失败'); }
    finally { setFormLoading(false); }
  };

  // associate form
  const handleAssociateForm = async () => {
    if (!currentRow || selectedFormId == null) { message.warning('请选择表单'); return; }
    try {
      await updateCategroy({ ...currentRow, extra: String(selectedFormId), type: CATEGORY_TYPE });
      message.success('关联表单成功');
      setFormDialogOpen(false);
      fetchTree();
    } catch { message.error('关联失败'); }
  };

  const columns: ColumnsType<any> = [
    { title: '分类昵称', dataIndex: 'name', width: 300 },
    { title: '英文名称', dataIndex: 'url', ellipsis: true, width: 180 },
    { title: '已关联的表单', dataIndex: 'extra', ellipsis: true, width: 130 },
    { title: '启用状态', dataIndex: 'status', width: 100,
      render: (v: number) => v ? '是' : '否' },
    {
      title: '操作', width: 280, fixed: 'right',
      render: (_: any, record: any) => (
        <Space size="small" split={<span style={{ color: '#dcdfe6' }}>|</span>}>
          <a onClick={() => handleAdd(record)}>添加子目录</a>
          <a onClick={() => handleEdit(record)}>编辑</a>
          <a onClick={() => handleOpenFormConfig(record)}>配置列表</a>
          <a onClick={() => handleDelete(record)} style={{ color: '#ff4d4f' }}>删除</a>
        </Space>
      ),
    },
  ];

  const formColumns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 60 },
    { title: '表单名称', dataIndex: 'name', ellipsis: true },
    { title: '表单信息', dataIndex: 'info', ellipsis: true },
  ];

  return (
    <div className="divBox">
      <Card>
        <div style={{ marginBottom: 12 }}>
          <Button type="primary" icon={<PlusOutlined />}
            onClick={() => handleAdd({ id: 0, name: '顶层目录' })}>添加分类</Button>
        </div>
        <Alert type="success" showIcon
          message="温馨提示"
          description="添加一级分类以后，务必添加二级分类并配置表单，否则会出现渲染错误"
          style={{ marginBottom: 16, background: '#fff1e5', border: 'none', color: '#ff7d00' }}
        />
        <Table rowKey="id" columns={columns} dataSource={treeList} loading={loading}
          size="small" childrenColumnName="child" pagination={false}
          scroll={{ x: 900 }} defaultExpandAllRows />
      </Card>

      {/* 添加/编辑分类弹窗 */}
      <Modal title={modalTitle} open={modalOpen} onCancel={() => setModalOpen(false)}
        onOk={handleSave} confirmLoading={saving} destroyOnClose width={540}>
        <Form form={form} labelCol={{ span: 5 }} wrapperCol={{ span: 18 }} style={{ marginTop: 16 }}>
          <Form.Item label="父级" name="pid">
            <Cascader options={[{ id: 0, name: '顶级分类', child: treeList }]}
              fieldNames={{ label: 'name', value: 'id', children: 'child' }}
              changeOnSelect disabled style={{ width: '100%' }} />
          </Form.Item>
          <Form.Item label="分类名称" name="name" rules={[{ required: true, message: '请输入分类名称' }]}>
            <Input placeholder="分类名称" />
          </Form.Item>
          <Form.Item label="英文名称" name="url" rules={[{ required: true, message: '英文名称不能为空' }]}>
            <Input placeholder="URL" />
          </Form.Item>
          <Form.Item label="排序" name="sort">
            <InputNumber min={1} max={10} style={{ width: '100%' }} />
          </Form.Item>
          <Form.Item label="状态" name="status" valuePropName="checked">
            <Switch checkedChildren="开" unCheckedChildren="关" />
          </Form.Item>
        </Form>
      </Modal>

      {/* 选择表单配置弹窗 */}
      <Modal title="选择已配置的表单" open={formDialogOpen}
        onCancel={() => setFormDialogOpen(false)} footer={null} width={720}
        styles={{ body: { padding: 0, height: 600, overflow: 'auto' } }}>
        <div style={{ padding: '16px 16px 0' }}>
          <Table rowKey="id" columns={formColumns} dataSource={formList} loading={formLoading}
            size="small" scroll={{ y: 460 }}
            rowSelection={{ type: 'radio', selectedRowKeys: selectedFormId ? [selectedFormId] : [],
              onChange: (keys) => setSelectedFormId(keys[0] as number) }}
            pagination={{
              ...formPagination, showSizeChanger: false, size: 'small',
              onChange: (p) => fetchFormList(p),
            }} />
        </div>
        <div style={{ padding: '12px 20px' }}>
          <Button type="primary" block onClick={handleAssociateForm}>关联</Button>
        </div>
      </Modal>
    </div>
  );
};

export default ConfigCategory;

