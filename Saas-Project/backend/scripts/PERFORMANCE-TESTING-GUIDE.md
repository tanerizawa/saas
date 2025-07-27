# Performance Testing Guide

This document outlines how to perform performance testing on the backend API using the k6 load testing tool.

## Prerequisites

1. Install k6: https://k6.io/docs/getting-started/installation/

   ```bash
   # macOS
   brew install k6

   # Linux
   sudo apt-key adv --keyserver hkp://keyserver.ubuntu.com:80 --recv-keys C5AD17C747E3415A3642D57D77C6C491D6AC1D69
   echo "deb https://dl.k6.io/deb stable main" | sudo tee /etc/apt/sources.list.d/k6.list
   sudo apt-get update
   sudo apt-get install k6

   # Docker alternative
   docker pull grafana/k6
   ```

2. Make sure your backend service is running (locally or remotely).

## Test Scripts

The performance tests are located in the `scripts` directory:

- `performance-test.js` - Main k6 test script
- `run-performance-tests.sh` - Shell script to execute the tests with different configurations

## Available Test Scenarios

1. **Smoke Test**: Basic functionality check with minimal load.
2. **Load Test**: Simulates normal expected user load.
3. **Stress Test**: Simulates higher than normal load to find breaking points.
4. **Spike Test**: Simulates sudden extreme load spikes.

## Running Tests

Use the provided shell script to run the tests:

```bash
cd scripts
./run-performance-tests.sh [options]
```

### Options

- `-h, --help`: Show help message
- `-u, --url URL`: API URL to test (default: http://localhost:8080/api)
- `-t, --test TYPE`: Test type: smoke, load, stress, spike, or all (default: smoke)
- `-o, --output DIR`: Output directory for results (default: ./performance-results)

### Examples

```bash
# Run a quick smoke test against local API
./run-performance-tests.sh -t smoke

# Run a load test against staging environment
./run-performance-tests.sh -t load -u https://staging-api.example.com/api

# Run all test scenarios against production (be careful!)
./run-performance-tests.sh -t all -u https://api.example.com/api -o ./prod-perf-results
```

## Interpreting Results

After running the tests, results are saved in the specified output directory as JSON files. They contain detailed metrics about:

- Request rates
- Response times (min, max, average, percentiles)
- Error rates
- Custom metrics like user creation rates

### Key Performance Indicators (KPIs)

Pay special attention to these metrics:

1. **Response Time**: p95 should be < 500ms, p99 < 1500ms
2. **Error Rate**: Should be < 1%
3. **User Creation Rate**: Should be > 90%

## Continuous Integration

Add these tests to your CI/CD pipeline to catch performance regressions early:

```yaml
# Example GitHub Actions workflow step
- name: Run Performance Tests
  run: |
    cd backend/scripts
    ./run-performance-tests.sh -t smoke -u ${{ env.API_URL }}
```

## Extending the Tests

To add new test scenarios:

1. Update the `performance-test.js` file with your new scenario function
2. Add the scenario to the options configuration
3. Update the shell script to include the new test type
