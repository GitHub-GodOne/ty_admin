import React, { useState, useEffect, useCallback } from 'react';
import {
  Card, Table, Button, Space, Input, Modal, Form, message, Popconfirm,
  Select, InputNumber, Switch, Image, Radio, Checkbox,
} from 'antd';
import { PlusOutlined, SearchOutlined, ReloadOutlined, DeleteOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { groupList, groupSave, groupEdit, groupDelete } from '@/api/systemGroup';
import { getFormConfigList, getFormConfigInfo } from '@/api/systemFormConfig';
import {
  groupDataList as fetchGroupDataList,
  groupDataSave, groupDataEdit, groupDataDelete,
} from '@/api/systemGroupData';
import MaterialPicker from '@/components/MaterialPicker';

/** Inline image picker that opens MaterialPicker dialog */
const MaterialInput: React.FC<{
  value?: string; onChange?: (v: string) => void; limit?: number;
}> = ({ value, onChange, limit = 1 }) => {
  const [open, setOpen] = useState(false);
  return (
    <div>
      <div style={{ display: 'flex', gap: 8, flexWrap: 'wrap', marginBottom: value ? 8 : 0 }}>
        {value && (
          <div style={{ position: 'relative', width: 80, height: 80, border: '1px solid #d9d9d9', borderRadius: 4, overflow: 'hidden' }}>
            <img src={value} alt="" style={{ width: '100%', height: '100%', objectFit: 'cover' }} />
            <DeleteOutlined onClick={() => onChange?.('')}
              style={{ position: 'absolute', top: 2, right: 2, color: '#ff4d4f', cursor: 'pointer', fontSize: 14, background: 'rgba(255,255,255,0.8)', borderRadius: '50%', padding: 2 }} />
          </div>
        )}
      </div>
      <Button size="small" icon={<PlusOutlined />} onClick={() => setOpen(true)}>选择图片</Button>
      <MaterialPicker open={open} onCancel={() => setOpen(false)} limit={limit}
        onOk={(urls) => { onChange?.(urls[0] || ''); setOpen(false); }} />
    </div>
  );
};

const CombinedData: React.FC = () => {
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [keywords, setKeywords] = useState('');
  const [pagination, setPagination] = useState({ current: 1, pageSize: 20, total: 0 });
  const [modalVisible, setModalVisible] = useState(false);
  const [editId, setEditId] = useState<number | null>(null);
  const [form] = Form.useForm();
  // Form config select dialog
  const [formSelectVisible, setFormSelectVisible] = useState(false);
  const [formList, setFormList] = useState<any[]>([]);
  const [formLoading, setFormLoading] = useState(false);
  const [formPagination, setFormPagination] = useState({ current: 1, pageSize: 20, total: 0 });
  const [selectedFormRow, setSelectedFormRow] = useState<any>(null);
  // Data list modal
  const [dataModalVisible, setDataModalVisible] = useState(false);
  const [dataList, setDataList] = useState<any[]>([]);
  const [dataLoading, setDataLoading] = useState(false);
  const [dataPagination, setDataPagination] = useState({ current: 1, pageSize: 20, total: 0 });
  const [dataStatusFilter, setDataStatusFilter] = useState<number | undefined>(undefined);
  const [currentGroup, setCurrentGroup] = useState<any>(null);
  const [formConf, setFormConf] = useState<any>({ fields: [] });
  // Data add/edit dialog
  const [dataEditVisible, setDataEditVisible] = useState(false);
  const [dataEditId, setDataEditId] = useState<number | null>(null);
  const [dataForm] = Form.useForm();

  // ===== Group list CRUD =====
  const fetchList = useCallback(async (page = 1) => {
    setLoading(true);
    try {
      const res: any = await groupList({ page, limit: pagination.pageSize, keywords: keywords || undefined });
      setList(res?.list || []);
      setPagination((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch { message.error('获取组合数据失败'); }
    finally { setLoading(false); }
  }, [keywords, pagination.pageSize]);

  useEffect(() => { fetchList(1); }, []);

  const handleAdd = () => { setEditId(null); form.resetFields(); setModalVisible(true); };
  const handleEdit = (record: any) => {
    setEditId(record.id);
    form.setFieldsValue({ name: record.name, info: record.info, formId: record.formId });
    setModalVisible(true);
  };
  const handleDelete = async (id: number) => {
    try { await groupDelete({ id }); message.success('删除成功'); fetchList(pagination.current); } catch { /* */ }
  };
  const handleSave = async () => {
    try {
      const values = await form.validateFields();
      if (editId) { await groupEdit({ ...values, id: editId }); message.success('编辑成功'); }
      else { await groupSave(values); message.success('添加成功'); }
      setModalVisible(false); fetchList(pagination.current);
    } catch { /* */ }
  };

  // ===== Form config selection =====
  const fetchFormConfigList = async (page = 1) => {
    setFormLoading(true);
    try {
      const res: any = await getFormConfigList({ page, limit: formPagination.pageSize });
      setFormList(res?.list || []);
      setFormPagination((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch { message.error('获取表单列表失败'); }
    finally { setFormLoading(false); }
  };
  const handleOpenFormSelect = () => { setSelectedFormRow(null); setFormSelectVisible(true); fetchFormConfigList(1); };
  const handleConfirmFormSelect = () => {
    if (!selectedFormRow) { message.warning('请选择模板数据'); return; }
    form.setFieldsValue({ formId: selectedFormRow.id });
    setFormSelectVisible(false);
  };

  // ===== Data list (per group) =====
  const openDataList = async (record: any) => {
    setCurrentGroup(record);
    setDataModalVisible(true);
    setDataStatusFilter(undefined);
    // Load form config to get dynamic columns
    try {
      const configRes: any = await getFormConfigInfo({ id: record.formId });
      const conf = configRes?.content ? JSON.parse(configRes.content) : { fields: [] };
      setFormConf(conf);
    } catch { setFormConf({ fields: [] }); }
    fetchDataList(record.id, 1);
  };

  const fetchDataList = async (gid: number, page = 1, status?: number) => {
    setDataLoading(true);
    try {
      const res: any = await fetchGroupDataList({
        gid, page, limit: dataPagination.pageSize,
        status: status !== undefined ? status : undefined,
      });
      const rawList = res?.list || [];
      // Parse value JSON → flat row data
      const parsed = rawList.map((item: any) => {
        const row: any = { id: item.id, sort: item.sort, status: item.status };
        try {
          const val = typeof item.value === 'string' ? JSON.parse(item.value) : item.value;
          (val?.fields || []).forEach((f: any) => { row[f.name] = f.value; });
        } catch { /* */ }
        return row;
      });
      setDataList(parsed);
      setDataPagination((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch { message.error('获取数据列表失败'); }
    finally { setDataLoading(false); }
  };

  const handleDataStatusChange = (val: number | undefined) => {
    setDataStatusFilter(val);
    if (currentGroup) fetchDataList(currentGroup.id, 1, val);
  };

  const handleDataDelete = async (row: any) => {
    try {
      await groupDataDelete({ id: row.id });
      message.success('删除数据成功');
      if (currentGroup) fetchDataList(currentGroup.id, dataPagination.current, dataStatusFilter);
    } catch { /* */ }
  };

  // ===== Data add/edit =====
  const handleOpenDataEdit = (row?: any) => {
    dataForm.resetFields();
    if (row && row.id) {
      setDataEditId(row.id);
      const vals: any = {
        sort: row.sort ?? 1, status: !!row.status,
        _title: row._title ?? '', _info: row._info ?? '', _type: row._type ?? '',
      };
      formConf.fields?.forEach((f: any) => {
        const key = f.__vModel__ || f.name;
        if (key && row[key] !== undefined) vals[key] = row[key];
      });
      dataForm.setFieldsValue(vals);
    } else {
      setDataEditId(null);
      dataForm.setFieldsValue({ sort: 1, status: true, _title: '', _info: '', _type: 1 });
    }
    setDataEditVisible(true);
  };

  const handleDataSave = async () => {
    try {
      const values = await dataForm.validateFields();
      const fields: any[] = [
        { name: '_title', title: '标题', value: values._title ?? '' },
        { name: '_info', title: '简介', value: values._info ?? '' },
        { name: '_type', title: '类型', value: values._type ?? '' },
      ];
      formConf.fields?.forEach((f: any) => {
        const key = f.__vModel__ || f.name;
        if (key) fields.push({ name: key, title: key, value: values[key] ?? '' });
      });
      const pram = {
        gid: currentGroup.id,
        form: {
          fields,
          id: currentGroup.formId,
          sort: values.sort ?? 1,
          status: values.status ? 1 : 0,
        },
      };
      if (dataEditId) {
        await groupDataEdit(pram, dataEditId);
        message.success('编辑数据成功');
      } else {
        await groupDataSave(pram);
        message.success('添加数据成功');
      }
      setDataEditVisible(false);
      fetchDataList(currentGroup.id, dataPagination.current, dataStatusFilter);
    } catch { /* */ }
  };

  // ===== Column definitions =====
  const columns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 60 },
    { title: '数据组名称', dataIndex: 'name', width: 200 },
    { title: '简介', dataIndex: 'info', ellipsis: true },
    {
      title: '操作', width: 220,
      render: (_: any, record: any) => (
        <Space size="small">
          <a onClick={() => openDataList(record)}>数据列表</a>
          <a onClick={() => handleEdit(record)}>编辑</a>
          <Popconfirm title="确定删除?" onConfirm={() => handleDelete(record.id)}>
            <a style={{ color: '#ff4d4f' }}>删除</a>
          </Popconfirm>
        </Space>
      ),
    },
  ];

  const formColumns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 60 },
    { title: '表单名称', dataIndex: 'name', ellipsis: true },
    { title: '表单信息', dataIndex: 'info', ellipsis: true },
  ];

  // Dynamic data columns from form config
  const imgKeys = ['img', 'image', 'pic'];
  const typeMap: Record<string | number, string> = { 1: '精品推荐', 2: '热门榜单', 3: '首发新品', 4: '促销单品' };
  const dynamicDataColumns: ColumnsType<any> = [
    { title: '编号', dataIndex: 'id', width: 70 },
    { title: '标题', dataIndex: '_title', width: 120, ellipsis: true },
    { title: '简介', dataIndex: '_info', width: 150, ellipsis: true },
    { title: '类型', dataIndex: '_type', width: 80, render: (v: string) => typeMap[v] || v || '-' },
    ...(formConf.fields || []).map((f: any) => {
      const key = f.__vModel__ || f.name;
      const label = f.__config__?.label || key;
      return {
        title: label, dataIndex: key, ellipsis: true,
        render: (val: any) =>
          imgKeys.includes(key) && val
            ? <Image src={val} width={36} height={36} style={{ objectFit: 'cover' }} />
            : (val ?? '-'),
      };
    }),
    { title: '状态', dataIndex: 'status', width: 80,
      render: (v: number) => v === 1 ? '开启' : '关闭' },
    {
      title: '操作', width: 150,
      render: (_: any, record: any) => (
        <Space size="small">
          <a onClick={() => handleOpenDataEdit(record)}>编辑</a>
          <Popconfirm title="确定删除当前数据?" onConfirm={() => handleDataDelete(record)}>
            <a style={{ color: '#ff4d4f' }}>删除</a>
          </Popconfirm>
        </Space>
      ),
    },
  ];

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
      <Card bodyStyle={{ paddingBottom: 0 }}>
        <Space style={{ marginBottom: 16 }}>
          <Input placeholder="数据组名称" value={keywords} onChange={(e) => setKeywords(e.target.value)}
            onPressEnter={() => fetchList(1)} allowClear prefix={<SearchOutlined />} style={{ width: 240 }} />
          <Button type="primary" onClick={() => fetchList(1)}>搜索</Button>
          <Button icon={<ReloadOutlined />} onClick={() => { setKeywords(''); fetchList(1); }}>重置</Button>
        </Space>
      </Card>
      <Card>
        <div style={{ marginBottom: 16 }}>
          <Button type="primary" icon={<PlusOutlined />} onClick={handleAdd}>添加组合数据</Button>
        </div>
        <Table rowKey="id" columns={columns} dataSource={list} loading={loading} size="small"
          pagination={{
            ...pagination, showSizeChanger: true, showTotal: (t) => `共 ${t} 条`,
            onChange: (p, ps) => { setPagination((prev) => ({ ...prev, pageSize: ps })); fetchList(p); },
          }} />
      </Card>

      {/* 添加/编辑组合数据 */}
      <Modal title={editId ? '编辑组合数据' : '添加组合数据'} open={modalVisible} onOk={handleSave}
        onCancel={() => setModalVisible(false)} destroyOnClose width={500}>
        <Form form={form} labelCol={{ span: 6 }} wrapperCol={{ span: 17 }}>
          <Form.Item label="数据组名" name="name" rules={[{ required: true, message: '请输入数据组名称' }]}>
            <Input placeholder="请输入数据组名称" />
          </Form.Item>
          <Form.Item label="简介" name="info" rules={[{ required: true, message: '请输入简介' }]}>
            <Input.TextArea placeholder="请输入简介" rows={3} />
          </Form.Item>
          <Form.Item label="表单数据ID" required>
            <Space>
              <Form.Item name="formId" noStyle rules={[{ required: true, message: '请选择模板数据' }]}>
                <Input placeholder="表单ID" style={{ width: 160 }} readOnly />
              </Form.Item>
              <Button type="primary" onClick={handleOpenFormSelect}>选择模板数据</Button>
            </Space>
          </Form.Item>
        </Form>
      </Modal>

      {/* 数据列表 */}
      <Modal title={`数据列表 - ${currentGroup?.name || ''}`} open={dataModalVisible}
        onCancel={() => setDataModalVisible(false)} footer={null} width={900}
        styles={{ body: { maxHeight: 600, overflow: 'auto' } }}>
        <div style={{ marginBottom: 12 }}>
          <Space style={{ marginBottom: 12 }}>
            <span>状态：</span>
            <Select allowClear placeholder="状态" style={{ width: 120 }}
              value={dataStatusFilter} onChange={handleDataStatusChange}>
              <Select.Option value={1}>开启</Select.Option>
              <Select.Option value={2}>关闭</Select.Option>
            </Select>
          </Space>
        </div>
        <Button type="primary" style={{ marginBottom: 12 }} onClick={() => handleOpenDataEdit()}>添加数据</Button>
        <Table rowKey="id" columns={dynamicDataColumns} dataSource={dataList} loading={dataLoading}
          size="small" pagination={{
            ...dataPagination, showSizeChanger: true, showTotal: (t) => `共 ${t} 条`,
            onChange: (p, ps) => {
              setDataPagination((prev) => ({ ...prev, pageSize: ps }));
              if (currentGroup) fetchDataList(currentGroup.id, p, dataStatusFilter);
            },
          }} />
      </Modal>

      {/* 添加/编辑数据项 */}
      <Modal title={dataEditId ? '编辑数据' : '添加数据'} open={dataEditVisible}
        onOk={handleDataSave} onCancel={() => setDataEditVisible(false)} destroyOnClose width={700}>
        <Form form={dataForm} labelCol={{ span: 5 }} wrapperCol={{ span: 18 }}>
          <Form.Item label="排序" name="sort" rules={[{ required: true, message: '排序不能为空' }]}>
            <InputNumber min={0} style={{ width: '100%' }} />
          </Form.Item>
          <Form.Item label="状态" name="status" valuePropName="checked">
            <Switch checkedChildren="显示" unCheckedChildren="隐藏" />
          </Form.Item>
          <Form.Item label="标题" name="_title" rules={[{ required: true, message: '请输入标题' }]}>
            <Input placeholder="请输入标题" />
          </Form.Item>
          <Form.Item label="简介" name="_info">
            <Input.TextArea placeholder="请输入简介" rows={3} />
          </Form.Item>
          <Form.Item label="类型" name="_type" rules={[{ required: true, message: '请选择类型' }]}>
            <Radio.Group>
              <Radio value={1}>精品推荐</Radio>
              <Radio value={2}>热门榜单</Radio>
              <Radio value={3}>首发新品</Radio>
              <Radio value={4}>促销单品</Radio>
            </Radio.Group>
          </Form.Item>
          {formConf.fields?.map((f: any) => {
            const key = f.__vModel__ || f.name;
            const label = f.__config__?.label || key;
            const tag = f.__config__?.tag;
            const required = f.__config__?.required ?? false;
            const isSwitchTag = tag === 'el-switch';
            let child: React.ReactNode;
            switch (tag) {
              case 'self-upload':
                child = <MaterialInput limit={1} />;
                break;
              case 'el-input-number':
                child = <InputNumber min={f.min} max={f.max} step={f.step} style={{ width: '100%' }} />;
                break;
              case 'el-select':
                child = <Select placeholder={f.placeholder} options={f.__slot__?.options?.map((o: any) => ({ label: o.label, value: o.value }))} />;
                break;
              case 'el-radio-group':
                child = <Radio.Group>{f.__slot__?.options?.map((o: any) => <Radio key={String(o.value)} value={o.value}>{o.label}</Radio>)}</Radio.Group>;
                break;
              case 'el-checkbox-group':
                child = <Checkbox.Group options={f.__slot__?.options?.map((o: any) => ({ label: o.label, value: o.value }))} />;
                break;
              case 'el-switch':
                child = <Switch />;
                break;
              case 'el-input':
                child = f.type === 'textarea' || f.autosize
                  ? <Input.TextArea placeholder={f.placeholder || `请输入${label}`} autoSize={typeof f.autosize === 'object' ? f.autosize : { minRows: 3, maxRows: 6 }} />
                  : <Input placeholder={f.placeholder || `请输入${label}`} />;
                break;
              default:
                child = <Input placeholder={f.__config__?.placeholder || `请输入${label}`} />;
            }
            return (
              <Form.Item key={key} label={label} name={key}
                valuePropName={isSwitchTag ? 'checked' : 'value'}
                rules={required ? [{ required: true, message: `请输入${label}` }] : undefined}>
                {child}
              </Form.Item>
            );
          })}
        </Form>
      </Modal>

      {/* 选择模板数据 */}
      <Modal title="选择模板数据" open={formSelectVisible}
        onCancel={() => setFormSelectVisible(false)} width={700}
        footer={<Space>
          <Button onClick={() => setFormSelectVisible(false)}>取消</Button>
          <Button type="primary" onClick={handleConfirmFormSelect}>确定选择</Button>
        </Space>}>
        <Table rowKey="id" columns={formColumns} dataSource={formList} loading={formLoading}
          size="small" rowClassName={(record) => record.id === selectedFormRow?.id ? 'ant-table-row-selected' : ''}
          onRow={(record) => ({ onClick: () => setSelectedFormRow(record), style: { cursor: 'pointer' } })}
          pagination={{ ...formPagination, showSizeChanger: false, size: 'small',
            onChange: (p) => fetchFormConfigList(p) }} />
      </Modal>
    </div>
  );
};

export default CombinedData;
