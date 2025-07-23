# Deployment ke Render.com

Dokumen ini menjelaskan langkah-langkah untuk men-deploy aplikasi SaaS UMKM ke Render.com.

## Persiapan

1. Pastikan repository GitHub sudah siap dengan struktur berikut:

   - `backend/`: Kode backend Rust dengan Axum
   - `frontend/`: Kode frontend Next.js
   - `render.yaml`: File konfigurasi untuk Render.com

2. Buat akun di [Render.com](https://render.com) jika belum memilikinya.

## Langkah-langkah Deployment

### Opsi 1: Deployment Otomatis dengan Blueprint

1. Login ke dashboard Render.com
2. Pilih "New" > "Blueprint"
3. Hubungkan dengan repository GitHub
4. Render akan otomatis mendeteksi file `render.yaml` dan menyiapkan semua service

### Opsi 2: Deployment Manual

#### Backend (Rust)

1. Di dashboard Render.com, pilih "New" > "Web Service"
2. Hubungkan dengan repository GitHub
3. Konfigurasi:

   - **Name**: saas-umkm-backend
   - **Environment**: Rust
   - **Region**: Singapore
   - **Branch**: main
   - **Build Command**: `cd backend && cargo build --release`
   - **Start Command**: `cd backend && ./target/release/server`

4. Tambahkan Environment Variables:
   - `DATABASE_URL`: URL PostgreSQL dari database yang dibuat di Render
   - `REDIS_URL`: URL Redis dari service Redis yang dibuat di Render
   - `JWT_SECRET`: Secret key yang aman untuk JWT
   - `APP_HOST`: 0.0.0.0
   - `APP_PORT`: $PORT (Render menyediakan port secara dinamis)
   - `ENABLE_RATE_LIMITING`: false (untuk testing)
   - `RUST_LOG`: info

#### Frontend (Next.js)

1. Di dashboard Render.com, pilih "New" > "Web Service"
2. Hubungkan dengan repository GitHub
3. Konfigurasi:

   - **Name**: saas-umkm-frontend
   - **Environment**: Node
   - **Region**: Singapore
   - **Branch**: main
   - **Build Command**: `cd frontend && npm install && npm run build`
   - **Start Command**: `cd frontend && npm start`

4. Tambahkan Environment Variables:
   - `NODE_ENV`: production
   - `NEXT_PUBLIC_API_URL`: URL dari backend service yang telah dibuat

## Database dan Redis

1. Untuk Database PostgreSQL:

   - Pilih "New" > "PostgreSQL"
   - Beri nama "saas-umkm-db"
   - Pilih region "Singapore"
   - Pilih paket sesuai kebutuhan

2. Untuk Redis:
   - Pilih "New" > "Redis"
   - Beri nama "saas-umkm-redis"
   - Pilih region "Singapore"
   - Pilih paket sesuai kebutuhan

## Konfigurasi Tambahan

### Migrasi Database

Untuk migrasi database otomatis, tambahkan script migrasi ke dalam proses build di `render.yaml`:

```yaml
buildCommand: cd backend && cargo build --release && cargo run --bin migrate
```

### Custom Domain

Setelah deployment berhasil, Anda dapat mengonfigurasi custom domain melalui dashboard Render.com untuk masing-masing service.
