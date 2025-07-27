// API client for SaaS UMKM backend communication
// Implements type-safe API calls with error handling

import axios, { AxiosResponse, AxiosError } from "axios";

// Import mock API if enabled
import { mockAuthAPI, mockConfig } from "../mocks";

const API_BASE_URL =
  process.env.NEXT_PUBLIC_API_URL || "http://localhost:8000/api/v1";

// API Response Types
export interface ApiResponse<T = unknown> {
  data?: T;
  message?: string;
  error?: string;
  details?: string;
}

export interface AuthTokens {
  access_token: string;
  refresh_token: string;
  expires_at: string;
}

export interface User {
  id: string;
  email: string;
  full_name: string;
  role: string;
}

export interface AuthResponse {
  access_token: string;
  refresh_token: string;
  expires_at: string;
  user: User;
}

export interface RegisterRequest {
  email: string;
  password: string;
  full_name: string;
  role?: string;
}

export interface LoginRequest {
  email: string;
  password: string;
}

export interface RegisterResponse {
  message: string;
  user_id: string;
  email_verification_required: boolean;
}

// Create axios instance
const apiClient = axios.create({
  baseURL: API_BASE_URL,
  timeout: 10000,
  headers: {
    "Content-Type": "application/json",
  },
});

// Request interceptor to add auth token
apiClient.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem("access_token");
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  (error) => {
    return Promise.reject(error);
  }
);

// Response interceptor for token refresh and error handling
apiClient.interceptors.response.use(
  (response: AxiosResponse) => {
    return response;
  },
  async (error: AxiosError) => {
    const originalRequest = error.config as typeof error.config & {
      _retry?: boolean;
    };

    if (error.response?.status === 401 && !originalRequest._retry) {
      originalRequest._retry = true;

      try {
        const refreshToken = localStorage.getItem("refresh_token");
        if (refreshToken) {
          const response = await apiClient.post("/auth/refresh", {
            refresh_token: refreshToken,
          });

          const { access_token, refresh_token: newRefreshToken } =
            response.data;
          localStorage.setItem("access_token", access_token);
          localStorage.setItem("refresh_token", newRefreshToken);

          // Retry original request
          if (originalRequest.headers) {
            originalRequest.headers.Authorization = `Bearer ${access_token}`;
          }
          return apiClient(originalRequest);
        }
      } catch {
        // Refresh failed, redirect to login
        localStorage.removeItem("access_token");
        localStorage.removeItem("refresh_token");
        localStorage.removeItem("user");
        window.location.href = "/auth/login";
      }
    }

    return Promise.reject(error);
  }
);

// Auth API functions
export const authAPI = {
  register: async (data: RegisterRequest): Promise<RegisterResponse> => {
    // Use mock API if enabled
    if (mockConfig.enabled) {
      try {
        return await mockAuthAPI.register(data);
      } catch (error) {
        throw error;
      }
    }
    
    // Use real API
    const response = await apiClient.post<RegisterResponse>(
      "/auth/register",
      data
    );
    return response.data;
  },

  login: async (data: LoginRequest): Promise<AuthResponse> => {
    // Use mock API if enabled
    if (mockConfig.enabled) {
      try {
        const mockResponse = await mockAuthAPI.login(data);
        
        // Store tokens and user data
        localStorage.setItem("access_token", mockResponse.access_token);
        localStorage.setItem("refresh_token", mockResponse.refresh_token);
        localStorage.setItem("user", JSON.stringify(mockResponse.user));
        
        return mockResponse;
      } catch (error) {
        throw error;
      }
    }
    
    // Use real API
    const response = await apiClient.post<AuthResponse>("/auth/login", data);

    // Store tokens and user data
    localStorage.setItem("access_token", response.data.access_token);
    localStorage.setItem("refresh_token", response.data.refresh_token);
    localStorage.setItem("user", JSON.stringify(response.data.user));

    return response.data;
  },

  logout: async (): Promise<void> => {
    // Use mock API if enabled
    if (mockConfig.enabled) {
      try {
        await mockAuthAPI.logout();
      } catch (error) {
        console.error("Mock logout error:", error);
      }
      
      // Clear local storage
      localStorage.removeItem("access_token");
      localStorage.removeItem("refresh_token");
      localStorage.removeItem("user");
      return;
    }
    
    // Use real API
    try {
      await apiClient.post("/auth/logout");
    } finally {
      // Clear local storage regardless of API response
      localStorage.removeItem("access_token");
      localStorage.removeItem("refresh_token");
      localStorage.removeItem("user");
    }
  },

  refreshToken: async (refreshToken: string): Promise<AuthTokens> => {
    const response = await apiClient.post<AuthTokens>("/auth/refresh", {
      refresh_token: refreshToken,
    });
    return response.data;
  },

  getProfile: async (): Promise<User> => {
    const response = await apiClient.get<User>("/me");
    return response.data;
  },

  requestPasswordReset: async (email: string): Promise<{ message: string }> => {
    const response = await apiClient.post<{ message: string }>(
      "/auth/reset-password",
      {
        email,
      }
    );
    return response.data;
  },
};

// Utility functions
export const isAuthenticated = (): boolean => {
  if (typeof window === "undefined") return false;

  const token = localStorage.getItem("access_token");
  if (!token) return false;

  try {
    // Simple token expiry check (decode JWT payload)
    const payload = JSON.parse(atob(token.split(".")[1]));
    const currentTime = Math.floor(Date.now() / 1000);
    return payload.exp > currentTime;
  } catch {
    return false;
  }
};

export const getCurrentUser = (): User | null => {
  if (typeof window === "undefined") return null;

  const userStr = localStorage.getItem("user");
  if (!userStr) return null;

  try {
    return JSON.parse(userStr);
  } catch {
    return null;
  }
};

export const getToken = (): string | null => {
  if (typeof window === "undefined") return null;
  return localStorage.getItem("access_token");
};

export const clearAuth = (): void => {
  localStorage.removeItem("access_token");
  localStorage.removeItem("refresh_token");
  localStorage.removeItem("user");
};

export default apiClient;
