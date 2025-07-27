"use client";

import dynamic from "next/dynamic";
import React from "react";
import ErrorBoundary from "./ErrorBoundary";

const MockApiToggle = dynamic(() => import("@/components/dev/MockApiToggle"), { 
  ssr: false 
});

export default function ClientComponents() {
  return (
    <ErrorBoundary fallback={<div className="hidden">Client components failed to load</div>}>
      <MockApiToggle />
    </ErrorBoundary>
  );
}
