# Dokumentasi CI/CD Pipeline untuk Platform SaaS UMKM

Dokumen ini memberikan panduan komprehensif tentang pipeline Continuous Integration dan Continuous Deployment (CI/CD) yang diimplementasikan untuk platform SaaS UMKM menggunakan GitHub Actions. Pipeline dirancang untuk mendukung pendekatan arsitektur monolitik hingga deployment produksi.

## Overview Pipeline

CI/CD pipeline mengotomatisasi proses testing, building, dan deployment platform SaaS UMKM monolitik. Pipeline terdiri dari tahapan-tahapan berikut:

1. **Testing** - Menjalankan unit dan integration test untuk backend dan frontend
2. **Building** - Membangun Docker image untuk aplikasi monolitik
3. **Security Scanning** - Memindai kerentanan dalam kode dan dependensi
4. **Deployment** - Men-deploy aplikasi ke lingkungan Kubernetes

## Konfigurasi Workflow

Workflow utama didefinisikan dalam file `.github/workflows/ci-cd.yaml` dan dipicu oleh:

- Push event ke branch `main`
- Pull request yang menargetkan branch `main`
- Manual dispatch melalui UI GitHub

## Jobs dan Tahapan

### 1. Backend Testing

Job `test-backend`:

- Menyiapkan container service PostgreSQL untuk database test
- Menginstal Rust dan dependensi
- Menjalankan test suite dengan `cargo test`
- Melakukan code linting dengan `cargo clippy`

### 2. Frontend Testing

Job `test-frontend`:

- Menyiapkan lingkungan Node.js
- Menginstal dependensi dengan `npm ci`
- Menjalankan test Jest
- Melakukan pengecekan tipe TypeScript
- Menjalankan ESLint untuk kualitas kode

### 3. Building dan Pushing Docker Image

Job:

- `build-and-push-monolith` - Membangun Docker image monolitik yang mengandung backend Rust dan frontend Next.js

Job ini:

- Melakukan login ke GitHub Container Registry (ghcr.io)
- Membangun Docker image monolitik dengan tag yang sesuai
- Mendorong image ke registry

### 4. Security Scanning

- Menjalankan pemindaian kerentanan dependensi
- Melakukan Static Application Security Testing (SAST)
- Memindai kerentanan Docker image

### 5. Deployment

Mendeploy aplikasi ke lingkungan yang berbeda berdasarkan trigger:

- Pull Requests: Deploy ke lingkungan development
- Push ke main: Deploy ke lingkungan staging
- Release tags: Deploy ke lingkungan production

## Konfigurasi Environment

Workflow menggunakan variabel lingkungan dan secret berikut:

### Variabel Lingkungan

- `REGISTRY`: URL registry container (ghcr.io)
- `MONOLITH_IMAGE_NAME`: Nama repository image monolitik
- `CLUSTER_NAME`: Nama cluster Kubernetes

### Secret yang Dibutuhkan

- `KUBECONFIG`: Konfigurasi Kubernetes untuk deployment
- `REGISTRY_TOKEN`: Token untuk autentikasi GitHub Container Registry
- `DATABASE_URL`: Connection string PostgreSQL untuk test
- `REDIS_URL`: Connection string Redis

## Menggunakan Pipeline CI/CD

### Untuk Developer

1. **Pengembangan Branch**

   - Buat feature branch dari `main`
   - Kembangkan dan commit perubahan Anda
   - Push branch Anda ke GitHub
   - Buat pull request ke `main`

2. **Alur Kerja Pull Request**
   - CI/CD akan otomatis menjalankan test pada PR Anda
   - Periksa status PR untuk hasil test
   - Perbaiki test yang gagal atau masalah linting
   - PR dapat digabungkan setelah semua pemeriksaan lulus

### Untuk DevOps / SRE

1. **Pemantauan Workflow**

   - Pantau jalannya workflow di GitHub Actions
   - Periksa status deployment di Kubernetes
   - Tinjau log workflow untuk kesalahan

2. **Deployment Manual**
   - Gunakan workflow_dispatch untuk memicu deployment secara manual
   - Pilih lingkungan target
   - Pantau kemajuan deployment

## Memperluas Pipeline

Untuk menambahkan langkah atau job baru ke pipeline:

1. Edit file `.github/workflows/ci-cd.yaml`
2. Tambahkan job baru mengikuti struktur YAML
3. Pastikan dependensi antar job didefinisikan dengan benar
4. Uji perubahan pada feature branch sebelum menggabungkan ke main

## Pemecahan Masalah

### Masalah Umum

1. **Test Backend Gagal**

   - Periksa apakah layanan PostgreSQL berjalan dengan benar
   - Verifikasi variabel lingkungan untuk koneksi database
   - Tinjau log test untuk kesalahan spesifik

2. **Test Frontend Gagal**

   - Periksa kompatibilitas versi Node.js
   - Verifikasi dependensi package.json
   - Cari kegagalan test Jest

3. **Kegagalan Deployment**
   - Verifikasi KUBECONFIG telah diatur dengan benar
   - Periksa batas sumber daya Kubernetes atau kendala
   - Pastikan dependensi layanan tersedia di cluster

## Pemantauan dan Metrik

Pipeline CI/CD melacak beberapa metrik kunci:

- Durasi build
- Cakupan test
- Deployment gagal vs berhasil
- Frekuensi deployment

Metrik ini dapat diakses melalui insights GitHub Actions.

---

_Dokumentasi dibuat: 23 Juli 2025_  
_Terakhir diperbarui: 28 Juli 2025_
