import React, { useState, useEffect, useCallback } from "react";
import {
  Card,
  Table,
  Button,
  Space,
  Input,
  message,
  Popconfirm,
  Tag,
  Tabs,
  Modal,
  QRCode,
} from "antd";
import {
  PlusOutlined,
  SearchOutlined,
  ReloadOutlined,
  EyeOutlined,
} from "@ant-design/icons";
import type { ColumnsType } from "antd/es/table";
import {
  pagediyListApi,
  pagediyDeleteApi,
  pagediySetdefaultApi,
  pagediyGetSetHome,
  pagediySaveApi,
  pagediyInfoApi,
} from "@/api/pagediy";
import { useNavigate } from "react-router-dom";

const DesignList: React.FC = () => {
  const navigate = useNavigate();
  const [activeTab, setActiveTab] = useState("home");
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [keyword, setKeyword] = useState("");
  const [pagination, setPagination] = useState({
    current: 1,
    pageSize: 20,
    total: 0,
  });
  const [defaultId, setDefaultId] = useState<number | null>(null);
  const [previewVisible, setPreviewVisible] = useState(false);
  const [previewUrl, setPreviewUrl] = useState("");

  // 获取前端域名
  const frontDomain = localStorage.getItem("frontDomain") || "";
  const fullFrontDomain = frontDomain.startsWith("http")
    ? frontDomain
    : `https://${frontDomain}`;

  const fetchDefault = async () => {
    try {
      const res = await pagediyGetSetHome();
      setDefaultId(res?.id || res || null);
    } catch {
      /* ignore */
    }
  };

  const fetchList = useCallback(
    async (page = 1) => {
      setLoading(true);
      try {
        const res = await pagediyListApi({
          page,
          limit: pagination.pageSize,
          name: keyword || undefined,
        });
        setList(res?.list || []);
        setPagination((p) => ({ ...p, current: page, total: res?.total || 0 }));
      } catch {
        message.error("获取列表失败");
      } finally {
        setLoading(false);
      }
    },
    [pagination.pageSize, keyword],
  );

  useEffect(() => {
    fetchDefault();
    fetchList(1);
  }, []);

  const handleSetDefault = async (id: number) => {
    try {
      await pagediySetdefaultApi(id);
      message.success("已设为首页");
      setDefaultId(id);
      fetchList(pagination.current);
    } catch {
      message.error("设置失败");
    }
  };

  const handleCopy = async (record: any) => {
    try {
      const info = await pagediyInfoApi(record.id);
      await pagediySaveApi({
        ...info,
        name: (info.name || record.name) + "-副本",
        id: undefined,
        isDefault: 0,
      });
      message.success("复制成功");
      fetchList(pagination.current);
    } catch {
      message.error("复制失败");
    }
  };

  const handleDelete = async (id: number) => {
    if (id === defaultId) {
      message.warning("首页模板不能删除");
      return;
    }
    try {
      await pagediyDeleteApi({ id });
      message.success("删除成功");
      fetchList(pagination.current);
    } catch {
      message.error("删除失败");
    }
  };

  const handlePreview = (id: number) => {
    setPreviewUrl(`${fullFrontDomain}?id=${id}`);
    setPreviewVisible(true);
  };

  const handleEditHomePage = async () => {
    try {
      const homePageId = await pagediyGetSetHome();
      if (homePageId) {
        navigate(`/design/builder?id=${homePageId}`);
      } else {
        navigate("/design/builder");
      }
    } catch {
      navigate("/design/builder");
    }
  };

  const columns: ColumnsType<any> = [
    { title: "ID", dataIndex: "id", width: 60 },
    {
      title: "模板名称",
      dataIndex: "name",
      ellipsis: true,
      render: (name: string, record: any) => (
        <Space>
          {record.id === defaultId && <Tag color="green">首页</Tag>}
          <span>{name}</span>
        </Space>
      ),
    },
    { title: "创建时间", dataIndex: "addTime", width: 170 },
    { title: "更新时间", dataIndex: "updateTime", width: 170 },
    {
      title: "操作",
      width: 280,
      render: (_: any, r: any) => (
        <Space>
          <a onClick={() => navigate(`/design/builder?id=${r.id}`)}>设计</a>
          {r.id !== defaultId && (
            <a onClick={() => handleSetDefault(r.id)}>设为首页</a>
          )}
          <a onClick={() => handlePreview(r.id)}>预览</a>
          <a onClick={() => handleCopy(r)}>复制</a>
          {r.id !== defaultId && (
            <Popconfirm title="确定删除?" onConfirm={() => handleDelete(r.id)}>
              <a style={{ color: "#ff4d4f" }}>删除</a>
            </Popconfirm>
          )}
        </Space>
      ),
    },
  ];

  // 商城首页 Tab 内容
  const renderHomeTab = () => (
    <div style={{ display: "flex", gap: 40 }}>
      {/* 左侧手机预览 */}
      <div style={{ flexShrink: 0 }}>
        <div
          style={{
            width: 375,
            height: 667,
            borderRadius: 30,
            border: "8px solid #222",
            overflow: "hidden",
            background: "#fff",
            position: "relative",
          }}
        >
          <div
            style={{
              height: 24,
              background: "#000",
              borderRadius: "22px 22px 0 0",
            }}
          />
          <iframe
            src={fullFrontDomain}
            style={{ width: "100%", height: 619, border: "none" }}
            title="商城预览"
          />
          <div
            style={{
              position: "absolute",
              top: 24,
              left: 0,
              right: 0,
              bottom: 0,
              background: "transparent",
              pointerEvents: "none",
            }}
          />
        </div>
      </div>

      {/* 右侧操作区 */}
      <div style={{ flex: 1, maxWidth: 500 }}>
        <Button
          type="primary"
          size="large"
          onClick={handleEditHomePage}
          style={{ marginBottom: 24 }}
        >
          首页装修
        </Button>

        {/* 微信小程序二维码 */}
        <Card size="small" style={{ marginBottom: 16, background: "#f9f9f9" }}>
          <div
            style={{
              display: "flex",
              justifyContent: "space-between",
              alignItems: "center",
            }}
          >
            <div>
              <div style={{ fontSize: 18, fontWeight: 600, marginBottom: 8 }}>
                微信小程序
              </div>
              <div style={{ color: "#999" }}>扫描右侧二维码查看</div>
            </div>
            <QRCode value={fullFrontDomain} size={100} />
          </div>
        </Card>

        {/* 微信公众号二维码 */}
        <Card size="small" style={{ background: "#f9f9f9" }}>
          <div
            style={{
              display: "flex",
              justifyContent: "space-between",
              alignItems: "center",
            }}
          >
            <div>
              <div style={{ fontSize: 18, fontWeight: 600, marginBottom: 8 }}>
                微信公众号
              </div>
              <div style={{ color: "#999" }}>扫描右侧二维码查看</div>
            </div>
            <QRCode value={fullFrontDomain} size={100} />
          </div>
        </Card>
      </div>
    </div>
  );

  // 自定义页面 Tab 内容
  const renderCustomTab = () => (
    <>
      <div
        style={{
          marginBottom: 16,
          display: "flex",
          justifyContent: "space-between",
        }}
      >
        <Space>
          <Input
            placeholder="模板名称"
            value={keyword}
            onChange={(e) => setKeyword(e.target.value)}
            onPressEnter={() => fetchList(1)}
            prefix={<SearchOutlined />}
            style={{ width: 200 }}
            allowClear
          />
          <Button type="primary" onClick={() => fetchList(1)}>
            搜索
          </Button>
          <Button
            icon={<ReloadOutlined />}
            onClick={() => fetchList(pagination.current)}
          >
            刷新
          </Button>
        </Space>
        <Button
          type="primary"
          icon={<PlusOutlined />}
          onClick={() => navigate("/design/builder")}
        >
          添加模板
        </Button>
      </div>
      <Table
        rowKey="id"
        columns={columns}
        dataSource={list}
        loading={loading}
        size="small"
        pagination={{
          ...pagination,
          showTotal: (t) => `共 ${t} 条`,
          showSizeChanger: true,
          onChange: (p, size) => {
            setPagination((prev) => ({
              ...prev,
              pageSize: size || prev.pageSize,
            }));
            fetchList(p);
          },
        }}
      />
    </>
  );

  return (
    <Card title="页面设计">
      <Tabs
        activeKey={activeTab}
        onChange={setActiveTab}
        items={[
          { key: "home", label: "商城首页", children: renderHomeTab() },
          { key: "custom", label: "自定义页面", children: renderCustomTab() },
        ]}
      />

      {/* 预览弹窗 */}
      <Modal
        title="页面预览"
        open={previewVisible}
        onCancel={() => setPreviewVisible(false)}
        footer={null}
        width={430}
        centered
        destroyOnClose
      >
        <iframe
          src={previewUrl}
          style={{ width: 390, height: 650, border: "none", borderRadius: 8 }}
          title="页面预览"
        />
      </Modal>
    </Card>
  );
};

export default DesignList;
