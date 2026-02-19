import React, { useState, useEffect, useCallback } from 'react';
import { Card, Table, Form, Input, Button, Space, Tag, Modal, Switch, Tree, Checkbox, message } from 'antd';
import { PlusOutlined, SearchOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { getRoleList as roleListApi, addRole as roleSaveApi, updateRole as roleEditApi, delRole as roleDeleteApi, getInfo as roleInfoApi, menuCacheList } from '@/api/role';
import { useModal } from '@/hooks/useModal';
import { usePagination } from '@/hooks/usePagination';

/** 将后端菜单树转为 antd Tree 的 treeData */
function buildTreeData(list: any[]): any[] {
  return (list || []).map((item: any) => ({
    key: item.id,
    title: item.name,
    children: item.childList?.length ? buildTreeData(item.childList) : [],
  }));
}

/** 从菜单树中提取所有已勾选的叶子节点 id */
function getCheckedLeafIds(list: any[], allIds: number[] = []): number[] {
  (list || []).forEach((item: any) => {
    if (item.childList?.length) {
      getCheckedLeafIds(item.childList, allIds);
    } else if (item.checked) {
      allIds.push(item.id);
    }
  });
  return allIds;
}

/** 收集树中所有节点 key */
function getAllTreeKeys(treeData: any[]): number[] {
  const keys: number[] = [];
  const walk = (list: any[]) => {
    (list || []).forEach((node: any) => {
      keys.push(node.key);
      if (node.children?.length) walk(node.children);
    });
  };
  walk(treeData);
  return keys;
}

const OperationRole: React.FC = () => {
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [keywords, setKeywords] = useState('');
  const [modalVisible, setModalVisible] = useState(false);
  const [editingId, setEditingId] = useState<number | null>(null);
  const [confirmLoading, setConfirmLoading] = useState(false);
  const [form] = Form.useForm();
  const { deleteConfirm } = useModal();
  const { pagination, setTotal, antdPagination } = usePagination();
  const [menuTree, setMenuTree] = useState<any[]>([]);
  const [checkedKeys, setCheckedKeys] = useState<number[]>([]);
  const [expandedKeys, setExpandedKeys] = useState<number[]>([]);
  const [checkAll, setCheckAll] = useState(false);
  const [expandAll, setExpandAll] = useState(true);

  const fetchList = useCallback(async () => {
    setLoading(true);
    try {
      const res: any = await roleListApi({ roleName: keywords, page: pagination.page, limit: pagination.limit });
      setList(res?.list || []);
      setTotal(res?.total || 0);
    } catch { /* handled */ }
    finally { setLoading(false); }
  }, [keywords, pagination.page, pagination.limit]);

  useEffect(() => { fetchList(); }, [fetchList]);

  const handleAdd = async () => {
    setEditingId(null);
    form.resetFields();
    setCheckedKeys([]);
    setCheckAll(false);
    try {
      const res: any = await menuCacheList();
      const tree = buildTreeData(res || []);
      setMenuTree(tree);
      setExpandedKeys(getAllTreeKeys(tree));
      setExpandAll(true);
    } catch { /* handled */ }
    setModalVisible(true);
  };

  const handleEdit = async (id: number) => {
    try {
      const res: any = await roleInfoApi(id);
      setEditingId(id);
      form.setFieldsValue({ roleName: res?.roleName, status: res?.status === true || res?.status === 1 });
      const tree = res?.menuList || [];
      const treeData = buildTreeData(tree);
      setMenuTree(treeData);
      const leafIds = getCheckedLeafIds(tree);
      setCheckedKeys(leafIds);
      const allKeys = getAllTreeKeys(treeData);
      setExpandedKeys(allKeys);
      setExpandAll(true);
      setCheckAll(leafIds.length >= allKeys.length);
      setModalVisible(true);
    } catch { message.error('获取角色详情失败'); }
  };

  const handleDelete = (id: number) => {
    deleteConfirm(async () => { await roleDeleteApi({ id }); message.success('删除成功'); fetchList(); });
  };

  const handleModalOk = async () => {
    try {
      const values = await form.validateFields();
      setConfirmLoading(true);
      const params = { roleName: values.roleName, rules: checkedKeys.join(','), status: values.status ? true : false };
      if (editingId) {
        await roleEditApi({ ...params, id: editingId });
        message.success('编辑成功');
      } else {
        await roleSaveApi(params);
        message.success('添加成功');
      }
      setModalVisible(false);
      fetchList();
    } catch { /* validation */ }
    finally { setConfirmLoading(false); }
  };

  const columns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 80 },
    { title: '角色名称', dataIndex: 'roleName', width: 150 },
    { title: '状态', dataIndex: 'status', width: 80, render: (val: any) => <Tag color={val ? 'green' : 'default'}>{val ? '启用' : '禁用'}</Tag> },
    { title: '创建时间', dataIndex: 'createTime', width: 180 },
    {
      title: '操作', width: 150, fixed: 'right',
      render: (_: any, record: any) => (
        <Space>
          <a onClick={() => handleEdit(record.id)}>编辑</a>
          <a onClick={() => handleDelete(record.id)} style={{ color: '#ff4d4f' }}>删除</a>
        </Space>
      ),
    },
  ];

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
      <Card bodyStyle={{ paddingBottom: 0 }}>
        <Form layout="inline" style={{ marginBottom: 16 }}>
          <Form.Item><Input placeholder="角色名称" value={keywords} onChange={(e) => setKeywords(e.target.value)} allowClear style={{ width: 200 }} /></Form.Item>
          <Form.Item><Button type="primary" icon={<SearchOutlined />} onClick={fetchList}>搜索</Button></Form.Item>
          <Form.Item><Button onClick={() => setKeywords('')}>重置</Button></Form.Item>
          <Form.Item><Button type="primary" icon={<PlusOutlined />} onClick={handleAdd}>添加角色</Button></Form.Item>
        </Form>
      </Card>
      <Card>
        <Table rowKey="id" columns={columns} dataSource={list} loading={loading} pagination={antdPagination} scroll={{ x: 800 }} size="small" />
      </Card>
      <Modal title={editingId ? '编辑角色' : '添加角色'} open={modalVisible} onOk={handleModalOk}
        onCancel={() => setModalVisible(false)} confirmLoading={confirmLoading} destroyOnClose width={600}>
        <Form form={form} labelCol={{ span: 5 }} wrapperCol={{ span: 17 }} initialValues={{ status: true }}>
          <Form.Item label="角色名称" name="roleName" rules={[{ required: true, message: '请输入角色名称' }]}>
            <Input placeholder="请输入角色名称" />
          </Form.Item>
          <Form.Item label="状态" name="status" valuePropName="checked">
            <Switch checkedChildren="启用" unCheckedChildren="禁用" />
          </Form.Item>
          <Form.Item label="菜单权限">
            <div style={{ marginBottom: 8 }}>
              <Checkbox checked={checkAll} onChange={(e) => {
                const checked = e.target.checked;
                setCheckAll(checked);
                setCheckedKeys(checked ? getAllTreeKeys(menuTree) : []);
              }}>全选/全不选</Checkbox>
              <Checkbox checked={expandAll} onChange={(e) => {
                const checked = e.target.checked;
                setExpandAll(checked);
                setExpandedKeys(checked ? getAllTreeKeys(menuTree) : []);
              }}>展开/折叠</Checkbox>
            </div>
            <div style={{ maxHeight: 300, overflow: 'auto', border: '1px solid #d9d9d9', borderRadius: 6, padding: 8 }}>
              {menuTree.length > 0 ? (
                <Tree checkable treeData={menuTree} checkedKeys={checkedKeys}
                  expandedKeys={expandedKeys}
                  onExpand={(keys: any) => {
                    setExpandedKeys(keys as number[]);
                    setExpandAll(keys.length >= getAllTreeKeys(menuTree).length);
                  }}
                  onCheck={(keys: any) => {
                    setCheckedKeys(keys as number[]);
                    setCheckAll((keys as number[]).length >= getAllTreeKeys(menuTree).length);
                  }} />
              ) : <span style={{ color: '#999' }}>暂无菜单数据</span>}
            </div>
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
};

export default OperationRole;
