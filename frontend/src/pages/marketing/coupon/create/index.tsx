import React, { useState, useEffect } from 'react';
import { Card, Form, Input, InputNumber, Select, DatePicker, Radio, Button, Space, message, Cascader, Image } from 'antd';
import { CloseCircleOutlined, CameraOutlined } from '@ant-design/icons';
import { useNavigate, useParams } from 'react-router-dom';
import { couponSaveApi, couponInfoApi } from '@/api/marketing';
import { categoryApi } from '@/api/store';
import ProductPicker from '@/components/ProductPicker';
import dayjs from 'dayjs';

const { RangePicker } = DatePicker;

const CouponCreate: React.FC = () => {
  const navigate = useNavigate();
  const { id: editId } = useParams<{ id: string }>();
  const [form] = Form.useForm();
  const [loading, setLoading] = useState(false);

  // 表单状态
  const [useType, setUseType] = useState<number>(1); // 优惠券类型：1通用券 2商品券 3品类券
  const [threshold, setThreshold] = useState<boolean>(false); // 是否有门槛
  const [isFixedTime, setIsFixedTime] = useState<boolean>(false); // 使用有效期类型
  const [isForever, setIsForever] = useState<boolean>(false); // 领取是否限时
  const [isLimited, setIsLimited] = useState<boolean>(false); // 是否限量
  const [termTime, setTermTime] = useState<[dayjs.Dayjs, dayjs.Dayjs] | null>(null); // 使用有效期限
  const [isForeverTime, setIsForeverTime] = useState<[dayjs.Dayjs, dayjs.Dayjs] | null>(null); // 领取时间
  const [checkedProducts, setCheckedProducts] = useState<any[]>([]); // 选中的商品
  const [categoryList, setCategoryList] = useState<any[]>([]); // 品类列表
  const [productPickerOpen, setProductPickerOpen] = useState(false);

  // 加载品类数据
  useEffect(() => {
    categoryApi({ status: -1, type: 1 }).then((res: any) => {
      const formatData = (data: any[]): any[] => {
        return data.map((item) => ({
          value: item.id,
          label: item.name,
          disabled: !item.child || item.child.length === 0 ? false : true,
          children: item.child && item.child.length > 0 ? formatData(item.child) : undefined,
        }));
      };
      setCategoryList(formatData(res || []));
    }).catch(() => {});
  }, []);

  // 编辑时加载数据
  useEffect(() => {
    if (editId) {
      setLoading(true);
      couponInfoApi({ id: Number(editId) }).then((res: any) => {
        const info = res.coupon || res;
        setUseType(info.useType || 1);
        setThreshold(info.minPrice > 0);
        setIsFixedTime(info.isFixedTime || false);
        setIsForever(info.isForever || false);
        setIsLimited(info.isLimited || false);
        setCheckedProducts(res.product || []);

        if (info.isFixedTime && info.useStartTime && info.useEndTime) {
          setTermTime([dayjs(info.useStartTime), dayjs(info.useEndTime)]);
        }
        if (info.isForever && info.receiveStartTime && info.receiveEndTime) {
          setIsForeverTime([dayjs(info.receiveStartTime), dayjs(info.receiveEndTime)]);
        }

        form.setFieldsValue({
          name: info.name,
          useType: info.useType || 1,
          primaryKey: info.primaryKey ? Number(info.primaryKey) : undefined,
          money: info.money,
          minPrice: info.minPrice,
          day: info.day,
          type: info.type || 2,
          total: info.total,
          sort: info.sort,
          status: info.status,
        });
      }).catch(() => {
        message.error('获取优惠券信息失败');
      }).finally(() => {
        setLoading(false);
      });
    }
  }, [editId, form]);

  // 移除商品
  const handleRemoveProduct = (index: number) => {
    const newList = [...checkedProducts];
    newList.splice(index, 1);
    setCheckedProducts(newList);
  };

  // 提交表单
  const handleSubmit = async () => {
    // 验证
    if (isFixedTime && (!termTime || termTime.length !== 2)) {
      message.warning('请选择使用有效期限');
      return;
    }
    if (isForever && (!isForeverTime || isForeverTime.length !== 2)) {
      message.warning('请选择领取时间');
      return;
    }
    if (useType === 2 && checkedProducts.length === 0) {
      message.warning('请至少选择一个商品');
      return;
    }

    try {
      const values = await form.validateFields();
      setLoading(true);

      // 构建提交数据
      let primaryKey = '';
      if (useType === 2) {
        primaryKey = checkedProducts.map((item) => item.id).join(',');
      } else if (useType === 3) {
        primaryKey = values.primaryKey ? String(values.primaryKey) : '';
      }

      const data: any = {
        name: values.name,
        useType,
        primaryKey,
        money: values.money,
        minPrice: threshold ? values.minPrice : 0,
        isFixedTime,
        day: isFixedTime ? null : values.day,
        useStartTime: isFixedTime && termTime ? termTime[0].format('YYYY-MM-DD HH:mm:ss') : '',
        useEndTime: isFixedTime && termTime ? termTime[1].format('YYYY-MM-DD HH:mm:ss') : '',
        isForever,
        receiveStartTime: isForever && isForeverTime ? isForeverTime[0].format('YYYY-MM-DD HH:mm:ss') : '',
        receiveEndTime: isForever && isForeverTime ? isForeverTime[1].format('YYYY-MM-DD HH:mm:ss') : '',
        type: values.type,
        isLimited,
        total: isLimited ? values.total : 0,
        sort: values.sort || 0,
        status: values.status,
      };

      if (editId) {
        data.id = Number(editId);
      }

      await couponSaveApi(data);
      message.success(editId ? '修改成功' : '新增成功');
      setTimeout(() => {
        navigate('/marketing/coupon/list');
      }, 200);
    } catch {
      /* validation error */
    } finally {
      setLoading(false);
    }
  };

  return (
    <div>
      <Card title={editId ? '编辑优惠券' : '创建优惠券'}>
        <Form form={form} labelCol={{ span: 4 }} wrapperCol={{ span: 16 }} style={{ maxWidth: 800 }}>
          {/* 优惠券名称 */}
          <Form.Item label="优惠劵名称" name="name" rules={[{ required: true, message: '请输入优惠券名称' }]}>
            <Input placeholder="请输入优惠券名称" style={{ width: 350 }} />
          </Form.Item>

          {/* 优惠券类型 */}
          <Form.Item label="优惠劵类型">
            <Radio.Group value={useType} onChange={(e) => setUseType(e.target.value)}>
              <Radio value={1}>通用券</Radio>
              <Radio value={2}>商品券</Radio>
              <Radio value={3}>品类券</Radio>
            </Radio.Group>
          </Form.Item>

          {/* 选择品类 */}
          {useType === 3 && (
            <Form.Item label="选择品类" name="primaryKey" rules={[{ required: true, message: '请选择品类' }]}>
              <Cascader
                options={categoryList}
                placeholder="请选择"
                style={{ width: 350 }}
                changeOnSelect
                showSearch
              />
            </Form.Item>
          )}

          {/* 选择商品 */}
          {useType === 2 && (
            <Form.Item label="商品" required>
              <div style={{ display: 'flex', flexWrap: 'wrap', gap: 10 }}>
                {checkedProducts.map((item, index) => (
                  <div key={item.id} style={{ width: 60, height: 60, border: '1px dotted rgba(0,0,0,0.1)', position: 'relative', cursor: 'pointer' }}>
                    <Image src={item.image} width={58} height={58} preview={false} />
                    <CloseCircleOutlined
                      style={{ position: 'absolute', right: -8, top: -8, fontSize: 18, color: '#ff4d4f', cursor: 'pointer' }}
                      onClick={() => handleRemoveProduct(index)}
                    />
                  </div>
                ))}
                <div
                  style={{ width: 60, height: 60, border: '1px dashed #d9d9d9', display: 'flex', alignItems: 'center', justifyContent: 'center', cursor: 'pointer' }}
                  onClick={() => setProductPickerOpen(true)}
                >
                  <CameraOutlined style={{ fontSize: 24, color: '#999' }} />
                </div>
              </div>
            </Form.Item>
          )}

          {/* 优惠券面值 */}
          <Form.Item label="优惠券面值" name="money" rules={[{ required: true, message: '请输入优惠券面值' }]}>
            <InputNumber min={1} max={99999.99} precision={2} style={{ width: 200 }} />
          </Form.Item>

          {/* 使用门槛 */}
          <Form.Item label="使用门槛">
            <Radio.Group value={threshold} onChange={(e) => setThreshold(e.target.value)}>
              <Radio value={false}>无门槛</Radio>
              <Radio value={true}>有门槛</Radio>
            </Radio.Group>
          </Form.Item>

          {/* 优惠券最低消费 */}
          {threshold && (
            <Form.Item label="优惠券最低消费" name="minPrice" rules={[{ required: true, message: '请输入最低消费' }]}>
              <InputNumber min={1} precision={2} style={{ width: 200 }} />
            </Form.Item>
          )}

          {/* 使用有效期 */}
          <Form.Item label="使用有效期">
            <Radio.Group value={isFixedTime} onChange={(e) => setIsFixedTime(e.target.value)}>
              <Radio value={false}>天数</Radio>
              <Radio value={true}>时间段</Radio>
            </Radio.Group>
          </Form.Item>

          {/* 使用有效期限（天） */}
          {!isFixedTime && (
            <Form.Item label="使用有效期限（天）" name="day" rules={[{ required: true, message: '请输入使用有效期限（天）' }]}>
              <InputNumber min={0} max={999} style={{ width: 200 }} />
            </Form.Item>
          )}

          {/* 使用有效期限（时间段） */}
          {isFixedTime && (
            <Form.Item label="使用有效期限" required>
              <RangePicker
                showTime
                style={{ width: 450 }}
                value={termTime}
                onChange={(v) => setTermTime(v as [dayjs.Dayjs, dayjs.Dayjs] | null)}
                format="YYYY-MM-DD HH:mm:ss"
              />
            </Form.Item>
          )}

          {/* 领取是否限时 */}
          <Form.Item label="领取是否限时">
            <Radio.Group value={isForever} onChange={(e) => setIsForever(e.target.value)}>
              <Radio value={true}>限时</Radio>
              <Radio value={false}>不限时</Radio>
            </Radio.Group>
          </Form.Item>

          {/* 领取时间 */}
          {isForever && (
            <Form.Item label="领取时间" required>
              <RangePicker
                showTime
                style={{ width: 450 }}
                value={isForeverTime}
                onChange={(v) => setIsForeverTime(v as [dayjs.Dayjs, dayjs.Dayjs] | null)}
                format="YYYY-MM-DD HH:mm:ss"
              />
            </Form.Item>
          )}

          {/* 领取方式 */}
          <Form.Item label="领取方式" name="type" initialValue={2}>
            <Radio.Group>
              <Radio value={1}>手动领取</Radio>
              <Radio value={2}>新人券</Radio>
              <Radio value={3}>赠送券</Radio>
            </Radio.Group>
          </Form.Item>

          {/* 是否限量 */}
          <Form.Item label="是否限量">
            <Radio.Group value={isLimited} onChange={(e) => setIsLimited(e.target.value)}>
              <Radio value={true}>限量</Radio>
              <Radio value={false}>不限量</Radio>
            </Radio.Group>
          </Form.Item>

          {/* 发布数量 */}
          {isLimited && (
            <Form.Item label="发布数量" name="total" rules={[{ required: true, message: '请输入发布数量' }]}>
              <InputNumber min={1} style={{ width: 200 }} />
            </Form.Item>
          )}

          {/* 排序 */}
          <Form.Item label="排序" name="sort" initialValue={0}>
            <InputNumber min={0} style={{ width: 200 }} />
          </Form.Item>

          {/* 状态 */}
          <Form.Item label="状态" name="status" initialValue={false}>
            <Radio.Group>
              <Radio value={true}>开启</Radio>
              <Radio value={false}>关闭</Radio>
            </Radio.Group>
          </Form.Item>

          {/* 提交按钮 */}
          <Form.Item wrapperCol={{ offset: 4 }}>
            <Space>
              <Button type="primary" loading={loading} onClick={handleSubmit}>
                立即创建
              </Button>
              <Button onClick={() => navigate('/marketing/coupon/list')}>返回</Button>
            </Space>
          </Form.Item>
        </Form>
      </Card>

      {/* 商品选择器 */}
      <ProductPicker
        open={productPickerOpen}
        onCancel={() => setProductPickerOpen(false)}
        multiple
        selectedProducts={checkedProducts}
        onOk={(products) => {
          setCheckedProducts(products);
          setProductPickerOpen(false);
        }}
      />
    </div>
  );
};

export default CouponCreate;
