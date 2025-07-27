import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { mockAuthAPI, mockConfig } from "../src/mocks";
import { mockLicenseAPI } from "../src/mocks/licenses";

describe("Mock API Services", () => {
  // Save original mock config
  const originalEnabled = mockConfig.enabled;
  
  beforeEach(() => {
    // Enable mocks for tests
    mockConfig.enabled = true;
    
    // Reset localStorage mock
    vi.spyOn(Storage.prototype, 'setItem');
    vi.spyOn(Storage.prototype, 'getItem');
    vi.spyOn(Storage.prototype, 'removeItem');
  });
  
  afterEach(() => {
    // Restore original config
    mockConfig.enabled = originalEnabled;
    
    // Clear all mocks
    vi.clearAllMocks();
  });

  describe("Auth Mock API", () => {
    it("should login with valid credentials", async () => {
      const result = await mockAuthAPI.login({
        email: "admin@saasumkm.com",
        password: "password"
      });
      
      expect(result).toBeDefined();
      expect(result.user).toBeDefined();
      expect(result.access_token).toBeDefined();
      expect(result.user.email).toBe("admin@saasumkm.com");
    });
    
    it("should reject invalid credentials", async () => {
      await expect(
        mockAuthAPI.login({
          email: "admin@saasumkm.com",
          password: "wrongpassword"
        })
      ).rejects.toMatchObject({
        response: {
          status: 401,
          data: {
            message: "Invalid credentials"
          }
        }
      });
    });
    
    it("should register a new user", async () => {
      const result = await mockAuthAPI.register({
        email: "newuser@example.com",
        password: "Password123!",
        full_name: "New User"
      });
      
      expect(result).toBeDefined();
      expect(result.user_id).toBeDefined();
      expect(result.message).toBe("User registered successfully");
    });
    
    it("should reject registration with existing email", async () => {
      // First register a user
      await mockAuthAPI.register({
        email: "existinguser@example.com",
        password: "Password123!",
        full_name: "Existing User"
      });
      
      // Try to register with the same email
      await expect(
        mockAuthAPI.register({
          email: "existinguser@example.com",
          password: "Password123!",
          full_name: "Existing User Again"
        })
      ).rejects.toMatchObject({
        response: {
          status: 400,
          data: {
            message: "Email already exists"
          }
        }
      });
    });
  });
  
  describe("License Mock API", () => {
    it("should get licenses for a user", async () => {
      const licenses = await mockLicenseAPI.getLicenses("2");
      
      expect(licenses).toBeDefined();
      expect(licenses.length).toBeGreaterThan(0);
      expect(licenses[0].ownerId).toBe("2");
    });
    
    it("should get a license by id", async () => {
      const license = await mockLicenseAPI.getLicenseById("1");
      
      expect(license).toBeDefined();
      expect(license.id).toBe("1");
      expect(license.type).toBe("NIB");
    });
    
    it("should reject getting a non-existent license", async () => {
      await expect(
        mockLicenseAPI.getLicenseById("non-existent-id")
      ).rejects.toMatchObject({
        response: {
          status: 404,
          data: {
            message: "License not found"
          }
        }
      });
    });
    
    it("should apply for a new license", async () => {
      const userId = "2";
      const applicationData = {
        type: "SITU",
        businessName: "Test Business",
        businessAddress: "Test Address",
        businessType: "Individual",
        documents: {
          "ktp": "https://example.com/ktp-test.pdf",
          "npwp": "https://example.com/npwp-test.pdf"
        }
      };
      
      const newLicense = await mockLicenseAPI.applyForLicense(applicationData, userId);
      
      expect(newLicense).toBeDefined();
      expect(newLicense.type).toBe("SITU");
      expect(newLicense.status).toBe("pending");
      expect(newLicense.ownerId).toBe(userId);
    });
    
    it("should upload a document", async () => {
      const mockFile = new File(["test"], "test-document.pdf", { type: "application/pdf" });
      
      const result = await mockLicenseAPI.uploadDocument(mockFile);
      
      expect(result).toBeDefined();
      expect(result.url).toContain("test-document.pdf");
    });
  });
});
