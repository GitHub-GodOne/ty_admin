# 快速开始指南

## 前置条件

- Rust 1.70+
- PostgreSQL 数据库
- 已配置好数据库连接（`config/development.yaml`）

## 启动步骤

### 1. 数据库初始化

```bash
# 重置数据库并运行迁移
cargo loco db reset

# 导入种子数据
cargo loco db seed
```

### 2. 启动服务

```bash
cargo loco start
```

服务将在 `http://localhost:5150` 启动

### 3. 测试接口

```bash
# 获取管理员列表
curl "http://localhost:5150/api/admin/system/admin/list?page=1&limit=10"

# 获取管理员详情
curl "http://localhost:5150/api/admin/system/admin/info?id=6"
```

## 可用的测试账号

从种子数据中导入的管理员账号：

| ID | 账号 | 姓名 | 角色 | 状态 |
|----|------|------|------|------|
| 2 | hzw | 念念猫 | 1 | 启用 |
| 5 | demo | 演示账号 | 5 | 启用 |
| 6 | admin | 超级管理员 | 1 | 启用 |
| 7 | test | 小月 | 6 | 启用 |

## API 端点

所有接口都在 `/api/admin/system/admin` 路径下：

- `GET /list` - 分页列表
- `POST /save` - 新增管理员
- `GET /delete` - 删除管理员
- `POST /update` - 修改管理员
- `GET /info` - 管理员详情
- `GET /updateStatus` - 修改状态
- `GET /update/isSms` - 修改短信接收状态

详细文档请查看 `docs/system_admin_api.md`

## 开发模式

```bash
# 监听文件变化自动重启
cargo watch -x 'loco start'
```

## 常见问题

### Q: 数据库连接失败
A: 检查 `config/development.yaml` 中的数据库配置

### Q: 端口被占用
A: 修改 `config/development.yaml` 中的 `server.port` 配置

### Q: 编译错误
A: 确保 Rust 版本 >= 1.70，运行 `cargo clean` 后重新编译

## 下一步

- 实现密码加密功能
- 添加权限验证中间件
- 集成 Swagger 文档
- 编写单元测试和集成测试
