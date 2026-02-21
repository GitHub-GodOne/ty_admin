import React, { useState, useEffect, useCallback, useMemo } from 'react';
import {
  Card, Table, Button, Space, Input, Modal, Form, message, Tabs, Select,
  Switch, InputNumber, Slider, Radio, Checkbox, Divider, Tooltip, Popconfirm,
} from 'antd';
import {
  PlusOutlined, SearchOutlined, ReloadOutlined, EditOutlined,
  DeleteOutlined, CopyOutlined, HolderOutlined,
  FontSizeOutlined, AlignLeftOutlined, NumberOutlined, UnorderedListOutlined,
  CheckSquareOutlined, SwapOutlined, UploadOutlined, LockOutlined,
} from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import {
  DndContext, closestCenter, PointerSensor, useSensor, useSensors,
} from '@dnd-kit/core';
import {
  SortableContext, verticalListSortingStrategy, useSortable, arrayMove,
} from '@dnd-kit/sortable';
import { CSS } from '@dnd-kit/utilities';
import {
  getFormConfigList, getFormConfigInfo, getFormConfigSave, getFormConfigEdit,
} from '@/api/systemFormConfig';

// ===================== 组件配置定义 =====================
let idGlobal = 100;

const inputComponents = [
  {
    __config__: { label: '单行文本', tag: 'el-input', tagIcon: 'input', required: true, changeTag: true, layout: 'colFormItem', span: 24, regList: [], tips: false, tipsDesc: '', tipsIsLink: false, tipsLink: '' },
    __slot__: { prepend: '', append: '' },
    placeholder: '请输入', style: { width: '95%' }, clearable: true, maxlength: null, 'show-word-limit': false, readonly: false, disabled: false,
  },
  {
    __config__: { label: '多行文本', tag: 'el-input', tagIcon: 'textarea', required: true, changeTag: true, layout: 'colFormItem', span: 24, regList: [], tips: false, tipsDesc: '', tipsIsLink: false, tipsLink: '' },
    type: 'textarea', placeholder: '请输入', autosize: { minRows: 4, maxRows: 4 }, style: { width: '95%' }, maxlength: null, 'show-word-limit': false, readonly: false, disabled: false,
  },
  {
    __config__: { label: '密码', tag: 'el-input', tagIcon: 'password', required: true, changeTag: true, layout: 'colFormItem', span: 24, regList: [], tips: false, tipsDesc: '', tipsIsLink: false, tipsLink: '' },
    __slot__: { prepend: '', append: '' },
    placeholder: '请输入', 'show-password': true, style: { width: '100%' }, clearable: true, maxlength: null, readonly: false, disabled: false,
  },
  {
    __config__: { label: '计数器', tag: 'el-input-number', tagIcon: 'number', required: true, changeTag: true, layout: 'colFormItem', span: 24, regList: [], tips: false, tipsDesc: '', tipsIsLink: false, tipsLink: '' },
    placeholder: '', min: undefined, max: undefined, step: 1, 'step-strictly': false, precision: undefined, 'controls-position': '', disabled: false,
  },
];

const selectComponents = [
  {
    __config__: { label: '下拉选择', tag: 'el-select', tagIcon: 'select', required: true, changeTag: true, layout: 'colFormItem', span: 24, regList: [], tips: false, tipsDesc: '', tipsIsLink: false, tipsLink: '' },
    __slot__: { options: [{ label: '选项一', value: 1 }, { label: '选项二', value: 2 }] },
    placeholder: '请选择', style: { width: '100%' }, clearable: true, disabled: false, filterable: false, multiple: false,
  },
  {
    __config__: { label: '单选框组', tag: 'el-radio-group', tagIcon: 'radio', required: true, changeTag: true, layout: 'colFormItem', span: 24, optionType: 'default', regList: [], border: false, tips: false, tipsDesc: '', tipsIsLink: false, tipsLink: '' },
    __slot__: { options: [{ label: '选项一', value: 1 }, { label: '选项二', value: 2 }] },
    style: {}, size: 'medium', disabled: false,
  },
  {
    __config__: { label: '多选框组', tag: 'el-checkbox-group', tagIcon: 'checkbox', defaultValue: [], required: true, changeTag: true, layout: 'colFormItem', span: 24, optionType: 'default', regList: [], border: false, tips: false, tipsDesc: '', tipsIsLink: false, tipsLink: '' },
    __slot__: { options: [{ label: '选项一', value: 1 }, { label: '选项二', value: 2 }] },
    style: {}, size: 'medium', min: null, max: null, disabled: false,
  },
  {
    __config__: { label: '开关', tag: 'el-switch', tagIcon: 'switch', defaultValue: false, required: true, changeTag: true, layout: 'colFormItem', span: 24, regList: [], tips: false, tipsDesc: '', tipsIsLink: false, tipsLink: '' },
    style: {}, disabled: false, 'active-text': '', 'inactive-text': '', 'active-color': null, 'inactive-color': null, 'active-value': true, 'inactive-value': false,
  },
  {
    __config__: { label: '自定义上传', tag: 'self-upload', tagIcon: 'selfUpload', layout: 'colFormItem', defaultValue: null, showLabel: true, required: true, span: 24, showTip: false, regList: [], changeTag: true, tips: false, tipsDesc: '', tipsIsLink: false, tipsLink: '' },
    __slot__: { 'list-type': true },
    disabled: true, accept: 'image', name: 'file', multiple: false,
  },
];

const defaultFormConf = {
  formRef: 'elForm', formModel: 'formData', size: 'medium',
  labelPosition: 'right', labelWidth: 100, formRules: 'rules',
  gutter: 15, disabled: false, span: 24, formBtns: true,
};

const componentIconMap: Record<string, React.ReactNode> = {
  input: <FontSizeOutlined />, textarea: <AlignLeftOutlined />, password: <LockOutlined />,
  number: <NumberOutlined />, select: <UnorderedListOutlined />, radio: <CheckSquareOutlined />,
  checkbox: <CheckSquareOutlined />, switch: <SwapOutlined />, selfUpload: <UploadOutlined />,
};

const hasOptions = (tag: string) => ['el-select', 'el-radio-group', 'el-checkbox-group'].includes(tag);

function cloneComponent(origin: any): any {
  const clone = JSON.parse(JSON.stringify(origin));
  const config = clone.__config__;
  config.formId = ++idGlobal;
  config.span = 24;
  config.renderKey = +new Date();
  clone.__vModel__ = `field${idGlobal}`;
  if (clone.placeholder !== undefined) clone.placeholder += config.label;
  return clone;
}

// ===================== 左侧面板：组件列表 =====================
const LeftPanel: React.FC<{ onAdd: (item: any) => void }> = ({ onAdd }) => {
  const groups = [
    { title: '输入型组件', list: inputComponents },
    { title: '选择型组件', list: selectComponents },
  ];
  return (
    <div style={{ width: 260, borderRight: '1px solid #f0f0f0', height: '100%', overflow: 'auto', background: '#fff' }}>
      <div style={{ height: 42, lineHeight: '42px', paddingLeft: 12, fontWeight: 600, fontSize: 16, color: '#409eff', borderBottom: '1px solid #f0f0f0' }}>
        表单组件
      </div>
      <div style={{ padding: 8 }}>
        {groups.map((g) => (
          <div key={g.title}>
            <div style={{ fontSize: 14, color: '#222', margin: '6px 2px', fontWeight: 500 }}>{g.title}</div>
            <div style={{ display: 'flex', flexWrap: 'wrap', gap: 4 }}>
              {g.list.map((item) => (
                <div key={item.__config__.tagIcon}
                  onClick={() => onAdd(item)}
                  style={{
                    width: '48%', padding: '8px 10px', background: '#f6f7ff', fontSize: 12,
                    cursor: 'pointer', border: '1px dashed #f6f7ff', borderRadius: 3,
                    display: 'flex', alignItems: 'center', gap: 6,
                  }}
                  onMouseEnter={(e) => { e.currentTarget.style.borderColor = '#787be8'; e.currentTarget.style.color = '#787be8'; }}
                  onMouseLeave={(e) => { e.currentTarget.style.borderColor = '#f6f7ff'; e.currentTarget.style.color = 'inherit'; }}
                >
                  {componentIconMap[item.__config__.tagIcon] || <FontSizeOutlined />}
                  {item.__config__.label}
                </div>
              ))}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

// ===================== 中间面板：可排序的表单项 =====================
const SortableField: React.FC<{
  field: any; isActive: boolean;
  onClick: () => void; onCopy: () => void; onDelete: () => void;
}> = ({ field, isActive, onClick, onCopy, onDelete }) => {
  const { attributes, listeners, setNodeRef, transform, transition } = useSortable({ id: String(field.__config__.formId) });
  const style: React.CSSProperties = {
    transform: CSS.Transform.toString(transform), transition,
    position: 'relative', cursor: 'move', marginBottom: 4,
    padding: '12px 10px', borderRadius: 6,
    background: isActive ? '#f6f7ff' : 'transparent',
    border: isActive ? '1px dashed #409eff' : '1px dashed transparent',
  };
  const config = field.__config__;
  const tag = config.tag;
  const isTextarea = tag === 'el-input' && (field.type === 'textarea' || field.autosize);

  const renderPreview = () => {
    if (tag === 'el-input' && !isTextarea) return <Input size="small" placeholder={field.placeholder} disabled style={{ width: '95%' }} />;
    if (isTextarea) return <Input.TextArea size="small" placeholder={field.placeholder} disabled rows={2} style={{ width: '95%' }} />;
    if (tag === 'el-input-number') return <InputNumber size="small" disabled style={{ width: '95%' }} />;
    if (tag === 'el-select') return <Select size="small" placeholder={field.placeholder || '请选择'} disabled style={{ width: '100%' }} options={field.__slot__?.options?.map((o: any) => ({ label: o.label, value: o.value }))} />;
    if (tag === 'el-radio-group') return <Radio.Group disabled>{field.__slot__?.options?.map((o: any) => <Radio key={String(o.value)} value={o.value}>{o.label}</Radio>)}</Radio.Group>;
    if (tag === 'el-checkbox-group') return <Checkbox.Group disabled options={field.__slot__?.options?.map((o: any) => ({ label: o.label, value: o.value }))} />;
    if (tag === 'el-switch') return <Switch disabled />;
    if (tag === 'self-upload') return <Button size="small" icon={<UploadOutlined />} disabled>点击上传</Button>;
    return <Input size="small" placeholder={field.placeholder} disabled />;
  };

  return (
    <div ref={setNodeRef} style={style} {...attributes} onClick={(e) => { e.stopPropagation(); onClick(); }}>
      <div style={{ display: 'flex', alignItems: 'flex-start' }}>
        <div {...listeners} style={{ cursor: 'move', marginRight: 8, marginTop: 4, color: '#999' }}><HolderOutlined /></div>
        <div style={{ flex: 1 }}>
          <div style={{ marginBottom: 4 }}>
            <span style={{ fontSize: 13 }}>{config.label}</span>
            {config.required && <span style={{ color: '#ff4d4f', marginLeft: 4 }}>*</span>}
          </div>
          {renderPreview()}
        </div>
      </div>
      {isActive && (
        <div style={{ position: 'absolute', top: -10, right: 24, display: 'flex', gap: 4 }}>
          <Tooltip title="复制"><span onClick={(e) => { e.stopPropagation(); onCopy(); }}
            style={{ width: 22, height: 22, lineHeight: '22px', textAlign: 'center', borderRadius: '50%', border: '1px solid #409eff', color: '#409eff', background: '#fff', cursor: 'pointer', fontSize: 12 }}>
            <CopyOutlined /></span></Tooltip>
          <Tooltip title="删除"><span onClick={(e) => { e.stopPropagation(); onDelete(); }}
            style={{ width: 22, height: 22, lineHeight: '22px', textAlign: 'center', borderRadius: '50%', border: '1px solid #f56c6c', color: '#f56c6c', background: '#fff', cursor: 'pointer', fontSize: 12 }}>
            <DeleteOutlined /></span></Tooltip>
        </div>
      )}
    </div>
  );
};

const CenterPanel: React.FC<{
  drawingList: any[]; activeId: number | null; formConf: any;
  onSelect: (item: any) => void; onCopy: (item: any) => void;
  onDelete: (id: number) => void; onReorder: (list: any[]) => void;
}> = ({ drawingList, activeId, formConf, onSelect, onCopy, onDelete, onReorder }) => {
  const sensors = useSensors(useSensor(PointerSensor, { activationConstraint: { distance: 5 } }));
  const handleDragEnd = (event: any) => {
    const { active, over } = event;
    if (active.id !== over?.id) {
      const oldIndex = drawingList.findIndex((f) => String(f.__config__.formId) === active.id);
      const newIndex = drawingList.findIndex((f) => String(f.__config__.formId) === over.id);
      onReorder(arrayMove(drawingList, oldIndex, newIndex));
    }
  };
  return (
    <div style={{ flex: 1, height: '100%', display: 'flex', flexDirection: 'column', borderRight: '1px solid #f0f0f0' }}>
      <div style={{ flex: 1, overflow: 'auto', padding: '12px 12px 15px' }}>
        <Form size={formConf.size === 'mini' ? 'small' : formConf.size === 'medium' ? 'middle' : 'small'}
          labelCol={{ style: { width: formConf.labelWidth } }}
          labelAlign={formConf.labelPosition === 'left' ? 'left' : 'right'}
          disabled={formConf.disabled}>
          <DndContext sensors={sensors} collisionDetection={closestCenter} onDragEnd={handleDragEnd}>
            <SortableContext items={drawingList.map((f) => String(f.__config__.formId))} strategy={verticalListSortingStrategy}>
              {drawingList.map((field) => (
                <SortableField key={field.__config__.formId} field={field}
                  isActive={activeId === field.__config__.formId}
                  onClick={() => onSelect(field)}
                  onCopy={() => onCopy(field)}
                  onDelete={() => onDelete(field.__config__.formId)} />
              ))}
            </SortableContext>
          </DndContext>
          {drawingList.length === 0 && (
            <div style={{ textAlign: 'center', fontSize: 18, color: '#ccb1ea', letterSpacing: 4, marginTop: '30%' }}>
              从左侧拖入或点选组件进行表单设计
            </div>
          )}
        </Form>
      </div>
    </div>
  );
};

// ===================== 右侧面板：组件属性 & 表单属性 =====================
const RightPanel: React.FC<{
  activeData: any; formConf: any;
  onFieldChange: (field: any) => void; onFormConfChange: (conf: any) => void;
}> = ({ activeData, formConf, onFieldChange, onFormConfChange }) => {
  const [tab, setTab] = useState('field');

  const updateConfig = (key: string, val: any) => {
    const next = JSON.parse(JSON.stringify(activeData));
    next.__config__[key] = val;
    onFieldChange(next);
  };
  const updateField = (key: string, val: any) => {
    const next = JSON.parse(JSON.stringify(activeData));
    next[key] = val;
    onFieldChange(next);
  };
  const updateFormConf = (key: string, val: any) => {
    onFormConfChange({ ...formConf, [key]: val });
  };

  // Options editor for select / radio / checkbox
  const renderOptionsEditor = () => {
    if (!activeData || !hasOptions(activeData.__config__?.tag)) return null;
    const options: any[] = activeData.__slot__?.options || [];
    const setOptions = (newOpts: any[]) => {
      const next = JSON.parse(JSON.stringify(activeData));
      if (!next.__slot__) next.__slot__ = {};
      next.__slot__.options = newOpts;
      onFieldChange(next);
    };
    return (
      <div style={{ marginTop: 8 }}>
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 6 }}>
          <span style={{ fontWeight: 500, fontSize: 13 }}>选项列表</span>
          <Button size="small" type="link" onClick={() => setOptions([...options, { label: `选项${options.length + 1}`, value: options.length + 1 }])}>
            <PlusOutlined /> 添加
          </Button>
        </div>
        {options.map((opt, idx) => (
          <div key={idx} style={{ display: 'flex', gap: 4, marginBottom: 4, alignItems: 'center' }}>
            <Input size="small" value={opt.label} placeholder="标签"
              onChange={(e) => { const n = [...options]; n[idx] = { ...n[idx], label: e.target.value }; setOptions(n); }} style={{ flex: 1 }} />
            <InputNumber size="small" value={opt.value} placeholder="值"
              onChange={(v) => { const n = [...options]; n[idx] = { ...n[idx], value: v }; setOptions(n); }} style={{ width: 70 }} />
            <DeleteOutlined style={{ color: '#ff4d4f', cursor: 'pointer', fontSize: 13 }}
              onClick={() => { const n = options.filter((_, i) => i !== idx); setOptions(n); }} />
          </div>
        ))}
      </div>
    );
  };

  // 组件属性 tab
  const fieldTab = activeData ? (
    <div style={{ padding: '8px 12px' }}>
      <Form size="small" layout="vertical">
        <Form.Item label="组件类型">
          <Input value={activeData.__config__?.tag} disabled size="small" />
        </Form.Item>
        <Form.Item label="字段名">
          <Input value={activeData.__vModel__ || ''} size="small"
            onChange={(e) => updateField('__vModel__', e.target.value)} />
        </Form.Item>
        <Form.Item label="标题">
          <Input value={activeData.__config__?.label || ''} size="small"
            onChange={(e) => updateConfig('label', e.target.value)} />
        </Form.Item>
        {activeData.placeholder !== undefined && (
          <Form.Item label="占位提示">
            <Input value={activeData.placeholder || ''} size="small"
              onChange={(e) => updateField('placeholder', e.target.value)} />
          </Form.Item>
        )}
        <Form.Item label="表单栅格">
          <Slider min={1} max={24} value={activeData.__config__?.span || 24}
            onChange={(v) => updateConfig('span', v)} />
        </Form.Item>
        <Form.Item label="是否必填">
          <Switch checked={!!activeData.__config__?.required}
            onChange={(v) => updateConfig('required', v)} />
        </Form.Item>
        <Form.Item label="是否禁用">
          <Switch checked={!!activeData.disabled}
            onChange={(v) => updateField('disabled', v)} />
        </Form.Item>
        {activeData.__config__?.tag === 'el-input' && (
          <Form.Item label="最大长度">
            <InputNumber value={activeData.maxlength} style={{ width: '100%' }}
              onChange={(v) => updateField('maxlength', v)} />
          </Form.Item>
        )}
        {activeData.__config__?.tag === 'el-input-number' && (
          <>
            <Form.Item label="最小值">
              <InputNumber value={activeData.min} style={{ width: '100%' }}
                onChange={(v) => updateField('min', v)} />
            </Form.Item>
            <Form.Item label="最大值">
              <InputNumber value={activeData.max} style={{ width: '100%' }}
                onChange={(v) => updateField('max', v)} />
            </Form.Item>
            <Form.Item label="步长">
              <InputNumber value={activeData.step} style={{ width: '100%' }}
                onChange={(v) => updateField('step', v)} />
            </Form.Item>
          </>
        )}
        <Form.Item label="开启提示">
          <Switch checked={!!activeData.__config__?.tips}
            onChange={(v) => updateConfig('tips', v)} />
        </Form.Item>
        {activeData.__config__?.tips && (
          <Form.Item label="提示说明">
            <Input.TextArea value={activeData.__config__?.tipsDesc || ''} rows={2}
              onChange={(e) => updateConfig('tipsDesc', e.target.value)} />
          </Form.Item>
        )}
        {renderOptionsEditor()}
      </Form>
    </div>
  ) : (
    <div style={{ textAlign: 'center', color: '#999', marginTop: 60 }}>请选择组件</div>
  );

  // 表单属性 tab
  const formTab = (
    <div style={{ padding: '8px 12px' }}>
      <Form size="small" layout="vertical">
        <Form.Item label="表单名">
          <Input value={formConf.formRef || ''} onChange={(e) => updateFormConf('formRef', e.target.value)} />
        </Form.Item>
        <Form.Item label="表单模型">
          <Input value={formConf.formModel || ''} onChange={(e) => updateFormConf('formModel', e.target.value)} />
        </Form.Item>
        <Form.Item label="校验模型">
          <Input value={formConf.formRules || ''} onChange={(e) => updateFormConf('formRules', e.target.value)} />
        </Form.Item>
        <Form.Item label="表单尺寸">
          <Radio.Group value={formConf.size || 'medium'} onChange={(e) => updateFormConf('size', e.target.value)}>
            <Radio.Button value="medium">中等</Radio.Button>
            <Radio.Button value="small">小</Radio.Button>
            <Radio.Button value="mini">迷你</Radio.Button>
          </Radio.Group>
        </Form.Item>
        <Form.Item label="标签对齐">
          <Radio.Group value={formConf.labelPosition || 'right'} onChange={(e) => updateFormConf('labelPosition', e.target.value)}>
            <Radio.Button value="left">左对齐</Radio.Button>
            <Radio.Button value="right">右对齐</Radio.Button>
            <Radio.Button value="top">顶部对齐</Radio.Button>
          </Radio.Group>
        </Form.Item>
        <Form.Item label="标签宽度">
          <InputNumber value={formConf.labelWidth || 100} min={0} max={200} style={{ width: '100%' }}
            onChange={(v) => updateFormConf('labelWidth', v)} />
        </Form.Item>
        <Form.Item label="栅格间隔">
          <InputNumber value={formConf.gutter || 15} min={0} style={{ width: '100%' }}
            onChange={(v) => updateFormConf('gutter', v)} />
        </Form.Item>
        <Form.Item label="禁用表单">
          <Switch checked={!!formConf.disabled} onChange={(v) => updateFormConf('disabled', v)} />
        </Form.Item>
        <Form.Item label="表单按钮">
          <Switch checked={formConf.formBtns !== false} onChange={(v) => updateFormConf('formBtns', v)} />
        </Form.Item>
      </Form>
    </div>
  );

  return (
    <div style={{ width: 350, borderLeft: '1px solid #f0f0f0', height: '100%', display: 'flex', flexDirection: 'column', background: '#fff' }}>
      <Tabs activeKey={tab} onChange={setTab} centered size="small"
        items={[
          { key: 'field', label: '组件属性', children: <div style={{ height: 'calc(100% - 46px)', overflow: 'auto' }}>{fieldTab}</div> },
          { key: 'form', label: '表单属性', children: <div style={{ height: 'calc(100% - 46px)', overflow: 'auto' }}>{formTab}</div> },
        ]}
        style={{ height: '100%' }}
      />
    </div>
  );
};

// ===================== 表单构建器主组件 =====================
const FormBuilder: React.FC<{
  editId?: number | null;
  onSave: () => void;
  onCancel: () => void;
  initialData?: { name: string; info: string; content: string };
}> = ({ editId, onSave, onCancel, initialData }) => {
  const [drawingList, setDrawingList] = useState<any[]>([]);
  const [activeId, setActiveId] = useState<number | null>(null);
  const [formConf, setFormConf] = useState<any>({ ...defaultFormConf });
  const [selfForm, setSelfForm] = useState({ name: '', info: '' });
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    if (initialData) {
      setSelfForm({ name: initialData.name || '', info: initialData.info || '' });
      try {
        const parsed = JSON.parse(initialData.content || '{}');
        const fields = parsed.fields || [];
        setDrawingList(fields);
        if (fields.length > 0) setActiveId(fields[0].__config__?.formId ?? null);
        const { fields: _, ...conf } = parsed;
        setFormConf({ ...defaultFormConf, ...conf });
        if (fields.length > 0) {
          const maxId = Math.max(...fields.map((f: any) => f.__config__?.formId || 0));
          if (maxId >= idGlobal) idGlobal = maxId + 1;
        }
      } catch { /* */ }
    }
  }, [initialData]);

  const activeData = useMemo(
    () => drawingList.find((f) => f.__config__?.formId === activeId) || null,
    [drawingList, activeId],
  );

  const handleAdd = useCallback((item: any) => {
    const clone = cloneComponent(item);
    setDrawingList((prev) => [...prev, clone]);
    setActiveId(clone.__config__.formId);
  }, []);

  const handleCopy = useCallback((item: any) => {
    const clone = cloneComponent(item);
    const idx = drawingList.findIndex((f) => f.__config__.formId === item.__config__.formId);
    const next = [...drawingList];
    next.splice(idx + 1, 0, clone);
    setDrawingList(next);
    setActiveId(clone.__config__.formId);
  }, [drawingList]);

  const handleDelete = useCallback((id: number) => {
    setDrawingList((prev) => {
      const next = prev.filter((f) => f.__config__.formId !== id);
      if (activeId === id) setActiveId(next.length > 0 ? next[0].__config__.formId : null);
      return next;
    });
  }, [activeId]);

  const handleFieldChange = useCallback((updated: any) => {
    setDrawingList((prev) =>
      prev.map((f) => (f.__config__.formId === updated.__config__.formId ? updated : f)),
    );
  }, []);

  const handleSaveJSON = async () => {
    if (!selfForm.name) { message.warning('请输入表单名称'); return; }
    setSaving(true);
    try {
      const content = JSON.stringify({ ...formConf, fields: drawingList });
      if (editId) {
        await getFormConfigEdit({ id: editId, name: selfForm.name, info: selfForm.info, content });
        message.success('编辑成功');
      } else {
        await getFormConfigSave({ name: selfForm.name, info: selfForm.info, content });
        message.success('保存成功');
      }
      onSave();
    } catch { message.error('保存失败'); }
    finally { setSaving(false); }
  };

  return (
    <div style={{ display: 'flex', flexDirection: 'column', height: '100%' }}>
      {/* 顶部操作栏 */}
      <div style={{
        height: 48, display: 'flex', alignItems: 'center', justifyContent: 'space-between',
        padding: '0 16px', borderBottom: '1px solid #f0f0f0', background: '#fff', flexShrink: 0,
      }}>
        <Space>
          <Input placeholder="表单名称" value={selfForm.name} style={{ width: 200 }}
            onChange={(e) => setSelfForm((p) => ({ ...p, name: e.target.value }))} />
          <Input placeholder="表单说明" value={selfForm.info} style={{ width: 200 }}
            onChange={(e) => setSelfForm((p) => ({ ...p, info: e.target.value }))} />
        </Space>
        <Space>
          <Button onClick={onCancel}>取消</Button>
          <Button type="primary" loading={saving} onClick={handleSaveJSON}>保存</Button>
        </Space>
      </div>
      {/* 三栏布局 */}
      <div style={{ flex: 1, display: 'flex', overflow: 'hidden' }}>
        <LeftPanel onAdd={handleAdd} />
        <CenterPanel drawingList={drawingList} activeId={activeId} formConf={formConf}
          onSelect={(item) => setActiveId(item.__config__.formId)}
          onCopy={handleCopy} onDelete={handleDelete} onReorder={setDrawingList} />
        <RightPanel activeData={activeData} formConf={formConf}
          onFieldChange={handleFieldChange} onFormConfChange={setFormConf} />
      </div>
    </div>
  );
};

// ===================== 列表页面 =====================
const MaintainFormConfig: React.FC = () => {
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [keywords, setKeywords] = useState('');
  const [pagination, setPagination] = useState({ current: 1, pageSize: 20, total: 0 });
  const [builderVisible, setBuilderVisible] = useState(false);
  const [editId, setEditId] = useState<number | null>(null);
  const [initialData, setInitialData] = useState<any>(undefined);

  const fetchList = useCallback(async (page = 1) => {
    setLoading(true);
    try {
      const res: any = await getFormConfigList({ page, limit: pagination.pageSize, keywords: keywords || undefined });
      setList(res?.list || []);
      setPagination((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch { message.error('获取列表失败'); }
    finally { setLoading(false); }
  }, [keywords, pagination.pageSize]);

  useEffect(() => { fetchList(1); }, []);

  const handleCreate = () => {
    setEditId(null);
    setInitialData(undefined);
    setBuilderVisible(true);
  };

  const handleEdit = async (record: any) => {
    try {
      const res: any = await getFormConfigInfo({ id: record.id });
      setEditId(record.id);
      setInitialData({ name: res?.name || record.name, info: res?.info || record.info, content: res?.content || '' });
      setBuilderVisible(true);
    } catch { message.error('获取表单详情失败'); }
  };

  const columns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 60 },
    { title: '表单名称', dataIndex: 'name', width: 200 },
    { title: '表单说明', dataIndex: 'info', ellipsis: true },
    {
      title: '操作', width: 120,
      render: (_: any, record: any) => (
        <Space size="small">
          <a onClick={() => handleEdit(record)}>编辑</a>
        </Space>
      ),
    },
  ];

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
      <Card bodyStyle={{ paddingBottom: 0 }}>
        <Space style={{ marginBottom: 16 }}>
          <Input placeholder="表单名称" value={keywords} onChange={(e) => setKeywords(e.target.value)}
            onPressEnter={() => fetchList(1)} allowClear prefix={<SearchOutlined />} style={{ width: 240 }} />
          <Button type="primary" onClick={() => fetchList(1)}>搜索</Button>
          <Button icon={<ReloadOutlined />} onClick={() => { setKeywords(''); fetchList(1); }}>重置</Button>
        </Space>
      </Card>
      <Card>
        <div style={{ marginBottom: 16 }}>
          <Button type="primary" icon={<PlusOutlined />} onClick={handleCreate}>创建表单</Button>
        </div>
        <Table rowKey="id" columns={columns} dataSource={list} loading={loading} size="small"
          pagination={{
            ...pagination, showSizeChanger: true, showTotal: (t) => `共 ${t} 条`,
            onChange: (p, ps) => { setPagination((prev) => ({ ...prev, pageSize: ps })); fetchList(p); },
          }} />
      </Card>

      {/* 全屏表单构建器弹窗 */}
      <Modal open={builderVisible} footer={null} closable={false} destroyOnClose
        width="100vw"
        styles={{ body: { height: 'calc(100vh - 48px)', padding: 0, overflow: 'hidden' } }}
        style={{ top: 0, maxWidth: '100vw', margin: 0, padding: 0 }}>
        <FormBuilder editId={editId} initialData={initialData}
          onCancel={() => setBuilderVisible(false)}
          onSave={() => { setBuilderVisible(false); fetchList(pagination.current); }} />
      </Modal>
    </div>
  );
};

export default MaintainFormConfig;

