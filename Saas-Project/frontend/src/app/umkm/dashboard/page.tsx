"use client";

import { useAuth } from "@/contexts/AuthContext";
import { useState, useEffect } from "react";
import Link from "next/link";
import { 
  DocumentTextIcon, 
  ClipboardDocumentCheckIcon,
  ExclamationTriangleIcon,
  ClockIcon,
  CheckCircleIcon,
  PlusIcon,
  BuildingOfficeIcon,
  UserIcon,
  BellIcon
} from "@heroicons/react/24/outline";

interface LicenseApplication {
  id: string;
  license_type: string;
  status: "pending" | "approved" | "rejected" | "draft";
  created_at: string;
  updated_at: string;
  notes?: string;
}

interface CompanyProfile {
  id: string;
  name: string;
  business_type: string;
  address: string;
  phone: string;
  email: string;
}

export default function UmkmDashboard() {
  const { user, isUmkmOwner } = useAuth();
  const [applications, setApplications] = useState<LicenseApplication[]>([]);
  const [companyProfile, setCompanyProfile] = useState<CompanyProfile | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // Mock data - replace with actual API calls
    setTimeout(() => {
      setApplications([
        {
          id: "1",
          license_type: "SIUP (Surat Izin Usaha Perdagangan)",
          status: "pending",
          created_at: "2024-01-15T10:30:00Z",
          updated_at: "2024-01-15T10:30:00Z",
          notes: "Dokumen sedang dalam proses verifikasi"
        },
        {
          id: "2", 
          license_type: "TDP (Tanda Daftar Perusahaan)",
          status: "approved",
          created_at: "2024-01-10T09:15:00Z",
          updated_at: "2024-01-12T14:20:00Z"
        },
        {
          id: "3",
          license_type: "NPWP Badan",
          status: "draft",
          created_at: "2024-01-16T16:45:00Z",
          updated_at: "2024-01-16T16:45:00Z"
        }
      ]);

      setCompanyProfile({
        id: "1",
        name: "Toko Berkah Jaya",
        business_type: "Perdagangan Umum",
        address: "Jl. Merdeka No. 123, Jakarta",
        phone: "021-12345678",
        email: "info@berkah-jaya.com"
      });

      setLoading(false);
    }, 1000);
  }, []);

  const getStatusBadge = (status: string) => {
    const badges = {
      pending: "bg-yellow-100 text-yellow-800 border-yellow-300",
      approved: "bg-green-100 text-green-800 border-green-300",
      rejected: "bg-red-100 text-red-800 border-red-300",
      draft: "bg-gray-100 text-gray-800 border-gray-300"
    };
    
    const labels = {
      pending: "Pending",
      approved: "Disetujui", 
      rejected: "Ditolak",
      draft: "Draft"
    };

    return (
      <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border ${badges[status as keyof typeof badges]}`}>
        {labels[status as keyof typeof labels]}
      </span>
    );
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case "pending":
        return <ClockIcon className="h-5 w-5 text-yellow-500" />;
      case "approved":
        return <CheckCircleIcon className="h-5 w-5 text-green-500" />;
      case "rejected":
        return <ExclamationTriangleIcon className="h-5 w-5 text-red-500" />;
      default:
        return <DocumentTextIcon className="h-5 w-5 text-gray-500" />;
    }
  };

  if (!isUmkmOwner()) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-center">
          <ExclamationTriangleIcon className="mx-auto h-16 w-16 text-red-500 mb-4" />
          <h2 className="text-xl font-semibold text-gray-900 mb-2">Akses Ditolak</h2>
          <p className="text-gray-600">Halaman ini hanya dapat diakses oleh UMKM owner.</p>
        </div>
      </div>
    );
  }

  if (loading) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="animate-spin rounded-full h-32 w-32 border-b-2 border-blue-600"></div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Header */}
      <div className="bg-white shadow-sm border-b">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center py-4">
            <div className="flex items-center">
              <BuildingOfficeIcon className="h-8 w-8 text-blue-600 mr-3" />
              <div>
                <h1 className="text-2xl font-bold text-gray-900">Dashboard UMKM</h1>
                <p className="text-sm text-gray-500">Selamat datang, {user?.email}</p>
              </div>
            </div>
            <div className="flex items-center space-x-4">
              <button className="p-2 text-gray-400 hover:text-gray-500">
                <BellIcon className="h-6 w-6" />
              </button>
              <div className="flex items-center space-x-2">
                <UserIcon className="h-8 w-8 text-gray-400" />
                <span className="text-sm font-medium text-gray-700">{companyProfile?.name}</span>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Quick Stats */}
        <div className="grid grid-cols-1 md:grid-cols-4 gap-6 mb-8">
          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center">
              <div className="flex-shrink-0">
                <DocumentTextIcon className="h-8 w-8 text-blue-600" />
              </div>
              <div className="ml-4">
                <p className="text-sm font-medium text-gray-500">Total Perizinan</p>
                <p className="text-2xl font-semibold text-gray-900">{applications.length}</p>
              </div>
            </div>
          </div>

          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center">
              <div className="flex-shrink-0">
                <ClockIcon className="h-8 w-8 text-yellow-600" />
              </div>
              <div className="ml-4">
                <p className="text-sm font-medium text-gray-500">Pending</p>
                <p className="text-2xl font-semibold text-gray-900">
                  {applications.filter(app => app.status === 'pending').length}
                </p>
              </div>
            </div>
          </div>

          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center">
              <div className="flex-shrink-0">
                <CheckCircleIcon className="h-8 w-8 text-green-600" />
              </div>
              <div className="ml-4">
                <p className="text-sm font-medium text-gray-500">Disetujui</p>
                <p className="text-2xl font-semibold text-gray-900">
                  {applications.filter(app => app.status === 'approved').length}
                </p>
              </div>
            </div>
          </div>

          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center">
              <div className="flex-shrink-0">
                <ClipboardDocumentCheckIcon className="h-8 w-8 text-purple-600" />
              </div>
              <div className="ml-4">
                <p className="text-sm font-medium text-gray-500">Draft</p>
                <p className="text-2xl font-semibold text-gray-900">
                  {applications.filter(app => app.status === 'draft').length}
                </p>
              </div>
            </div>
          </div>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
          {/* Recent Applications */}
          <div className="lg:col-span-2">
            <div className="bg-white rounded-lg shadow">
              <div className="px-6 py-4 border-b border-gray-200">
                <div className="flex items-center justify-between">
                  <h2 className="text-lg font-medium text-gray-900">Pengajuan Perizinan Terbaru</h2>
                  <Link
                    href="/umkm/licenses"
                    className="inline-flex items-center px-3 py-2 border border-transparent text-sm leading-4 font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700"
                  >
                    <PlusIcon className="h-4 w-4 mr-1" />
                    Ajukan Baru
                  </Link>
                </div>
              </div>
              <div className="divide-y divide-gray-200">
                {applications.map((app) => (
                  <div key={app.id} className="px-6 py-4 hover:bg-gray-50">
                    <div className="flex items-center justify-between">
                      <div className="flex items-center space-x-3">
                        {getStatusIcon(app.status)}
                        <div>
                          <p className="text-sm font-medium text-gray-900">{app.license_type}</p>
                          <p className="text-sm text-gray-500">
                            Diajukan pada {new Date(app.created_at).toLocaleDateString('id-ID')}
                          </p>
                          {app.notes && (
                            <p className="text-sm text-gray-600 mt-1">{app.notes}</p>
                          )}
                        </div>
                      </div>
                      <div className="flex items-center space-x-2">
                        {getStatusBadge(app.status)}
                        <button className="text-blue-600 hover:text-blue-500 text-sm font-medium">
                          Lihat Detail
                        </button>
                      </div>
                    </div>
                  </div>
                ))}
                {applications.length === 0 && (
                  <div className="px-6 py-8 text-center">
                    <DocumentTextIcon className="mx-auto h-12 w-12 text-gray-400" />
                    <h3 className="mt-2 text-sm font-medium text-gray-900">Belum ada pengajuan</h3>
                    <p className="mt-1 text-sm text-gray-500">
                      Mulai dengan mengajukan perizinan pertama Anda.
                    </p>
                    <div className="mt-6">
                      <button className="inline-flex items-center px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700">
                        <PlusIcon className="h-4 w-4 mr-2" />
                        Ajukan Perizinan Baru
                      </button>
                    </div>
                  </div> 
                )}
              </div>
            </div>
          </div>

          {/* Company Profile */}
          <div className="space-y-6">
            <div className="bg-white rounded-lg shadow">
              <div className="px-6 py-4 border-b border-gray-200">
                <h2 className="text-lg font-medium text-gray-900">Profil Perusahaan</h2>
              </div>
              <div className="px-6 py-4">
                {companyProfile ? (
                  <div className="space-y-4">
                    <div>
                      <p className="text-sm font-medium text-gray-500">Nama Perusahaan</p>
                      <p className="text-sm text-gray-900">{companyProfile.name}</p>
                    </div>
                    <div>
                      <p className="text-sm font-medium text-gray-500">Jenis Usaha</p>
                      <p className="text-sm text-gray-900">{companyProfile.business_type}</p>
                    </div>
                    <div>
                      <p className="text-sm font-medium text-gray-500">Alamat</p>
                      <p className="text-sm text-gray-900">{companyProfile.address}</p>
                    </div>
                    <div>
                      <p className="text-sm font-medium text-gray-500">Telepon</p>
                      <p className="text-sm text-gray-900">{companyProfile.phone}</p>
                    </div>
                    <div>
                      <p className="text-sm font-medium text-gray-500">Email</p>
                      <p className="text-sm text-gray-900">{companyProfile.email}</p>
                    </div>
                    <Link 
                      href="/umkm/profile"
                      className="w-full mt-4 px-4 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 hover:bg-gray-50 text-center block"
                    >
                      Edit Profil
                    </Link>
                  </div>
                ) : (
                  <div className="text-center py-4">
                    <p className="text-sm text-gray-500">Profil perusahaan belum lengkap</p>
                    <button className="mt-2 px-4 py-2 bg-blue-600 text-white rounded-md text-sm font-medium hover:bg-blue-700">
                      Lengkapi Profil
                    </button>
                  </div>
                )}
              </div>
            </div>

            {/* Quick Actions */}
            <div className="bg-white rounded-lg shadow">
              <div className="px-6 py-4 border-b border-gray-200">
                <h2 className="text-lg font-medium text-gray-900">Aksi Cepat</h2>
              </div>
              <div className="px-6 py-4 space-y-3">
                <Link 
                  href="/umkm/licenses/apply?type=siup"
                  className="w-full text-left px-4 py-3 border border-gray-200 rounded-lg hover:bg-gray-50 transition-colors block"
                >
                  <div className="flex items-center">
                    <DocumentTextIcon className="h-5 w-5 text-blue-600 mr-3" />
                    <span className="text-sm font-medium text-gray-900">Ajukan SIUP</span>
                  </div>
                </Link>
                <Link 
                  href="/umkm/licenses/apply?type=tdp"
                  className="w-full text-left px-4 py-3 border border-gray-200 rounded-lg hover:bg-gray-50 transition-colors block"
                >
                  <div className="flex items-center">
                    <ClipboardDocumentCheckIcon className="h-5 w-5 text-green-600 mr-3" />
                    <span className="text-sm font-medium text-gray-900">Ajukan TDP</span>
                  </div>
                </Link>
                <Link 
                  href="/umkm/licenses/apply?type=npwp"
                  className="w-full text-left px-4 py-3 border border-gray-200 rounded-lg hover:bg-gray-50 transition-colors block"
                >
                  <div className="flex items-center">
                    <DocumentTextIcon className="h-5 w-5 text-purple-600 mr-3" />
                    <span className="text-sm font-medium text-gray-900">Ajukan NPWP</span>
                  </div>
                </Link>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
