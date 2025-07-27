// Mock data for licenses
import { v4 as uuidv4 } from "uuid";
import { mockConfig, getMockData } from "./config";

export interface License {
  id: string;
  type: string;
  licenseNumber: string;
  status: "pending" | "approved" | "rejected" | "expired";
  applicationDate: string;
  issuedDate?: string;
  expiryDate?: string;
  rejectionReason?: string;
  ownerId: string;
  documentUrls?: Record<string, string>;
}

// Initial licenses data
export const mockLicenses: License[] = [
  {
    id: "1",
    type: "NIB",
    licenseNumber: "NIB123456789",
    status: "approved",
    applicationDate: "2025-05-01",
    issuedDate: "2025-05-10",
    expiryDate: "2030-05-10",
    ownerId: "2",
    documentUrls: {
      "ktp": "https://example.com/mock-ktp.pdf",
      "npwp": "https://example.com/mock-npwp.pdf",
    }
  },
  {
    id: "2",
    type: "SIUP",
    licenseNumber: "SIUP987654321",
    status: "pending",
    applicationDate: "2025-06-15",
    ownerId: "2",
    documentUrls: {
      "ktp": "https://example.com/mock-ktp.pdf",
      "npwp": "https://example.com/mock-npwp.pdf",
    }
  },
  {
    id: "3",
    type: "TDP",
    licenseNumber: "TDP123123123",
    status: "rejected",
    applicationDate: "2025-04-20",
    rejectionReason: "Dokumen tidak lengkap",
    ownerId: "2",
    documentUrls: {
      "ktp": "https://example.com/mock-ktp.pdf",
    }
  }
];

export interface LicenseApplicationData {
  type: string;
  businessName: string;
  businessAddress: string;
  businessType: string;
  documents: Record<string, string>;
}

// Mock license API
export const mockLicenseAPI = {
  getLicenses: async (userId: string): Promise<License[]> => {
    if (!mockConfig.enabled) {
      throw new Error("Mock API is disabled");
    }

    const userLicenses = mockLicenses.filter(license => license.ownerId === userId);
    return getMockData<License[]>(userLicenses);
  },

  getLicenseById: async (licenseId: string): Promise<License> => {
    if (!mockConfig.enabled) {
      throw new Error("Mock API is disabled");
    }

    const license = mockLicenses.find(license => license.id === licenseId);
    
    if (!license) {
      return Promise.reject({
        response: {
          status: 404,
          data: {
            message: "License not found",
          },
        },
      });
    }
    
    return getMockData<License>(license);
  },

  applyForLicense: async (data: LicenseApplicationData, userId: string): Promise<License> => {
    if (!mockConfig.enabled) {
      throw new Error("Mock API is disabled");
    }

    const today = new Date().toISOString().split('T')[0];
    
    const newLicense: License = {
      id: uuidv4(),
      type: data.type,
      licenseNumber: `${data.type}${Date.now().toString().slice(-9)}`,
      status: "pending",
      applicationDate: today,
      ownerId: userId,
      documentUrls: data.documents,
    };
    
    mockLicenses.push(newLicense);
    return getMockData<License>(newLicense);
  },

  uploadDocument: async (file: File): Promise<{ url: string }> => {
    if (!mockConfig.enabled) {
      throw new Error("Mock API is disabled");
    }

    // Simulate document upload
    return getMockData({
      url: `https://example.com/mock-${file.name}`,
    });
  }
};
