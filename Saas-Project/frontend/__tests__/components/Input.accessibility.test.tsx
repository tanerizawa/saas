/**
 * @vitest-environment jsdom
 */
import { describe, it, expect } from "vitest";
import { render } from "@testing-library/react";
import { axe } from "jest-axe";
import { Input } from "../../src/components/ui/input";
import { Label } from "../../src/components/ui/label";

describe("Input Component Accessibility", () => {
  it("should not have accessibility violations - basic input", async () => {
    const { container } = render(
      <div>
        <Label htmlFor="test-input">Test Label</Label>
        <Input id="test-input" placeholder="Enter text" />
      </div>
    );
    
    const results = await axe(container);
    expect(results).toHaveNoViolations();
  });

  it("should not have accessibility violations - different input types", async () => {
    const inputTypes = ['text', 'email', 'password', 'number', 'tel', 'url'] as const;
    
    for (const type of inputTypes) {
      const { container } = render(
        <div>
          <Label htmlFor={`${type}-input`}>{type} Input</Label>
          <Input id={`${type}-input`} type={type} placeholder={`Enter ${type}`} />
        </div>
      );
      
      const results = await axe(container);
      expect(results).toHaveNoViolations();
    }
  });

  it("should not have accessibility violations - disabled input", async () => {
    const { container } = render(
      <div>
        <Label htmlFor="disabled-input">Disabled Input</Label>
        <Input id="disabled-input" disabled placeholder="This is disabled" />
      </div>
    );
    
    const results = await axe(container);
    expect(results).toHaveNoViolations();
  });

  it("should not have accessibility violations - required input", async () => {
    const { container } = render(
      <div>
        <Label htmlFor="required-input">Required Input *</Label>
        <Input 
          id="required-input" 
          required 
          aria-required="true"
          placeholder="This field is required" 
        />
      </div>
    );
    
    const results = await axe(container);
    expect(results).toHaveNoViolations();
  });
});
