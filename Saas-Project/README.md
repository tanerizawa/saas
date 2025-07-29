# SaaS UMKM Platform - Production Ready

**Status**: ✅ Production Ready  
**Version**: 1.0.0  
**Architecture**: Monolithic Rust Backend + Next.js Frontend  
**Database**: PostgreSQL 15  

Platform SaaS UMKM adalah sistem manajemen perizinan dan operasional bisnis yang dirancang khusus untuk Usaha Mikro, Kecil, dan Menengah (UMKM) di Indonesia. Platform menggunakan arsitektur monolitik dengan prinsip-prinsip Domain-Driven Design (DDD) dan Hexagonal Architecture untuk memastikan keamanan, skalabilitas, dan kemudahan penggunaan.

## 🚀 Quick Start

### One-Command Deployment
```bash
# Production deployment
./deploy.sh --production

# Development setup
./deploy.sh --development
```

### Manual Setup
```bash
# 1. Clone and setup
git clone <repository-url>
cd saas-project
cp .env.example .env

# 2. Start database
docker-compose up -d postgres

# 3. Run migrations
cd backend && sqlx migrate run

# 4. Start services
cargo run --bin server &
cd ../frontend && npm install && npm run dev
```

## 🏗️ Production Architecture

- **Backend**: Rust/Axum with JWT authentication, Argon2 password hashing
- **Frontend**: Next.js with TypeScript, WCAG 2.1 AA compliant
- **Database**: PostgreSQL with 22 production-ready migrations
- **Security**: Production-grade security features implemented
- **Deployment**: Docker containerized with SSL/HTTPS support
- **Monitoring**: Health checks and structured logging

## 📡 API Endpoints

### Authentication (Active ✅)
- `POST /api/v1/auth/register` - User registration
- `POST /api/v1/auth/login` - User login  
- `POST /api/v1/auth/refresh` - Token refresh
- `POST /api/v1/auth/logout` - User logout
- `POST /api/v1/auth/reset-password` - Password reset
- `GET /api/v1/auth/health` - Auth system health

### System (Active ✅)
- `GET /health` - System health check
- `GET /api/v1/users/profile` - User profile

### Business Modules (Next Phase �)
- Companies Management - Schema ready
- Financial Management - Schema ready  
- License Management - Schema ready

## 🔒 Security Features

- **JWT Authentication**: Secure token-based authentication
- **Password Security**: Argon2 hashing with proper salts
- **Input Validation**: Comprehensive validation and sanitization
- **CORS Policy**: Production-configured cross-origin settings
- **Error Handling**: No sensitive information leakage
- **Health Monitoring**: System health endpoints

## � Production Status

| Component | Status | Description |
|-----------|--------|-------------|
| Authentication | ✅ Complete | JWT + Argon2, fully tested |
| User Management | ✅ Complete | Profile management working |
| Database Schema | ✅ Complete | 22 migrations, production ready |
| Security | ✅ Complete | Production-grade security |
| Documentation | ✅ Complete | Comprehensive guides |
| Deployment | ✅ Complete | One-command deployment |
| Health Monitoring | ✅ Complete | System health endpoints |

## 📚 Documentation

- **[Production Guide](PRODUCTION-GUIDE.md)** - Complete deployment and operation guide
- **[Architecture](docs/architecture-monolith.md)** - Technical architecture details
- **[Troubleshooting](docs/TROUBLESHOOTING.md)** - Common issues and solutions
- **[API Testing](test-auth-production.sh)** - Authentication endpoint testing

## 🛠️ Development

### Prerequisites
- Rust 1.73+
- Node.js 18+
- PostgreSQL 15+
- Docker & Docker Compose

### Tech Stack
- **Backend**: Rust, Axum, SQLx, Tokio
- **Frontend**: Next.js, TypeScript, Tailwind CSS
- **Database**: PostgreSQL with migrations
- **Testing**: Vitest, React Testing Library
- **Deployment**: Docker, Docker Compose

## � Support

For production deployment support or technical questions, see:
- **[Production Guide](PRODUCTION-GUIDE.md)** - Complete operational guide
- **[Architecture Documentation](docs/)** - Technical details
- **Health Endpoints**: `/health` and `/api/v1/auth/health`

---

**Production Ready**: ✅ Ready for deployment  
**Security Grade**: Production-grade authentication and security  
**Deployment**: One-command deployment with `./deploy.sh --production`

## 📑 Ringkasan

Platform SaaS UMKM adalah solusi terintegrasi yang membantu Usaha Mikro, Kecil, dan Menengah (UMKM) di Indonesia dalam:

- Mengelola izin usaha dan dokumen legal (NIB, SIUP, TDP, NPWP)
- Mengelola profil dan informasi perusahaan
- Monitoring kepatuhan regulasi
- Manajemen operasional bisnis
- Pelaporan dan analitik

Platform dibangun menggunakan **arsitektur monolitik** dengan prinsip-prinsip Domain-Driven Design, memberikan solusi yang robust namun tetap mudah di-maintain.

## 🎯 Fitur Utama

### 📝 Pengelolaan Izin Usaha

- **NIB (Nomor Induk Berusaha)** - Registrasi dan tracking status pengajuan
- **SIUP (Surat Izin Usaha Perdagangan)** - Manajemen perizinan usaha
- **TDP (Tanda Daftar Perusahaan)** - Pendaftaran dan update informasi perusahaan
- **NPWP** - Integrasi dengan sistem perpajakan

### 🏢 Manajemen Bisnis

- **Profil Perusahaan** - Informasi detail dan verifikasi
- **Dokumen Digital** - Penyimpanan dan manajemen dokumen
- **Role-based Access Control** - Pengaturan hak akses pengguna
- **Analytics Dashboard** - Visualisasi data bisnis

### 💰 Sistem Keuangan

- **Integrasi Payment Gateway** - Pembayaran dengan Midtrans
- **Manajemen Pajak** - Perhitungan dan pelaporan
- **Laporan Keuangan** - Laporan real-time
- **Invoice & Pembayaran** - Pelacakan status pembayaran

### 🔐 Keamanan & Autentikasi

- **JWT Authentication** - Manajemen token aman
- **Role-based Authorization** - Kontrol akses berbasis peran
- **Two-factor Authentication** - Keamanan tambahan
- **Audit Logging** - Pelacakan aktivitas sistem

## 🏗️ Arsitektur

Platform SaaS UMKM menggunakan **arsitektur monolitik modular** dengan pendekatan Domain-Driven Design:

```
┌─────────────────────────────────────────────────┐
│                 PRESENTATION                    │
│    ┌─────────────────────────────────────────┐  │
│    │     Next.js Frontend (React)            │  │
│    └─────────────────────────────────────────┘  │
├─────────────────────────────────────────────────┤
│                 APPLICATION                     │
│    ┌─────────────────────────────────────────┐  │
│    │     CQRS Pattern Implementation        │  │
│    └─────────────────────────────────────────┘  │
├─────────────────────────────────────────────────┤
│                   DOMAIN                        │
│    ┌─────────────────────────────────────────┐  │
│    │     Domain-Driven Design (DDD)         │  │
│    └─────────────────────────────────────────┘  │
├─────────────────────────────────────────────────┤
│               INFRASTRUCTURE                    │
│    ┌─────────────────────────────────────────┐  │
│    │     Axum Web Framework + SQLx          │  │
│    └─────────────────────────────────────────┘  │
└─────────────────────────────────────────────────┘
```

Untuk dokumentasi arsitektur lengkap, lihat [Architecture Documentation](docs/architecture-monolith.md).

## 🛠️ Quick Start

### Prasyarat

- [Rust](https://www.rust-lang.org/) (1.74+)
- [Node.js](https://nodejs.org/) (18+)
- [Docker](https://www.docker.com/) dan Docker Compose
- [PostgreSQL](https://www.postgresql.org/) (15+)

### Setup Development Environment

```bash
# Clone repository
git clone https://github.com/username/saas-umkm.git
cd saas-umkm

# Setup development environment
./scripts/setup-dev.sh

# Start services dengan Docker
docker-compose up -d

# Build backend
cd backend
cargo build

# Run database migrations
cargo run --bin migrate

# Run backend server
cargo run --bin server

# Di terminal terpisah, jalankan frontend
cd ../frontend
npm install
npm run dev
```

## 🔧 Development Guide

### VS Code Configuration

Kami menyarankan menggunakan VS Code dengan extensions:

- rust-analyzer (Rust)
- ESLint (JavaScript/TypeScript)
- Prettier (Code formatting)
- Docker (Docker integration)

Workspace settings tersedia di `saas-umkm.code-workspace`.

### Backend Development (Rust)

```bash
# Run backend checks
cargo check

# Run tests
cargo test

# Run server dengan hot reload
cargo watch -x run
```

### Frontend Development (Next.js)

```bash
# Run development server
npm run dev

# Type checking
npm run type-check

# Run tests
npm test

# Build untuk production
npm run build
```

## 🐳 Deployment & DevOps

### Docker Deployment

```bash
# Build images
docker-compose -f docker-compose.prod.yml build

# Run services
docker-compose -f docker-compose.prod.yml up -d
```

### Kubernetes Deployment

```bash
# Apply Kubernetes configurations
kubectl apply -f infrastructure/kubernetes/

# Check deployment status
kubectl get pods -n saas-umkm
```

Lihat [Kubernetes Deployment Guide](docs/kubernetes-deployment-guide.md) untuk detail lengkap.

### CI/CD Pipeline

Platform menggunakan GitHub Actions untuk otomatisasi CI/CD:

- Automated testing untuk backend dan frontend
- Docker image building dan publishing
- Deployment ke lingkungan development, staging, dan production
- Security scanning

Lihat [CI/CD Pipeline Documentation](docs/ci-cd-pipeline.md) untuk detail.

## 📊 Monitoring & Observability

Sistem monitoring menggunakan:

- **Prometheus**: Pengumpulan metrics
- **Grafana**: Visualisasi dan alerting
- **ELK Stack**: Log aggregation dan analysis
- **Jaeger**: Distributed tracing

Akses dashboard:

- Prometheus: http://localhost:9090
- Grafana: http://localhost:3000

## 🧪 Testing

### Backend Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_user_repository

# Run with logging
RUST_LOG=debug cargo test
```

### Frontend Testing

```bash
# Run tests
npm test

# Run tests with coverage
npm run test:coverage

# Update snapshots
npm run test:update
```

## 📚 API Documentation

API documentation tersedia dengan Swagger UI pada endpoint `/docs` saat aplikasi berjalan:

- Development: http://localhost:8000/docs
- Production: https://api.saas-umkm.id/docs

### Postman Collection

Untuk testing API dengan Postman, gunakan collection yang tersedia:

```bash
# Import collection file
postman/SAAS-UMKM-API.postman_collection.json
```

## 🔄 Status Proyek & Roadmap

Saat ini proyek berada pada **Phase 5: Production Deployment & Scaling**.

### Progress by Phase

1. ✅ **Foundation & Authentication** - 100% Complete
2. ✅ **License Management** - 100% Complete (Backend)
3. ✅ **Company Management** - 100% Complete
4. ✅ **Enhanced User Experience** - 100% Complete
5. 🚧 **Production Deployment & Scaling** - In Progress

Untuk detail status dan roadmap, lihat [Phase Status](docs/PHASE-5-IMPLEMENTATION-STATUS.md).

## 🤝 Contributing

1. Fork repository
2. Create feature branch: `git checkout -b feature/amazing-feature`
3. Commit changes: `git commit -m 'Add amazing feature'`
4. Push to branch: `git push origin feature/amazing-feature`
5. Open Pull Request

## 📝 License

Distributed under the MIT License. See `LICENSE` for more information.

---

Built with ❤️ for Indonesian SMEs
