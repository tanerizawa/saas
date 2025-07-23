#!/bin/bash
# Script untuk mempersiapkan deployment ke Render.com

# Set warna untuk output
GREEN="\033[0;32m"
YELLOW="\033[1;33m"
RED="\033[0;31m"
NC="\033[0m" # No Color

echo -e "${YELLOW}====================================================${NC}"
echo -e "${YELLOW}🚀 Mempersiapkan Deployment ke Render.com${NC}"
echo -e "${YELLOW}====================================================${NC}"

# Cek apakah git diinstal
if ! command -v git &> /dev/null; then
    echo -e "${RED}❌ Git tidak ditemukan. Silakan instal git terlebih dahulu.${NC}"
    exit 1
fi

# Cek apakah dalam repositori git
if ! git rev-parse --is-inside-work-tree &> /dev/null; then
    echo -e "${RED}❌ Direktori ini bukan repositori git. Inisialisasi git terlebih dahulu.${NC}"
    git init
    echo -e "${GREEN}✅ Repositori git berhasil diinisialisasi.${NC}"
fi

# Memastikan semua perubahan sudah di-stage dan di-commit
echo -e "${YELLOW}Memeriksa status perubahan...${NC}"
if [[ -n $(git status --porcelain) ]]; then
    echo -e "${YELLOW}Ada perubahan yang belum di-commit:${NC}"
    git status --short
    
    echo -e "${YELLOW}Menambahkan semua perubahan...${NC}"
    git add .
    
    echo -e "${YELLOW}Membuat commit...${NC}"
    git commit -m "Persiapan deployment ke Render.com"
    
    echo -e "${GREEN}✅ Semua perubahan berhasil di-commit.${NC}"
else
    echo -e "${GREEN}✅ Tidak ada perubahan yang perlu di-commit.${NC}"
fi

# Memastikan bahwa backend dan frontend siap untuk di-deploy
echo -e "${YELLOW}Memeriksa backend...${NC}"
if [ -d "backend" ]; then
    cd backend
    
    # Cek apakah cargo tersedia
    if command -v cargo &> /dev/null; then
        echo -e "${YELLOW}Menjalankan cargo check untuk memastikan kode dapat dicompile...${NC}"
        cargo check
        if [ $? -eq 0 ]; then
            echo -e "${GREEN}✅ Backend siap untuk di-deploy.${NC}"
        else
            echo -e "${RED}❌ Backend memiliki error kompilasi. Silakan perbaiki terlebih dahulu.${NC}"
            exit 1
        fi
    else
        echo -e "${YELLOW}⚠️ Cargo tidak ditemukan, melewati pemeriksaan backend.${NC}"
    fi
    
    cd ..
else
    echo -e "${RED}❌ Direktori backend tidak ditemukan.${NC}"
    exit 1
fi

echo -e "${YELLOW}Memeriksa frontend...${NC}"
if [ -d "frontend" ]; then
    cd frontend
    
    # Cek apakah npm tersedia
    if command -v npm &> /dev/null; then
        echo -e "${YELLOW}Memeriksa package.json...${NC}"
        if [ -f "package.json" ]; then
            echo -e "${GREEN}✅ Frontend siap untuk di-deploy.${NC}"
        else
            echo -e "${RED}❌ File package.json tidak ditemukan di direktori frontend.${NC}"
            exit 1
        fi
    else
        echo -e "${YELLOW}⚠️ npm tidak ditemukan, melewati pemeriksaan frontend.${NC}"
    fi
    
    cd ..
else
    echo -e "${RED}❌ Direktori frontend tidak ditemukan.${NC}"
    exit 1
fi

# Memastikan file konfigurasi Render.com sudah ada
echo -e "${YELLOW}Memeriksa file konfigurasi Render.com...${NC}"
if [ -f "render.yaml" ]; then
    echo -e "${GREEN}✅ File render.yaml ditemukan.${NC}"
else
    echo -e "${RED}❌ File render.yaml tidak ditemukan. File ini diperlukan untuk deployment ke Render.com.${NC}"
    exit 1
fi

echo -e "${YELLOW}====================================================${NC}"
echo -e "${GREEN}✅ Semua persiapan untuk deployment ke Render.com selesai!${NC}"
echo -e "${YELLOW}====================================================${NC}"
echo
echo -e "${YELLOW}Langkah selanjutnya:${NC}"
echo -e "1. Push kode ke repository GitHub"
echo -e "   ${YELLOW}git push origin main${NC}"
echo
echo -e "2. Buat akun di Render.com jika belum memilikinya"
echo -e "   ${YELLOW}https://render.com${NC}"
echo
echo -e "3. Hubungkan repository GitHub dengan Render.com"
echo -e "   Pilih ${YELLOW}New > Blueprint${NC} di dashboard Render.com"
echo
echo -e "4. Render.com akan otomatis mendeteksi file render.yaml dan menyiapkan semua service"
echo
echo -e "5. Lihat dokumentasi lengkap di ${YELLOW}RENDER-DEPLOYMENT.md${NC}"
echo -e "${YELLOW}====================================================${NC}"
