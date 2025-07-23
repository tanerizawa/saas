"use client";

import React from "react";

type License = {
  id: string;
  type: string;
  licenseNumber: string;
  status: string;
  applicationDate: string;
  issuedDate?: string;
  expiryDate?: string;
  issuer: string;
  documents: Array<{
    id: string;
    name: string;
    type: string;
    status: string;
    url: string;
  }>;
};

type LicenseDetailProps = {
  license: License;
};

const LicenseDetail = ({ license }: LicenseDetailProps) => {
  return (
    <div className="bg-white rounded-lg shadow-sm p-6">
      <div className="flex justify-between items-start">
        <h2 className="text-2xl font-bold">{license.type}</h2>
        <span
          className={`px-3 py-1 rounded-full text-sm font-medium ${
            license.status === "approved"
              ? "bg-green-100 text-green-800"
              : license.status === "pending"
              ? "bg-yellow-100 text-yellow-800"
              : license.status === "rejected"
              ? "bg-red-100 text-red-800"
              : "bg-gray-100 text-gray-800"
          }`}
        >
          {license.status === "approved"
            ? "Disetujui"
            : license.status === "pending"
            ? "Diproses"
            : license.status === "rejected"
            ? "Ditolak"
            : "Draft"}
        </span>
      </div>

      <div className="mt-6 grid grid-cols-1 md:grid-cols-2 gap-4">
        <div>
          <p className="text-sm text-gray-500">Nomor Izin</p>
          <p className="font-medium">{license.licenseNumber || "-"}</p>
        </div>

        <div>
          <p className="text-sm text-gray-500">Tanggal Pengajuan</p>
          <p className="font-medium">
            {new Date(license.applicationDate).toLocaleDateString("id-ID")}
          </p>
        </div>

        {license.issuedDate && (
          <div>
            <p className="text-sm text-gray-500">Tanggal Terbit</p>
            <p className="font-medium">
              {new Date(license.issuedDate).toLocaleDateString("id-ID")}
            </p>
          </div>
        )}

        {license.expiryDate && (
          <div>
            <p className="text-sm text-gray-500">Tanggal Berakhir</p>
            <p className="font-medium">
              {new Date(license.expiryDate).toLocaleDateString("id-ID")}
            </p>
          </div>
        )}

        <div>
          <p className="text-sm text-gray-500">Diterbitkan Oleh</p>
          <p className="font-medium">{license.issuer}</p>
        </div>
      </div>

      <div className="mt-8">
        <h3 className="text-lg font-semibold mb-3">Dokumen Pendukung</h3>

        <div className="space-y-2">
          {license.documents.map((doc) => (
            <div
              key={doc.id}
              className="flex items-center justify-between p-3 border border-gray-200 rounded-md"
            >
              <div>
                <p className="font-medium">{doc.name}</p>
                <p className="text-sm text-gray-500">{doc.type}</p>
              </div>

              <a
                href={doc.url}
                target="_blank"
                rel="noreferrer"
                className="text-blue-600 hover:text-blue-800"
              >
                Lihat
              </a>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

export default LicenseDetail;
