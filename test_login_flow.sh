#!/bin/bash

echo "=== 测试登录和Token流程 ==="
echo ""

# 1. 登录
echo "1. 登录..."
LOGIN_RESPONSE=$(curl -s -X POST http://localhost:5150/api/admin/login \
  -H "Content-Type: application/json" \
  -d '{"account":"admin","pwd":"123456"}')

echo "登录响应:"
echo "$LOGIN_RESPONSE" | jq '.'
echo ""

# 提取Token
TOKEN=$(echo "$LOGIN_RESPONSE" | jq -r '.data.Token')
echo "提取的Token: $TOKEN"
echo ""

if [ "$TOKEN" = "null" ] || [ -z "$TOKEN" ]; then
    echo "❌ 登录失败，未获取到Token"
    exit 1
fi

# 2. 获取用户信息
echo "2. 获取用户信息..."
USER_INFO=$(curl -s http://localhost:5150/api/admin/getAdminInfoByToken \
  -H "Authori-zation: $TOKEN")

echo "用户信息响应:"
echo "$USER_INFO" | jq '.'
echo ""

# 3. 获取菜单
echo "3. 获取菜单..."
MENUS=$(curl -s http://localhost:5150/api/admin/getMenus \
  -H "Authori-zation: $TOKEN")

echo "菜单响应:"
echo "$MENUS" | jq '.'
echo ""

# 检查是否成功
if echo "$MENUS" | jq -e '.code == 200' > /dev/null; then
    MENU_COUNT=$(echo "$MENUS" | jq '.data | length')
    echo "✅ 成功获取 $MENU_COUNT 个菜单项"
else
    echo "❌ 获取菜单失败"
    exit 1
fi

echo ""
echo "=== 测试完成 ==="
