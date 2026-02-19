import dayjs from 'dayjs';

export function filterEmpty(val: unknown): string {
  return val ? String(val) : '-';
}

export function formatDate(time: number): string {
  if (time === 0) return '-';
  return dayjs(time * 1000).format('YYYY-MM-DD HH:mm');
}

export function filterYesOrNo(value: unknown): string {
  return value ? '是' : '否';
}

export function filterShowOrHide(value: unknown): string {
  return value ? '显示' : '不显示';
}

export function filterYesOrNoIs(value: unknown): string {
  return value ? '否' : '是';
}

export function paidFilter(status: string | boolean): string {
  const map: Record<string, string> = { true: '已支付', false: '未支付' };
  return map[String(status)] || '-';
}

export function orderStatusFilter(status: number): string {
  const map: Record<number, string> = {
    0: '待发货', 1: '待收货', 2: '已收货', 3: '待评价', [-2]: '已退款', [-1]: '退款中',
  };
  return map[status] || '-';
}

export function refundStatusFilter(status: number): string {
  const map: Record<number, string> = { 0: '未退款', 1: '申请中', 2: '已退款', 3: '退款中' };
  return map[status] || '-';
}

export function payTypeFilter(status: string): string {
  const map: Record<string, string> = { weixin: '微信', alipay: '支付宝', yue: '余额' };
  return map[status] || '-';
}

export function orderTypeFilter(status: number): string {
  const map: Record<number, string> = { 1: '普通订单', 2: '核销订单' };
  return map[status] || '-';
}

export function typeFilter(status: string): string {
  const map: Record<string, string> = { wechat: '微信用户', routine: '小程序用户', h5: 'H5用户' };
  return map[status] || '-';
}

export function filterIsPromoter(status: string | boolean): string {
  return String(status) === 'true' ? '推广员' : '普通用户';
}

export function keywordStatusFilter(status: string): string {
  const map: Record<string, string> = { text: '文字消息', image: '图片消息', news: '图文消息', voice: '声音消息' };
  return map[status] || '-';
}

export function couponUserTypeFilter(status: number): string {
  const map: Record<number, string> = { 1: '通用券', 2: '商品券', 3: '品类券' };
  return map[status] || '-';
}

export function couponTypeFilter(status: number): string {
  const map: Record<number, string> = { 1: '手动领取', 2: '新人券', 3: '赠送券' };
  return map[status] || '-';
}

export function payStatusFilter(status: string | boolean): string {
  return String(status) === 'true' ? '已支付' : '未支付';
}

export function extractTypeFilter(status: string): string {
  const map: Record<string, string> = { bank: '银行卡', alipay: '支付宝', weixin: '微信' };
  return map[status] || '-';
}

export function extractStatusFilter(status: number | string): string {
  const map: Record<string, string> = { '-1': '已拒绝', '0': '审核中', '1': '已提现' };
  return map[String(status)] || '-';
}

export function bargainStatusFilter(status: number): string {
  const map: Record<number, string> = { 1: '进行中', 2: '未完成', 3: '已成功' };
  return map[status] || '-';
}

export function groupStatusFilter(status: number): string {
  const map: Record<number, string> = { 1: '进行中', 2: '已成功', 3: '未完成' };
  return map[status] || '-';
}

export function toThousandFilter(num: number): string {
  return (+num || 0).toString().replace(/^-?\d+/g, (m) => m.replace(/(?=(?!\b)(\d{3})+$)/g, ','));
}

export function numberFormatter(num: number, digits = 2): string {
  const si = [
    { value: 1e18, symbol: 'E' }, { value: 1e15, symbol: 'P' },
    { value: 1e12, symbol: 'T' }, { value: 1e9, symbol: 'G' },
    { value: 1e6, symbol: 'M' }, { value: 1e3, symbol: 'k' },
  ];
  for (const s of si) {
    if (num >= s.value) {
      return (num / s.value).toFixed(digits).replace(/\.0+$|(\.[0-9]*[1-9])0+$/, '$1') + s.symbol;
    }
  }
  return num.toString();
}
