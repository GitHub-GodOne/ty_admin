# CORS 配置说明

## 问题描述

前端运行在 `http://localhost:9527`，后端运行在 `http://127.0.0.1:5150`，浏览器的同源策略会阻止跨域请求。

## 解决方案

### 1. 配置 CORS 中间件

在 `config/development.yaml` 中添加 CORS 配置：

```yaml
server:
  port: 5150
  binding: 0.0.0.0  # 重要：绑定到 0.0.0.0 以接受所有连接
  host: http://localhost
  middlewares:
    cors:
      enable: true
      allow_origins:
        - http://localhost:9527
        - http://127.0.0.1:9527
      allow_credentials: true
      allow_methods:
        - GET
        - POST
        - PUT
        - DELETE
        - OPTIONS
        - PATCH
      allow_headers:
        - Content-Type
        - Authorization
        - X-Requested-With
        - Accept
        - Origin
        - Access-Control-Request-Method
        - Access-Control-Request-Headers
      expose_headers:
        - Content-Length
        - Content-Type
        - Authorization
      max_age: 3600
```

### 2. 关键配置说明

#### `binding: 0.0.0.0`
- **必须设置为 `0.0.0.0`**，而不是 `localhost`
- `localhost` 只监听本地回环接口
- `0.0.0.0` 监听所有网络接口，包括 `127.0.0.1` 和 `localhost`

#### `allow_credentials: true`
- 允许携带 Cookie 和认证信息
- **注意**：当设置为 `true` 时，不能使用 `allow_headers: ["*"]`
- 必须明确列出允许的请求头

#### `allow_origins`
- 列出所有允许的前端域名
- 开发环境包括 `localhost` 和 `127.0.0.1`
- 生产环境应该只包含实际的域名

### 3. 常见错误

#### 错误 1: `Cannot combine Access-Control-Allow-Credentials: true with Access-Control-Allow-Headers: *`

**原因**: CORS 规范不允许同时使用通配符和凭证

**解决**: 明确列出所有需要的请求头

```yaml
# ❌ 错误
allow_credentials: true
allow_headers:
  - "*"

# ✅ 正确
allow_credentials: true
allow_headers:
  - Content-Type
  - Authorization
  - X-Requested-With
```

#### 错误 2: `No 'Access-Control-Allow-Origin' header is present`

**原因**:
1. CORS 中间件未启用
2. 服务器绑定地址不正确
3. 前端请求的域名不在 `allow_origins` 列表中

**解决**:
1. 确认 `cors.enable: true`
2. 设置 `binding: 0.0.0.0`
3. 添加前端域名到 `allow_origins`

### 4. 测试 CORS 配置

使用 curl 测试预检请求：

```bash
curl -X OPTIONS 'http://127.0.0.1:5150/api/admin/system/admin/list' \
  -H 'Origin: http://localhost:9527' \
  -H 'Access-Control-Request-Method: GET' \
  -H 'Access-Control-Request-Headers: Content-Type' \
  -i
```

成功的响应应该包含：
```
HTTP/1.1 200 OK
access-control-allow-origin: http://localhost:9527
access-control-allow-credentials: true
access-control-allow-methods: GET,POST,PUT,DELETE,OPTIONS,PATCH
access-control-allow-headers: content-type,authorization,...
```

### 5. 生产环境配置

在 `config/production.yaml` 中：

```yaml
server:
  binding: 0.0.0.0
  middlewares:
    cors:
      enable: true
      allow_origins:
        - https://your-frontend-domain.com  # 只允许生产域名
      allow_credentials: true
      allow_methods:
        - GET
        - POST
        - PUT
        - DELETE
        - OPTIONS
        - PATCH
      allow_headers:
        - Content-Type
        - Authorization
        - X-Requested-With
        - Accept
      max_age: 86400  # 24小时
```

### 6. 安全建议

1. **不要在生产环境使用通配符** `*`
2. **明确列出允许的域名**，不要使用 `allow_origins: ["*"]`
3. **只允许必要的 HTTP 方法**
4. **定期审查 CORS 配置**
5. **使用 HTTPS** 在生产环境

### 7. 调试技巧

#### 查看服务器日志
```bash
tail -f /tmp/ty_admin_server.log | grep -i cors
```

#### 浏览器开发者工具
1. 打开 Network 标签
2. 查看 OPTIONS 请求（预检请求）
3. 检查响应头中的 `Access-Control-*` 字段

#### 常用响应头
- `Access-Control-Allow-Origin`: 允许的源
- `Access-Control-Allow-Methods`: 允许的方法
- `Access-Control-Allow-Headers`: 允许的请求头
- `Access-Control-Allow-Credentials`: 是否允许凭证
- `Access-Control-Max-Age`: 预检请求缓存时间

## 总结

CORS 配置已成功启用，前端 `http://localhost:9527` 现在可以正常访问后端 `http://127.0.0.1:5150` 的所有接口！

关键配置：
- ✅ `binding: 0.0.0.0`
- ✅ `cors.enable: true`
- ✅ 明确的 `allow_origins` 列表
- ✅ 明确的 `allow_headers` 列表
- ✅ `allow_credentials: true`
