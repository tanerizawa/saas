import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import DocumentUpload from "../src/components/licenses/DocumentUpload";

// Mock file data
const createMockFile = (name: string, size: number, mimeType: string) => {
  const file = new File([""], name, { type: mimeType });
  Object.defineProperty(file, "size", { value: size });
  return file;
};

describe("DocumentUpload Component", () => {
  // Mock callback
  const mockUploadComplete = vi.fn();

  // Mock required documents
  const requiredDocs = ["ktp", "npwp"];

  beforeEach(() => {
    // Reset mocks before each test
    mockUploadComplete.mockClear();

    // Mock window.URL.createObjectURL
    global.URL.createObjectURL = vi.fn(() => "mock-url");
  });

  it("renders with all document types", () => {
    render(
      <DocumentUpload
        licenseId="license-123"
        requiredDocuments={requiredDocs}
        onUploadComplete={mockUploadComplete}
      />
    );

    // Check that the component header is rendered
    expect(screen.getByText("Unggah Dokumen")).toBeInTheDocument();

    // Check that document types are displayed
    expect(screen.getByText("KTP (Kartu Tanda Penduduk)")).toBeInTheDocument();
    expect(
      screen.getByText("NPWP (Nomor Pokok Wajib Pajak)")
    ).toBeInTheDocument();
    expect(screen.getByText("Akta Pendirian Perusahaan")).toBeInTheDocument();

    // Check that required labels are displayed correctly using data-testid
    expect(screen.getByTestId("document-status-ktp")).toHaveTextContent(
      "Wajib"
    );
    expect(screen.getByTestId("document-status-npwp")).toHaveTextContent(
      "Wajib"
    );
  });

  it("shows optional label for non-required documents", () => {
    render(
      <DocumentUpload
        licenseId="license-123"
        requiredDocuments={requiredDocs}
        onUploadComplete={mockUploadComplete}
      />
    );

    // Check that some document types are correctly marked as optional using data-testid
    expect(
      screen.getByTestId("document-status-akta_pendirian")
    ).toHaveTextContent("Opsional");
    expect(screen.getByTestId("document-status-surat_kuasa")).toHaveTextContent(
      "Opsional"
    );
    expect(screen.getByTestId("document-status-lainnya")).toHaveTextContent(
      "Opsional"
    );
  });

  it("handles file upload correctly", async () => {
    render(
      <DocumentUpload
        licenseId="license-123"
        requiredDocuments={requiredDocs}
        onUploadComplete={mockUploadComplete}
      />
    );

    // Create a mock file
    const file = createMockFile("test-file.pdf", 1024, "application/pdf");

    // Get the KTP file input using the data-testid
    const ktpInput = screen.getByTestId("file-input-ktp") as HTMLInputElement;

    // Simulate file upload
    await userEvent.upload(ktpInput, file);

    // Check that the file is displayed
    expect(screen.getByText("test-file.pdf")).toBeInTheDocument();

    // Initially should show "Uploading..."
    expect(screen.getByText("Uploading...")).toBeInTheDocument();

    // After timeout, should show "Uploaded"
    await waitFor(
      () => {
        expect(screen.getByText("✓ Uploaded")).toBeInTheDocument();
      },
      { timeout: 2000 }
    );
  });

  it("allows removing uploaded documents", async () => {
    render(
      <DocumentUpload
        licenseId="license-123"
        requiredDocuments={requiredDocs}
        onUploadComplete={mockUploadComplete}
      />
    );

    // Create and upload a mock file
    const file = createMockFile("test-file.pdf", 1024, "application/pdf");
    const ktpInput = screen.getByTestId("file-input-ktp") as HTMLInputElement;
    await userEvent.upload(ktpInput, file);

    // Wait for the upload to complete
    await waitFor(
      () => {
        expect(screen.getByText("✓ Uploaded")).toBeInTheDocument();
      },
      { timeout: 2000 }
    );

    // Find and click the remove button using data-testid
    const removeButton = screen.getByTestId("remove-button-test-file.pdf");
    await userEvent.click(removeButton);

    // Verify the file is removed
    expect(screen.queryByText("test-file.pdf")).not.toBeInTheDocument();
  });

  it("disables upload button when no documents are uploaded", () => {
    render(
      <DocumentUpload
        licenseId="license-123"
        requiredDocuments={requiredDocs}
        onUploadComplete={mockUploadComplete}
      />
    );

    // Find the upload button
    const uploadButton = screen.getByTestId("upload-all-button");

    // Should be disabled initially
    expect(uploadButton).toHaveClass("bg-gray-400");
    expect(uploadButton).toHaveClass("cursor-not-allowed");
  });

  it("calls onUploadComplete when uploading documents", async () => {
    render(
      <DocumentUpload
        licenseId="license-123"
        requiredDocuments={requiredDocs}
        onUploadComplete={mockUploadComplete}
      />
    );

    // Create and upload a mock file
    const file = createMockFile("test-file.pdf", 1024, "application/pdf");
    const ktpInput = screen.getByTestId("file-input-ktp") as HTMLInputElement;
    await userEvent.upload(ktpInput, file);

    // Wait for the upload to complete
    await waitFor(
      () => {
        expect(screen.getByText("✓ Uploaded")).toBeInTheDocument();
      },
      { timeout: 2000 }
    );

    // Click the upload button
    const uploadButton = screen.getByTestId("upload-all-button");
    await userEvent.click(uploadButton);

    // Wait for the simulated API call to complete
    await waitFor(
      () => {
        expect(mockUploadComplete).toHaveBeenCalledTimes(1);
      },
      { timeout: 2500 }
    );

    // Verify the callback received the document data
    expect(mockUploadComplete).toHaveBeenCalledWith(
      expect.arrayContaining([
        expect.objectContaining({
          name: "test-file.pdf",
          type: "ktp",
          status: "uploaded",
        }),
      ])
    );
  });
});
