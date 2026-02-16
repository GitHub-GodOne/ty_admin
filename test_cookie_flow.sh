#!/bin/bash

echo "=== 测试Cookie登录流程 ==="
echo ""

# 1. 登录并保存Cookie
echo "1. 登录并保存Cookie..."
curl -c cookies.txt -X POST http://localhost:5150/api/admin/login \
  -H "Content-Type: application/json" \
  -d '{"account":"admin","pwd":"123456"}' | jq '.'

echo ""
echo "2. 查看保存的Cookie..."
cat cookies.txt

echo ""
echo "3. 使用Cookie访问 getAdminInfoByToken..."
curl -b cookies.txt http://localhost:5150/api/admin/getAdminInfoByToken | jq '.'

echo ""
echo "4. 使用Cookie访问 getMenus..."
MENU_RESPONSE=$(curl -s -b cookies.txt http://localhost:5150/api/admin/getMenus)
echo "$MENU_RESPONSE" | jq '.'

# 检查是否成功
if echo "$MENU_RESPONSE" | jq -e '.code == 200' > /dev/null; then
    MENU_COUNT=$(echo "$MENU_RESPONSE" | jq '.data | length')
    echo ""
    echo "✅ 成功！使用Cookie获取到 $MENU_COUNT 个菜单项"
else
    echo ""
    echo "❌ 失败！无法使用Cookie访问"
fi

echo ""
echo "=== 测试完成 ==="
