# 后台用户服务接口文档

## 接口列表

### 1. 分页列表
- **路径**: `GET /api/admin/system/admin/list`
- **权限**: `admin:system:admin:list`
- **描述**: 分页显示后台管理员列表

**请求参数**:
```json
{
  "realName": "管理员姓名（可选）",
  "roles": "角色ID（可选）",
  "status": 1,  // 状态：1有效 0无效（可选）
  "page": 1,    // 页码，默认1
  "limit": 20   // 每页数量，默认20
}
```

**响应示例**:
```json
{
  "code": 200,
  "message": "操作成功",
  "data": {
    "page": 1,
    "limit": 20,
    "total_page": 5,
    "total": 100,
    "list": [
      {
        "id": 1,
        "account": "admin",
        "realName": "超级管理员",
        "roles": "1",
        "roleNames": null,
        "lastIp": "192.168.1.1",
        "updateTime": "2025-01-01 12:00:00",
        "createTime": "2024-01-01 10:00:00",
        "loginCount": 100,
        "level": 1,
        "status": 1,
        "phone": "18888888888",
        "isSms": 0
      }
    ]
  }
}
```

---

### 2. 新增后台管理员
- **路径**: `POST /api/admin/system/admin/save`
- **权限**: `admin:system:admin:save`
- **描述**: 新增后台管理员

**请求参数**:
```json
{
  "account": "admin123",        // 账号，必填，最长32字符
  "pwd": "password123",         // 密码，必填，最长32字符
  "realName": "张三",           // 姓名，必填，最长16字符
  "roles": "1,2,3",            // 角色ID，必填，最长128字符
  "status": 1,                 // 状态：1有效 0无效，必填
  "phone": "18888888888"       // 手机号，必填，11位
}
```

**响应示例**:
```json
{
  "code": 200,
  "message": "添加管理员成功",
  "data": null
}
```

---

### 3. 删除后台管理员
- **路径**: `GET /api/admin/system/admin/delete`
- **权限**: `admin:system:admin:delete`
- **描述**: 删除后台管理员

**请求参数**:
```
?id=1  // 管理员ID
```

**响应示例**:
```json
{
  "code": 200,
  "message": "删除成功",
  "data": null
}
```

---

### 4. 修改后台管理员
- **路径**: `POST /api/admin/system/admin/update`
- **权限**: `admin:system:admin:update`
- **描述**: 修改后台管理员信息

**请求参数**:
```json
{
  "id": 1,                     // 管理员ID，必填
  "account": "admin123",        // 账号，必填，最长32字符
  "pwd": "newpassword",         // 密码，必填，最长32字符
  "realName": "李四",           // 姓名，必填，最长16字符
  "roles": "1,2",              // 角色ID，必填，最长128字符
  "status": 1,                 // 状态：1有效 0无效，必填
  "phone": "18888888888"       // 手机号，必填，11位
}
```

**响应示例**:
```json
{
  "code": 200,
  "message": "修改成功",
  "data": null
}
```

---

### 5. 后台管理员详情
- **路径**: `GET /api/admin/system/admin/info`
- **权限**: `admin:system:admin:info`
- **描述**: 获取后台管理员详情

**请求参数**:
```
?id=1  // 管理员ID
```

**响应示例**:
```json
{
  "code": 200,
  "message": "操作成功",
  "data": {
    "id": 1,
    "account": "admin",
    "pwd": "encrypted_password",
    "real_name": "超级管理员",
    "roles": "1",
    "last_ip": "192.168.1.1",
    "update_time": "2025-01-01T12:00:00",
    "create_time": "2024-01-01T10:00:00",
    "login_count": 100,
    "level": 1,
    "status": 1,
    "phone": "18888888888",
    "is_sms": 0
  }
}
```

---

### 6. 修改后台管理员状态
- **路径**: `GET /api/admin/system/admin/updateStatus`
- **权限**: `admin:system:admin:update:status`
- **描述**: 修改后台管理员状态（启用/禁用）

**请求参数**:
```
?id=1&status=true  // id: 管理员ID, status: true启用 false禁用
```

**响应示例**:
```json
{
  "code": 200,
  "message": "修改成功",
  "data": "修改成功"
}
```

---

### 7. 修改后台管理员是否接收短信状态
- **路径**: `GET /api/admin/system/admin/update/isSms`
- **权限**: `admin:system:admin:update:sms`
- **描述**: 切换后台管理员是否接收短信状态

**请求参数**:
```
?id=1  // 管理员ID
```

**响应示例**:
```json
{
  "code": 200,
  "message": "修改成功",
  "data": "修改成功"
}
```

---

## 通用响应格式

所有接口都遵循统一的响应格式：

```json
{
  "code": 200,        // 响应码：200成功，500失败
  "message": "操作成功",  // 响应消息
  "data": {}          // 响应数据，可能为null
}
```

## 错误处理

当请求失败时，响应格式如下：

```json
{
  "code": 500,
  "message": "错误信息描述",
  "data": null
}
```

常见错误：
- 参数验证失败
- 账号已存在
- 管理员不存在
- 数据库操作失败

## 注意事项

1. **密码加密**: 当前代码中密码未加密，实际使用时需要实现密码加密逻辑（TODO标记处）
2. **权限验证**: 接口文档中标注了所需权限，实际使用时需要实现权限验证中间件
3. **角色名称**: `roleNames` 字段需要关联查询角色表获取，当前返回null
4. **日期格式**: 所有日期时间字段统一使用 `YYYY-MM-DD HH:MM:SS` 格式

## 测试示例

使用 curl 测试接口：

```bash
# 1. 获取管理员列表
curl -X GET "http://localhost:5150/api/admin/system/admin/list?page=1&limit=10"

# 2. 新增管理员
curl -X POST "http://localhost:5150/api/admin/system/admin/save" \
  -H "Content-Type: application/json" \
  -d '{
    "account": "test001",
    "pwd": "123456",
    "realName": "测试管理员",
    "roles": "1",
    "status": 1,
    "phone": "18888888888"
  }'

# 3. 删除管理员
curl -X GET "http://localhost:5150/api/admin/system/admin/delete?id=1"

# 4. 修改管理员
curl -X POST "http://localhost:5150/api/admin/system/admin/update" \
  -H "Content-Type: application/json" \
  -d '{
    "id": 1,
    "account": "test001",
    "pwd": "newpassword",
    "realName": "测试管理员2",
    "roles": "1,2",
    "status": 1,
    "phone": "18888888888"
  }'

# 5. 获取管理员详情
curl -X GET "http://localhost:5150/api/admin/system/admin/info?id=1"

# 6. 修改管理员状态
curl -X GET "http://localhost:5150/api/admin/system/admin/updateStatus?id=1&status=true"

# 7. 修改是否接收短信
curl -X GET "http://localhost:5150/api/admin/system/admin/update/isSms?id=1"
```
