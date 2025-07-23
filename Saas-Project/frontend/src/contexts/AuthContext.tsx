// Authentication Context for SaaS UMKM Platform
// Manages user authentication state across the application

"use client";

import React, {
  createContext,
  useContext,
  useEffect,
  useState,
  ReactNode,
} from "react";
import {
  authAPI,
  User,
  LoginRequest,
  RegisterRequest,
  isAuthenticated,
  getCurrentUser,
  getToken,
  clearAuth,
} from "@/lib/api";

interface AuthContextType {
  user: User | null;
  token: string | null;
  loading: boolean;
  isLoading: boolean;
  isLoggedIn: boolean;
  login: (credentials: LoginRequest) => Promise<void>;
  register: (
    data: RegisterRequest
  ) => Promise<{ message: string; user_id: string }>;
  logout: () => Promise<void>;
  refreshUser: () => Promise<void>;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

interface AuthProviderProps {
  children: ReactNode;
}

export function AuthProvider({ children }: AuthProviderProps) {
  const [user, setUser] = useState<User | null>(null);
  const [loading, setLoading] = useState(true);
  const [token, setToken] = useState<string | null>(null);

  const isLoggedIn = !!user && isAuthenticated();

  useEffect(() => {
    // Check for existing authentication on mount
    const checkAuth = () => {
      try {
        if (isAuthenticated()) {
          const currentUser = getCurrentUser();
          const currentToken = getToken();
          if (currentUser) {
            setUser(currentUser);
            setToken(currentToken);
          } else {
            // Token exists but no user data, try to fetch profile
            refreshUser();
          }
        } else {
          clearAuth();
        }
      } catch (error) {
        console.error("Auth check failed:", error);
        clearAuth();
      } finally {
        setLoading(false);
      }
    };

    checkAuth();
  }, []);

  const login = async (credentials: LoginRequest): Promise<void> => {
    try {
      setLoading(true);
      const response = await authAPI.login(credentials);
      setUser(response.user);
      setToken(getToken()); // Get the token that was stored in localStorage by authAPI.login
    } catch (error) {
      setUser(null);
      setToken(null);
      throw error;
    } finally {
      setLoading(false);
    }
  };

  const register = async (
    data: RegisterRequest
  ): Promise<{ message: string; user_id: string }> => {
    try {
      setLoading(true);
      const response = await authAPI.register(data);
      return response;
    } catch (error) {
      throw error;
    } finally {
      setLoading(false);
    }
  };

  const logout = async (): Promise<void> => {
    try {
      setLoading(true);
      await authAPI.logout();
    } catch (error) {
      console.error("Logout error:", error);
    } finally {
      setUser(null);
      setToken(null);
      clearAuth();
      setLoading(false);
      // Redirect to login page
      window.location.href = "/auth/login";
    }
  };

  const refreshUser = async (): Promise<void> => {
    try {
      if (isAuthenticated()) {
        const userProfile = await authAPI.getProfile();
        setUser(userProfile);
        setToken(getToken());
        // Update localStorage with fresh user data
        localStorage.setItem("user", JSON.stringify(userProfile));
      }
    } catch (error) {
      console.error("Failed to refresh user:", error);
      // If profile fetch fails, clear auth
      setUser(null);
      setToken(null);
      clearAuth();
    }
  };

  const value: AuthContextType = {
    user,
    token,
    loading,
    isLoading: loading, // Alias for backward compatibility
    isLoggedIn,
    login,
    register,
    logout,
    refreshUser,
  };

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
}

export function useAuth(): AuthContextType {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error("useAuth must be used within an AuthProvider");
  }
  return context;
}

// HOC for protecting routes
interface WithAuthOptions {
  redirectTo?: string;
  requireAuth?: boolean;
}

export function withAuth<P extends object>(
  Component: React.ComponentType<P>,
  options: WithAuthOptions = { requireAuth: true, redirectTo: "/auth/login" }
) {
  return function AuthenticatedComponent(props: P) {
    const { isLoggedIn, loading } = useAuth();
    const { requireAuth = true, redirectTo = "/auth/login" } = options;

    useEffect(() => {
      if (!loading) {
        if (requireAuth && !isLoggedIn) {
          window.location.href = redirectTo;
        } else if (!requireAuth && isLoggedIn) {
          // Redirect authenticated users away from auth pages
          window.location.href = "/dashboard";
        }
      }
    }, [isLoggedIn, loading, requireAuth, redirectTo]);

    if (loading) {
      return (
        <div className="min-h-screen flex items-center justify-center">
          <div className="animate-spin rounded-full h-32 w-32 border-b-2 border-blue-600"></div>
        </div>
      );
    }

    if (requireAuth && !isLoggedIn) {
      return null; // Will redirect
    }

    if (!requireAuth && isLoggedIn) {
      return null; // Will redirect
    }

    return <Component {...props} />;
  };
}
