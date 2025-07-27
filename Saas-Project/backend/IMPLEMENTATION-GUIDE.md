````markdown
# Backend - Panduan Perbaikan Kode

## PHASE 5 - FILE MANAGEMENT IMPLEMENTATION

### 1. Masalah Utama yang Sudah Diperbaiki

1. **Urutan inisialisasi di main.rs**

   - Memindahkan inisialisasi `cache_service` sebelum `license_repository`
   - Menghapus sementara penggunaan `new_with_cache` hingga implementasi dibuat

2. **Middleware Stack di create_app()**

   - Mengganti `layers` dengan `service_builder` dan perbaikan cara penanganan layering
   - Menggunakan `.into_inner()` untuk mengubah ServiceBuilder menjadi Layer

3. **PgConnectOptions yang tidak tersedia**

   - Menghapus metode yang tidak lagi didukung (`tcp_keepalives_*` dan `connect_timeout`)
   - Menggunakan metode yang masih didukung oleh sqlx terbaru

4. **DTO untuk License**
   - Membuat struktur `LicenseDto` untuk menjembatani perbedaan skema database dan domain model
   - Menambahkan implementasi konversi antara DTO dan domain entity

### 2. Implementasi File Management (Phase 5)

1. **File Handler**

   - Implementasi lengkap handler file di `src/infrastructure/web/handlers/files.rs`
   - Endpoint untuk upload, download, daftar, dan hapus file
   - Validasi tipe file dan keamanan untuk uploads
   - Streaming file untuk efisiensi download file besar

2. **File Repository**

   - Implementasi `FileRepository` dengan pola repository
   - Pemisahan metadata file (disimpan di database) dan konten file (disimpan di filesystem)
   - Penggunaan UUID untuk penamaan file yang aman
   - Migrasi database untuk tabel files

3. **Testing**
   - Unit tests untuk validasi file
   - Integration tests untuk API endpoints

### 3. Panduan Implementasi Repository dengan DTO

```rust
// Template untuk implementasi repository
async fn get_license_by_id(&self, id: Uuid) -> Result<Option<License>, sqlx::Error> {
    use crate::domain::dto::LicenseDto;

    let license_dto = sqlx::query_as!(
        LicenseDto,
        "SELECT * FROM licenses WHERE id = $1",
        id
    )
    .fetch_optional(&self.pool)
    .await?;

    // Konversi DTO ke domain entity jika ada
    Ok(license_dto.map(|dto| dto.into()))
}

async fn get_licenses_by_user(&self, user_id: Uuid) -> Result<Vec<License>, sqlx::Error> {
    use crate::domain::dto::LicenseDto;

    let licenses_dto = sqlx::query_as!(
        LicenseDto,
        "SELECT * FROM licenses WHERE user_id = $1 ORDER BY created_at DESC",
        user_id
    )
    .fetch_all(&self.pool)
    .await?;

    // Konversi semua DTO ke domain entities
    Ok(licenses_dto.into_iter().map(|dto| dto.into()).collect())
}
```

## PHASE 6 - FINANCIAL MANAGEMENT IMPLEMENTATION (COMPLETED)

### 1. Implementasi Yang Sudah Dilakukan

1. **Financial Module Domain**

   - Struktur domain model di `domain/finance_simplified.rs` dengan:
     - Transaction & Account entities
     - TransactionType & AccountType enums
     - Status enum untuk kondisi entities

2. **Financial Repository**

   - Implementasi `FinancialRepository` di `repositories/financial_repository.rs`:
     - CRUD operations untuk transaksi dan akun
     - Caching dengan Redis untuk performa optimal
     - Invalidasi cache otomatis saat data berubah

3. **Financial Handlers**

   - API endpoints di `handlers/finance_v2.rs`:
     - Create/Get/List Transactions
     - Create/Get/List Accounts
     - Financial summary reporting

4. **Database**
   - Migration file `20250725000001_financial_module_v2.sql`
   - Trigger untuk update saldo akun otomatis saat transaksi dibuat
   - Rollback script di `rollbacks/20250725000001_financial_module_v2.sql`

### 2. Contoh Penggunaan Financial Module

#### 2.1 Membuat Akun Keuangan Baru

```bash
curl -X POST http://localhost:8080/api/v1/finance/accounts \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Tabungan Utama",
    "account_type": "savings",
    "initial_balance": 1000000,
    "currency": "IDR"
  }'
```

#### 2.2 Membuat Transaksi

```bash
curl -X POST http://localhost:8080/api/v1/finance/transactions \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "account_id": "3f7e4567-e89b-12d3-a456-426614174000",
    "amount": 500000,
    "transaction_type": "deposit",
    "description": "Setoran bulanan",
    "category": "savings"
  }'
```

#### 2.3 Mendapatkan Ringkasan Keuangan

```bash
curl -X GET http://localhost:8080/api/v1/finance/reports/summary \
  -H "Authorization: Bearer $TOKEN"
```

### 3. Struktur Kode

```
src/
  ├── domain/
  │   └── finance_simplified.rs   # Domain model untuk finansial
  ├── infrastructure/
  │   ├── repositories/
  │   │   └── financial_repository.rs  # Repository untuk akses data finansial
  │   └── web/handlers/
  │       └── finance_v2.rs       # API handlers untuk endpoint finansial
  └── migrations/
      ├── 20250725000001_financial_module_v2.sql  # Migrasi database
      └── rollbacks/
          └── 20250725000001_financial_module_v2.sql  # Rollback script
```

## PHASE 7 - REPORTING & ANALYTICS (REKOMENDASI)

### 1. Rencana Implementasi

1. **Dashboard & Visualisasi**

   - Implementasi endpoint untuk data visualisasi
   - Agregasi data untuk tren dan insights
   - Dashboard summary dengan key metrics

2. **Advanced Reporting**

   - Laporan keuangan lengkap (bulanan, triwulanan, tahunan)
   - Analisis kategori pengeluaran dan pendapatan
   - Perbandingan periode dan tren

3. **Ekspor Data**

   - Ekspor laporan ke PDF menggunakan wkhtmltopdf atau similiar
   - Ekspor data transaksi ke Excel/CSV 
   - Scheduling laporan reguler

4. **Notifikasi & Alerts**
   - Event-based notifications (transaksi besar, saldo rendah)
   - Email notifications menggunakan lettre atau sendgrid
   - In-app notifications dengan WebSocket

### 2. Contoh Kode untuk Phase 7

```rust
// Contoh endpoint untuk ekspor data ke CSV
async fn export_transactions_csv(
    State(state): State<Arc<AppState>>,
    auth: AuthenticatedUser,
    Query(params): Query<ExportParams>,
) -> Result<impl IntoResponse, AppError> {
    // Dapatkan data transaksi
    let (transactions, _) = state.financial_repository()
        .get_transactions_by_user_date_range(
            *auth.user_id.as_uuid(),
            params.start_date,
            params.end_date,
        ).await?;
    
    // Generate CSV
    let mut wtr = csv::Writer::from_writer(vec![]);
    for tx in &transactions {
        wtr.serialize(TransactionExport::from(tx.clone()))?;
    }
    
    let csv_bytes = wtr.into_inner()?;
    
    // Return file
    let headers = HeaderMap::new()
        .insert(header::CONTENT_TYPE, "text/csv".parse().unwrap())
        .insert(header::CONTENT_DISPOSITION, 
                format!("attachment; filename=\"transactions_{}.csv\"", 
                        chrono::Utc::now().format("%Y%m%d")).parse().unwrap());
    
    Ok((StatusCode::OK, headers, csv_bytes))
}
```
