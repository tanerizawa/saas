import { describe, it, expect, vi, beforeAll, afterAll } from "vitest";
import axios from "axios";
import * as api from "../src/lib/api";

// Mock axios
vi.mock("axios", () => ({
  default: {
    create: vi.fn(() => ({
      interceptors: {
        request: {
          use: vi.fn(),
        },
        response: {
          use: vi.fn(),
        },
      },
    })),
    get: vi.fn(),
    post: vi.fn(),
    put: vi.fn(),
    delete: vi.fn(),
  },
}));

describe("API Client", () => {
  beforeAll(() => {
    // Clear process.env before tests
    process.env.NEXT_PUBLIC_API_URL = undefined;
  });
  
  afterAll(() => {
    // Reset any mocks after tests
    vi.clearAllMocks();
  });
  
  it("uses correct base URL from environment variable", () => {
    // Set the environment variable
    process.env.NEXT_PUBLIC_API_URL = "http://test-api.example.com";
    
    // Force re-import of the module to get updated URL
    vi.resetModules();
    import("../src/lib/api").then(() => {
      expect(axios.create).toHaveBeenCalledWith(expect.objectContaining({
        baseURL: "http://test-api.example.com",
      }));
    });
  });
  
  it("uses default base URL when environment variable is not set", () => {
    // Clear the environment variable
    process.env.NEXT_PUBLIC_API_URL = undefined;
    
    // Force re-import of the module
    vi.resetModules();
    import("../src/lib/api").then(() => {
      expect(axios.create).toHaveBeenCalledWith(expect.objectContaining({
        baseURL: "http://localhost:8000/api/v1",
      }));
    });
  });
  
  it("exports authentication functions", () => {
    expect(api.authAPI).toBeDefined();
    expect(api.authAPI.login).toBeDefined();
    expect(api.authAPI.register).toBeDefined();
    expect(api.authAPI.logout).toBeDefined();
    expect(api.isAuthenticated).toBeDefined();
    expect(api.getToken).toBeDefined();
    expect(api.getCurrentUser).toBeDefined();
    expect(api.clearAuth).toBeDefined();
  });
});
