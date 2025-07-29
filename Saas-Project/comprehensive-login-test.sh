#!/bin/bash

echo "🔐 COMPREHENSIVE ADMIN LOGIN TEST"
echo "================================="

# 1. Check backend health
echo "1. Checking backend health..."
HEALTH=$(curl -s http://localhost:8001/health)
if echo "$HEALTH" | grep -q "healthy"; then
    echo "✅ Backend is healthy"
else
    echo "❌ Backend is not responding"
    echo "Response: $HEALTH"
    exit 1
fi

# 2. Check database admin account status
echo ""
echo "2. Checking admin account in database..."
DB_STATUS=$(docker exec $(docker ps -q --filter "name=postgres") sh -c 'PGPASSWORD=saas_password psql -U saas_user -d saas_umkm_db -t -c "SELECT email, email_verified, status FROM users WHERE email = '\''admin@saas-umkm.local'\'';"' 2>/dev/null)

echo "Admin account status: $DB_STATUS"

# 3. Test login API
echo ""
echo "3. Testing login API..."
LOGIN_RESPONSE=$(curl -s -X POST http://localhost:8001/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "admin@saas-umkm.local",
    "password": "AdminPass123!"
  }')

# Extract HTTP status
HTTP_STATUS=$(echo $LOGIN_RESPONSE | tr -d '\n' | sed -e 's/.*HTTPSTATUS://')
RESPONSE_BODY=$(echo $LOGIN_RESPONSE | sed -e 's/HTTPSTATUS:.*//g')

echo "HTTP Status: $HTTP_STATUS"
echo "Response body length: ${#RESPONSE_BODY} characters"

if [ "$HTTP_STATUS" = "200" ]; then
    echo "✅ HTTP 200 - Login API responding correctly"
    
    if echo "$RESPONSE_BODY" | grep -q "access_token"; then
        echo "✅ SUCCESS! Access token found in response"
        echo "🚀 ADMIN LOGIN IS WORKING!"
    else
        echo "❌ No access token in response"
        echo "Response: $RESPONSE_BODY"
    fi
elif [ "$HTTP_STATUS" = "403" ]; then
    echo "❌ HTTP 403 - Account verification issue"
    echo "Response: $RESPONSE_BODY"
elif [ "$HTTP_STATUS" = "401" ]; then
    echo "❌ HTTP 401 - Invalid credentials"
    echo "Response: $RESPONSE_BODY"
else
    echo "❌ Unexpected HTTP status: $HTTP_STATUS"
    echo "Response: $RESPONSE_BODY"
fi

echo ""
echo "4. Frontend Integration Test..."
echo "Try logging in to frontend with:"
echo "Email: admin@saas-umkm.local"
echo "Password: AdminPass123!"
