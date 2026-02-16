# HomeController 接口实现文档

## 概述

已完成 HomeController.java 中所有8个统计接口的 Rust 实现。

## 实现的接口

### 1. 首页数据
- **路径**: `GET /api/admin/statistics/home/index`
- **权限**: `admin:statistics:home:index`
- **功能**: 获取首页统计数据（今日/昨日销售额、访问量、订单量、新增用户）
- **响应**: `HomeRateResponse`

### 2. 用户曲线图
- **路径**: `GET /api/admin/statistics/home/chart/user`
- **权限**: `admin:statistics:home:chart:user`
- **功能**: 获取用户增长曲线图数据
- **响应**: `Map<String, Value>` - 包含日期和数量数组

### 3. 用户购买统计
- **路径**: `GET /api/admin/statistics/home/chart/user/buy`
- **权限**: `admin:statistics:home:chart:user:buy`
- **功能**: 统计付费用户和未付费用户数量
- **响应**: `Map<String, i32>` - payUser, notPayUser

### 4. 30天订单量趋势
- **路径**: `GET /api/admin/statistics/home/chart/order`
- **权限**: `admin:statistics:home:chart:order`
- **功能**: 获取最近30天的订单量和订单金额趋势
- **响应**: `Map<String, Value>` - date, orderNum, orderAmount

### 5. 周订单量趋势
- **路径**: `GET /api/admin/statistics/home/chart/order/week`
- **权限**: `admin:statistics:home:chart:order:week`
- **功能**: 获取本周订单量趋势
- **响应**: `Map<String, Value>` - date, orderNum, orderAmount

### 6. 月订单量趋势
- **路径**: `GET /api/admin/statistics/home/chart/order/month`
- **权限**: `admin:statistics:home:chart:order:month`
- **功能**: 获取本月订单量趋势
- **响应**: `Map<String, Value>` - date, orderNum, orderAmount

### 7. 年订单量趋势
- **路径**: `GET /api/admin/statistics/home/chart/order/year`
- **权限**: `admin:statistics:home:chart:order:year`
- **功能**: 获取本年订单量趋势
- **响应**: `Map<String, Value>` - date, orderNum, orderAmount

### 8. 首页经营数据
- **路径**: `GET /api/admin/statistics/home/operating/data`
- **权限**: `admin:statistics:home:operating:data`
- **功能**: 获取经营数据（待发货、退款中、库存预警等）
- **响应**: `HomeOperatingDataResponse`

## 数据结构

### HomeRateResponse
```rust
pub struct HomeRateResponse {
    pub sales: Value,                    // 今日销售额
    pub yesterday_sales: Value,          // 昨日销售额
    pub pageviews: Value,                // 今日访问量
    pub yesterday_pageviews: Value,      // 昨日访问量
    pub order_num: Value,                // 今日订单量
    pub yesterday_order_num: Value,      // 昨日订单量
    pub new_user_num: Value,             // 今日新增用户
    pub yesterday_new_user_num: Value,   // 昨日新增用户
}
```

### HomeOperatingDataResponse
```rust
pub struct HomeOperatingDataResponse {
    pub not_shipping_order_num: i32,      // 待发货订单数量
    pub refunding_order_num: i32,         // 退款中订单数量
    pub not_write_off_order_num: i32,     // 待核销订单数量
    pub vigilance_inventory_num: i32,     // 库存预警商品数量
    pub on_sale_product_num: i32,         // 上架商品数量
    pub not_sale_product_num: i32,        // 仓库中商品数量
    pub not_audit_num: i32,               // 提现申请待审核数量
    pub total_recharge_amount: Decimal,   // 用户充值总金额
}
```

## 当前状态

✅ **接口框架已完成**
- 所有8个接口路由已注册
- 请求/响应结构已定义
- 与Java版本API路径完全一致
- CORS已配置，前端可正常访问

⚠️ **待实现的功能**
- 真实的数据库查询逻辑
- 统计计算逻辑
- 日期范围处理
- 数据聚合和分组

## 测试示例

```bash
# 1. 首页数据
curl http://127.0.0.1:5150/api/admin/statistics/home/index

# 2. 用户曲线图
curl http://127.0.0.1:5150/api/admin/statistics/home/chart/user

# 3. 用户购买统计
curl http://127.0.0.1:5150/api/admin/statistics/home/chart/user/buy

# 4. 30天订单量趋势
curl http://127.0.0.1:5150/api/admin/statistics/home/chart/order

# 5. 周订单量趋势
curl http://127.0.0.1:5150/api/admin/statistics/home/chart/order/week

# 6. 月订单量趋势
curl http://127.0.0.1:5150/api/admin/statistics/home/chart/order/month

# 7. 年订单量趋势
curl http://127.0.0.1:5150/api/admin/statistics/home/chart/order/year

# 8. 首页经营数据
curl http://127.0.0.1:5150/api/admin/statistics/home/operating/data
```

## 下一步工作

1. **实现统计逻辑**
   - 查询订单表统计销售额和订单量
   - 查询用户表统计新增用户
   - 查询商品表统计库存和上架状态
   - 查询提现表统计待审核数量

2. **日期处理**
   - 实现今日/昨日数据对比
   - 实现周/月/年时间范围查询
   - 实现30天数据聚合

3. **性能优化**
   - 添加缓存机制
   - 优化数据库查询
   - 使用索引提升查询速度

## 文件位置

- **控制器**: `src/controllers/home.rs`
- **路由注册**: `src/app.rs`
- **模块声明**: `src/controllers/mod.rs`
