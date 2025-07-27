import "@testing-library/jest-dom";
import { vi, expect } from "vitest";
import { configureAxe, toHaveNoViolations } from "jest-axe";

// Extend matchers with Jest-axe - ensure proper typing
expect.extend(toHaveNoViolations);

// Configure axe for testing - conditionally check for global
if (typeof globalThis !== 'undefined') {
  // Browser environment might not have global directly
  // @ts-ignore - We're adding our own property
  globalThis.configureAxe = configureAxe({
    rules: {
      // Add any specific rules configuration here
    },
  });
} else if (typeof global !== 'undefined') {
  // Node.js environment
  // @ts-ignore - We're adding our own property
  global.configureAxe = configureAxe({
    rules: {
      // Add any specific rules configuration here
    },
  });
}

// Mock Next.js router
vi.mock("next/navigation", () => ({
  useRouter: () => ({
    push: vi.fn(),
    back: vi.fn(),
    forward: vi.fn(),
  }),
  useSearchParams: () => ({
    get: vi.fn(),
  }),
  usePathname: () => "/",
}));

// Add any other global setup code here
