/**
 * @vitest-environment jsdom
 */
import { describe, it, expect } from "vitest";
import { render } from "@testing-library/react";
import { axe } from "jest-axe";
import { Button } from "../../src/components/ui/button";

describe("Button Component Accessibility", () => {
  it("should not have accessibility violations - default button", async () => {
    const { container } = render(
      <Button>Default Button</Button>
    );
    
    const results = await axe(container);
    expect(results).toHaveNoViolations();
  });

  it("should not have accessibility violations - disabled button", async () => {
    const { container } = render(
      <Button disabled>Disabled Button</Button>
    );
    
    const results = await axe(container);
    expect(results).toHaveNoViolations();
  });

  it("should not have accessibility violations - all variants", async () => {
    const variants = ['default', 'destructive', 'outline', 'secondary', 'ghost', 'link'] as const;
    
    for (const variant of variants) {
      const { container } = render(
        <Button variant={variant}>{variant} Button</Button>
      );
      
      const results = await axe(container);
      expect(results).toHaveNoViolations();
    }
  });

  it("should not have accessibility violations - all sizes", async () => {
    const sizes = ['default', 'sm', 'lg', 'icon'] as const;
    
    for (const size of sizes) {
      const { container } = render(
        <Button size={size}>
          {size === 'icon' ? '🔍' : `${size} Button`}
        </Button>
      );
      
      const results = await axe(container);
      expect(results).toHaveNoViolations();
    }
  });
});
