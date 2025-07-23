#!/bin/bash
# Script start untuk Render.com

# Set warna untuk output
GREEN="\033[0;32m"
YELLOW="\033[1;33m"
RED="\033[0;31m"
NC="\033[0m" # No Color

echo -e "${YELLOW}====================================================${NC}"
echo -e "${YELLOW}🚀 Memulai server backend di Render.com${NC}"
echo -e "${YELLOW}====================================================${NC}"

# Pastikan kita berada di direktori backend
if [[ "$(basename $(pwd))" != "backend" ]]; then
    echo -e "${YELLOW}Pindah ke direktori backend...${NC}"
    cd backend || { echo -e "${RED}❌ Direktori backend tidak ditemukan.${NC}"; exit 1; }
fi

# Menjalankan migrasi database
echo -e "${YELLOW}Menjalankan migrasi database...${NC}"
./target/release/migrate

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Migrasi database berhasil!${NC}"
else
    echo -e "${RED}⚠️ Migrasi database gagal, tetapi akan melanjutkan menjalankan server...${NC}"
fi

# Menjalankan server backend
echo -e "${YELLOW}Memulai server backend...${NC}"
PORT="${PORT:-8000}"
echo -e "${YELLOW}Server akan berjalan di port: ${PORT}${NC}"

# Atur RUST_LOG jika belum diatur
export RUST_LOG="${RUST_LOG:-info}"
echo -e "${YELLOW}Level logging: ${RUST_LOG}${NC}"

# Jalankan server
echo -e "${YELLOW}Menjalankan server...${NC}"
./target/release/server
