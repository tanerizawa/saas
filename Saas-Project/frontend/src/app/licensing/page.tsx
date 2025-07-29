"use client";

import { useAuth } from "@/contexts/AuthContext";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import Link from "next/link";
import {
  DocumentTextIcon,
  CheckCircleIcon,
  ClockIcon,
  XCircleIcon,
  PlusIcon,
  EyeIcon,
} from "@heroicons/react/24/outline";

interface License {
  id: string;
  type: string;
  company_name: string;
  license_number?: string;
  status: "pending" | "approved" | "rejected" | "expired";
  created_at: string;
  expires_at?: string;
  description: string;
}

export default function LicensingPage() {
  const { user, isLoading } = useAuth();
  const router = useRouter();
  const [licenses, setLicenses] = useState<License[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (!isLoading && !user) {
      router.push("/auth/login");
      return;
    }
  }, [user, isLoading, router]);

  useEffect(() => {
    if (user) {
      loadLicenses();
    }
  }, [user]); // eslint-disable-line react-hooks/exhaustive-deps

  const loadLicenses = async () => {
    try {
      setLoading(true);
      
      // Mock licenses data
      const mockLicenses: License[] = user?.role === "super_admin" ? [
        {
          id: "lic-001",
          type: "NIB",
          company_name: "PT. Contoh Usaha 1",
          license_number: "NIB-2025001",
          status: "approved",
          created_at: "2025-01-15T00:00:00Z",
          expires_at: "2026-01-15T00:00:00Z",
          description: "Nomor Induk Berusaha untuk perdagangan umum"
        },
        {
          id: "lic-002",
          type: "SIUP",
          company_name: "CV. Usaha Maju",
          license_number: "SIUP-2025002",
          status: "approved",
          created_at: "2025-01-20T00:00:00Z",
          expires_at: "2026-01-20T00:00:00Z",
          description: "Surat Izin Usaha Perdagangan"
        },
        {
          id: "lic-003",
          type: "TDP",
          company_name: "UMKM Berkah",
          status: "pending",
          created_at: "2025-07-25T00:00:00Z",
          description: "Tanda Daftar Perusahaan"
        },
        {
          id: "lic-004",
          type: "Halal",
          company_name: "Warung Pak Budi",
          status: "rejected",
          created_at: "2025-06-10T00:00:00Z",
          description: "Sertifikat Halal MUI"
        }
      ] : [
        {
          id: "lic-001",
          type: "NIB",
          company_name: "UMKM Saya",
          license_number: "NIB-2025001",
          status: "approved",
          created_at: "2025-01-15T00:00:00Z",
          expires_at: "2026-01-15T00:00:00Z",
          description: "Nomor Induk Berusaha untuk perdagangan umum"
        },
        {
          id: "lic-002",
          type: "SIUP",
          company_name: "UMKM Saya",
          status: "pending",
          created_at: "2025-07-20T00:00:00Z",
          description: "Surat Izin Usaha Perdagangan"
        }
      ];

      await new Promise(resolve => setTimeout(resolve, 800));
      setLicenses(mockLicenses);
      console.log("📋 Licenses loaded:", mockLicenses);
    } catch (error) {
      console.error("Failed to load licenses:", error);
    } finally {
      setLoading(false);
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case "approved":
        return <CheckCircleIcon className="h-5 w-5 text-green-500" />;
      case "pending":
        return <ClockIcon className="h-5 w-5 text-yellow-500" />;
      case "rejected":
        return <XCircleIcon className="h-5 w-5 text-red-500" />;
      case "expired":
        return <XCircleIcon className="h-5 w-5 text-gray-500" />;
      default:
        return <ClockIcon className="h-5 w-5 text-gray-500" />;
    }
  };

  const getStatusBadge = (status: string) => {
    const baseClass = "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium";
    switch (status) {
      case "approved":
        return `${baseClass} bg-green-100 text-green-800`;
      case "pending":
        return `${baseClass} bg-yellow-100 text-yellow-800`;
      case "rejected":
        return `${baseClass} bg-red-100 text-red-800`;
      case "expired":
        return `${baseClass} bg-gray-100 text-gray-800`;
      default:
        return `${baseClass} bg-gray-100 text-gray-800`;
    }
  };

  const getStatusText = (status: string) => {
    switch (status) {
      case "approved": return "Disetujui";
      case "pending": return "Menunggu";
      case "rejected": return "Ditolak";
      case "expired": return "Kadaluarsa";
      default: return status;
    }
  };

  if (isLoading || loading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="animate-spin rounded-full h-32 w-32 border-b-2 border-blue-600"></div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Header */}
      <div className="bg-white shadow">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center py-6">
            <div>
              <nav className="flex" aria-label="Breadcrumb">
                <ol className="flex items-center space-x-4">
                  <li>
                    <Link href="/dashboard" className="text-gray-400 hover:text-gray-500">
                      Dashboard
                    </Link>
                  </li>
                  <li>
                    <span className="text-gray-400">/</span>
                  </li>
                  <li>
                    <span className="text-gray-900 font-medium">Perizinan Usaha</span>
                  </li>
                </ol>
              </nav>
              <h1 className="mt-2 text-3xl font-bold text-gray-900">
                Perizinan Usaha
              </h1>
            </div>
            <button className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700">
              <PlusIcon className="h-4 w-4 mr-2" />
              Ajukan Izin Baru
            </button>
          </div>
        </div>
      </div>

      {/* Main Content */}
      <div className="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
        <div className="px-4 py-6 sm:px-0">
          {/* Stats Cards */}
          <div className="grid grid-cols-1 md:grid-cols-4 gap-6 mb-8">
            <div className="bg-white overflow-hidden shadow rounded-lg">
              <div className="p-5">
                <div className="flex items-center">
                  <div className="flex-shrink-0">
                    <CheckCircleIcon className="h-6 w-6 text-green-500" />
                  </div>
                  <div className="ml-5 w-0 flex-1">
                    <dl>
                      <dt className="text-sm font-medium text-gray-500 truncate">
                        Izin Disetujui
                      </dt>
                      <dd className="text-lg font-medium text-gray-900">
                        {licenses.filter(l => l.status === "approved").length}
                      </dd>
                    </dl>
                  </div>
                </div>
              </div>
            </div>

            <div className="bg-white overflow-hidden shadow rounded-lg">
              <div className="p-5">
                <div className="flex items-center">
                  <div className="flex-shrink-0">
                    <ClockIcon className="h-6 w-6 text-yellow-500" />
                  </div>
                  <div className="ml-5 w-0 flex-1">
                    <dl>
                      <dt className="text-sm font-medium text-gray-500 truncate">
                        Menunggu Persetujuan
                      </dt>
                      <dd className="text-lg font-medium text-gray-900">
                        {licenses.filter(l => l.status === "pending").length}
                      </dd>
                    </dl>
                  </div>
                </div>
              </div>
            </div>

            <div className="bg-white overflow-hidden shadow rounded-lg">
              <div className="p-5">
                <div className="flex items-center">
                  <div className="flex-shrink-0">
                    <XCircleIcon className="h-6 w-6 text-red-500" />
                  </div>
                  <div className="ml-5 w-0 flex-1">
                    <dl>
                      <dt className="text-sm font-medium text-gray-500 truncate">
                        Ditolak
                      </dt>
                      <dd className="text-lg font-medium text-gray-900">
                        {licenses.filter(l => l.status === "rejected").length}
                      </dd>
                    </dl>
                  </div>
                </div>
              </div>
            </div>

            <div className="bg-white overflow-hidden shadow rounded-lg">
              <div className="p-5">
                <div className="flex items-center">
                  <div className="flex-shrink-0">
                    <DocumentTextIcon className="h-6 w-6 text-blue-500" />
                  </div>
                  <div className="ml-5 w-0 flex-1">
                    <dl>
                      <dt className="text-sm font-medium text-gray-500 truncate">
                        Total Izin
                      </dt>
                      <dd className="text-lg font-medium text-gray-900">
                        {licenses.length}
                      </dd>
                    </dl>
                  </div>
                </div>
              </div>
            </div>
          </div>

          {/* Licenses List */}
          <div className="bg-white shadow overflow-hidden sm:rounded-md">
            <div className="px-4 py-5 sm:p-6">
              <h3 className="text-lg leading-6 font-medium text-gray-900 mb-4">
                Daftar Perizinan
              </h3>
              
              <div className="space-y-4">
                {licenses.map((license) => (
                  <div key={license.id} className="border border-gray-200 rounded-lg p-4 hover:bg-gray-50">
                    <div className="flex items-center justify-between">
                      <div className="flex items-center space-x-4">
                        <div className="flex-shrink-0">
                          {getStatusIcon(license.status)}
                        </div>
                        <div>
                          <h4 className="text-lg font-medium text-gray-900">
                            {license.type} - {license.company_name}
                          </h4>
                          <p className="text-sm text-gray-500 mt-1">
                            {license.description}
                          </p>
                          {license.license_number && (
                            <p className="text-sm text-gray-600 mt-1">
                              <strong>Nomor:</strong> {license.license_number}
                            </p>
                          )}
                          <div className="flex items-center space-x-4 mt-2 text-sm text-gray-500">
                            <span>
                              Dibuat: {new Date(license.created_at).toLocaleDateString("id-ID")}
                            </span>
                            {license.expires_at && (
                              <span>
                                Berlaku sampai: {new Date(license.expires_at).toLocaleDateString("id-ID")}
                              </span>
                            )}
                          </div>
                        </div>
                      </div>
                      <div className="flex items-center space-x-3">
                        <span className={getStatusBadge(license.status)}>
                          {getStatusText(license.status)}
                        </span>
                        <button className="text-blue-600 hover:text-blue-900">
                          <EyeIcon className="h-5 w-5" />
                        </button>
                      </div>
                    </div>
                  </div>
                ))}
              </div>

              {licenses.length === 0 && (
                <div className="text-center py-8">
                  <DocumentTextIcon className="mx-auto h-12 w-12 text-gray-400" />
                  <h3 className="mt-2 text-sm font-medium text-gray-900">Belum ada perizinan</h3>
                  <p className="mt-1 text-sm text-gray-500">
                    Mulai dengan mengajukan izin usaha baru
                  </p>
                  <div className="mt-6">
                    <button className="inline-flex items-center px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700">
                      <PlusIcon className="h-4 w-4 mr-2" />
                      Ajukan Izin Pertama
                    </button>
                  </div>
                </div>
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
