import React, { useEffect, useState } from 'react';
import { Form, Input, InputNumber, Select, Switch, Radio, Checkbox, Upload, Button, Spin, message } from 'antd';
import { UploadOutlined } from '@ant-design/icons';
import { getToken } from '@/utils/auth';

interface FormGeneratorProps {
  formConfig: any;
  onSubmit?: (values: any) => void;
  loading?: boolean;
}

const FormGenerator: React.FC<FormGeneratorProps> = ({ formConfig, onSubmit, loading = false }) => {
  const [form] = Form.useForm();

  useEffect(() => {
    if (formConfig?.fields) {
      const defaults: Record<string, any> = {};
      formConfig.fields.forEach((f: any) => {
        if (f.value !== undefined) defaults[f.name] = f.value;
      });
      form.setFieldsValue(defaults);
    }
  }, [formConfig]);

  const renderField = (field: any) => {
    switch (field.type) {
      case 'input':
        return <Input placeholder={field.placeholder} disabled={field.disabled} />;
      case 'number':
        return <InputNumber style={{ width: '100%' }} placeholder={field.placeholder} />;
      case 'textarea':
        return <Input.TextArea rows={field.rows || 4} placeholder={field.placeholder} />;
      case 'select':
        return (
          <Select placeholder={field.placeholder}>
            {(field.options || []).map((o: any) => (
              <Select.Option key={o.value} value={o.value}>{o.label}</Select.Option>
            ))}
          </Select>
        );
      case 'radio':
        return (
          <Radio.Group>
            {(field.options || []).map((o: any) => (
              <Radio key={o.value} value={o.value}>{o.label}</Radio>
            ))}
          </Radio.Group>
        );
      case 'checkbox':
        return (
          <Checkbox.Group>
            {(field.options || []).map((o: any) => (
              <Checkbox key={o.value} value={o.value}>{o.label}</Checkbox>
            ))}
          </Checkbox.Group>
        );
      case 'switch':
        return <Switch />;
      case 'upload':
        return (
          <Upload action="/api/admin/upload/image" headers={{ 'Authori-zation': getToken() || '' }}>
            <Button icon={<UploadOutlined />}>上传文件</Button>
          </Upload>
        );
      default:
        return <Input placeholder={field.placeholder} />;
    }
  };

  const handleFinish = (values: any) => {
    onSubmit?.(values);
  };

  if (!formConfig?.fields?.length) {
    return <Spin spinning={loading}><div style={{ padding: 24, textAlign: 'center', color: '#999' }}>暂无表单配置</div></Spin>;
  }

  return (
    <Spin spinning={loading}>
      <Form form={form} layout="vertical" onFinish={handleFinish}>
        {formConfig.fields.map((field: any, i: number) => (
          <Form.Item
            key={field.name || i}
            name={field.name}
            label={field.label}
            rules={field.required ? [{ required: true, message: `请输入${field.label}` }] : undefined}
            valuePropName={field.type === 'switch' ? 'checked' : 'value'}
          >
            {renderField(field)}
          </Form.Item>
        ))}
        <Form.Item>
          <Button type="primary" htmlType="submit">提交</Button>
        </Form.Item>
      </Form>
    </Spin>
  );
};

export default FormGenerator;
