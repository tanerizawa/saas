#!/bin/bash
# Script build untuk Render.com

# Set warna untuk output
GREEN="\033[0;32m"
YELLOW="\033[1;33m"
RED="\033[0;31m"
NC="\033[0m" # No Color

echo -e "${YELLOW}====================================================${NC}"
echo -e "${YELLOW}🚀 Memulai proses build backend di Render.com${NC}"
echo -e "${YELLOW}====================================================${NC}"

# Pastikan kita berada di direktori backend
if [[ "$(basename $(pwd))" != "backend" ]]; then
    echo -e "${YELLOW}Pindah ke direktori backend...${NC}"
    cd backend || { echo -e "${RED}❌ Direktori backend tidak ditemukan.${NC}"; exit 1; }
fi

# Tampilkan versi Rust
echo -e "${YELLOW}Versi Rust:${NC}"
rustc --version

# Tampilkan versi Cargo
echo -e "${YELLOW}Versi Cargo:${NC}"
cargo --version

# Build dalam mode release
echo -e "${YELLOW}Memulai build release...${NC}"
cargo build --release

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Build backend berhasil!${NC}"
else
    echo -e "${RED}❌ Build backend gagal.${NC}"
    exit 1
fi

# Tampilkan binary yang dihasilkan
echo -e "${YELLOW}Binary yang dihasilkan:${NC}"
ls -la target/release/server

echo -e "${YELLOW}====================================================${NC}"
echo -e "${GREEN}✅ Proses build backend selesai!${NC}"
echo -e "${YELLOW}====================================================${NC}"
