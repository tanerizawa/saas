"use client";

import { useState } from "react";

type Document = {
  id?: string;
  name: string;
  file: File | null;
  type: string;
  status: "uploading" | "uploaded" | "failed";
};

type DocumentUploadProps = {
  licenseId?: string;
  requiredDocuments?: string[];
  onUploadComplete?: (documents: Document[]) => void;
};

const DocumentUpload = ({
  licenseId,
  requiredDocuments = [],
  onUploadComplete,
}: DocumentUploadProps) => {
  const [documents, setDocuments] = useState<Document[]>([]);
  const [uploading, setUploading] = useState(false);

  // Define default document types for Indonesian UMKM licenses
  const documentTypes = [
    { value: "ktp", label: "KTP (Kartu Tanda Penduduk)" },
    { value: "npwp", label: "NPWP (Nomor Pokok Wajib Pajak)" },
    { value: "akta_pendirian", label: "Akta Pendirian Perusahaan" },
    { value: "surat_kuasa", label: "Surat Kuasa" },
    { value: "bukti_kepemilikan", label: "Bukti Kepemilikan Tempat Usaha" },
    { value: "izin_lokasi", label: "Izin Lokasi" },
    { value: "laporan_keuangan", label: "Laporan Keuangan" },
    { value: "sertifikat_halal", label: "Sertifikat Halal" },
    { value: "lainnya", label: "Dokumen Lainnya" },
  ];

  const handleFileChange = (
    e: React.ChangeEvent<HTMLInputElement>,
    type: string
  ) => {
    if (!e.target.files || e.target.files.length === 0) return;

    const file = e.target.files[0];

    setDocuments([
      ...documents,
      {
        name: file.name,
        file,
        type,
        status: "uploading",
      },
    ]);

    // Simulate upload process
    setTimeout(() => {
      setDocuments((prevDocs) =>
        prevDocs.map((doc) =>
          doc.name === file.name && doc.type === type
            ? { ...doc, status: "uploaded", id: `doc-${Date.now()}` }
            : doc
        )
      );
    }, 1500);
  };

  const handleRemoveDocument = (index: number) => {
    setDocuments(documents.filter((_, i) => i !== index));
  };

  const handleUpload = async () => {
    setUploading(true);

    // Simulate API call
    await new Promise((resolve) => setTimeout(resolve, 2000));

    setUploading(false);

    if (onUploadComplete) {
      onUploadComplete(documents);
    }
  };

  return (
    <div className="bg-white rounded-md shadow-sm p-6">
      <h2 className="text-2xl font-semibold mb-6">Unggah Dokumen</h2>

      <div className="space-y-6">
        {documentTypes.map((docType, index) => (
          <div
            key={docType.value}
            className="border border-gray-200 rounded-md p-4"
            data-testid={`document-section-${docType.value}`}
          >
            <div className="flex justify-between items-center mb-3">
              <h3 className="text-lg font-medium">{docType.label}</h3>
              <span
                className={`text-sm font-medium ${
                  requiredDocuments?.includes(docType.value)
                    ? "text-red-600"
                    : "text-gray-500"
                }`}
                data-testid={`document-status-${docType.value}`}
              >
                {requiredDocuments?.includes(docType.value)
                  ? "Wajib"
                  : "Opsional"}
              </span>
            </div>

            <label className="block w-full">
              <input
                type="file"
                accept=".pdf,.jpg,.jpeg,.png"
                className="hidden"
                data-testid={`file-input-${docType.value}`}
                onChange={(e) => handleFileChange(e, docType.value)}
              />
              <div className="border-2 border-dashed border-gray-300 rounded-md p-6 hover:border-blue-500 transition-colors cursor-pointer">
                <div className="flex flex-col items-center">
                  <svg
                    className="w-8 h-8 text-gray-400"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                    xmlns="http://www.w3.org/2000/svg"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth="2"
                      d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"
                    ></path>
                  </svg>
                  <p className="mt-2 text-sm text-gray-500">
                    Klik untuk pilih file atau drag & drop file disini
                  </p>
                  <p className="mt-1 text-xs text-gray-400">
                    PDF, JPG, atau PNG (Maks. 5MB)
                  </p>
                </div>
              </div>
            </label>

            {documents
              .filter((doc) => doc.type === docType.value)
              .map((doc, i) => (
                <div
                  key={i}
                  className="mt-3 flex items-center justify-between p-3 bg-gray-50 rounded-md"
                  data-testid={`uploaded-file-${doc.name}`}
                >
                  <div className="flex items-center">
                    <svg
                      className="w-5 h-5 mr-3 text-gray-400"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                      xmlns="http://www.w3.org/2000/svg"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth="2"
                        d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z"
                      ></path>
                    </svg>
                    <span className="text-sm">{doc.name}</span>
                    {doc.status === "uploading" && (
                      <div className="ml-3 animate-pulse text-yellow-500">
                        Uploading...
                      </div>
                    )}
                    {doc.status === "uploaded" && (
                      <div className="ml-3 text-green-500">✓ Uploaded</div>
                    )}
                    {doc.status === "failed" && (
                      <div className="ml-3 text-red-500">Failed</div>
                    )}
                  </div>
                  <button
                    type="button"
                    onClick={() => handleRemoveDocument(i)}
                    className="text-red-500 hover:text-red-700"
                    data-testid={`remove-button-${doc.name}`}
                  >
                    Remove
                  </button>
                </div>
              ))}
          </div>
        ))}
      </div>

      <div className="mt-8 flex justify-end">
        <button
          type="button"
          onClick={handleUpload}
          disabled={uploading || documents.length === 0}
          data-testid="upload-all-button"
          className={`py-2 px-4 rounded-md shadow-sm text-white ${
            uploading || documents.length === 0
              ? "bg-gray-400 cursor-not-allowed"
              : "bg-blue-600 hover:bg-blue-700"
          }`}
        >
          {uploading ? "Mengunggah..." : "Unggah Semua Dokumen"}
        </button>
      </div>
    </div>
  );
};

export default DocumentUpload;
