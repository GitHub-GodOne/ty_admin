import React, { lazy } from 'react';
import { createBrowserRouter, Navigate } from 'react-router-dom';
import LazyLoad from './LazyLoad';
import AuthGuard from './AuthGuard';

const Layout = lazy(() => import('@/layout'));
const Login = lazy(() => import('@/pages/login'));
const Dashboard = lazy(() => import('@/pages/dashboard'));
const NotFound = lazy(() => import('@/pages/error/404'));
const NotAuth = lazy(() => import('@/pages/error/401'));

// Store
const StoreList = lazy(() => import('@/pages/store/index'));
const StoreSort = lazy(() => import('@/pages/store/sort'));
const StoreAttr = lazy(() => import('@/pages/store/storeAttr'));
const StoreComment = lazy(() => import('@/pages/store/storeComment'));
const StoreCreate = lazy(() => import('@/pages/store/createStore'));

// Order
const OrderList = lazy(() => import('@/pages/order/index'));

// User
const UserList = lazy(() => import('@/pages/user/list'));
const UserGrade = lazy(() => import('@/pages/user/grade'));
const UserTag = lazy(() => import('@/pages/user/group'));
const UserGroup = lazy(() => import('@/pages/user/userGroup'));

// Marketing
const CouponList = lazy(() => import('@/pages/marketing/coupon/list'));
const CouponCreate = lazy(() => import('@/pages/marketing/coupon/create'));
const CouponRecord = lazy(() => import('@/pages/marketing/coupon/record'));
const BargainGoods = lazy(() => import('@/pages/marketing/bargain/goods'));
const BargainList = lazy(() => import('@/pages/marketing/bargain/list'));
const BargainCreate = lazy(() => import('@/pages/marketing/bargain/create'));
const GroupGoods = lazy(() => import('@/pages/marketing/groupBuy/goods'));
const GroupList = lazy(() => import('@/pages/marketing/groupBuy/list'));
const GroupCreate = lazy(() => import('@/pages/marketing/groupBuy/create'));
const SeckillConfig = lazy(() => import('@/pages/marketing/seckill/config'));
const SeckillList = lazy(() => import('@/pages/marketing/seckill/list'));
const SeckillCreate = lazy(() => import('@/pages/marketing/seckill/create'));
const IntegralConfig = lazy(() => import('@/pages/marketing/integral/config'));
const IntegralLog = lazy(() => import('@/pages/marketing/integral/log'));
const AtmosphereList = lazy(() => import('@/pages/marketing/atmosphere/list'));
const AtmosphereAdd = lazy(() => import('@/pages/marketing/atmosphere/add'));
// Financial
const FinancialExtract = lazy(() => import('@/pages/financial/extract'));
const FinancialRecharge = lazy(() => import('@/pages/financial/recharge'));
const FinancialMonitor = lazy(() => import('@/pages/financial/monitor'));
const FinancialCommission = lazy(() => import('@/pages/financial/commission'));
const FinancialBrokerage = lazy(() => import('@/pages/financial/brokerage'));

// Content
const ArticleList = lazy(() => import('@/pages/content/article'));
const ArticleCreate = lazy(() => import('@/pages/content/articleCreate'));
const ArticleCategory = lazy(() => import('@/pages/content/category'));

// Distribution
const DistributionList = lazy(() => import('@/pages/distribution/index'));
const DistributionConfig = lazy(() => import('@/pages/distribution/config'));

// Operation
const SystemRole = lazy(() => import('@/pages/operation/role'));
const SystemAdmin = lazy(() => import('@/pages/operation/admin'));
const SystemConfig = lazy(() => import('@/pages/operation/config'));
const SystemMenu = lazy(() => import('@/pages/operation/menu'));
const SystemNotification = lazy(() => import('@/pages/operation/notification'));
const SystemStorePoint = lazy(() => import('@/pages/operation/storePoint'));
const SystemStoreStaff = lazy(() => import('@/pages/operation/storeStaff'));
const CollateOrder = lazy(() => import('@/pages/operation/collateOrder'));

// Maintain
const MaterialList = lazy(() => import('@/pages/maintain/material'));
const FormConfig = lazy(() => import('@/pages/maintain/formConfig'));
const ClearCache = lazy(() => import('@/pages/maintain/clearCache'));
const ScheduleJobList = lazy(() => import('@/pages/maintain/schedule/list'));
const ScheduleLogList = lazy(() => import('@/pages/maintain/schedule/logList'));

// DevConfig
const ConfigCategory = lazy(() => import('@/pages/maintain/devconfig/configCategory'));
const CombinedData = lazy(() => import('@/pages/maintain/devconfig/combinedData'));

// SystemSetting - Logistics
const ExpressList = lazy(() => import('@/pages/systemSetting/logistics/express'));
const CityList = lazy(() => import('@/pages/systemSetting/logistics/city'));
const FreightTemplate = lazy(() => import('@/pages/systemSetting/deliverGoods/freight'));

// Statistics
const StatProduct = lazy(() => import('@/pages/statistic/product'));
const StatUser = lazy(() => import('@/pages/statistic/user'));
const StatTrade = lazy(() => import('@/pages/statistic/trade'));

// Design
const DesignList = lazy(() => import('@/pages/design/index'));
const PageBuilder = lazy(() => import('@/pages/design/builder/index'));

function withAuth(element: React.ReactNode) {
  return <AuthGuard>{element}</AuthGuard>;
}

function lazy_(element: React.ReactNode) {
  return <LazyLoad>{element}</LazyLoad>;
}

export const router = createBrowserRouter([
  { path: '/login', element: lazy_(<Login />) },
  { path: '/401', element: lazy_(<NotAuth />) },
  { path: '/404', element: lazy_(<NotFound />) },
  {
    path: '/',
    element: withAuth(lazy_(<Layout />)),
    children: [
      { index: true, element: <Navigate to="/dashboard" replace /> },
      { path: 'dashboard', element: lazy_(<Dashboard />) },
      // Store
      { path: 'store/index', element: lazy_(<StoreList />) },
      { path: 'store/sort', element: lazy_(<StoreSort />) },
      { path: 'store/attr', element: lazy_(<StoreAttr />) },
      { path: 'store/comment', element: lazy_(<StoreComment />) },
      { path: 'store/list/creatProduct', element: lazy_(<StoreCreate />) },
      // Order
      { path: 'order/index', element: lazy_(<OrderList />) },
      // User
      { path: 'user/index', element: lazy_(<UserList />) },
      { path: 'user/grade', element: lazy_(<UserGrade />) },
      { path: 'user/label', element: lazy_(<UserTag />) },
      { path: 'user/group', element: lazy_(<UserGroup />) },
      // Marketing
      { path: 'marketing/coupon/list', element: lazy_(<CouponList />) },
      { path: 'marketing/coupon/create', element: lazy_(<CouponCreate />) },
      { path: 'marketing/coupon/record', element: lazy_(<CouponRecord />) },
      { path: 'marketing/bargain/goods', element: lazy_(<BargainGoods />) },
      { path: 'marketing/bargain/list', element: lazy_(<BargainList />) },
      { path: 'marketing/bargain/creatBargain', element: lazy_(<BargainCreate />) },
      { path: 'marketing/groupBuy/goods', element: lazy_(<GroupGoods />) },
      { path: 'marketing/groupBuy/list', element: lazy_(<GroupList />) },
      { path: 'marketing/groupBuy/creatGroup', element: lazy_(<GroupCreate />) },
      { path: 'marketing/seckill/config', element: lazy_(<SeckillConfig />) },
      { path: 'marketing/seckill/list', element: lazy_(<SeckillList />) },
      { path: 'marketing/seckill/creatSeckill', element: lazy_(<SeckillCreate />) },
      { path: 'marketing/integral/config', element: lazy_(<IntegralConfig />) },
      { path: 'marketing/integral/log', element: lazy_(<IntegralLog />) },
      { path: 'marketing/atmosphere/list', element: lazy_(<AtmosphereList />) },
      { path: 'marketing/atmosphere/add', element: lazy_(<AtmosphereAdd />) },
      { path: 'marketing/border/list', element: lazy_(<AtmosphereList />) },
      { path: 'marketing/border/add', element: lazy_(<AtmosphereAdd />) },
      // Financial
      { path: 'financial/commission/template', element: lazy_(<FinancialExtract />) },
      { path: 'financial/record/charge', element: lazy_(<FinancialRecharge />) },
      { path: 'financial/record/monitor', element: lazy_(<FinancialMonitor />) },
      { path: 'financial/record/brokerage', element: lazy_(<FinancialBrokerage />) },
      // Content
      { path: 'content/article', element: lazy_(<ArticleList />) },
      { path: 'content/article/create', element: lazy_(<ArticleCreate />) },
      { path: 'content/category', element: lazy_(<ArticleCategory />) },
      // Distribution
      { path: 'distribution/index', element: lazy_(<DistributionList />) },
      { path: 'distribution/config', element: lazy_(<DistributionConfig />) },
      // Operation
      { path: 'operation/role', element: lazy_(<SystemRole />) },
      { path: 'operation/admin', element: lazy_(<SystemAdmin />) },
      { path: 'operation/config', element: lazy_(<SystemConfig />) },
      { path: 'operation/menu', element: lazy_(<SystemMenu />) },
      { path: 'operation/notification', element: lazy_(<SystemNotification />) },
      { path: 'operation/storePoint', element: lazy_(<SystemStorePoint />) },
      { path: 'operation/storeStaff', element: lazy_(<SystemStoreStaff />) },
      // Delivery settings (Vue-matching routes)
      { path: 'operation/deliverGoods/takeGoods/deliveryAddress', element: lazy_(<SystemStorePoint />) },
      { path: 'operation/deliverGoods/takeGoods/collateOrder', element: lazy_(<CollateOrder />) },
      { path: 'operation/deliverGoods/takeGoods/collateUser', element: lazy_(<SystemStoreStaff />) },
      { path: 'operation/deliverGoods/freightSet', element: lazy_(<FreightTemplate />) },
      // Maintain
      { path: 'maintain/material', element: lazy_(<MaterialList />) },
      { path: 'maintain/formConfig', element: lazy_(<FormConfig />) },
      { path: 'maintain/clearCache', element: lazy_(<ClearCache />) },
      { path: 'maintain/schedule/job', element: lazy_(<ScheduleJobList />) },
      { path: 'maintain/schedule/log', element: lazy_(<ScheduleLogList />) },
      // DevConfig
      { path: 'maintain/devconfig/configCategory', element: lazy_(<ConfigCategory />) },
      { path: 'maintain/devconfig/combinedData', element: lazy_(<CombinedData />) },
      { path: 'maintain/devconfig/formConfig', element: lazy_(<FormConfig />) },
      // SystemSetting - Logistics
      { path: 'systemSetting/logistics/express', element: lazy_(<ExpressList />) },
      { path: 'systemSetting/logistics/city', element: lazy_(<CityList />) },
      { path: 'systemSetting/deliverGoods/freight', element: lazy_(<FreightTemplate />) },
      // Vue-matching logistics routes
      { path: 'maintain/logistics/companyList', element: lazy_(<ExpressList />) },
      { path: 'maintain/logistics/cityList', element: lazy_(<CityList />) },
      // Statistics
      { path: 'statistic/product', element: lazy_(<StatProduct />) },
      { path: 'statistic/user', element: lazy_(<StatUser />) },
      { path: 'statistic/trade', element: lazy_(<StatTrade />) },
      // Design
      { path: 'design/index', element: lazy_(<DesignList />) },
      { path: 'design/builder', element: lazy_(<PageBuilder />) },
    ],
  },
  { path: '*', element: <Navigate to="/404" replace /> },
]);
