import React, { useState } from 'react';
import { Modal, Tabs, Input, Select, Form, Button } from 'antd';

interface LinkAddressProps {
  open: boolean;
  onCancel: () => void;
  onOk: (link: { name: string; url: string }) => void;
}

const linkOptions = [
  { label: '商品分类', value: '/pages/goods_cate/goods_cate' },
  { label: '购物车', value: '/pages/order_addcart/order_addcart' },
  { label: '个人中心', value: '/pages/user/index' },
  { label: '领券中心', value: '/pages/users/user_get_coupon/index' },
  { label: '我的收藏', value: '/pages/users/user_goods_collection/index' },
  { label: '我的订单', value: '/pages/users/order_list/index' },
  { label: '新闻资讯', value: '/pages/news_list/index' },
  { label: '地址管理', value: '/pages/users/user_address_list/index' },
  { label: '签到', value: '/pages/users/user_sgin/index' },
  { label: '积分商城', value: '/pages/activity/goods_point/index' },
];

const LinkAddress: React.FC<LinkAddressProps> = ({ open, onCancel, onOk }) => {
  const [activeTab, setActiveTab] = useState('preset');
  const [selectedLink, setSelectedLink] = useState<string>('');
  const [customUrl, setCustomUrl] = useState('');
  const [customName, setCustomName] = useState('');

  const handleOk = () => {
    if (activeTab === 'preset') {
      const found = linkOptions.find((o) => o.value === selectedLink);
      if (!found) return;
      onOk({ name: found.label, url: found.value });
    } else {
      if (!customUrl) return;
      onOk({ name: customName || customUrl, url: customUrl });
    }
  };

  return (
    <Modal title="选择链接" open={open} onCancel={onCancel} onOk={handleOk} width={500} destroyOnClose>
      <Tabs activeKey={activeTab} onChange={setActiveTab} items={[
        {
          key: 'preset',
          label: '预设链接',
          children: (
            <Select style={{ width: '100%' }} placeholder="请选择链接" value={selectedLink} onChange={setSelectedLink}>
              {linkOptions.map((o) => <Select.Option key={o.value} value={o.value}>{o.label}</Select.Option>)}
            </Select>
          ),
        },
        {
          key: 'custom',
          label: '自定义链接',
          children: (
            <div>
              <Input placeholder="链接名称" value={customName} onChange={(e) => setCustomName(e.target.value)} style={{ marginBottom: 12 }} />
              <Input placeholder="链接地址" value={customUrl} onChange={(e) => setCustomUrl(e.target.value)} />
            </div>
          ),
        },
      ]} />
    </Modal>
  );
};

export default LinkAddress;
