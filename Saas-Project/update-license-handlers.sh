#!/bin/bash
# Script to update all license handler functions to use AppState instead of generic R

echo "🔧 Updating license handlers to use AppState..."

# Update handler function signatures
sed -i '' 's/async fn get_user_licenses<R>(/async fn get_user_licenses(/g' backend/src/infrastructure/web/handlers/licenses.rs
sed -i '' 's/State(repo): State<Arc<R>>,/State(app_state): State<AppState>,/g' backend/src/infrastructure/web/handlers/licenses.rs
sed -i '' 's/where\n    R: LicenseRepository,\n{//g' backend/src/infrastructure/web/handlers/licenses.rs

# Replace repo calls with app_state.license_repository calls
sed -i '' 's/repo\./app_state.license_repository./g' backend/src/infrastructure/web/handlers/licenses.rs

# Remove generic type parameters from function definitions
sed -i '' 's/<R>//g' backend/src/infrastructure/web/handlers/licenses.rs
sed -i '' 's/where\n    R: LicenseRepository,//g' backend/src/infrastructure/web/handlers/licenses.rs

echo "✅ License handler functions updated to use AppState"
