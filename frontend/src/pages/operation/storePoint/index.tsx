import React, { useState, useEffect, useCallback, useRef } from 'react';
import { Card, Table, Tabs, Form, Input, Button, Space, Tag, Modal, Switch, Cascader, TimePicker, message } from 'antd';
import { PlusOutlined, SearchOutlined, EnvironmentOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { storeListApi, storeGetCountApi, storeSaveApi, storeDeleteApi, storeUpdateApi, storeInfoApi, storeUpdateStatusApi, allDeleteApi, storeRecoveryApi } from '@/api/storePoint';
import { cityListTree } from '@/api/logistics';
import { getTxMapKeyApi } from '@/api/systemConfig';
import { useModal } from '@/hooks/useModal';
import { usePagination } from '@/hooks/usePagination';
import MaterialPicker from '@/components/MaterialPicker';
import dayjs from 'dayjs';

const OperationStore: React.FC = () => {
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [keywords, setKeywords] = useState('');
  const [activeTab, setActiveTab] = useState('1');
  const [counts, setCounts] = useState<any>({});
  const [modalVisible, setModalVisible] = useState(false);
  const [editingId, setEditingId] = useState<number | null>(null);
  const [confirmLoading, setConfirmLoading] = useState(false);
  const [pickerOpen, setPickerOpen] = useState(false);
  const [imageUrl, setImageUrl] = useState('');
  const [form] = Form.useForm();
  const { deleteConfirm } = useModal();
  const { pagination, setTotal, antdPagination } = usePagination();
  const [mapVisible, setMapVisible] = useState(false);
  const [mapKeyUrl, setMapKeyUrl] = useState('');
  const [cityOptions, setCityOptions] = useState<any[]>([]);

  // Load city tree and map key on mount
  useEffect(() => {
    cityListTree().then((res: any) => setCityOptions(Array.isArray(res) ? res : [])).catch(() => {});
    getTxMapKeyApi().then((res: any) => {
      if (res?.value) {
        setMapKeyUrl(`https://apis.map.qq.com/tools/locpicker?type=1&key=${res.value}&referer=myapp`);
      }
    }).catch(() => {});
  }, []);

  // Listen for map picker message
  useEffect(() => {
    const handler = (event: MessageEvent) => {
      const loc = event.data;
      if (loc && loc.module === 'locationPicker') {
        form.setFieldsValue({ latitude: `${loc.latlng.lng},${loc.latlng.lat}` });
        setMapVisible(false);
      }
    };
    window.addEventListener('message', handler);
    return () => window.removeEventListener('message', handler);
  }, [form]);

  const statusMap: Record<string, number> = { '1': 1, '2': 0, '3': -1 };

  const fetchCounts = useCallback(async () => {
    try { const res: any = await storeGetCountApi({ keywords }); setCounts(res || {}); } catch { /* handled */ }
  }, [keywords]);

  const fetchList = useCallback(async () => {
    setLoading(true);
    try {
      const params: any = { page: pagination.page, limit: pagination.limit };
      if (keywords) params.keywords = keywords;
      const status = statusMap[activeTab];
      if (status !== undefined) params.status = status;
      const res: any = await storeListApi(params);
      setList(res?.list || []);
      setTotal(res?.total || 0);
    } catch { /* handled */ }
    finally { setLoading(false); }
  }, [keywords, activeTab, pagination.page, pagination.limit]);

  useEffect(() => { fetchList(); fetchCounts(); }, [fetchList]);

  const handleAdd = () => {
    setEditingId(null); form.resetFields(); setImageUrl(''); setModalVisible(true);
  };

  const handleEdit = async (id: number) => {
    try {
      const res: any = await storeInfoApi({ id });
      setEditingId(id);
      const addressArr = res?.address ? res.address.split(',') : [];
      const dayTimeArr = res?.dayTime ? res.dayTime.split(',') : [];
      form.setFieldsValue({
        name: res?.name, introduction: res?.introduction, phone: res?.phone,
        address: addressArr,
        detailedAddress: res?.detailedAddress,
        latitude: res?.latitude,
        dayTime: dayTimeArr.length === 2 ? [dayjs(dayTimeArr[0], 'HH:mm:ss'), dayjs(dayTimeArr[1], 'HH:mm:ss')] : undefined,
      });
      setImageUrl(res?.image || '');
      setModalVisible(true);
    } catch { message.error('获取门店详情失败'); }
  };

  const handleDelete = (id: number) => {
    deleteConfirm(async () => { await storeDeleteApi({ id }); message.success('删除成功'); fetchList(); fetchCounts(); });
  };

  const handleCompletelyDelete = (id: number) => {
    deleteConfirm(async () => { await allDeleteApi({ id }); message.success('彻底删除成功'); fetchList(); fetchCounts(); }, '确定彻底删除该门店吗？此操作不可恢复！');
  };

  const handleRecovery = async (id: number) => {
    try { await storeRecoveryApi({ id }); message.success('恢复成功'); fetchList(); fetchCounts(); } catch { /* handled */ }
  };

  const handleToggleStatus = async (record: any) => {
    try { await storeUpdateStatusApi({ id: record.id, status: record.isShow === 1 ? 0 : 1 }); message.success('状态已更新'); fetchList(); fetchCounts(); } catch { /* handled */ }
  };

  const handleModalOk = async () => {
    try {
      const values = await form.validateFields();
      setConfirmLoading(true);
      const address = Array.isArray(values.address) ? values.address.join(',') : values.address;
      const dayTime = Array.isArray(values.dayTime)
        ? values.dayTime.map((t: any) => (t ? t.format('HH:mm:ss') : '')).join(',')
        : values.dayTime || '';
      const params = { ...values, address, dayTime, image: imageUrl };
      delete params.dayTime_raw;
      if (editingId) { await storeUpdateApi(params, editingId); message.success('编辑成功'); }
      else { await storeSaveApi(params); message.success('添加成功'); }
      setModalVisible(false); fetchList(); fetchCounts();
    } catch { /* validation */ }
    finally { setConfirmLoading(false); }
  };

  const columns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 60 },
    { title: '门店名称', dataIndex: 'name', width: 150, ellipsis: true },
    { title: '门店简介', dataIndex: 'introduction', ellipsis: true, width: 180 },
    { title: '手机号码', dataIndex: 'phone', width: 120 },
    { title: '地址', dataIndex: 'address', ellipsis: true, width: 200 },
    { title: '营业时间', dataIndex: 'dayTime', width: 140 },
    { title: '状态', dataIndex: 'isShow', width: 80, render: (v: any, r: any) => activeTab !== '3'
      ? <Switch size="small" checked={v === 1} onChange={() => handleToggleStatus(r)} />
      : <Tag color="red">已删除</Tag> },
    {
      title: '操作', width: 180, fixed: 'right',
      render: (_: any, record: any) => activeTab === '3' ? (
        <Space size="small">
          <a onClick={() => handleRecovery(record.id)}>恢复</a>
          <a style={{ color: '#ff4d4f' }} onClick={() => handleCompletelyDelete(record.id)}>彻底删除</a>
        </Space>
      ) : (
        <Space size="small">
          <a onClick={() => handleEdit(record.id)}>编辑</a>
          <a style={{ color: '#ff4d4f' }} onClick={() => handleDelete(record.id)}>删除</a>
        </Space>
      ),
    },
  ];

  const tabItems = [
    { key: '1', label: `显示中(${counts.show ?? 0})` },
    { key: '2', label: `已隐藏(${counts.hide ?? 0})` },
    { key: '3', label: `回收站(${counts.recycle ?? 0})` },
  ];

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
      <Card bodyStyle={{ paddingBottom: 0 }}>
        <Form layout="inline" style={{ marginBottom: 16 }}>
          <Form.Item><Input placeholder="门店名称/手机号" value={keywords} onChange={(e) => setKeywords(e.target.value)} allowClear style={{ width: 200 }} /></Form.Item>
          <Form.Item><Button type="primary" icon={<SearchOutlined />} onClick={() => { fetchList(); fetchCounts(); }}>搜索</Button></Form.Item>
          <Form.Item><Button onClick={() => setKeywords('')}>重置</Button></Form.Item>
          <Form.Item><Button type="primary" icon={<PlusOutlined />} onClick={handleAdd}>添加门店</Button></Form.Item>
        </Form>
      </Card>
      <Card>
        <Tabs activeKey={activeTab} onChange={(k) => setActiveTab(k)} items={tabItems} />
        <Table rowKey="id" columns={columns} dataSource={list} loading={loading} pagination={antdPagination} scroll={{ x: 1200 }} size="small" />
      </Card>

      <Modal title={editingId ? '编辑提货点' : '添加提货点'} open={modalVisible} onOk={handleModalOk}
        onCancel={() => setModalVisible(false)} confirmLoading={confirmLoading} destroyOnClose width={700}>
        <Form form={form} labelCol={{ span: 5 }} wrapperCol={{ span: 17 }}>
          <Form.Item label="提货点名称" name="name" rules={[{ required: true, message: '请输入提货点名称' }]}>
            <Input placeholder="请输入提货点名称" maxLength={40} />
          </Form.Item>
          <Form.Item label="提货点简介" name="introduction">
            <Input.TextArea rows={2} placeholder="请输入提货点简介" maxLength={100} />
          </Form.Item>
          <Form.Item label="手机号码" name="phone" rules={[
            { required: true, message: '请输入手机号码' },
            { pattern: /^1[3456789]\d{9}$/, message: '手机号格式不正确' },
          ]}>
            <Input placeholder="请输入手机号码" />
          </Form.Item>
          <Form.Item label="提货点地址" name="address" rules={[{ required: true, message: '请选择提货点地址' }]}>
            <Cascader options={cityOptions} fieldNames={{ label: 'name', value: 'name', children: 'child' }}
              placeholder="请选择省市区" expandTrigger="hover" />
          </Form.Item>
          <Form.Item label="详细地址" name="detailedAddress" rules={[{ required: true, message: '请输入详细地址' }]}>
            <Input placeholder="请输入详细地址" />
          </Form.Item>
          <Form.Item label="营业时间" name="dayTime">
            <TimePicker.RangePicker format="HH:mm:ss" />
          </Form.Item>
          <Form.Item label="提货点logo">
            <Space>
              {imageUrl ? (
                <img src={imageUrl} alt="logo" style={{ width: 80, height: 80, objectFit: 'cover', borderRadius: 4, border: '1px solid #d9d9d9' }} />
              ) : null}
              <Button size="small" onClick={() => setPickerOpen(true)}>选择图片</Button>
            </Space>
          </Form.Item>
          <Form.Item label="经纬度" name="latitude" rules={[{ required: true, message: '请选择经纬度' }]}>
            <Input placeholder="请点击查找位置选择" readOnly
              addonAfter={<Button type="link" size="small" icon={<EnvironmentOutlined />}
                style={{ padding: 0 }} onClick={() => setMapVisible(true)}>查找位置</Button>} />
          </Form.Item>
        </Form>
      </Modal>
      <Modal title="选择位置" open={mapVisible} onCancel={() => setMapVisible(false)} footer={null} width={560}
        styles={{ body: { height: 500, padding: 0 } }} destroyOnClose>
        {mapKeyUrl ? (
          <iframe src={mapKeyUrl} width="100%" height="100%" frameBorder={0} style={{ display: 'block' }} />
        ) : <div style={{ padding: 40, textAlign: 'center' }}>未配置腾讯地图Key</div>}
      </Modal>
      <MaterialPicker open={pickerOpen} onCancel={() => setPickerOpen(false)}
        onOk={(urls) => { setImageUrl(urls[0] || ''); setPickerOpen(false); }} />
    </div>
  );
};

export default OperationStore;
