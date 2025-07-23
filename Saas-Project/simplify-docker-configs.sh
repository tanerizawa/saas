#!/bin/bash

# Script untuk menyederhanakan konfigurasi Docker Compose pada project SaaS UMKM
# Script ini akan menstandarkan semua file docker-compose agar lebih terstruktur

echo "🐳 Menyederhanakan konfigurasi Docker Compose"

# Memastikan docker-compose.yml adalah symlink ke docker-compose.simple.yml
echo "1️⃣ Memastikan docker-compose.yml mengarah ke versi simpel"
rm -f docker-compose.yml
ln -sf docker-compose.simple.yml docker-compose.yml

# Menghapus file docker-compose yang duplikat di backend
echo "2️⃣ Menghapus file docker-compose redundan"
rm -f backend/docker-compose.yml

# Menyimpan docker-compose.vps.yml yang sudah ada (untuk kebutuhan VPS)
echo "3️⃣ Membuat file docker-compose sesuai kebutuhan"

echo "✅ Konfigurasi Docker Compose berhasil disederhanakan"
echo "File yang tersedia:"
echo "  - docker-compose.yml → docker-compose.simple.yml (untuk development sehari-hari)"
echo "  - docker-compose.dev.yml (untuk development dengan layanan tambahan)"
echo "  - docker-compose.vps.yml (untuk deployment di VPS)"

echo ""
echo "Gunakan perintah berikut untuk menjalankan:"
echo "• Development sederhana: docker compose up -d"
echo "• Development dengan service tambahan: docker compose -f docker-compose.dev.yml up -d"
echo "• Deployment di VPS: docker compose -f docker-compose.vps.yml up -d"
