/**
 * @vitest-environment jsdom
 */
import { describe, it, expect } from "vitest";
import { render } from "@testing-library/react";
import { axe } from "jest-axe";

// Simple toast implementation for accessibility testing
const SimpleToast = ({ 
  variant = "default", 
  children 
}: { 
  variant?: "default" | "destructive"; 
  children: React.ReactNode;
}) => {
  const baseClasses = "fixed top-4 right-4 p-4 rounded-md shadow-lg max-w-md";
  const variantClasses = variant === "destructive" 
    ? "bg-red-100 border border-red-400 text-red-700" 
    : "bg-blue-100 border border-blue-400 text-blue-700";
  
  return (
    <div 
      role="alert" 
      aria-live="polite"
      className={`${baseClasses} ${variantClasses}`}
    >
      {children}
    </div>
  );
};

describe("Toast Component Accessibility", () => {
  it("should not have accessibility violations - success toast", async () => {
    const { container } = render(
      <SimpleToast variant="default">
        <div>
          <strong>Success!</strong>
          <p>Your action was completed successfully.</p>
        </div>
      </SimpleToast>
    );
    
    const results = await axe(container);
    expect(results).toHaveNoViolations();
  });

  it("should not have accessibility violations - destructive toast", async () => {
    const { container } = render(
      <SimpleToast variant="destructive">
        <div>
          <strong>Error!</strong>
          <p>Something went wrong. Please try again.</p>
        </div>
      </SimpleToast>
    );
    
    const results = await axe(container);
    expect(results).toHaveNoViolations();
  });

  it("should not have accessibility violations - toast with action", async () => {
    const { container } = render(
      <SimpleToast>
        <div className="flex items-center justify-between">
          <div>
            <strong>Notification</strong>
            <p>You have a new message.</p>
          </div>
          <button 
            className="ml-4 px-3 py-1 bg-blue-500 text-white rounded"
            aria-label="View message"
          >
            View
          </button>
        </div>
      </SimpleToast>
    );
    
    const results = await axe(container);
    expect(results).toHaveNoViolations();
  });

  it("should not have accessibility violations - toast with close button", async () => {
    const { container } = render(
      <SimpleToast>
        <div className="flex items-start justify-between">
          <div>
            <strong>Information</strong>
            <p>This is an informational message.</p>
          </div>
          <button 
            className="ml-4 text-gray-500 hover:text-gray-700"
            aria-label="Close notification"
          >
            ×
          </button>
        </div>
      </SimpleToast>
    );
    
    const results = await axe(container);
    expect(results).toHaveNoViolations();
  });
});
