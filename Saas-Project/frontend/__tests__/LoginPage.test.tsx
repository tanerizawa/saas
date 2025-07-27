import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import LoginPage from "../src/app/auth/login/page";

// Mock Next.js router
vi.mock("next/navigation", () => ({
  useRouter: () => ({
    push: vi.fn(),
    back: vi.fn(),
    forward: vi.fn(),
  }),
  useSearchParams: () => ({
    get: vi.fn(),
  }),
  usePathname: () => "/",
}));

// Create a mockLogin function that we can reference
const mockLogin = vi.fn();

// Mock useAuth
vi.mock("@/contexts/AuthContext", () => {
  return {
    useAuth: () => ({
      login: mockLogin,
      user: null,
      loading: false,
      token: null,
      isLoading: false,
      isLoggedIn: false,
      register: vi.fn(),
      logout: vi.fn(),
      refreshUser: vi.fn(),
    }),
    withAuth: (Component: React.ComponentType) => Component,
  };
});

describe("LoginPage Component", () => {
  beforeEach(() => {
    // Reset mocks before each test
    vi.clearAllMocks();
  });

  it("renders the login form", () => {
    render(<LoginPage />);
    
    expect(screen.getByText("Masuk ke Akun Anda")).toBeInTheDocument();
    expect(screen.getByLabelText(/Email/)).toBeInTheDocument();
    expect(screen.getByLabelText(/Password/)).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /Masuk/ })).toBeInTheDocument();
  });

  it("validates form inputs", async () => {
    const user = userEvent.setup();
    render(<LoginPage />);
    
    const loginButton = screen.getByRole("button", { name: /Masuk/ });
    
    // Submit without inputs
    await user.click(loginButton);
    
    // Wait for validation messages
    await waitFor(() => {
      expect(screen.getByText("Email wajib diisi")).toBeInTheDocument();
      expect(screen.getByText("Password wajib diisi")).toBeInTheDocument();
    });
  });

  it("handles successful login", async () => {
    // Setup successful login mock
    mockLogin.mockResolvedValueOnce(undefined);
    
    const user = userEvent.setup();
    render(<LoginPage />);
    
    await user.type(screen.getByLabelText(/Email/), "test@example.com");
    await user.type(screen.getByLabelText(/Password/), "Password123!");
    await user.click(screen.getByRole("button", { name: /Masuk/ }));
    
    await waitFor(() => {
      expect(mockLogin).toHaveBeenCalledWith({
        email: "test@example.com",
        password: "Password123!",
      });
    });
  });

  it("handles login error", async () => {
    // Setup failed login mock
    mockLogin.mockRejectedValueOnce(new Error("Invalid credentials"));
    
    const user = userEvent.setup();
    render(<LoginPage />);
    
    await user.type(screen.getByLabelText(/Email/), "wrong@example.com");
    await user.type(screen.getByLabelText(/Password/), "WrongPassword");
    await user.click(screen.getByRole("button", { name: /Masuk/ }));
    
    await waitFor(() => {
      expect(mockLogin).toHaveBeenCalledWith({
        email: "wrong@example.com",
        password: "WrongPassword",
      });
      expect(screen.getByText(/Invalid credentials/)).toBeInTheDocument();
    });
  });
});
