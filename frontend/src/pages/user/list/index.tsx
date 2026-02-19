import React, { useState, useEffect, useCallback, useRef } from 'react';
import {
  Card, Table, Form, Input, Select, DatePicker, Button, Space, Avatar, Tag, Tabs,
  Switch, message, Modal, Dropdown, InputNumber, Radio, Image, Descriptions, Cascader, Drawer,
} from 'antd';
import {
  SearchOutlined, ReloadOutlined, UserOutlined, DownOutlined, UpOutlined,
  SettingOutlined,
} from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import type { MenuProps } from 'antd';
import {
  userListApi, userUpdateApi, userInfoApi, groupListApi, levelListApi, tagListApi,
  groupPiApi, tagPiApi, foundsApi, updateSpreadApi, updatePhoneApi,
  userLevelUpdateApi, infobyconditionApi, topdetailApi,
} from '@/api/user';
import { spreadClearApi } from '@/api/distribution';
import { couponUserApi, integralListApi } from '@/api/marketing';
import { cityListTree } from '@/api/logistics';
import CouponList from '@/components/CouponList';
import UserListDialog from '@/components/UserList';
import dayjs from 'dayjs';

const { RangePicker } = DatePicker;

const sexMap: Record<number, string> = { 0: '未知', 1: '男', 2: '女', 3: '保密' };
const isPromoterMap: Record<number, string> = { 0: '普通用户', 1: '推广员' };
const userTypeMap: Record<string, string> = {
  wechat: '公众号', routine: '小程序', h5: 'H5', iosWx: '微信ios', androidWx: '微信安卓', ios: 'ios',
};

const tabItems = [
  { key: '', label: '全部用户' },
  { key: 'wechat', label: '微信公众号用户' },
  { key: 'routine', label: '微信小程序用户' },
  { key: 'h5', label: 'H5用户' },
];

const payCountOptions = [
  { value: '', label: '全部' },
  { value: '0', label: '0' },
  { value: '1', label: '1+' },
  { value: '2', label: '2+' },
  { value: '3', label: '3+' },
  { value: '4', label: '4+' },
  { value: '5', label: '5+' },
];

const accessTypeOptions = [
  { value: 0, label: '全部' },
  { value: 1, label: '首次访问' },
  { value: 2, label: '时间段访问过' },
  { value: 3, label: '时间段未访问' },
];

const sexOptions = [
  { value: '', label: '全部' },
  { value: 0, label: '未知' },
  { value: 1, label: '男' },
  { value: 2, label: '女' },
  { value: 3, label: '保密' },
];

const countryOptions = [
  { value: '', label: '全部' },
  { value: 'CN', label: '中国' },
  { value: 'OTHER', label: '国外' },
];

const identityOptions = [
  { value: '', label: '全部' },
  { value: 1, label: '推广员' },
  { value: 0, label: '普通用户' },
];

const ALL_COLUMNS = ['ID', '头像', '姓名', '用户等级', '分组', '推荐人', '手机号', '余额', '积分'];

const UserList: React.FC = () => {
  // --- 搜索参数 ---
  const [keywords, setKeywords] = useState('');
  const [levelData, setLevelData] = useState<number[]>([]);
  const [groupData, setGroupData] = useState<number[]>([]);
  const [labelData, setLabelData] = useState<number[]>([]);
  const [country, setCountry] = useState('');
  const [address, setAddress] = useState<string[]>([]);
  const [payCount, setPayCount] = useState('');
  const [dateRange, setDateRange] = useState<[dayjs.Dayjs, dayjs.Dayjs] | null>(null);
  const [accessType, setAccessType] = useState<number>(0);
  const [sex, setSex] = useState<number | string>('');
  const [isPromoter, setIsPromoter] = useState<number | string>('');
  const [collapse, setCollapse] = useState(false);

  // --- Tab ---
  const [loginType, setLoginType] = useState('');

  // --- 列表数据 ---
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [pagination, setPagination] = useState({ current: 1, pageSize: 20, total: 0 });

  // --- 下拉数据 ---
  const [levelList, setLevelList] = useState<any[]>([]);
  const [groupList, setGroupList] = useState<any[]>([]);
  const [labelLists, setLabelLists] = useState<any[]>([]);
  const [cityData, setCityData] = useState<any[]>([]);

  // --- 选择 ---
  const [selectedRowKeys, setSelectedRowKeys] = useState<React.Key[]>([]);
  const [selectedRows, setSelectedRows] = useState<any[]>([]);

  // --- 列显示控制 ---
  const [checkedCities, setCheckedCities] = useState<string[]>(() => {
    try {
      const stored = localStorage.getItem('user_stroge');
      return stored ? JSON.parse(stored) : [...ALL_COLUMNS];
    } catch { return [...ALL_COLUMNS]; }
  });
  const [columnSettingOpen, setColumnSettingOpen] = useState(false);

  // --- 弹窗状态 ---
  const [batchDialogOpen, setBatchDialogOpen] = useState(false);
  const [batchName, setBatchName] = useState<'group' | 'label'>('group');
  const [batchGroupId, setBatchGroupId] = useState<any>(undefined);
  const [batchUserIds, setBatchUserIds] = useState('');
  const [batchLoading, setBatchLoading] = useState(false);

  const [pointDialogOpen, setPointDialogOpen] = useState(false);
  const [pointForm, setPointForm] = useState({ moneyType: 1, moneyValue: 0, integralType: 1, integralValue: 0 });
  const [pointLoading, setPointLoading] = useState(false);
  const [pointUid, setPointUid] = useState(0);

  const [phoneDialogOpen, setPhoneDialogOpen] = useState(false);
  const [phoneValue, setPhoneValue] = useState('');
  const [phoneUid, setPhoneUid] = useState(0);

  const [levelDialogOpen, setLevelDialogOpen] = useState(false);
  const [levelUid, setLevelUid] = useState(0);
  const [levelValue, setLevelValue] = useState<number | undefined>(undefined);

  const [spreadDialogOpen, setSpreadDialogOpen] = useState(false);
  const [spreadUserId, setSpreadUserId] = useState(0);
  const [spreadUid, setSpreadUid] = useState<number | undefined>(undefined);
  const [spreadAvatar, setSpreadAvatar] = useState('');
  const [userPickerOpen, setUserPickerOpen] = useState(false);

  const [couponDialogOpen, setCouponDialogOpen] = useState(false);

  const [detailDialogOpen, setDetailDialogOpen] = useState(false);
  const [detailData, setDetailData] = useState<any>(null);
  const [detailLoading, setDetailLoading] = useState(false);
  const [detailTabKey, setDetailTabKey] = useState('0');
  const [detailTableData, setDetailTableData] = useState<any[]>([]);
  const [detailTableLoading, setDetailTableLoading] = useState(false);
  const [detailPagination, setDetailPagination] = useState({ page: 1, limit: 10, total: 0 });

  const [editDialogOpen, setEditDialogOpen] = useState(false);
  const [editData, setEditData] = useState<any>(null);
  const [editLoading, setEditLoading] = useState(false);
  const [editForm] = Form.useForm();
  const [editLabelData, setEditLabelData] = useState<number[]>([]);

  // --- 加载下拉数据 ---
  useEffect(() => {
    groupListApi({ page: 1, limit: 9999 }).then((res: any) => setGroupList(res?.list || [])).catch(() => {});
    levelListApi().then((res: any) => {
      const data = Array.isArray(res) ? res : res?.list || [];
      setLevelList(data);
      localStorage.setItem('single-admin-levelKey', JSON.stringify(data));
    }).catch(() => {});
    tagListApi({ page: 1, limit: 9999 }).then((res: any) => setLabelLists(res?.list || [])).catch(() => {});
    // 加载城市数据
    cityListTree().then((res: any) => {
      const formatCityData = (data: any[]): any[] => {
        return data.map((item) => ({
          value: item.cityId,
          label: item.name,
          children: item.child && item.child.length > 0 ? formatCityData(item.child) : undefined,
        }));
      };
      setCityData(formatCityData(res || []));
    }).catch(() => {});
  }, []);

  // --- 获取列表 ---
  const fetchList = useCallback(async (page = 1) => {
    setLoading(true);
    try {
      const params: any = {
        page,
        limit: pagination.pageSize,
        userType: loginType || '',
      };
      if (keywords) params.keywords = keywords;
      if (levelData.length) params.level = levelData.join(',');
      if (groupData.length) params.groupId = groupData.join(',');
      if (labelData.length) params.labelId = labelData.join(',');
      if (country) params.country = country;
      if (address.length > 0) {
        params.province = address[0];
        if (address.length > 1) params.city = address[1];
      }
      if (payCount !== '') params.payCount = payCount;
      if (dateRange) params.dateLimit = `${dateRange[0].format('YYYY-MM-DD')},${dateRange[1].format('YYYY-MM-DD')}`;
      if (accessType) params.accessType = accessType;
      if (sex !== '') params.sex = sex;
      if (isPromoter !== '') params.isPromoter = isPromoter;

      const res = await userListApi(params);
      setList(res?.list || []);
      setPagination((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch { message.error('获取用户列表失败'); }
    finally { setLoading(false); }
  }, [keywords, levelData, groupData, labelData, country, payCount, dateRange, accessType, sex, isPromoter, loginType, pagination.pageSize]);

  useEffect(() => { fetchList(1); }, []);

  const handleSearch = () => { fetchList(1); };

  const handleReset = () => {
    setKeywords(''); setLevelData([]); setGroupData([]); setLabelData([]);
    setCountry(''); setAddress([]); setPayCount(''); setDateRange(null); setAccessType(0);
    setSex(''); setIsPromoter('');
  };

  const handleTabChange = (key: string) => {
    setLoginType(key);
    setTimeout(() => fetchList(1), 0);
  };

  // --- 批量设置分组/标签 ---
  const openBatchDialog = (name: 'group' | 'label', row?: any) => {
    setBatchName(name);
    if (row) {
      setBatchUserIds(String(row.uid));
      if (name === 'group') {
        setBatchGroupId(row.groupId ? Number(row.groupId) : undefined);
      } else {
        setBatchGroupId(row.tagId ? row.tagId.split(',').map(Number) : []);
      }
    } else {
      if (selectedRows.length === 0) { message.warning('请选择要设置的用户'); return; }
      setBatchUserIds(selectedRows.map((r) => r.uid).join(','));
      setBatchGroupId(name === 'group' ? undefined : []);
    }
    setBatchDialogOpen(true);
  };

  const handleBatchSubmit = async () => {
    if (batchGroupId === undefined || (Array.isArray(batchGroupId) && !batchGroupId.length)) {
      message.warning(batchName === 'group' ? '请选择用户分组' : '请选择用户标签'); return;
    }
    setBatchLoading(true);
    try {
      if (batchName === 'group') {
        await groupPiApi({ groupId: batchGroupId, id: batchUserIds });
      } else {
        await tagPiApi({ tagId: Array.isArray(batchGroupId) ? batchGroupId.join(',') : batchGroupId, id: batchUserIds });
      }
      message.success('设置成功');
      setBatchDialogOpen(false);
      fetchList(pagination.current);
    } catch { message.error('设置失败'); }
    finally { setBatchLoading(false); }
  };

  // --- 发送优惠券 ---
  const handleSendCoupon = () => {
    if (selectedRows.length === 0) { message.warning('请选择要设置的用户'); return; }
    setCouponDialogOpen(true);
  };

  const handleCouponOk = async (coupons: any[]) => {
    try {
      const userIds = selectedRows.map((r) => r.uid).join(',');
      for (const coupon of coupons) {
        await couponUserApi({ couponId: coupon.id, uid: userIds });
      }
      message.success('发送成功');
      setCouponDialogOpen(false);
      setSelectedRowKeys([]); setSelectedRows([]);
    } catch { message.error('发送失败'); }
  };

  // --- 积分余额 ---
  const openPointDialog = (uid: number) => {
    setPointUid(uid);
    setPointForm({ moneyType: 1, moneyValue: 0, integralType: 1, integralValue: 0 });
    setPointDialogOpen(true);
  };

  const handlePointSubmit = async () => {
    setPointLoading(true);
    try {
      await foundsApi({ ...pointForm, uid: pointUid });
      message.success('设置成功');
      setPointDialogOpen(false);
      fetchList(pagination.current);
    } catch { message.error('设置失败'); }
    finally { setPointLoading(false); }
  };

  // --- 修改手机号 ---
  const openPhoneDialog = (row: any) => {
    setPhoneUid(row.uid);
    setPhoneValue(row.phone || '');
    setPhoneDialogOpen(true);
  };

  const handlePhoneSubmit = async () => {
    if (!phoneValue) { message.warning('请填写手机号'); return; }
    try {
      await updatePhoneApi({ id: phoneUid, phone: phoneValue });
      message.success('修改成功');
      setPhoneDialogOpen(false);
      fetchList(pagination.current);
    } catch { message.error('修改失败'); }
  };

  // --- 修改用户等级 ---
  const openLevelDialog = (uid: number, level: number) => {
    setLevelUid(uid);
    setLevelValue(level || undefined);
    setLevelDialogOpen(true);
  };

  const handleLevelSubmit = async () => {
    if (!levelValue && levelValue !== 0) { message.warning('请选择等级'); return; }
    try {
      await userLevelUpdateApi({ uid: levelUid, levelId: levelValue });
      message.success('设置成功');
      setLevelDialogOpen(false);
      fetchList(pagination.current);
    } catch { message.error('设置失败'); }
  };

  // --- 修改上级推广人 ---
  const openSpreadDialog = (row: any) => {
    setSpreadUserId(row.uid);
    setSpreadUid(undefined);
    setSpreadAvatar('');
    setSpreadDialogOpen(true);
  };

  const handleSpreadSubmit = async () => {
    if (!spreadUid) { message.warning('请选择推广人'); return; }
    try {
      await updateSpreadApi({ userId: spreadUserId, spreadUid });
      message.success('设置成功');
      setSpreadDialogOpen(false);
      fetchList(pagination.current);
    } catch { message.error('设置失败'); }
  };

  const handleClearSpread = (row: any) => {
    Modal.confirm({
      title: '提示',
      content: `解除【${row.nickname}】的上级推广人吗？`,
      onOk: async () => {
        try {
          await spreadClearApi(row.uid);
          message.success('清除成功');
          fetchList(pagination.current);
        } catch { message.error('清除失败'); }
      },
    });
  };

  // --- 用户详情 ---
  const detailUidRef = React.useRef(0);
  const [detailUid, setDetailUid] = useState(0);

  const openDetail = async (uid: number) => {
    detailUidRef.current = uid;
    setDetailUid(uid);
    setDetailLoading(true);
    setDetailDialogOpen(true);
    setDetailTabKey('0');
    setDetailTableData([]);
    setDetailPagination({ page: 1, limit: 10, total: 0 });
    try {
      const res = await topdetailApi({ userId: uid });
      setDetailData(res?.user || res);
    } catch { message.error('获取详情失败'); }
    finally { setDetailLoading(false); }
  };

  const detailRequestId = React.useRef(0);

  const fetchDetailTabData = async (type: number, page = 1, limit = 10, uid?: number) => {
    const userId = uid || detailUidRef.current;
    if (!userId) return;
    const requestId = ++detailRequestId.current;
    setDetailTableLoading(true);
    try {
      let res: any;
      if (type === 1) {
        // 积分明细
        res = await integralListApi({ limit, page, _t: Date.now() }, { uid: userId });
      } else {
        // 消费记录(0), 签到记录(2), 持有优惠券(3), 余额变动(4), 好友关系(5)
        res = await infobyconditionApi({ type, userId, page, limit, _t: Date.now() });
      }
      // 只处理最新请求的响应，防止旧请求覆盖
      if (requestId !== detailRequestId.current) return;
      setDetailTableData(res?.list || []);
      setDetailPagination((p) => ({ ...p, page, total: res?.total || 0 }));
    } catch (e) { console.error('获取详情tab数据失败', e); }
    finally {
      if (requestId === detailRequestId.current) setDetailTableLoading(false);
    }
  };

  const handleDetailTabChange = (key: string) => {
    setDetailTabKey(key);
    setDetailTableData([]);
    setDetailPagination({ page: 1, limit: 10, total: 0 });
    const type = Number(key) - 1;
    if (Number(key) > 0) {
      fetchDetailTabData(type, 1, 10);
    }
  };

  // --- 编辑用户 ---
  const openEdit = async (uid: number) => {
    setEditLoading(true);
    setEditDialogOpen(true);
    try {
      const res = await userInfoApi({ id: uid });
      const data = {
        id: res?.uid || uid,
        mark: res?.mark || '',
        addres: res?.addres || '',
        groupId: res?.groupId ? Number(res.groupId) : undefined,
        isPromoter: res?.isPromoter ?? false,
        status: res?.status ?? false,
      };
      setEditData(data);
      editForm.setFieldsValue(data);
      setEditLabelData(res?.tagId ? res.tagId.split(',').map(Number) : []);
    } catch { message.error('获取用户信息失败'); }
    finally { setEditLoading(false); }
  };

  const handleEditSubmit = async () => {
    try {
      const values = await editForm.validateFields();
      values.tagId = editLabelData.join(',');
      await userUpdateApi({ id: values.id || editData?.id }, values);
      message.success('编辑成功');
      setEditDialogOpen(false);
      fetchList(pagination.current);
    } catch { /* validation or api error */ }
  };

  // --- 列设置 ---
  const handleColumnSave = () => {
    localStorage.setItem('user_stroge', JSON.stringify(checkedCities));
    setColumnSettingOpen(false);
    message.success('保存成功');
  };

  // --- 更多操作菜单 ---
  const getMoreMenuItems = (record: any): MenuProps['items'] => {
    const items: MenuProps['items'] = [
      { key: 'point', label: '积分余额', onClick: () => openPointDialog(record.uid) },
      { key: 'group', label: '设置分组', onClick: () => openBatchDialog('group', record) },
      { key: 'label', label: '设置标签', onClick: () => openBatchDialog('label', record) },
      { key: 'phone', label: '修改手机号', onClick: () => openPhoneDialog(record) },
      { key: 'level', label: '修改用户等级', onClick: () => openLevelDialog(record.uid, record.level) },
      { key: 'spread', label: '修改上级推广人', onClick: () => openSpreadDialog(record) },
    ];
    if (record.spreadUid && record.spreadUid > 0) {
      items.push({ key: 'clearSpread', label: '清除上级推广人', onClick: () => handleClearSpread(record) });
    }
    return items;
  };

  // --- 获取等级名称 ---
  const getLevelName = (level: any) => {
    if (level === null || level === undefined || level === '' || level === 0) return '-';
    const item = levelList.find((l: any) => String(l.id) === String(level));
    return item ? item.name : '-';
  };

  // --- 表格列定义 ---
  const columns: ColumnsType<any> = [
    ...(checkedCities.includes('ID') ? [{ title: 'ID', dataIndex: 'uid', width: 80 }] : []),
    ...(checkedCities.includes('头像') ? [{
      title: '头像', dataIndex: 'avatar', width: 80,
      render: (v: string) => <Image src={v} width={36} height={36} style={{ borderRadius: 4 }} fallback="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mN8/+F/PQAJpAN42kRrYAAAAABJRU5ErkJggg==" />,
    }] : []),
    ...(checkedCities.includes('姓名') ? [{
      title: '姓名', width: 160,
      render: (_: any, record: any) => <span>{record.nickname || '-'} | {sexMap[record.sex] || '未知'}</span>,
    }] : []),
    ...(checkedCities.includes('用户等级') ? [{
      title: '用户等级', width: 100,
      render: (_: any, record: any) => <span>{getLevelName(record.level)}</span>,
    }] : []),
    ...(checkedCities.includes('分组') ? [{
      title: '分组', dataIndex: 'groupName', width: 100,
      render: (v: string) => v || '-',
    }] : []),
    ...(checkedCities.includes('推荐人') ? [{
      title: '推荐人', dataIndex: 'spreadNickname', width: 130,
      render: (v: string) => v || '-',
    }] : []),
    ...(checkedCities.includes('手机号') ? [{
      title: '手机号', dataIndex: 'phone', width: 120,
      render: (v: string) => v || '-',
    }] : []),
    ...(checkedCities.includes('余额') ? [{
      title: '余额', dataIndex: 'nowMoney', width: 100,
    }] : []),
    ...(checkedCities.includes('积分') ? [{
      title: '积分', dataIndex: 'integral', width: 100,
    }] : []),
    {
      title: () => (
        <Space>
          <span>操作</span>
          <SettingOutlined style={{ cursor: 'pointer' }} onClick={() => setColumnSettingOpen(!columnSettingOpen)} />
        </Space>
      ),
      width: 180, fixed: 'right' as const,
      render: (_: any, record: any) => (
        <Space size={0} split={<span style={{ color: '#dcdfe6', margin: '0 6px' }}>|</span>}>
          <a onClick={() => openDetail(record.uid)}>详情</a>
          <a onClick={() => openEdit(record.uid)}>编辑</a>
          <Dropdown menu={{ items: getMoreMenuItems(record) }} trigger={['click']}>
            <a onClick={(e) => e.preventDefault()}>更多 <DownOutlined style={{ fontSize: 10 }} /></a>
          </Dropdown>
        </Space>
      ),
    },
  ];

  // --- 展开行 ---
  const expandedRowRender = (record: any) => (
    <Descriptions size="small" column={3} style={{ fontSize: 12 }}>
      <Descriptions.Item label="身份">{isPromoterMap[record.isPromoter] || '-'}</Descriptions.Item>
      <Descriptions.Item label="首次访问">{record.createTime || '-'}</Descriptions.Item>
      <Descriptions.Item label="近次访问">{record.lastLoginTime || '-'}</Descriptions.Item>
      <Descriptions.Item label="手机号">{record.phone || '-'}</Descriptions.Item>
      <Descriptions.Item label="标签">{record.tagName || '-'}</Descriptions.Item>
      <Descriptions.Item label="地址">{record.addres || '-'}</Descriptions.Item>
      <Descriptions.Item label="备注" span={3}>{record.mark || '-'}</Descriptions.Item>
    </Descriptions>
  );

  // --- 搜索表单 ---
  const selWidth = { width: 200 };
  const formItemStyle = { marginBottom: 16 };

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 14 }}>
      {/* 搜索区域 */}
      <Card bodyStyle={{ padding: '20px 20px 4px' }}>
        <Form size="small" labelCol={{ style: { width: 75 } }}>
          <div style={{ display: 'flex', flexWrap: 'wrap' }}>
            <Form.Item label="用户搜索" style={formItemStyle}>
              <Input placeholder="请输入姓名或手机号" value={keywords} onChange={(e) => setKeywords(e.target.value)}
                onPressEnter={handleSearch} allowClear style={selWidth} />
            </Form.Item>
            <Form.Item label="用户等级" style={formItemStyle}>
              <Select mode="multiple" value={levelData} onChange={setLevelData} placeholder="请选择"
                allowClear showSearch optionFilterProp="label" style={selWidth}
                options={levelList.map((l: any) => ({ value: l.id, label: l.name }))} />
            </Form.Item>
            {!collapse && (
              <Form.Item label=" " colon={false} style={formItemStyle}>
                <Space>
                  <Button type="primary" onClick={handleSearch}>搜索</Button>
                  <Button onClick={handleReset}>重置</Button>
                  <a onClick={() => setCollapse(true)}>展开 <DownOutlined /></a>
                </Space>
              </Form.Item>
            )}
          </div>

          {collapse && (
            <>
              <div style={{ display: 'flex', flexWrap: 'wrap' }}>
                <Form.Item label="用户分组" style={formItemStyle}>
                  <Select mode="multiple" value={groupData} onChange={setGroupData} placeholder="请选择"
                    allowClear showSearch optionFilterProp="label" style={selWidth}
                    options={groupList.map((g: any) => ({ value: g.id, label: g.groupName }))} />
                </Form.Item>
                <Form.Item label="用户标签" style={formItemStyle}>
                  <Select mode="multiple" value={labelData} onChange={setLabelData} placeholder="请选择"
                    allowClear showSearch optionFilterProp="label" style={selWidth}
                    options={labelLists.map((l: any) => ({ value: l.id, label: l.name }))} />
                </Form.Item>
                <Form.Item label="国家" style={formItemStyle}>
                  <Select value={country} onChange={(v) => setCountry(v)} options={countryOptions}
                    style={selWidth} allowClear />
                </Form.Item>
                <Form.Item label="省份" style={formItemStyle}>
                  <Cascader
                    value={address}
                    onChange={(v) => setAddress(v as string[])}
                    options={cityData}
                    placeholder="请选择"
                    style={selWidth}
                    allowClear
                    showSearch
                  />
                </Form.Item>
                <Form.Item label="消费情况" style={formItemStyle}>
                  <Select value={payCount} onChange={setPayCount} options={payCountOptions}
                    style={selWidth} allowClear />
                </Form.Item>
                <Form.Item label="时间选择" style={formItemStyle}>
                  <RangePicker value={dateRange} onChange={(v) => setDateRange(v as [dayjs.Dayjs, dayjs.Dayjs] | null)}
                    style={selWidth} />
                </Form.Item>
                <Form.Item label="访问情况" style={formItemStyle}>
                  <Select value={accessType} onChange={setAccessType} options={accessTypeOptions}
                    style={selWidth} allowClear />
                </Form.Item>
                <Form.Item label="性别" style={formItemStyle}>
                  <Select value={sex} onChange={setSex} options={sexOptions} style={selWidth} />
                </Form.Item>
                <Form.Item label="身份" style={formItemStyle}>
                  <Select value={isPromoter} onChange={setIsPromoter} options={identityOptions} style={selWidth} />
                </Form.Item>
              </div>
              <div style={{ display: 'flex', justifyContent: 'flex-end', marginBottom: 16 }}>
                <Space>
                  <Button type="primary" onClick={handleSearch}>搜索</Button>
                  <Button onClick={handleReset}>重置</Button>
                  <a onClick={() => setCollapse(false)}>收起 <UpOutlined /></a>
                </Space>
              </div>
            </>
          )}
        </Form>
      </Card>

      {/* 列表区域 */}
      <Card bodyStyle={{ paddingTop: 0 }}>
        <Tabs activeKey={loginType} onChange={handleTabChange}
          items={tabItems} style={{ marginBottom: 0 }} />
        <div style={{ marginBottom: 16, display: 'flex', gap: 8 }}>
          <Button type="primary" onClick={handleSendCoupon}>发送优惠券</Button>
          <Button disabled={!selectedRows.length} onClick={() => openBatchDialog('group')}>批量设置分组</Button>
          <Button disabled={!selectedRows.length} onClick={() => openBatchDialog('label')}>批量设置标签</Button>
        </div>

        <div style={{ position: 'relative' }}>
          <Table
            rowKey="uid"
            columns={columns}
            dataSource={list}
            loading={loading}
            size="small"
            scroll={{ x: 1200 }}
            expandable={{ expandedRowRender }}
            rowSelection={{
              selectedRowKeys,
              onChange: (keys, rows) => { setSelectedRowKeys(keys); setSelectedRows(rows); },
            }}
            pagination={{
              ...pagination,
              showSizeChanger: true,
              pageSizeOptions: ['20', '40', '60', '80'],
              showTotal: (t) => `共 ${t} 条`,
              onChange: (p, ps) => {
                setPagination((prev) => ({ ...prev, pageSize: ps || 20 }));
                fetchList(p);
              },
            }}
          />

          {/* 列设置浮层 */}
          {columnSettingOpen && (
            <div style={{
              position: 'absolute', top: 0, right: 0, width: 200, background: '#fff',
              zIndex: 999, boxShadow: '0 0 14px rgba(0,0,0,0.1)', paddingBottom: 15,
            }}>
              <div style={{
                height: 50, padding: '15px 20px', borderBottom: '1px solid #eee',
                display: 'flex', justifyContent: 'space-between', alignItems: 'center',
              }}>
                <label>
                  <input type="checkbox"
                    checked={checkedCities.length === ALL_COLUMNS.length}
                    onChange={(e) => setCheckedCities(e.target.checked ? [...ALL_COLUMNS] : [])}
                  /> 全选
                </label>
                <a onClick={handleColumnSave}>保存</a>
              </div>
              {ALL_COLUMNS.map((col) => (
                <div key={col} style={{ padding: '8px 20px 0' }}>
                  <label>
                    <input type="checkbox" checked={checkedCities.includes(col)}
                      onChange={(e) => {
                        setCheckedCities((prev) =>
                          e.target.checked ? [...prev, col] : prev.filter((c) => c !== col)
                        );
                      }}
                    /> {col}
                  </label>
                </div>
              ))}
            </div>
          )}
        </div>
      </Card>

      {/* 批量设置分组/标签 */}
      <Modal title="设置" open={batchDialogOpen} onCancel={() => setBatchDialogOpen(false)}
        onOk={handleBatchSubmit} confirmLoading={batchLoading} destroyOnClose>
        {batchName === 'group' ? (
          <Form.Item label="用户分组" required>
            <Select value={batchGroupId} onChange={setBatchGroupId} placeholder="请选择分组"
              style={{ width: '100%' }} showSearch optionFilterProp="label"
              options={groupList.map((g: any) => ({ value: g.id, label: g.groupName }))} />
          </Form.Item>
        ) : (
          <Form.Item label="用户标签" required>
            <Select mode="multiple" value={batchGroupId} onChange={setBatchGroupId}
              placeholder="请选择标签" style={{ width: '100%' }} showSearch optionFilterProp="label"
              options={labelLists.map((l: any) => ({ value: l.id, label: l.name }))} />
          </Form.Item>
        )}
      </Modal>

      {/* 积分余额 */}
      <Modal title="积分余额" open={pointDialogOpen} onCancel={() => setPointDialogOpen(false)}
        onOk={handlePointSubmit} confirmLoading={pointLoading} destroyOnClose maskClosable={false}>
        <Form labelCol={{ span: 6 }}>
          <Form.Item label="修改余额" required>
            <Radio.Group value={pointForm.moneyType}
              onChange={(e) => setPointForm((p) => ({ ...p, moneyType: e.target.value }))}>
              <Radio value={1}>增加</Radio>
              <Radio value={2}>减少</Radio>
            </Radio.Group>
          </Form.Item>
          <Form.Item label="余额" required>
            <InputNumber value={pointForm.moneyValue} min={0} max={999999} precision={2} step={0.1}
              onChange={(v) => setPointForm((p) => ({ ...p, moneyValue: v || 0 }))} />
          </Form.Item>
          <Form.Item label="修改积分" required>
            <Radio.Group value={pointForm.integralType}
              onChange={(e) => setPointForm((p) => ({ ...p, integralType: e.target.value }))}>
              <Radio value={1}>增加</Radio>
              <Radio value={2}>减少</Radio>
            </Radio.Group>
          </Form.Item>
          <Form.Item label="积分" required>
            <InputNumber value={pointForm.integralValue} min={0} max={999999}
              onChange={(v) => setPointForm((p) => ({ ...p, integralValue: v || 0 }))} />
          </Form.Item>
        </Form>
      </Modal>

      {/* 修改手机号 */}
      <Modal title="修改手机号" open={phoneDialogOpen} onCancel={() => setPhoneDialogOpen(false)}
        onOk={handlePhoneSubmit} destroyOnClose>
        <Form.Item label="手机号" required>
          <Input value={phoneValue} onChange={(e) => setPhoneValue(e.target.value)} placeholder="请输入手机号" />
        </Form.Item>
      </Modal>

      {/* 修改用户等级 */}
      <Modal title="设置用户等级" open={levelDialogOpen} onCancel={() => setLevelDialogOpen(false)}
        onOk={handleLevelSubmit} destroyOnClose>
        <Form.Item label="用户等级" required>
          <Select value={levelValue} onChange={setLevelValue} placeholder="请选择等级"
            style={{ width: '100%' }}
            options={levelList.map((l: any) => ({ value: l.id, label: l.name }))} />
        </Form.Item>
      </Modal>

      {/* 修改上级推广人 */}
      <Modal title="修改推广人" open={spreadDialogOpen} onCancel={() => setSpreadDialogOpen(false)}
        onOk={handleSpreadSubmit} destroyOnClose width={540}>
        <Form labelCol={{ span: 6 }}>
          <Form.Item label="选择用户">
            <div style={{ cursor: 'pointer' }} onClick={() => setUserPickerOpen(true)}>
              {spreadAvatar ? (
                <Avatar src={spreadAvatar} size={64} />
              ) : (
                <div style={{
                  width: 64, height: 64, border: '1px dashed #d9d9d9', borderRadius: 4,
                  display: 'flex', alignItems: 'center', justifyContent: 'center', color: '#999',
                }}>
                  <UserOutlined style={{ fontSize: 24 }} />
                </div>
              )}
            </div>
          </Form.Item>
        </Form>
      </Modal>

      {/* 用户选择器（推广人） */}
      <UserListDialog open={userPickerOpen} onCancel={() => setUserPickerOpen(false)}
        multiple={false}
        onOk={(rows) => {
          if (rows.length > 0) {
            setSpreadUid(rows[0].uid);
            setSpreadAvatar(rows[0].avatar || '');
          }
          setUserPickerOpen(false);
        }}
      />

      {/* 发送优惠券 */}
      <CouponList open={couponDialogOpen} onCancel={() => setCouponDialogOpen(false)} onOk={handleCouponOk} />

      {/* 用户详情 - Drawer */}
      <Drawer
        open={detailDialogOpen}
        width={1100}
        onClose={() => { setDetailDialogOpen(false); setDetailData(null); setDetailTabKey('0'); }}
        destroyOnClose
        title={detailData ? (
          <div>
            <div style={{ display: 'flex', alignItems: 'center', padding: '0 10px' }}>
              <Avatar src={detailData.avatar} size={60} style={{ marginRight: 15 }} />
              <span style={{ fontWeight: 500, fontSize: 16 }}>{detailData.nickname}</span>
            </div>
            <div style={{ display: 'flex', padding: '20px 15px 24px 10px', borderBottom: '1px dashed #eee', marginTop: 10 }}>
              {[
                { label: '余额', value: detailData.nowMoney },
                { label: '积分', value: detailData.integral },
                { label: '经验', value: detailData.experience },
                { label: '佣金', value: detailData.brokeragePrice },
                { label: '消费次数', value: detailData.payCount },
                { label: '连续签到', value: detailData.signNum },
              ].map((item) => (
                <div key={item.label} style={{ width: 155, fontSize: 14, color: 'rgba(0,0,0,0.85)' }}>
                  <div style={{ marginBottom: 12, fontSize: 13, color: '#666' }}>{item.label}</div>
                  <div>{item.value ?? 0}</div>
                </div>
              ))}
            </div>
          </div>
        ) : '用户详情'}
      >
        {detailLoading ? <div style={{ textAlign: 'center', padding: 40 }}>加载中...</div> : detailData && (
          <div>
            <Tabs
              activeKey={detailTabKey}
              onChange={handleDetailTabChange}
              type="card"
              items={[
                { key: '0', label: '用户信息' },
                { key: '1', label: '消费记录' },
                { key: '2', label: '积分明细' },
                { key: '3', label: '签到记录' },
                { key: '4', label: '持有优惠券' },
                { key: '5', label: '余额变动' },
                { key: '6', label: '好友关系' },
              ]}
              style={{ marginBottom: 16 }}
            />

            {detailTabKey === '0' && (
              <div>
                <div style={{ padding: '25px 0', borderBottom: '1px dashed #eee' }}>
                  <div style={{ paddingLeft: 10, borderLeft: '3px solid #1890ff', fontWeight: 500, fontSize: 14, color: '#303133' }}>基本信息</div>
                  <div style={{ display: 'flex', flexWrap: 'wrap', marginTop: 8 }}>
                    {[['用户ID', detailData.uid], ['真实姓名', detailData.realName || '-'], ['用户账号', detailData.account || '-'],
                      ['用户电话', detailData.phone || '-'], ['生日', detailData.birthday || '-'], ['性别', sexMap[detailData.sex] || '未知'],
                      ['国家', detailData.country === 'CN' ? '中国' : '其他'], ['用户地址', detailData.addres || '-'],
                    ].map(([label, value]) => (
                      <div key={label as string} style={{ width: '30%', display: 'flex', margin: '16px 30px 0 0', fontSize: 13, color: '#606266' }}>
                        <div>{label}：</div><div style={{ flex: 1 }}>{value}</div>
                      </div>
                    ))}
                  </div>
                </div>
                <div style={{ padding: '25px 0', borderBottom: '1px dashed #eee' }}>
                  <div style={{ paddingLeft: 10, borderLeft: '3px solid #1890ff', fontWeight: 500, fontSize: 14, color: '#303133' }}>用户概况</div>
                  <div style={{ display: 'flex', flexWrap: 'wrap', marginTop: 8 }}>
                    {[['创建ip', detailData.addIp || '-'], ['注册类型', userTypeMap[detailData.userType] || '-'],
                      ['添加时间', detailData.createTime || '-'], ['状态', detailData.status === true || detailData.status === 1 ? '正常' : '禁止'],
                      ['最后一次登录ip', detailData.lastIp || '-'], ['最后一次登录时间', detailData.lastLoginTime || '-'],
                    ].map(([label, value]) => (
                      <div key={label as string} style={{ width: '30%', display: 'flex', margin: '16px 30px 0 0', fontSize: 13, color: '#606266' }}>
                        <div>{label}：</div><div style={{ flex: 1 }}>{value}</div>
                      </div>
                    ))}
                  </div>
                </div>
                <div style={{ padding: '25px 0', borderBottom: '1px dashed #eee' }}>
                  <div style={{ paddingLeft: 10, borderLeft: '3px solid #1890ff', fontWeight: 500, fontSize: 14, color: '#303133' }}>推广信息</div>
                  <div style={{ display: 'flex', flexWrap: 'wrap', marginTop: 8 }}>
                    {[['是否为推广员', detailData.isPromoter === true || detailData.isPromoter === 1 ? '是' : '否'],
                      ['下级人数', detailData.spreadCount || '-'], ['成为分销员时间', detailData.promoterTime || '-'],
                      ['上级推广员ID', detailData.spreadUid || '-'], ['绑定上级推广员时间', detailData.spreadTime || '-'],
                    ].map(([label, value]) => (
                      <div key={label as string} style={{ width: '30%', display: 'flex', margin: '16px 30px 0 0', fontSize: 13, color: '#606266' }}>
                        <div>{label}：</div><div style={{ flex: 1 }}>{value}</div>
                      </div>
                    ))}
                  </div>
                </div>
                <div style={{ padding: '25px 0' }}>
                  <div style={{ paddingLeft: 10, borderLeft: '3px solid #1890ff', fontWeight: 500, fontSize: 14, color: '#303133' }}>用户备注</div>
                  <div style={{ margin: '16px 0 0', fontSize: 13, color: '#606266' }}>备注：{detailData.mark || '-'}</div>
                </div>
              </div>
            )}

            {detailTabKey === '1' && <Table rowKey={(r: any, i: any) => r?.orderId || String(i)} dataSource={detailTableData} loading={detailTableLoading} size="small" columns={[{ title: '订单ID', dataIndex: 'orderId', width: 100 }, { title: '收货人', dataIndex: 'realName' }, { title: '商品数量', dataIndex: 'totalNum' }, { title: '商品总价', dataIndex: 'totalPrice' }, { title: '实付金额', dataIndex: 'payPrice' }, { title: '交易完成时间', dataIndex: 'payTime' }]} pagination={{ current: detailPagination.page, pageSize: detailPagination.limit, total: detailPagination.total, showSizeChanger: true, pageSizeOptions: ['10','20','30','40'], showTotal: (t: number) => `共 ${t} 条`, onChange: (p: number, ps: number) => { setDetailPagination((prev) => ({ ...prev, limit: ps })); fetchDetailTabData(0, p, ps); } }} />}

            {detailTabKey === '2' && <Table rowKey={(r: any, i: any) => String(i)} dataSource={detailTableData} loading={detailTableLoading} size="small" columns={[{ title: '来源/用途', dataIndex: 'title', width: 100 }, { title: '积分变化', dataIndex: 'integral' }, { title: '变化后积分', dataIndex: 'balance' }, { title: '日期', dataIndex: 'updateTime' }, { title: '备注', dataIndex: 'mark' }]} pagination={{ current: detailPagination.page, pageSize: detailPagination.limit, total: detailPagination.total, showSizeChanger: true, pageSizeOptions: ['10','20','30','40'], showTotal: (t: number) => `共 ${t} 条`, onChange: (p: number, ps: number) => { setDetailPagination((prev) => ({ ...prev, limit: ps })); fetchDetailTabData(1, p, ps); } }} />}

            {detailTabKey === '3' && <Table rowKey={(r: any, i: any) => String(i)} dataSource={detailTableData} loading={detailTableLoading} size="small" columns={[{ title: '动作', dataIndex: 'title', width: 100 }, { title: '获得积分', dataIndex: 'number' }, { title: '签到时间', dataIndex: 'createTime' }]} pagination={{ current: detailPagination.page, pageSize: detailPagination.limit, total: detailPagination.total, showSizeChanger: true, pageSizeOptions: ['10','20','30','40'], showTotal: (t: number) => `共 ${t} 条`, onChange: (p: number, ps: number) => { setDetailPagination((prev) => ({ ...prev, limit: ps })); fetchDetailTabData(2, p, ps); } }} />}

            {detailTabKey === '4' && <Table rowKey={(r: any, i: any) => String(i)} dataSource={detailTableData} loading={detailTableLoading} size="small" columns={[{ title: '优惠券名称', dataIndex: 'name' }, { title: '面值', dataIndex: 'money' }, { title: '有效期', dataIndex: 'endTime' }, { title: '最低消费额', dataIndex: 'minPrice' }, { title: '兑换时间', dataIndex: 'updateTime' }]} pagination={{ current: detailPagination.page, pageSize: detailPagination.limit, total: detailPagination.total, showSizeChanger: true, pageSizeOptions: ['10','20','30','40'], showTotal: (t: number) => `共 ${t} 条`, onChange: (p: number, ps: number) => { setDetailPagination((prev) => ({ ...prev, limit: ps })); fetchDetailTabData(3, p, ps); } }} />}

            {detailTabKey === '5' && <Table rowKey={(r: any, i: any) => String(i)} dataSource={detailTableData} loading={detailTableLoading} size="small" columns={[{ title: '变动金额', dataIndex: 'number' }, { title: '变动后', dataIndex: 'balance' }, { title: '类型', dataIndex: 'title' }, { title: '创建时间', dataIndex: 'add_time' }, { title: '备注', dataIndex: 'mark' }]} pagination={{ current: detailPagination.page, pageSize: detailPagination.limit, total: detailPagination.total, showSizeChanger: true, pageSizeOptions: ['10','20','30','40'], showTotal: (t: number) => `共 ${t} 条`, onChange: (p: number, ps: number) => { setDetailPagination((prev) => ({ ...prev, limit: ps })); fetchDetailTabData(4, p, ps); } }} />}

            {detailTabKey === '6' && <Table rowKey={(r: any, i: any) => r?.uid || String(i)} dataSource={detailTableData} loading={detailTableLoading} size="small" columns={[{ title: 'ID', dataIndex: 'uid' }, { title: '昵称', dataIndex: 'nickname' }, { title: '等级', dataIndex: 'level' }, { title: '加入时间', dataIndex: 'createTime' }]} pagination={{ current: detailPagination.page, pageSize: detailPagination.limit, total: detailPagination.total, showSizeChanger: true, pageSizeOptions: ['10','20','30','40'], showTotal: (t: number) => `共 ${t} 条`, onChange: (p: number, ps: number) => { setDetailPagination((prev) => ({ ...prev, limit: ps })); fetchDetailTabData(5, p, ps); } }} />}
          </div>
        )}
      </Drawer>

      {/* 编辑用户 - 匹配Vue edit.vue */}
      <Modal title="编辑" open={editDialogOpen} onCancel={() => setEditDialogOpen(false)}
        onOk={handleEditSubmit} width={600} destroyOnClose confirmLoading={editLoading}>
        {editData && (
          <Form form={editForm} labelCol={{ span: 5 }} initialValues={editData}>
            <Form.Item label="用户编号" name="id" rules={[{ required: true, message: '请输入用户编号' }]}>
              <Input disabled />
            </Form.Item>
            <Form.Item label="用户地址" name="addres" rules={[{ required: true, message: '请输入用户地址' }]}>
              <Input placeholder="请输入用户地址" />
            </Form.Item>
            <Form.Item label="用户备注" name="mark" rules={[{ required: true, message: '请输入用户备注' }]}>
              <Input.TextArea placeholder="请输入备注" rows={3} />
            </Form.Item>
            <Form.Item label="用户分组" name="groupId" rules={[{ required: true, message: '请选择用户分组' }]}>
              <Select placeholder="请选择" allowClear showSearch optionFilterProp="label" style={{ width: '100%' }}
                options={groupList.map((g: any) => ({ value: g.id, label: g.groupName }))} />
            </Form.Item>
            <Form.Item label="用户标签">
              <Select mode="multiple" value={editLabelData} onChange={setEditLabelData}
                placeholder="请选择" allowClear showSearch optionFilterProp="label" style={{ width: '100%' }}
                options={labelLists.map((l: any) => ({ value: l.id, label: l.name }))} />
            </Form.Item>
            <Form.Item label="推广员" name="isPromoter" rules={[{ required: true, message: '请选择状态' }]}>
              <Radio.Group>
                <Radio value={true}>开启</Radio>
                <Radio value={false}>关闭</Radio>
              </Radio.Group>
            </Form.Item>
            <Form.Item label="状态" name="status" rules={[{ required: true, message: '请选择状态' }]}>
              <Radio.Group>
                <Radio value={true}>开启</Radio>
                <Radio value={false}>关闭</Radio>
              </Radio.Group>
            </Form.Item>
          </Form>
        )}
      </Modal>
    </div>
  );
};

export default UserList;
