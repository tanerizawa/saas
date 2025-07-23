#!/bin/bash

# Test Authentication API Endpoints
# This script tests the basic authentication functionality

BASE_URL="http://localhost:3000"
API_BASE="$BASE_URL/api/v1"

echo "🧪 Testing SaaS UMKM Authentication API"
echo "======================================"

# Test health endpoint
echo "1. Testing health endpoint..."
curl -s "$BASE_URL/health" | jq '.' || echo "Health endpoint failed"

echo -e "\n2. Testing auth health endpoint..."
curl -s "$API_BASE/auth/health" | jq '.' || echo "Auth health endpoint failed"

# Test registration
echo -e "\n3. Testing user registration..."
REGISTER_RESPONSE=$(curl -s -X POST "$API_BASE/auth/register" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "password123",
    "full_name": "Test User",
    "role": "umkm_owner"
  }')

echo "Registration Response:"
echo "$REGISTER_RESPONSE" | jq '.' || echo "$REGISTER_RESPONSE"

# Test login
echo -e "\n4. Testing user login..."
LOGIN_RESPONSE=$(curl -s -X POST "$API_BASE/auth/login" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "password123"
  }')

echo "Login Response:"
echo "$LOGIN_RESPONSE" | jq '.' || echo "$LOGIN_RESPONSE"

# Extract access token if login successful
ACCESS_TOKEN=$(echo "$LOGIN_RESPONSE" | jq -r '.access_token // empty')

if [ -n "$ACCESS_TOKEN" ]; then
    echo -e "\n5. Testing protected profile endpoint..."
    curl -s "$API_BASE/me" \
      -H "Authorization: Bearer $ACCESS_TOKEN" | jq '.' || echo "Profile endpoint failed"
    
    echo -e "\n6. Testing token refresh..."
    REFRESH_TOKEN=$(echo "$LOGIN_RESPONSE" | jq -r '.refresh_token // empty')
    if [ -n "$REFRESH_TOKEN" ]; then
        curl -s -X POST "$API_BASE/auth/refresh" \
          -H "Content-Type: application/json" \
          -d "{\"refresh_token\": \"$REFRESH_TOKEN\"}" | jq '.' || echo "Token refresh failed"
    fi
    
    echo -e "\n7. Testing logout..."
    curl -s -X POST "$API_BASE/auth/logout" \
      -H "Authorization: Bearer $ACCESS_TOKEN" | jq '.' || echo "Logout failed"
else
    echo "❌ Login failed, skipping protected endpoints"
fi

echo -e "\n✅ Authentication API testing completed!"
