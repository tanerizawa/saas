"use client";

import { useAuth } from "@/contexts/AuthContext";
import { useState, useEffect } from "react";
import { 
  BuildingOfficeIcon,
  PencilIcon,
  CheckIcon,
  XMarkIcon,
  UserIcon,
  PhoneIcon,
  EnvelopeIcon,
  MapPinIcon,
  BuildingStorefrontIcon
} from "@heroicons/react/24/outline";

interface CompanyProfile {
  id: string;
  name: string;
  business_type: string;
  address: string;
  phone: string;
  email: string;
  owner_name: string;
  owner_phone: string;
  owner_email: string;
  registration_number?: string;
  tax_number?: string;
  established_date?: string;
  description?: string;
}

export default function UmkmProfilePage() {
  const { user, isUmkmOwner } = useAuth();
  const [profile, setProfile] = useState<CompanyProfile | null>(null);
  const [editingField, setEditingField] = useState<string | null>(null);
  const [editValues, setEditValues] = useState<Partial<CompanyProfile>>({});
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    // Mock data - replace with actual API call
    setTimeout(() => {
      setProfile({
        id: "1",
        name: "Toko Berkah Jaya",
        business_type: "Perdagangan Umum",
        address: "Jl. Merdeka No. 123, Jakarta Pusat, DKI Jakarta 10110",
        phone: "021-12345678",
        email: "info@berkah-jaya.com",
        owner_name: "Budi Santoso",
        owner_phone: "08123456789",
        owner_email: "budi@berkah-jaya.com",
        registration_number: "12345678901234567890",
        tax_number: "01.234.567.8-901.000",
        established_date: "2020-01-15",
        description: "Toko yang menjual berbagai kebutuhan sehari-hari dengan fokus pada produk lokal berkualitas."
      });
      setLoading(false);
    }, 1000);
  }, []);

  const handleEdit = (field: string) => {
    if (!profile) return;
    setEditingField(field);
    setEditValues({ [field]: profile[field as keyof CompanyProfile] });
  };

  const handleSave = async (field: string) => {
    setSaving(true);
    try {
      // TODO: Replace with actual API call
      console.log(`Saving ${field}:`, editValues[field as keyof CompanyProfile]);
      
      // Simulate API call
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      // Update local state
      setProfile(prev => prev ? { ...prev, [field]: editValues[field as keyof CompanyProfile] } : null);
      setEditingField(null);
      setEditValues({});
    } catch (error) {
      console.error('Error saving profile:', error);
    } finally {
      setSaving(false);
    }
  };

  const handleCancel = () => {
    setEditingField(null);
    setEditValues({});
  };

  const businessTypes = [
    "Perdagangan Umum",
    "Makanan dan Minuman",
    "Fashion dan Tekstil", 
    "Kerajinan Tangan",
    "Jasa",
    "Teknologi",
    "Pertanian",
    "Perikanan",
    "Peternakan",
    "Lainnya"
  ];

  if (!isUmkmOwner()) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-center">
          <BuildingOfficeIcon className="mx-auto h-16 w-16 text-red-500 mb-4" />
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

  if (!profile) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-center">
          <BuildingOfficeIcon className="mx-auto h-16 w-16 text-gray-400 mb-4" />
          <h2 className="text-xl font-semibold text-gray-900 mb-2">Profil Tidak Ditemukan</h2>
          <p className="text-gray-600">Silakan hubungi customer service untuk bantuan.</p>
        </div>
      </div>
    );
  }

  const ProfileField = ({ 
    label, 
    field, 
    value, 
    icon: Icon, 
    type = "text",
    options = null 
  }: {
    label: string;
    field: string;
    value: string;
    icon: React.ComponentType<{ className?: string }>;
    type?: string;
    options?: string[] | null;
  }) => {
    const isEditing = editingField === field;
    
    return (
      <div className="bg-white rounded-lg shadow p-6">
        <div className="flex items-start justify-between">
          <div className="flex items-start space-x-3 flex-1">
            <Icon className="h-6 w-6 text-gray-400 mt-1" />
            <div className="flex-1">
              <label className="block text-sm font-medium text-gray-700 mb-2">
                {label}
              </label>
              {isEditing ? (
                <div className="space-y-3">
                  {options ? (
                    <select
                      value={editValues[field as keyof CompanyProfile] || value}
                      onChange={(e) => setEditValues(prev => ({ ...prev, [field]: e.target.value }))}
                      className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                    >
                      {options.map(option => (
                        <option key={option} value={option}>{option}</option>
                      ))}
                    </select>
                  ) : type === "textarea" ? (
                    <textarea
                      value={editValues[field as keyof CompanyProfile] || value}
                      onChange={(e) => setEditValues(prev => ({ ...prev, [field]: e.target.value }))}
                      rows={3}
                      className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                    />
                  ) : (
                    <input
                      type={type}
                      value={editValues[field as keyof CompanyProfile] || value}
                      onChange={(e) => setEditValues(prev => ({ ...prev, [field]: e.target.value }))}
                      className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                    />
                  )}
                  <div className="flex space-x-2">
                    <button
                      onClick={() => handleSave(field)}
                      disabled={saving}
                      className="inline-flex items-center px-3 py-1 border border-transparent text-sm font-medium rounded text-white bg-green-600 hover:bg-green-700 disabled:opacity-50"
                    >
                      <CheckIcon className="h-4 w-4 mr-1" />
                      {saving ? "Menyimpan..." : "Simpan"}
                    </button>
                    <button
                      onClick={handleCancel}
                      className="inline-flex items-center px-3 py-1 border border-gray-300 text-sm font-medium rounded text-gray-700 bg-white hover:bg-gray-50"
                    >
                      <XMarkIcon className="h-4 w-4 mr-1" />
                      Batal
                    </button>
                  </div>
                </div>
              ) : (
                <div className="flex items-center justify-between">
                  <p className="text-gray-900">{value || "-"}</p>
                  <button
                    onClick={() => handleEdit(field)}
                    className="ml-2 p-1 text-gray-400 hover:text-gray-600"
                  >
                    <PencilIcon className="h-4 w-4" />
                  </button>
                </div>
              )}
            </div>
          </div>
        </div>
      </div>
    );
  };

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Header */}
      <div className="bg-white shadow-sm border-b">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="py-6">
            <div className="flex items-center">
              <BuildingOfficeIcon className="h-8 w-8 text-blue-600 mr-3" />
              <div>
                <h1 className="text-2xl font-bold text-gray-900">Profil Perusahaan</h1>
                <p className="text-sm text-gray-500">Kelola informasi perusahaan dan data pemilik</p>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="space-y-6">
          {/* Company Information Section */}
          <div className="bg-white rounded-lg shadow-sm overflow-hidden">
            <div className="px-6 py-4 border-b border-gray-200">
              <h2 className="text-lg font-medium text-gray-900 flex items-center">
                <BuildingStorefrontIcon className="h-6 w-6 text-blue-600 mr-2" />
                Informasi Perusahaan
              </h2>
            </div>
            <div className="p-6 space-y-6">
              <ProfileField
                label="Nama Perusahaan"
                field="name"
                value={profile.name}
                icon={BuildingOfficeIcon}
              />
              
              <ProfileField
                label="Jenis Usaha"
                field="business_type"
                value={profile.business_type}
                icon={BuildingStorefrontIcon}
                options={businessTypes}
              />
              
              <ProfileField
                label="Alamat Lengkap"
                field="address"
                value={profile.address}
                icon={MapPinIcon}
                type="textarea"
              />
              
              <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                <ProfileField
                  label="Telepon Perusahaan"
                  field="phone"
                  value={profile.phone}
                  icon={PhoneIcon}
                  type="tel"
                />
                
                <ProfileField
                  label="Email Perusahaan"
                  field="email"
                  value={profile.email}
                  icon={EnvelopeIcon}
                  type="email"
                />
              </div>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                <ProfileField
                  label="Nomor Registrasi"
                  field="registration_number"
                  value={profile.registration_number || ""}
                  icon={BuildingOfficeIcon}
                />
                
                <ProfileField
                  label="NPWP"
                  field="tax_number"
                  value={profile.tax_number || ""}
                  icon={BuildingOfficeIcon}
                />
              </div>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                <ProfileField
                  label="Tanggal Berdiri"
                  field="established_date"
                  value={profile.established_date || ""}
                  icon={BuildingOfficeIcon}
                  type="date"
                />
              </div>

              <ProfileField
                label="Deskripsi Usaha"
                field="description"
                value={profile.description || ""}
                icon={BuildingOfficeIcon}
                type="textarea"
              />
            </div>
          </div>

          {/* Owner Information Section */}
          <div className="bg-white rounded-lg shadow-sm overflow-hidden">
            <div className="px-6 py-4 border-b border-gray-200">
              <h2 className="text-lg font-medium text-gray-900 flex items-center">
                <UserIcon className="h-6 w-6 text-green-600 mr-2" />
                Informasi Pemilik
              </h2>
            </div>
            <div className="p-6 space-y-6">
              <ProfileField
                label="Nama Lengkap Pemilik"
                field="owner_name"
                value={profile.owner_name}
                icon={UserIcon}
              />
              
              <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                <ProfileField
                  label="Telepon Pemilik"
                  field="owner_phone"
                  value={profile.owner_phone}
                  icon={PhoneIcon}
                  type="tel"
                />
                
                <ProfileField
                  label="Email Pemilik"
                  field="owner_email"
                  value={profile.owner_email}
                  icon={EnvelopeIcon}
                  type="email"
                />
              </div>
            </div>
          </div>

          {/* Account Information */}
          <div className="bg-blue-50 rounded-lg p-6">
            <div className="flex items-start">
              <UserIcon className="h-6 w-6 text-blue-600 mt-1 mr-3" />
              <div>
                <h3 className="text-lg font-medium text-blue-900 mb-2">Informasi Akun</h3>
                <div className="space-y-2 text-sm text-blue-700">
                  <p><strong>Email Login:</strong> {user?.email}</p>
                  <p><strong>Role:</strong> UMKM Owner</p>
                  <p><strong>Status:</strong> <span className="text-green-600 font-medium">Aktif</span></p>
                </div>
                <div className="mt-4">
                  <button className="text-blue-600 hover:text-blue-500 text-sm font-medium underline">
                    Ubah Password
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
