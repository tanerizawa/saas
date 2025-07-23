# Arsitektur Platform SaaS UMKM

## Ringkasan Eksekutif

Platform SaaS UMKM adalah sistem manajemen perizinan dan operasional bisnis yang dirancang khusus untuk Usaha Mikro, Kecil, dan Menengah (UMKM) di Indonesia. Platform ini menggunakan **arsitektur monolitik** dengan prinsip-prinsip Domain-Driven Design (DDD) dan Hexagonal Architecture untuk memastikan keamanan, skalabilitas, dan kemudahan penggunaan.

## Prinsip Arsitektur

### 1. Monolitik dengan Domain Boundaries yang Jelas

Platform menggunakan pendekatan **Modular Monolith** dengan keunggulan:

- **Kesederhanaan Pengembangan**: Single codebase yang mudah dikelola
- **Efisiensi Resource**: Overhead lebih rendah, cocok untuk VPS 8GB RAM
- **Kompleksitas Operasional Rendah**: Lebih sedikit komponen untuk dimonitor
- **Konsistensi Data**: Semua operasi menggunakan database yang sama

### 2. Domain-Driven Design (DDD)

- **Bounded Contexts**: Domain (Users, Licensing, Business, Finance) dengan batasan jelas
- **Aggregates**: Entitas terkait dikelompokkan untuk menjaga konsistensi
- **Value Objects**: Menggunakan newtype pattern untuk type safety (`struct UserId(Uuid)`)
- **Domain Services**: Logika bisnis kompleks yang melibatkan beberapa entity

### 3. Hexagonal Architecture (Ports & Adapters)

- **Domain Layer**: Business logic murni tanpa dependensi eksternal
- **Application Layer**: Use cases dan orchestration (commands/queries)
- **Infrastructure Layer**: Database, web services, external APIs

## Diagram Arsitektur

```
┌─────────────────────────────────────────────────┐
│                 PRESENTATION                    │
│              (Web Handlers)                     │
│    ┌─────────────────────────────────────────┐  │
│    │     Next.js Frontend (React)            │  │
│    │   - TypeScript + Tailwind CSS          │  │
│    │   - Server Components                  │  │
│    │   - API Routes                         │  │
│    └─────────────────────────────────────────┘  │
├─────────────────────────────────────────────────┤
│                 APPLICATION                     │
│           (Commands & Queries)                  │
│    ┌─────────────────────────────────────────┐  │
│    │     CQRS Pattern Implementation        │  │
│    │   - Command Handlers (Write)           │  │
│    │   - Query Handlers (Read)              │  │
│    │   - Event Sourcing                     │  │
│    └─────────────────────────────────────────┘  │
├─────────────────────────────────────────────────┤
│                   DOMAIN                        │
│        (Entities, Value Objects,                │
│         Repository Interfaces)                  │
│    ┌─────────────────────────────────────────┐  │
│    │     Domain-Driven Design (DDD)         │  │
│    │   - User, License, Business Entities   │  │
│    │   - Value Objects (Email, NIB, etc)    │  │
│    │   - Domain Services                    │  │
│    └─────────────────────────────────────────┘  │
├─────────────────────────────────────────────────┤
│               INFRASTRUCTURE                    │
│      (Database, Repository Impl,                │
│       External Services)                        │
│    ┌─────────────────────────────────────────┐  │
│    │     Axum Web Framework + SQLx          │  │
│    │   - PostgreSQL Database                │  │
│    │   - Redis Caching                      │  │
│    │   - External API Integration           │  │
│    └─────────────────────────────────────────┘  │
└─────────────────────────────────────────────────┘
```

## Arsitektur Sistem

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Web Browser   │    │   Mobile App    │    │  External APIs  │
└─────────┬───────┘    └─────────┬───────┘    └─────────┬───────┘
          │                      │                      │
          └──────────────────────┼──────────────────────┘
                                 │
                    ┌─────────────┴───────────┐
                    │       Caddy Proxy      │
                    │   (HTTPS, Load Bal.)   │
                    └─────────────┬───────────┘
                                 │
                ┌────────────────┼────────────────┐
                │                │                │
    ┌───────────┴─────────┐    ┌─┴──────────────┐ │
    │   Next.js Frontend  │    │  Rust Backend  │ │
    │   (TypeScript)      │    │   (Axum)       │ │
    └─────────────────────┘    └─┬──────────────┘ │
                                 │                │
                    ┌────────────┴────────────┐   │
                    │    PostgreSQL DB        │   │
                    │  (Optimized for 8GB)    │   │
                    └───────────┬─────────────┘   │
                                │                 │
                    ┌───────────┴─────────────┐   │
                    │    Redis Cache          │   │
                    │    (Performance)        │   │
                    └─────────────────────────┘   │
                                                  │
                    ┌─────────────────────────────┘
                    │     Monitoring Stack        │
                    │    (Prometheus/Grafana)     │
                    └─────────────────────────────┘
```

## Komponen Utama

### 1. Frontend (Next.js)

- **App Router**: Struktur routing modern berbasis direktori
- **TypeScript**: Type safety untuk meminimalisir bug
- **Tailwind CSS**: Styling yang konsisten dan efisien
- **Context API**: State management (AuthContext, dll)
- **React Hook Form**: Form validation dengan Zod

### 2. Backend (Rust/Axum)

- **Domain Layer**: Entitas bisnis dan logika domain
- **Application Layer**: Command/Query handlers (CQRS)
- **Infrastructure Layer**: Database, caching, dan integrasi eksternal
- **Web Layer**: REST API endpoints dengan Axum

### 3. Database

- **PostgreSQL**: Database relasional utama
- **SQLx**: Query database dengan compile-time verification
- **Migrasi**: Evolusi skema database terstruktur

### 4. Keamanan

- **JWT Authentication**: Token-based authentication
- **RBAC**: Role-based access control
- **Argon2**: Password hashing yang aman
- **Rate Limiting**: Pembatasan request untuk mencegah abuse

## Skalabilitas & Performa

Meskipun menggunakan arsitektur monolitik, platform dirancang dengan skalabilitas dan performa sebagai prioritas:

1. **Horizontal Scaling**:

   - Stateless design memungkinkan multiple instance
   - Load balancing dengan Kubernetes

2. **Optimasi Performa**:

   - Redis caching untuk mengurangi beban database
   - Database indexing untuk query cepat
   - Optimasi aset frontend (code splitting, lazy loading)

3. **Monitoring**:
   - Prometheus untuk metrics collection
   - Grafana untuk visualisasi dan alerting
   - Distributed tracing untuk analisis performa

## Deployment

Deployment menggunakan pendekatan cloud-native dengan:

1. **Containerization**:

   - Docker untuk aplikasi dan layanan
   - Kubernetes untuk orchestration

2. **CI/CD Pipeline**:

   - GitHub Actions untuk otomatisasi
   - Automated testing dan deployment

3. **Environment Isolation**:
   - Development, Staging, dan Production
   - Environment-specific configuration

---

_Diperbarui: Juli 23, 2025_
