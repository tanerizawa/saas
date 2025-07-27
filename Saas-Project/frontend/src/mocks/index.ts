// Mock API Index
// Exports all mock API services

import { mockConfig } from "./config";
import { mockAuthAPI } from "./auth";
import { mockLicenseAPI } from "./licenses";

export {
  mockConfig,
  mockAuthAPI,
  mockLicenseAPI
};

// Helper to check if mock API is enabled
export const isMockEnabled = (): boolean => mockConfig.enabled;
