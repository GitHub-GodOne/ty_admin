import React, { useState, useEffect, useCallback } from 'react';
import {
  Card, Form, Input, InputNumber, Select, Button, Tabs, Radio, Space,
  Image, Table, Tag, message, DatePicker, Spin, Modal,
} from 'antd';
import { PlusOutlined, DeleteOutlined, ArrowLeftOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import MDEditor from '@uiw/react-md-editor';
import MaterialPicker from '@/components/MaterialPicker';
import { productLstApi, productDetailApi } from '@/api/store';
import { shippingTemplatesList } from '@/api/logistics';
import {
  seckillStoreSaveApi, seckillStoreUpdateApi, seckillStoreInfoApi, seckillListApi,
} from '@/api/marketing';
import { useNavigate, useSearchParams } from 'react-router-dom';
import dayjs from 'dayjs';

const { RangePicker } = DatePicker;

const CreateSeckill: React.FC = () => {
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const editId = searchParams.get('id');
  const queryProductId = searchParams.get('productId');
  const queryTimeId = searchParams.get('timeId');

  const [form] = Form.useForm();
  const [currentTab, setCurrentTab] = useState(editId ? '1' : '0');
  const [pageLoading, setPageLoading] = useState(false);
  const [saving, setSaving] = useState(false);

  // 下拉数据
  const [seckillTimeList, setSeckillTimeList] = useState<any[]>([]);
  const [shippingList, setShippingList] = useState<any[]>([]);

  // 商品选择
  const [productList, setProductList] = useState<any[]>([]);
  const [productLoading, setProductLoading] = useState(false);
  const [selectedProduct, setSelectedProduct] = useState<any>(null);
  const [productKeyword, setProductKeyword] = useState('');
  const [productPickerOpen, setProductPickerOpen] = useState(false);
  const [productId, setProductId] = useState<number>(0);

  // 商品数据
  const [sliderImages, setSliderImages] = useState<string[]>([]);
  const [coverImage, setCoverImage] = useState('');
  const [content, setContent] = useState('');

  // 规格相关
  const [specType, setSpecType] = useState(false);
  const [attrs, setAttrs] = useState<any[]>([]);
  const [attrValues, setAttrValues] = useState<any[]>([]);
  const [selectedRowKeys, setSelectedRowKeys] = useState<React.Key[]>([]);

  // 素材选择器
  const [coverPickerOpen, setCoverPickerOpen] = useState(false);
  const [sliderPickerOpen, setSliderPickerOpen] = useState(false);

  // 加载下拉数据
  useEffect(() => {
    seckillListApi({ page: 1, limit: 100 }).then((res: any) => setSeckillTimeList(res?.list || [])).catch(() => {});
    shippingTemplatesList({ page: 1, limit: 100 }).then((res: any) => setShippingList(res?.list || [])).catch(() => {});
  }, []);

  // 搜索商品
  const fetchProducts = useCallback(async (kw?: string) => {
    setProductLoading(true);
    try {
      const res = await productLstApi({ page: 1, limit: 20, keywords: kw || undefined, type: 1 });
      setProductList(res?.list || []);
    } catch { /* noop */ }
    finally { setProductLoading(false); }
  }, []);

  // 选择商品后加载商品详情（新增模式）
  const getProdect = async (id: number) => {
    setPageLoading(true);
    try {
      const res = await productDetailApi(id);
      if (!res) return;
      const info = res.productInfo || res;
      setSelectedProduct(info);
      setCoverImage(info.image || '');
      setSliderImages(info.sliderImages
        ? (typeof info.sliderImages === 'string' ? JSON.parse(info.sliderImages) : info.sliderImages)
        : (info.sliderImage ? (typeof info.sliderImage === 'string' ? JSON.parse(info.sliderImage) : info.sliderImage) : []));
      setContent(info.content || '');
      setSpecType(!!info.specType);
      form.setFieldsValue({
        title: info.storeName || '',
        unitName: info.unitName || '',
        tempId: info.tempId,
        timeId: queryTimeId ? Number(queryTimeId) : undefined,
        status: 0,
        num: 1,
      });
      // 加载规格属性
      if (info.attr && Array.isArray(info.attr)) setAttrs(info.attr);
      if (info.attrValue) {
        const vals = Array.isArray(info.attrValue) ? info.attrValue : Object.values(info.attrValue);
        // 初始化quota为stock
        const processed = vals.map((v: any, idx: number) => {
          const row: any = { ...v, _key: idx, quota: v.stock || 0 };
          // 多规格时解析attrValue字段
          if (info.specType && v.attrValue) {
            try {
              const parsed = typeof v.attrValue === 'string' ? JSON.parse(v.attrValue) : v.attrValue;
              Object.assign(row, parsed);
            } catch { /* noop */ }
          }
          return row;
        });
        setAttrValues(processed);
        setSelectedRowKeys(processed.map((_: any, i: number) => i));
      }
      setProductPickerOpen(false);
    } catch { message.error('获取商品详情失败'); }
    finally { setPageLoading(false); }
  };

  // 编辑模式加载秒杀商品详情
  const getSeckillProdect = async (id: string) => {
    setPageLoading(true);
    try {
      const res = await seckillStoreInfoApi({ id });
      if (!res) return;
      setSelectedProduct({ id: res.productId, image: res.image });
      setProductId(res.productId);
      setCoverImage(res.image || '');
      const imgs = res.sliderImage || res.images || '';
      setSliderImages(imgs
        ? (typeof imgs === 'string' ? (imgs.startsWith('[') ? JSON.parse(imgs) : imgs.split(',')) : imgs)
        : []);
      setContent(res.content || '');
      setSpecType(!!res.specType);
      form.setFieldsValue({
        title: res.storeName || '',
        unitName: res.unitName || '',
        timeId: res.timeId ? Number(res.timeId) : undefined,
        tempId: res.tempId,
        num: res.num || 1,
        sort: res.sort || 0,
        status: res.status ?? 0,
        dateRange: res.startTimeStr && res.stopTimeStr
          ? [dayjs(res.startTimeStr), dayjs(res.stopTimeStr)] : undefined,
      });
      if (res.attr && Array.isArray(res.attr)) setAttrs(res.attr);
      if (res.attrValue) {
        const vals = Array.isArray(res.attrValue) ? res.attrValue : Object.values(res.attrValue);
        const processed = vals.map((v: any, idx: number) => {
          const row: any = { ...v, _key: idx };
          if (res.specType && v.attrValue) {
            try {
              const parsed = typeof v.attrValue === 'string' ? JSON.parse(v.attrValue) : v.attrValue;
              Object.assign(row, parsed);
            } catch { /* noop */ }
          }
          return row;
        });
        setAttrValues(processed);
        // 编辑时，有id的行自动选中
        const keys = processed.map((_: any, i: number) => i).filter((i: number) => processed[i].id);
        setSelectedRowKeys(keys.length > 0 ? keys : processed.map((_: any, i: number) => i));
      }
    } catch { message.error('获取秒杀商品详情失败'); }
    finally { setPageLoading(false); }
  };

  useEffect(() => {
    if (editId) {
      getSeckillProdect(editId);
    } else if (queryProductId) {
      setProductId(Number(queryProductId));
      getProdect(Number(queryProductId));
    }
  }, [editId]);

  // 提交
  const handleSubmit = async () => {
    try {
      const values = await form.validateFields();
      if (!coverImage) { message.warning('请选择商品'); setCurrentTab(editId ? '1' : '0'); return; }
      if (sliderImages.length === 0) { message.warning('请上传商品轮播图'); setCurrentTab('1'); return; }
      if (!values.dateRange || values.dateRange.length !== 2) {
        message.warning('请选择活动日期'); setCurrentTab('1'); return;
      }

      // 获取选中的行
      const selectedValues = specType
        ? attrValues.filter((_, idx) => selectedRowKeys.includes(idx))
        : attrValues;
      if (selectedValues.length === 0) {
        message.warning('请选择至少一个商品属性'); setCurrentTab('1'); return;
      }

      setSaving(true);
      const submitData: any = {
        productId: productId || selectedProduct?.id,
        image: coverImage,
        images: JSON.stringify(sliderImages),
        title: values.title,
        unitName: values.unitName,
        startTime: values.dateRange[0].format('YYYY-MM-DD'),
        stopTime: values.dateRange[1].format('YYYY-MM-DD'),
        status: values.status ?? 0,
        num: values.num || 1,
        timeId: values.timeId,
        tempId: values.tempId,
        sort: values.sort || 0,
        specType: specType,
        attr: attrs,
        attrValue: selectedValues.map((row: any) => ({
          ...row,
          attrValue: JSON.stringify(row.attrValue || {}),
        })),
        content,
      };

      if (editId) {
        submitData.id = Number(editId);
        await seckillStoreUpdateApi({ id: editId }, submitData);
        message.success('编辑成功');
      } else {
        await seckillStoreSaveApi(submitData);
        message.success('新增成功');
      }
      navigate('/marketing/seckill/list');
    } catch (e: any) {
      if (e?.errorFields) message.warning('请填写完整商品信息');
    } finally { setSaving(false); }
  };

  // Tab切换
  const handleTabClick = (key: string) => {
    // 从选择商品tab切到基础信息tab时，如果还没选商品则加载
    if (!editId && key === '1' && productId && !selectedProduct) {
      getProdect(productId);
    }
    setCurrentTab(key);
  };

  const handleNextFromTab0 = () => {
    if (!coverImage) { message.warning('请选择商品'); return; }
    setCurrentTab('1');
  };

  const handleNextFromTab1 = () => {
    form.validateFields().then(() => {
      if (specType && selectedRowKeys.length === 0) {
        message.warning('请选择至少一个商品属性'); return;
      }
      setCurrentTab('2');
    }).catch(() => {});
  };

  const handlePrev = () => {
    const prev = Number(currentTab) - 1;
    setCurrentTab(String(prev < 0 ? 0 : prev));
  };

  // 构建SKU表格列 - 与Vue一致：只有秒杀价和限量可编辑，其余只读
  const buildSkuColumns = (): ColumnsType<any> => {
    const specCols: ColumnsType<any> = specType
      ? attrs.map((a: any) => ({
          title: a.attrName,
          dataIndex: a.attrName,
          width: 100,
          render: (v: any) => <span>{v ?? '-'}</span>,
        }))
      : [];

    return [
      ...specCols,
      {
        title: '图片', dataIndex: 'image', width: 80,
        render: (v: string) => v ? <Image src={v} width={40} height={40} style={{ borderRadius: 4 }} /> : '-',
      },
      {
        title: '秒杀价', dataIndex: 'price', width: 145,
        render: (v: any, _: any, idx: number) => (
          <InputNumber size="small" min={0} precision={2} step={0.1} style={{ width: '100%' }}
            value={v} onChange={(val) => {
              setAttrValues((prev) => prev.map((row, i) => i === idx ? { ...row, price: val } : row));
            }} />
        ),
      },
      { title: '成本价', dataIndex: 'cost', width: 100, render: (v: any) => <span>{v ?? 0}</span> },
      { title: '原价', dataIndex: 'otPrice', width: 100, render: (v: any) => <span>{v ?? 0}</span> },
      { title: '库存', dataIndex: 'stock', width: 80, render: (v: any) => <span>{v ?? 0}</span> },
      {
        title: '限量', dataIndex: 'quota', width: 145,
        render: (v: any, record: any, idx: number) => (
          <InputNumber size="small" min={1} max={record.stock || 99999} step={1}
            precision={0} style={{ width: '100%' }}
            value={v} onChange={(val) => {
              setAttrValues((prev) => prev.map((row, i) => i === idx ? { ...row, quota: val } : row));
            }} />
        ),
      },
      { title: '商品编号', dataIndex: 'barCode', width: 100, render: (v: any) => <span>{v ?? '-'}</span> },
      { title: '重量(KG)', dataIndex: 'weight', width: 100, render: (v: any) => <span>{v ?? 0}</span> },
      { title: '体积(m³)', dataIndex: 'volume', width: 100, render: (v: any) => <span>{v ?? 0}</span> },
    ];
  };

  // 多规格行选择
  const rowSelection = specType ? {
    selectedRowKeys,
    onChange: (keys: React.Key[]) => setSelectedRowKeys(keys),
  } : undefined;

  // Tab配置
  const tabItems = editId
    ? [{ key: '1', label: '基础信息' }, { key: '2', label: '商品详情' }]
    : [{ key: '0', label: '选择商品' }, { key: '1', label: '基础信息' }, { key: '2', label: '商品详情' }];

  return (
    <Spin spinning={pageLoading}>
      <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
        <Card>
          <div style={{ display: 'flex', alignItems: 'center', gap: 12 }}>
            <Button icon={<ArrowLeftOutlined />} onClick={() => navigate('/marketing/seckill/list')} />
            <span style={{ fontSize: 16, fontWeight: 500 }}>
              {editId ? '编辑商品' : '添加商品'}
            </span>
          </div>
        </Card>

        <Card>
          <Tabs activeKey={currentTab} onChange={handleTabClick} items={tabItems} />

          <Form form={form} labelCol={{ span: 4 }} wrapperCol={{ span: 18 }} style={{ marginTop: 20 }}>

            {/* ===== Tab0 选择商品（仅新增时显示） ===== */}
            {!editId && (
              <div style={{ display: currentTab === '0' ? 'block' : 'none' }}>
                <Form.Item label="选择商品：">
                  <div
                    onClick={() => { fetchProducts(); setProductPickerOpen(true); }}
                    style={{
                      width: 60, height: 60, border: '1px dashed #d9d9d9', borderRadius: 4,
                      display: 'flex', alignItems: 'center', justifyContent: 'center', cursor: 'pointer',
                      overflow: 'hidden', background: '#fafafa',
                    }}
                  >
                    {coverImage
                      ? <img src={coverImage} alt="" style={{ width: '100%', height: '100%', objectFit: 'cover' }} />
                      : <PlusOutlined style={{ fontSize: 20, color: '#999' }} />
                    }
                  </div>
                </Form.Item>
              </div>
            )}
            {/* ===== Tab1 基础信息（含SKU表格） ===== */}
            <div style={{ display: currentTab === '1' ? 'block' : 'none' }}>
              <Form.Item label="商品主图：" required>
                <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
                  <div
                    onClick={() => setCoverPickerOpen(true)}
                    style={{
                      width: 60, height: 60, border: '1px dashed #d9d9d9', borderRadius: 4,
                      display: 'flex', alignItems: 'center', justifyContent: 'center', cursor: 'pointer',
                      overflow: 'hidden', background: '#fafafa',
                    }}
                  >
                    {coverImage
                      ? <img src={coverImage} alt="" style={{ width: '100%', height: '100%', objectFit: 'cover' }} />
                      : <PlusOutlined style={{ fontSize: 20, color: '#999' }} />
                    }
                  </div>
                </div>
                <MaterialPicker open={coverPickerOpen} onCancel={() => setCoverPickerOpen(false)}
                  onOk={(urls) => { setCoverImage(urls[0] || ''); setCoverPickerOpen(false); }} />
              </Form.Item>
              <Form.Item label="商品轮播图：" required>
                <div style={{ display: 'flex', gap: 8, flexWrap: 'wrap', alignItems: 'center' }}>
                  {sliderImages.map((img, idx) => (
                    <div key={idx} style={{ position: 'relative', width: 60, height: 60, border: '1px dashed #d9d9d9', borderRadius: 4, overflow: 'hidden' }}>
                      <img src={img} alt="" style={{ width: '100%', height: '100%', objectFit: 'cover' }} />
                      <DeleteOutlined style={{ position: 'absolute', top: -2, right: -2, color: '#ff4d4f', cursor: 'pointer', fontSize: 16 }}
                        onClick={() => setSliderImages((prev) => prev.filter((_, i) => i !== idx))} />
                    </div>
                  ))}
                  {sliderImages.length < 10 && (
                    <div
                      onClick={() => setSliderPickerOpen(true)}
                      style={{
                        width: 60, height: 60, border: '1px dashed #d9d9d9', borderRadius: 4,
                        display: 'flex', alignItems: 'center', justifyContent: 'center', cursor: 'pointer',
                        background: '#fafafa',
                      }}
                    >
                      <PlusOutlined style={{ fontSize: 20, color: '#999' }} />
                    </div>
                  )}
                </div>
                <MaterialPicker open={sliderPickerOpen} onCancel={() => setSliderPickerOpen(false)} multiple
                  limit={10 - sliderImages.length}
                  onOk={(urls) => { setSliderImages((prev) => [...prev, ...urls].slice(0, 10)); setSliderPickerOpen(false); }} />
              </Form.Item>
              <Form.Item label="商品标题：" name="title" rules={[{ required: true, message: '请输入商品标题' }]}>
                <Input placeholder="请输入商品名称" maxLength={249} style={{ maxWidth: 460 }} />
              </Form.Item>
              <Form.Item label="单位：" name="unitName" rules={[{ required: true, message: '请输入单位' }]}>
                <Input placeholder="请输入单位" style={{ maxWidth: 460 }} />
              </Form.Item>
              <Form.Item label="运费模板：" name="tempId" rules={[{ required: true, message: '请选择运费模板' }]}>
                <Select placeholder="请选择" style={{ maxWidth: 460 }}>
                  {shippingList.map((item: any) => (
                    <Select.Option key={item.id} value={item.id}>{item.name}</Select.Option>
                  ))}
                </Select>
              </Form.Item>
              <Form.Item label="当天参与活动次数：" name="num" rules={[{ required: true, message: '请输入活动次数' }]} initialValue={1}>
                <InputNumber min={1} step={1} precision={0} placeholder="请输入活动次数" style={{ width: 460 }} />
              </Form.Item>
              <Form.Item label="活动日期：" name="dateRange" rules={[{ required: true, message: '请选择活动日期' }]}>
                <RangePicker style={{ width: 460 }}
                  disabledDate={(current) => current && current < dayjs().subtract(1, 'day').endOf('day')} />
              </Form.Item>
              <Form.Item label="活动时间：" name="timeId" rules={[{ required: true, message: '请选择活动时间' }]}>
                <Select placeholder="请选择" style={{ maxWidth: 460 }}>
                  {seckillTimeList.map((item: any) => (
                    <Select.Option key={item.id} value={item.id}>
                      {item.name} | {item.time ? item.time.split(',').join(' - ') : ''}
                    </Select.Option>
                  ))}
                </Select>
              </Form.Item>
              <Form.Item label="活动状态：" required>
                <Form.Item name="status" noStyle initialValue={0}>
                  <Radio.Group>
                    <Radio value={0}>关闭</Radio>
                    <Radio value={1}>开启</Radio>
                  </Radio.Group>
                </Form.Item>
              </Form.Item>

              {/* 商品属性表格 */}
              <Form.Item label="商品属性：" required>
                <div style={{ overflowX: 'auto' }}>
                  <Table
                    dataSource={attrValues}
                    columns={buildSkuColumns()}
                    rowKey={(_, idx) => idx as number}
                    rowSelection={rowSelection}
                    size="small"
                    pagination={false}
                    bordered
                    scroll={{ x: 1200 }}
                  />
                </div>
              </Form.Item>
            </div>
            {/* ===== Tab2 商品详情 ===== */}
            <div style={{ display: currentTab === '2' ? 'block' : 'none' }}>
              <Form.Item label="商品详情：">
                <div data-color-mode="light">
                  <MDEditor value={content} onChange={(val) => setContent(val || '')} height={500} preview="live" />
                </div>
              </Form.Item>
            </div>
          </Form>

          {/* 底部按钮 */}
          <div style={{ marginTop: 30, paddingLeft: 130 }}>
            <Space>
              {((!editId && Number(currentTab) > 0) || (editId && currentTab === '2')) && (
                <Button size="small" onClick={handlePrev}>上一步</Button>
              )}
              {currentTab === '0' && !editId && (
                <Button size="small" type="primary" onClick={handleNextFromTab0}>下一步</Button>
              )}
              {currentTab === '1' && (
                <Button size="small" onClick={handleNextFromTab1}>下一步</Button>
              )}
              <Button size="small" type="primary" loading={saving} onClick={handleSubmit}>提交</Button>
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
              {
                title: '图片', dataIndex: 'image', width: 60,
                render: (v: string) => v ? <Image src={v} width={36} height={36} style={{ borderRadius: 4 }} /> : '-',
              },
              { title: '商品名称', dataIndex: 'storeName', ellipsis: true },
              { title: '价格', dataIndex: 'price', width: 80 },
              {
                title: '操作', width: 80,
                render: (_: any, record: any) => (
                  <a onClick={() => { setProductId(record.id); getProdect(record.id); }}>选择</a>
                ),
              },
            ]} />
        </Modal>
      </div>
    </Spin>
  );
};

export default CreateSeckill;
