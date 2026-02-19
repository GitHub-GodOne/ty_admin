import React, { useState, useEffect, useCallback } from 'react';
import {
  Card, Form, Input, Switch, Button, DatePicker, Radio, Table, Space, Image,
  Cascader, message, Tabs, Spin, Modal, Popconfirm,
} from 'antd';
import { PlusOutlined, DeleteOutlined, ArrowLeftOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { createAtuosphere, atuosphereUpdateApi } from '@/api/marketing';
import { productLstApi } from '@/api/store';
import { categoryApi } from '@/api/store';
import MaterialPicker from '@/components/MaterialPicker';
import { useNavigate, useSearchParams, useLocation } from 'react-router-dom';
import dayjs from 'dayjs';

const { RangePicker } = DatePicker;

const AtmosphereAdd: React.FC = () => {
  const navigate = useNavigate();
  const location = useLocation();
  const [searchParams] = useSearchParams();
  const editId = searchParams.get('id');
  const isBorder = location.pathname.includes('border');
  const styleType = isBorder ? 0 : 1;
  const typeName = isBorder ? '活动边框' : '氛围图';
  const listPath = isBorder ? '/marketing/border/list' : '/marketing/atmosphere/list';
  const sizeHint = isBorder ? '750*750px' : '750*100px';

  const [form] = Form.useForm();
  const [currentTab, setCurrentTab] = useState('1');
  const [pageLoading, setPageLoading] = useState(false);
  const [saving, setSaving] = useState(false);
  const [method, setMethod] = useState(0);
  const [styleImg, setStyleImg] = useState('');
  const [pickerOpen, setPickerOpen] = useState(false);

  // 指定商品
  const [selectedProducts, setSelectedProducts] = useState<any[]>([]);
  const [multipleSelection, setMultipleSelection] = useState<React.Key[]>([]);
  const [productPickerOpen, setProductPickerOpen] = useState(false);
  const [productList, setProductList] = useState<any[]>([]);
  const [productLoading, setProductLoading] = useState(false);
  const [productKeyword, setProductKeyword] = useState('');

  // 指定分类
  const [categoryList, setCategoryList] = useState<any[]>([]);
  const [proCategorylist, setProCategorylist] = useState<number[]>([]);

  // 加载分类
  useEffect(() => {
    categoryApi({ status: -1, type: 1 }).then((res: any) => {
      const transform = (list: any[]): any[] => list.map((item: any) => ({
        value: item.id, label: item.name,
        children: item.child?.length ? transform(item.child) : undefined,
      }));
      setCategoryList(transform(res || []));
    }).catch(() => {});
  }, []);

  // 编辑模式从localStorage加载
  useEffect(() => {
    if (!editId) return;
    const raw = localStorage.getItem('activitystyle');
    if (!raw) return;
    try {
      const info = JSON.parse(raw);
      form.setFieldsValue({
        name: info.name,
        timeVal: info.starttime && info.endtime ? [dayjs(info.starttime), dayjs(info.endtime)] : undefined,
        status: !!info.status,
      });
      setStyleImg(info.style || '');
      setMethod(info.method ?? 0);
      if (info.method === 3 && info.products) {
        setProCategorylist(info.products.split(',').map((s: string) => Number(s)));
      }
      if (info.method === 1 && info.products) {
        // 加载指定商品
        import('@/api/store').then(({ productListbyidsApi }) => {
          productListbyidsApi(info.products).then((res: any) => {
            setSelectedProducts(res?.list || []);
          }).catch(() => {});
        });
      }
    } catch {}
  }, [editId]);

  const fetchProducts = useCallback(async (kw?: string) => {
    setProductLoading(true);
    try {
      const res = await productLstApi({ page: 1, limit: 20, keywords: kw || undefined, type: 1 });
      setProductList(res?.list || []);
    } catch {} finally { setProductLoading(false); }
  }, []);

  const handleAddGoods = () => { fetchProducts(); setProductPickerOpen(true); };

  const handleSelectProducts = (rows: any[]) => {
    const existing = selectedProducts.map((p: any) => p.id);
    const newOnes = rows.filter((r: any) => !existing.includes(r.id));
    setSelectedProducts((prev) => [...prev, ...newOnes]);
    setProductPickerOpen(false);
  };

  const handleDeleteProduct = (index: number) => {
    setSelectedProducts((prev) => prev.filter((_, i) => i !== index));
  };

  const handleBatchDel = () => {
    setSelectedProducts((prev) => prev.filter((_, i) => !multipleSelection.includes(i)));
    setMultipleSelection([]);
  };

  const handleNextStep = () => {
    form.validateFields().then(() => {
      if (!styleImg) { message.warning(`请上传${typeName}`); return; }
      setCurrentTab('2');
    }).catch(() => {});
  };

  const handleSubmit = async () => {
    try {
      const values = await form.validateFields();
      if (!styleImg) { message.warning(`请上传${typeName}`); setCurrentTab('1'); return; }

      let products = '';
      if (method === 1) {
        if (selectedProducts.length === 0) { message.error('请选择至少一个商品'); return; }
        products = selectedProducts.map((p: any) => p.id).join(',');
      } else if (method === 3) {
        if (proCategorylist.length === 0) { message.error('请选择至少一个分类'); return; }
        products = proCategorylist.join(',');
      }

      setSaving(true);
      const timeRange = values.timeVal;
      const data: any = {
        name: values.name,
        starttime: timeRange ? timeRange[0].format('YYYY-MM-DD HH:mm:ss') : '',
        endtime: timeRange ? timeRange[1].format('YYYY-MM-DD HH:mm:ss') : '',
        style: styleImg,
        status: !!values.status,
        method,
        products,
        styleType: styleType === 1,
      };
      if (editId) {
        data.id = Number(editId);
        await atuosphereUpdateApi(data);
        message.success('编辑成功');
      } else {
        await createAtuosphere(data);
        message.success('新增成功');
      }
      navigate(listPath);
    } catch (e: any) { if (e?.errorFields) message.warning('请填写完整信息'); }
    finally { setSaving(false); }
  };

  const productColumns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 55 },
    { title: '商品图', dataIndex: 'image', width: 80,
      render: (v: string) => v ? <Image src={v} width={36} height={36} style={{ borderRadius: 4 }} /> : '-' },
    { title: '商品名称', dataIndex: 'storeName', ellipsis: true, width: 200 },
    { title: '售价', dataIndex: 'price', width: 90 },
    { title: '库存', dataIndex: 'stock', width: 70 },
    { title: '操作', width: 80,
      render: (_: any, __: any, idx: number) => <a style={{ color: '#ff4d4f' }} onClick={() => handleDeleteProduct(idx)}>删除</a> },
  ];

  return (
    <Spin spinning={pageLoading}>
      <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
        <Card>
          <div style={{ display: 'flex', alignItems: 'center', gap: 12 }}>
            <Button icon={<ArrowLeftOutlined />} onClick={() => navigate(listPath)} />
            <span style={{ fontSize: 16, fontWeight: 500 }}>{editId ? '编辑' : '添加'}{typeName}</span>
          </div>
        </Card>
        <Card>
          <Tabs activeKey={currentTab} onChange={setCurrentTab}
            items={[{ key: '1', label: '基础设置' }, { key: '2', label: '使用范围' }]} />
          <Form form={form} labelCol={{ span: 3 }} wrapperCol={{ span: 18 }} style={{ marginTop: 20 }}>
            {/* Tab1 基础设置 */}
            <div style={{ display: currentTab === '1' ? 'block' : 'none' }}>
              <Form.Item label="活动名称：" name="name" rules={[{ required: true, message: '请输入活动名称' }]}>
                <Input placeholder="请输入活动名称" style={{ width: 460 }} />
              </Form.Item>
              <Form.Item label="活动时间：" name="timeVal" rules={[{ required: true, message: '请选择活动时间' }]}>
                <RangePicker showTime style={{ width: 460 }} format="YYYY-MM-DD HH:mm:ss"
                  disabledDate={(current) => current && current < dayjs().subtract(1, 'day').endOf('day')} />
              </Form.Item>
              <div style={{ color: '#999', fontSize: 12, marginLeft: 100, marginTop: -16, marginBottom: 16 }}>
                设置活动{typeName}在商城展示时间
              </div>
              <Form.Item label={`${typeName}：`} required>
                <div onClick={() => setPickerOpen(true)}
                  style={{ width: 60, height: 60, border: '1px dashed #d9d9d9', borderRadius: 4,
                    display: 'flex', alignItems: 'center', justifyContent: 'center', cursor: 'pointer',
                    overflow: 'hidden', background: '#fafafa', marginBottom: 4 }}>
                  {styleImg ? <img src={styleImg} alt="" style={{ width: '100%', height: '100%', objectFit: 'cover' }} />
                    : <PlusOutlined style={{ fontSize: 20, color: '#999' }} />}
                </div>
                <div style={{ color: '#999', fontSize: 12 }}>{sizeHint}</div>
                <MaterialPicker open={pickerOpen} onCancel={() => setPickerOpen(false)}
                  onOk={(urls) => { setStyleImg(urls[0] || ''); setPickerOpen(false); }} />
              </Form.Item>
              <Form.Item label="是否开启：" name="status" valuePropName="checked" initialValue={false}>
                <Switch checkedChildren="开启" unCheckedChildren="关闭" />
              </Form.Item>
            </div>
            {/* Tab2 使用范围 */}
            <div style={{ display: currentTab === '2' ? 'block' : 'none' }}>
              <Form.Item label=" " colon={false}>
                <Radio.Group value={method} onChange={(e) => setMethod(e.target.value)}>
                  <Radio value={0}>全部商品参与</Radio>
                  <Radio value={1}>指定商品参与</Radio>
                  <Radio value={3}>指定分类参与</Radio>
                </Radio.Group>
              </Form.Item>
              {method === 1 && (
                <>
                  <Form.Item label=" " colon={false}>
                    <Space>
                      <Button size="small" type="primary" onClick={handleAddGoods}>添加商品</Button>
                      <Button size="small" disabled={multipleSelection.length === 0} onClick={handleBatchDel}>批量删除</Button>
                    </Space>
                  </Form.Item>
                  <Form.Item label=" " colon={false}>
                    <Table dataSource={selectedProducts} columns={productColumns} rowKey="id" size="small"
                      pagination={false}
                      rowSelection={{ selectedRowKeys: multipleSelection,
                        onChange: (keys) => setMultipleSelection(keys) }} />
                  </Form.Item>
                </>
              )}
              {method === 3 && (
                <Form.Item label="选择分类：">
                  <Cascader options={categoryList} value={proCategorylist}
                    onChange={(val: any) => setProCategorylist(val || [])}
                    placeholder="请选择分类" style={{ width: 460 }}
                    multiple showCheckedStrategy="SHOW_CHILD"
                    fieldNames={{ label: 'label', value: 'value', children: 'children' }} />
                </Form.Item>
              )}
            </div>
          </Form>
          {/* 底部按钮 */}
          <div style={{ marginTop: 24, paddingLeft: 90 }}>
            <Space>
              {currentTab === '1' && (
                <Button size="small" type="primary" onClick={handleNextStep}>下一步</Button>
              )}
              {currentTab === '2' && (
                <>
                  <Button size="small" onClick={() => setCurrentTab('1')}>上一步</Button>
                  <Button size="small" type="primary" loading={saving} onClick={handleSubmit}>保存</Button>
                </>
              )}
            </Space>
          </div>
        </Card>
        {/* 商品选择弹窗 */}
        <Modal title="选择商品" open={productPickerOpen} onCancel={() => setProductPickerOpen(false)}
          footer={null} width={720} destroyOnClose>
          <div style={{ marginBottom: 12 }}>
            <Space>
              <Input placeholder="搜索商品名称/ID" value={productKeyword}
                onChange={(e) => setProductKeyword(e.target.value)}
                onPressEnter={() => fetchProducts(productKeyword)}
                allowClear style={{ width: 300 }} />
              <Button type="primary" onClick={() => fetchProducts(productKeyword)}>搜索</Button>
            </Space>
          </div>
          <Table dataSource={productList} loading={productLoading} rowKey="id" size="small"
            pagination={false} scroll={{ y: 400 }}
            columns={[
              { title: 'ID', dataIndex: 'id', width: 60 },
              { title: '图片', dataIndex: 'image', width: 60,
                render: (v: string) => v ? <Image src={v} width={36} height={36} style={{ borderRadius: 4 }} /> : '-' },
              { title: '商品名称', dataIndex: 'storeName', ellipsis: true },
              { title: '价格', dataIndex: 'price', width: 80 },
              { title: '操作', width: 80,
                render: (_: any, record: any) => (
                  <a onClick={() => handleSelectProducts([record])}>选择</a>
                ) },
            ]} />
        </Modal>
      </div>
    </Spin>
  );
};

export default AtmosphereAdd;
