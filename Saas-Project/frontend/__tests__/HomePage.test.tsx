import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/react";
import HomePage from "../src/app/page";

// Mocking Next.js useRouter
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

// Mocking AuthContext
vi.mock("@/contexts/AuthContext", () => {
  return {
    useAuth: () => ({
      user: null,
      loading: false,
      isLoggedIn: false,
      token: null,
      isLoading: false,
      login: vi.fn(),
      register: vi.fn(),
      logout: vi.fn(),
      refreshUser: vi.fn(),
    }),
  };
});

describe("HomePage Component", () => {
  beforeEach(() => {
    // Reset mocks before each test
    vi.clearAllMocks();
  });

  it("renders landing page when user is not logged in", () => {
    render(<HomePage />);
    
    // Check that important elements are in the document
    expect(screen.getByText("Platform SaaS untuk")).toBeInTheDocument();
    expect(screen.getByText("UMKM Indonesia")).toBeInTheDocument();
    expect(screen.getByText("Masuk")).toBeInTheDocument();
    expect(screen.getByText("Daftar")).toBeInTheDocument();
  });

  it("displays features section", () => {
    render(<HomePage />);
    
    expect(screen.getByText("Fitur Utama")).toBeInTheDocument();
    expect(screen.getByText("Manajemen Perizinan")).toBeInTheDocument();
    expect(screen.getByText("Profil Perusahaan")).toBeInTheDocument();
    expect(screen.getByText("Manajemen Keuangan")).toBeInTheDocument();
    expect(screen.getByText("Keamanan Data")).toBeInTheDocument();
  });

  it("displays call-to-action section", () => {
    render(<HomePage />);
    
    expect(screen.getByText("Siap untuk memulai?")).toBeInTheDocument();
    expect(screen.getByText("Daftar Gratis Sekarang")).toBeInTheDocument();
  });
});
