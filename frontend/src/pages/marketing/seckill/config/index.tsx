import React, { useState, useEffect, useCallback } from 'react';
import { Card, Table, Form, Input, Select, Button, Space, Switch, Modal, message, Popconfirm, TimePicker, Image } from 'antd';
import { PlusOutlined, SearchOutlined, ReloadOutlined, DeleteOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { seckillListApi, seckillInfoApi, seckillSaveApi, seckillUpdateApi, seckillDeleteApi, seckillConfigStatusApi } from '@/api/marketing';
import MaterialPicker from '@/components/MaterialPicker';
import { useNavigate } from 'react-router-dom';
import dayjs from 'dayjs';

const SeckillConfig: React.FC = () => {
  const navigate = useNavigate();
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [name, setName] = useState('');
  const [status, setStatus] = useState<string | undefined>(undefined);
  const [pagination, setPagination] = useState({ current: 1, pageSize: 20, total: 0 });
  const [modalOpen, setModalOpen] = useState(false);
  const [editId, setEditId] = useState<number | null>(null);
  const [formLoading, setFormLoading] = useState(false);
  const [form] = Form.useForm();
  const [coverImage, setCoverImage] = useState('');
  const [sliderImages, setSliderImages] = useState<string[]>([]);
  const [coverPickerOpen, setCoverPickerOpen] = useState(false);
  const [sliderPickerOpen, setSliderPickerOpen] = useState(false);

  const fetchList = useCallback(async (page = 1) => {
    setLoading(true);
    try {
      const res = await seckillListApi({ page, limit: pagination.pageSize, name: name || undefined, status });
      setList(res?.list || []);
      setPagination((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch {
      message.error('获取秒杀配置列表失败');
    } finally {
      setLoading(false);
    }
  }, [name, status, pagination.pageSize]);

  useEffect(() => { fetchList(1); }, []);

  const handleReset = () => { setName(''); setStatus(undefined); };

  const handleDelete = async (id: number) => {
    try {
      await seckillDeleteApi({ id });
      message.success('删除成功');
      fetchList(pagination.current);
    } catch { /* noop */ }
  };

  const handleStatusChange = async (row: any) => {
    try {
      await seckillConfigStatusApi(row.id, { status: row.status === '1' ? '0' : '1' });
      message.success('修改成功');
      fetchList(pagination.current);
    } catch { /* noop */ }
  };

  const handleEdit = async (id: number) => {
    setEditId(id);
    setFormLoading(true);
    setModalOpen(true);
    try {
      const res = await seckillInfoApi({ id });
      if (res) {
        const timeArr = res.time ? res.time.split(',') : [];
        form.setFieldsValue({
          name: res.name,
          time: timeArr.length === 2 ? [dayjs(timeArr[0], 'HH:mm'), dayjs(timeArr[1], 'HH:mm')] : undefined,
          sort: res.sort,
          status: res.status,
        });
        setCoverImage(res.img || '');
        const imgs = res.silderImgs || '';
        setSliderImages(imgs ? (typeof imgs === 'string' ? (imgs.startsWith('[') ? JSON.parse(imgs) : imgs.split(',')) : imgs) : []);
      }
    } catch { /* noop */ } finally {
      setFormLoading(false);
    }
  };

  const handleAdd = () => {
    setEditId(null);
    form.resetFields();
    setCoverImage('');
    setSliderImages([]);
    setModalOpen(true);
  };

  const handleSave = async () => {
    try {
      const values = await form.validateFields();
      const timeRange = values.time;
      if (timeRange && timeRange[0].hour() > timeRange[1].hour()) {
        message.error('请填写正确的时间范围');
        return;
      }
      const data = {
        name: values.name,
        time: timeRange ? `${timeRange[0].format('HH:mm')},${timeRange[1].format('HH:mm')}` : '',
        sort: values.sort || 0,
        status: values.status || '0',
        img: coverImage || undefined,
        silderImgs: sliderImages.length > 0 ? sliderImages.join(',') : undefined,
      };
      if (editId) {
        await seckillUpdateApi({ id: editId }, data);
      } else {
        await seckillSaveApi(data);
      }
      message.success('操作成功');
      setModalOpen(false);
      fetchList(pagination.current);
    } catch { /* noop */ }
  };

  const columns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 60 },
    {
      title: '秒杀名称', dataIndex: 'name', width: 150,
      render: (v: string, r: any) => (
        <a onClick={() => navigate(`/marketing/seckill/list?timeId=${r.id}`)}>{v}</a>
      ),
    },
    {
      title: '秒杀时段', dataIndex: 'time', width: 150,
      render: (v: string) => v ? v.split(',').join(' - ') : '-',
    },
    {
      title: '状态', width: 120,
      render: (_: any, record: any) => (
        <Switch size="small" checked={String(record.status) === '1'}
          checkedChildren="开启" unCheckedChildren="关闭"
          onChange={() => handleStatusChange(record)} />
      ),
    },
    { title: '创建时间', dataIndex: 'createTime', width: 160 },
    {
      title: '操作', width: 200, fixed: 'right',
      render: (_: any, record: any) => (
        <Space size="small">
          <a onClick={() => handleEdit(record.id)}>编辑</a>
          <Popconfirm title="永久删除该配置?" onConfirm={() => handleDelete(record.id)}>
            <a style={{ color: '#ff4d4f' }}>删除</a>
          </Popconfirm>
          <a onClick={() => navigate(`/marketing/seckill/creatSeckill?timeId=${record.id}`)}>添加商品</a>
        </Space>
      ),
    },
  ];

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
      <Card bodyStyle={{ paddingBottom: 0 }}>
        <Form layout="inline" style={{ marginBottom: 16 }}>
          <Form.Item label="秒杀状态">
            <Select value={status} onChange={(v) => setStatus(v)} placeholder="请选择"
              allowClear style={{ width: 160 }}>
              <Select.Option value="0">关闭</Select.Option>
              <Select.Option value="1">开启</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item label="秒杀名称">
            <Input placeholder="请输入秒杀名称" value={name} onChange={(e) => setName(e.target.value)}
              onPressEnter={() => fetchList(1)} allowClear style={{ width: 200 }} />
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
          <Button type="primary" icon={<PlusOutlined />} onClick={handleAdd}>添加秒杀配置</Button>
        </div>
        <Table rowKey="id" columns={columns} dataSource={list} loading={loading} size="small"
          scroll={{ x: 900 }}
          pagination={{ ...pagination, showSizeChanger: true, pageSizeOptions: ['10', '20', '30', '40'],
            showTotal: (t: number) => `共 ${t} 条`,
            onChange: (p: number, ps: number) => { setPagination((prev) => ({ ...prev, pageSize: ps })); fetchList(p); } }} />
      </Card>

      <Modal title={editId ? '编辑数据' : '添加数据'} open={modalOpen} onOk={handleSave}
        onCancel={() => setModalOpen(false)} width={600} destroyOnClose confirmLoading={formLoading}>
        <Form form={form} labelCol={{ span: 6 }} wrapperCol={{ span: 16 }} preserve={false}>
          <Form.Item label="秒杀名称" name="name" rules={[{ required: true, message: '请输入秒杀名称' }]}>
            <Input placeholder="请输入秒杀名称" />
          </Form.Item>
          <Form.Item label="秒杀时段" name="time" rules={[{ required: true, message: '请选择秒杀时段' }]}>
            <TimePicker.RangePicker format="HH:mm" style={{ width: '100%' }} />
          </Form.Item>
          <Form.Item label="排序" name="sort" initialValue={0}>
            <Input type="number" placeholder="请输入排序" />
          </Form.Item>
          <Form.Item label="状态" name="status" initialValue="1">
            <Select>
              <Select.Option value="1">开启</Select.Option>
              <Select.Option value="0">关闭</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item label="主图">
            <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
              {coverImage && (
                <div style={{ position: 'relative' }}>
                  <Image src={coverImage} width={80} height={80} style={{ borderRadius: 4, objectFit: 'cover' }} />
                  <DeleteOutlined style={{ position: 'absolute', top: -6, right: -6, color: '#ff4d4f', cursor: 'pointer' }}
                    onClick={() => setCoverImage('')} />
                </div>
              )}
              {!coverImage && (
                <Button icon={<PlusOutlined />} onClick={() => setCoverPickerOpen(true)}>选择主图</Button>
              )}
            </div>
            <MaterialPicker open={coverPickerOpen} onCancel={() => setCoverPickerOpen(false)}
              onOk={(urls) => { setCoverImage(urls[0] || ''); setCoverPickerOpen(false); }} />
          </Form.Item>
          <Form.Item label="幻灯片">
            <div style={{ display: 'flex', gap: 8, flexWrap: 'wrap', alignItems: 'center' }}>
              {sliderImages.map((img, idx) => (
                <div key={idx} style={{ position: 'relative' }}>
                  <Image src={img} width={80} height={80} style={{ borderRadius: 4, objectFit: 'cover' }} />
                  <DeleteOutlined style={{ position: 'absolute', top: -6, right: -6, color: '#ff4d4f', cursor: 'pointer' }}
                    onClick={() => setSliderImages((prev) => prev.filter((_, i) => i !== idx))} />
                </div>
              ))}
              {sliderImages.length < 10 && (
                <Button icon={<PlusOutlined />} onClick={() => setSliderPickerOpen(true)}>选择幻灯片</Button>
              )}
            </div>
            <div style={{ color: '#999', fontSize: 12, marginTop: 4 }}>最多上传10张</div>
            <MaterialPicker open={sliderPickerOpen} onCancel={() => setSliderPickerOpen(false)} multiple
              limit={10 - sliderImages.length}
              onOk={(urls) => { setSliderImages((prev) => [...prev, ...urls].slice(0, 10)); setSliderPickerOpen(false); }} />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
};

export default SeckillConfig;
