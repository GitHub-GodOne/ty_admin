import React, { useState, useEffect, useCallback } from 'react';
import {
  Card, Form, Input, InputNumber, Select, Button, Tabs, Radio, Checkbox, Space,
  Image, Table, Tag, message, Cascader, Spin,
} from 'antd';
import {
  PlusOutlined, DeleteOutlined, ArrowLeftOutlined, UploadOutlined, ReloadOutlined,
} from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import MDEditor from '@uiw/react-md-editor';
import MaterialPicker from '@/components/MaterialPicker';
import {
  productCreateApi, productUpdateApi, productDetailApi, categoryApi,
  templateListApi,
} from '@/api/store';
import { shippingTemplatesList } from '@/api/logistics';
import { marketingSendApi } from '@/api/marketing';
import { useNavigate, useSearchParams } from 'react-router-dom';

const { TextArea } = Input;

const defaultAttrValue = {
  image: '', price: 0, cost: 0, otPrice: 0, stock: 0,
  barCode: '', weight: 0, volume: 0, brokerage: 0, brokerageTwo: 0,
};

const recommendOptions = [
  { label: '优品推荐', value: 'isGood' },
  { label: '热卖推荐', value: 'isHot' },
  { label: '促销单品', value: 'isBenefit' },
  { label: '精品推荐', value: 'isBest' },
  { label: '新品首发', value: 'isNew' },
];

const activityList = ['默认', '秒杀', '砍价', '拼团'];

const CreateStore: React.FC = () => {
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const editId = searchParams.get('id');
  const isDisabled = searchParams.get('disabled') === '1';

  const [form] = Form.useForm();
  const [currentTab, setCurrentTab] = useState('0');
  const [pageLoading, setPageLoading] = useState(false);
  const [saving, setSaving] = useState(false);

  // 下拉数据
  const [categoryList, setCategoryList] = useState<any[]>([]);
  const [shippingList, setShippingList] = useState<any[]>([]);
  const [ruleList, setRuleList] = useState<any[]>([]);
  const [couponList, setCouponList] = useState<any[]>([]);

  // 商品数据
  const [sliderImages, setSliderImages] = useState<string[]>([]);
  const [coverImage, setCoverImage] = useState('');
  const [videoLink, setVideoLink] = useState('');
  const [content, setContent] = useState('');

  // 规格相关
  const [specType, setSpecType] = useState(false); // false=单规格 true=多规格
  const [isSub, setIsSub] = useState(false); // 佣金设置
  const [attrs, setAttrs] = useState<{ attrName: string; attrValue: string[] }[]>([]);
  const [singleAttr, setSingleAttr] = useState<any>({ ...defaultAttrValue });
  const [multiAttrValues, setMultiAttrValues] = useState<any[]>([]);

  // 新规格输入
  const [newAttrName, setNewAttrName] = useState('');
  const [newAttrVal, setNewAttrVal] = useState('');
  const [showNewAttr, setShowNewAttr] = useState(false);
  const [tagInputIndex, setTagInputIndex] = useState<number | null>(null);
  const [tagInputVal, setTagInputVal] = useState('');

  // 其他设置
  const [checkboxGroup, setCheckboxGroup] = useState<string[]>([]);
  const [activity, setActivity] = useState<string[]>([...activityList]);

  // 批量设置行
  const [batchRow, setBatchRow] = useState<any>({ ...defaultAttrValue });

  // 素材选择器
  const [coverPickerOpen, setCoverPickerOpen] = useState(false);
  const [sliderPickerOpen, setSliderPickerOpen] = useState(false);
  const [videoPickerOpen, setVideoPickerOpen] = useState(false);

  // 加载下拉数据
  useEffect(() => {
    categoryApi({ type: 1 }).then((res: any) => setCategoryList(Array.isArray(res) ? res : [])).catch(() => {});
    shippingTemplatesList({ page: 1, limit: 100 }).then((res: any) => setShippingList(res?.list || [])).catch(() => {});
    templateListApi({ page: 1, limit: 100 }).then((res: any) => setRuleList(res?.list || [])).catch(() => {});
    marketingSendApi({ page: 1, limit: 100 }).then((res: any) => setCouponList(res?.list || [])).catch(() => {});
  }, []);

  // 编辑模式加载商品详情
  useEffect(() => {
    if (!editId) return;
    setPageLoading(true);
    productDetailApi(editId).then((res: any) => {
      if (!res) return;
      const info = res.productInfo || res;
      form.setFieldsValue({
        storeName: info.storeName, keyword: info.keyword, unitName: info.unitName,
        cateIds: info.cateIds ? (typeof info.cateIds === 'string' ? info.cateIds.split(',').map(Number) : info.cateIds) : [],
        tempId: info.tempId, sort: info.sort || 0,
        giveIntegral: info.giveIntegral || 0, ficti: info.ficti || 0,
      });
      setCoverImage(info.image || '');
      setSliderImages(info.sliderImages ? (typeof info.sliderImages === 'string' ? JSON.parse(info.sliderImages) : info.sliderImages) : []);
      setVideoLink(info.videoLink || '');
      setContent(info.content || '');
      setSpecType(!!info.specType);
      setIsSub(!!info.isSub);

      // 推荐
      const checks: string[] = [];
      if (info.isGood) checks.push('isGood');
      if (info.isHot) checks.push('isHot');
      if (info.isBenefit) checks.push('isBenefit');
      if (info.isBest) checks.push('isBest');
      if (info.isNew) checks.push('isNew');
      setCheckboxGroup(checks);

      // 活动优先级
      if (info.activity && Array.isArray(info.activity)) setActivity(info.activity);

      // 规格
      if (info.attr && Array.isArray(info.attr)) setAttrs(info.attr);
      if (info.attrValue) {
        if (info.specType) {
          setMultiAttrValues(Array.isArray(info.attrValue) ? info.attrValue : Object.values(info.attrValue));
        } else {
          const val = Array.isArray(info.attrValue) ? info.attrValue[0] : Object.values(info.attrValue)[0];
          if (val) setSingleAttr({ ...defaultAttrValue, ...val });
        }
      }
    }).catch(() => message.error('获取商品详情失败'))
      .finally(() => setPageLoading(false));
  }, [editId]);

  // 多规格属性组合生成
  const generateMultiAttr = useCallback((attrList: typeof attrs) => {
    if (attrList.length === 0) { setMultiAttrValues([]); return; }
    const combine = (arr: string[][]): string[][] => {
      if (arr.length === 0) return [[]];
      const [first, ...rest] = arr;
      const restCombined = combine(rest);
      const result: string[][] = [];
      for (const item of first) {
        for (const combo of restCombined) {
          result.push([item, ...combo]);
        }
      }
      return result;
    };
    const valArrays = attrList.map((a) => a.attrValue);
    const combos = combine(valArrays);
    const existing = [...multiAttrValues];
    const newValues = combos.map((combo) => {
      const key = combo.join(',');
      const found = existing.find((e) => {
        const eKey = attrList.map((a) => e[a.attrName]).join(',');
        return eKey === key;
      });
      const row: any = found ? { ...found } : { ...defaultAttrValue };
      attrList.forEach((a, i) => { row[a.attrName] = combo[i]; });
      return row;
    });
    setMultiAttrValues(newValues);
  }, [multiAttrValues]);

  // 添加新规格
  const handleAddAttrName = () => {
    if (!newAttrName || !newAttrVal) { message.warning('请输入规格名和规格值'); return; }
    const newAttrs = [...attrs, { attrName: newAttrName, attrValue: [newAttrVal] }];
    setAttrs(newAttrs);
    setNewAttrName(''); setNewAttrVal(''); setShowNewAttr(false);
    generateMultiAttr(newAttrs);
  };

  // 删除规格
  const handleRemoveAttr = (index: number) => {
    const newAttrs = attrs.filter((_, i) => i !== index);
    setAttrs(newAttrs);
    generateMultiAttr(newAttrs);
  };

  // 添加规格值 tag
  const handleAddTag = (attrIndex: number) => {
    if (!tagInputVal.trim()) return;
    const newAttrs = [...attrs];
    if (newAttrs[attrIndex].attrValue.includes(tagInputVal.trim())) {
      message.warning('规格值已存在'); return;
    }
    newAttrs[attrIndex] = { ...newAttrs[attrIndex], attrValue: [...newAttrs[attrIndex].attrValue, tagInputVal.trim()] };
    setAttrs(newAttrs);
    setTagInputIndex(null); setTagInputVal('');
    generateMultiAttr(newAttrs);
  };

  // 删除规格值 tag
  const handleRemoveTag = (attrIndex: number, tagIndex: number) => {
    const newAttrs = [...attrs];
    newAttrs[attrIndex] = { ...newAttrs[attrIndex], attrValue: newAttrs[attrIndex].attrValue.filter((_, i) => i !== tagIndex) };
    if (newAttrs[attrIndex].attrValue.length === 0) {
      newAttrs.splice(attrIndex, 1);
    }
    setAttrs(newAttrs);
    generateMultiAttr(newAttrs);
  };

  // 选择规格模板
  const handleSelectRule = (ruleId: number) => {
    const rule = ruleList.find((r: any) => r.id === ruleId);
    if (!rule) return;
    try {
      const ruleValue = typeof rule.ruleValue === 'string' ? JSON.parse(rule.ruleValue) : rule.ruleValue;
      if (Array.isArray(ruleValue)) {
        setAttrs(ruleValue);
        generateMultiAttr(ruleValue);
      }
    } catch { /* noop */ }
  };

  // 批量设置
  const handleBatchAdd = () => {
    setMultiAttrValues((prev) => prev.map((row) => ({
      ...row,
      image: batchRow.image || row.image,
      price: batchRow.price || row.price,
      cost: batchRow.cost || row.cost,
      otPrice: batchRow.otPrice || row.otPrice,
      stock: batchRow.stock || row.stock,
      barCode: batchRow.barCode || row.barCode,
      weight: batchRow.weight || row.weight,
      volume: batchRow.volume || row.volume,
      brokerage: batchRow.brokerage || row.brokerage,
      brokerageTwo: batchRow.brokerageTwo || row.brokerageTwo,
    })));
    message.success('批量设置成功');
  };

  // 删除多规格行
  const handleDelAttrRow = (index: number) => {
    setMultiAttrValues((prev) => prev.filter((_, i) => i !== index));
  };

  // 提交
  const handleSubmit = async () => {
    try {
      const values = await form.validateFields();
      if (!coverImage) { message.warning('请上传商品封面图'); setCurrentTab('0'); return; }
      if (sliderImages.length === 0) { message.warning('请上传商品轮播图'); setCurrentTab('0'); return; }

      setSaving(true);
      const data: any = {
        ...values,
        image: coverImage,
        sliderImages: JSON.stringify(sliderImages),
        videoLink,
        content,
        specType,
        isSub,
        cateIds: Array.isArray(values.cateIds) ? values.cateIds.join(',') : values.cateIds,
        isGood: checkboxGroup.includes('isGood') ? 1 : 0,
        isHot: checkboxGroup.includes('isHot') ? 1 : 0,
        isBenefit: checkboxGroup.includes('isBenefit') ? 1 : 0,
        isBest: checkboxGroup.includes('isBest') ? 1 : 0,
        isNew: checkboxGroup.includes('isNew') ? 1 : 0,
        activity,
        attr: specType ? attrs : [],
        attrValue: specType ? multiAttrValues : [singleAttr],
      };

      if (editId) {
        data.id = Number(editId);
        await productUpdateApi(data);
        message.success('修改成功');
      } else {
        await productCreateApi(data);
        message.success('添加成功');
      }
      navigate('/store/index');
    } catch (e: any) {
      if (e?.errorFields) message.warning('请完善必填项');
    } finally { setSaving(false); }
  };

  // SKU 表格通用列
  const skuFields = [
    { key: 'price', title: '售价(元)' }, { key: 'cost', title: '成本价(元)' },
    { key: 'otPrice', title: '原价(元)' }, { key: 'stock', title: '库存' },
    { key: 'barCode', title: '商品编号' }, { key: 'weight', title: '重量(kg)' },
    { key: 'volume', title: '体积(m³)' },
  ];

  const brokerageFields = [
    { key: 'brokerage', title: '一级返佣(元)' },
    { key: 'brokerageTwo', title: '二级返佣(元)' },
  ];

  // 单规格列
  const singleColumns: ColumnsType<any> = [
    ...skuFields.map((f) => ({
      title: f.title, dataIndex: f.key, width: 120,
      render: (_: any, __: any, idx: number) => (
        <InputNumber size="small" disabled={isDisabled} min={0} style={{ width: '100%' }}
          value={singleAttr[f.key]}
          onChange={(v) => setSingleAttr((prev: any) => ({ ...prev, [f.key]: v }))}
          {...(f.key === 'barCode' ? {} : { precision: f.key === 'stock' ? 0 : 2 })}
        />
      ),
    })),
    ...(isSub ? brokerageFields.map((f) => ({
      title: f.title, dataIndex: f.key, width: 120,
      render: () => (
        <InputNumber size="small" disabled={isDisabled} min={0} precision={2} style={{ width: '100%' }}
          value={singleAttr[f.key]}
          onChange={(v: any) => setSingleAttr((prev: any) => ({ ...prev, [f.key]: v }))}
        />
      ),
    })) : []),
  ];

  // 多规格列
  const multiColumns: ColumnsType<any> = [
    ...attrs.map((a) => ({ title: a.attrName, dataIndex: a.attrName, width: 80 })),
    ...skuFields.map((f) => ({
      title: f.title, dataIndex: f.key, width: 120,
      render: (v: any, _: any, idx: number) => (
        <InputNumber size="small" disabled={isDisabled} min={0} style={{ width: '100%' }}
          value={v} onChange={(val) => {
            setMultiAttrValues((prev) => prev.map((row, i) => i === idx ? { ...row, [f.key]: val } : row));
          }}
        />
      ),
    })),
    ...(isSub ? brokerageFields.map((f) => ({
      title: f.title, dataIndex: f.key, width: 120,
      render: (v: any, _: any, idx: number) => (
        <InputNumber size="small" disabled={isDisabled} min={0} precision={2} style={{ width: '100%' }}
          value={v} onChange={(val: any) => {
            setMultiAttrValues((prev) => prev.map((row, i) => i === idx ? { ...row, [f.key]: val } : row));
          }}
        />
      ),
    })) : []),
    ...(!isDisabled ? [{
      title: '操作', width: 60, render: (_: any, __: any, idx: number) => (
        <a style={{ color: '#ff4d4f' }} onClick={() => handleDelAttrRow(idx)}>删除</a>
      ),
    }] : []),
  ];

  // 批量设置列
  const batchColumns: ColumnsType<any> = [
    ...skuFields.map((f) => ({
      title: f.title, dataIndex: f.key, width: 120,
      render: () => (
        <InputNumber size="small" min={0} style={{ width: '100%' }}
          value={batchRow[f.key]}
          onChange={(v) => setBatchRow((prev: any) => ({ ...prev, [f.key]: v }))}
        />
      ),
    })),
    ...(isSub ? brokerageFields.map((f) => ({
      title: f.title, dataIndex: f.key, width: 120,
      render: () => (
        <InputNumber size="small" min={0} precision={2} style={{ width: '100%' }}
          value={batchRow[f.key]}
          onChange={(v: any) => setBatchRow((prev: any) => ({ ...prev, [f.key]: v }))}
        />
      ),
    })) : []),
    { title: '操作', width: 80, render: () => <a onClick={handleBatchAdd}>批量添加</a> },
  ];

  return (
    <Spin spinning={pageLoading}>
      <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
        {/* 顶部 */}
        <Card>
          <div style={{ display: 'flex', alignItems: 'center', gap: 12 }}>
            <Button icon={<ArrowLeftOutlined />} onClick={() => navigate('/store/index')} />
            <span style={{ fontSize: 16, fontWeight: 500 }}>
              {editId ? (isDisabled ? '商品详情' : '编辑商品') : '添加商品'}
            </span>
          </div>
        </Card>

        <Card>
          <Tabs activeKey={currentTab} onChange={setCurrentTab}
            items={[
              { key: '0', label: '商品信息' },
              { key: '1', label: '规格库存' },
              { key: '2', label: '商品详情' },
              { key: '3', label: '其他设置' },
            ]} />

          <Form form={form} labelCol={{ xs: { span: 24 }, sm: { span: 3 } }} wrapperCol={{ xs: { span: 24 }, sm: { span: 20 } }} style={{ marginTop: 16 }}>
            {/* ===== Tab0 商品信息 ===== */}
            <div style={{ display: currentTab === '0' ? 'block' : 'none' }}>
              <Form.Item label="商品名称" name="storeName" rules={[{ required: true, message: '请输入商品名称' }]}>
                <Input placeholder="请输入商品名称" maxLength={249} disabled={isDisabled} />
              </Form.Item>
              <Form.Item label="商品分类" name="cateIds" rules={[{ required: true, message: '请选择商品分类' }]}>
                <Cascader options={categoryList} fieldNames={{ label: 'name', value: 'id', children: 'child' }}
                  placeholder="请选择分类" changeOnSelect allowClear disabled={isDisabled} style={{ maxWidth: 400 }} />
              </Form.Item>
              <Form.Item label="商品关键字" name="keyword" rules={[{ required: true, message: '请输入商品关键字' }]}>
                <Input placeholder="请输入商品关键字" disabled={isDisabled} />
              </Form.Item>
              <Form.Item label="单位" name="unitName" rules={[{ required: true, message: '请输入单位' }]}>
                <Input placeholder="请输入单位" disabled={isDisabled} style={{ maxWidth: 200 }} />
              </Form.Item>
              <Form.Item label="商品封面图" required>
                <div style={{ display: 'flex', gap: 8, flexWrap: 'wrap', alignItems: 'center' }}>
                  {coverImage && (
                    <div style={{ position: 'relative' }}>
                      <Image src={coverImage} width={80} height={80} style={{ borderRadius: 4, objectFit: 'cover' }} />
                      {!isDisabled && <DeleteOutlined style={{ position: 'absolute', top: -6, right: -6, color: '#ff4d4f', cursor: 'pointer' }}
                        onClick={() => setCoverImage('')} />}
                    </div>
                  )}
                  {!coverImage && !isDisabled && (
                    <Button icon={<PlusOutlined />} onClick={() => setCoverPickerOpen(true)}>选择封面图</Button>
                  )}
                </div>
                <div style={{ color: '#999', fontSize: 12, marginTop: 4 }}>建议尺寸：800*800px</div>
                <MaterialPicker open={coverPickerOpen} onCancel={() => setCoverPickerOpen(false)}
                  onOk={(urls) => { setCoverImage(urls[0] || ''); setCoverPickerOpen(false); }} />
              </Form.Item>
              <Form.Item label="商品轮播图" required>
                <div style={{ display: 'flex', gap: 8, flexWrap: 'wrap', alignItems: 'center' }}>
                  {sliderImages.map((img, idx) => (
                    <div key={idx} style={{ position: 'relative' }}>
                      <Image src={img} width={80} height={80} style={{ borderRadius: 4, objectFit: 'cover' }} />
                      {!isDisabled && <DeleteOutlined style={{ position: 'absolute', top: -6, right: -6, color: '#ff4d4f', cursor: 'pointer' }}
                        onClick={() => setSliderImages((prev) => prev.filter((_, i) => i !== idx))} />}
                    </div>
                  ))}
                  {sliderImages.length < 10 && !isDisabled && (
                    <Button icon={<PlusOutlined />} onClick={() => setSliderPickerOpen(true)}>选择轮播图</Button>
                  )}
                </div>
                <div style={{ color: '#999', fontSize: 12, marginTop: 4 }}>最多上传10张</div>
                <MaterialPicker open={sliderPickerOpen} onCancel={() => setSliderPickerOpen(false)} multiple
                  limit={10 - sliderImages.length}
                  onOk={(urls) => { setSliderImages((prev) => [...prev, ...urls].slice(0, 10)); setSliderPickerOpen(false); }} />
              </Form.Item>
              <Form.Item label="主图视频">
                {isDisabled ? (
                  videoLink ? (
                    <video src={videoLink} controls muted style={{ width: 300, maxWidth: '100%', borderRadius: 8 }} />
                  ) : <span style={{ color: '#999' }}>无</span>
                ) : (
                  <>
                    <Space wrap>
                      <Button icon={<UploadOutlined />} onClick={() => setVideoPickerOpen(true)}>选择视频</Button>
                      <Input placeholder="或输入视频链接" value={videoLink} onChange={(e) => setVideoLink(e.target.value)}
                        style={{ width: 300 }} allowClear />
                      {videoLink && <Button danger onClick={() => setVideoLink('')}>删除视频</Button>}
                    </Space>
                    {videoLink && (
                      <div style={{ marginTop: 8 }}>
                        <video src={videoLink} controls muted style={{ width: 300, maxWidth: '100%', borderRadius: 8 }} />
                      </div>
                    )}
                    <MaterialPicker open={videoPickerOpen} onCancel={() => setVideoPickerOpen(false)} accept="video"
                      onOk={(urls) => { setVideoLink(urls[0] || ''); setVideoPickerOpen(false); }} />
                  </>
                )}
              </Form.Item>
              <Form.Item label="运费模板" name="tempId" rules={[{ required: true, message: '请选择运费模板' }]}>
                <div style={{ display: 'flex', gap: 8, alignItems: 'center', flexWrap: 'wrap' }}>
                  <Select placeholder="请选择运费模板" disabled={isDisabled} style={{ maxWidth: 400, flex: 1 }}
                    value={form.getFieldValue('tempId')} onChange={(v) => form.setFieldsValue({ tempId: v })}>
                    {shippingList.map((item: any) => (
                      <Select.Option key={item.id} value={item.id}>{item.name}</Select.Option>
                    ))}
                  </Select>
                  {!isDisabled && (
                    <Button onClick={() => {
                      shippingTemplatesList({ page: 1, limit: 100 }).then((res: any) => setShippingList(res?.list || [])).catch(() => {});
                    }}>
                      <ReloadOutlined /> 刷新
                    </Button>
                  )}
                </div>
              </Form.Item>
            </div>

            {/* ===== Tab1 规格库存 ===== */}
            <div style={{ display: currentTab === '1' ? 'block' : 'none' }}>
              <Form.Item label="商品规格">
                <Radio.Group value={specType} onChange={(e) => setSpecType(e.target.value)} disabled={isDisabled}>
                  <Radio value={false}>单规格</Radio>
                  <Radio value={true}>多规格</Radio>
                </Radio.Group>
              </Form.Item>
              <Form.Item label="佣金设置">
                <Radio.Group value={isSub} onChange={(e) => setIsSub(e.target.value)} disabled={isDisabled}>
                  <Radio value={true}>单独设置</Radio>
                  <Radio value={false}>默认设置</Radio>
                </Radio.Group>
              </Form.Item>

              {/* 多规格 - 选择规格模板 & 添加规格 */}
              {specType && !isDisabled && (
                <>
                  <Form.Item label="选择规格">
                    <Space wrap>
                      <Select placeholder="选择规格模板" style={{ width: 200 }}
                        onChange={(v: number) => handleSelectRule(v)}>
                        {ruleList.map((item: any) => (
                          <Select.Option key={item.id} value={item.id}>{item.ruleName}</Select.Option>
                        ))}
                      </Select>
                    </Space>
                  </Form.Item>

                  {/* 已添加的规格 */}
                  {attrs.map((attr, attrIdx) => (
                    <Form.Item label={attr.attrName} key={attrIdx}>
                      <div style={{ display: 'flex', flexWrap: 'wrap', gap: 6, alignItems: 'center' }}>
                        {attr.attrValue.map((val, tagIdx) => (
                          <Tag key={tagIdx} closable onClose={() => handleRemoveTag(attrIdx, tagIdx)}>{val}</Tag>
                        ))}
                        {tagInputIndex === attrIdx ? (
                          <Input size="small" style={{ width: 100 }} autoFocus value={tagInputVal}
                            onChange={(e) => setTagInputVal(e.target.value)}
                            onBlur={() => handleAddTag(attrIdx)}
                            onPressEnter={() => handleAddTag(attrIdx)} />
                        ) : (
                          <Button size="small" onClick={() => { setTagInputIndex(attrIdx); setTagInputVal(''); }}>+ 添加</Button>
                        )}
                        <DeleteOutlined style={{ color: '#ff4d4f', cursor: 'pointer', marginLeft: 8 }}
                          onClick={() => handleRemoveAttr(attrIdx)} />
                      </div>
                    </Form.Item>
                  ))}

                  {/* 添加新规格 */}
                  {showNewAttr ? (
                    <Form.Item label="新规格">
                      <Space>
                        <Input placeholder="规格名" value={newAttrName} onChange={(e) => setNewAttrName(e.target.value)} style={{ width: 120 }} />
                        <Input placeholder="规格值" value={newAttrVal} onChange={(e) => setNewAttrVal(e.target.value)} style={{ width: 120 }} />
                        <Button type="primary" onClick={handleAddAttrName}>确定</Button>
                        <Button onClick={() => setShowNewAttr(false)}>取消</Button>
                      </Space>
                    </Form.Item>
                  ) : (
                    <Form.Item label=" " colon={false}>
                      <Button type="primary" onClick={() => setShowNewAttr(true)}>添加新规格</Button>
                    </Form.Item>
                  )}

                  {/* 批量设置 */}
                  {attrs.length > 0 && (
                    <Form.Item label="批量设置">
                      <div style={{ overflowX: 'auto' }}>
                        <Table dataSource={[batchRow]} columns={batchColumns} rowKey={() => 'batch'}
                          size="small" pagination={false} bordered scroll={{ x: 900 }} />
                      </div>
                    </Form.Item>
                  )}
                </>
              )}

              {/* 单规格表格 */}
              {!specType && (
                <Form.Item label="商品属性">
                  <div style={{ overflowX: 'auto' }}>
                    <Table dataSource={[singleAttr]} columns={singleColumns} rowKey={() => 'single'}
                      size="small" pagination={false} bordered scroll={{ x: 900 }} />
                  </div>
                </Form.Item>
              )}

              {/* 多规格表格 */}
              {specType && attrs.length > 0 && multiAttrValues.length > 0 && (
                <Form.Item label="商品属性">
                  <div style={{ overflowX: 'auto' }}>
                    <Table dataSource={multiAttrValues} columns={multiColumns}
                      rowKey={(_, idx) => String(idx)}
                      size="small" pagination={false} bordered scroll={{ x: 1200 }} />
                  </div>
                </Form.Item>
              )}
            </div>

            {/* ===== Tab2 商品详情 ===== */}
            <div style={{ display: currentTab === '2' ? 'block' : 'none' }}>
              <Form.Item label="商品详情">
                {isDisabled ? (
                  <div data-color-mode="light">
                    <MDEditor.Markdown source={content || '无'} style={{ padding: 16, minHeight: 200, border: '1px solid #d9d9d9', borderRadius: 4 }} />
                  </div>
                ) : (
                  <div data-color-mode="light">
                    <MDEditor
                      value={content}
                      onChange={(val) => setContent(val || '')}
                      height={500}
                      preview="live"
                    />
                  </div>
                )}
              </Form.Item>
            </div>

            {/* ===== Tab3 其他设置 ===== */}
            <div style={{ display: currentTab === '3' ? 'block' : 'none' }}>
              <Form.Item label="排序" name="sort" initialValue={0}>
                <InputNumber min={0} disabled={isDisabled} style={{ width: 260 }} />
              </Form.Item>
              <Form.Item label="积分" name="giveIntegral" initialValue={0}>
                <InputNumber min={0} disabled={isDisabled} style={{ width: 260 }} />
              </Form.Item>
              <Form.Item label="虚拟销量" name="ficti" initialValue={0}>
                <InputNumber min={0} disabled={isDisabled} style={{ width: 260 }} />
              </Form.Item>

              <Form.Item label="商品推荐">
                <Checkbox.Group value={checkboxGroup} onChange={(v) => setCheckboxGroup(v as string[])}
                  disabled={isDisabled} options={recommendOptions} />
              </Form.Item>

              <Form.Item label="活动优先级">
                <div style={{ display: 'flex', gap: 8, flexWrap: 'wrap' }}>
                  {activity.map((item) => {
                    const colorMap: Record<string, string> = {
                      '默认': '#0256FF', '秒杀': '#ff4d4f', '砍价': '#faad14', '拼团': '#52c41a',
                    };
                    return (
                      <Tag key={item} color={colorMap[item] || '#0256FF'}
                        style={{ padding: '4px 16px', fontSize: 14, cursor: isDisabled ? 'default' : 'move' }}>
                        {item}
                      </Tag>
                    );
                  })}
                </div>
                <div style={{ color: '#999', fontSize: 12, marginTop: 4 }}>拖拽可调整优先级顺序</div>
              </Form.Item>

              <Form.Item label="关联优惠券" name="couponIds">
                <Select mode="multiple" placeholder="请选择优惠券" disabled={isDisabled}
                  style={{ maxWidth: 500 }} allowClear>
                  {couponList.map((item: any) => (
                    <Select.Option key={item.id} value={item.id}>{item.name}</Select.Option>
                  ))}
                </Select>
              </Form.Item>
            </div>
          </Form>

          {/* 底部按钮 */}
          {!isDisabled && (
            <div style={{ display: 'flex', justifyContent: 'center', marginTop: 24, gap: 16 }}>
              {currentTab !== '0' && (
                <Button onClick={() => setCurrentTab(String(Number(currentTab) - 1))}>上一步</Button>
              )}
              {currentTab !== '3' ? (
                <Button type="primary" onClick={() => setCurrentTab(String(Number(currentTab) + 1))}>下一步</Button>
              ) : (
                <Button type="primary" loading={saving} onClick={handleSubmit}>
                  {editId ? '保存修改' : '提交商品'}
                </Button>
              )}
            </div>
          )}
        </Card>
      </div>
    </Spin>
  );
};

export default CreateStore;

