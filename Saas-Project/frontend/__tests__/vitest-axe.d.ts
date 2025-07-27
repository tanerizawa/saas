// Import the necessary types from axe-core
import type { AxeResults } from 'axe-core';

// Extend Vitest's Assertion interface
declare module 'vitest' {
  interface Assertion<T = any> {
    toHaveNoViolations(): void;
  }
  
  interface AsymmetricMatchersContaining {
    toHaveNoViolations(): void;
  }
}

// Extend the global namespace for test environment
declare global {
  namespace Vi {
    interface Assertion {
      toHaveNoViolations(): void;
    }
    
    interface AsymmetricMatchersContaining {
      toHaveNoViolations(): void;
    }
  }
}
