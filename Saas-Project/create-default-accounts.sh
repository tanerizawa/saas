#!/bin/bash
# Script to create default user and admin accounts

API_BASE="http://localhost:8000/api/v1"

echo "Creating default user account..."
USER_RESPONSE=$(curl -s -X POST "$API_BASE/auth/register" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "Password123!",
    "full_name": "Default User",
    "role": "user"
  }')

echo "User registration response:"
echo "$USER_RESPONSE" | jq '.' || echo "$USER_RESPONSE"

echo "Creating admin account..."
ADMIN_RESPONSE=$(curl -s -X POST "$API_BASE/auth/register" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "admin@example.com",
    "password": "AdminPass123!",
    "full_name": "Admin User",
    "role": "super_admin"
  }')

echo "Admin registration response:"
echo "$ADMIN_RESPONSE" | jq '.' || echo "$ADMIN_RESPONSE"

echo "Creating staff account..."
STAFF_RESPONSE=$(curl -s -X POST "$API_BASE/auth/register" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "staff@example.com",
    "password": "StaffPass123!",
    "full_name": "Staff User",
    "role": "admin_staff"
  }')

echo "Staff registration response:"
echo "$STAFF_RESPONSE" | jq '.' || echo "$STAFF_RESPONSE"

echo "Account creation completed."
echo
echo "Default User Login:"
echo "Email: user@example.com"
echo "Password: Password123!"
echo
echo "Admin Login:"
echo "Email: admin@example.com"
echo "Password: AdminPass123!"
echo
echo "Staff Login:"
echo "Email: staff@example.com"
echo "Password: StaffPass123!"
