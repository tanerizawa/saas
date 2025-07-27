#!/bin/bash
set -e

# Default configuration
DB_HOST=${DB_HOST:-"localhost"}
DB_PORT=${DB_PORT:-5432}
DB_NAME=${DB_NAME:-"saas_test_db"}
DB_USER=${DB_USER:-"saas_user"}
DB_PASSWORD=${DB_PASSWORD:-"saas_password"}
REPORT_DIR=${REPORT_DIR:-"./test-reports"}
PARALLEL_TESTS=${PARALLEL_TESTS:-true}
SHOW_OUTPUT=${SHOW_OUTPUT:-false}
COVERAGE=${COVERAGE:-false}

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to display usage
usage() {
  echo "Usage: $0 [options]"
  echo
  echo "Options:"
  echo "  -h, --help             Show this help message and exit"
  echo "  --db-host HOST         Database host (default: $DB_HOST)"
  echo "  --db-port PORT         Database port (default: $DB_PORT)"
  echo "  --db-name NAME         Database name (default: $DB_NAME)"
  echo "  --db-user USER         Database user (default: $DB_USER)"
  echo "  --db-password PASSWORD Database password (default: $DB_PASSWORD)"
  echo "  --report-dir DIR       Directory for test reports (default: $REPORT_DIR)"
  echo "  --no-parallel          Disable parallel test execution"
  echo "  --show-output          Show test output"
  echo "  --coverage             Run tests with coverage"
  echo "  --integration-only     Run only integration tests"
  echo "  --unit-only            Run only unit tests"
  echo "  --benchmarks           Run benchmarks"
  echo "  --specific TEST        Run a specific test or pattern"
  echo
  exit 1
}

# Parse arguments
SPECIFIC_TEST=""
TEST_TYPE="all"
RUN_BENCHMARKS=false

while [[ $# -gt 0 ]]; do
  case "$1" in
    -h|--help)
      usage
      ;;
    --db-host)
      DB_HOST="$2"
      shift 2
      ;;
    --db-port)
      DB_PORT="$2"
      shift 2
      ;;
    --db-name)
      DB_NAME="$2"
      shift 2
      ;;
    --db-user)
      DB_USER="$2"
      shift 2
      ;;
    --db-password)
      DB_PASSWORD="$2"
      shift 2
      ;;
    --report-dir)
      REPORT_DIR="$2"
      shift 2
      ;;
    --no-parallel)
      PARALLEL_TESTS=false
      shift
      ;;
    --show-output)
      SHOW_OUTPUT=true
      shift
      ;;
    --coverage)
      COVERAGE=true
      shift
      ;;
    --integration-only)
      TEST_TYPE="integration"
      shift
      ;;
    --unit-only)
      TEST_TYPE="unit"
      shift
      ;;
    --benchmarks)
      RUN_BENCHMARKS=true
      shift
      ;;
    --specific)
      SPECIFIC_TEST="$2"
      shift 2
      ;;
    *)
      echo -e "${RED}Unknown option: $1${NC}"
      usage
      ;;
  esac
done

# Create report directory if it doesn't exist
mkdir -p "$REPORT_DIR"

# Export database connection string for tests
export TEST_DATABASE_URL="postgres://$DB_USER:$DB_PASSWORD@$DB_HOST:$DB_PORT/$DB_NAME"

# Helper function to print section headers
print_header() {
  echo -e "\n${BLUE}====== $1 ======${NC}\n"
}

# Helper function to print status
print_status() {
  if [ "$1" -eq 0 ]; then
    echo -e "${GREEN}✓ $2${NC}"
  else
    echo -e "${RED}✗ $2${NC}"
    exit 1
  fi
}

# Check database connection
print_header "Checking database connection"
if PGPASSWORD="$DB_PASSWORD" psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" -c "SELECT 1" > /dev/null 2>&1; then
  echo -e "${GREEN}Database connection successful${NC}"
else
  echo -e "${RED}Failed to connect to database${NC}"
  exit 1
fi

# Build tests first
print_header "Building tests"
cargo build --tests
print_status $? "Build completed"

# Setup test environment
print_header "Setting up test environment"
echo "Using database: $TEST_DATABASE_URL"

# Define test filter based on test type
if [ "$TEST_TYPE" = "integration" ]; then
  TEST_FILTER="--test *_test"
elif [ "$TEST_TYPE" = "unit" ]; then
  TEST_FILTER="--lib"
else
  TEST_FILTER=""
fi

# Add specific test pattern if provided
if [ -n "$SPECIFIC_TEST" ]; then
  SPECIFIC_FILTER="-- $SPECIFIC_TEST"
else
  SPECIFIC_FILTER=""
fi

# Run tests
print_header "Running tests"

# Setup test command
if [ "$COVERAGE" = true ]; then
  if ! command -v cargo-tarpaulin &> /dev/null; then
    echo -e "${YELLOW}cargo-tarpaulin not found, installing...${NC}"
    cargo install cargo-tarpaulin
  fi
  
  echo "Running tests with coverage..."
  TEST_CMD="cargo tarpaulin --out Html --output-dir $REPORT_DIR $TEST_FILTER $SPECIFIC_FILTER"
else
  PARALLEL_FLAG=""
  if [ "$PARALLEL_TESTS" = true ]; then
    PARALLEL_FLAG="--parallel"
  fi
  
  OUTPUT_FLAG=""
  if [ "$SHOW_OUTPUT" = true ]; then
    OUTPUT_FLAG="--show-output"
  fi
  
  TEST_CMD="cargo test $PARALLEL_FLAG $OUTPUT_FLAG $TEST_FILTER $SPECIFIC_FILTER"
fi

# Execute test command
echo "$ $TEST_CMD"
eval "$TEST_CMD"
TEST_EXIT_CODE=$?

print_status $TEST_EXIT_CODE "Tests completed"

# Run benchmarks if requested
if [ "$RUN_BENCHMARKS" = true ]; then
  print_header "Running benchmarks"
  
  if ! command -v cargo-criterion &> /dev/null; then
    echo -e "${YELLOW}cargo-criterion not found, installing...${NC}"
    cargo install cargo-criterion
  fi
  
  echo "Running benchmarks..."
  cargo criterion --output-format verbose
  print_status $? "Benchmarks completed"
fi

# Generate test report summary
print_header "Test Summary"
echo "Test report available at: $REPORT_DIR"
echo -e "${GREEN}All tests completed successfully!${NC}"
