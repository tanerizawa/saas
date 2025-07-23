# Backend - Panduan Perbaikan Kode

## 1. Masalah Utama yang Sudah Diperbaiki

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

## 2. Langkah-langkah Selanjutnya

1. **Selesaikan implementasi Repository**

   - Perbarui semua metode di LicenseRepository untuk menggunakan LicenseDto
   - Pastikan konversi antara domain model dan DTO berjalan dengan benar

2. **Testing**

   - Jalankan `cargo check` untuk memverifikasi perbaikan kompilasi
   - Jalankan migrasi database untuk memastikan skema sesuai
   - Jalankan server untuk menguji API endpoint

3. **Implementasi Caching**

   - Buat implementasi LicenseRepository dengan dukungan cache
   - Gunakan Redis untuk menyimpan data yang sering diakses

4. **Optimasi**
   - Perbaiki performa query
   - Tambahkan index yang dibutuhkan
   - Implementasi pagination untuk hasil query besar

## 3. Panduan Implementasi Repository dengan DTO

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
