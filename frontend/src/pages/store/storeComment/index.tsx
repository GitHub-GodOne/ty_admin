import React, { useState, useEffect, useRef } from 'react';
import {
  Card, Table, Button, Space, Image, Rate, message, Popconfirm,
  Tag, Input, Select, DatePicker, Form, Modal, Row, Col, TreeSelect,
} from 'antd';
import { PlusOutlined, SearchOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import {
  replyListApi, replyDeleteApi, replyCreatApi, replyCommentApi,
  productLstApi, categoryApi,
} from '@/api/store';
import dayjs from 'dayjs';

const { RangePicker } = DatePicker;
const { TextArea } = Input;

const StoreComment: React.FC = () => {
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [pagination, setPagination] = useState({ current: 1, pageSize: 20, total: 0 });
  const [selectedRowKeys, setSelectedRowKeys] = useState<React.Key[]>([]);

  // 搜索条件 — 用 ref 避免闭包陈旧问题
  const [productSearch, setProductSearch] = useState('');
  const [isReply, setIsReply] = useState<string>('');
  const [nickname, setNickname] = useState('');
  const [dateRange, setDateRange] = useState<[dayjs.Dayjs, dayjs.Dayjs] | null>(null);
  const filterRef = useRef({ productSearch: '', isReply: '', nickname: '', dateRange: null as any });
  // 同步 ref
  filterRef.current = { productSearch, isReply, nickname, dateRange };

  // 添加虚拟评论
  const [addModalOpen, setAddModalOpen] = useState(false);
  const [addForm] = Form.useForm();

  // 商品选择弹窗
  const [productModalOpen, setProductModalOpen] = useState(false);
  const [productList, setProductList] = useState<any[]>([]);
  const [productLoading, setProductLoading] = useState(false);
  const [productPagination, setProductPagination] = useState({ current: 1, pageSize: 10, total: 0 });
  const [productKeyword, setProductKeyword] = useState('');
  const [selectedProduct, setSelectedProduct] = useState<any>(null);
  const [categoryTree, setCategoryTree] = useState<any[]>([]);
  const [productCateId, setProductCateId] = useState<string | undefined>(undefined);

  // 回复弹窗
  const [replyModalOpen, setReplyModalOpen] = useState(false);
  const [replyRecord, setReplyRecord] = useState<any>(null);
  const [replyContent, setReplyContent] = useState('');

  // 详情弹窗
  const [detailModalOpen, setDetailModalOpen] = useState(false);
  const [detailRecord, setDetailRecord] = useState<any>(null);

  const fetchList = async (page = 1, overrideFilters?: any) => {
    setLoading(true);
    try {
      const f = overrideFilters || filterRef.current;
      const params: any = { page, limit: pagination.pageSize };
      if (f.productSearch) params.productSearch = f.productSearch;
      if (f.isReply !== '') params.isReply = f.isReply;
      if (f.nickname) params.nickname = f.nickname;
      if (f.dateRange && f.dateRange[0] && f.dateRange[1]) {
        params.dateLimit = `${f.dateRange[0].format('YYYY-MM-DD')},${f.dateRange[1].format('YYYY-MM-DD')}`;
      }
      const res = await replyListApi(params);
      setList(res?.list || []);
      setPagination((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch { message.error('获取评论列表失败'); }
    finally { setLoading(false); }
  };

  useEffect(() => { fetchList(1); }, []);

  const handleSearch = () => fetchList(1);

  const handleReset = () => {
    setProductSearch('');
    setIsReply('');
    setNickname('');
    setDateRange(null);
    // 传入空过滤条件，避免闭包陈旧
    fetchList(1, { productSearch: '', isReply: '', nickname: '', dateRange: null });
  };

  const handleDelete = async (id: number) => {
    await replyDeleteApi(id);
    message.success('删除成功');
    setSelectedRowKeys((keys) => keys.filter((k) => k !== id));
    fetchList(pagination.current);
  };

  // ========== 商品选择弹窗 ==========
  const formatTreeData = (list: any[]): any[] =>
    list.map((item: any) => ({
      title: item.name,
      value: String(item.id),
      children: item.childList?.length ? formatTreeData(item.childList) : [],
    }));

  const fetchCategoryTree = async () => {
    try {
      const res = await categoryApi({ type: 1 });
      setCategoryTree(formatTreeData(res || []));
    } catch {}
  };

  const fetchProducts = async (page = 1, keyword = productKeyword, cateId = productCateId) => {
    setProductLoading(true);
    try {
      const params: any = { page, limit: 10, type: 1 };
      if (keyword) params.keywords = keyword;
      if (cateId) params.cateId = cateId;
      const res = await productLstApi(params);
      setProductList(res?.list || []);
      setProductPagination((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch { message.error('获取商品列表失败'); }
    finally { setProductLoading(false); }
  };

  const openProductModal = () => {
    setProductKeyword('');
    setProductCateId(undefined);
    setProductModalOpen(true);
    fetchCategoryTree();
    fetchProducts(1, '', undefined);
  };

  const handleSelectProduct = (product: any) => {
    setSelectedProduct(product);
    addForm.setFieldsValue({ productId: product.id });
    setProductModalOpen(false);
  };

  const productColumns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 60 },
    {
      title: '商品图片', dataIndex: 'image', width: 70,
      render: (v: string) => v ? <Image src={v} width={40} height={40} style={{ borderRadius: 4 }} preview={false} /> : '-',
    },
    { title: '商品名称', dataIndex: 'storeName', ellipsis: true },
    {
      title: '操作', width: 80,
      render: (_: any, r: any) => <a onClick={() => handleSelectProduct(r)}>选择</a>,
    },
  ];

  // ========== 添加虚拟评论 ==========
  const openAddModal = () => {
    addForm.resetFields();
    setSelectedProduct(null);
    setAddModalOpen(true);
  };

  const handleAddSubmit = async () => {
    const values = await addForm.validateFields();
    if (!values.productId) { message.warning('请选择商品'); return; }
    const payload: any = {
      productId: values.productId,
      productScore: values.productScore,
      serviceScore: values.serviceScore,
      comment: values.comment,
      nickname: values.nickname || '',
      avatar: values.avatar || '',
      pics: values.pics || '',
      sku: values.sku || '',
    };
    await replyCreatApi(payload);
    message.success('添加虚拟评论成功');
    setAddModalOpen(false);
    addForm.resetFields();
    setSelectedProduct(null);
    fetchList(1);
  };

  // ========== 管理员回复 ==========
  const openReplyModal = (record: any) => {
    setReplyRecord(record);
    setReplyContent('');
    setReplyModalOpen(true);
  };

  const handleReplySubmit = async () => {
    if (!replyContent.trim()) { message.warning('请输入回复内容'); return; }
    await replyCommentApi({ ids: replyRecord.id, merchantReplyContent: replyContent });
    message.success('回复成功');
    setReplyModalOpen(false);
    fetchList(pagination.current);
  };

  // ========== 查看详情 ==========
  const openDetail = (record: any) => {
    setDetailRecord(record);
    setDetailModalOpen(true);
  };

  const columns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 60 },
    {
      title: '商品信息', width: 220,
      render: (_: any, r: any) => {
        const p = r.storeProduct;
        if (!p) return '-';
        return (
          <Space>
            <Image src={p.image} width={40} height={40} style={{ borderRadius: 4 }} preview={false} />
            <span style={{ fontSize: 12 }}>{p.storeName}</span>
          </Space>
        );
      },
    },
    { title: '用户昵称', dataIndex: 'nickname', width: 100 },
    {
      title: '用户头像', dataIndex: 'avatar', width: 70,
      render: (v: string) => v ? <Image src={v} width={32} height={32} style={{ borderRadius: '50%' }} preview={false} /> : '-',
    },
    {
      title: '商品评分', dataIndex: 'productScore', width: 140,
      render: (v: number) => <Rate disabled value={v} style={{ fontSize: 12 }} />,
    },
    {
      title: '服务评分', dataIndex: 'serviceScore', width: 140,
      render: (v: number) => <Rate disabled value={v} style={{ fontSize: 12 }} />,
    },
    { title: '评论内容', dataIndex: 'comment', ellipsis: true, width: 200 },
    {
      title: '评论图片', dataIndex: 'pics', width: 120,
      render: (pics: string[]) => {
        if (!pics || pics.length === 0) return '-';
        return (
          <Image.PreviewGroup>
            <Space size={2}>
              {pics.slice(0, 3).map((url, i) => <Image key={i} src={url} width={30} height={30} style={{ borderRadius: 2 }} />)}
              {pics.length > 3 && <span style={{ fontSize: 12, color: '#999' }}>+{pics.length - 3}</span>}
            </Space>
          </Image.PreviewGroup>
        );
      },
    },
    {
      title: '是否回复', dataIndex: 'isReply', width: 80,
      render: (v: boolean) => v ? <Tag color="green">已回复</Tag> : <Tag color="orange">未回复</Tag>,
    },
    { title: '评论时间', dataIndex: 'createTime', width: 160 },
    {
      title: '操作', width: 180, fixed: 'right' as const,
      render: (_: any, r: any) => (
        <Space>
          <a onClick={() => openDetail(r)}>详情</a>
          {!r.isReply && <a onClick={() => openReplyModal(r)}>回复</a>}
          <Popconfirm title="确定删除?" onConfirm={() => handleDelete(r.id)}>
            <a style={{ color: '#ff4d4f' }}>删除</a>
          </Popconfirm>
        </Space>
      ),
    },
  ];

  return (
    <Card
      title="商品评论"
      extra={<Button type="primary" icon={<PlusOutlined />} onClick={openAddModal}>添加虚拟评论</Button>}
    >
      {/* 搜索栏 */}
      <div style={{ marginBottom: 16, display: 'flex', flexWrap: 'wrap', gap: 12, alignItems: 'center' }}>
        <Input placeholder="商品名称" value={productSearch} onChange={(e) => setProductSearch(e.target.value)} style={{ width: 160 }} allowClear />
        <Input placeholder="用户昵称" value={nickname} onChange={(e) => setNickname(e.target.value)} style={{ width: 130 }} allowClear />
        <Select
          placeholder="回复状态"
          value={isReply}
          onChange={(v) => setIsReply(v)}
          style={{ width: 120 }}
          allowClear
          onClear={() => setIsReply('')}
          options={[
            { label: '全部', value: '' },
            { label: '已回复', value: 'true' },
            { label: '未回复', value: 'false' },
          ]}
        />
        <RangePicker value={dateRange} onChange={(v) => setDateRange(v as any)} />
        <Button type="primary" icon={<SearchOutlined />} onClick={handleSearch}>搜索</Button>
        <Button onClick={handleReset}>重置</Button>
      </div>

      <Table
        rowKey="id"
        columns={columns}
        dataSource={list}
        loading={loading}
        size="small"
        scroll={{ x: 1400 }}
        rowSelection={{ selectedRowKeys, onChange: setSelectedRowKeys }}
        pagination={{ ...pagination, showTotal: (t) => `共 ${t} 条`, onChange: (p) => fetchList(p) }}
      />

      {/* 添加虚拟评论弹窗 */}
      <Modal title="添加虚拟评论" open={addModalOpen} onCancel={() => setAddModalOpen(false)} onOk={handleAddSubmit} destroyOnClose width={600}>
        <Form form={addForm} layout="vertical">
          <Form.Item name="productId" label="选择商品" rules={[{ required: true, message: '请选择商品' }]}>
            <Input type="hidden" />
          </Form.Item>
          <div style={{ marginBottom: 16 }}>
            {selectedProduct ? (
              <div style={{ display: 'flex', alignItems: 'center', gap: 8, padding: 8, background: '#f5f5f5', borderRadius: 6 }}>
                <Image src={selectedProduct.image} width={40} height={40} style={{ borderRadius: 4 }} preview={false} />
                <span style={{ flex: 1 }}>{selectedProduct.storeName}</span>
                <Button size="small" onClick={openProductModal}>重新选择</Button>
              </div>
            ) : (
              <Button onClick={openProductModal} icon={<PlusOutlined />}>选择商品</Button>
            )}
          </div>
          <Row gutter={16}>
            <Col span={12}>
              <Form.Item name="nickname" label="用户昵称">
                <Input placeholder="评论用户昵称" />
              </Form.Item>
            </Col>
            <Col span={12}>
              <Form.Item name="avatar" label="用户头像URL">
                <Input placeholder="头像图片地址" />
              </Form.Item>
            </Col>
          </Row>
          <Row gutter={16}>
            <Col span={12}>
              <Form.Item name="productScore" label="商品评分" rules={[{ required: true }]} initialValue={5}>
                <Rate />
              </Form.Item>
            </Col>
            <Col span={12}>
              <Form.Item name="serviceScore" label="服务评分" rules={[{ required: true }]} initialValue={5}>
                <Rate />
              </Form.Item>
            </Col>
          </Row>
          <Form.Item name="comment" label="评论内容" rules={[{ required: true, message: '请输入评论内容' }]}>
            <TextArea placeholder="请输入评论内容（最多512字符）" rows={3} maxLength={512} showCount />
          </Form.Item>
          <Form.Item name="pics" label="评论图片">
            <Input placeholder="图片URL，多个用逗号分隔" />
          </Form.Item>
          <Form.Item name="sku" label="SKU信息">
            <Input placeholder="如：红色,XL" />
          </Form.Item>
        </Form>
      </Modal>

      {/* 商品选择弹窗 */}
      <Modal title="选择商品" open={productModalOpen} onCancel={() => setProductModalOpen(false)} footer={null} destroyOnClose width={650}>
        <div style={{ marginBottom: 12, display: 'flex', gap: 8 }}>
          <TreeSelect
            treeData={categoryTree}
            value={productCateId}
            onChange={(v) => { setProductCateId(v); fetchProducts(1, productKeyword, v); }}
            placeholder="商品分类"
            allowClear
            onClear={() => { setProductCateId(undefined); fetchProducts(1, productKeyword, undefined); }}
            style={{ width: 180 }}
            treeDefaultExpandAll
          />
          <Input
            placeholder="搜索商品名称"
            value={productKeyword}
            onChange={(e) => setProductKeyword(e.target.value)}
            onPressEnter={() => fetchProducts(1, productKeyword, productCateId)}
            allowClear
            style={{ flex: 1 }}
          />
          <Button type="primary" icon={<SearchOutlined />} onClick={() => fetchProducts(1, productKeyword, productCateId)}>搜索</Button>
        </div>
        <Table
          rowKey="id"
          columns={productColumns}
          dataSource={productList}
          loading={productLoading}
          size="small"
          pagination={{
            ...productPagination,
            showTotal: (t) => `共 ${t} 条`,
            onChange: (p) => fetchProducts(p, productKeyword, productCateId),
          }}
        />
      </Modal>

      {/* 管理员回复弹窗 */}
      <Modal title="回复评论" open={replyModalOpen} onCancel={() => setReplyModalOpen(false)} onOk={handleReplySubmit} destroyOnClose>
        {replyRecord && (
          <div style={{ marginBottom: 12, padding: 12, background: '#f5f5f5', borderRadius: 6 }}>
            <div><b>{replyRecord.nickname}</b> 评论：</div>
            <div style={{ marginTop: 4 }}>{replyRecord.comment}</div>
          </div>
        )}
        <TextArea placeholder="请输入回复内容" rows={4} value={replyContent} onChange={(e) => setReplyContent(e.target.value)} />
      </Modal>

      {/* 评论详情弹窗 */}
      <Modal title="评论详情" open={detailModalOpen} onCancel={() => setDetailModalOpen(false)} footer={null} destroyOnClose width={600}>
        {detailRecord && (
          <div style={{ lineHeight: 2.2 }}>
            <Row gutter={16}>
              <Col span={12}><b>用户昵称：</b>{detailRecord.nickname || '-'}</Col>
              <Col span={12}><b>评论时间：</b>{detailRecord.createTime || '-'}</Col>
            </Row>
            {detailRecord.storeProduct && (
              <Row gutter={16}>
                <Col span={24}>
                  <b>商品：</b>
                  <Space>
                    <Image src={detailRecord.storeProduct.image} width={32} height={32} style={{ borderRadius: 4 }} preview={false} />
                    {detailRecord.storeProduct.storeName}
                  </Space>
                </Col>
              </Row>
            )}
            <Row gutter={16}>
              <Col span={12}><b>商品评分：</b><Rate disabled value={detailRecord.productScore} style={{ fontSize: 12 }} /></Col>
              <Col span={12}><b>服务评分：</b><Rate disabled value={detailRecord.serviceScore} style={{ fontSize: 12 }} /></Col>
            </Row>
            <div><b>评论内容：</b>{detailRecord.comment}</div>
            {detailRecord.pics && detailRecord.pics.length > 0 && (
              <div>
                <b>评论图片：</b>
                <Image.PreviewGroup>
                  <Space style={{ marginTop: 4 }}>
                    {detailRecord.pics.map((url: string, i: number) => (
                      <Image key={i} src={url} width={60} height={60} style={{ borderRadius: 4 }} />
                    ))}
                  </Space>
                </Image.PreviewGroup>
              </div>
            )}
            {detailRecord.sku && <div><b>SKU：</b>{detailRecord.sku}</div>}
            <div>
              <b>回复状态：</b>
              {detailRecord.isReply ? <Tag color="green">已回复</Tag> : <Tag color="orange">未回复</Tag>}
            </div>
            {detailRecord.isReply && detailRecord.merchantReplyContent && (
              <div style={{ marginTop: 8, padding: 12, background: '#f0f9eb', borderRadius: 6 }}>
                <b>商家回复：</b>{detailRecord.merchantReplyContent}
              </div>
            )}
          </div>
        )}
      </Modal>
    </Card>
  );
};

export default StoreComment;
