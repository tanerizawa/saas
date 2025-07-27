/**
 * @vitest-environment jsdom
 */
import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { axe } from "jest-axe";
import HomePage from "../src/app/page";

// Mock required modules for HomePage
vi.mock("@/contexts/AuthContext", () => ({
  useAuth: () => ({
    user: null,
    isLoggedIn: false,
  }),
}));

vi.mock("next/navigation", () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

// Enhanced accessibility tests
describe("HomePage Accessibility Tests", () => {
  it("should have proper semantic structure", async () => {
    const { container } = render(<HomePage />);
    
    // Check for semantic HTML elements
    const main = container.querySelector('main');
    const headings = container.querySelectorAll('h1, h2, h3, h4, h5, h6');
    
    expect(main).toBeInTheDocument();
    expect(headings.length).toBeGreaterThan(0);
    
    const results = await axe(container);
    expect(results).toHaveNoViolations();
  });

  it("should have proper navigation landmarks", async () => {
    const { container } = render(<HomePage />);
    
    // Check for navigation elements
    const nav = container.querySelector('nav');
    if (nav) {
      expect(nav).toBeInTheDocument();
    }
    
    const results = await axe(container);
    expect(results).toHaveNoViolations();
  });
});

// Comprehensive accessibility best practices tests
describe("Accessibility Best Practices", () => {
  it("should have proper image alt texts", () => {
    const { container } = render(
      <div>
        <img src="/test.jpg" alt="Test image" />
        <img src="/logo.png" alt="Company logo" />
      </div>
    );
    
    const images = container.querySelectorAll('img');
    images.forEach(img => {
      expect(img).toHaveAttribute('alt');
      expect(img.getAttribute('alt')).not.toBe('');
    });
  });
  
  it("should have proper button labeling", () => {
    render(
      <div>
        <button aria-label="Close dialog">×</button>
        <button>Save Changes</button>
        <button aria-label="Search">
          <span>🔍</span>
        </button>
      </div>
    );
    
    const closeButton = screen.getByRole('button', { name: 'Close dialog' });
    const saveButton = screen.getByRole('button', { name: 'Save Changes' });
    const searchButton = screen.getByRole('button', { name: 'Search' });
    
    expect(closeButton).toBeInTheDocument();
    expect(saveButton).toBeInTheDocument();
    expect(searchButton).toBeInTheDocument();
  });

  it("should have proper form accessibility", async () => {
    const { container } = render(
      <form>
        <div>
          <label htmlFor="username">Username</label>
          <input id="username" type="text" required aria-required="true" />
        </div>
        <div>
          <label htmlFor="email">Email Address</label>
          <input id="email" type="email" required aria-required="true" />
        </div>
        <div>
          <label htmlFor="password">Password</label>
          <input id="password" type="password" required aria-required="true" />
        </div>
        <fieldset>
          <legend>Preferences</legend>
          <label>
            <input type="checkbox" name="newsletter" />
            Subscribe to newsletter
          </label>
          <label>
            <input type="checkbox" name="updates" />
            Receive product updates
          </label>
        </fieldset>
        <button type="submit">Create Account</button>
      </form>
    );
    
    const results = await axe(container);
    expect(results).toHaveNoViolations();
  });

  it("should have proper heading hierarchy", async () => {
    const { container } = render(
      <div>
        <h1>Main Title</h1>
        <section>
          <h2>Section Title</h2>
          <article>
            <h3>Article Title</h3>
            <p>Content goes here...</p>
            <h4>Subsection</h4>
            <p>More content...</p>
          </article>
        </section>
        <section>
          <h2>Another Section</h2>
          <p>Section content...</p>
        </section>
      </div>
    );
    
    const results = await axe(container);
    expect(results).toHaveNoViolations();
  });

  it("should have proper link accessibility", async () => {
    const { container } = render(
      <div>
        <a href="/about">About Us</a>
        <a href="https://external.com" target="_blank" rel="noopener noreferrer">
          External Link
          <span className="sr-only"> (opens in new tab)</span>
        </a>
        <a href="#section1" aria-describedby="skip-link-desc">
          Skip to main content
        </a>
        <p id="skip-link-desc">This link jumps to the main content section</p>
      </div>
    );
    
    const results = await axe(container);
    expect(results).toHaveNoViolations();
  });

  it("should handle focus management properly", async () => {
    const { container } = render(
      <div>
        <button tabIndex={0}>First focusable</button>
        <input type="text" placeholder="Enter text" aria-label="Enter text" />
        <label htmlFor="options-select">Choose option</label>
        <select id="options-select">
          <option>Option 1</option>
          <option>Option 2</option>
        </select>
        <textarea placeholder="Enter description" aria-label="Enter description"></textarea>
        <button>Last focusable</button>
        <div tabIndex={-1}>Not in tab order</div>
      </div>
    );
    
    const results = await axe(container);
    expect(results).toHaveNoViolations();
  });

  it("should have proper ARIA usage", async () => {
    const { container } = render(
      <div>
        <div role="banner">
          <h1>Site Header</h1>
        </div>
        <nav role="navigation" aria-label="Main navigation">
          <ul>
            <li><a href="/">Home</a></li>
            <li><a href="/about">About</a></li>
            <li><a href="/contact">Contact</a></li>
          </ul>
        </nav>
        <main role="main">
          <section aria-labelledby="content-heading">
            <h2 id="content-heading">Main Content</h2>
            <p>This is the main content area.</p>
          </section>
        </main>
        <aside role="complementary" aria-label="Related links">
          <h3>Related</h3>
          <ul>
            <li><a href="/related1">Related Article 1</a></li>
            <li><a href="/related2">Related Article 2</a></li>
          </ul>
        </aside>
        <footer role="contentinfo">
          <p>&copy; 2024 Company Name</p>
        </footer>
      </div>
    );
    
    const results = await axe(container);
    expect(results).toHaveNoViolations();
  });

  it("should check for accessibility violations using axe", async () => {
    const { container } = render(
      <div>
        <button aria-label="Accessible Button">Click me</button>
        <img src="/test.jpg" alt="Test image" />
        <form>
          <label htmlFor="test-input">Test Input</label>
          <input id="test-input" type="text" />
        </form>
      </div>
    );
    
    const results = await axe(container);
    
    // Now we can use the custom matcher!
    expect(results).toHaveNoViolations();
  });
});
