import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import LicenseList from "../src/components/licenses/LicenseList";

// Mock license data
const mockLicenses = [
  {
    id: "1",
    type: "NIB",
    licenseNumber: "NIB123456789",
    status: "approved",
    applicationDate: "2025-05-01",
    issuedDate: "2025-05-10",
    expiryDate: "2030-05-10",
  },
  {
    id: "2",
    type: "SIUP",
    licenseNumber: "SIUP987654321",
    status: "pending",
    applicationDate: "2025-06-15",
  },
  {
    id: "3",
    type: "TDP",
    licenseNumber: "TDP123123123",
    status: "rejected",
    applicationDate: "2025-04-20",
  },
];

describe("LicenseList Component", () => {
  // Mock select handler
  const mockSelectLicense = vi.fn();

  beforeEach(() => {
    // Clear mock data before each test
    mockSelectLicense.mockClear();
  });

  it("renders empty state when no licenses provided", () => {
    render(<LicenseList onSelectLicense={mockSelectLicense} />);

    expect(screen.getByText("Tidak ada perizinan")).toBeInTheDocument();
    expect(
      screen.getByText(
        "Silakan ajukan permohonan perizinan baru untuk memulai."
      )
    ).toBeInTheDocument();
  });

  it("renders a list of licenses when provided", () => {
    render(
      <LicenseList
        initialLicenses={mockLicenses}
        onSelectLicense={mockSelectLicense}
      />
    );

    // Check if all license types are displayed
    expect(screen.getByText("NIB")).toBeInTheDocument();
    expect(screen.getByText("SIUP")).toBeInTheDocument();
    expect(screen.getByText("TDP")).toBeInTheDocument();

    // Check if license numbers are displayed
    expect(screen.getByText("NIB123456789")).toBeInTheDocument();
    expect(screen.getByText("SIUP987654321")).toBeInTheDocument();

    // Check status badges using test-ids
    expect(screen.getByTestId("status-badge-approved")).toBeInTheDocument();
    expect(screen.getByTestId("status-badge-pending")).toBeInTheDocument();
    expect(screen.getByTestId("status-badge-rejected")).toBeInTheDocument();
  });

  it("filters licenses based on status", async () => {
    render(
      <LicenseList
        initialLicenses={mockLicenses}
        onSelectLicense={mockSelectLicense}
      />
    );

    // Click on the "Approved" filter button
    const approvedButton = screen.getByTestId("filter-approved");
    await userEvent.click(approvedButton);

    // Should only show approved licenses
    expect(screen.getByText("NIB")).toBeInTheDocument();
    expect(screen.queryByText("SIUP")).not.toBeInTheDocument();
    expect(screen.queryByText("TDP")).not.toBeInTheDocument();

    // Click on the "Pending" filter button
    const pendingButton = screen.getByTestId("filter-pending");
    await userEvent.click(pendingButton);

    // Should only show pending licenses
    expect(screen.queryByText("NIB")).not.toBeInTheDocument();
    expect(screen.getByText("SIUP")).toBeInTheDocument();
    expect(screen.queryByText("TDP")).not.toBeInTheDocument();

    // Click on the "All" filter button to show all licenses again
    const allButton = screen.getByTestId("filter-all");
    await userEvent.click(allButton);

    // Should show all licenses again
    expect(screen.getByText("NIB")).toBeInTheDocument();
    expect(screen.getByText("SIUP")).toBeInTheDocument();
    expect(screen.getByText("TDP")).toBeInTheDocument();
  });

  it("calls onSelectLicense when a license row is clicked", async () => {
    render(
      <LicenseList
        initialLicenses={mockLicenses}
        onSelectLicense={mockSelectLicense}
      />
    );

    // Click on the first license row
    const firstLicenseRow = screen.getByTestId("license-row-1");
    await userEvent.click(firstLicenseRow);

    // Check if the onSelectLicense callback was called with the correct license
    expect(mockSelectLicense).toHaveBeenCalledTimes(1);
    expect(mockSelectLicense).toHaveBeenCalledWith(mockLicenses[0]);
  });

  it("calls onSelectLicense when the Detail button is clicked", async () => {
    render(
      <LicenseList
        initialLicenses={mockLicenses}
        onSelectLicense={mockSelectLicense}
      />
    );

    // Click the second detail button
    const detailButton = screen.getByTestId("detail-button-2");
    await userEvent.click(detailButton);

    // Check if the onSelectLicense callback was called with the correct license
    expect(mockSelectLicense).toHaveBeenCalledTimes(1);
    expect(mockSelectLicense).toHaveBeenCalledWith(mockLicenses[1]);
  });
});
