# JWT认证和权限验证实现说明

## 概述

已创建JWT认证和权限验证的基础框架，参考Java版本的实现。

## 已实现的组件

### 1. 认证模块 (`src/utils/auth.rs`)

- ✅ `LoginUserVo` - 登录用户信息结构
- ✅ `get_token_from_request()` - 从请求头获取Token
- ✅ `generate_token()` - 生成UUID Token
- ✅ `create_token()` - 创建Token并存储（需要Redis）
- ✅ `get_login_user()` - 从Redis获取登录用户信息（需要Redis）
- ✅ `verify_token()` - 验证Token是否有效
- ✅ `has_authority()` - 检查用户是否有指定权限

### 2. 权限检查模块 (`src/utils/permission.rs`)

- ✅ `check_permission()` - 检查请求是否有指定权限
- ✅ `get_current_user()` - 获取当前登录用户

### 3. 密码加密模块 (`src/utils/crypto.rs`)

- ✅ DES加密算法（与Java完全一致）
- ✅ Base64编码
- ✅ PKCS7填充

## 与Java版本的对应关系

| Java | Rust |
|------|------|
| `TokenComponent.createToken()` | `auth::create_token()` |
| `TokenComponent.getLoginUser()` | `auth::get_login_user()` |
| `TokenComponent.verifyToken()` | `auth::verify_token()` |
| `SecurityUtil.getLoginUserVo()` | `permission::get_current_user()` |
| `@PreAuthorize("hasAuthority('...')")` | `permission::check_permission()` |
| `Constants.HEADER_AUTHORIZATION_KEY` | `"Authori-zation"` |

## 当前状态

### ✅ 已完成
1. Token生成逻辑（UUID）
2. 权限检查逻辑（超级管理员、权限列表）
3. Token过期验证
4. 密码DES加密/解密
5. HomeController 8个统计接口（返回模拟数据）
6. 权限验证框架（auth.rs 和 permission.rs）

### ⚠️ 待完成（需要Redis）
1. Token存储到Redis
2. 从Redis读取用户信息
3. Token自动刷新（距离过期20分钟时）
4. 登出时删除Redis中的Token
5. 启用接口的权限验证（当前已注释）

### 📝 临时方案

由于Redis集成需要额外配置，当前采用以下临时方案：

1. **登录接口**：生成Token但不存储到Redis
2. **权限验证**：暂时禁用（所有接口都可访问）
3. **HomeController接口**：返回模拟数据，权限检查已注释
4. **后续集成Redis后**：取消注释权限检查代码即可

## 使用示例

### 1. 登录时生成Token

```rust
// 在 admin_login.rs 中
let token = auth::create_token(
    admin.id,
    admin.account.clone(),
    admin.roles.clone(),
    permissions,
).await;
```

### 2. 接口中验证权限

```rust
// 在 home.rs 中
async fn index_date(
    State(_ctx): State<AppContext>,
) -> Result<Response> {
    // TODO: 权限验证（需要Redis集成后启用）
    // permission::check_permission(&req, "admin:statistics:home:index").await?;

    // 业务逻辑...
}
```

注意：当前权限验证已注释，等待Redis集成后启用。

### 3. 获取当前登录用户

```rust
// 需要Redis集成后才能使用
let login_user = permission::get_current_user(&req).await?;
println!("当前用户: {}", login_user.account);
```

注意：当前 `get_login_user` 返回 None，需要Redis集成后才能正常工作。

## 权限验证规则

### 超级管理员
- 角色ID包含 `"1"` 的用户拥有所有权限
- 自动通过所有权限检查

### 普通用户
- 检查 `permissions` 列表
- 支持通配符 `"*:*:*"` 表示所有权限
- 精确匹配权限字符串

## 下一步工作

### 1. 集成Redis

```toml
# Cargo.toml
redis = { version = "0.24", features = ["tokio-comp"] }
```

```rust
// 在 auth.rs 中取消注释Redis相关代码
pub async fn create_token(...) -> String {
    let token = generate_token();
    // 取消注释以下代码
    // let redis_key = format!("TOKEN:ADMIN:{}", token);
    // redis.set(redis_key, login_user, expire_minutes * 60).await;
    token
}
```

### 2. 实现权限查询

从数据库查询用户的实际权限列表：

```rust
// 查询 system_role_menu 表
// 根据用户角色获取菜单权限
// 返回权限字符串列表
```

### 3. 添加权限中间件

使用Axum的middleware实现全局权限验证。

## 测试

```bash
# 测试Token生成
cargo test --lib auth::tests::test_generate_token

# 测试权限检查
cargo test --lib auth::tests::test_has_authority

# 测试Token验证
cargo test --lib auth::tests::test_verify_token
```

## 注意事项

1. **请求头名称**：`"Authori-zation"`（注意拼写，与Java保持一致）
2. **Token前缀**：`"TOKEN:ADMIN:"`
3. **Token有效期**：5小时（300分钟）
4. **自动刷新**：距离过期20分钟时自动刷新

## 文件位置

- `src/utils/auth.rs` - JWT认证模块
- `src/utils/permission.rs` - 权限检查模块
- `src/utils/crypto.rs` - 密码加密模块
- `src/controllers/admin_login.rs` - 登录接口（使用Token生成）
- `src/controllers/home.rs` - 统计接口（包含权限验证）
