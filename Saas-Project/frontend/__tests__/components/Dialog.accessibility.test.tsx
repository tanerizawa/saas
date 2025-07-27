/**
 * @vitest-environment jsdom
 */
import { describe, it, expect } from "vitest";
import { render } from "@testing-library/react";
import { axe } from "jest-axe";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "../../src/components/ui/dialog";
import { Button } from "../../src/components/ui/button";

describe("Dialog Component Accessibility", () => {
  it("should not have accessibility violations - basic dialog", async () => {
    const { container } = render(
      <Dialog>
        <DialogTrigger asChild>
          <Button>Open Dialog</Button>
        </DialogTrigger>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Dialog Title</DialogTitle>
            <DialogDescription>
              This is a description of the dialog content.
            </DialogDescription>
          </DialogHeader>
          <div>
            <p>Dialog content goes here.</p>
          </div>
        </DialogContent>
      </Dialog>
    );
    
    const results = await axe(container);
    expect(results).toHaveNoViolations();
  });

  it("should not have accessibility violations - dialog with form", async () => {
    const { container } = render(
      <Dialog>
        <DialogTrigger asChild>
          <Button>Open Form Dialog</Button>
        </DialogTrigger>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>User Information</DialogTitle>
            <DialogDescription>
              Please fill out the form below.
            </DialogDescription>
          </DialogHeader>
          <form>
            <div className="space-y-4">
              <div>
                <label htmlFor="name">Name</label>
                <input
                  id="name"
                  type="text"
                  className="w-full p-2 border rounded"
                  placeholder="Enter your name"
                />
              </div>
              <div>
                <label htmlFor="email">Email</label>
                <input
                  id="email"
                  type="email"
                  className="w-full p-2 border rounded"
                  placeholder="Enter your email"
                />
              </div>
            </div>
          </form>
        </DialogContent>
      </Dialog>
    );
    
    const results = await axe(container);
    expect(results).toHaveNoViolations();
  });

  it("should not have accessibility violations - confirmation dialog", async () => {
    const { container } = render(
      <Dialog>
        <DialogTrigger asChild>
          <Button variant="destructive">Delete Item</Button>
        </DialogTrigger>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Confirm Deletion</DialogTitle>
            <DialogDescription>
              Are you sure you want to delete this item? This action cannot be undone.
            </DialogDescription>
          </DialogHeader>
          <div className="flex justify-end space-x-2 mt-4">
            <Button variant="outline">Cancel</Button>
            <Button variant="destructive">Delete</Button>
          </div>
        </DialogContent>
      </Dialog>
    );
    
    const results = await axe(container);
    expect(results).toHaveNoViolations();
  });
});
