import React, { useState, useEffect } from 'react';
import { Card, Form, Input, Radio, Select, Button, Upload, message } from 'antd';
import { UploadOutlined } from '@ant-design/icons';
import { useSearchParams, useNavigate, useLocation } from 'react-router-dom';
import { replyInfoApi, keywordsInfoApi, replySaveApi, replyUpdateApi } from '@/api/wxApi';

const typeOptions = [
  { value: 'text', label: '文字消息' },
  { value: 'image', label: '图片消息' },
  { value: 'news', label: '图文消息' },
  { value: 'voice', label: '声音消息' },
];

const FollowReply: React.FC = () => {
  const [searchParams] = useSearchParams();
  const navigate = useNavigate();
  const location = useLocation();
  const editId = searchParams.get('id');
  const [form] = Form.useForm();
  const [saving, setSaving] = useState(false);
  const [msgType, setMsgType] = useState('text');

  // Determine mode from route path
  const isFollowPage = location.pathname.includes('/follow');
  const isDefaultPage = location.pathname.includes('/replyIndex');

  useEffect(() => {
    if (editId) {
      // Editing existing keyword reply
      replyInfoApi({ id: editId }).then((res: any) => {
        if (res) {
          const data = typeof res.data === 'string' ? JSON.parse(res.data) : res.data;
          form.setFieldsValue({
            keywords: res.keywords || '',
            status: res.status ? 1 : 0,
            type: res.type || 'text',
            content: data?.content || '',
            mediaId: data?.mediaId || '',
            srcUrl: data?.srcUrl || '',
          });
          setMsgType(res.type || 'text');
        }
      }).catch(() => {});
    } else if (isFollowPage) {
      // Follow reply - load by keyword 'subscribe'
      keywordsInfoApi({ keywords: 'subscribe' }).then((res: any) => {
        if (res) {
          const data = typeof res.data === 'string' ? JSON.parse(res.data) : res.data;
          form.setFieldsValue({
            keywords: 'subscribe',
            status: res.status ? 1 : 0,
            type: res.type || 'text',
            content: data?.content || '',
            id: res.id,
          });
          setMsgType(res.type || 'text');
        }
      }).catch(() => {});
    } else if (isDefaultPage) {
      // Default reply - load by keyword 'default'
      keywordsInfoApi({ keywords: 'default' }).then((res: any) => {
        if (res) {
          const data = typeof res.data === 'string' ? JSON.parse(res.data) : res.data;
          form.setFieldsValue({
            keywords: 'default',
            status: res.status ? 1 : 0,
            type: res.type || 'text',
            content: data?.content || '',
            id: res.id,
          });
          setMsgType(res.type || 'text');
        }
      }).catch(() => {});
    }
  }, [editId, isFollowPage, isDefaultPage]);

  const handleSave = async () => {
    try {
      const values = await form.validateFields();
      setSaving(true);
      const payload = {
        keywords: values.keywords || (isFollowPage ? 'subscribe' : isDefaultPage ? 'default' : ''),
        status: values.status ?? 1,
        type: values.type || 'text',
        data: JSON.stringify({
          content: values.content || '',
          mediaId: values.mediaId || '',
          srcUrl: values.srcUrl || '',
        }),
      };
      const existingId = editId || form.getFieldValue('id');
      if (existingId) {
        await replyUpdateApi({ id: existingId }, payload);
      } else {
        await replySaveApi(payload);
      }
      message.success('保存成功');
      if (!isFollowPage && !isDefaultPage) {
        navigate('/appSetting/publicAccount/wxReply/keyword');
      }
    } catch { /* validation */ }
    finally { setSaving(false); }
  };

  const title = isFollowPage ? '微信关注回复' : isDefaultPage ? '无效关键词回复' : (editId ? '编辑关键字回复' : '添加关键字回复');

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
      <Card title={title}>
        <Form form={form} labelCol={{ span: 4 }} wrapperCol={{ span: 16 }}
          initialValues={{ status: 1, type: 'text' }} style={{ maxWidth: 700 }}>
          {!isFollowPage && !isDefaultPage && (
            <Form.Item label="关键字" name="keywords" rules={[{ required: true, message: '请输入关键字' }]}>
              <Input placeholder="请输入关键字，多个用逗号分隔" />
            </Form.Item>
          )}
          <Form.Item label="规则状态" name="status">
            <Radio.Group>
              <Radio value={1}>启用</Radio>
              <Radio value={0}>禁用</Radio>
            </Radio.Group>
          </Form.Item>
          <Form.Item label="消息类型" name="type">
            <Select options={typeOptions} onChange={(v) => { setMsgType(v); form.setFieldValue('type', v); }} />
          </Form.Item>
          {msgType === 'text' && (
            <Form.Item label="回复内容" name="content" rules={[{ required: true, message: '请输入回复内容' }]}>
              <Input.TextArea rows={4} placeholder="请输入文字回复内容" />
            </Form.Item>
          )}
          {msgType === 'image' && (
            <Form.Item label="图片地址" name="srcUrl" rules={[{ required: true, message: '请输入图片地址' }]}>
              <Input placeholder="请输入图片URL地址" />
            </Form.Item>
          )}
          {msgType === 'voice' && (
            <Form.Item label="语音MediaId" name="mediaId" rules={[{ required: true, message: '请输入语音MediaId' }]}>
              <Input placeholder="请输入微信语音素材MediaId" />
            </Form.Item>
          )}
          {msgType === 'news' && (
            <Form.Item label="图文内容" name="content" rules={[{ required: true, message: '请输入图文内容' }]}>
              <Input.TextArea rows={4} placeholder="请输入图文消息内容(JSON格式)" />
            </Form.Item>
          )}
          <Form.Item label="备注" name="mediaId" hidden><Input /></Form.Item>
          <Form.Item wrapperCol={{ offset: 4 }}>
            <Button type="primary" loading={saving} onClick={handleSave}>保存</Button>
            {!isFollowPage && !isDefaultPage && (
              <Button style={{ marginLeft: 12 }} onClick={() => navigate(-1)}>返回</Button>
            )}
          </Form.Item>
        </Form>
      </Card>
    </div>
  );
};

export default FollowReply;
