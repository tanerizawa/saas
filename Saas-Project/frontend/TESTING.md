# Testing Implementation for Frontend

This document describes the implementation of testing for the frontend components using Vitest and React Testing Library.

## Overview

We've added proper testing infrastructure for the frontend components using:

- Vitest: For test running and assertions
- React Testing Library: For rendering and interacting with React components
- User Event: For simulating user interactions

## Components With Tests

### LicenseApplicationForm

We've enhanced the `LicenseApplicationForm` component with:

1. Proper accessibility attributes (labels with htmlFor, inputs with IDs)
2. Data-testid attributes for easier testing
3. Complete form fields for better user experience
4. Type safety fixes for form handlers

Tests cover:

- Rendering with initial values
- Updating form values
- Form submission
- Reset functionality

### LicenseList

We've enhanced the `LicenseList` component with:

1. Data-testid attributes for better testing
2. Proper status filtering
3. Improved user interaction for row and button clicks

Tests cover:

- Rendering empty state
- Rendering license list
- Filtering licenses by status
- Handling row clicks
- Handling detail button clicks

### DocumentUpload

We've enhanced the `DocumentUpload` component with:

1. Data-testid attributes for document elements
2. File upload indicators
3. Document status labels (required/optional)
4. Progress indicators for uploads

Tests cover:

- Rendering required and optional document types
- Handling file uploads
- Removing uploaded documents
- Upload button state management
- Upload completion callback functionality

1. Data-testid attributes for file inputs and buttons
2. Proper file upload handling
3. Document status tracking

Tests cover:

- Rendering document types
- Required vs optional document handling
- File upload functionality
- Document removal
- Upload button states
- Upload completion callback

## Running Tests

To run the tests, use one of the following commands:

```bash
# Run all tests once
npm run test

# Run tests in watch mode (for development)
npm run test:watch

# Run tests with coverage report
npm run test:coverage

# Run tests with UI
npm run test:ui

# Run tests and update snapshots
npm run test:update

# Run tests for specific components
npm run test:component

# Run tests in CI environment (outputs JUnit format)
npm run test:ci
```

## Troubleshooting Test Issues

If you encounter issues running tests:

1. Check that all dependencies are installed correctly:

   ```
   npm install
   ```

2. Check that Jest DOM matchers are properly set up in `vitest.setup.ts`

3. If you're getting specific test failures:
   - Check that the component's JSX structure matches what the tests expect
   - Make sure data-testid attributes are correctly set
   - Verify that the form state is being updated correctly

## Testing Best Practices

### Component Testing

1. **Use data-testid attributes**: Add data-testid attributes to elements you want to test to make your tests more resilient to UI changes

   ```tsx
   <button data-testid="submit-button">Submit</button>
   ```

2. **Test user behavior, not implementation details**:

   - Focus on testing what users do (clicking buttons, filling forms) rather than testing implementation details
   - Use `userEvent` instead of `fireEvent` when possible to more closely simulate real user interactions

3. **Test accessibility**: Make sure your components are accessible by testing them with proper accessibility attributes

   - Labels should be properly associated with inputs using htmlFor/id
   - ARIA attributes should be correctly used

4. **Group tests logically**: Use describe blocks to group related tests, making them easier to read and maintain

5. **Mock dependencies**: Use mocks for external dependencies like API calls, but be careful not to over-mock

### Test Structure

Follow this structure for consistent test organization:

```tsx
// Import dependencies
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import ComponentToTest from '../path/to/Component';

// Mock data (outside describe block)
const mockData = [...];

// Main test suite
describe('ComponentName Component', () => {
  // Mock functions
  const mockFunction = vi.fn();

  // Setup before each test
  beforeEach(() => {
    mockFunction.mockClear();
  });

  // Individual tests
  it('should render correctly', () => {
    render(<ComponentToTest />);
    expect(screen.getByTestId('element-id')).toBeInTheDocument();
  });

  // Group related tests
  describe('when user interacts with component', () => {
    it('should update when clicked', async () => {
      // Test implementation
    });
  });
});
```

## Edge Cases to Test

When writing tests, consider these common edge cases:

- **Empty states**: How does the component behave when no data is provided?
- **Loading states**: Does the component handle loading states gracefully?
- **Error states**: How does the component handle errors?
- **Boundary conditions**: Test with minimum/maximum values, empty strings, etc.
- **User interactions**: Test keyboard navigation, mouse clicks, and touch events
- **Form validation**: Test form validation logic with valid and invalid inputs
- **Async operations**: Test components that rely on async operations like API calls
- **Internationalization**: Test components with different language settings if applicable

### Test Examples

#### Testing Form Submissions

```tsx
it("submits form with correct data", async () => {
  const mockSubmit = vi.fn();
  render(<FormComponent onSubmit={mockSubmit} />);

  // Fill out the form
  await userEvent.type(screen.getByTestId("name-input"), "Test User");
  await userEvent.selectOptions(screen.getByTestId("role-select"), "admin");

  // Submit the form
  await userEvent.click(screen.getByTestId("submit-button"));

  // Verify the submit handler was called with correct data
  expect(mockSubmit).toHaveBeenCalledTimes(1);
  expect(mockSubmit).toHaveBeenCalledWith({
    name: "Test User",
    role: "admin",
  });
});
```

#### Testing Filtering/Sorting

````tsx
it('filters items correctly', async () => {
  const items = [
    { id: 1, category: 'fruits', name: 'Apple' },
    { id: 2, category: 'vegetables', name: 'Carrot' }
  ];

  render(<FilterableList items={items} />);

  // Select filter
  await userEvent.click(screen.getByTestId('filter-fruits'));

  // Check that only fruits are shown
  expect(screen.getByText('Apple')).toBeInTheDocument();
  expect(screen.queryByText('Carrot')).not.toBeInTheDocument();
});

## Advanced Testing Strategies

### Integration Testing

Integration tests verify that multiple components work together correctly:

```tsx
it('completes the license application flow', async () => {
  render(<LicenseApplicationFlow />);

  // Fill out the form
  await userEvent.type(screen.getByTestId('business-name-input'), 'Test Company');

  // Navigate to next step
  await userEvent.click(screen.getByTestId('next-button'));

  // Verify we're on the document upload screen
  expect(screen.getByText('Upload Documents')).toBeInTheDocument();

  // Continue with the flow...
});
````

### Snapshot Testing

Use snapshot testing sparingly for UI components that don't change often:

```tsx
it("matches snapshot", () => {
  const { container } = render(<StaticComponent />);
  expect(container).toMatchSnapshot();
});
```

### Performance Testing

For performance-sensitive components:

```tsx
it("renders large lists efficiently", async () => {
  const start = performance.now();

  render(<LargeList items={generateManyItems(1000)} />);

  const duration = performance.now() - start;
  expect(duration).toBeLessThan(200); // Should render in under 200ms
});
```

## Test Coverage Goals

Our testing goals for this project:

1. Unit test coverage: Aim for >80% code coverage
2. All critical user flows covered by integration tests

## Conclusion

We have successfully implemented comprehensive testing for all key components in the frontend:

- ✅ LicenseApplicationForm: Complete tests for all functionality
- ✅ LicenseList: Complete tests for rendering, filtering, and interactions
- ✅ DocumentUpload: Complete tests for file handling, document requirements, and state management

All tests are now passing, providing confidence in the functionality of these components. The testing infrastructure is set up correctly with Vitest and React Testing Library, making it easy to add more tests as needed. 3. Key UI components covered by snapshot tests 4. Accessibility compliance in all form components

## Next Steps

To expand test coverage:

1. Add tests for other components (DocumentUpload, TwoFactorSetup, etc.)
2. Add more edge cases to existing tests
3. Implement integration tests between components
4. Set up E2E tests with Playwright or Cypress
5. Add visual regression testing for UI components
6. Implement a11y testing with axe-core
