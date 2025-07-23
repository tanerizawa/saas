#!/bin/bash

# Script sederhana untuk menjalankan backend dengan Docker IP langsung

# Dapatkan IP PostgreSQL
PG_IP=$(docker inspect saas-postgres -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' 2>/dev/null || echo "localhost")

# Jalankan server dengan IP Docker
cd /Users/odangrodiana/Desktop/01_DEVELOPMENT_PROJECTS/saas/Saas-Project/backend
export DATABASE_URL="postgresql://saas_user:saas_password@${PG_IP}:5432/saas_umkm_db"
export REDIS_URL="redis://localhost:6379"
export JWT_SECRET="your_secure_jwt_secret_key_here"

echo "==============================================="
echo "🚀 Menjalankan Backend SaaS UMKM"
echo "📊 DATABASE_URL=$DATABASE_URL"
echo "📊 REDIS_URL=$REDIS_URL"
echo "==============================================="

# Manual edit src/main.rs to comment out migration
MAIN_FILE="src/main.rs"
BACKUP_FILE="${MAIN_FILE}.bak"

# Make backup
cp "$MAIN_FILE" "$BACKUP_FILE"
echo "✅ Backup dibuat: $BACKUP_FILE"

# Edit file to comment out migration line
sed -i '' 's/db\.migrate()\.await?/\/\/ db\.migrate()\.await? \/\/ Disabled temporarily/' "$MAIN_FILE"
echo "✅ Baris migrasi dinonaktifkan"

# Run server
echo "🚀 Menjalankan server..."
cargo run --bin server

# Restore original file
mv "$BACKUP_FILE" "$MAIN_FILE"
echo "✅ File asli dikembalikan"
