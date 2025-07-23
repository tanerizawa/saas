# Panduan Onboarding Developer SaaS UMKM

Selamat datang di tim pengembangan SaaS UMKM! Dokumen ini akan membantu Anda memahami project ini dan memulai kontribusi dengan cepat.

## Tentang Project

SaaS UMKM adalah platform berbasis cloud untuk membantu Usaha Mikro, Kecil, dan Menengah (UMKM) di Indonesia mengelola bisnis mereka. Platform ini menyediakan fitur manajemen keuangan, inventaris, pelanggan, dan lisensi dalam satu aplikasi terintegrasi.

## Arsitektur

Project ini menggunakan **arsitektur monolitik** dengan prinsip **Domain-Driven Design**:

- **Backend**: Rust dengan kerangka kerja Axum
- **Frontend**: Next.js (React)
- **Database**: PostgreSQL
- **Deployment**: Docker dan Kubernetes

Untuk detail lebih lanjut, lihat [Ringkasan Arsitektur](./docs/arsitektur-ringkasan.md) dan [Dokumentasi Arsitektur Monolitik](./docs/architecture-monolith.md).

## Memulai Pengembangan

### Prasyarat

Pastikan Anda telah menginstal:

- Rust (1.70.0+)
- Node.js (18.0.0+)
- Docker dan Docker Compose
- PostgreSQL (untuk pengembangan lokal)
- Git

### Setup Lingkungan Pengembangan

1. **Clone repository**:

```bash
git clone https://github.com/saas-umkm/platform.git
cd platform
```

2. **Setup lingkungan pengembangan otomatis**:

```bash
./scripts/setup-dev.sh
```

Atau jalankan task VS Code: "🛠️ Setup Development Environment"

### Menjalankan Aplikasi

#### Menggunakan Tasks VS Code

Kita telah menyiapkan task VS Code untuk memudahkan pengembangan:

1. **Backend**:

   - "🦀 Backend: Check" - Menjalankan cargo check
   - "🦀 Backend: Build" - Membangun aplikasi backend
   - "🦀 Backend: Run Server" - Menjalankan server backend
   - "🦀 Backend: Run Migration" - Menjalankan migrasi database

2. **Frontend**:

   - "⚛️ Frontend: Install Dependencies" - Menginstal dependensi frontend
   - "⚛️ Frontend: Dev Server" - Menjalankan server development frontend
   - "⚛️ Frontend: Build" - Membangun aplikasi frontend

3. **Docker**:
   - "🐳 Docker: Start Services" - Menjalankan layanan pendukung dengan Docker
   - "🐳 Docker: Stop Services" - Menghentikan layanan Docker
   - "🐳 Docker: View Logs" - Melihat log container Docker

#### Manual

Anda juga dapat menjalankan aplikasi secara manual:

```bash
# Jalankan layanan pendukung
docker-compose up -d

# Backend
cd backend
cargo run --bin server

# Frontend (di terminal terpisah)
cd frontend
npm install
npm run dev
```

## Struktur Kode

```
saas-umkm/
├── backend/             # Aplikasi backend Rust monolitik
│   ├── src/
│   │   ├── domain/      # Model domain bisnis
│   │   ├── application/ # Layanan aplikasi dan use case
│   │   └── infrastructure/ # Implementasi teknis
│   └── migrations/      # Database migrations
├── frontend/           # Aplikasi frontend Next.js
│   ├── src/
│   │   ├── app/        # Komponen Next.js App Router
│   │   ├── components/ # Komponen UI yang dapat digunakan kembali
│   │   └── lib/        # Utilitas dan klien API
└── infrastructure/     # Konfigurasi deployment
```

## Workflow Pengembangan

1. **Buat branch fitur baru** dari `main`
2. **Implementasikan perubahan** Anda
3. **Jalankan test** untuk memastikan kualitas kode
4. **Submit pull request** ke `main`
5. **Code review** oleh tim
6. **Merge** setelah persetujuan

## Test

### Backend

```bash
cd backend
cargo test
```

Atau gunakan task VS Code "🦀 Backend: Test"

### Frontend

```bash
cd frontend
npm run test
```

## Deployment

Deployment dihandle oleh pipeline CI/CD menggunakan GitHub Actions. Lihat [Dokumentasi CI/CD](./docs/ci-cd-pipeline.md) untuk detail lengkap.

## Resources Penting

- [Arsitektur Monolitik](./docs/architecture-monolith.md)
- [Panduan API](./docs/api-guide.md)
- [Troubleshooting](./docs/TROUBLESHOOTING.md)
- [Standar Kode](./docs/coding-standards.md)

## Kontak

Jika Anda memiliki pertanyaan, silakan hubungi:

- **Lead Developer**: lead.dev@saasumkm.id
- **Project Manager**: pm@saasumkm.id
- **DevOps**: devops@saasumkm.id

## Lisensi

Kode ini dilisensikan di bawah [Lisensi MIT](LICENSE).

---

Selamat coding! 🚀
