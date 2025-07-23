# SaaS UMKM Platform - Render.com Deployment

## Overview

Repository ini berisi SaaS platform untuk UMKM Indonesia, yang dapat di-deploy dengan mudah ke Render.com. Platform ini membantu UMKM dalam pengelolaan perizinan, operasi bisnis, dan kepatuhan keuangan.

## Arsitektur

- **Backend**: Rust dengan Axum framework, mengikuti pola Domain-Driven Design (DDD)
- **Frontend**: Next.js 14 dengan TypeScript dan Tailwind CSS
- **Database**: PostgreSQL
- **Cache**: Redis

## Deploy ke Render.com dengan Blueprint

### Langkah 1: Fork atau Clone Repository

```bash
git clone https://github.com/username/saas-umkm-platform.git
cd saas-umkm-platform
```

### Langkah 2: Persiapkan Deployment

Jalankan script persiapan deployment:

```bash
./prepare-render-deploy.sh
```

### Langkah 3: Push ke GitHub

```bash
git push origin main
```

### Langkah 4: Deploy di Render.com

1. Login ke [Render Dashboard](https://dashboard.render.com/)
2. Klik "New" > "Blueprint"
3. Hubungkan dengan repository GitHub Anda
4. Render akan otomatis membuat semua service yang didefinisikan dalam `render.yaml`

## Deployment Manual

Jika ingin melakukan deployment secara manual tanpa menggunakan Blueprint, ikuti petunjuk dalam [RENDER-DEPLOYMENT.md](RENDER-DEPLOYMENT.md)

## Variabel Lingkungan

### Backend

Lihat `.env.example` di direktori backend untuk daftar lengkap variabel lingkungan yang diperlukan.

### Frontend

Lihat `.env.example` di direktori frontend untuk daftar lengkap variabel lingkungan yang diperlukan.

## Koneksi Database

Setelah database PostgreSQL dibuat di Render.com, Anda perlu mengatur `DATABASE_URL` di environment variables untuk backend service.

Format URL:

```
postgresql://user:password@host:port/database
```

## Monitoring dan Logging

Render.com menyediakan dashboard untuk monitoring dan logging. Anda dapat melihat logs melalui Render Dashboard untuk setiap service.

## Scaling

Untuk meningkatkan performa, pertimbangkan untuk upgrade plan di Render.com dari free tier ke paid tier sesuai kebutuhan traffic dan resources.

## Support

Jika Anda mengalami masalah dengan deployment, silakan buat issue di repository ini atau hubungi tim support.
