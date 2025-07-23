#!/bin/bash

# Warna untuk output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Load environment variables
if [ -f "../.env" ]; then
  source ../.env
fi

# Set default values jika tidak ada di environment
DB_USER=${DB_USER:-saas_user}
DB_PASSWORD=${DB_PASSWORD:-saas_password}
DB_NAME=${DB_NAME:-saas_umkm_db}
DB_HOST=${DB_HOST:-localhost}
DB_PORT=${DB_PORT:-5432}

echo -e "${YELLOW}=== SAAS UMKM Platform - Database Cleanup Script ===${NC}"
echo -e "${YELLOW}Script ini akan menghapus database dan membuat ulang. Semua data akan hilang!${NC}"
echo

# Konfirmasi sebelum melanjutkan
read -p "Anda yakin ingin menghapus database $DB_NAME? (y/n): " CONFIRM
if [[ "$CONFIRM" != "y" && "$CONFIRM" != "Y" ]]; then
  echo -e "${YELLOW}Operasi dibatalkan.${NC}"
  exit 0
fi

# Periksa apakah Docker berjalan dan PostgreSQL container aktif
echo -e "\n${YELLOW}[1/5] Memeriksa status PostgreSQL container...${NC}"
if docker compose ps | grep -q "postgres"; then
  echo -e "${GREEN}✓ Container PostgreSQL berjalan${NC}"
else
  echo -e "${RED}✗ Container PostgreSQL tidak berjalan${NC}"
  echo -e "${YELLOW}  Menjalankan docker compose up -d untuk PostgreSQL...${NC}"
  cd ..
  docker compose up -d postgres
  sleep 5
  cd backend
fi

# Drop database jika sudah ada
echo -e "\n${YELLOW}[2/5] Menghapus database jika sudah ada...${NC}"
docker compose exec -T postgres psql -U "$DB_USER" -c "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname='$DB_NAME';" postgres
docker compose exec -T postgres psql -U "$DB_USER" -c "DROP DATABASE IF EXISTS $DB_NAME;" postgres

if [ $? -eq 0 ]; then
  echo -e "${GREEN}✓ Database $DB_NAME berhasil dihapus${NC}"
else
  echo -e "${RED}✗ Gagal menghapus database. Pastikan user memiliki hak akses yang cukup.${NC}"
  exit 1
fi

# Buat database baru
echo -e "\n${YELLOW}[3/5] Membuat database baru...${NC}"
docker compose exec -T postgres psql -U "$DB_USER" -c "CREATE DATABASE $DB_NAME;" postgres

if [ $? -eq 0 ]; then
  echo -e "${GREEN}✓ Database $DB_NAME berhasil dibuat${NC}"
else
  echo -e "${RED}✗ Gagal membuat database. Pastikan user memiliki hak akses yang cukup.${NC}"
  exit 1
fi

# Membuat ekstensi PostgreSQL yang dibutuhkan
echo -e "\n${YELLOW}[4/5] Membuat ekstensi PostgreSQL yang dibutuhkan...${NC}"
docker compose exec -T postgres psql -U "$DB_USER" -d "$DB_NAME" -c "CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\";"
docker compose exec -T postgres psql -U "$DB_USER" -d "$DB_NAME" -c "CREATE EXTENSION IF NOT EXISTS \"pg_trgm\";"

if [ $? -eq 0 ]; then
  echo -e "${GREEN}✓ Ekstensi PostgreSQL berhasil dibuat${NC}"
else
  echo -e "${RED}✗ Gagal membuat ekstensi PostgreSQL.${NC}"
  exit 1
fi

# Jalankan migrasi awal jika file migrations ada
echo -e "\n${YELLOW}[5/5] Menjalankan migrasi awal...${NC}"
if [ -d "migrations" ]; then
  echo -e "${YELLOW}Menjalankan migrasi dengan sqlx...${NC}"
  cargo run --bin migrate
  
  if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Migrasi berhasil dijalankan${NC}"
  else
    echo -e "${RED}✗ Gagal menjalankan migrasi.${NC}"
    exit 1
  fi
else
  echo -e "${YELLOW}Direktori migrations tidak ditemukan. Lewati migrasi.${NC}"
fi

echo -e "\n${GREEN}=== Database berhasil dibersihkan dan dipersiapkan untuk pengembangan! ===${NC}"
echo -e "${YELLOW}Informasi koneksi database:${NC}"
echo -e "  Database Host: ${GREEN}$DB_HOST${NC}"
echo -e "  Database Port: ${GREEN}$DB_PORT${NC}"
echo -e "  Database Name: ${GREEN}$DB_NAME${NC}"
echo -e "  Database User: ${GREEN}$DB_USER${NC}"
echo -e "  Database Password: ${GREEN}$DB_PASSWORD${NC}"
echo
echo -e "${YELLOW}Connection string untuk Rust:${NC}"
echo -e "  ${GREEN}postgresql://$DB_USER:$DB_PASSWORD@$DB_HOST:$DB_PORT/$DB_NAME${NC}"
