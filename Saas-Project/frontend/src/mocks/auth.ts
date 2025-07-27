// Mock data for authentication
import { AuthResponse, RegisterResponse, User } from "../lib/api";
import { v4 as uuidv4 } from "uuid";
import { mockConfig, getMockData } from "./config";

// Mock user data
export const mockUsers: User[] = [
  {
    id: "1",
    email: "admin@saasumkm.com",
    full_name: "Admin UMKM",
    role: "admin",
  },
  {
    id: "2",
    email: "user@example.com",
    full_name: "User Demo",
    role: "user",
  },
];

// Mock authentication API
export const mockAuthAPI = {
  login: async (credentials: { email: string; password: string }): Promise<AuthResponse> => {
    if (!mockConfig.enabled) {
      throw new Error("Mock API is disabled");
    }

    const user = mockUsers.find(u => u.email === credentials.email);
    
    if (user && credentials.password === "password") { // Simple password check for mock
      const now = new Date();
      const expiresAt = new Date(now.getTime() + 60 * 60 * 1000); // 1 hour from now
      
      return getMockData<AuthResponse>({
        access_token: `mock-access-token-${uuidv4()}`,
        refresh_token: `mock-refresh-token-${uuidv4()}`,
        expires_at: expiresAt.toISOString(),
        user,
      });
    }
    
    // Simulate API error
    return Promise.reject({
      response: {
        status: 401,
        data: {
          message: "Invalid credentials",
        },
      },
    });
  },

  register: async (data: {
    email: string;
    password: string;
    full_name: string;
    role?: string;
  }): Promise<RegisterResponse> => {
    if (!mockConfig.enabled) {
      throw new Error("Mock API is disabled");
    }

    // Check if user with this email already exists
    if (mockUsers.some(u => u.email === data.email)) {
      return Promise.reject({
        response: {
          status: 400,
          data: {
            message: "Email already exists",
          },
        },
      });
    }

    // Create new user
    const newUserId = uuidv4();
    
    // Add to mockUsers array for future logins
    mockUsers.push({
      id: newUserId,
      email: data.email,
      full_name: data.full_name,
      role: data.role || "user",
    });

    return getMockData<RegisterResponse>({
      message: "User registered successfully",
      user_id: newUserId,
      email_verification_required: false,
    });
  },

  logout: async (): Promise<void> => {
    if (!mockConfig.enabled) {
      throw new Error("Mock API is disabled");
    }

    return getMockData<void>(undefined);
  },

  refreshToken: async (refreshToken: string): Promise<{ access_token: string; refresh_token: string; expires_at: string }> => {
    if (!mockConfig.enabled) {
      throw new Error("Mock API is disabled");
    }

    // Validate refresh token (in a real app, we'd verify this properly)
    if (refreshToken.startsWith("mock-refresh-token-")) {
      const now = new Date();
      const expiresAt = new Date(now.getTime() + 60 * 60 * 1000); // 1 hour from now
      
      return getMockData({
        access_token: `mock-access-token-${uuidv4()}`,
        refresh_token: `mock-refresh-token-${uuidv4()}`,
        expires_at: expiresAt.toISOString(),
      });
    }
    
    return Promise.reject({
      response: {
        status: 401,
        data: {
          message: "Invalid refresh token",
        },
      },
    });
  },

  getProfile: async (): Promise<User> => {
    if (!mockConfig.enabled) {
      throw new Error("Mock API is disabled");
    }

    // In a real app, we'd get this from the token
    // Here we'll just return the first mock user
    return getMockData(mockUsers[0]);
  },
};
