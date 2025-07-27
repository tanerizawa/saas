// Performance monitoring utilities
// Provides functions to measure and report component rendering performance

"use client";

import { useEffect, useRef } from "react";

interface PerformanceMetrics {
  componentName: string;
  renderTime: number;
  firstPaint: number | null;
  firstContentfulPaint: number | null;
  largestContentfulPaint: number | null;
}

/**
 * Hook to measure component rendering performance
 * @param componentName The name of the component to track
 * @param enabled Whether to enable performance tracking (defaults to dev mode)
 */
export function usePerformanceTracking(componentName: string, enabled = process.env.NODE_ENV === "development") {
  const startTimeRef = useRef<number>(performance.now());
  const metricsRef = useRef<PerformanceMetrics>({
    componentName,
    renderTime: 0,
    firstPaint: null,
    firstContentfulPaint: null,
    largestContentfulPaint: null,
  });

  useEffect(() => {
    if (!enabled) return;
    
    // Calculate render time
    const endTime = performance.now();
    metricsRef.current.renderTime = endTime - startTimeRef.current;
    
    // Get paint metrics
    const paintEntries = performance.getEntriesByType("paint");
    const firstPaint = paintEntries.find(entry => entry.name === "first-paint");
    const firstContentfulPaint = paintEntries.find(entry => entry.name === "first-contentful-paint");
    
    if (firstPaint) {
      metricsRef.current.firstPaint = firstPaint.startTime;
    }
    
    if (firstContentfulPaint) {
      metricsRef.current.firstContentfulPaint = firstContentfulPaint.startTime;
    }
    
    // Setup LCP observer
    if ("PerformanceObserver" in window) {
      const lcpObserver = new PerformanceObserver((entryList) => {
        const entries = entryList.getEntries();
        const lastEntry = entries[entries.length - 1];
        metricsRef.current.largestContentfulPaint = lastEntry.startTime;
        
        // Log performance metrics
        console.log(`Performance metrics for ${componentName}:`, metricsRef.current);
      });
      
      lcpObserver.observe({ type: "largest-contentful-paint", buffered: true });
      
      return () => {
        lcpObserver.disconnect();
      };
    }
    
    // Log metrics even if LCP is not available
    console.log(`Performance metrics for ${componentName}:`, metricsRef.current);
  }, [componentName, enabled]);

  return metricsRef.current;
}

/**
 * Higher-order component to wrap a component with performance tracking
 * @param Component The component to track
 * @param componentName Optional name for the component (defaults to displayName or Component.name)
 */
export function withPerformanceTracking<P extends object>(
  Component: React.ComponentType<P>,
  componentName?: string
): React.FC<P> {
  const displayName = componentName || Component.displayName || Component.name || "UnknownComponent";
  
  const PerformanceTrackedComponent: React.FC<P> = (props) => {
    usePerformanceTracking(displayName);
    return <Component {...props} />;
  };
  
  PerformanceTrackedComponent.displayName = `PerformanceTracked(${displayName})`;
  
  return PerformanceTrackedComponent;
}
