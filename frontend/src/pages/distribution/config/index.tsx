import React, { useState, useEffect } from 'react';
import { Card, Form, InputNumber, Switch, Button, Input, Radio, message, Spin, Divider } from 'antd';
import { configApi as distributionConfigApi, configUpdateApi as distributionConfigSaveApi } from '@/api/distribution';

const { TextArea } = Input;

const DistributionConfig: React.FC = () => {
  const [form] = Form.useForm();
  const [loading, setLoading] = useState(false);
  const [submitLoading, setSubmitLoading] = useState(false);

  useEffect(() => {
    setLoading(true);
    distributionConfigApi()
      .then((res: any) => {
        form.setFieldsValue({
          brokerageFuncStatus: res?.brokerageFuncStatus === 1,
          storeBrokerageQuota: res?.storeBrokerageQuota ?? 0,
          storeBrokerageRatio: res?.storeBrokerageRatio ?? 0,
          storeBrokerageTwo: res?.storeBrokerageTwo ?? 0,
          brokerageBindind: res?.brokerageBindind ?? 0,
          userExtractMinPrice: res?.userExtractMinPrice ?? 0,
          userExtractBank: res?.userExtractBank ?? '',
          extractTime: res?.extractTime ?? 0,
          storeBrokerageIsBubble: res?.storeBrokerageIsBubble === 1,
        });
      })
      .catch(() => message.error('获取配置失败'))
      .finally(() => setLoading(false));
  }, [form]);

  const handleSubmit = async (values: any) => {
    setSubmitLoading(true);
    try {
      const params = {
        brokerageFuncStatus: values.brokerageFuncStatus ? 1 : 0,
        storeBrokerageQuota: values.storeBrokerageQuota,
        storeBrokerageRatio: values.storeBrokerageRatio,
        storeBrokerageTwo: values.storeBrokerageTwo,
        brokerageBindind: values.brokerageBindind,
        userExtractMinPrice: values.userExtractMinPrice,
        userExtractBank: values.userExtractBank || '',
        extractTime: values.extractTime,
        storeBrokerageIsBubble: values.storeBrokerageIsBubble ? 1 : 0,
      };
      await distributionConfigSaveApi(params);
      message.success('保存成功');
    } catch { /* handled */ }
    finally { setSubmitLoading(false); }
  };

  return (
    <Spin spinning={loading}>
      <Card title="分销配置">
        <Form form={form} labelCol={{ span: 4 }} wrapperCol={{ span: 12 }} onFinish={handleSubmit}
          initialValues={{ brokerageFuncStatus: false, storeBrokerageQuota: 0, storeBrokerageRatio: 0,
            storeBrokerageTwo: 0, brokerageBindind: 0, userExtractMinPrice: 0, userExtractBank: '',
            extractTime: 0, storeBrokerageIsBubble: false }}>

          <Divider orientation="left">基础设置</Divider>

          <Form.Item label="分销功能开关" name="brokerageFuncStatus" valuePropName="checked"
            extra="关闭后分销功能将不可用">
            <Switch checkedChildren="开启" unCheckedChildren="关闭" />
          </Form.Item>

          <Form.Item label="分销模式" name="storeBrokerageQuota"
            extra="-1为关闭，0为用户购买后自动成为分销员，大于0为购买金额达到该值后自动成为分销员">
            <Radio.Group>
              <Radio value={-1}>关闭</Radio>
              <Radio value={0}>人人分销</Radio>
              <Radio value={1}>指定分销</Radio>
            </Radio.Group>
          </Form.Item>

          <Form.Item label="分销关系绑定" name="brokerageBindind"
            extra="绑定类型：所有用户都可以绑定推广关系，或仅新用户首次访问时绑定">
            <Radio.Group>
              <Radio value={0}>所有用户</Radio>
              <Radio value={1}>新用户</Radio>
            </Radio.Group>
          </Form.Item>

          <Form.Item label="分销气泡展示" name="storeBrokerageIsBubble" valuePropName="checked"
            extra="是否在小程序/公众号展示分销气泡提示">
            <Switch checkedChildren="展示" unCheckedChildren="不展示" />
          </Form.Item>

          <Divider orientation="left">佣金设置</Divider>

          <Form.Item label="一级返佣比例(%)" name="storeBrokerageRatio"
            rules={[{ required: true, message: '请输入一级返佣比例' }]}
            extra="一级推广人获得的佣金比例，一级+二级不能超过100%">
            <InputNumber min={0} max={100} style={{ width: 200 }} />
          </Form.Item>

          <Form.Item label="二级返佣比例(%)" name="storeBrokerageTwo"
            rules={[{ required: true, message: '请输入二级返佣比例' }]}
            extra="二级推广人获得的佣金比例，一级+二级不能超过100%">
            <InputNumber min={0} max={100} style={{ width: 200 }} />
          </Form.Item>

          <Form.Item label="冻结时间(天)" name="extractTime"
            rules={[{ required: true, message: '请输入冻结时间' }]}
            extra="佣金冻结天数，冻结期间不可提现">
            <InputNumber min={0} style={{ width: 200 }} />
          </Form.Item>

          <Divider orientation="left">提现设置</Divider>

          <Form.Item label="提现最低金额" name="userExtractMinPrice"
            rules={[{ required: true, message: '请输入提现最低金额' }]}
            extra="用户申请提现的最低金额限制">
            <InputNumber min={0} precision={2} style={{ width: 200 }} prefix="¥" />
          </Form.Item>

          <Form.Item label="提现银行" name="userExtractBank"
            extra="支持的提现银行列表，每行一个银行名称">
            <TextArea rows={4} placeholder="请输入提现银行，每行一个" />
          </Form.Item>

          <Form.Item wrapperCol={{ offset: 4 }}>
            <Button type="primary" htmlType="submit" loading={submitLoading}>保存配置</Button>
          </Form.Item>
        </Form>
      </Card>
    </Spin>
  );
};

export default DistributionConfig;
