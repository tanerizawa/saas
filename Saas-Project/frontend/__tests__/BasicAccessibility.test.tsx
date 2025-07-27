/**
 * @vitest-environment jsdom
 */
import { describe, test, expect } from "vitest";
import { render, screen } from "@testing-library/react";

// Simple component with good accessibility
const AccessibleButton = ({ onClick, label, disabled = false }) => (
  <button 
    onClick={onClick} 
    disabled={disabled}
    aria-label={label}
  >
    {label}
  </button>
);

// Test for basic accessibility attributes
describe("Basic Accessibility Tests", () => {
  test("button should have proper accessibility attributes", () => {
    render(<AccessibleButton label="Submit" onClick={() => {}} />);
    
    const button = screen.getByRole("button", { name: "Submit" });
    
    // Check if button exists with correct role
    expect(button).toBeInTheDocument();
    
    // Check for aria attributes
    expect(button).toHaveAttribute("aria-label", "Submit");
    
    // Check that button is not disabled by default
    expect(button).not.toBeDisabled();
  });

  test("disabled button should have proper accessibility state", () => {
    render(<AccessibleButton label="Submit" onClick={() => {}} disabled={true} />);
    
    const button = screen.getByRole("button", { name: "Submit" });
    
    // Check if button is properly disabled
    expect(button).toBeDisabled();
  });
});
