import React from "react";
import { createLazyComponent } from "./LazyLoad";

// Lazily loaded components for License Management features
export const LazyLicenseForm = createLazyComponent(
  () => import("../licenses/LicenseApplicationForm"),
  { height: "400px", message: "Loading license application form..." }
);

export const LazyLicenseDetail = createLazyComponent(
  () => import("../licenses/LicenseDetail"),
  { height: "300px", message: "Loading license details..." }
);

export const LazyDocumentUpload = createLazyComponent(
  () => import("../licenses/DocumentUpload"),
  { height: "250px", message: "Loading document upload..." }
);

export const LazyLicenseList = createLazyComponent(
  () => import("../licenses/LicenseList"),
  { height: "400px", message: "Loading license list..." }
);

// Lazily loaded analytics components for Phase 4
export const LazyAnalyticsDashboard = createLazyComponent(
  () => import("../analytics/AnalyticsDashboard"),
  { height: "500px", message: "Loading analytics dashboard..." }
);

export const LazyPerformanceMetrics = createLazyComponent(
  () => import("../analytics/PerformanceMetrics"),
  { height: "350px", message: "Loading performance metrics..." }
);

// Lazily loaded user settings components
export const LazyTwoFactorSetup = createLazyComponent(
  () => import("../security/TwoFactorSetup"),
  { height: "300px", message: "Setting up two-factor authentication..." }
);
