/**
 * @vitest-environment jsdom
 */
import { describe, it, expect } from "vitest";
import { render } from "@testing-library/react";
import { axe } from "jest-axe";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "../../src/components/ui/select";
import { Label } from "../../src/components/ui/label";

describe("Select Component Accessibility", () => {
  it("should not have accessibility violations - basic select", async () => {
    const { container } = render(
      <div>
        <Label htmlFor="test-select">Choose an option</Label>
        <Select>
          <SelectTrigger id="test-select">
            <SelectValue placeholder="Select an option" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="option1">Option 1</SelectItem>
            <SelectItem value="option2">Option 2</SelectItem>
            <SelectItem value="option3">Option 3</SelectItem>
          </SelectContent>
        </Select>
      </div>
    );
    
    const results = await axe(container);
    expect(results).toHaveNoViolations();
  });

  it("should not have accessibility violations - disabled select", async () => {
    const { container } = render(
      <div>
        <Label htmlFor="disabled-select">Disabled Select</Label>
        <Select disabled>
          <SelectTrigger id="disabled-select">
            <SelectValue placeholder="This is disabled" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="option1">Option 1</SelectItem>
            <SelectItem value="option2">Option 2</SelectItem>
          </SelectContent>
        </Select>
      </div>
    );
    
    const results = await axe(container);
    expect(results).toHaveNoViolations();
  });

  it("should not have accessibility violations - required select", async () => {
    const { container } = render(
      <div>
        <Label htmlFor="required-select">Required Select *</Label>
        <Select required>
          <SelectTrigger id="required-select" aria-required="true">
            <SelectValue placeholder="This field is required" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="option1">Option 1</SelectItem>
            <SelectItem value="option2">Option 2</SelectItem>
            <SelectItem value="option3">Option 3</SelectItem>
          </SelectContent>
        </Select>
      </div>
    );
    
    const results = await axe(container);
    expect(results).toHaveNoViolations();
  });

  it("should not have accessibility violations - select with groups", async () => {
    const { container } = render(
      <div>
        <Label htmlFor="grouped-select">Grouped Select</Label>
        <Select>
          <SelectTrigger id="grouped-select">
            <SelectValue placeholder="Select from groups" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="fruit-apple">Apple</SelectItem>
            <SelectItem value="fruit-banana">Banana</SelectItem>
            <SelectItem value="vegetable-carrot">Carrot</SelectItem>
            <SelectItem value="vegetable-broccoli">Broccoli</SelectItem>
          </SelectContent>
        </Select>
      </div>
    );
    
    const results = await axe(container);
    expect(results).toHaveNoViolations();
  });
});
