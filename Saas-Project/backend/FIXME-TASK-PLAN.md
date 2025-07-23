# Backend Fix Task Plan

This document outlines the critical backend fixes needed to resolve the compilation errors and improve the functionality of the SaaS UMKM backend.

## Critical Fixes

1. **Fix LicenseRepository Implementation**

   - Complete implementation of all required methods in `PostgresLicenseRepository`
   - Fix mapping of database fields to struct fields in `sqlx::query_as!` macros
   - Resolve the issue with the `business_id` field not found in License struct

2. **Update Database Connection Logic**

   - Replace outdated PgConnectOptions methods:
     - `tcp_keepalives_idle`
     - `tcp_keepalives_interval`
     - `tcp_keepalives_retries`
     - `connect_timeout`
   - Use current sqlx API methods for connection configuration

3. **Fix Middleware Issues**

   - Either implement the missing rate limiter module or remove references to it
   - Update middleware imports and usages in `main.rs`

4. **Service Layer Configuration**

   - Fix the `cache_service` not found in scope in `main.rs`
   - Update service builder code to properly handle different middleware layers

5. **Implement From/ToString for Custom Types**
   - Fix conversion between database string types and enum types

## Implementation Notes

### LicenseRepository Implementation Example

```rust
// In cached_license_repository.rs
impl LicenseRepository for PostgresLicenseRepository {
    async fn get_licenses_by_company(&self, company_id: Uuid) -> Result<Vec<License>, sqlx::Error> {
        // Cache key format
        let cache_key = format!("licenses:company:{}", company_id);

        // Try to get from cache first
        if let Some(cached) = self.cache.get::<Vec<License>>(&cache_key).await.ok().flatten() {
            return Ok(cached);
        }

        // If not in cache, query database
        let licenses = sqlx::query_as!(
            LicenseDto, // Use a DTO to map database fields
            r#"
            SELECT * FROM licenses WHERE company_id = $1
            "#,
            company_id
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|dto| dto.into()) // Convert DTO to domain entity
        .collect();

        // Store in cache for future requests
        if !licenses.is_empty() {
            let _ = self.cache.set(&cache_key, &licenses, Some(300)).await; // 5 minute cache
        }

        Ok(licenses)
    }

    // Similar implementation for other required methods...
}
```

### Database Connection Example

```rust
// In database.rs
impl Database {
    pub fn parse_pool_options(database_url: &str) -> Result<PgConnectOptions, anyhow::Error> {
        let options = PgConnectOptions::from_str(database_url)?;

        // Modern sqlx options for PostgreSQL
        let options = options
            .log_statements(LevelFilter::Debug)
            .log_slow_statements(LevelFilter::Warn, Duration::from_secs(1))
            .application_name("saas-umkm-backend");

        // Add any additional options that are available in current sqlx version

        Ok(options)
    }
}
```

## Testing Approach

After implementing these fixes:

1. Run `cargo check` to verify compilation succeeds
2. Run `cargo test` to verify tests pass
3. Run `cargo run --bin migrate` to verify database migrations work
4. Run `cargo run --bin server` to start the server and test API endpoints

## Future Enhancements

Once the critical fixes are in place:

1. Refactor the repository implementations for better maintainability
2. Add proper error handling and logging
3. Implement comprehensive tests for the repositories
4. Update documentation to reflect current implementation
