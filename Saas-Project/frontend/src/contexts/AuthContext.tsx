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
  // Role-based helpers
  isSuperAdmin: () => boolean;
  isAdminStaff: () => boolean;
  isUmkmOwner: () => boolean;
  getDefaultRoute: () => string;
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
      console.log("🔍 AuthContext: Starting auth check...");
      try {
        if (isAuthenticated()) {
          console.log("✅ AuthContext: User is authenticated");
          const currentUser = getCurrentUser();
          const currentToken = getToken();
          console.log("👤 AuthContext: Current user:", currentUser);
          console.log("🔑 AuthContext: Current token exists:", !!currentToken);
          
          if (currentUser) {
            setUser(currentUser);
            setToken(currentToken);
            console.log("✅ AuthContext: User state set successfully");
          } else {
            console.log("⚠️ AuthContext: Token exists but no user data, trying to fetch profile");
            // Token exists but no user data, try to fetch profile
            refreshUser();
          }
        } else {
          console.log("❌ AuthContext: User not authenticated, clearing auth");
          clearAuth();
        }
      } catch (error) {
        console.error("❌ AuthContext: Auth check failed:", error);
        clearAuth();
      } finally {
        console.log("🏁 AuthContext: Auth check completed, setting loading to false");
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
      
      // Redirect to appropriate login based on current route
      const currentPath = window.location.pathname;
      if (currentPath.startsWith('/umkm/')) {
        window.location.href = '/umkm/login';
      } else {
        window.location.href = '/auth/login';
      }
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

  // Role-based helper functions
  const isSuperAdmin = (): boolean => {
    return user?.role === "super_admin";
  };

  const isAdminStaff = (): boolean => {
    return user?.role === "admin_staff";
  };

  const isUmkmOwner = (): boolean => {
    return user?.role === "umkm_owner";
  };

  const getDefaultRoute = (): string => {
    if (!user) return '/';
    
    switch (user.role) {
      case 'super_admin':
        return '/admin';
      case 'admin_staff':
        return '/staff';
      case 'umkm_owner':
        return '/umkm/dashboard';
      default:
        return '/';
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
    isSuperAdmin,
    isAdminStaff,
    isUmkmOwner,
    getDefaultRoute,
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
