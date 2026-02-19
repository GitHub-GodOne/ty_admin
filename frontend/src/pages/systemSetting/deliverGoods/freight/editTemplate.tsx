import React, { useState, useEffect, useCallback } from 'react';
import {
  Drawer, Form, Input, InputNumber, Radio, Button, Table, Space,
  Cascader, message, Spin, Popconfirm,
} from 'antd';
import { PlusOutlined, DeleteOutlined } from '@ant-design/icons';
import { cityListTree, templateDetailApi, shippingSave, shippingUpdate } from '@/api/logistics';

interface RegionRow {
  first: number;
  firstPrice: number;
  renewal: number;
  renewalPrice: number;
  city_ids: any[];
  cityId: string;
  title: string;
}

interface FreeRow {
  number: number;
  price: number;
  city_ids: any[];
  cityId: string;
  title: string;
}

interface Props {
  visible: boolean;
  editId?: number | null;
  onClose: () => void;
  onSuccess: () => void;
}

const columnLabels: Record<number, { first: string;续: string; free: string }> = {
  1: { first: '首件', '续': '续件', free: '包邮件数' },
  2: { first: '首件重量（kg）', '续': '续件重量（kg）', free: '包邮重量（kg）' },
  3: { first: '首件体积（m³）', '续': '续件体积（m³）', free: '包邮体积（m³）' },
};

const defaultRegion = (): RegionRow => ({
  first: 1, firstPrice: 0, renewal: 0, renewalPrice: 0,
  city_ids: [], cityId: '0', title: '',
});

const defaultFree = (): FreeRow => ({
  number: 1, price: 1, city_ids: [], cityId: '', title: '',
});

const EditTemplate: React.FC<Props> = ({ visible, editId, onClose, onSuccess }) => {
  const [form] = Form.useForm();
  const [loading, setLoading] = useState(false);
  const [submitLoading, setSubmitLoading] = useState(false);
  const [cityOptions, setCityOptions] = useState<any[]>([]);
  const [appoint, setAppoint] = useState(0);
  const [billingType, setBillingType] = useState(1);
  const [regionList, setRegionList] = useState<RegionRow[]>([defaultRegion()]);
  const [freeList, setFreeList] = useState<FreeRow[]>([]);

  // Load city tree
  useEffect(() => {
    if (visible) {
      cityListTree().then((res: any) => {
        setCityOptions(Array.isArray(res) ? res : []);
      }).catch(() => {});
    }
  }, [visible]);

  // Load template detail for editing
  useEffect(() => {
    if (visible && editId) {
      setLoading(true);
      templateDetailApi({ id: editId }).then((res: any) => {
        const info = res;
        const ap = info.appoint ?? 0;
        const tp = info.type || 1;
        setAppoint(ap);
        setBillingType(ap === 0 ? 1 : tp);
        form.setFieldsValue({ name: info.name, appoint: ap, type: tp, sort: info.sort ?? 0 });

        if (info.regionList?.length) {
          setRegionList(info.regionList.map((r: any) => ({
            ...r,
            city_ids: r.title ? (typeof r.title === 'string' ? JSON.parse(r.title) : r.title) : [],
          })));
        } else {
          setRegionList([defaultRegion()]);
        }
        if (info.freeList?.length) {
          setFreeList(info.freeList.map((f: any) => ({
            ...f,
            city_ids: f.title ? (typeof f.title === 'string' ? JSON.parse(f.title) : f.title) : [],
          })));
        } else {
          setFreeList([]);
        }
      }).catch(() => message.error('获取模板详情失败'))
        .finally(() => setLoading(false));
    } else if (visible) {
      // Reset for new
      form.setFieldsValue({ name: '', appoint: 0, type: 1, sort: 0 });
      setAppoint(0);
      setBillingType(1);
      setRegionList([defaultRegion()]);
      setFreeList([]);
    }
  }, [visible, editId, form]);

  const handleSubmit = async () => {
    try {
      const values = await form.validateFields();
      setSubmitLoading(true);

      const param: any = {
        name: values.name,
        appoint: values.appoint,
        type: values.appoint === 0 ? 0 : values.type,
        sort: values.sort || 0,
      };

      if (values.appoint > 0) {
        // Process region list
        param.shippingTemplatesRegionRequestList = regionList.map((r) => {
          const title = r.city_ids.length > 0 ? JSON.stringify(r.city_ids) : '[[0,0]]';
          const flatIds = r.city_ids.map((ids: any[]) => ids[ids.length - 1]);
          return {
            first: r.first, firstPrice: r.firstPrice,
            renewal: r.renewal, renewalPrice: r.renewalPrice,
            title, cityId: flatIds.length > 0 ? flatIds.join(',') : '0',
          };
        });
        // For appoint=2, first row is default
        if (values.appoint === 2 && param.shippingTemplatesRegionRequestList.length > 0) {
          param.shippingTemplatesRegionRequestList[0].title = '[[0,0]]';
          param.shippingTemplatesRegionRequestList[0].cityId = '0';
        }
        // Process free list
        if (values.appoint === 2) {
          param.shippingTemplatesFreeRequestList = freeList.map((f) => {
            const title = f.city_ids.length > 0 ? JSON.stringify(f.city_ids) : '[[0,0]]';
            const flatIds = f.city_ids.map((ids: any[]) => ids[ids.length - 1]);
            return {
              number: f.number, price: f.price,
              title, cityId: flatIds.length > 0 ? flatIds.join(',') : '0',
            };
          });
        }
      }

      if (editId) {
        await shippingUpdate(param, { id: editId });
      } else {
        await shippingSave(param);
      }
      message.success('操作成功');
      onSuccess();
      onClose();
    } catch (e: any) {
      if (e?.errorFields) return; // form validation
    } finally {
      setSubmitLoading(false);
    }
  };

  const labels = columnLabels[billingType] || columnLabels[1];
  const cascaderProps = { children: 'child', label: 'name', value: 'cityId', multiple: true };

  const regionColumns = [
    {
      title: '送达到', dataIndex: 'city_ids', width: 260,
      render: (_: any, record: RegionRow, index: number) => {
        if (appoint === 2 && index === 0) return <span>默认运费</span>;
        return (
          <Cascader
            options={cityOptions}
            fieldNames={cascaderProps}
            value={record.city_ids}
            onChange={(val) => {
              const next = [...regionList];
              next[index] = { ...next[index], city_ids: val };
              setRegionList(next);
            }}
            multiple maxTagCount="responsive" style={{ width: '100%' }}
          />
        );
      },
    },
    {
      title: labels.first, dataIndex: 'first', width: 120,
      render: (_: any, __: any, index: number) => (
        <InputNumber min={billingType === 1 ? 1 : 0.1} value={regionList[index].first}
          onChange={(v) => { const n = [...regionList]; n[index] = { ...n[index], first: v ?? 0 }; setRegionList(n); }}
          style={{ width: '100%' }} />
      ),
    },
    {
      title: '运费（元）', dataIndex: 'firstPrice', width: 120,
      render: (_: any, __: any, index: number) => (
        <InputNumber min={0} value={regionList[index].firstPrice}
          onChange={(v) => { const n = [...regionList]; n[index] = { ...n[index], firstPrice: v ?? 0 }; setRegionList(n); }}
          style={{ width: '100%' }} />
      ),
    },
    {
      title: labels['续'], dataIndex: 'renewal', width: 120,
      render: (_: any, __: any, index: number) => (
        <InputNumber min={billingType === 1 ? 1 : 0.1} value={regionList[index].renewal}
          onChange={(v) => { const n = [...regionList]; n[index] = { ...n[index], renewal: v ?? 0 }; setRegionList(n); }}
          style={{ width: '100%' }} />
      ),
    },
    {
      title: '续费（元）', dataIndex: 'renewalPrice', width: 120,
      render: (_: any, __: any, index: number) => (
        <InputNumber min={0} value={regionList[index].renewalPrice}
          onChange={(v) => { const n = [...regionList]; n[index] = { ...n[index], renewalPrice: v ?? 0 }; setRegionList(n); }}
          style={{ width: '100%' }} />
      ),
    },
    {
      title: '操作', width: 80,
      render: (_: any, __: any, index: number) => {
        if (appoint === 2 && index === 0) return null;
        return (
          <Popconfirm title="确定删除?" onConfirm={() => setRegionList((l) => l.filter((_, i) => i !== index))}>
            <a style={{ color: '#ff4d4f' }}><DeleteOutlined /></a>
          </Popconfirm>
        );
      },
    },
  ];

  const freeColumns = [
    {
      title: '选择区域', dataIndex: 'city_ids', width: 260,
      render: (_: any, record: FreeRow, index: number) => (
        <Cascader
          options={cityOptions} fieldNames={cascaderProps}
          value={record.city_ids}
          onChange={(val) => { const n = [...freeList]; n[index] = { ...n[index], city_ids: val }; setFreeList(n); }}
          multiple maxTagCount="responsive" style={{ width: '100%' }}
        />
      ),
    },
    {
      title: labels.free, dataIndex: 'number', width: 160,
      render: (_: any, __: any, index: number) => (
        <InputNumber min={billingType === 1 ? 1 : 0.1} value={freeList[index].number}
          onChange={(v) => { const n = [...freeList]; n[index] = { ...n[index], number: v ?? 0 }; setFreeList(n); }}
          style={{ width: '100%' }} />
      ),
    },
    {
      title: '包邮金额（元）', dataIndex: 'price', width: 140,
      render: (_: any, __: any, index: number) => (
        <InputNumber min={0} value={freeList[index].price}
          onChange={(v) => { const n = [...freeList]; n[index] = { ...n[index], price: v ?? 0 }; setFreeList(n); }}
          style={{ width: '100%' }} />
      ),
    },
    {
      title: '操作', width: 80,
      render: (_: any, __: any, index: number) => (
        <Popconfirm title="确定删除?" onConfirm={() => setFreeList((l) => l.filter((_, i) => i !== index))}>
          <a style={{ color: '#ff4d4f' }}><DeleteOutlined /></a>
        </Popconfirm>
      ),
    },
  ];

  return (
    <Drawer
      title="运费模板"
      open={visible}
      onClose={onClose}
      width={1000}
      destroyOnClose
      footer={
        <Space style={{ float: 'right' }}>
          <Button onClick={onClose}>取消</Button>
          <Button type="primary" loading={submitLoading} onClick={handleSubmit}>确定</Button>
        </Space>
      }
    >
      <Spin spinning={loading}>
        <Form form={form} labelCol={{ span: 4 }} wrapperCol={{ span: 18 }}>
          <Form.Item label="模板名称" name="name" rules={[{ required: true, message: '请输入模板名称' }]}>
            <Input placeholder="请输入模板名称" style={{ width: 460 }} />
          </Form.Item>
          <Form.Item label="包邮方式" name="appoint" rules={[{ required: true }]}>
            <Radio.Group onChange={(e) => { setAppoint(e.target.value); if (e.target.value === 2) { setRegionList((l) => l.length ? l : [defaultRegion()]); } }}>
              <Radio value={0}>全国包邮</Radio>
              <Radio value={1}>部分包邮</Radio>
              <Radio value={2}>自定义</Radio>
            </Radio.Group>
          </Form.Item>

          {appoint > 0 && (
            <>
              <Form.Item label="计费方式" name="type" rules={[{ required: true }]}>
                <Radio.Group onChange={(e) => setBillingType(e.target.value)}>
                  <Radio value={1}>按件数</Radio>
                  <Radio value={2}>按重量</Radio>
                  <Radio value={3}>按体积</Radio>
                </Radio.Group>
              </Form.Item>
              <Form.Item label="运费">
                <Table rowKey={(_, i) => `r${i}`} columns={regionColumns} dataSource={regionList}
                  pagination={false} size="small" bordered scroll={{ x: 800 }} />
                <Button type="primary" icon={<PlusOutlined />} style={{ marginTop: 12 }}
                  onClick={() => setRegionList((l) => [...l, defaultRegion()])}>
                  添加区域
                </Button>
              </Form.Item>

              {appoint === 2 && (
                <Form.Item label="包邮区域">
                  <Table rowKey={(_, i) => `f${i}`} columns={freeColumns} dataSource={freeList}
                    pagination={false} size="small" bordered scroll={{ x: 700 }} />
                  <Button type="primary" icon={<PlusOutlined />} style={{ marginTop: 12 }}
                    onClick={() => setFreeList((l) => [...l, defaultFree()])}>
                    添加指定包邮区域
                  </Button>
                </Form.Item>
              )}
            </>
          )}

          <Form.Item label="排序" name="sort">
            <Input placeholder="请输入排序" style={{ width: 460 }} />
          </Form.Item>
        </Form>
      </Spin>
    </Drawer>
  );
};

export default EditTemplate;
