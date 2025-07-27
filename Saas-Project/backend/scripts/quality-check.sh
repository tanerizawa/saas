#!/bin/bash
set -e

# Directories and files to check
TESTS_DIR="src/tests"
BENCH_DIR="benches"
API_DIR="src/infrastructure/web"
DOMAIN_DIR="src/domain"
APP_DIR="src/application"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Helper function to print section headers
print_header() {
  echo -e "\n${BLUE}====== $1 ======${NC}\n"
}

# Check test coverage statistics
print_header "Checking Test Coverage"
if ! command -v cargo-tarpaulin &> /dev/null; then
  echo -e "${YELLOW}cargo-tarpaulin not found. Installing...${NC}"
  cargo install cargo-tarpaulin
fi

# Run coverage analysis
echo "Running test coverage analysis..."
cargo tarpaulin --out Html --output-dir ./coverage-report

# Check security vulnerabilities
print_header "Checking Security Vulnerabilities"
if ! command -v cargo-audit &> /dev/null; then
  echo -e "${YELLOW}cargo-audit not found. Installing...${NC}"
  cargo install cargo-audit
fi

echo "Running security audit..."
cargo audit

# Check dependency licenses
print_header "Checking Dependency Licenses"
if ! command -v cargo-deny &> /dev/null; then
  echo -e "${YELLOW}cargo-deny not found. Installing...${NC}"
  cargo install cargo-deny
fi

echo "Running license checks..."
cargo deny check licenses

# Run clippy lints
print_header "Running Clippy Lints"
echo "Checking for code quality issues..."
cargo clippy -- -D warnings

# Run rustfmt
print_header "Checking Code Formatting"
echo "Verifying code formatting..."
cargo fmt -- --check

# Analyze test quality
print_header "Analyzing Test Quality"
echo "Checking test coverage in critical areas..."

# Count tests in important modules
domain_tests=$(find "$DOMAIN_DIR" -name "*_test.rs" | xargs grep -l "#\[test\]" | wc -l)
app_tests=$(find "$APP_DIR" -name "*_test.rs" | xargs grep -l "#\[test\]" | wc -l)
api_tests=$(find "$API_DIR" -name "*_test.rs" | xargs grep -l "#\[test\]" | wc -l)
integration_tests=$(find "$TESTS_DIR" -name "*_test.rs" | xargs grep -l "#\[tokio::test\]" | wc -l)
benchmarks=$(find "$BENCH_DIR" -name "*.rs" 2>/dev/null | wc -l || echo 0)

echo -e "Domain tests: ${GREEN}$domain_tests${NC}"
echo -e "Application tests: ${GREEN}$app_tests${NC}"
echo -e "API tests: ${GREEN}$api_tests${NC}"
echo -e "Integration tests: ${GREEN}$integration_tests${NC}"
echo -e "Benchmarks: ${GREEN}$benchmarks${NC}"

# Check missing tests in important modules
print_header "Checking for Missing Tests"
find src -name "*.rs" -not -path "*/target/*" -not -path "*/\.*" | while read -r file; do
  base_name=$(basename "$file" .rs)
  dir_name=$(dirname "$file")
  test_file="$dir_name/${base_name}_test.rs"
  
  # Skip main.rs, lib.rs, mod.rs and files that already have tests
  if [[ "$base_name" == "main" || "$base_name" == "lib" || "$base_name" == "mod" ]]; then
    continue
  fi
  
  # Check if there's a corresponding test file
  if [[ ! -f "$test_file" && ! "$file" =~ "_test.rs" ]]; then
    echo -e "${YELLOW}Warning: No test file found for $file${NC}"
  fi
done

# Generate report summary
print_header "Quality Check Summary"
echo -e "${GREEN}Quality check completed!${NC}"
echo "See coverage-report/tarpaulin-report.html for detailed coverage information."
