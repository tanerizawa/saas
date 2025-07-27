# Testing Approach

This document outlines our testing approach for the SaaS application.

## Testing Strategies

We employ several testing strategies to ensure the quality and reliability of our codebase:

1. **Unit Tests**: Testing individual components in isolation
2. **Integration Tests**: Testing interactions between components
3. **End-to-End Tests**: Testing the entire application flow
4. **Repository Tests**: Testing database interactions

## Repository Testing

We have two implementations of our repositories:

1. **PostgreSQL Implementation**: For production use, connecting to a real database
2. **In-Memory Implementation**: For testing without database dependencies

### In-Memory Testing

The in-memory implementation allows us to:

- Run tests quickly without database setup
- Test in isolation without network dependencies
- Verify business logic independently of database concerns

To run the in-memory tests:

```bash
cargo run --bin in_memory_user_repo_test
```

### PostgreSQL Testing

For integration tests with the actual database:

1. Set up the test database:
```bash
cargo run --bin db_setup
```

2. Run the tests with the provided environment variables:
```bash
export TEST_DATABASE_URL="postgres://saas_user:saas_password@localhost:5432/saas_test_db"
export TEST_SCHEMA="test_schema_xyz"
cargo test
```

## Test Organization

Our tests are organized as follows:

- Unit tests are located alongside the code they test
- Integration tests are in the `tests/` directory
- Standalone test binaries are in the `src/bin/` directory

## Continuous Testing

We recommend running tests:

1. Before committing changes
2. After major refactoring
3. When adding new features
4. In the CI/CD pipeline

## Test Data

We use test factories to create consistent test data. For example:

```rust
// Creating a test user
let user = User {
    id: UserId::new(),
    email: Email::new("test@example.com").unwrap(),
    password_hash: "hashed_password".to_string(),
    full_name: "Test User".to_string(),
    // ... other fields
};
```

## Best Practices

1. Use descriptive test names
2. Test both happy paths and error conditions
3. Keep tests independent of each other
4. Clean up resources after tests
5. Use assertions to verify expected outcomes
