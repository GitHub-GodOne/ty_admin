import React, { useState, useEffect } from 'react';
import { Card, Form, Input, Button, InputNumber, Switch, Select, message, Spin, Space } from 'antd';
import { useNavigate, useSearchParams } from 'react-router-dom';
import { PlusOutlined, DeleteOutlined } from '@ant-design/icons';
import MDEditor from '@uiw/react-md-editor';
import { AddArticle as articleSaveApi, InfoArticle as articleInfoApi, UpdateArticle as articleEditApi } from '@/api/article';
import { treeCategroy } from '@/api/categoryApi';
import MaterialPicker from '@/components/MaterialPicker';

const { TextArea } = Input;

const ArticleCreate: React.FC = () => {
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const editId = searchParams.get('id');
  const isEdit = !!editId;

  const [form] = Form.useForm();
  const [loading, setLoading] = useState(false);
  const [submitLoading, setSubmitLoading] = useState(false);
  const [content, setContent] = useState('');
  const [coverUrl, setCoverUrl] = useState('');
  const [pickerOpen, setPickerOpen] = useState(false);
  const [categoryList, setCategoryList] = useState<{ label: string; value: string }[]>([]);

  // 加载分类列表 (type=3 为文章分类)
  useEffect(() => {
    treeCategroy({ type: 3, status: 1 }).then((res: any) => {
      const list = (res || []).map((item: any) => ({ label: item.name, value: String(item.id) }));
      setCategoryList(list);
    }).catch(() => {});
  }, []);

  // 编辑模式加载文章详情
  useEffect(() => {
    if (!isEdit) return;
    setLoading(true);
    articleInfoApi(editId)
      .then((res: any) => {
        form.setFieldsValue({
          title: res?.title,
          author: res?.author,
          cid: res?.cid ? String(res.cid) : undefined,
          synopsis: res?.synopsis,
          shareTitle: res?.shareTitle,
          shareSynopsis: res?.shareSynopsis,
          sort: res?.sort ?? 0,
          status: res?.status === 1 || res?.status === true,
          isHot: res?.isHot === 1 || res?.isHot === true,
          isBanner: res?.isBanner === 1 || res?.isBanner === true,
        });
        setContent(res?.content || '');
        setCoverUrl(res?.imageInput || '');
      })
      .catch(() => message.error('获取文章详情失败'))
      .finally(() => setLoading(false));
  }, [editId, isEdit, form]);

  const handleSubmit = async (values: any) => {
    if (!coverUrl) { message.warning('请选择封面图'); return; }
    if (!content) { message.warning('请输入文章内容'); return; }
    setSubmitLoading(true);
    try {
      const params = {
        ...values,
        imageInput: coverUrl,
        content,
        status: values.status ? 1 : 0,
        isHot: values.isHot ? true : false,
        isBanner: values.isBanner ? true : false,
      };
      if (isEdit) {
        await articleEditApi({ ...params, id: editId });
        message.success('编辑成功');
      } else {
        await articleSaveApi(params);
        message.success('添加成功');
      }
      navigate('/content/article');
    } catch { /* handled by interceptor */ }
    finally { setSubmitLoading(false); }
  };

  return (
    <Spin spinning={loading}>
      <Card title={isEdit ? '编辑文章' : '创建文章'}>
        <Form form={form} labelCol={{ span: 3 }} wrapperCol={{ span: 18 }}
          onFinish={handleSubmit} initialValues={{ sort: 0, status: true, isHot: false, isBanner: false }}>

          <Form.Item label="标题" name="title" rules={[{ required: true, message: '请输入标题' }]}>
            <Input placeholder="请输入文章标题" maxLength={200} />
          </Form.Item>

          <Form.Item label="作者" name="author" rules={[{ required: true, message: '请输入作者' }]}>
            <Input placeholder="请输入作者" maxLength={50} />
          </Form.Item>

          <Form.Item label="文章分类" name="cid" rules={[{ required: true, message: '请选择分类' }]}>
            <Select placeholder="请选择文章分类" options={categoryList} allowClear style={{ width: 300 }} />
          </Form.Item>

          <Form.Item label="封面图" required>
            <Space align="start">
              {coverUrl ? (
                <div style={{ position: 'relative', width: 120, height: 120, border: '1px solid #d9d9d9', borderRadius: 6, overflow: 'hidden' }}>
                  <img src={coverUrl} alt="封面" style={{ width: '100%', height: '100%', objectFit: 'cover' }} />
                  <div style={{ position: 'absolute', top: 0, right: 0, display: 'flex', gap: 2 }}>
                    <Button type="link" size="small" danger icon={<DeleteOutlined />}
                      onClick={() => setCoverUrl('')} />
                  </div>
                </div>
              ) : (
                <div onClick={() => setPickerOpen(true)}
                  style={{ width: 120, height: 120, border: '1px dashed #d9d9d9', borderRadius: 6,
                    display: 'flex', alignItems: 'center', justifyContent: 'center', cursor: 'pointer',
                    flexDirection: 'column', color: '#999' }}>
                  <PlusOutlined style={{ fontSize: 24 }} />
                  <span style={{ fontSize: 12, marginTop: 4 }}>选择封面</span>
                </div>
              )}
              <Button size="small" onClick={() => setPickerOpen(true)}>从素材库选择</Button>
            </Space>
          </Form.Item>

          <Form.Item label="文章简介" name="synopsis" rules={[{ required: true, message: '请输入简介' }]}>
            <TextArea rows={3} placeholder="请输入文章简介" maxLength={200} showCount />
          </Form.Item>

          <Form.Item label="分享标题" name="shareTitle" rules={[{ required: true, message: '请输入分享标题' }]}>
            <Input placeholder="请输入分享标题" maxLength={200} />
          </Form.Item>

          <Form.Item label="分享简介" name="shareSynopsis" rules={[{ required: true, message: '请输入分享简介' }]}>
            <TextArea rows={2} placeholder="请输入分享简介" maxLength={200} showCount />
          </Form.Item>

          <Form.Item label="文章内容" required>
            <div data-color-mode="light">
              <MDEditor value={content} onChange={(val) => setContent(val || '')} height={500} />
            </div>
          </Form.Item>

          <Form.Item label="是否热门" name="isHot" valuePropName="checked">
            <Switch checkedChildren="是" unCheckedChildren="否" />
          </Form.Item>

          <Form.Item label="是否Banner" name="isBanner" valuePropName="checked">
            <Switch checkedChildren="是" unCheckedChildren="否" />
          </Form.Item>

          <Form.Item label="排序" name="sort">
            <InputNumber min={0} style={{ width: 200 }} />
          </Form.Item>

          <Form.Item label="状态" name="status" valuePropName="checked">
            <Switch checkedChildren="显示" unCheckedChildren="隐藏" />
          </Form.Item>

          <Form.Item wrapperCol={{ offset: 3 }}>
            <Button type="primary" htmlType="submit" loading={submitLoading} style={{ marginRight: 16 }}>
              {isEdit ? '保存修改' : '提交'}
            </Button>
            <Button onClick={() => navigate('/content/article')}>返回</Button>
          </Form.Item>
        </Form>
      </Card>

      <MaterialPicker open={pickerOpen} onCancel={() => setPickerOpen(false)}
        onOk={(urls) => { setCoverUrl(urls[0] || ''); setPickerOpen(false); }} />
    </Spin>
  );
};

export default ArticleCreate;
