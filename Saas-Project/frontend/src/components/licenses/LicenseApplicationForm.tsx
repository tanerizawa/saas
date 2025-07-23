"use client";

import { useState } from "react";

type License = {
  id: string;
  type: string;
  status: string;
  applicationDate: string;
  expiryDate?: string;
};

type LicenseFormData = {
  licenseType: string;
  businessName: string;
  businessAddress: string;
  ownerName: string;
  businessPhone: string;
  businessEmail: string;
  additionalInfo?: string;
  documentRefs?: string[];
};

type LicenseApplicationFormProps = {
  onSubmit?: (data: LicenseFormData) => void;
  initialValues?: Partial<License>;
};

const LicenseApplicationForm = ({
  onSubmit,
  initialValues,
}: LicenseApplicationFormProps) => {
  const [formData, setFormData] = useState<LicenseFormData>({
    licenseType: initialValues?.type || "NIB",
    businessName: "",
    businessAddress: "",
    ownerName: "",
    businessPhone: "",
    businessEmail: "",
    documentRefs: [],
  });

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSubmit?.(formData);
  };

  const handleChange = (
    e: React.ChangeEvent<
      HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement
    >
  ) => {
    const { name, value } = e.target;
    setFormData({
      ...formData,
      [name]: value,
    });
  };

  const handleReset = () => {
    setFormData({
      licenseType: initialValues?.type || "NIB",
      businessName: "",
      businessAddress: "",
      ownerName: "",
      businessPhone: "",
      businessEmail: "",
      documentRefs: [],
    });
  };

  return (
    <div className="bg-white rounded-md shadow-sm p-6">
      <h2 className="text-2xl font-semibold mb-6">
        Formulir Pengajuan Perizinan
      </h2>

      <form onSubmit={handleSubmit} className="space-y-4">
        <div className="space-y-2">
          <label
            htmlFor="licenseType"
            className="block text-sm font-medium text-gray-700"
          >
            Jenis Perizinan
          </label>
          <select
            id="licenseType"
            name="licenseType"
            value={formData.licenseType}
            onChange={handleChange}
            data-testid="license-type-select"
            className="mt-1 block w-full py-2 px-3 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
          >
            <option value="NIB">NIB - Nomor Induk Berusaha</option>
            <option value="SIUP">SIUP - Surat Izin Usaha Perdagangan</option>
            <option value="TDP">TDP - Tanda Daftar Perusahaan</option>
            <option value="NPWP">NPWP - Nomor Pokok Wajib Pajak (Badan)</option>
          </select>
        </div>

        <div className="space-y-2">
          <label
            htmlFor="businessName"
            className="block text-sm font-medium text-gray-700"
          >
            Nama Perusahaan
          </label>
          <input
            type="text"
            id="businessName"
            name="businessName"
            value={formData.businessName}
            onChange={handleChange}
            data-testid="business-name-input"
            className="mt-1 block w-full py-2 px-3 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
          />
        </div>

        <div className="space-y-2">
          <label
            htmlFor="businessAddress"
            className="block text-sm font-medium text-gray-700"
          >
            Alamat Perusahaan
          </label>
          <input
            type="text"
            id="businessAddress"
            name="businessAddress"
            value={formData.businessAddress}
            onChange={handleChange}
            data-testid="business-address-input"
            className="mt-1 block w-full py-2 px-3 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
          />
        </div>

        <div className="space-y-2">
          <label
            htmlFor="ownerName"
            className="block text-sm font-medium text-gray-700"
          >
            Nama Pemilik
          </label>
          <input
            type="text"
            id="ownerName"
            name="ownerName"
            value={formData.ownerName}
            onChange={handleChange}
            data-testid="owner-name-input"
            className="mt-1 block w-full py-2 px-3 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
          />
        </div>

        <div className="space-y-2">
          <label
            htmlFor="businessPhone"
            className="block text-sm font-medium text-gray-700"
          >
            Telepon Perusahaan
          </label>
          <input
            type="tel"
            id="businessPhone"
            name="businessPhone"
            value={formData.businessPhone}
            onChange={handleChange}
            data-testid="business-phone-input"
            className="mt-1 block w-full py-2 px-3 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
          />
        </div>

        <div className="space-y-2">
          <label
            htmlFor="businessEmail"
            className="block text-sm font-medium text-gray-700"
          >
            Email Perusahaan
          </label>
          <input
            type="email"
            id="businessEmail"
            name="businessEmail"
            value={formData.businessEmail}
            onChange={handleChange}
            data-testid="business-email-input"
            className="mt-1 block w-full py-2 px-3 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
          />
        </div>

        <div className="space-y-2">
          <label
            htmlFor="additionalInfo"
            className="block text-sm font-medium text-gray-700"
          >
            Informasi Tambahan
          </label>
          <textarea
            id="additionalInfo"
            name="additionalInfo"
            value={formData.additionalInfo || ""}
            onChange={handleChange}
            data-testid="additional-info-input"
            rows={3}
            className="mt-1 block w-full py-2 px-3 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
          />
        </div>

        <div className="pt-5">
          <div className="flex justify-end">
            <button
              type="button"
              onClick={handleReset}
              data-testid="cancel-button"
              className="bg-white py-2 px-4 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 mr-3"
            >
              Batal
            </button>
            <button
              type="submit"
              data-testid="submit-button"
              className="bg-blue-600 py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
            >
              Ajukan Perizinan
            </button>
          </div>
        </div>
      </form>
    </div>
  );
};

export default LicenseApplicationForm;
