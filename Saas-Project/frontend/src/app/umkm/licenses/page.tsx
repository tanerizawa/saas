"use client";

import { useAuth } from "@/contexts/AuthContext";
import { useState, useEffect } from "react";
import { useRouter } from "next/navigation";
import { 
  DocumentTextIcon,
  ClockIcon,
  CheckCircleIcon,
  ExclamationTriangleIcon,
  PlusIcon,
  EyeIcon,
  ArrowLeftIcon
} from "@heroicons/react/24/outline";

interface LicenseApplication {
  id: string;
  license_type: string;
  status: "pending" | "approved" | "rejected" | "draft";
  created_at: string;
  updated_at: string;
  notes?: string;
  documents?: string[];
}

const LICENSE_TYPES = [
  {
    id: "siup",
    name: "SIUP (Surat Izin Usaha Perdagangan)",
    description: "Izin usaha untuk kegiatan perdagangan barang dan jasa",
    requirements: [
      "KTP pemilik",
      "Akta pendirian perusahaan",
      "NPWP perusahaan",
      "Surat keterangan domisili",
      "Foto perusahaan"
    ],
    processing_time: "7-14 hari kerja",
    fee: "Rp 500.000"
  },
  {
    id: "tdp",
    name: "TDP (Tanda Daftar Perusahaan)",
    description: "Tanda daftar perusahaan di dinas perdagangan setempat",
    requirements: [
      "SIUP yang berlaku",
      "Akta pendirian perusahaan",
      "NPWP perusahaan",
      "Surat keterangan domisili"
    ],
    processing_time: "5-10 hari kerja",
    fee: "Rp 300.000"
  },
  {
    id: "npwp",
    name: "NPWP Badan",
    description: "Nomor Pokok Wajib Pajak untuk badan usaha",
    requirements: [
      "Akta pendirian perusahaan",
      "KTP pengurus",
      "Surat keterangan domisili",
      "Surat pernyataan bermaterai"
    ],
    processing_time: "3-7 hari kerja",
    fee: "Gratis"
  },
  {
    id: "iumk",
    name: "IUMK (Izin Usaha Mikro Kecil)",
    description: "Izin usaha untuk UMKM dengan omzet di bawah Rp 300 juta",
    requirements: [
      "KTP pemilik",
      "Surat keterangan domisili",
      "Pas foto 3x4",
      "Surat pernyataan tidak memiliki SIUP"
    ],
    processing_time: "1-3 hari kerja",
    fee: "Gratis"
  }
];

export default function UmkmLicensesPage() {
  const { isUmkmOwner } = useAuth();
  const [applications, setApplications] = useState<LicenseApplication[]>([]);
  const [loading, setLoading] = useState(true);
  const [showNewApplication, setShowNewApplication] = useState(false);
  const router = useRouter();

  useEffect(() => {
    // Mock data - replace with actual API call
    setTimeout(() => {
      setApplications([
        {
          id: "1",
          license_type: "SIUP (Surat Izin Usaha Perdagangan)",
          status: "pending",
          created_at: "2024-01-15T10:30:00Z",
          updated_at: "2024-01-15T10:30:00Z",
          notes: "Dokumen sedang dalam proses verifikasi",
          documents: ["ktp.pdf", "akta-pendirian.pdf", "npwp.pdf"]
        },
        {
          id: "2", 
          license_type: "TDP (Tanda Daftar Perusahaan)",
          status: "approved",
          created_at: "2024-01-10T09:15:00Z",
          updated_at: "2024-01-12T14:20:00Z",
          documents: ["siup.pdf", "akta-pendirian.pdf"]
        },
        {
          id: "3",
          license_type: "NPWP Badan",
          status: "draft",
          created_at: "2024-01-16T16:45:00Z",
          updated_at: "2024-01-16T16:45:00Z"
        }
      ]);
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
      pending: "Sedang Diproses",
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

  const handleNewApplication = (licenseType: string) => {
    router.push(`/umkm/licenses/apply?type=${licenseType}`);
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

  if (showNewApplication) {
    return (
      <div className="min-h-screen bg-gray-50">
        {/* Header */}
        <div className="bg-white shadow-sm border-b">
          <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div className="py-6">
              <div className="flex items-center">
                <button
                  onClick={() => setShowNewApplication(false)}
                  className="mr-4 p-2 text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100"
                >
                  <ArrowLeftIcon className="h-6 w-6" />
                </button>
                <DocumentTextIcon className="h-8 w-8 text-blue-600 mr-3" />
                <div>
                  <h1 className="text-2xl font-bold text-gray-900">Pengajuan Perizinan Baru</h1>
                  <p className="text-sm text-gray-500">Pilih jenis perizinan yang ingin Anda ajukan</p>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            {LICENSE_TYPES.map((license) => (
              <div key={license.id} className="bg-white rounded-lg shadow-sm border border-gray-200 hover:shadow-md transition-shadow">
                <div className="p-6">
                  <div className="flex items-start justify-between mb-4">
                    <div className="flex-1">
                      <h3 className="text-lg font-semibold text-gray-900 mb-2">
                        {license.name}
                      </h3>
                      <p className="text-gray-600 text-sm mb-4">
                        {license.description}
                      </p>
                    </div>
                  </div>

                  <div className="space-y-4">
                    <div>
                      <h4 className="text-sm font-medium text-gray-700 mb-2">Persyaratan Dokumen:</h4>
                      <ul className="text-sm text-gray-600 space-y-1">
                        {license.requirements.map((req, index) => (
                          <li key={index} className="flex items-center">
                            <span className="w-1.5 h-1.5 bg-blue-600 rounded-full mr-2"></span>
                            {req}
                          </li>
                        ))}
                      </ul>
                    </div>

                    <div className="flex justify-between items-center text-sm">
                      <div>
                        <span className="text-gray-500">Waktu Proses: </span>
                        <span className="font-medium text-gray-900">{license.processing_time}</span>
                      </div>
                      <div>
                        <span className="text-gray-500">Biaya: </span>
                        <span className="font-medium text-gray-900">{license.fee}</span>
                      </div>
                    </div>

                    <div className="pt-4 border-t border-gray-200">
                      <button
                        onClick={() => handleNewApplication(license.id)}
                        className="w-full bg-blue-600 hover:bg-blue-700 text-white py-2 px-4 rounded-md text-sm font-medium transition-colors"
                      >
                        Ajukan Perizinan Ini
                      </button>
                    </div>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Header */}
      <div className="bg-white shadow-sm border-b">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="py-6">
            <div className="flex items-center justify-between">
              <div className="flex items-center">
                <DocumentTextIcon className="h-8 w-8 text-blue-600 mr-3" />
                <div>
                  <h1 className="text-2xl font-bold text-gray-900">Manajemen Perizinan</h1>
                  <p className="text-sm text-gray-500">Kelola pengajuan dan status perizinan usaha Anda</p>
                </div>
              </div>
              <button
                onClick={() => setShowNewApplication(true)}
                className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700"
              >
                <PlusIcon className="h-4 w-4 mr-2" />
                Ajukan Perizinan Baru
              </button>
            </div>
          </div>
        </div>
      </div>

      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Statistics */}
        <div className="grid grid-cols-1 md:grid-cols-4 gap-6 mb-8">
          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center">
              <DocumentTextIcon className="h-8 w-8 text-blue-600" />
              <div className="ml-4">
                <p className="text-sm font-medium text-gray-500">Total Pengajuan</p>
                <p className="text-2xl font-semibold text-gray-900">{applications.length}</p>
              </div>
            </div>
          </div>

          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center">
              <ClockIcon className="h-8 w-8 text-yellow-600" />
              <div className="ml-4">
                <p className="text-sm font-medium text-gray-500">Sedang Diproses</p>
                <p className="text-2xl font-semibold text-gray-900">
                  {applications.filter(app => app.status === 'pending').length}
                </p>
              </div>
            </div>
          </div>

          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center">
              <CheckCircleIcon className="h-8 w-8 text-green-600" />
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
              <DocumentTextIcon className="h-8 w-8 text-gray-600" />
              <div className="ml-4">
                <p className="text-sm font-medium text-gray-500">Draft</p>
                <p className="text-2xl font-semibold text-gray-900">
                  {applications.filter(app => app.status === 'draft').length}
                </p>
              </div>
            </div>
          </div>
        </div>

        {/* Applications List */}
        <div className="bg-white rounded-lg shadow">
          <div className="px-6 py-4 border-b border-gray-200">
            <h2 className="text-lg font-medium text-gray-900">Riwayat Pengajuan Perizinan</h2>
          </div>
          <div className="divide-y divide-gray-200">
            {applications.map((app) => (
              <div key={app.id} className="px-6 py-4 hover:bg-gray-50">
                <div className="flex items-center justify-between">
                  <div className="flex items-center space-x-4">
                    {getStatusIcon(app.status)}
                    <div className="flex-1">
                      <h3 className="text-sm font-medium text-gray-900">{app.license_type}</h3>
                      <div className="flex items-center space-x-4 mt-1">
                        <p className="text-sm text-gray-500">
                          Diajukan: {new Date(app.created_at).toLocaleDateString('id-ID')}
                        </p>
                        {app.documents && (
                          <p className="text-sm text-gray-500">
                            {app.documents.length} dokumen
                          </p>
                        )}
                      </div>
                      {app.notes && (
                        <p className="text-sm text-gray-600 mt-2">{app.notes}</p>
                      )}
                    </div>
                  </div>
                  
                  <div className="flex items-center space-x-3">
                    {getStatusBadge(app.status)}
                    <button className="inline-flex items-center px-3 py-1 border border-gray-300 rounded text-sm font-medium text-gray-700 hover:bg-gray-50">
                      <EyeIcon className="h-4 w-4 mr-1" />
                      Detail
                    </button>
                  </div>
                </div>
              </div>
            ))}
            
            {applications.length === 0 && (
              <div className="px-6 py-12 text-center">
                <DocumentTextIcon className="mx-auto h-12 w-12 text-gray-400" />
                <h3 className="mt-2 text-sm font-medium text-gray-900">Belum ada pengajuan perizinan</h3>
                <p className="mt-1 text-sm text-gray-500">
                  Mulai dengan mengajukan perizinan pertama untuk usaha Anda.
                </p>
                <div className="mt-6">
                  <button
                    onClick={() => setShowNewApplication(true)}
                    className="inline-flex items-center px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700"
                  >
                    <PlusIcon className="h-4 w-4 mr-2" />
                    Ajukan Perizinan Pertama
                  </button>
                </div>
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
