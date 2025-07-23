#!/bin/bash

# Script untuk memperbaiki masalah migrasi database
# Masalah: Error: Migrate(VersionMissing(2))

# Warna untuk output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${YELLOW}=== SAAS UMKM Platform - Migration Fix Script ===${NC}"
echo -e "${YELLOW}Script ini akan memperbaiki masalah versioning pada migrasi database${NC}"
echo

# Load environment variables
if [ -f ".env" ]; then
  source .env
fi

# Set default values jika tidak ada di environment
DB_USER=${DB_USER:-saas_user}
DB_PASSWORD=${DB_PASSWORD:-saas_password}
DB_NAME=${DB_NAME:-saas_umkm_db}
DB_HOST=${DB_HOST:-localhost}
DB_PORT=${DB_PORT:-5432}

# Konfirmasi sebelum melanjutkan
echo -e "${RED}PERHATIAN: Script ini akan mereset data migrasi di database!${NC}"
read -p "Lanjutkan untuk memperbaiki migrasi? (y/n): " CONFIRM
if [[ "$CONFIRM" != "y" && "$CONFIRM" != "Y" ]]; then
  echo -e "${YELLOW}Operasi dibatalkan.${NC}"
  exit 0
fi

# Pindah ke direktori migrations
cd "migrations" || { echo -e "${RED}Direktori migrations tidak ditemukan!${NC}"; exit 1; }
MIGRATIONS_DIR=$(pwd)
echo -e "${GREEN}📁 Direktori migrasi: $MIGRATIONS_DIR${NC}"

# Backup migrasi yang ada
BACKUP_DIR="${MIGRATIONS_DIR}_backup_$(date +%Y%m%d%H%M%S)"
echo -e "${GREEN}💾 Membuat backup di: $BACKUP_DIR${NC}"
mkdir -p "$BACKUP_DIR"
cp *.sql "$BACKUP_DIR/"

# Menghapus file migrasi dengan format lama
echo -e "${YELLOW}🔄 Menstandarkan format file migrasi...${NC}"
if [ -f "00001_initial_schema.sql" ]; then
  echo -e "${GREEN}Menghapus file migrasi lama: 00001_initial_schema.sql${NC}"
  rm -f "00001_initial_schema.sql"
fi

# Reset tracking migrasi di database
echo -e "${YELLOW}🔄 Reset tracking migrasi di database...${NC}"
echo -e "${GREEN}Menjalankan perintah: DROP TABLE IF EXISTS _sqlx_migrations${NC}"

# Coba dengan PostgreSQL lokal
echo -e "${GREEN}Mencoba dengan PostgreSQL lokal...${NC}"
PGPASSWORD="$DB_PASSWORD" psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" -c 'DROP TABLE IF EXISTS _sqlx_migrations;' 2>/dev/null
LOCAL_RESULT=$?

if [ $LOCAL_RESULT -ne 0 ]; then
  # Jika PostgreSQL lokal gagal, coba dengan Docker
  echo -e "${YELLOW}PostgreSQL lokal gagal, mencoba dengan Docker...${NC}"
  cd ..
  docker compose exec -T postgres psql -U "$DB_USER" -d "$DB_NAME" -c 'DROP TABLE IF EXISTS _sqlx_migrations;' 2>/dev/null
  DOCKER_RESULT=$?
  cd migrations

  if [ $DOCKER_RESULT -ne 0 ]; then
    echo -e "${RED}✗ Gagal menghapus tabel migrasi. Pastikan database berjalan dan kredensial benar.${NC}"
    exit 1
  else
    echo -e "${GREEN}✓ Tabel migrasi berhasil dihapus menggunakan Docker${NC}"
  fi
else
  echo -e "${GREEN}✓ Tabel migrasi berhasil dihapus menggunakan PostgreSQL lokal${NC}"
fi

cd .. # Kembali ke direktori backend

echo -e "\n${GREEN}=== Persiapan migrasi selesai! ===${NC}"
echo -e "${YELLOW}Menjalankan migrasi ulang...${NC}"

# Jalankan migrasi dengan alamat IP Docker langsung
echo -e "${GREEN}Menjalankan migrasi dengan alamat IP Docker...${NC}"
export DATABASE_URL="postgresql://saas_user:saas_password@172.18.0.3:5432/saas_umkm_db"
echo -e "${GREEN}DATABASE_URL=$DATABASE_URL${NC}"
echo -e "${GREEN}Menjalankan: cargo run --bin migrate${NC}"
cargo run --bin migrate

if [ $? -eq 0 ]; then
  echo -e "${GREEN}✓ Migrasi berhasil dijalankan${NC}"
  echo -e "\n${YELLOW}Sekarang coba jalankan server:${NC}"
  echo -e "${GREEN}cargo run --bin server${NC}"
else
  echo -e "${RED}✗ Gagal menjalankan migrasi.${NC}"
  echo -e "${YELLOW}Silakan periksa kembali struktur file migrasi atau koneksi database.${NC}"
  exit 1
fi
