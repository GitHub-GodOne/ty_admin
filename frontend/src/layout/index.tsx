import React, { useState, useEffect, useMemo } from 'react';
import { Outlet, useNavigate, useLocation } from 'react-router-dom';
import { Layout as AntLayout, Menu, Dropdown, Button, Avatar, Breadcrumb, Drawer } from 'antd';
import {
  MenuFoldOutlined, MenuUnfoldOutlined, DashboardOutlined, ShopOutlined,
  OrderedListOutlined, UserOutlined, GiftOutlined, MoneyCollectOutlined,
  FileTextOutlined, ShareAltOutlined, SettingOutlined, ToolOutlined,
  BarChartOutlined, LogoutOutlined, WechatOutlined,
  AppstoreOutlined, SkinOutlined,
} from '@ant-design/icons';
import type { MenuProps } from 'antd';
import { useAppStore } from '@/stores/useAppStore';
import { useAuthStore } from '@/stores/useAuthStore';
import { useThemeConfigStore } from '@/stores/useThemeConfigStore';
import { useTagsViewStore } from '@/stores/useTagsViewStore';
import HeaderSearch from '@/components/HeaderSearch';
import Screenfull from '@/components/Screenfull';
import ThemeDrawer from '@/components/ThemeDrawer';
import TagsView from './TagsView';

const { Header, Sider, Content } = AntLayout;
const MOBILE_BREAKPOINT = 992;

// __CONTINUE_MENU__

const staticMenuItems: MenuProps['items'] = [
  { key: '/dashboard', icon: <DashboardOutlined />, label: '主页' },
  { key: '/store', icon: <ShopOutlined />, label: '商品', children: [
    { key: '/store/index', label: '商品管理' }, { key: '/store/sort', label: '商品分类' },
    { key: '/store/attr', label: '商品规格' }, { key: '/store/comment', label: '商品评论' },
  ]},
  { key: '/order', icon: <OrderedListOutlined />, label: '订单', children: [
    { key: '/order/index', label: '订单管理' },
  ]},
  { key: '/user', icon: <UserOutlined />, label: '用户', children: [
    { key: '/user/index', label: '用户管理' }, { key: '/user/grade', label: '用户等级' },
    { key: '/user/label', label: '用户标签' }, { key: '/user/group', label: '用户分组' },
  ]},
  { key: '/marketing', icon: <GiftOutlined />, label: '营销', children: [
    { key: '/marketing/coupon', label: '优惠券', children: [
      { key: '/marketing/coupon/list', label: '优惠券列表' },
      { key: '/marketing/coupon/record', label: '领取记录' },
    ]},
    { key: '/marketing/bargain', label: '砍价管理', children: [
      { key: '/marketing/bargain/goods', label: '砍价商品' },
      { key: '/marketing/bargain/list', label: '砍价列表' },
    ]},
    { key: '/marketing/groupBuy', label: '拼团管理', children: [
      { key: '/marketing/groupBuy/goods', label: '拼团商品' },
      { key: '/marketing/groupBuy/list', label: '拼团列表' },
    ]},
    { key: '/marketing/seckill', label: '秒杀管理', children: [
      { key: '/marketing/seckill/config', label: '秒杀配置' },
      { key: '/marketing/seckill/list', label: '秒杀商品' },
    ]},
    { key: '/marketing/integral', label: '积分', children: [
      { key: '/marketing/integral/config', label: '积分配置' },
      { key: '/marketing/integral/log', label: '积分日志' },
    ]},
    { key: '/marketing/atmosphere/list', label: '活动氛围' },
    { key: '/marketing/border/list', label: '活动边框' },
  ]},
  { key: '/financial', icon: <MoneyCollectOutlined />, label: '财务', children: [
    { key: '/financial/commission', label: '财务操作', children: [
      { key: '/financial/commission/template', label: '申请提现' },
    ]},
    { key: '/financial/record', label: '财务记录', children: [
      { key: '/financial/record/charge', label: '充值记录' },
      { key: '/financial/record/monitor', label: '资金监控' },
      { key: '/financial/record/brokerage', label: '佣金记录' },
    ]},
  ]},
  { key: '/content', icon: <FileTextOutlined />, label: '内容', children: [
    { key: '/content/article', label: '文章管理' }, { key: '/content/category', label: '文章分类' },
  ]},
  { key: '/distribution', icon: <ShareAltOutlined />, label: '分销', children: [
    { key: '/distribution/index', label: '分销员管理' }, { key: '/distribution/config', label: '分销配置' },
  ]},
  { key: '/operation', icon: <SettingOutlined />, label: '设置', children: [
    { key: '/operation/system', label: '系统设置', children: [
      { key: '/operation/config', label: '系统配置' },
      { key: '/operation/notification', label: '系统通知' },
    ]},
    { key: '/operation/auth', label: '权限管理', children: [
      { key: '/operation/admin', label: '管理员列表' },
      { key: '/operation/role', label: '角色管理' },
      { key: '/operation/menu', label: '菜单管理' },
    ]},
    { key: '/operation/deliverGoods', label: '发货设置', children: [
      { key: '/operation/deliverGoods/freightSet', label: '运费模板' },
      { key: '/operation/deliverGoods/takeGoods', label: '提货设置', children: [
        { key: '/operation/deliverGoods/takeGoods/deliveryAddress', label: '提货点' },
        { key: '/operation/deliverGoods/takeGoods/collateOrder', label: '核销订单' },
        { key: '/operation/deliverGoods/takeGoods/collateUser', label: '核销员' },
      ]},
    ]},
  ]},
  { key: '/appSetting', icon: <WechatOutlined />, label: '应用', children: [
    { key: '/appSetting/publicAccount', label: '公众号', children: [
      { key: '/appSetting/publicAccount/wxMenus', label: '微信菜单' },
      { key: '/appSetting/publicAccount/wxReply', label: '自动回复', children: [
        { key: '/appSetting/publicAccount/wxReply/follow', label: '微信关注回复' },
        { key: '/appSetting/publicAccount/wxReply/keyword', label: '关键字回复' },
        { key: '/appSetting/publicAccount/wxReply/replyIndex', label: '无效关键词回复' },
      ]},
    ]},
  ]},
  { key: '/maintain', icon: <ToolOutlined />, label: '维护', children: [
    { key: '/maintain/material', label: '素材管理' }, { key: '/maintain/formConfig', label: '表单配置' },
    { key: '/maintain/clearCache', label: '清除缓存' },
    { key: '/maintain/schedule', label: '定时任务', children: [
      { key: '/maintain/schedule/job', label: '定时任务' },
      { key: '/maintain/schedule/log', label: '任务日志' },
    ]},
    { key: '/maintain/devconfig', label: '开发配置', children: [
      { key: '/maintain/devconfig/configCategory', label: '配置分类' },
      { key: '/maintain/devconfig/combinedData', label: '组合数据' },
      { key: '/maintain/devconfig/formConfig', label: '表单配置' },
    ]},
    { key: '/maintain/logistics', label: '物流设置', children: [
      { key: '/maintain/logistics/companyList', label: '物流公司' },
      { key: '/maintain/logistics/cityList', label: '城市数据' },
    ]},
  ]},
  { key: '/statistic', icon: <BarChartOutlined />, label: '统计', children: [
    { key: '/statistic/product', label: '商品统计' }, { key: '/statistic/user', label: '用户统计' },
    { key: '/statistic/trade', label: '交易统计' },
  ]},
  { key: '/design', icon: <AppstoreOutlined />, label: '装修', children: [
    { key: '/design/index', label: '页面管理' },
  ]},
];

function buildTitleMap(items: MenuProps['items'], map: Record<string, string> = {}): Record<string, string> {
  items?.forEach((item: any) => {
    if (item?.key && item?.label) map[item.key] = item.label;
    if (item?.children) buildTitleMap(item.children, map);
  });
  return map;
}
const titleMap = buildTitleMap(staticMenuItems);

// __CONTINUE_COMPONENT__

const Layout: React.FC = () => {
  const navigate = useNavigate();
  const location = useLocation();
  const collapsed = !useAppStore((s) => s.sidebar.opened);
  const toggleSidebar = useAppStore((s) => s.toggleSidebar);
  const closeSidebar = useAppStore((s) => s.closeSidebar);
  const toggleDevice = useAppStore((s) => s.toggleDevice);
  const device = useAppStore((s) => s.device);
  const userName = useAuthStore((s) => s.name);
  const logoutAction = useAuthStore((s) => s.logoutAction);
  const themeConfig = useThemeConfigStore((s) => s.themeConfig);
  const setThemeConfig = useThemeConfigStore((s) => s.setThemeConfig);
  const addView = useTagsViewStore((s) => s.addView);
  const [drawerOpen, setDrawerOpen] = useState(false);

  const searchOptions = useMemo(() => {
    const opts: { label: string; value: string; path: string }[] = [];
    const walk = (items: MenuProps['items'], parentLabel = '') => {
      items?.forEach((item: any) => {
        if (item?.children) walk(item.children, item.label || '');
        else if (item?.key && item?.label) opts.push({ label: parentLabel ? `${parentLabel} > ${item.label}` : item.label, value: item.key, path: item.key });
      });
    };
    walk(staticMenuItems);
    return opts;
  }, []);

  useEffect(() => {
    const handleResize = () => {
      const isMobile = window.innerWidth < MOBILE_BREAKPOINT;
      toggleDevice(isMobile ? 'mobile' : 'desktop');
      if (isMobile) closeSidebar(true);
    };
    handleResize();
    window.addEventListener('resize', handleResize);
    return () => window.removeEventListener('resize', handleResize);
  }, []);

  useEffect(() => {
    const path = location.pathname;
    const title = titleMap[path] || path.split('/').pop() || '';
    addView({ path, title, meta: { title, affix: path === '/dashboard' } });
    if (device === 'mobile') setDrawerOpen(false);
  }, [location.pathname]);

  const [openKeys, setOpenKeys] = useState<string[]>(() => {
    const parts = location.pathname.split('/').filter(Boolean);
    const keys: string[] = [];
    for (let i = 1; i <= parts.length; i++) {
      keys.push('/' + parts.slice(0, i).join('/'));
    }
    return keys;
  });
  const selectedKeys = [location.pathname];
  const handleMenuClick: MenuProps['onClick'] = ({ key }) => navigate(key);
  const handleLogout = async () => { await logoutAction(); navigate('/login'); };
  const userMenuItems: MenuProps['items'] = [
    { key: 'logout', icon: <LogoutOutlined />, label: '退出登录', onClick: handleLogout },
  ];
  const isMobile = device === 'mobile';
  const layoutMode = isMobile ? 'classic' : themeConfig.layout;

// __CONTINUE_RENDER__

  // 右侧工具栏（所有布局共用）
  const headerRight = (
    <div style={{ display: 'flex', alignItems: 'center', gap: 12 }}>
      <HeaderSearch options={searchOptions} onSelect={(path) => navigate(path)} />
      <Screenfull />
      <Button type="text" icon={<SkinOutlined />} style={{ fontSize: 16, color: themeConfig.topBarColor }}
        onClick={() => setThemeConfig({ isDrawer: true })} />
      <Dropdown menu={{ items: userMenuItems }} placement="bottomRight">
        <div style={{ cursor: 'pointer', display: 'flex', alignItems: 'center', gap: 6, color: themeConfig.topBarColor }}>
          <Avatar size="small" icon={<UserOutlined />} />
          <span style={{ display: isMobile ? 'none' : 'inline' }}>{userName || 'Admin'}</span>
        </div>
      </Dropdown>
    </div>
  );

  // Logo
  const logo = themeConfig.isShowLogo ? (
    <div style={{
      height: 48, display: 'flex', alignItems: 'center', justifyContent: 'center',
      color: themeConfig.menuBarColor, fontWeight: 'bold',
      fontSize: collapsed && !isMobile ? 14 : 18,
      borderBottom: '1px solid rgba(255,255,255,0.1)',
      whiteSpace: 'nowrap', overflow: 'hidden', flexShrink: 0,
    }}>
      {collapsed && !isMobile ? 'TA' : themeConfig.globalTitle}
    </div>
  ) : null;

  // 侧边栏菜单
  const siderMenu = (
    <Menu theme="dark" mode="inline" selectedKeys={selectedKeys}
      openKeys={isMobile || collapsed ? undefined : openKeys}
      onOpenChange={setOpenKeys} onClick={handleMenuClick} items={staticMenuItems}
      style={{ background: themeConfig.menuBar, borderRight: 0, color: themeConfig.menuBarColor }} />
  );

  // 移动端抽屉
  const mobileDrawer = isMobile && (
    <Drawer placement="left" open={drawerOpen} onClose={() => setDrawerOpen(false)}
      width={210} styles={{ body: { padding: 0, background: themeConfig.menuBar }, header: { display: 'none' } }}>
      <div style={{ display: 'flex', flexDirection: 'column', height: '100%', background: themeConfig.menuBar }}>
        {logo}
        <div style={{ flex: 1, overflow: 'auto' }}>{siderMenu}</div>
      </div>
    </Drawer>
  );

// __CONTINUE_LAYOUTS__

  const headerStyle: React.CSSProperties = {
    padding: '0 16px', background: themeConfig.topBar, color: themeConfig.topBarColor,
    display: 'flex', alignItems: 'center', justifyContent: 'space-between',
    boxShadow: '0 1px 4px rgba(0,0,0,0.08)', height: 48, lineHeight: '48px',
    position: 'sticky', top: 0, zIndex: 9,
  };

  const contentArea = (
    <>
      {themeConfig.isTagsview && <TagsView />}
      <Content style={{ margin: isMobile ? 8 : 16, padding: isMobile ? 12 : 16, background: '#f5f5f5', minHeight: 280 }}>
        <Outlet />
      </Content>
    </>
  );

  const rootStyle: React.CSSProperties = {
    minHeight: '100vh',
    filter: themeConfig.isGrayscale ? 'grayscale(1)' : themeConfig.isInvert ? 'invert(0.8)' : undefined,
  };

  const collapseBtn = (
    <Button type="text"
      icon={isMobile ? <MenuUnfoldOutlined /> : (collapsed ? <MenuUnfoldOutlined /> : <MenuFoldOutlined />)}
      onClick={() => isMobile ? setDrawerOpen(true) : toggleSidebar()}
      style={{ fontSize: 16, color: themeConfig.topBarColor }} />
  );

  // ===== 横向布局 (transverse): 顶部菜单，无侧边栏 =====
  if (layoutMode === 'transverse') {
    return (
      <AntLayout style={rootStyle}>
        {mobileDrawer}
        <Header style={{ ...headerStyle, gap: 12 }}>
          {themeConfig.isShowLogo && (
            <div style={{ fontWeight: 'bold', fontSize: 18, color: themeConfig.topBarColor, marginRight: 8, whiteSpace: 'nowrap' }}>
              {themeConfig.globalTitle}
            </div>
          )}
          <div style={{ flex: 1, overflow: 'hidden' }}>
            <Menu mode="horizontal" selectedKeys={selectedKeys} onClick={handleMenuClick}
              items={staticMenuItems}
              style={{ background: 'transparent', borderBottom: 'none', color: themeConfig.topBarColor, lineHeight: '46px' }} />
          </div>
          {headerRight}
        </Header>
        {contentArea}
        <ThemeDrawer />
      </AntLayout>
    );
  }

  // ===== 分栏布局 (columns): 左侧图标栏 + 子菜单侧边栏 =====
  if (layoutMode === 'columns') {
    // 一级菜单（只显示图标）
    const topLevelKey = '/' + location.pathname.split('/').filter(Boolean)[0];
    const topLevelItems: MenuProps['items'] = staticMenuItems?.map((item: any) => ({
      key: item.key, icon: item.icon, label: '',
    }));
    // 当前选中一级菜单的子菜单
    const activeParent = (staticMenuItems as any[])?.find((item: any) => item.key === topLevelKey);
    const subMenuItems: MenuProps['items'] = activeParent?.children || [];
    const parentLabel = activeParent?.label || '';

    return (
      <AntLayout style={rootStyle}>
        {mobileDrawer}
        {!isMobile && (
          <div style={{ display: 'flex', position: 'fixed', left: 0, top: 0, bottom: 0, zIndex: 10 }}>
            {/* 一级图标栏 */}
            <div style={{ width: 64, background: themeConfig.menuBar, display: 'flex', flexDirection: 'column', alignItems: 'center', paddingTop: 4 }}>
              {themeConfig.isShowLogo && (
                <div style={{ height: 48, display: 'flex', alignItems: 'center', justifyContent: 'center',
                  color: themeConfig.menuBarColor, fontWeight: 'bold', fontSize: 14, width: '100%',
                  borderBottom: '1px solid rgba(255,255,255,0.1)' }}>TA</div>
              )}
              <Menu theme="dark" mode="inline" selectedKeys={[topLevelKey]}
                onClick={({ key }) => {
                  const parent = (staticMenuItems as any[])?.find((item: any) => item.key === key);
                  if (parent?.children?.length) navigate(parent.children[0].key);
                  else navigate(key);
                }}
                items={topLevelItems} inlineCollapsed
                style={{ background: themeConfig.menuBar, borderRight: 0, width: 64 }} />
            </div>
            {/* 二级子菜单栏 */}
            {subMenuItems && subMenuItems.length > 0 && (
              <div style={{ width: 150, background: '#fff', borderRight: '1px solid #f0f0f0', overflow: 'auto' }}>
                <div style={{ height: 48, display: 'flex', alignItems: 'center', justifyContent: 'center',
                  fontWeight: 600, fontSize: 14, borderBottom: '1px solid #f0f0f0' }}>{parentLabel}</div>
                <Menu mode="inline" selectedKeys={selectedKeys} onClick={handleMenuClick}
                  items={subMenuItems} style={{ borderRight: 0 }} />
              </div>
            )}
          </div>
        )}
        <AntLayout style={{ marginLeft: isMobile ? 0 : (subMenuItems && subMenuItems.length > 0 ? 214 : 64), transition: 'margin-left 0.2s' }}>
          <Header style={headerStyle}>
            <div style={{ display: 'flex', alignItems: 'center', gap: 12 }}>
              {isMobile && collapseBtn}
              {themeConfig.isBreadcrumb && !isMobile && <Breadcrumb items={getBreadcrumb(location.pathname)} />}
            </div>
            {headerRight}
          </Header>
          {contentArea}
        </AntLayout>
        <ThemeDrawer />
      </AntLayout>
    );
  }

  // ===== 默认布局 (defaults): 侧边栏深色 + 顶栏白色 =====
  // ===== 经典布局 (classic): 侧边栏深色 + 顶栏深色 logo =====
  const showLogoInHeader = layoutMode === 'classic';

  return (
    <AntLayout style={rootStyle}>
      {!isMobile && (
        <Sider trigger={null} collapsible collapsed={collapsed} width={210} collapsedWidth={64}
          style={{ background: themeConfig.menuBar, overflow: 'auto', height: '100vh',
            position: 'fixed', left: 0, top: 0, bottom: 0, zIndex: 10,
            display: 'flex', flexDirection: 'column' }}>
          {!showLogoInHeader && logo}
          <div style={{ flex: 1, overflow: 'auto' }}>{siderMenu}</div>
        </Sider>
      )}
      {mobileDrawer}
      <AntLayout style={{ marginLeft: isMobile ? 0 : (collapsed ? 64 : 210), transition: 'margin-left 0.2s' }}>
        <Header style={headerStyle}>
          <div style={{ display: 'flex', alignItems: 'center', gap: 12 }}>
            {showLogoInHeader && !isMobile && themeConfig.isShowLogo && (
              <div style={{ fontWeight: 'bold', fontSize: collapsed ? 14 : 16, color: themeConfig.topBarColor, whiteSpace: 'nowrap' }}>
                {collapsed ? 'TA' : themeConfig.globalTitle}
              </div>
            )}
            {collapseBtn}
            {themeConfig.isBreadcrumb && !isMobile && <Breadcrumb items={getBreadcrumb(location.pathname)} />}
          </div>
          {headerRight}
        </Header>
        {contentArea}
      </AntLayout>
      <ThemeDrawer />
    </AntLayout>
  );
};

function getBreadcrumb(pathname: string): { title: string }[] {
  const parts = pathname.split('/').filter(Boolean);
  return parts.map((p) => ({ title: titleMap['/' + parts.slice(0, parts.indexOf(p) + 1).join('/')] || p }));
}

export default Layout;
