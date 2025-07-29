#!/bin/bash

# Create Default Admin Account Script
# This script creates a default admin account for the SaaS UMKM Platform

echo "🔐 Creating Default Admin Account for SaaS UMKM Platform"
echo "======================================================="

# Admin account details
ADMIN_EMAIL="admin@saas-umkm.local"
ADMIN_PASSWORD="AdminPass123!"
ADMIN_NAME="System Administrator"

echo "📧 Admin Email: $ADMIN_EMAIL"
echo "👤 Admin Name: $ADMIN_NAME"
echo "🔑 Admin Password: $ADMIN_PASSWORD"
echo ""

echo "�️  Cleaning up existing admin account..."
PGPASSWORD=saas_password docker exec -i $(docker ps -q --filter "name=postgres") psql -U saas_user -d saas_umkm_db -c "DELETE FROM users WHERE email = '$ADMIN_EMAIL';" 2>/dev/null || echo "No existing admin account found"

echo "�🚀 Creating admin account via API..."

# Create admin account
RESPONSE=$(curl -s -X POST http://localhost:8000/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d "{
    \"email\": \"$ADMIN_EMAIL\",
    \"password\": \"$ADMIN_PASSWORD\",
    \"full_name\": \"$ADMIN_NAME\"
  }")

if [ $? -eq 0 ]; then
    echo "✅ Admin account created successfully!"
    
    echo "🔐 Verifying admin email in database..."
    # Set email as verified in database
    PGPASSWORD=saas_password docker exec -i $(docker ps -q --filter "name=postgres") psql -U saas_user -d saas_umkm_db -c "UPDATE users SET email_verified = TRUE WHERE email = '$ADMIN_EMAIL';" 2>/dev/null
    if [ $? -eq 0 ]; then
        echo "✅ Admin email verified successfully!"
    else
        echo "⚠️  Could not verify email automatically, please verify manually"
    fi
    
    echo ""
    echo "📋 Admin Login Credentials:"
    echo "   Email: $ADMIN_EMAIL"
    echo "   Password: $ADMIN_PASSWORD"
    echo ""
    echo "🧪 Testing admin login..."
    
    # Test login
    LOGIN_RESPONSE=$(curl -s -X POST http://localhost:8000/api/v1/auth/login \
      -H "Content-Type: application/json" \
      -d "{
        \"email\": \"$ADMIN_EMAIL\",
        \"password\": \"$ADMIN_PASSWORD\"
      }")
    
    if [ $? -eq 0 ]; then
        echo "✅ Admin login test successful!"
        echo ""
        echo "🎯 Admin account is ready for frontend login!"
        echo ""
        echo "📝 Frontend Login Instructions:"
        echo "   1. Open your frontend application"
        echo "   2. Navigate to the login page"
        echo "   3. Use the credentials above to login"
        echo "   4. You should have admin access to the system"
    else
        echo "❌ Admin login test failed!"
        echo "Response: $LOGIN_RESPONSE"
    fi
else
    echo "❌ Failed to create admin account!"
    echo "Response: $RESPONSE"
    echo ""
    echo "🔍 Troubleshooting:"
    echo "   1. Make sure the backend server is running on port 8000"
    echo "   2. Check if the database is connected"
    echo "   3. Verify the authentication endpoints are working"
fi

echo ""
echo "🏁 Admin account creation script completed!"
