import React from 'react';
import {
  Form,
  Input,
  InputNumber,
  Select,
  Radio,
  Checkbox,
  Button,
  Switch,
} from 'antd';
import UploadPicture from '@/components/UploadPicture';

interface FormField {
  __config__: {
    tag: string;
    label: string;
    required?: boolean;
    tagIcon?: string;
    defaultValue?: any;
  };
  __vModel__: string;
  __slot__?: {
    options?: { label: string; value: any }[];
  };
  placeholder?: string;
  type?: string;
  disabled?: boolean;
  maxlength?: number;
  min?: number;
  max?: number;
  step?: number;
  autosize?: { minRows?: number; maxRows?: number } | boolean;
}

interface FormConf {
  fields: FormField[];
  labelWidth?: number;
}

interface FormParserProps {
  formConf: FormConf;
  formData: Record<string, any>;
  onSubmit: (values: Record<string, any>) => void;
  loading?: boolean;
}

const renderField = (field: FormField) => {
  const tag = field.__config__?.tag;
  const type = field.type;

  switch (tag) {
    case 'el-input':
      if (type === 'textarea' || field.autosize) {
        const autoSize =
          typeof field.autosize === 'object'
            ? { minRows: field.autosize.minRows || 3, maxRows: field.autosize.maxRows || 6 }
            : { minRows: 3, maxRows: 6 };
        return (
          <Input.TextArea
            placeholder={field.placeholder}
            disabled={field.disabled}
            maxLength={field.maxlength}
            autoSize={autoSize}
          />
        );
      }
      return (
        <Input
          placeholder={field.placeholder}
          disabled={field.disabled}
          maxLength={field.maxlength}
        />
      );

    case 'el-input-number':
      return (
        <InputNumber
          min={field.min}
          max={field.max}
          step={field.step}
          disabled={field.disabled}
          style={{ width: '100%' }}
        />
      );

    case 'el-select':
      return (
        <Select
          placeholder={field.placeholder}
          disabled={field.disabled}
          options={field.__slot__?.options?.map((o) => ({
            label: o.label,
            value: o.value,
          }))}
        />
      );

    case 'el-radio-group':
      return (
        <Radio.Group disabled={field.disabled}>
          {field.__slot__?.options?.map((o) => (
            <Radio key={String(o.value)} value={o.value}>
              {o.label}
            </Radio>
          ))}
        </Radio.Group>
      );

    case 'el-checkbox-group':
      return (
        <Checkbox.Group
          disabled={field.disabled}
          options={field.__slot__?.options?.map((o) => ({
            label: o.label,
            value: o.value,
          }))}
        />
      );

    case 'el-switch':
      return <Switch disabled={field.disabled} />;

    case 'self-upload':
      return <UploadPicture limit={1} />;

    default:
      return <Input placeholder={field.placeholder} />;
  }
};

const FormParser: React.FC<FormParserProps> = ({
  formConf,
  formData,
  onSubmit,
  loading,
}) => {
  const [form] = Form.useForm();

  React.useEffect(() => {
    if (formData && Object.keys(formData).length > 0) {
      // Convert switch values: "1"/"0" -> boolean for el-switch
      const converted: Record<string, any> = {};
      formConf.fields.forEach((field) => {
        const key = field.__vModel__;
        const val = formData[key];
        if (field.__config__?.tag === 'el-switch') {
          converted[key] = val === '1' || val === 1 || val === true;
        } else if (field.__config__?.tag === 'el-checkbox-group' && typeof val === 'string') {
          try {
            converted[key] = JSON.parse(val);
          } catch {
            converted[key] = val;
          }
        } else {
          converted[key] = val ?? field.__config__?.defaultValue;
        }
      });
      form.setFieldsValue(converted);
    }
  }, [formData, formConf, form]);

  const handleFinish = (values: Record<string, any>) => {
    // Convert back: boolean -> "1"/"0" for switches, arrays -> JSON string
    const result: Record<string, any> = {};
    formConf.fields.forEach((field) => {
      const key = field.__vModel__;
      const val = values[key];
      if (field.__config__?.tag === 'el-switch') {
        result[key] = val ? '1' : '0';
      } else if (Array.isArray(val)) {
        result[key] = JSON.stringify(val);
      } else {
        result[key] = val != null ? String(val) : '';
      }
    });
    onSubmit(result);
  };

  return (
    <Form
      form={form}
      labelCol={{ span: 4 }}
      wrapperCol={{ span: 16 }}
      onFinish={handleFinish}
    >
      {formConf.fields.map((field) => (
        <Form.Item
          key={field.__vModel__}
          label={field.__config__?.label}
          name={field.__vModel__}
          rules={
            field.__config__?.required
              ? [{ required: true, message: `请输入${field.__config__?.label}` }]
              : undefined
          }
          valuePropName={
            field.__config__?.tag === 'el-switch' ? 'checked' : 'value'
          }
        >
          {renderField(field)}
        </Form.Item>
      ))}
      <Form.Item wrapperCol={{ offset: 4 }}>
        <Button type="primary" htmlType="submit" loading={loading}>
          保存设置
        </Button>
      </Form.Item>
    </Form>
  );
};

export default FormParser;
