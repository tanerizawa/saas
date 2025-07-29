#!/bin/bash

echo "🧪 FINAL ADMIN LOGIN TEST"
echo "========================"

echo "📧 Testing admin login with verified email..."
echo ""

# Test the login
RESPONSE=$(curl -s -X POST http://localhost:8000/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "admin@saas-umkm.local",
    "password": "AdminPass123!"
  }')

echo "Response received. Checking for tokens..."

if echo "$RESPONSE" | grep -q "access_token"; then
    echo "✅ SUCCESS! Admin login works perfectly!"
    echo ""
    echo "🎯 Response contains:"
    echo "$RESPONSE" | grep -o '"access_token":"[^"]*"' | head -c 50
    echo "..."
    echo ""
    echo "🚀 FRONTEND LOGIN SHOULD NOW WORK!"
    echo ""
    echo "📋 Login Credentials for Frontend:"
    echo "   Email: admin@saas-umkm.local"
    echo "   Password: AdminPass123!"
    echo ""
    echo "✅ Account Status: VERIFIED & READY"
else
    echo "❌ Login failed or no access token received"
    echo ""
    echo "Raw response:"
    echo "$RESPONSE"
    echo ""
    echo "🔍 Checking if account is verified in database..."
    docker exec -i $(docker ps -q --filter "name=postgres") sh -c 'PGPASSWORD=saas_password psql -U saas_user -d saas_umkm_db -c "SELECT email, email_verified, status FROM users WHERE email = '\''admin@saas-umkm.local'\'';"'
fi

echo ""
echo "🏁 Test completed!"
