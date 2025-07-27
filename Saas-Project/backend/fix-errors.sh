#!/bin/bash
# Fix file repository errors related to AppError::DatabaseError

echo "Fixing AppError::DatabaseError usage in file_repository.rs..."

# Replace DatabaseError with InternalError
find ./src -type f -name "*.rs" -exec sed -i '' 's/AppError::DatabaseError(/AppError::InternalError(/g' {} \;

echo "Fixed AppError::DatabaseError usage."
