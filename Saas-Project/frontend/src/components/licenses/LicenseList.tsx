"use client";

import { useState } from "react";

type License = {
  id: string;
  type: string;
  licenseNumber: string;
  status: string;
  applicationDate: string;
  issuedDate?: string;
  expiryDate?: string;
};

type LicenseListProps = {
  initialLicenses?: License[];
  onSelectLicense?: (license: License) => void;
};

const LicenseList = ({
  initialLicenses = [],
  onSelectLicense,
}: LicenseListProps) => {
  const [licenses, setLicenses] = useState<License[]>(initialLicenses);
  const [filter, setFilter] = useState("all");

  const filteredLicenses =
    filter === "all"
      ? licenses
      : licenses.filter((license) => license.status === filter);

  const getStatusBadge = (status: string) => {
    switch (status) {
      case "approved":
        return (
          <span
            className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800"
            data-testid="status-badge-approved"
          >
            Disetujui
          </span>
        );
      case "pending":
        return (
          <span
            className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-yellow-100 text-yellow-800"
            data-testid="status-badge-pending"
          >
            Diproses
          </span>
        );
      case "rejected":
        return (
          <span
            className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-red-100 text-red-800"
            data-testid="status-badge-rejected"
          >
            Ditolak
          </span>
        );
      default:
        return (
          <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-gray-100 text-gray-800">
            Draft
          </span>
        );
    }
  };

  return (
    <div className="bg-white rounded-md shadow-sm p-6">
      <div className="flex justify-between items-center mb-6">
        <h2 className="text-2xl font-semibold">Daftar Perizinan</h2>
        <div className="flex space-x-2">
          <button
            data-testid="filter-all"
            onClick={() => setFilter("all")}
            className={`px-3 py-1 text-sm rounded-md ${
              filter === "all"
                ? "bg-blue-100 text-blue-700"
                : "bg-gray-100 text-gray-700"
            }`}
          >
            Semua
          </button>
          <button
            data-testid="filter-approved"
            onClick={() => setFilter("approved")}
            className={`px-3 py-1 text-sm rounded-md ${
              filter === "approved"
                ? "bg-green-100 text-green-700"
                : "bg-gray-100 text-gray-700"
            }`}
          >
            Disetujui
          </button>
          <button
            data-testid="filter-pending"
            onClick={() => setFilter("pending")}
            className={`px-3 py-1 text-sm rounded-md ${
              filter === "pending"
                ? "bg-yellow-100 text-yellow-700"
                : "bg-gray-100 text-gray-700"
            }`}
          >
            Diproses
          </button>
          <button
            data-testid="filter-rejected"
            onClick={() => setFilter("rejected")}
            className={`px-3 py-1 text-sm rounded-md ${
              filter === "rejected"
                ? "bg-red-100 text-red-700"
                : "bg-gray-100 text-gray-700"
            }`}
          >
            Ditolak
          </button>
        </div>
      </div>

      {filteredLicenses.length === 0 ? (
        <div className="text-center py-8" data-testid="empty-state">
          <svg
            className="mx-auto h-12 w-12 text-gray-400"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth="2"
              d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
            ></path>
          </svg>
          <h3 className="mt-2 text-sm font-medium text-gray-900">
            Tidak ada perizinan
          </h3>
          <p className="mt-1 text-sm text-gray-500">
            Silakan ajukan permohonan perizinan baru untuk memulai.
          </p>
        </div>
      ) : (
        <div className="overflow-x-auto">
          <table className="min-w-full divide-y divide-gray-200">
            <thead className="bg-gray-50">
              <tr>
                <th
                  scope="col"
                  className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                >
                  Jenis Perizinan
                </th>
                <th
                  scope="col"
                  className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                >
                  Nomor
                </th>
                <th
                  scope="col"
                  className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                >
                  Tanggal Pengajuan
                </th>
                <th
                  scope="col"
                  className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                >
                  Tanggal Berakhir
                </th>
                <th
                  scope="col"
                  className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                >
                  Status
                </th>
                <th scope="col" className="relative px-6 py-3">
                  <span className="sr-only">Edit</span>
                </th>
              </tr>
            </thead>
            <tbody className="bg-white divide-y divide-gray-200">
              {filteredLicenses.map((license) => (
                <tr
                  key={license.id}
                  className="hover:bg-gray-50 cursor-pointer"
                  onClick={() => onSelectLicense?.(license)}
                  data-testid={`license-row-${license.id}`}
                >
                  <td className="px-6 py-4 whitespace-nowrap">
                    <div className="text-sm font-medium text-gray-900">
                      {license.type}
                    </div>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    <div className="text-sm text-gray-500">
                      {license.licenseNumber || "-"}
                    </div>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    <div className="text-sm text-gray-500">
                      {new Date(license.applicationDate).toLocaleDateString(
                        "id-ID"
                      )}
                    </div>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    <div className="text-sm text-gray-500">
                      {license.expiryDate
                        ? new Date(license.expiryDate).toLocaleDateString(
                            "id-ID"
                          )
                        : "-"}
                    </div>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    {getStatusBadge(license.status)}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        onSelectLicense?.(license);
                      }}
                      className="text-blue-600 hover:text-blue-900"
                      data-testid={`detail-button-${license.id}`}
                    >
                      Detail
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
};

export default LicenseList;
