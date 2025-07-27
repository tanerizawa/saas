#!/bin/bash
set -e

# Default variables
API_URL=${API_URL:-"http://localhost:8080/api"}
TEST_TYPE=${TEST_TYPE:-"smoke"}
OUTPUT_DIR=${OUTPUT_DIR:-"./performance-results"}

# Create output directory if it doesn't exist
mkdir -p "$OUTPUT_DIR"

# Print help
function show_help {
  echo "Usage: $0 [options]"
  echo ""
  echo "Options:"
  echo "  -h, --help          Show this help message"
  echo "  -u, --url URL       API URL to test (default: http://localhost:8080/api)"
  echo "  -t, --test TYPE     Test type: smoke, load, stress, spike, or all (default: smoke)"
  echo "  -o, --output DIR    Output directory for results (default: ./performance-results)"
  echo ""
}

# Parse command line options
while [[ $# -gt 0 ]]; do
  case "$1" in
    -h|--help)
      show_help
      exit 0
      ;;
    -u|--url)
      API_URL="$2"
      shift 2
      ;;
    -t|--test)
      TEST_TYPE="$2"
      shift 2
      ;;
    -o|--output)
      OUTPUT_DIR="$2"
      shift 2
      ;;
    *)
      echo "Unknown option: $1"
      show_help
      exit 1
      ;;
  esac
done

# Check if k6 is installed
if ! command -v k6 &> /dev/null; then
  echo "k6 is not installed. Please install it from https://k6.io/docs/getting-started/installation/"
  exit 1
fi

# Function to run tests
function run_test {
  local test_name=$1
  local timestamp=$(date +%Y%m%d_%H%M%S)
  local output_file="${OUTPUT_DIR}/${test_name}_${timestamp}"
  
  echo "Running $test_name test against $API_URL"
  echo "Results will be saved to ${output_file}.json"
  
  API_URL=$API_URL k6 run \
    --out json="${output_file}.json" \
    --out summary \
    --tag testname=$test_name \
    --include-system-env-vars \
    --include-test-run-id=true \
    $([ "$test_name" != "all" ] && echo "--scenario $test_name") \
    ./performance-test.js
    
  echo "Test completed. Results saved to ${output_file}.json"
}

# Run the specified test(s)
case "$TEST_TYPE" in
  smoke)
    run_test "smoke_test"
    ;;
  load)
    run_test "load_test"
    ;;
  stress)
    run_test "stress_test"
    ;;
  spike)
    run_test "spike_test"
    ;;
  all)
    echo "Running all tests sequentially"
    run_test "all"
    ;;
  *)
    echo "Invalid test type: $TEST_TYPE"
    show_help
    exit 1
    ;;
esac

echo "All tests completed successfully"
