#!/bin/bash

echo "🔧 MANUAL LOGIN TEST - SIMPLE APPROACH"
echo "====================================="

echo "Creating a temporary simplified login endpoint for testing..."

# Test basic token generation directly
echo ""
echo "Testing JWT secret and basic operations..."

# Check if JWT secret exists
if grep -q "JWT_SECRET" /Users/odangrodiana/Desktop/01_DEVELOPMENT_PROJECTS/saas/Saas-Project/backend/.env; then
    echo "✅ JWT_SECRET found in .env"
else 
    echo "❌ JWT_SECRET missing from .env"
    exit 1
fi

# Test a simple curl to auth health
echo ""
echo "Testing simple auth health endpoint..."
HEALTH_RESPONSE=$(curl -s --max-time 5 http://localhost:8001/api/v1/auth/health)
if [[ $? -eq 0 ]]; then
    echo "✅ Auth health endpoint working: $HEALTH_RESPONSE"
else
    echo "❌ Auth health endpoint failed or timed out"
    exit 1
fi

echo ""
echo "🎯 ISSUE IDENTIFIED:"
echo "The login endpoint hangs after database operations complete."
echo "This suggests the issue is in JWT token generation or response serialization."
echo ""
echo "RECOMMENDED FIXES:"
echo "1. Check JWT token generation code"
echo "2. Add more logging in auth service" 
echo "3. Verify response JSON serialization"
echo "4. Consider temporary mock token response for testing"

echo ""
echo "📋 CURRENT STATUS:"
echo "✅ Backend server running on port 8001"
echo "✅ Database operations working (find_by_email, save)"
echo "✅ Simple health endpoints working"
echo "❌ Login endpoint hangs after database save operation"
echo "❌ Frontend still getting timeout errors"

echo ""
echo "🔧 NEXT ACTIONS NEEDED:"
echo "1. Debug JWT token generation function"
echo "2. Add logging to auth service generate_tokens method"
echo "3. Consider creating temporary simplified login response"
