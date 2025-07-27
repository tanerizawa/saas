// Mock for AuthContext
import React, { ReactNode } from "react";
import { vi } from "vitest";

export interface User {
  id: string;
  email: string;
  full_name: string;
  role: string;
}

interface RegisterRequestData {
  email: string;
  password: string;
  full_name: string;
  role?: string;
}

export interface AuthContextType {
  user: User | null;
  token: string | null;
  loading: boolean;
  isLoading: boolean;
  isLoggedIn: boolean;
  login: (credentials: { email: string; password: string }) => Promise<void>;
  register: (data: RegisterRequestData) => Promise<{ message: string; user_id: string }>;
  logout: () => Promise<void>;
  refreshUser: () => Promise<void>;
}

export const AuthContext = React.createContext<AuthContextType | undefined>(undefined);

export const useAuth = (): AuthContextType => {
  return {
    user: null,
    token: null,
    loading: false,
    isLoading: false,
    isLoggedIn: false,
    login: vi.fn().mockResolvedValue(undefined),
    register: vi.fn().mockResolvedValue({ message: "User registered", user_id: "123" }),
    logout: vi.fn().mockResolvedValue(undefined),
    refreshUser: vi.fn().mockResolvedValue(undefined),
  };
};

export const AuthProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  return <div>{children}</div>;
};

export const withAuth = (Component: React.ComponentType<unknown>) => {
  return Component;
};
