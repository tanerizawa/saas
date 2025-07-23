#!/bin/bash

# Script to start the backend server without running migrations
# Perbaikan untuk masalah: Error: Migrate(VersionMissing(2))

# Warna untuk output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${YELLOW}=== SAAS UMKM Platform - Run Backend Server (Skip Migrations) ===${NC}"

# Dapatkan IP address dari container PostgreSQL
PG_IP=$(docker inspect saas-postgres -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' 2>/dev/null)

if [ -z "$PG_IP" ] || [ "$PG_IP" == "" ]; then
  echo -e "${YELLOW}Tidak dapat mendapatkan IP PostgreSQL dari Docker, menggunakan localhost...${NC}"
  PG_IP="localhost"
fi

# Set environment variables
export DATABASE_URL="postgresql://saas_user:saas_password@${PG_IP}:5432/saas_umkm_db" 
export REDIS_URL="redis://127.0.0.1:6379"
export JWT_SECRET="your_secure_jwt_secret_key_here"
export APP_HOST="0.0.0.0"
export APP_PORT="8000"
export SKIP_MIGRATIONS="true"  # Add this to skip migrations

echo -e "${GREEN}🚀 Starting backend server with fixed database connections...${NC}"
echo -e "${GREEN}📊 DATABASE_URL=${DATABASE_URL}${NC}"
echo -e "${GREEN}📊 REDIS_URL=${REDIS_URL}${NC}"
echo -e "${YELLOW}⚠️ Migrations will be SKIPPED${NC}"

# Buat temporary file untuk penyuntingan source code
MAIN_FILE="src/main.rs"
TEMP_FILE=$(mktemp)
BACKUP_FILE="${MAIN_FILE}.bak"

# Buat backup file asli
cp "$MAIN_FILE" "$BACKUP_FILE"
echo -e "${GREEN}✅ Backup main.rs dibuat di ${BACKUP_FILE}${NC}"

# Mencari baris yang menjalankan migrasi dan komentar
echo -e "${YELLOW}🔄 Memodifikasi kode untuk melewati migrasi...${NC}"
cat "$MAIN_FILE" | while IFS= read -r line; do
    if [[ "$line" == *"db.migrate().await?"* ]]; then
        echo "    // Migration disabled to fix version issue" >> "$TEMP_FILE"
        echo "    // Original line: $line" >> "$TEMP_FILE"
        echo '    info!("🔄 Database migrations SKIPPED (to fix version issues)");' >> "$TEMP_FILE"
    else
        echo "$line" >> "$TEMP_FILE"
    fi
done

# Terapkan perubahan
mv "$TEMP_FILE" "$MAIN_FILE"
echo -e "${GREEN}✅ Kode berhasil dimodifikasi${NC}"

# Jalankan server
echo -e "${YELLOW}🚀 Menjalankan server...${NC}"
cargo run --bin server

# Kembalikan file asli
mv "$BACKUP_FILE" "$MAIN_FILE"
echo -e "\n${GREEN}✅ File main.rs telah dikembalikan ke kondisi semula${NC}"
