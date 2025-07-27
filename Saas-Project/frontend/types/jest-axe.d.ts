// Type definitions for jest-axe with Vitest

import "jest-axe";
import { AxeResults } from "axe-core";

// Extend Vitest's expect matchers
declare module "vitest" {
  interface Assertion<T = any> {
    toHaveNoViolations(): T;
  }
  
  interface AsymmetricMatchersContaining {
    toHaveNoViolations(): any;
  }
}

// Global namespace for Vitest
declare global {
  namespace Vi {
    interface Assertion {
      toHaveNoViolations(): void;
    }
    interface AsymmetricMatchersContaining {
      toHaveNoViolations(): void;
    }
  }

  function configureAxe(options: any): void;
}

// Additional declaration for jest-axe if needed
declare module "jest-axe" {
  export interface AxeResults {
    violations: import("axe-core").Result[];
    passes: import("axe-core").Result[];
    incomplete: import("axe-core").Result[];
    inapplicable: import("axe-core").Result[];
  }
  
  export function axe(
    html: Element | string,
    options?: any
  ): Promise<AxeResults>;
  
  export const toHaveNoViolations: {
    toHaveNoViolations(results: AxeResults): {
      pass: boolean;
      message(): string;
    };
  };
  
  export function configureAxe(options?: any): any;
}
