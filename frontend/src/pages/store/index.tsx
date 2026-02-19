import React, { useState, useEffect, useCallback } from 'react';
import {
  Card, Table, Tabs, Form, Input, Select, Button, Space, Image, Switch, Cascader,
  message, Popconfirm, Modal, Drawer, InputNumber, Tooltip, Spin,
} from 'antd';
import {
  PlusOutlined, SearchOutlined, ReloadOutlined, ExportOutlined,
  CopyOutlined, EditOutlined,
} from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import {
  productLstApi, productHeadersApi, putOnShellApi, offShellApi,
  productDeleteApi, restoreApi, categoryApi, productExcelApi,
  copyProductApi, stockAddApi, productDetailApi,
} from '@/api/store';
import { useNavigate } from 'react-router-dom';

const Store: React.FC = () => {
  const navigate = useNavigate();
  const [activeTab, setActiveTab] = useState('1');
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [keyword, setKeyword] = useState('');
  const [cateId, setCateId] = useState<number[]>([]);
  const [categoryList, setCategoryList] = useState<any[]>([]);
  const [pagination, setPagination] = useState({ current: 1, pageSize: 20, total: 0 });
  const [headerTabs, setHeaderTabs] = useState<{ name: string; count: number; type: number }[]>([]);

  // 商品采集
  const [copyModalOpen, setCopyModalOpen] = useState(false);
  const [copyUrl, setCopyUrl] = useState('');
  const [copyLoading, setCopyLoading] = useState(false);

  // 库存编辑
  const [stockDrawerOpen, setStockDrawerOpen] = useState(false);
  const [stockRecord, setStockRecord] = useState<any>(null);

  // 获取分类
  const fetchCategory = async () => {
    try {
      const res = await categoryApi({ type: 1 });
      setCategoryList(Array.isArray(res) ? res : []);
    } catch { /* noop */ }
  };

  const fetchHeaders = async () => {
    try {
      const lastCateId = cateId.length > 0 ? cateId[cateId.length - 1] : undefined;
      const res = await productHeadersApi({
        page: pagination.current, limit: pagination.pageSize,
        type: activeTab, keywords: keyword || undefined, cateId: lastCateId,
      });
      if (Array.isArray(res)) {
        setHeaderTabs(res);
      }
    } catch { /* noop */ }
  };

  const fetchList = useCallback(async (page = 1) => {
    setLoading(true);
    try {
      const lastCateId = cateId.length > 0 ? cateId[cateId.length - 1] : undefined;
      const res = await productLstApi({
        page, limit: pagination.pageSize, type: activeTab,
        keywords: keyword || undefined, cateId: lastCateId,
      });
      setList(res?.list || []);
      setPagination((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch {
      message.error('获取商品列表失败');
    } finally {
      setLoading(false);
    }
  }, [activeTab, keyword, cateId, pagination.pageSize]);

  useEffect(() => { fetchHeaders(); fetchCategory(); }, []);
  useEffect(() => { fetchList(1); }, [activeTab]);

  const handleReset = () => { setKeyword(''); setCateId([]); };

  const handleShelf = async (id: number, isOn: boolean) => {
    try {
      isOn ? await putOnShellApi(id) : await offShellApi(id);
      message.success(isOn ? '已上架' : '已下架');
      fetchList(pagination.current);
      fetchHeaders();
    } catch { /* noop */ }
  };

  const handleDelete = async (id: number) => {
    try {
      await productDeleteApi(id, isRecycleTab() ? 'delete' : 'recycle');
      message.success('操作成功');
      fetchList(pagination.current);
      fetchHeaders();
    } catch { /* noop */ }
  };

  const handleRestore = async (id: number) => {
    try {
      await restoreApi(id);
      message.success('已恢复');
      fetchList(pagination.current);
      fetchHeaders();
    } catch { /* noop */ }
  };

  // 导出
  const handleExport = async () => {
    try {
      const res = await productExcelApi({ type: activeTab, keywords: keyword || undefined });
      if (res?.fileName) window.open(res.fileName);
      else message.success('导出成功');
    } catch { message.error('导出失败'); }
  };

  // 商品采集
  const handleCopy = async () => {
    if (!copyUrl) { message.warning('请输入商品链接'); return; }
    setCopyLoading(true);
    try {
      await copyProductApi({ url: copyUrl });
      message.success('采集成功，请到仓库中查看');
      setCopyModalOpen(false);
      setCopyUrl('');
      fetchList(1);
      fetchHeaders();
    } catch { message.error('采集失败'); } finally { setCopyLoading(false); }
  };

  // 库存编辑
  const [stockLoading, setStockLoading] = useState(false);
  const [stockAttrValues, setStockAttrValues] = useState<any[]>([]);

  const handleStockEdit = async (record: any) => {
    setStockRecord(record);
    setStockDrawerOpen(true);
    setStockLoading(true);
    try {
      const res = await productDetailApi(record.id);
      const info = res?.productInfo || res || {};
      let attrValues: any[] = [];
      if (info.attrValue) {
        attrValues = Array.isArray(info.attrValue) ? info.attrValue : Object.values(info.attrValue);
      }
      // 如果没有 SKU 数据，用商品本身作为单规格行
      if (attrValues.length === 0) {
        attrValues = [{ suk: '默认', stock: record.stock || 0, price: record.price || 0, _addStock: 0 }];
      } else {
        attrValues = attrValues.map((v: any) => ({ ...v, _addStock: 0 }));
      }
      setStockAttrValues(attrValues);
    } catch {
      setStockAttrValues([{ suk: '默认', stock: record.stock || 0, price: record.price || 0, _addStock: 0 }]);
    } finally { setStockLoading(false); }
  };

  const handleStockSave = async () => {
    try {
      // 构建按SKU的库存更新数据
      const attrStock = stockAttrValues
        .filter((v) => v.id && v._addStock)
        .map((v) => ({ attrValueId: v.id, stock: v._addStock || 0 }));

      if (attrStock.length > 0) {
        // 有SKU数据，按SKU更新
        await stockAddApi({ id: stockRecord.id, stock: 0, attrStock });
      } else {
        // 无SKU数据（单规格），直接加总库存
        const totalAdd = stockAttrValues.reduce((sum, v) => sum + (v._addStock || 0), 0);
        if (totalAdd === 0) { message.warning('请输入库存数量'); return; }
        await stockAddApi({ id: stockRecord.id, stock: totalAdd });
      }
      message.success('库存修改成功');
      setStockDrawerOpen(false);
      fetchList(pagination.current);
    } catch { /* noop */ }
  };

  // 判断当前 tab 是否为回收站
  const isRecycleTab = () => {
    const tab = headerTabs.find((t) => String(t.type) === activeTab);
    return tab?.name?.includes('回收') || false;
  };

  const columns: ColumnsType<any> = [
    { title: 'ID', dataIndex: 'id', width: 60 },
    {
      title: '商品图', dataIndex: 'image', width: 80,
      render: (v: string) => <Image src={v} width={50} height={50} style={{ borderRadius: 4 }} />,
    },
    { title: '商品名称', dataIndex: 'storeName', ellipsis: true, width: 200 },
    { title: '售价', dataIndex: 'price', width: 80, render: (v: any) => `¥${v}` },
    { title: '销量', dataIndex: 'sales', width: 70 },
    { title: '库存', dataIndex: 'stock', width: 70 },
    { title: '排序', dataIndex: 'sort', width: 60 },
    {
      title: '操作', width: 240, fixed: 'right',
      render: (_: any, record: any) => (
        <Space size="small" wrap>
          {!isRecycleTab() && <a onClick={() => navigate(`/store/list/creatProduct?id=${record.id}`)}>编辑</a>}
          {!isRecycleTab() && (
            <>
              <Switch size="small" checked={record.isShow} checkedChildren="上架" unCheckedChildren="下架"
                onChange={(checked) => handleShelf(record.id, checked)} />
              <a onClick={() => handleStockEdit(record)}><EditOutlined /> 库存</a>
            </>
          )}
          {isRecycleTab() && <a onClick={() => handleRestore(record.id)}>恢复</a>}
          <Popconfirm title={isRecycleTab() ? '确定永久删除?' : '确定加入回收站?'}
            onConfirm={() => handleDelete(record.id)}>
            <a style={{ color: '#ff4d4f' }}>{isRecycleTab() ? '删除' : '回收'}</a>
          </Popconfirm>
        </Space>
      ),
    },
  ];

  // 展开行 - 显示详细信息
  const expandedRowRender = (record: any) => (
    <div style={{ display: 'flex', gap: 24, flexWrap: 'wrap', fontSize: 13, color: '#666', padding: '4px 0' }}>
      <span>分类：{record.cateName || '-'}</span>
      <span>市场价：¥{record.otPrice || 0}</span>
      <span>成本价：¥{record.cost || 0}</span>
      <span>收藏数：{record.collectCount || 0}</span>
      <span>虚拟销量：{record.ficti || 0}</span>
      <span>浏览量：{record.browse || 0}</span>
      <span>创建时间：{record.createTime || '-'}</span>
    </div>
  );

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
      {/* 搜索区域 */}
      <Card bodyStyle={{ paddingBottom: 0 }}>
        <Form layout="inline" style={{ marginBottom: 16 }}>
          <Form.Item label="商品分类">
            <Cascader options={categoryList} fieldNames={{ label: 'name', value: 'id', children: 'child' }}
              value={cateId} onChange={(v: any) => setCateId(v || [])}
              placeholder="请选择分类" changeOnSelect allowClear style={{ width: 200 }} />
          </Form.Item>
          <Form.Item label="商品搜索">
            <Input placeholder="商品名称/关键字/ID" value={keyword} onChange={(e) => setKeyword(e.target.value)}
              onPressEnter={() => fetchList(1)} allowClear prefix={<SearchOutlined />} style={{ width: 220 }} />
          </Form.Item>
          <Form.Item>
            <Button type="primary" onClick={() => fetchList(1)}>搜索</Button>
          </Form.Item>
          <Form.Item>
            <Button icon={<ReloadOutlined />} onClick={handleReset}>重置</Button>
          </Form.Item>
        </Form>
      </Card>

      {/* 列表区域 */}
      <Card>
        <div style={{ marginBottom: 16 }}>
          <Tabs activeKey={activeTab} onChange={(k) => setActiveTab(k)} style={{ marginBottom: 8 }}
            items={headerTabs.map((t) => ({
              key: String(t.type),
              label: `${t.name}(${t.count})`,
            }))} />
          <div style={{ display: 'flex', justifyContent: 'flex-end', flexWrap: 'wrap', gap: 8 }}>
            <Button icon={<ExportOutlined />} onClick={handleExport}>导出</Button>
            <Button icon={<CopyOutlined />} onClick={() => setCopyModalOpen(true)}>商品采集</Button>
            <Button type="primary" icon={<PlusOutlined />}
              onClick={() => navigate('/store/list/creatProduct')}>添加商品</Button>
          </div>
        </div>
        <Table rowKey="id" columns={columns} dataSource={list} loading={loading} size="small"
          scroll={{ x: 1000 }}
          expandable={{ expandedRowRender, rowExpandable: () => true }}
          pagination={{
            ...pagination, showSizeChanger: true, pageSizeOptions: ['20', '40', '60', '80'],
            showTotal: (t: number) => `共 ${t} 条`,
            onChange: (p: number, ps: number) => { setPagination((prev) => ({ ...prev, pageSize: ps })); fetchList(p); },
          }} />
      </Card>

      {/* 商品采集弹窗 */}
      <Modal title="商品采集" open={copyModalOpen} onCancel={() => { setCopyModalOpen(false); setCopyUrl(''); }}
        onOk={handleCopy} confirmLoading={copyLoading} destroyOnClose>
        <div style={{ marginBottom: 12, color: '#999', fontSize: 13 }}>
          支持采集淘宝、天猫、京东、苏宁、拼多多商品，请粘贴商品链接
        </div>
        <Input.TextArea rows={3} placeholder="请粘贴商品链接" value={copyUrl}
          onChange={(e) => setCopyUrl(e.target.value)} />
      </Modal>

      {/* 库存编辑抽屉 */}
      <Drawer title={`编辑库存 - ${stockRecord?.storeName || ''}`} open={stockDrawerOpen}
        onClose={() => setStockDrawerOpen(false)} width={600}
        extra={<Button type="primary" onClick={handleStockSave}>保存</Button>}>
        <Spin spinning={stockLoading}>
          {stockRecord && (
            <div style={{ marginBottom: 16, display: 'flex', gap: 12, alignItems: 'center' }}>
              {stockRecord.image && <Image src={stockRecord.image} width={60} height={60} style={{ borderRadius: 4 }} />}
              <div>
                <div style={{ fontWeight: 500 }}>{stockRecord.storeName}</div>
                <div style={{ color: '#999', fontSize: 13 }}>
                  售价：¥{stockRecord.price} | 当前总库存：{stockRecord.stock}
                </div>
              </div>
            </div>
          )}
          <Table
            dataSource={stockAttrValues}
            rowKey={(_, idx) => String(idx)}
            size="small"
            pagination={false}
            bordered
            scroll={{ x: 400 }}
            columns={[
              { title: '规格', dataIndex: 'suk', width: 120, ellipsis: true },
              { title: '售价(元)', dataIndex: 'price', width: 90, render: (v: any) => `¥${v || 0}` },
              { title: '当前库存', dataIndex: 'stock', width: 90 },
              {
                title: '增加库存', width: 120,
                render: (_: any, __: any, idx: number) => (
                  <InputNumber size="small" min={0} style={{ width: '100%' }}
                    value={stockAttrValues[idx]?._addStock || 0}
                    onChange={(v) => setStockAttrValues((prev) =>
                      prev.map((row, i) => i === idx ? { ...row, _addStock: v || 0 } : row)
                    )} />
                ),
              },
            ]}
          />
        </Spin>
      </Drawer>
    </div>
  );
};

export default Store;
