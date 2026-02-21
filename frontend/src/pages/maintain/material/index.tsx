import React, { useState, useEffect, useCallback } from 'react';
import { Card, Tabs, Tree, Button, Upload, Space, message, Popconfirm, Empty, Spin, Typography, Modal, Form, Input, InputNumber, Dropdown, Pagination, Grid, Drawer } from 'antd';
import { PlusOutlined, DeleteOutlined, ReloadOutlined, FolderOutlined, MoreOutlined, PlayCircleOutlined, MenuOutlined } from '@ant-design/icons';
import type { DataNode } from 'antd/es/tree';
import { fileListApi, fileDeleteApi, fileImageApi } from '@/api/systemSetting';
import { treeCategroy, addCategroy, updateCategroy, deleteCategroy } from '@/api/categoryApi';
import { getToken } from '@/utils/auth';

const { useBreakpoint } = Grid;

const IMAGE_TYPES = 'jpg,jpeg,gif,png,bmp,PNG,JPG';
const VIDEO_TYPES = 'video/mp4';
// Category type for attachments: 1=image, 2=video (in Vue: type=1 for image category)
const CATEGORY_TYPE = 1;

const MaintainMaterial: React.FC = () => {
  const screens = useBreakpoint();
  const isMobile = !screens.md;
  const [activeTab, setActiveTab] = useState<'pic' | 'video'>('pic');
  const [fileList, setFileList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [pagination, setPagination] = useState({ current: 1, pageSize: 18, total: 0 });
  const [categoryTree, setCategoryTree] = useState<DataNode[]>([]);
  const [selectedPid, setSelectedPid] = useState<number>(0);
  const [catModalVisible, setCatModalVisible] = useState(false);
  const [catEditId, setCatEditId] = useState<number | null>(null);
  const [catForm] = Form.useForm();
  const [previewVideo, setPreviewVideo] = useState('');
  const [previewImage, setPreviewImage] = useState('');
  const [drawerVisible, setDrawerVisible] = useState(false);

  const attType = activeTab === 'pic' ? IMAGE_TYPES : VIDEO_TYPES;

  // Fetch category tree
  const fetchCategories = useCallback(async () => {
    try {
      const res: any = await treeCategroy({ type: CATEGORY_TYPE, status: 1 });
      const tree = Array.isArray(res) ? res : [];
      const nodes: DataNode[] = [
        {
          key: 0, title: activeTab === 'pic' ? '全部图片' : '全部视频', icon: <FolderOutlined />,
          children: tree.map((c: any) => convertTreeNode(c)),
        },
      ];
      setCategoryTree(nodes);
    } catch { /* */ }
  }, [activeTab]);

  const convertTreeNode = (node: any): DataNode => ({
    key: node.id,
    title: (
      <span style={{ display: 'inline-flex', alignItems: 'center', gap: 4 }}>
        {node.name}
        <Dropdown menu={{ items: [
          { key: 'add', label: '添加子分类', onClick: () => openCatModal(node.id) },
          { key: 'edit', label: '编辑', onClick: () => openCatEdit(node) },
          { key: 'del', label: <span style={{ color: '#ff4d4f' }}>删除</span>, onClick: () => handleDeleteCat(node.id) },
        ]}} trigger={['click']}>
          <MoreOutlined style={{ cursor: 'pointer', fontSize: 12 }} onClick={(e) => e.stopPropagation()} />
        </Dropdown>
      </span>
    ),
    children: node.child?.map((c: any) => convertTreeNode(c)) || [],
  });

  // Fetch file list
  const fetchFiles = useCallback(async (page = 1) => {
    setLoading(true);
    try {
      const params: any = { page, limit: pagination.pageSize, attType };
      if (selectedPid > 0) params.pid = selectedPid;
      const res = await fileListApi(params);
      setFileList(res?.list || []);
      setPagination((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch { message.error('获取素材列表失败'); }
    finally { setLoading(false); }
  }, [pagination.pageSize, attType, selectedPid]);

  useEffect(() => { fetchCategories(); }, [fetchCategories]);
  useEffect(() => { fetchFiles(1); }, [attType, selectedPid]);

  const handleTabChange = (key: string) => {
    setActiveTab(key as 'pic' | 'video');
    setSelectedPid(0);
  };

  const handleDelete = async (id: number) => {
    try { await fileDeleteApi(id); message.success('删除成功'); fetchFiles(pagination.current); } catch { /* */ }
  };

  // Category CRUD
  const openCatModal = (pid: number) => {
    setCatEditId(null);
    catForm.setFieldsValue({ name: '', sort: 0, pid });
    setCatModalVisible(true);
  };

  const openCatEdit = (node: any) => {
    setCatEditId(node.id);
    catForm.setFieldsValue({ name: node.name, sort: node.sort || 0, pid: node.pid || 0 });
    setCatModalVisible(true);
  };

  const handleDeleteCat = (id: number) => {
    Modal.confirm({
      title: '确定删除该分类？', onOk: async () => {
        try { await deleteCategroy({ id }); message.success('删除成功'); fetchCategories(); } catch { /* */ }
      },
    });
  };

  const handleCatOk = async () => {
    try {
      const values = await catForm.validateFields();
      const params = { ...values, type: CATEGORY_TYPE, status: 1 };
      if (catEditId) { await updateCategroy({ ...params, id: catEditId }); message.success('编辑成功'); }
      else { await addCategroy(params); message.success('添加成功'); }
      setCatModalVisible(false);
      fetchCategories();
    } catch { /* */ }
  };

  // Upload handler for video (uses FormData)
  const handleVideoUpload = async (options: any) => {
    const formData = new FormData();
    formData.append('multipart', options.file);
    try {
      await fileImageApi(formData, { model: 'product', pid: selectedPid || 0 });
      message.success('上传成功');
      fetchFiles(pagination.current);
    } catch { message.error('上传失败'); }
  };

  const uploadProps: any = {
    name: 'multipart',
    action: '/api/admin/upload/image',
    headers: { 'Authori-zation': getToken() || '' },
    data: { model: 'product', pid: selectedPid || 0 },
    showUploadList: false,
    onChange(info: any) {
      if (info.file.status === 'done') { message.success('上传成功'); fetchFiles(pagination.current); }
      else if (info.file.status === 'error') { message.error('上传失败'); }
    },
  };

  const isVideo = (file: any) => file.attType === 'video/mp4' || file.sattDir?.endsWith('.mp4');

  const renderFileItem = (file: any) => {
    const url = file.attDir || file.sattDir || file.imageUrl;
    const name = file.name || file.realName || '未命名';
    const id = file.attId || file.id;

    return (
      <Card key={id} hoverable bodyStyle={{ padding: 8, textAlign: 'center' }}
        style={{ overflow: 'hidden' }}
        cover={
          isVideo(file) ? (
            <div style={{ height: 120, display: 'flex', alignItems: 'center', justifyContent: 'center', background: '#000', cursor: 'pointer', position: 'relative' }}
              onClick={() => setPreviewVideo(url)}>
              <video src={url} style={{ maxHeight: 120, maxWidth: '100%' }} />
              <PlayCircleOutlined style={{ position: 'absolute', fontSize: 32, color: 'rgba(255,255,255,0.8)' }} />
            </div>
          ) : (
            <img src={url} alt={name} style={{ height: 120, objectFit: 'cover', width: '100%', cursor: 'pointer' }}
              onClick={() => setPreviewImage(url)} />
          )
        }
        actions={[
          <Popconfirm key="del" title="确定删除?" onConfirm={() => handleDelete(id)}>
            <DeleteOutlined style={{ color: '#ff4d4f' }} />
          </Popconfirm>,
        ]}
      >
        <Typography.Text ellipsis style={{ fontSize: 12 }}>{name}</Typography.Text>
      </Card>
    );
  };

  const categoryContent = (
    <>
      <Button type="primary" size="small" block icon={<PlusOutlined />} style={{ marginBottom: 8 }}
        onClick={() => openCatModal(0)}>添加分类</Button>
      <Tree treeData={categoryTree} defaultExpandAll selectedKeys={[selectedPid]}
        onSelect={(keys) => {
          setSelectedPid(keys.length ? Number(keys[0]) : 0);
          if (isMobile) setDrawerVisible(false);
        }}
        style={{ fontSize: 13 }} />
    </>
  );

  return (
    <div style={{ display: 'flex', gap: isMobile ? 0 : 16, flexDirection: isMobile ? 'column' : 'row' }}>
      {/* Desktop: Category sidebar */}
      {!isMobile && (
        <Card style={{ width: 220, flexShrink: 0 }} bodyStyle={{ padding: 8 }}>
          {categoryContent}
        </Card>
      )}

      {/* Mobile: Category drawer */}
      {isMobile && (
        <Drawer title="素材分类" placement="left" open={drawerVisible}
          onClose={() => setDrawerVisible(false)} width={260} bodyStyle={{ padding: 8 }}>
          {categoryContent}
        </Drawer>
      )}

      {/* Right: Content area */}
      <Card style={{ flex: 1 }} bodyStyle={{ padding: isMobile ? '8px 10px' : '12px 16px' }}>
        <Tabs activeKey={activeTab} onChange={handleTabChange}
          items={[{ key: 'pic', label: '图片' }, { key: 'video', label: '视频' }]} />

        <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: 12, flexWrap: 'wrap', gap: 8 }}>
          <Space>
            {isMobile && (
              <Button icon={<MenuOutlined />} onClick={() => setDrawerVisible(true)}>分类</Button>
            )}
            {activeTab === 'pic' ? (
              <Upload {...uploadProps} accept="image/*">
                <Button type="primary" icon={<PlusOutlined />}>{isMobile ? '上传' : '上传图片'}</Button>
              </Upload>
            ) : (
              <Upload customRequest={handleVideoUpload} showUploadList={false} accept="video/mp4">
                <Button type="primary" icon={<PlusOutlined />}>{isMobile ? '上传' : '上传视频'}</Button>
              </Upload>
            )}
          </Space>
          <Button icon={<ReloadOutlined />} onClick={() => fetchFiles(pagination.current)}>{isMobile ? '' : '刷新'}</Button>
        </div>

        <Spin spinning={loading}>
          {fileList.length === 0 ? (
            <Empty description="素材为空" style={{ marginTop: 60 }} />
          ) : (
            <div style={{ display: 'grid', gridTemplateColumns: isMobile ? 'repeat(auto-fill, minmax(100px, 1fr))' : 'repeat(auto-fill, minmax(140px, 1fr))', gap: isMobile ? 8 : 12 }}>
              {fileList.map(renderFileItem)}
            </div>
          )}
          {pagination.total > pagination.pageSize && (
            <div style={{ textAlign: 'center', marginTop: 16 }}>
              <Pagination current={pagination.current} pageSize={pagination.pageSize} total={pagination.total}
                showSizeChanger={false} simple={isMobile} onChange={(p) => fetchFiles(p)} />
            </div>
          )}
        </Spin>
      </Card>

      {/* Category add/edit modal */}
      <Modal title={catEditId ? '编辑分类' : '添加分类'} open={catModalVisible}
        onOk={handleCatOk} onCancel={() => setCatModalVisible(false)} destroyOnClose width={isMobile ? '90%' : 400}>
        <Form form={catForm} labelCol={{ span: 5 }} wrapperCol={{ span: 17 }}>
          <Form.Item label="分类名称" name="name" rules={[{ required: true, message: '请输入分类名称' }]}>
            <Input placeholder="请输入分类名称" />
          </Form.Item>
          <Form.Item label="排序" name="sort"><InputNumber min={0} style={{ width: '100%' }} /></Form.Item>
          <Form.Item name="pid" hidden><Input /></Form.Item>
        </Form>
      </Modal>

      {/* Video preview modal */}
      <Modal title="视频预览" open={!!previewVideo} onCancel={() => setPreviewVideo('')} footer={null}
        destroyOnClose width={isMobile ? '95%' : 640}>
        <video src={previewVideo} controls autoPlay style={{ width: '100%' }} />
      </Modal>

      {/* Image preview modal */}
      <Modal open={!!previewImage} onCancel={() => setPreviewImage('')} footer={null}
        destroyOnClose width={isMobile ? '95%' : 640} centered>
        <img src={previewImage} alt="预览" style={{ width: '100%' }} />
      </Modal>
    </div>
  );
};

export default MaintainMaterial;
