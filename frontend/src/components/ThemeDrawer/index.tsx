import React from 'react';
import { Drawer, Divider, Switch, Tooltip, ColorPicker, Select } from 'antd';
import { CloseOutlined, CheckOutlined } from '@ant-design/icons';
import { useThemeConfigStore } from '@/stores/useThemeConfigStore';

const layoutOptions = [
  { value: 'defaults', label: '默认布局' },
  { value: 'classic', label: '经典布局' },
  { value: 'transverse', label: '横向布局' },
  { value: 'columns', label: '分栏布局' },
];

const animationOptions = [
  { value: 'opacitys', label: '渐隐' },
  { value: 'slide-right', label: '右滑' },
  { value: 'slide-left', label: '左滑' },
  { value: 'slide-bottom', label: '下滑' },
];

const tagsStyleOptions = [
  { value: 'tags-style-one', label: '风格一' },
  { value: 'tags-style-two', label: '风格二' },
  { value: 'tags-style-three', label: '风格三' },
  { value: 'tags-style-four', label: '风格四' },
  { value: 'tags-style-five', label: '风格五' },
];

const presetColors = [
  '#0256FF', '#009688', '#536dfe', '#ff5c93', '#ee4f12',
  '#0096c7', '#9c27b0', '#ff9800', '#FF3D68', '#00C1D4',
];

// __CONTINUE_HERE__

const ThemeDrawer: React.FC = () => {
  const { themeConfig, setThemeConfig } = useThemeConfigStore();

  const SectionTitle: React.FC<{ title: string }> = ({ title }) => (
    <Divider style={{ margin: '16px 0 12px', fontSize: 13, fontWeight: 600 }}>{title}</Divider>
  );

  return (
    <Drawer
      title="主题设置"
      placement="right"
      width={300}
      open={themeConfig.isDrawer}
      onClose={() => setThemeConfig({ isDrawer: false })}
      styles={{ body: { padding: '0 16px 16px' } }}
    >
      {/* 主题颜色 */}
      <SectionTitle title="主题颜色" />
      <div style={{ display: 'flex', gap: 8, flexWrap: 'wrap', marginBottom: 12 }}>
        {presetColors.map((color) => (
          <Tooltip key={color} title={color}>
            <div onClick={() => setThemeConfig({ primary: color })}
              style={{
                width: 28, height: 28, borderRadius: 4, background: color, cursor: 'pointer',
                display: 'flex', alignItems: 'center', justifyContent: 'center',
                border: themeConfig.primary === color ? '2px solid #333' : '2px solid transparent',
              }}>
              {themeConfig.primary === color && <CheckOutlined style={{ color: '#fff', fontSize: 12 }} />}
            </div>
          </Tooltip>
        ))}
      </div>
      <div style={{ display: 'flex', alignItems: 'center', gap: 8, marginBottom: 8 }}>
        <span style={{ fontSize: 13 }}>自定义颜色：</span>
        <ColorPicker value={themeConfig.primary}
          onChange={(_, hex) => setThemeConfig({ primary: hex })} size="small" />
      </div>

      {/* 菜单栏颜色 */}
      <SectionTitle title="菜单设置" />
      <SettingRow label="菜单背景">
        <ColorPicker value={themeConfig.menuBar}
          onChange={(_, hex) => setThemeConfig({ menuBar: hex, menuBgColor: hex })} size="small" />
      </SettingRow>
      <SettingRow label="菜单文字">
        <ColorPicker value={themeConfig.menuBarColor}
          onChange={(_, hex) => setThemeConfig({ menuBarColor: hex })} size="small" />
      </SettingRow>
      <SettingRow label="顶栏背景">
        <ColorPicker value={themeConfig.topBar}
          onChange={(_, hex) => setThemeConfig({ topBar: hex })} size="small" />
      </SettingRow>
      <SettingRow label="顶栏文字">
        <ColorPicker value={themeConfig.topBarColor}
          onChange={(_, hex) => setThemeConfig({ topBarColor: hex })} size="small" />
      </SettingRow>

{/* __CONTINUE_LAYOUT__ */}

      {/* 布局设置 */}
      <SectionTitle title="布局设置" />
      <div style={{ display: 'flex', gap: 12, marginBottom: 12, flexWrap: 'wrap' }}>
        {layoutOptions.map((opt) => {
          const active = themeConfig.layout === opt.value;
          const borderColor = active ? themeConfig.primary : '#e8e8e8';
          const bg = active ? `${themeConfig.primary}10` : '#fafafa';
          return (
            <Tooltip key={opt.value} title={opt.label}>
              <div onClick={() => setThemeConfig({ layout: opt.value })}
                style={{ width: 58, borderRadius: 4, cursor: 'pointer', border: `2px solid ${borderColor}`, background: bg, padding: 4, textAlign: 'center' }}>
                <LayoutIcon type={opt.value} primary={themeConfig.primary} />
                <div style={{ fontSize: 10, marginTop: 2, color: active ? themeConfig.primary : '#999' }}>{opt.label}</div>
              </div>
            </Tooltip>
          );
        })}
      </div>

      {/* 界面显示 */}
      <SectionTitle title="界面显示" />
      <SettingRow label="显示 Logo">
        <Switch size="small" checked={themeConfig.isShowLogo}
          onChange={(v) => setThemeConfig({ isShowLogo: v })} />
      </SettingRow>
      <SettingRow label="面包屑导航">
        <Switch size="small" checked={themeConfig.isBreadcrumb}
          onChange={(v) => setThemeConfig({ isBreadcrumb: v })} />
      </SettingRow>
      <SettingRow label="标签栏">
        <Switch size="small" checked={themeConfig.isTagsview}
          onChange={(v) => setThemeConfig({ isTagsview: v })} />
      </SettingRow>
      <SettingRow label="固定 Header">
        <Switch size="small" checked={themeConfig.isFixedHeader}
          onChange={(v) => setThemeConfig({ isFixedHeader: v })} />
      </SettingRow>
      <SettingRow label="页脚">
        <Switch size="small" checked={themeConfig.isFooter}
          onChange={(v) => setThemeConfig({ isFooter: v })} />
      </SettingRow>
      <SettingRow label="灰色模式">
        <Switch size="small" checked={themeConfig.isGrayscale}
          onChange={(v) => setThemeConfig({ isGrayscale: v })} />
      </SettingRow>
      <SettingRow label="色弱模式">
        <Switch size="small" checked={themeConfig.isInvert}
          onChange={(v) => setThemeConfig({ isInvert: v })} />
      </SettingRow>

      {/* 其他设置 */}
      <SectionTitle title="其他设置" />
      <SettingRow label="切换动画">
        <Select size="small" value={themeConfig.animation} style={{ width: 100 }}
          onChange={(v) => setThemeConfig({ animation: v })}
          options={animationOptions} />
      </SettingRow>
      <SettingRow label="标签风格">
        <Select size="small" value={themeConfig.tagsStyle} style={{ width: 100 }}
          onChange={(v) => setThemeConfig({ tagsStyle: v })}
          options={tagsStyleOptions} />
      </SettingRow>
    </Drawer>
  );
};

// 布局缩略图
const LayoutIcon: React.FC<{ type: string; primary: string }> = ({ type, primary }) => {
  const dark = '#273352';
  const light = '#e8e8e8';
  const h = 32;
  if (type === 'defaults') {
    // 默认：左侧深色窄栏 + 右上浅色顶栏 + 右下内容
    return (
      <div style={{ display: 'flex', height: h, borderRadius: 2, overflow: 'hidden' }}>
        <div style={{ width: 10, background: dark }} />
        <div style={{ flex: 1, display: 'flex', flexDirection: 'column' }}>
          <div style={{ height: 8, background: '#fff', borderBottom: `1px solid ${light}` }} />
          <div style={{ flex: 1, background: light }} />
        </div>
      </div>
    );
  }
  if (type === 'classic') {
    // 经典：左侧深色宽栏 + 右上浅色顶栏 + 右下内容
    return (
      <div style={{ display: 'flex', height: h, borderRadius: 2, overflow: 'hidden' }}>
        <div style={{ width: 16, background: dark }} />
        <div style={{ flex: 1, display: 'flex', flexDirection: 'column' }}>
          <div style={{ height: 8, background: '#fff', borderBottom: `1px solid ${light}` }} />
          <div style={{ flex: 1, background: light }} />
        </div>
      </div>
    );
  }
  if (type === 'transverse') {
    // 横向：顶部深色栏 + 下方内容
    return (
      <div style={{ display: 'flex', flexDirection: 'column', height: h, borderRadius: 2, overflow: 'hidden' }}>
        <div style={{ height: 8, background: dark }} />
        <div style={{ flex: 1, background: light }} />
      </div>
    );
  }
  // columns 分栏：左侧窄图标栏 + 子菜单栏 + 右侧内容
  return (
    <div style={{ display: 'flex', height: h, borderRadius: 2, overflow: 'hidden' }}>
      <div style={{ width: 8, background: dark }} />
      <div style={{ width: 12, background: '#fff', borderRight: `1px solid ${light}` }} />
      <div style={{ flex: 1, display: 'flex', flexDirection: 'column' }}>
        <div style={{ height: 8, background: '#fff', borderBottom: `1px solid ${light}` }} />
        <div style={{ flex: 1, background: light }} />
      </div>
    </div>
  );
};

const SettingRow: React.FC<{ label: string; children: React.ReactNode }> = ({ label, children }) => (
  <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 10 }}>
    <span style={{ fontSize: 13 }}>{label}</span>
    {children}
  </div>
);

export default ThemeDrawer;
