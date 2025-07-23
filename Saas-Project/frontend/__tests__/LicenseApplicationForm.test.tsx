import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import LicenseApplicationForm from "../src/components/licenses/LicenseApplicationForm";

describe("LicenseApplicationForm Component", () => {
  // Mock submit handler
  const mockSubmit = vi.fn();

  beforeEach(() => {
    // Clear mock data before each test
    mockSubmit.mockClear();
  });

  it("renders the form with correct initial values", () => {
    render(<LicenseApplicationForm onSubmit={mockSubmit} />);

    // Check that form title is rendered
    expect(
      screen.getByText("Formulir Pengajuan Perizinan")
    ).toBeInTheDocument();

    // Check that license type dropdown has the correct default value
    const licenseTypeSelect = screen.getByTestId("license-type-select");
    expect(licenseTypeSelect).toHaveValue("NIB");

    // Check that submit button is rendered
    expect(screen.getByText("Ajukan Perizinan")).toBeInTheDocument();
  });

  it("updates form values when user inputs data", async () => {
    render(<LicenseApplicationForm onSubmit={mockSubmit} />);

    // Type in the business name field
    const businessNameInput = screen.getByTestId("business-name-input");
    await userEvent.type(businessNameInput, "PT Example");
    expect(businessNameInput).toHaveValue("PT Example");

    // Type in the business address field
    const businessAddressInput = screen.getByTestId("business-address-input");
    await userEvent.type(businessAddressInput, "Jl. Contoh No. 123");
    expect(businessAddressInput).toHaveValue("Jl. Contoh No. 123");

    // Type in the owner name field
    const ownerNameInput = screen.getByTestId("owner-name-input");
    await userEvent.type(ownerNameInput, "John Doe");
    expect(ownerNameInput).toHaveValue("John Doe");

    // Type in the business phone field
    const businessPhoneInput = screen.getByTestId("business-phone-input");
    await userEvent.type(businessPhoneInput, "0812345678");
    expect(businessPhoneInput).toHaveValue("0812345678");

    // Select a different license type
    const licenseTypeSelect = screen.getByTestId("license-type-select");
    await userEvent.selectOptions(licenseTypeSelect, "SIUP");
    expect(licenseTypeSelect).toHaveValue("SIUP");
  });

  it("calls onSubmit with correct form data when submitted", async () => {
    render(<LicenseApplicationForm onSubmit={mockSubmit} />);

    // Fill out the form
    await userEvent.type(
      screen.getByTestId("business-name-input"),
      "PT Test Company"
    );
    await userEvent.type(
      screen.getByTestId("business-address-input"),
      "Jl. Test No. 456"
    );
    await userEvent.type(screen.getByTestId("owner-name-input"), "Jane Smith");
    await userEvent.type(
      screen.getByTestId("business-phone-input"),
      "0812345678"
    );
    await userEvent.type(
      screen.getByTestId("business-email-input"),
      "contact@testcompany.com"
    );
    await userEvent.type(
      screen.getByTestId("additional-info-input"),
      "This is a test submission"
    );
    await userEvent.selectOptions(
      screen.getByTestId("license-type-select"),
      "SIUP"
    );

    // Submit the form
    await userEvent.click(screen.getByTestId("submit-button"));

    // Verify that onSubmit was called with the correct data
    expect(mockSubmit).toHaveBeenCalledTimes(1);
    expect(mockSubmit).toHaveBeenCalledWith(
      expect.objectContaining({
        licenseType: "SIUP",
        businessName: "PT Test Company",
        businessAddress: "Jl. Test No. 456",
        ownerName: "Jane Smith",
        businessPhone: "0812345678",
        businessEmail: "contact@testcompany.com",
        additionalInfo: "This is a test submission",
      })
    );
  });

  it("initializes with provided initialValues", () => {
    // Render with initial values
    render(
      <LicenseApplicationForm
        onSubmit={mockSubmit}
        initialValues={{ type: "TDP" }}
      />
    );

    // Verify license type is set from initialValues
    const licenseTypeSelect = screen.getByTestId("license-type-select");
    expect(licenseTypeSelect).toHaveValue("TDP");
  });

  it("cancel button clears the form", async () => {
    render(<LicenseApplicationForm onSubmit={mockSubmit} />);

    // Fill out the form
    await userEvent.type(
      screen.getByTestId("business-name-input"),
      "PT Test Company"
    );

    // Click cancel button
    await userEvent.click(screen.getByTestId("cancel-button"));

    // Verify the input is empty after reset
    expect(screen.getByTestId("business-name-input")).toHaveValue("");
  });
});
