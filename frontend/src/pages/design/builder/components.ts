// Component type definitions for the page builder

export interface DiyComponent {
  name: string;
  cname: string;
  icon: string;
  category: 'basic' | 'marketing' | 'tool';
  singleton?: boolean; // can only add once
  fixed?: 'top' | 'bottom'; // fixed position
  conflicts?: string[]; // mutually exclusive components
  defaultConfig: Record<string, any>;
}

export const componentCategories = [
  { key: 'basic', label: '基础组件' },
  { key: 'marketing', label: '营销组件' },
  { key: 'tool', label: '工具组件' },
];

export const componentList: DiyComponent[] = [
  // Basic Components
  {
    name: 'banner',
    cname: '轮播图',
    icon: 'PictureOutlined',
    category: 'basic',
    defaultConfig: {
      swiperConfig: {
        imgList: [
          { img: 'https://via.placeholder.com/750x360?text=Banner+1', link: '' },
          { img: 'https://via.placeholder.com/750x360?text=Banner+2', link: '' },
        ],
        speed: 3000,
        autoplay: true,
      },
      bgColor: '#ffffff',
      padding: { top: 0, bottom: 0, left: 0, right: 0 },
      borderRadius: 0,
    },
  },
  {
    name: 'home_title',
    cname: '标题栏',
    icon: 'FontSizeOutlined',
    category: 'basic',
    defaultConfig: {
      title: '标题文字',
      subtitle: '',
      titleColor: '#333333',
      subtitleColor: '#999999',
      bgColor: '#ffffff',
      textAlign: 'left',
      fontSize: 16,
      padding: { top: 10, bottom: 10, left: 12, right: 12 },
      showMore: false,
      moreLink: '',
    },
  },
  {
    name: 'home_menu',
    cname: '图文导航',
    icon: 'AppstoreOutlined',
    category: 'basic',
    defaultConfig: {
      menuList: [
        { img: 'https://via.placeholder.com/100?text=Menu1', title: '菜单1', link: '' },
        { img: 'https://via.placeholder.com/100?text=Menu2', title: '菜单2', link: '' },
        { img: 'https://via.placeholder.com/100?text=Menu3', title: '菜单3', link: '' },
        { img: 'https://via.placeholder.com/100?text=Menu4', title: '菜单4', link: '' },
      ],
      rowCount: 4,
      bgColor: '#ffffff',
      titleColor: '#333333',
      padding: { top: 10, bottom: 10, left: 0, right: 0 },
    },
  },
  {
    name: 'search_box',
    cname: '搜索框',
    icon: 'SearchOutlined',
    category: 'basic',
    singleton: true,
    fixed: 'top',
    conflicts: ['home_comb'],
    defaultConfig: {
      placeholder: '搜索商品',
      bgColor: '#ffffff',
      borderColor: '#eeeeee',
      borderRadius: 20,
      textAlign: 'left',
    },
  },
  {
    name: 'nav_bar',
    cname: '导航栏',
    icon: 'MenuOutlined',
    category: 'basic',
    singleton: true,
    fixed: 'top',
    conflicts: ['home_comb'],
    defaultConfig: {
      title: '页面标题',
      bgColor: '#ffffff',
      titleColor: '#333333',
      showBack: true,
    },
  },
  {
    name: 'home_footer',
    cname: '底部导航',
    icon: 'LayoutOutlined',
    category: 'basic',
    singleton: true,
    fixed: 'bottom',
    defaultConfig: {
      menuList: [
        { icon: 'https://via.placeholder.com/48?text=Home', activeIcon: '', title: '首页', link: '/pages/index/index' },
        { icon: 'https://via.placeholder.com/48?text=Cat', activeIcon: '', title: '分类', link: '/pages/category/index' },
        { icon: 'https://via.placeholder.com/48?text=Cart', activeIcon: '', title: '购物车', link: '/pages/cart/index' },
        { icon: 'https://via.placeholder.com/48?text=Me', activeIcon: '', title: '我的', link: '/pages/user/index' },
      ],
      bgColor: '#ffffff',
      activeColor: '#e93323',
      inactiveColor: '#999999',
    },
  },
  {
    name: 'home_comb',
    cname: '头部组合',
    icon: 'BlockOutlined',
    category: 'basic',
    singleton: true,
    fixed: 'top',
    conflicts: ['search_box', 'nav_bar'],
    defaultConfig: {
      bgColor: '#e93323',
      titleColor: '#ffffff',
      showSearch: true,
      showLogo: true,
      logoUrl: '',
    },
  },
  {
    name: 'z_auxiliary_box',
    cname: '辅助空白',
    icon: 'BorderOutlined',
    category: 'basic',
    defaultConfig: {
      height: 20,
      bgColor: '#f5f5f5',
    },
  },
  {
    name: 'z_auxiliary_line',
    cname: '辅助线',
    icon: 'LineOutlined',
    category: 'basic',
    defaultConfig: {
      lineColor: '#eeeeee',
      lineStyle: 'solid',
      lineHeight: 1,
      padding: { left: 0, right: 0 },
      margin: { top: 0, bottom: 0 },
    },
  },
  {
    name: 'z_ueditor',
    cname: '富文本',
    icon: 'FileTextOutlined',
    category: 'basic',
    defaultConfig: {
      content: '<p>请输入内容</p>',
      bgColor: '#ffffff',
      padding: { top: 10, bottom: 10, left: 12, right: 12 },
    },
  },
  // Marketing Components
  {
    name: 'home_goods_list',
    cname: '商品列表',
    icon: 'ShoppingOutlined',
    category: 'marketing',
    defaultConfig: {
      listStyle: 'grid', // grid | list | scroll
      columns: 2,
      showPrice: true,
      showName: true,
      showSales: false,
      bgColor: '#f5f5f5',
      cardBgColor: '#ffffff',
      borderRadius: 8,
      padding: { top: 10, bottom: 10, left: 10, right: 10 },
      goodsIds: [],
      sortType: 'default', // default | sales | price
      limit: 10,
    },
  },
  {
    name: 'home_bargain',
    cname: '砍价',
    icon: 'ScissorOutlined',
    category: 'marketing',
    defaultConfig: {
      listStyle: 'scroll',
      limit: 6,
      bgColor: '#ffffff',
      titleColor: '#e93323',
      priceColor: '#e93323',
      padding: { top: 10, bottom: 10, left: 12, right: 12 },
    },
  },
  {
    name: 'home_coupon',
    cname: '优惠券',
    icon: 'TagOutlined',
    category: 'marketing',
    defaultConfig: {
      couponIds: [],
      bgColor: '#ffffff',
      primaryColor: '#e93323',
      padding: { top: 10, bottom: 10, left: 12, right: 12 },
    },
  },
  {
    name: 'home_seckill',
    cname: '秒杀',
    icon: 'ThunderboltOutlined',
    category: 'marketing',
    defaultConfig: {
      listStyle: 'scroll',
      limit: 6,
      bgColor: '#ffffff',
      titleColor: '#e93323',
      priceColor: '#e93323',
      padding: { top: 10, bottom: 10, left: 12, right: 12 },
    },
  },
  {
    name: 'home_group',
    cname: '拼团',
    icon: 'TeamOutlined',
    category: 'marketing',
    defaultConfig: {
      listStyle: 'scroll',
      limit: 6,
      bgColor: '#ffffff',
      titleColor: '#e93323',
      priceColor: '#e93323',
      padding: { top: 10, bottom: 10, left: 12, right: 12 },
    },
  },
  {
    name: 'home_article',
    cname: '文章',
    icon: 'ReadOutlined',
    category: 'marketing',
    defaultConfig: {
      listStyle: 'list',
      limit: 5,
      bgColor: '#ffffff',
      titleColor: '#333333',
      padding: { top: 10, bottom: 10, left: 12, right: 12 },
    },
  },
  {
    name: 'home_video',
    cname: '视频',
    icon: 'PlayCircleOutlined',
    category: 'marketing',
    defaultConfig: {
      videoUrl: '',
      coverImg: '',
      autoplay: false,
      bgColor: '#000000',
      padding: { top: 0, bottom: 0, left: 0, right: 0 },
    },
  },
  {
    name: 'home_tab',
    cname: '选项卡',
    icon: 'SwitcherOutlined',
    category: 'marketing',
    defaultConfig: {
      tabList: [
        { title: '选项1', content: [] },
        { title: '选项2', content: [] },
      ],
      activeColor: '#e93323',
      inactiveColor: '#333333',
      bgColor: '#ffffff',
    },
  },
  {
    name: 'home_news_roll',
    cname: '滚动消息',
    icon: 'NotificationOutlined',
    category: 'marketing',
    defaultConfig: {
      newsList: [
        { text: '这是一条滚动消息', link: '' },
      ],
      speed: 50,
      bgColor: '#fffbe6',
      textColor: '#faad14',
      iconColor: '#faad14',
      padding: { top: 8, bottom: 8, left: 12, right: 12 },
    },
  },
  {
    name: 'home_hotspot',
    cname: '热区',
    icon: 'AimOutlined',
    category: 'marketing',
    defaultConfig: {
      bgImg: '',
      hotAreas: [],
      padding: { top: 0, bottom: 0, left: 0, right: 0 },
    },
  },
  // Tool Components
  {
    name: 'picture_cube',
    cname: '图片魔方',
    icon: 'PicCenterOutlined',
    category: 'tool',
    defaultConfig: {
      layout: '2', // 1 | 2 | 3 | 4 | custom
      imgList: [
        { img: 'https://via.placeholder.com/375x200?text=Image1', link: '' },
        { img: 'https://via.placeholder.com/375x200?text=Image2', link: '' },
      ],
      gap: 4,
      padding: { top: 0, bottom: 0, left: 0, right: 0 },
      borderRadius: 0,
    },
  },
  {
    name: 'home_merchant',
    cname: '店铺信息',
    icon: 'ShopOutlined',
    category: 'tool',
    defaultConfig: {
      showLogo: true,
      showName: true,
      showDesc: true,
      bgColor: '#ffffff',
      padding: { top: 10, bottom: 10, left: 12, right: 12 },
    },
  },
];

// Create a new component instance with unique timestamp
export function createComponentInstance(comp: DiyComponent) {
  const timestamp = Date.now();
  return {
    name: comp.name,
    cname: comp.cname,
    timestamp,
    id: `id${timestamp}`,
    isHide: false,
    fixed: comp.fixed,
    singleton: comp.singleton,
    conflicts: comp.conflicts,
    defaultConfig: JSON.parse(JSON.stringify(comp.defaultConfig)),
  };
}

export type ComponentInstance = ReturnType<typeof createComponentInstance>;

// Get component definition by name
export function getComponentDef(name: string): DiyComponent | undefined {
  return componentList.find((c) => c.name === name);
}
