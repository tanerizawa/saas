#!/bin/bash
# This script modifies main.rs to disable authentication middleware before running the server

echo "==============================================="
echo "🚀 Menjalankan Backend SaaS UMKM Tanpa Autentikasi"
echo "📊 DATABASE_URL=postgresql://saas_user:saas_password@$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' saas-postgres):5432/saas_umkm_db"
echo "📊 REDIS_URL=redis://$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' saas-redis):6379"
echo "==============================================="

# Backup original file
cp src/main.rs src/main.rs.bak
echo "✅ Backup dibuat: src/main.rs.bak"

# Modify main.rs to disable migrations and authentication middleware
sed -i '' 's/if state\.config\.rate_limiter\.is_some() {/if false {/' src/main.rs
echo "✅ Auth middleware dinonaktifkan"

# Skip migrations
sed -i '' 's/match sqlx::migrate!().run(&db.pool()).await {/\/\/ Migrations disabled\n    \/\* match sqlx::migrate!().run(&db.pool()).await {/' src/main.rs
sed -i '' 's/}    \/\/ End of migrations/} \*\/    \/\/ End of migrations/' src/main.rs
echo "✅ Migrations dinonaktifkan"

# Run server
echo "🚀 Menjalankan server..."
cargo run --bin server

# Restore original file
mv src/main.rs.bak src/main.rs
echo "✅ File asli dikembalikan"
