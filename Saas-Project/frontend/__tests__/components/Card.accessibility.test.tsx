/**
 * @vitest-environment jsdom
 */
import { describe, it, expect } from "vitest";
import { render } from "@testing-library/react";
import { axe } from "jest-axe";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "../../src/components/ui/card";
import { Button } from "../../src/components/ui/button";

describe("Card Component Accessibility", () => {
  it("should not have accessibility violations - basic card", async () => {
    const { container } = render(
      <Card>
        <CardHeader>
          <CardTitle>Card Title</CardTitle>
          <CardDescription>This is a description of the card content.</CardDescription>
        </CardHeader>
        <CardContent>
          <p>This is the main content of the card.</p>
        </CardContent>
      </Card>
    );
    
    const results = await axe(container);
    expect(results).toHaveNoViolations();
  });

  it("should not have accessibility violations - interactive card", async () => {
    const { container } = render(
      <Card>
        <CardHeader>
          <CardTitle>Product Card</CardTitle>
          <CardDescription>Premium subscription plan</CardDescription>
        </CardHeader>
        <CardContent>
          <p>Access to all premium features</p>
          <ul>
            <li>Unlimited storage</li>
            <li>Priority support</li>
            <li>Advanced analytics</li>
          </ul>
        </CardContent>
        <CardFooter>
          <Button>Select Plan</Button>
        </CardFooter>
      </Card>
    );
    
    const results = await axe(container);
    expect(results).toHaveNoViolations();
  });

  it("should not have accessibility violations - card with image", async () => {
    const { container } = render(
      <Card>
        <div className="w-full h-32 bg-gray-200 flex items-center justify-center">
          <span>Image Placeholder</span>
        </div>
        <CardHeader>
          <CardTitle>Article Title</CardTitle>
          <CardDescription>Published on January 1, 2024</CardDescription>
        </CardHeader>
        <CardContent>
          <p>This is a preview of the article content...</p>
        </CardContent>
        <CardFooter>
          <Button variant="outline">Read More</Button>
        </CardFooter>
      </Card>
    );
    
    const results = await axe(container);
    expect(results).toHaveNoViolations();
  });

  it("should not have accessibility violations - clickable card", async () => {
    const { container } = render(
      <Card 
        role="button" 
        tabIndex={0}
        className="cursor-pointer hover:shadow-lg transition-shadow"
        aria-label="Navigate to project details"
      >
        <CardHeader>
          <CardTitle>Project Alpha</CardTitle>
          <CardDescription>Web application development</CardDescription>
        </CardHeader>
        <CardContent>
          <p>Status: In Progress</p>
          <p>Due: March 15, 2024</p>
        </CardContent>
      </Card>
    );
    
    const results = await axe(container);
    expect(results).toHaveNoViolations();
  });
});
