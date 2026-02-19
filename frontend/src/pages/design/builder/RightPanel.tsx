import React, { useState } from "react";
import {
  Form,
  Input,
  InputNumber,
  Switch,
  Select,
  ColorPicker,
  Divider,
  Button,
  Space,
  Image,
} from "antd";
import { PlusOutlined, DeleteOutlined } from "@ant-design/icons";
import type { ComponentInstance } from "./components";
import UploadPicture from "@/components/UploadPicture";
import GoodList from "@/components/GoodList";

interface RightPanelProps {
  activeComponent: ComponentInstance | null;
  pageConfig: {
    name: string;
    title: string;
    bgColor: string;
    bgPic: string;
    titleColor: string;
    titleBgColor: string;
  };
  onUpdateConfig: (config: Record<string, any>) => void;
  onUpdatePage: (config: Record<string, any>) => void;
}

const RightPanel: React.FC<RightPanelProps> = ({
  activeComponent,
  pageConfig,
  onUpdateConfig,
  onUpdatePage,
}) => {
  const [goodsModalOpen, setGoodsModalOpen] = useState(false);
  const [currentGoodsTarget, setCurrentGoodsTarget] = useState<string>(""); // 用于标识当前选择商品的目标字段

  if (!activeComponent) {
    // Page settings
    return (
      <div
        style={{
          width: 300,
          height: "100%",
          overflow: "auto",
          borderLeft: "1px solid #f0f0f0",
          background: "#fff",
        }}
      >
        <div
          style={{
            padding: "12px 16px",
            fontWeight: 600,
            borderBottom: "1px solid #f0f0f0",
          }}
        >
          页面设置
        </div>
        <div style={{ padding: 16 }}>
          <Form layout="vertical" size="small">
            <Form.Item label="页面名称">
              <Input
                value={pageConfig.name}
                onChange={(e) => onUpdatePage({ name: e.target.value })}
                placeholder="请输入页面名称"
                maxLength={15}
              />
            </Form.Item>
            <Form.Item label="页面标题">
              <Input
                value={pageConfig.title}
                onChange={(e) => onUpdatePage({ title: e.target.value })}
                placeholder="请输入页面标题"
              />
            </Form.Item>
            <Form.Item label="背景颜色">
              <ColorPicker
                value={pageConfig.bgColor}
                onChange={(_, hex) => onUpdatePage({ bgColor: hex })}
                showText
              />
            </Form.Item>
            <Form.Item label="背景图片">
              <UploadPicture
                value={pageConfig.bgPic}
                onChange={(url) => onUpdatePage({ bgPic: url as string })}
                limit={1}
              />
            </Form.Item>
            <Form.Item label="标题颜色">
              <ColorPicker
                value={pageConfig.titleColor}
                onChange={(_, hex) => onUpdatePage({ titleColor: hex })}
                showText
              />
            </Form.Item>
            <Form.Item label="标题栏背景">
              <ColorPicker
                value={pageConfig.titleBgColor}
                onChange={(_, hex) => onUpdatePage({ titleBgColor: hex })}
                showText
              />
            </Form.Item>
          </Form>
        </div>
      </div>
    );
  }

  const cfg = activeComponent.defaultConfig;
  const update = (key: string, value: any) => {
    onUpdateConfig({ [key]: value });
  };
  const updateNested = (parentKey: string, childKey: string, value: any) => {
    onUpdateConfig({ [parentKey]: { ...cfg[parentKey], [childKey]: value } });
  };

  // 处理商品选择
  const handleGoodsSelect = (goods: any[]) => {
    if (currentGoodsTarget === "goodsList") {
      const existingIds = (cfg.goodsList?.list || []).map((g: any) => g.id);
      const newGoods = goods.filter((g) => !existingIds.includes(g.id));
      update("goodsList", {
        ...cfg.goodsList,
        list: [
          ...(cfg.goodsList?.list || []),
          ...newGoods.map((g) => ({
            id: g.id,
            image: g.image,
            storeName: g.storeName,
            price: g.price,
          })),
        ],
      });
    } else if (currentGoodsTarget === "goodsIds") {
      const existingIds = cfg.goodsIds || [];
      const newIds = goods
        .map((g) => g.id)
        .filter((id) => !existingIds.includes(id));
      update("goodsIds", [...existingIds, ...newIds]);
      update("goodsData", [
        ...(cfg.goodsData || []),
        ...goods.filter((g) => newIds.includes(g.id)),
      ]);
    }
    setGoodsModalOpen(false);
  };

  // 通用内边距配置
  const renderPaddingConfig = () => (
    <>
      <Divider>内边距</Divider>
      <Space style={{ width: "100%" }} wrap>
        <Form.Item label="上" style={{ marginBottom: 8 }}>
          <InputNumber
            value={cfg.padding?.top}
            onChange={(v) => updateNested("padding", "top", v)}
            min={0}
            max={50}
            style={{ width: 60 }}
          />
        </Form.Item>
        <Form.Item label="下" style={{ marginBottom: 8 }}>
          <InputNumber
            value={cfg.padding?.bottom}
            onChange={(v) => updateNested("padding", "bottom", v)}
            min={0}
            max={50}
            style={{ width: 60 }}
          />
        </Form.Item>
        <Form.Item label="左" style={{ marginBottom: 8 }}>
          <InputNumber
            value={cfg.padding?.left}
            onChange={(v) => updateNested("padding", "left", v)}
            min={0}
            max={50}
            style={{ width: 60 }}
          />
        </Form.Item>
        <Form.Item label="右" style={{ marginBottom: 8 }}>
          <InputNumber
            value={cfg.padding?.right}
            onChange={(v) => updateNested("padding", "right", v)}
            min={0}
            max={50}
            style={{ width: 60 }}
          />
        </Form.Item>
      </Space>
    </>
  );

  // Render config fields based on component type
  const renderConfig = () => {
    switch (activeComponent.name) {
      case "banner":
        return (
          <>
            <Form.Item label="背景颜色">
              <ColorPicker
                value={cfg.bgColor}
                onChange={(_, hex) => update("bgColor", hex)}
                showText
              />
            </Form.Item>
            <Form.Item label="自动播放速度(ms)">
              <InputNumber
                value={cfg.swiperConfig?.speed}
                onChange={(v) =>
                  update("swiperConfig", { ...cfg.swiperConfig, speed: v })
                }
                min={1000}
                max={10000}
                step={500}
                style={{ width: "100%" }}
              />
            </Form.Item>
            <Form.Item label="自动播放">
              <Switch
                checked={cfg.swiperConfig?.autoplay}
                onChange={(v) =>
                  update("swiperConfig", { ...cfg.swiperConfig, autoplay: v })
                }
              />
            </Form.Item>
            <Form.Item label="圆角">
              <InputNumber
                value={cfg.borderRadius}
                onChange={(v) => update("borderRadius", v)}
                min={0}
                max={30}
                style={{ width: "100%" }}
              />
            </Form.Item>
            <Divider>
              图片列表 ({cfg.swiperConfig?.imgList?.length || 0})
            </Divider>
            {(cfg.swiperConfig?.imgList || []).map((item: any, i: number) => (
              <div
                key={i}
                style={{
                  marginBottom: 12,
                  padding: 12,
                  background: "#fafafa",
                  borderRadius: 6,
                }}
              >
                <Form.Item label={`图片${i + 1}`} style={{ marginBottom: 8 }}>
                  <UploadPicture
                    value={item.img}
                    onChange={(url) => {
                      const list = [...(cfg.swiperConfig?.imgList || [])];
                      list[i] = { ...list[i], img: url as string };
                      update("swiperConfig", {
                        ...cfg.swiperConfig,
                        imgList: list,
                      });
                    }}
                    limit={1}
                  />
                </Form.Item>
                <Form.Item label="跳转链接" style={{ marginBottom: 8 }}>
                  <Input
                    value={item.link}
                    onChange={(e) => {
                      const list = [...(cfg.swiperConfig?.imgList || [])];
                      list[i] = { ...list[i], link: e.target.value };
                      update("swiperConfig", {
                        ...cfg.swiperConfig,
                        imgList: list,
                      });
                    }}
                    placeholder="跳转链接"
                  />
                </Form.Item>
                <Button
                  size="small"
                  danger
                  icon={<DeleteOutlined />}
                  onClick={() => {
                    const list = (cfg.swiperConfig?.imgList || []).filter(
                      (_: any, idx: number) => idx !== i,
                    );
                    update("swiperConfig", {
                      ...cfg.swiperConfig,
                      imgList: list,
                    });
                  }}
                >
                  删除
                </Button>
              </div>
            ))}
            <Button
              type="dashed"
              block
              icon={<PlusOutlined />}
              onClick={() => {
                const list = [
                  ...(cfg.swiperConfig?.imgList || []),
                  { img: "", link: "" },
                ];
                update("swiperConfig", { ...cfg.swiperConfig, imgList: list });
              }}
            >
              添加图片
            </Button>
            {renderPaddingConfig()}
          </>
        );

      case "home_title":
        return (
          <>
            <Form.Item label="标题">
              <Input
                value={cfg.title}
                onChange={(e) => update("title", e.target.value)}
              />
            </Form.Item>
            <Form.Item label="副标题">
              <Input
                value={cfg.subtitle}
                onChange={(e) => update("subtitle", e.target.value)}
              />
            </Form.Item>
            <Form.Item label="标题颜色">
              <ColorPicker
                value={cfg.titleColor}
                onChange={(_, hex) => update("titleColor", hex)}
                showText
              />
            </Form.Item>
            <Form.Item label="副标题颜色">
              <ColorPicker
                value={cfg.subtitleColor}
                onChange={(_, hex) => update("subtitleColor", hex)}
                showText
              />
            </Form.Item>
            <Form.Item label="背景颜色">
              <ColorPicker
                value={cfg.bgColor}
                onChange={(_, hex) => update("bgColor", hex)}
                showText
              />
            </Form.Item>
            <Form.Item label="对齐方式">
              <Select
                value={cfg.textAlign}
                onChange={(v) => update("textAlign", v)}
                options={[
                  { value: "left", label: "左对齐" },
                  { value: "center", label: "居中" },
                  { value: "right", label: "右对齐" },
                ]}
              />
            </Form.Item>
            <Form.Item label="字号">
              <InputNumber
                value={cfg.fontSize}
                onChange={(v) => update("fontSize", v)}
                min={12}
                max={30}
                style={{ width: "100%" }}
              />
            </Form.Item>
            <Form.Item label="显示更多">
              <Switch
                checked={cfg.showMore}
                onChange={(v) => update("showMore", v)}
              />
            </Form.Item>
            {cfg.showMore && (
              <Form.Item label="更多链接">
                <Input
                  value={cfg.moreLink}
                  onChange={(e) => update("moreLink", e.target.value)}
                  placeholder="跳转链接"
                />
              </Form.Item>
            )}
            {renderPaddingConfig()}
          </>
        );

      case "home_menu":
        return (
          <>
            <Form.Item label="每行数量">
              <InputNumber
                value={cfg.rowCount}
                onChange={(v) => update("rowCount", v)}
                min={2}
                max={5}
                style={{ width: "100%" }}
              />
            </Form.Item>
            <Form.Item label="背景颜色">
              <ColorPicker
                value={cfg.bgColor}
                onChange={(_, hex) => update("bgColor", hex)}
                showText
              />
            </Form.Item>
            <Form.Item label="文字颜色">
              <ColorPicker
                value={cfg.titleColor}
                onChange={(_, hex) => update("titleColor", hex)}
                showText
              />
            </Form.Item>
            <Divider>菜单项 ({cfg.menuList?.length || 0})</Divider>
            {(cfg.menuList || []).map((item: any, i: number) => (
              <div
                key={i}
                style={{
                  marginBottom: 12,
                  padding: 12,
                  background: "#fafafa",
                  borderRadius: 6,
                }}
              >
                <Form.Item label="标题" style={{ marginBottom: 8 }}>
                  <Input
                    value={item.title}
                    onChange={(e) => {
                      const list = [...(cfg.menuList || [])];
                      list[i] = { ...list[i], title: e.target.value };
                      update("menuList", list);
                    }}
                  />
                </Form.Item>
                <Form.Item label="图标" style={{ marginBottom: 8 }}>
                  <UploadPicture
                    value={item.img}
                    onChange={(url) => {
                      const list = [...(cfg.menuList || [])];
                      list[i] = { ...list[i], img: url as string };
                      update("menuList", list);
                    }}
                    limit={1}
                  />
                </Form.Item>
                <Form.Item label="跳转链接" style={{ marginBottom: 8 }}>
                  <Input
                    value={item.link}
                    onChange={(e) => {
                      const list = [...(cfg.menuList || [])];
                      list[i] = { ...list[i], link: e.target.value };
                      update("menuList", list);
                    }}
                    placeholder="跳转链接"
                  />
                </Form.Item>
                <Button
                  size="small"
                  danger
                  icon={<DeleteOutlined />}
                  onClick={() => {
                    update(
                      "menuList",
                      (cfg.menuList || []).filter(
                        (_: any, idx: number) => idx !== i,
                      ),
                    );
                  }}
                >
                  删除
                </Button>
              </div>
            ))}
            <Button
              type="dashed"
              block
              icon={<PlusOutlined />}
              onClick={() => {
                update("menuList", [
                  ...(cfg.menuList || []),
                  { img: "", title: "新菜单", link: "" },
                ]);
              }}
            >
              添加菜单
            </Button>
            {renderPaddingConfig()}
          </>
        );

      case "search_box":
        return (
          <>
            <Form.Item label="占位文字">
              <Input
                value={cfg.placeholder}
                onChange={(e) => update("placeholder", e.target.value)}
              />
            </Form.Item>
            <Form.Item label="背景颜色">
              <ColorPicker
                value={cfg.bgColor}
                onChange={(_, hex) => update("bgColor", hex)}
                showText
              />
            </Form.Item>
            <Form.Item label="边框颜色">
              <ColorPicker
                value={cfg.borderColor}
                onChange={(_, hex) => update("borderColor", hex)}
                showText
              />
            </Form.Item>
            <Form.Item label="圆角">
              <InputNumber
                value={cfg.borderRadius}
                onChange={(v) => update("borderRadius", v)}
                min={0}
                max={30}
                style={{ width: "100%" }}
              />
            </Form.Item>
            <Form.Item label="对齐方式">
              <Select
                value={cfg.textAlign}
                onChange={(v) => update("textAlign", v)}
                options={[
                  { value: "left", label: "左对齐" },
                  { value: "center", label: "居中" },
                ]}
              />
            </Form.Item>
          </>
        );

      case "nav_bar":
        return (
          <>
            <Form.Item label="标题">
              <Input
                value={cfg.title}
                onChange={(e) => update("title", e.target.value)}
              />
            </Form.Item>
            <Form.Item label="背景颜色">
              <ColorPicker
                value={cfg.bgColor}
                onChange={(_, hex) => update("bgColor", hex)}
                showText
              />
            </Form.Item>
            <Form.Item label="标题颜色">
              <ColorPicker
                value={cfg.titleColor}
                onChange={(_, hex) => update("titleColor", hex)}
                showText
              />
            </Form.Item>
            <Form.Item label="显示返回">
              <Switch
                checked={cfg.showBack}
                onChange={(v) => update("showBack", v)}
              />
            </Form.Item>
          </>
        );

      case "home_comb":
        return (
          <>
            <Form.Item label="背景颜色">
              <ColorPicker
                value={cfg.bgColor}
                onChange={(_, hex) => update("bgColor", hex)}
                showText
              />
            </Form.Item>
            <Form.Item label="文字颜色">
              <ColorPicker
                value={cfg.titleColor}
                onChange={(_, hex) => update("titleColor", hex)}
                showText
              />
            </Form.Item>
            <Form.Item label="显示搜索">
              <Switch
                checked={cfg.showSearch}
                onChange={(v) => update("showSearch", v)}
              />
            </Form.Item>
            <Form.Item label="显示Logo">
              <Switch
                checked={cfg.showLogo}
                onChange={(v) => update("showLogo", v)}
              />
            </Form.Item>
            {cfg.showLogo && (
              <Form.Item label="Logo图片">
                <UploadPicture
                  value={cfg.logoUrl}
                  onChange={(url) => update("logoUrl", url as string)}
                  limit={1}
                />
              </Form.Item>
            )}
          </>
        );

      case "z_auxiliary_box":
        return (
          <>
            <Form.Item label="高度(px)">
              <InputNumber
                value={cfg.height}
                onChange={(v) => update("height", v)}
                min={1}
                max={200}
                style={{ width: "100%" }}
              />
            </Form.Item>
            <Form.Item label="背景颜色">
              <ColorPicker
                value={cfg.bgColor}
                onChange={(_, hex) => update("bgColor", hex)}
                showText
              />
            </Form.Item>
          </>
        );

      case "z_auxiliary_line":
        return (
          <>
            <Form.Item label="线条颜色">
              <ColorPicker
                value={cfg.lineColor}
                onChange={(_, hex) => update("lineColor", hex)}
                showText
              />
            </Form.Item>
            <Form.Item label="线条样式">
              <Select
                value={cfg.lineStyle}
                onChange={(v) => update("lineStyle", v)}
                options={[
                  { value: "solid", label: "实线" },
                  { value: "dashed", label: "虚线" },
                  { value: "dotted", label: "点线" },
                ]}
              />
            </Form.Item>
            <Form.Item label="线条粗细">
              <InputNumber
                value={cfg.lineHeight}
                onChange={(v) => update("lineHeight", v)}
                min={1}
                max={10}
                style={{ width: "100%" }}
              />
            </Form.Item>
            <Divider>外边距</Divider>
            <Space>
              <Form.Item label="上" style={{ marginBottom: 8 }}>
                <InputNumber
                  value={cfg.margin?.top}
                  onChange={(v) => updateNested("margin", "top", v)}
                  min={0}
                  max={50}
                  style={{ width: 60 }}
                />
              </Form.Item>
              <Form.Item label="下" style={{ marginBottom: 8 }}>
                <InputNumber
                  value={cfg.margin?.bottom}
                  onChange={(v) => updateNested("margin", "bottom", v)}
                  min={0}
                  max={50}
                  style={{ width: 60 }}
                />
              </Form.Item>
            </Space>
          </>
        );

      case "z_ueditor":
        return (
          <>
            <Form.Item label="内容">
              <Input.TextArea
                value={cfg.content}
                onChange={(e) => update("content", e.target.value)}
                rows={8}
                placeholder="支持HTML内容"
              />
            </Form.Item>
            <Form.Item label="背景颜色">
              <ColorPicker
                value={cfg.bgColor}
                onChange={(_, hex) => update("bgColor", hex)}
                showText
              />
            </Form.Item>
            {renderPaddingConfig()}
          </>
        );

      case "home_goods_list":
        return (
          <>
            <Form.Item label="数据来源">
              <Select
                value={cfg.sourceType || "auto"}
                onChange={(v) => update("sourceType", v)}
                options={[
                  { value: "auto", label: "自动获取" },
                  { value: "manual", label: "手动选择" },
                ]}
              />
            </Form.Item>
            {cfg.sourceType === "manual" && (
              <>
                <Divider>已选商品 ({cfg.goodsData?.length || 0})</Divider>
                <div
                  style={{
                    display: "flex",
                    flexWrap: "wrap",
                    gap: 8,
                    marginBottom: 12,
                  }}
                >
                  {(cfg.goodsData || []).map((g: any, i: number) => (
                    <div
                      key={g.id}
                      style={{ position: "relative", width: 60, height: 60 }}
                    >
                      <Image
                        src={g.image}
                        width={60}
                        height={60}
                        style={{ borderRadius: 4 }}
                      />
                      <DeleteOutlined
                        style={{
                          position: "absolute",
                          top: -6,
                          right: -6,
                          color: "#ff4d4f",
                          cursor: "pointer",
                          background: "#fff",
                          borderRadius: "50%",
                          padding: 2,
                        }}
                        onClick={() => {
                          update(
                            "goodsIds",
                            (cfg.goodsIds || []).filter(
                              (id: number) => id !== g.id,
                            ),
                          );
                          update(
                            "goodsData",
                            (cfg.goodsData || []).filter(
                              (_: any, idx: number) => idx !== i,
                            ),
                          );
                        }}
                      />
                    </div>
                  ))}
                </div>
                <Button
                  type="dashed"
                  block
                  icon={<PlusOutlined />}
                  onClick={() => {
                    setCurrentGoodsTarget("goodsIds");
                    setGoodsModalOpen(true);
                  }}
                >
                  选择商品
                </Button>
              </>
            )}
            {cfg.sourceType !== "manual" && (
              <>
                <Form.Item label="排序方式">
                  <Select
                    value={cfg.sortType}
                    onChange={(v) => update("sortType", v)}
                    options={[
                      { value: "default", label: "默认" },
                      { value: "sales", label: "销量" },
                      { value: "price", label: "价格" },
                    ]}
                  />
                </Form.Item>
                <Form.Item label="显示数量">
                  <InputNumber
                    value={cfg.limit}
                    onChange={(v) => update("limit", v)}
                    min={1}
                    max={50}
                    style={{ width: "100%" }}
                  />
                </Form.Item>
              </>
            )}
            <Divider>样式设置</Divider>
            <Form.Item label="列表样式">
              <Select
                value={cfg.listStyle}
                onChange={(v) => update("listStyle", v)}
                options={[
                  { value: "grid", label: "网格" },
                  { value: "list", label: "列表" },
                  { value: "scroll", label: "横向滚动" },
                ]}
              />
            </Form.Item>
            {cfg.listStyle === "grid" && (
              <Form.Item label="列数">
                <InputNumber
                  value={cfg.columns}
                  onChange={(v) => update("columns", v)}
                  min={1}
                  max={3}
                  style={{ width: "100%" }}
                />
              </Form.Item>
            )}
            <Form.Item label="显示名称">
              <Switch
                checked={cfg.showName}
                onChange={(v) => update("showName", v)}
              />
            </Form.Item>
            <Form.Item label="显示价格">
              <Switch
                checked={cfg.showPrice}
                onChange={(v) => update("showPrice", v)}
              />
            </Form.Item>
            <Form.Item label="显示销量">
              <Switch
                checked={cfg.showSales}
                onChange={(v) => update("showSales", v)}
              />
            </Form.Item>
            <Form.Item label="背景颜色">
              <ColorPicker
                value={cfg.bgColor}
                onChange={(_, hex) => update("bgColor", hex)}
                showText
              />
            </Form.Item>
            <Form.Item label="卡片背景">
              <ColorPicker
                value={cfg.cardBgColor}
                onChange={(_, hex) => update("cardBgColor", hex)}
                showText
              />
            </Form.Item>
            <Form.Item label="卡片圆角">
              <InputNumber
                value={cfg.borderRadius}
                onChange={(v) => update("borderRadius", v)}
                min={0}
                max={20}
                style={{ width: "100%" }}
              />
            </Form.Item>
            {renderPaddingConfig()}
          </>
        );

      case "home_coupon":
        return (
          <>
            <Form.Item label="背景颜色">
              <ColorPicker
                value={cfg.bgColor}
                onChange={(_, hex) => update("bgColor", hex)}
                showText
              />
            </Form.Item>
            <Form.Item label="主题色">
              <ColorPicker
                value={cfg.primaryColor}
                onChange={(_, hex) => update("primaryColor", hex)}
                showText
              />
            </Form.Item>
            <Form.Item label="显示数量">
              <InputNumber
                value={cfg.limit || 3}
                onChange={(v) => update("limit", v)}
                min={1}
                max={10}
                style={{ width: "100%" }}
              />
            </Form.Item>
            {renderPaddingConfig()}
          </>
        );

      case "home_bargain":
      case "home_seckill":
      case "home_group":
        return (
          <>
            <Form.Item label="列表样式">
              <Select
                value={cfg.listStyle}
                onChange={(v) => update("listStyle", v)}
                options={[
                  { value: "scroll", label: "横向滚动" },
                  { value: "grid", label: "网格" },
                ]}
              />
            </Form.Item>
            <Form.Item label="显示数量">
              <InputNumber
                value={cfg.limit}
                onChange={(v) => update("limit", v)}
                min={1}
                max={20}
                style={{ width: "100%" }}
              />
            </Form.Item>
            <Form.Item label="背景颜色">
              <ColorPicker
                value={cfg.bgColor}
                onChange={(_, hex) => update("bgColor", hex)}
                showText
              />
            </Form.Item>
            <Form.Item label="标题颜色">
              <ColorPicker
                value={cfg.titleColor}
                onChange={(_, hex) => update("titleColor", hex)}
                showText
              />
            </Form.Item>
            <Form.Item label="价格颜色">
              <ColorPicker
                value={cfg.priceColor}
                onChange={(_, hex) => update("priceColor", hex)}
                showText
              />
            </Form.Item>
            {renderPaddingConfig()}
          </>
        );

      case "home_article":
        return (
          <>
            <Form.Item label="列表样式">
              <Select
                value={cfg.listStyle}
                onChange={(v) => update("listStyle", v)}
                options={[
                  { value: "list", label: "列表" },
                  { value: "card", label: "卡片" },
                ]}
              />
            </Form.Item>
            <Form.Item label="显示数量">
              <InputNumber
                value={cfg.limit}
                onChange={(v) => update("limit", v)}
                min={1}
                max={20}
                style={{ width: "100%" }}
              />
            </Form.Item>
            <Form.Item label="背景颜色">
              <ColorPicker
                value={cfg.bgColor}
                onChange={(_, hex) => update("bgColor", hex)}
                showText
              />
            </Form.Item>
            <Form.Item label="标题颜色">
              <ColorPicker
                value={cfg.titleColor}
                onChange={(_, hex) => update("titleColor", hex)}
                showText
              />
            </Form.Item>
            {renderPaddingConfig()}
          </>
        );

      case "home_video":
        return (
          <>
            <Form.Item label="视频地址">
              <Input
                value={cfg.videoUrl}
                onChange={(e) => update("videoUrl", e.target.value)}
                placeholder="视频URL"
              />
            </Form.Item>
            <Form.Item label="封面图">
              <UploadPicture
                value={cfg.coverImg}
                onChange={(url) => update("coverImg", url as string)}
                limit={1}
              />
            </Form.Item>
            <Form.Item label="自动播放">
              <Switch
                checked={cfg.autoplay}
                onChange={(v) => update("autoplay", v)}
              />
            </Form.Item>
            <Form.Item label="循环播放">
              <Switch checked={cfg.loop} onChange={(v) => update("loop", v)} />
            </Form.Item>
            <Form.Item label="背景颜色">
              <ColorPicker
                value={cfg.bgColor}
                onChange={(_, hex) => update("bgColor", hex)}
                showText
              />
            </Form.Item>
            {renderPaddingConfig()}
          </>
        );

      case "home_tab":
        return (
          <>
            <Form.Item label="激活颜色">
              <ColorPicker
                value={cfg.activeColor}
                onChange={(_, hex) => update("activeColor", hex)}
                showText
              />
            </Form.Item>
            <Form.Item label="默认颜色">
              <ColorPicker
                value={cfg.inactiveColor}
                onChange={(_, hex) => update("inactiveColor", hex)}
                showText
              />
            </Form.Item>
            <Form.Item label="背景颜色">
              <ColorPicker
                value={cfg.bgColor}
                onChange={(_, hex) => update("bgColor", hex)}
                showText
              />
            </Form.Item>
            <Divider>选项卡 ({cfg.tabList?.length || 0})</Divider>
            {(cfg.tabList || []).map((tab: any, i: number) => (
              <div
                key={i}
                style={{
                  marginBottom: 12,
                  padding: 12,
                  background: "#fafafa",
                  borderRadius: 6,
                }}
              >
                <Form.Item label={`选项${i + 1}`} style={{ marginBottom: 8 }}>
                  <Input
                    value={tab.title}
                    onChange={(e) => {
                      const list = [...(cfg.tabList || [])];
                      list[i] = { ...list[i], title: e.target.value };
                      update("tabList", list);
                    }}
                  />
                </Form.Item>
                {cfg.tabList.length > 1 && (
                  <Button
                    size="small"
                    danger
                    icon={<DeleteOutlined />}
                    onClick={() => {
                      update(
                        "tabList",
                        cfg.tabList.filter((_: any, idx: number) => idx !== i),
                      );
                    }}
                  >
                    删除
                  </Button>
                )}
              </div>
            ))}
            <Button
              type="dashed"
              block
              icon={<PlusOutlined />}
              onClick={() => {
                update("tabList", [
                  ...(cfg.tabList || []),
                  {
                    title: `选项${(cfg.tabList?.length || 0) + 1}`,
                    content: [],
                  },
                ]);
              }}
            >
              添加选项
            </Button>
          </>
        );

      case "home_news_roll":
        return (
          <>
            <Form.Item label="滚动速度">
              <InputNumber
                value={cfg.speed}
                onChange={(v) => update("speed", v)}
                min={10}
                max={200}
                style={{ width: "100%" }}
              />
            </Form.Item>
            <Form.Item label="背景颜色">
              <ColorPicker
                value={cfg.bgColor}
                onChange={(_, hex) => update("bgColor", hex)}
                showText
              />
            </Form.Item>
            <Form.Item label="文字颜色">
              <ColorPicker
                value={cfg.textColor}
                onChange={(_, hex) => update("textColor", hex)}
                showText
              />
            </Form.Item>
            <Form.Item label="图标颜色">
              <ColorPicker
                value={cfg.iconColor}
                onChange={(_, hex) => update("iconColor", hex)}
                showText
              />
            </Form.Item>
            <Divider>消息列表 ({cfg.newsList?.length || 0})</Divider>
            {(cfg.newsList || []).map((item: any, i: number) => (
              <div
                key={i}
                style={{
                  marginBottom: 12,
                  padding: 12,
                  background: "#fafafa",
                  borderRadius: 6,
                }}
              >
                <Form.Item label="文字" style={{ marginBottom: 8 }}>
                  <Input
                    value={item.text}
                    onChange={(e) => {
                      const list = [...(cfg.newsList || [])];
                      list[i] = { ...list[i], text: e.target.value };
                      update("newsList", list);
                    }}
                  />
                </Form.Item>
                <Form.Item label="链接" style={{ marginBottom: 8 }}>
                  <Input
                    value={item.link}
                    onChange={(e) => {
                      const list = [...(cfg.newsList || [])];
                      list[i] = { ...list[i], link: e.target.value };
                      update("newsList", list);
                    }}
                    placeholder="跳转链接"
                  />
                </Form.Item>
                <Button
                  size="small"
                  danger
                  icon={<DeleteOutlined />}
                  onClick={() => {
                    update(
                      "newsList",
                      (cfg.newsList || []).filter(
                        (_: any, idx: number) => idx !== i,
                      ),
                    );
                  }}
                >
                  删除
                </Button>
              </div>
            ))}
            <Button
              type="dashed"
              block
              icon={<PlusOutlined />}
              onClick={() => {
                update("newsList", [
                  ...(cfg.newsList || []),
                  { text: "新消息", link: "" },
                ]);
              }}
            >
              添加消息
            </Button>
            {renderPaddingConfig()}
          </>
        );

      case "home_hotspot":
        return (
          <>
            <Form.Item label="背景图">
              <UploadPicture
                value={cfg.bgImg}
                onChange={(url) => update("bgImg", url as string)}
                limit={1}
              />
            </Form.Item>
            <div
              style={{
                padding: 8,
                background: "#fffbe6",
                borderRadius: 4,
                marginBottom: 12,
                fontSize: 12,
                color: "#faad14",
              }}
            >
              提示：热区功能需要在移动端编辑器中设置点击区域
            </div>
            {renderPaddingConfig()}
          </>
        );

      case "picture_cube":
        return (
          <>
            <Form.Item label="布局列数">
              <Select
                value={cfg.layout}
                onChange={(v) => update("layout", v)}
                options={[
                  { value: "1", label: "1列" },
                  { value: "2", label: "2列" },
                  { value: "3", label: "3列" },
                  { value: "4", label: "4列" },
                ]}
              />
            </Form.Item>
            <Form.Item label="间距">
              <InputNumber
                value={cfg.gap}
                onChange={(v) => update("gap", v)}
                min={0}
                max={20}
                style={{ width: "100%" }}
              />
            </Form.Item>
            <Form.Item label="圆角">
              <InputNumber
                value={cfg.borderRadius}
                onChange={(v) => update("borderRadius", v)}
                min={0}
                max={20}
                style={{ width: "100%" }}
              />
            </Form.Item>
            <Divider>图片列表 ({cfg.imgList?.length || 0})</Divider>
            {(cfg.imgList || []).map((item: any, i: number) => (
              <div
                key={i}
                style={{
                  marginBottom: 12,
                  padding: 12,
                  background: "#fafafa",
                  borderRadius: 6,
                }}
              >
                <Form.Item label={`图片${i + 1}`} style={{ marginBottom: 8 }}>
                  <UploadPicture
                    value={item.img}
                    onChange={(url) => {
                      const list = [...(cfg.imgList || [])];
                      list[i] = { ...list[i], img: url as string };
                      update("imgList", list);
                    }}
                    limit={1}
                  />
                </Form.Item>
                <Form.Item label="跳转链接" style={{ marginBottom: 8 }}>
                  <Input
                    value={item.link}
                    onChange={(e) => {
                      const list = [...(cfg.imgList || [])];
                      list[i] = { ...list[i], link: e.target.value };
                      update("imgList", list);
                    }}
                    placeholder="跳转链接"
                  />
                </Form.Item>
                <Button
                  size="small"
                  danger
                  icon={<DeleteOutlined />}
                  onClick={() => {
                    update(
                      "imgList",
                      (cfg.imgList || []).filter(
                        (_: any, idx: number) => idx !== i,
                      ),
                    );
                  }}
                >
                  删除
                </Button>
              </div>
            ))}
            <Button
              type="dashed"
              block
              icon={<PlusOutlined />}
              onClick={() => {
                update("imgList", [
                  ...(cfg.imgList || []),
                  { img: "", link: "" },
                ]);
              }}
            >
              添加图片
            </Button>
            {renderPaddingConfig()}
          </>
        );

      case "home_merchant":
        return (
          <>
            <Form.Item label="显示Logo">
              <Switch
                checked={cfg.showLogo}
                onChange={(v) => update("showLogo", v)}
              />
            </Form.Item>
            <Form.Item label="显示名称">
              <Switch
                checked={cfg.showName}
                onChange={(v) => update("showName", v)}
              />
            </Form.Item>
            <Form.Item label="显示描述">
              <Switch
                checked={cfg.showDesc}
                onChange={(v) => update("showDesc", v)}
              />
            </Form.Item>
            <Form.Item label="背景颜色">
              <ColorPicker
                value={cfg.bgColor}
                onChange={(_, hex) => update("bgColor", hex)}
                showText
              />
            </Form.Item>
            {renderPaddingConfig()}
          </>
        );

      case "home_footer":
        return (
          <>
            <Form.Item label="背景颜色">
              <ColorPicker
                value={cfg.bgColor}
                onChange={(_, hex) => update("bgColor", hex)}
                showText
              />
            </Form.Item>
            <Form.Item label="激活颜色">
              <ColorPicker
                value={cfg.activeColor}
                onChange={(_, hex) => update("activeColor", hex)}
                showText
              />
            </Form.Item>
            <Form.Item label="默认颜色">
              <ColorPicker
                value={cfg.inactiveColor}
                onChange={(_, hex) => update("inactiveColor", hex)}
                showText
              />
            </Form.Item>
            <Divider>导航项 ({cfg.menuList?.length || 0})</Divider>
            {(cfg.menuList || []).map((item: any, i: number) => (
              <div
                key={i}
                style={{
                  marginBottom: 12,
                  padding: 12,
                  background: "#fafafa",
                  borderRadius: 6,
                }}
              >
                <Form.Item label="标题" style={{ marginBottom: 8 }}>
                  <Input
                    value={item.title}
                    onChange={(e) => {
                      const list = [...(cfg.menuList || [])];
                      list[i] = { ...list[i], title: e.target.value };
                      update("menuList", list);
                    }}
                  />
                </Form.Item>
                <Form.Item label="默认图标" style={{ marginBottom: 8 }}>
                  <UploadPicture
                    value={item.icon}
                    onChange={(url) => {
                      const list = [...(cfg.menuList || [])];
                      list[i] = { ...list[i], icon: url as string };
                      update("menuList", list);
                    }}
                    limit={1}
                  />
                </Form.Item>
                <Form.Item label="激活图标" style={{ marginBottom: 8 }}>
                  <UploadPicture
                    value={item.activeIcon}
                    onChange={(url) => {
                      const list = [...(cfg.menuList || [])];
                      list[i] = { ...list[i], activeIcon: url as string };
                      update("menuList", list);
                    }}
                    limit={1}
                  />
                </Form.Item>
                <Form.Item label="跳转链接" style={{ marginBottom: 8 }}>
                  <Input
                    value={item.link}
                    onChange={(e) => {
                      const list = [...(cfg.menuList || [])];
                      list[i] = { ...list[i], link: e.target.value };
                      update("menuList", list);
                    }}
                    placeholder="跳转链接"
                  />
                </Form.Item>
              </div>
            ))}
          </>
        );

      default:
        return (
          <div style={{ padding: 16, color: "#999", textAlign: "center" }}>
            该组件暂无可配置项
          </div>
        );
    }
  };

  return (
    <>
      <div
        style={{
          width: 300,
          height: "100%",
          overflow: "auto",
          borderLeft: "1px solid #f0f0f0",
          background: "#fff",
        }}
      >
        <div
          style={{
            padding: "12px 16px",
            fontWeight: 600,
            borderBottom: "1px solid #f0f0f0",
          }}
        >
          {activeComponent.cname} 设置
        </div>
        <div style={{ padding: 16 }}>
          <Form layout="vertical" size="small">
            {renderConfig()}
          </Form>
        </div>
      </div>
      <GoodList
        open={goodsModalOpen}
        onCancel={() => setGoodsModalOpen(false)}
        onOk={handleGoodsSelect}
        multiple
      />
    </>
  );
};

export default RightPanel;
