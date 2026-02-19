import React, { useState, useEffect } from 'react';
import { Card, Form, InputNumber, Button, message, Spin } from 'antd';
import { configInfo, configSaveForm } from '@/api/systemConfig';

const FORM_ID = 109;

// 积分配置字段定义，与Vue zbParser渲染一致
const configFields = [
  {
    name: 'integral_ratio',
    label: '积分抵用比例(订单)',
    tip: '积分抵用比例(1积分抵多少金额)，0为不开启',
    min: 0, precision: 2, step: 0.01,
  },
  {
    name: 'order_give_integral',
    label: '下单赠送积分比例',
    tip: '下单赠送积分比例(实际支付1元赠送多少积分)，0为不开启',
    min: 0, precision: 0, step: 1,
  },
  {
    name: 'freeze_integral_day',
    label: '积分冻结天数',
    tip: '用户收货后积分冻结天数，0为不冻结',
    min: 0, precision: 0, step: 1,
  },
];

const IntegralConfig: React.FC = () => {
  const [form] = Form.useForm();
  const [loading, setLoading] = useState(false);
  const [saving, setSaving] = useState(false);

  const getFormInfo = async () => {
    setLoading(true);
    try {
      const res = await configInfo({ id: FORM_ID });
      if (res && typeof res === 'object') {
        const values: Record<string, any> = {};
        configFields.forEach((f) => {
          const val = res[f.name];
          values[f.name] = val !== undefined && val !== null ? Number(val) : 0;
        });
        form.setFieldsValue(values);
      }
    } catch {
      message.error('获取积分配置失败');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => { getFormInfo(); }, []);

  const handleSubmit = async () => {
    setSaving(true);
    try {
      const values = await form.validateFields();
      const fields = configFields.map((f) => ({
        name: f.name, title: f.name, value: values[f.name],
      }));
      await configSaveForm({ fields, id: FORM_ID, sort: 0, status: true });
      message.success('操作成功');
      getFormInfo();
    } catch { /* noop */ }
    finally { setSaving(false); }
  };

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
      <Card title="积分配置">
        <Spin spinning={loading}>
          <Form form={form} labelCol={{ span: 6 }} wrapperCol={{ span: 12 }}>
            {configFields.map((f) => (
              <Form.Item key={f.name} label={f.label} name={f.name}
                extra={<span style={{ color: '#999', fontSize: 12 }}>{f.tip}</span>}>
                <InputNumber min={f.min} precision={f.precision} step={f.step}
                  style={{ width: '100%' }} />
              </Form.Item>
            ))}
            <Form.Item wrapperCol={{ offset: 6, span: 12 }}>
              <Button type="primary" onClick={handleSubmit} loading={saving}>保存</Button>
            </Form.Item>
          </Form>
        </Spin>
      </Card>
    </div>
  );
};

export default IntegralConfig;
