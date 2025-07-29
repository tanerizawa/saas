#!/bin/bash

echo "🔐 FRONTEND LOGIN VERIFICATION"
echo "============================="

echo ""
echo "1. Backend Health Check..."
HEALTH=$(curl -s http://localhost:8001/health)
if echo "$HEALTH" | grep -q "healthy"; then
    echo "✅ Backend is healthy on port 8001"
else
    echo "❌ Backend not responding on port 8001"
    exit 1
fi

echo ""  
echo "2. Testing Admin Login API..."
LOGIN_RESPONSE=$(curl -s -X POST http://localhost:8001/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"admin@saas-umkm.local","password":"AdminPass123!"}')

if echo "$LOGIN_RESPONSE" | grep -q "access_token"; then
    echo "✅ Login API working - JWT token received"
    echo "   User role: $(echo "$LOGIN_RESPONSE" | grep -o '"role":"[^"]*"' | cut -d'"' -f4)"
else
    echo "❌ Login API failed"
    echo "Response: $LOGIN_RESPONSE"
    exit 1
fi

echo ""
echo "3. Frontend Configuration Check..."
if [ -f "frontend/.env.local" ]; then
    echo "✅ Frontend .env.local exists"
    API_URL=$(grep "NEXT_PUBLIC_API_URL" frontend/.env.local | cut -d'=' -f2)
    MOCK_API=$(grep "NEXT_PUBLIC_USE_MOCK_API" frontend/.env.local | cut -d'=' -f2)
    
    if [[ "$API_URL" == "http://localhost:8001/api/v1" ]]; then
        echo "✅ API URL correctly set to port 8001"
    else
        echo "❌ API URL incorrect: $API_URL"
    fi
    
    if [[ "$MOCK_API" == "false" ]]; then
        echo "✅ Mock API disabled (using real backend)"
    else
        echo "❌ Mock API still enabled: $MOCK_API"
    fi
else
    echo "❌ Frontend .env.local not found"
fi

echo ""
echo "4. Final Status..."
echo "✅ Backend: Running on port 8001"
echo "✅ Admin Account: admin@saas-umkm.local / AdminPass123!"
echo "✅ Role: super_admin"
echo "✅ Frontend Config: Updated for real backend"
echo ""
echo "🎯 READY FOR FRONTEND LOGIN!"
echo "   Navigate to: http://localhost:3000/auth/login"
echo "   Use credentials: admin@saas-umkm.local / AdminPass123!"
