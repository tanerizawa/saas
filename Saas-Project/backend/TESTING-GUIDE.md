# Backend Testing and Performance Guide

This document provides a comprehensive overview of the testing and performance strategies implemented in the backend service.

## Testing Approach

The project follows a layered testing approach:

1. **Unit Tests**: Testing individual components in isolation
2. **Integration Tests**: Testing component interactions with external systems like the database
3. **Benchmarks**: Measuring performance of critical operations
4. **Performance Tests**: Simulating real-world load scenarios

### Running Tests

Use the provided test runner script:

```bash
./scripts/run-tests.sh [options]
```

#### Test Script Options

- `--db-host HOST`: Database host (default: localhost)
- `--db-port PORT`: Database port (default: 5432)
- `--db-name NAME`: Database name (default: saas_test_db)
- `--db-user USER`: Database user (default: saas_user)
- `--db-password PASSWORD`: Database password
- `--report-dir DIR`: Directory for test reports (default: ./test-reports)
- `--no-parallel`: Disable parallel test execution
- `--show-output`: Show test output
- `--coverage`: Run tests with coverage
- `--integration-only`: Run only integration tests
- `--unit-only`: Run only unit tests
- `--benchmarks`: Run benchmarks
- `--specific TEST`: Run a specific test or pattern

### Test Isolation

Our integration tests use dynamic schema creation for each test to ensure proper isolation:

```rust
// Each test gets its own schema
let schema_name = format!("test_{}", Uuid::new_v4().as_simple());

// Create schema
sqlx::query(&format!("CREATE SCHEMA IF NOT EXISTS {}", schema_name))
    .execute(&pool)
    .await
    .unwrap();

// Run test in isolated schema
// ...

// Clean up
sqlx::query(&format!("DROP SCHEMA IF EXISTS {} CASCADE", schema_name))
    .execute(pool)
    .await
    .unwrap();
```

## Performance Testing

### Benchmarks

We use Criterion for benchmarking critical repository operations:
- User lookups by ID
- Pagination performance
- Search performance

Run benchmarks with:

```bash
./scripts/run-tests.sh --benchmarks
```

### Load Testing

For realistic load testing, we use k6 to simulate various scenarios:

1. **Smoke Test**: Basic functionality check
2. **Load Test**: Normal expected user load
3. **Stress Test**: Higher than normal load to find breaking points
4. **Spike Test**: Sudden extreme load spikes

Run performance tests with:

```bash
cd scripts
./run-performance-tests.sh [options]
```

#### Performance Test Options

- `-u, --url URL`: API URL to test
- `-t, --test TYPE`: Test type: smoke, load, stress, spike, or all
- `-o, --output DIR`: Output directory for results

## Best Practices

1. **Isolation**: Every test must run in isolation and clean up after itself
2. **Database Testing**: Use dynamic schemas to prevent test interference
3. **Coverage**: Aim for high test coverage, especially in domain logic
4. **Performance Baselines**: Establish performance baselines through benchmarks
5. **Error Handling**: Test error conditions and edge cases thoroughly

## Continuous Integration

Tests are integrated into the CI/CD pipeline:

1. **Pull Request Validation**: Unit and integration tests
2. **Pre-Deployment**: Basic performance tests (smoke tests)
3. **Post-Deployment**: Full performance test suite against staging

## Extending Tests

### Adding New Unit/Integration Tests

1. Create test module with appropriate test helper functions
2. Follow the setup/teardown pattern for database tests
3. Use descriptive test names following the `test_<what_is_being_tested>` pattern

### Adding New Benchmarks

1. Add benchmark functions to the appropriate benchmark file
2. Include the new function in the criterion_group! macro

### Adding New Performance Tests

1. Add new scenarios to the performance test script
2. Define appropriate thresholds for the new scenarios
