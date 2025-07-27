/**
 * @vitest-environment jsdom
 */

import { describe, it, expect } from "vitest";
import { render } from "@testing-library/react";

// Create a simple component for testing accessibility
const SimpleComponent = () => {
  return (
    <div>
      <h1>Hello World</h1>
      <button>Click me</button>
    </div>
  );
};

describe("Basic Tests", () => {
  it("should render a component", () => {
    const { getByText } = render(<SimpleComponent />);
    expect(getByText("Hello World")).toBeDefined();
    expect(getByText("Click me")).toBeDefined();
  });
});
