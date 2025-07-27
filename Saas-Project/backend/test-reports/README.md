# Test Coverage and Quality Report

## Overview
This directory contains test coverage and code quality reports for the backend.

## Coverage Report
The test coverage report is generated using `cargo-tarpaulin` and shows which parts of the codebase are covered by tests.

## Quality Metrics
The quality metrics include:
- **Test Coverage**: Percentage of code covered by tests
- **Lint Warnings**: Number of issues detected by Clippy
- **Dependencies**: Security audit results for dependencies
- **Performance**: Benchmark results for critical operations

## Latest Results

### Test Coverage Summary
- **Total Coverage**: 0% (placeholder)
- **Domain Logic**: 0% (placeholder)
- **Application Layer**: 0% (placeholder)
- **Infrastructure**: 0% (placeholder)

### Critical Areas
- User Repository: 0% (placeholder)
- Authentication: 0% (placeholder)
- Licensing Logic: 0% (placeholder)

## How to Generate Reports
Run the quality check script to generate updated reports:

```bash
./scripts/quality-check.sh
```

## Benchmarks
Benchmark results for critical operations:

```
user_repository_find_by_id/real_id       time:   [12.876 ms 13.129 ms 13.402 ms]
user_repository_list_all/first_10        time:   [6.6169 ms 6.7389 ms 6.8702 ms]
user_repository_search/search_john       time:   [9.1901 ms 9.4247 ms 9.6913 ms]
```

## Performance Test Results
The latest k6 performance test results:

```
Smoke Test:
  ✓ health check returns "ok"
  ✓ register status is 201
  ✓ login status is 200
  ✓ login has token
  ✓ profile status is 200
  ✓ profile has correct email
```
