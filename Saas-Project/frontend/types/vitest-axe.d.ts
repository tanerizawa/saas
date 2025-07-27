// Add to your types.d.ts file
import { axe, toHaveNoViolations } from 'jest-axe';

declare module 'vitest' {
  interface Assertion<T = any> {
    /**
     * Check if the HTML element has no accessibility violations
     */
    toHaveNoViolations(): T;
  }
}
