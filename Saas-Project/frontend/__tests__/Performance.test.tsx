import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { renderHook } from "@testing-library/react";
import { usePerformanceTracking, withPerformanceTracking } from "../src/utils/performance";
import React from "react";

describe("Performance Utilities", () => {
  beforeEach(() => {
    // Mock performance API
    vi.spyOn(performance, "now").mockReturnValue(100);
    vi.spyOn(performance, "getEntriesByType").mockReturnValue([
      { name: "first-paint", startTime: 50 },
      { name: "first-contentful-paint", startTime: 100 }
    ] as PerformanceEntry[]);
    
    // Mock console.log
    vi.spyOn(console, "log").mockImplementation(() => {});
    
    // Mock PerformanceObserver
    global.PerformanceObserver = vi.fn().mockImplementation((callback) => {
      return {
        observe: vi.fn(() => {
          callback({
            getEntries: () => [{
              startTime: 200
            }]
          });
        }),
        disconnect: vi.fn()
      };
    }) as unknown as typeof PerformanceObserver;
    
    // Add supportedEntryTypes static property
    Object.defineProperty(global.PerformanceObserver, 'supportedEntryTypes', {
      value: ['largest-contentful-paint', 'first-input', 'layout-shift'],
      configurable: true
    });
  });
  
  afterEach(() => {
    vi.clearAllMocks();
  });
  
  describe("usePerformanceTracking", () => {
    it("should track component rendering performance", () => {
      // Update mock for second call to simulate end of render
      performance.now = vi.fn()
        .mockReturnValueOnce(100) // Start time
        .mockReturnValueOnce(150); // End time
      
      const { result } = renderHook(() => usePerformanceTracking("TestComponent", true));
      
      expect(result.current.componentName).toBe("TestComponent");
      expect(performance.getEntriesByType).toHaveBeenCalledWith("paint");
      expect(console.log).toHaveBeenCalled();
    });
    
    it("should not track if disabled", () => {
      const { result } = renderHook(() => usePerformanceTracking("TestComponent", false));
      
      expect(result.current.componentName).toBe("TestComponent");
      expect(performance.getEntriesByType).not.toHaveBeenCalled();
      expect(console.log).not.toHaveBeenCalled();
    });
  });
  
  describe("withPerformanceTracking", () => {
    it("should wrap a component with performance tracking", () => {
      const TestComponent: React.FC = () => <div>Test</div>;
      const WrappedComponent = withPerformanceTracking(TestComponent);
      
      expect(WrappedComponent.displayName).toBe("PerformanceTracked(TestComponent)");
    });
    
    it("should use provided component name", () => {
      const TestComponent: React.FC = () => <div>Test</div>;
      const WrappedComponent = withPerformanceTracking(TestComponent, "CustomName");
      
      expect(WrappedComponent.displayName).toBe("PerformanceTracked(CustomName)");
    });
  });
});
