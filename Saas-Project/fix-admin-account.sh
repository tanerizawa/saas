#!/bin/bash

# Fix Admin Account Script
# This script fixes the admin account by setting email_verified = true

echo "🔧 Fixing Admin Account Email Verification"
echo "=========================================="

# Check if postgres container is running
if ! docker ps --filter "name=postgres" --format "table {{.Names}}" | grep -q postgres; then
    echo "❌ PostgreSQL container is not running!"
    echo "Please start the database with: docker compose up -d postgres"
    exit 1
fi

echo "📧 Setting admin email as verified..."

# Use environment variable method to avoid password prompt
export PGPASSWORD=saas_password

# Get container ID
CONTAINER_ID=$(docker ps -q --filter "name=postgres")

if [ -z "$CONTAINER_ID" ]; then
    echo "❌ Could not find PostgreSQL container"
    exit 1
fi

# Update admin account to set email as verified
UPDATE_RESULT=$(docker exec -i $CONTAINER_ID psql -U saas_user -d saas_umkm_db -t -c "UPDATE users SET email_verified = TRUE WHERE email = 'admin@saas-umkm.local'; SELECT ROW_COUNT();" 2>/dev/null)

if [ $? -eq 0 ]; then
    echo "✅ Admin email verification updated successfully!"
    
    # Verify the update
    VERIFICATION_CHECK=$(docker exec -i $CONTAINER_ID psql -U saas_user -d saas_umkm_db -t -c "SELECT email_verified FROM users WHERE email = 'admin@saas-umkm.local';" 2>/dev/null | tr -d ' ')
    
    if [ "$VERIFICATION_CHECK" = "t" ]; then
        echo "✅ Email verification confirmed: TRUE"
        echo ""
        echo "🎯 Admin account is now ready for login!"
        echo ""
        echo "📋 Admin Credentials:"
        echo "   Email: admin@saas-umkm.local"
        echo "   Password: AdminPass123!"
        echo ""
        echo "🧪 Testing login now..."
        
        # Test login
        RESPONSE=$(curl -s -X POST http://localhost:8000/api/v1/auth/login \
          -H "Content-Type: application/json" \
          -d '{
            "email": "admin@saas-umkm.local",
            "password": "AdminPass123!"
          }')
        
        if echo "$RESPONSE" | grep -q "access_token"; then
            echo "✅ Admin login test SUCCESSFUL!"
            echo "🚀 Frontend login should now work!"
        else
            echo "❌ Admin login test failed"
            echo "Response: $RESPONSE"
        fi
    else
        echo "❌ Email verification check failed"
    fi
else
    echo "❌ Failed to update admin email verification"
    echo "Trying alternative method..."
    
    # Alternative method using docker exec without environment variable
    docker exec -i $CONTAINER_ID psql -U saas_user -d saas_umkm_db <<EOF
UPDATE users SET email_verified = TRUE WHERE email = 'admin@saas-umkm.local';
EOF
    
    if [ $? -eq 0 ]; then
        echo "✅ Alternative method successful!"
    else
        echo "❌ All methods failed. Please update manually."
    fi
fi

unset PGPASSWORD
echo ""
echo "🏁 Admin account fix completed!"
