export const page = {
  limit: [20, 40, 60, 80, 100],
  page: 1,
  layout: 'total, sizes, prev, pager, next, jumper',
};

export const categoryType = [
  { name: '产品分类', value: 1, shortName: '产品' },
  { name: '附件分类', value: 2, shortName: '附件' },
  { name: '文章分类', value: 3, shortName: '文章' },
  { name: '设置分类', value: 4, shortName: '设置' },
  { name: '菜单分类', value: 5, shortName: '菜单' },
  { name: '配置分类', value: 6, shortName: '配置' },
  { name: '秒杀配置', value: 7, shortName: '秒杀' },
];

export const roleListStatus = [
  { label: '全部', value: '' },
  { label: '显示', value: 1 },
  { label: '不显示', value: 0 },
];

export const switchStatus = [
  { label: '开启', value: 1 },
  { label: '关闭', value: 0 },
];

export const deletedOrNormal = [
  { label: '正常', value: 0 },
  { label: '已删除', value: 1 },
];

export const configCategory = [
  { label: '系统', value: '0' },
  { label: '应用', value: '1' },
  { label: '支付', value: '2' },
  { label: '其他', value: '3' },
];

export const fromList = {
  title: '选择时间',
  custom: true,
  fromTxt: [
    { text: '全部', val: '' },
    { text: '今天', val: 'today' },
    { text: '昨天', val: 'yesterday' },
    { text: '最近7天', val: 'lately7' },
    { text: '最近30天', val: 'lately30' },
    { text: '本月', val: 'month' },
    { text: '本年', val: 'year' },
  ],
};

export const timeList = {
  title: '选择时间',
  custom: true,
  fromTxt: [
    { text: '昨天', val: '' },
    { text: '最近7天', val: 'lately7' },
    { text: '最近30天', val: 'lately30' },
  ],
};
