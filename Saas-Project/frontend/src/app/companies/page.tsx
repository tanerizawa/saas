"use client";

import { useState, useEffect, useCallback } from "react";
import { useAuth } from "@/contexts/AuthContext";
import { toast } from "react-hot-toast";

interface Company {
  id: string;
  owner_id: string;
  company_name: string;
  business_type: string;
  business_scale: string;
  industry: string;
  description?: string;
  address: {
    street: string;
    city: string;
    province: string;
    postal_code: string;
  };
  phone?: string;
  email?: string;
  website?: string;
  nib?: string;
  siup?: string;
  tdp?: string;
  npwp?: string;
  employee_count?: number;
  annual_revenue?: number;
  status: string;
  verification_status: string;
  verification_notes?: string;
  created_at: string;
  updated_at: string;
}

interface CreateCompanyRequest {
  company_name: string;
  business_type: string;
  industry: string;
  description?: string;
  address_street: string;
  address_city: string;
  address_province: string;
  address_postal_code: string;
  phone?: string;
  email?: string;
  website?: string;
  nib?: string;
  siup?: string;
  tdp?: string;
  npwp?: string;
  employee_count?: number;
  annual_revenue?: number;
}

const businessTypes = [
  { value: "pt", label: "PT (Perseroan Terbatas)" },
  { value: "cv", label: "CV (Commanditaire Vennootschap)" },
  { value: "ud", label: "UD (Usaha Dagang)" },
  { value: "koperasi", label: "Koperasi" },
  { value: "perorangan", label: "Perorangan" },
];

const industries = [
  "Teknologi Informasi",
  "Manufaktur",
  "Perdagangan",
  "Jasa",
  "Pertanian",
  "Perikanan",
  "Konstruksi",
  "Transportasi",
  "Pariwisata",
  "Pendidikan",
  "Kesehatan",
  "Makanan & Minuman",
  "Fashion & Tekstil",
  "Kerajinan",
  "Lainnya",
];

const provinces = [
  "DKI Jakarta",
  "Jawa Barat",
  "Jawa Tengah",
  "Jawa Timur",
  "Sumatera Utara",
  "Sumatera Barat",
  "Sumatera Selatan",
  "Bali",
  "Yogyakarta",
  "Banten",
  "Kalimantan Timur",
  "Sulawesi Selatan",
  "Lainnya",
];

export default function CompaniesPage() {
  const { token } = useAuth();
  const [companies, setCompanies] = useState<Company[]>([]);
  const [loading, setLoading] = useState(true);
  const [showCreateForm, setShowCreateForm] = useState(false);
  const [formData, setFormData] = useState<CreateCompanyRequest>({
    company_name: "",
    business_type: "",
    industry: "",
    description: "",
    address_street: "",
    address_city: "",
    address_province: "",
    address_postal_code: "",
    phone: "",
    email: "",
    website: "",
    nib: "",
    siup: "",
    tdp: "",
    npwp: "",
    employee_count: undefined,
    annual_revenue: undefined,
  });
  const [submitting, setSubmitting] = useState(false);

  const fetchCompanies = useCallback(async () => {
    try {
      const response = await fetch(
        "http://localhost:8000/api/v1/companies/my",
        {
          headers: {
            Authorization: `Bearer ${token}`,
            "Content-Type": "application/json",
          },
        }
      );

      if (response.ok) {
        const data = await response.json();
        setCompanies(data);
      } else {
        toast.error("Gagal memuat data perusahaan");
      }
    } catch (error) {
      console.error("Error fetching companies:", error);
      toast.error("Terjadi kesalahan saat memuat data");
    } finally {
      setLoading(false);
    }
  }, [token]);

  useEffect(() => {
    if (token) {
      fetchCompanies();
    }
  }, [token, fetchCompanies]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setSubmitting(true);

    try {
      const response = await fetch("http://localhost:8000/api/v1/companies", {
        method: "POST",
        headers: {
          Authorization: `Bearer ${token}`,
          "Content-Type": "application/json",
        },
        body: JSON.stringify(formData),
      });

      if (response.ok) {
        toast.success("Perusahaan berhasil didaftarkan!");
        setShowCreateForm(false);
        setFormData({
          company_name: "",
          business_type: "",
          industry: "",
          description: "",
          address_street: "",
          address_city: "",
          address_province: "",
          address_postal_code: "",
          phone: "",
          email: "",
          website: "",
          nib: "",
          siup: "",
          tdp: "",
          npwp: "",
          employee_count: undefined,
          annual_revenue: undefined,
        });
        fetchCompanies();
      } else {
        const errorData = await response.json();
        toast.error(errorData.message || "Gagal mendaftarkan perusahaan");
      }
    } catch (error) {
      console.error("Error creating company:", error);
      toast.error("Terjadi kesalahan saat mendaftarkan perusahaan");
    } finally {
      setSubmitting(false);
    }
  };

  const handleInputChange = (
    e: React.ChangeEvent<
      HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement
    >
  ) => {
    const { name, value } = e.target;
    setFormData((prev) => ({
      ...prev,
      [name]:
        name === "employee_count" || name === "annual_revenue"
          ? value === ""
            ? undefined
            : Number(value)
          : value,
    }));
  };

  const formatCurrency = (amount: number) => {
    return new Intl.NumberFormat("id-ID", {
      style: "currency",
      currency: "IDR",
      maximumFractionDigits: 0,
    }).format(amount);
  };

  const getBusinessScale = (scale: string) => {
    switch (scale) {
      case "mikro":
        return "Mikro";
      case "kecil":
        return "Kecil";
      case "menengah":
        return "Menengah";
      default:
        return scale;
    }
  };

  const getVerificationStatus = (status: string) => {
    switch (status) {
      case "verified":
        return { text: "Terverifikasi", color: "text-green-600 bg-green-100" };
      case "pending":
        return {
          text: "Menunggu Verifikasi",
          color: "text-yellow-600 bg-yellow-100",
        };
      case "rejected":
        return { text: "Ditolak", color: "text-red-600 bg-red-100" };
      default:
        return { text: status, color: "text-gray-600 bg-gray-100" };
    }
  };

  if (loading) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto"></div>
          <p className="mt-4 text-gray-600">Memuat data perusahaan...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50 py-8">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        {/* Header */}
        <div className="sm:flex sm:items-center sm:justify-between">
          <div>
            <h1 className="text-2xl font-bold text-gray-900">
              Manajemen Perusahaan
            </h1>
            <p className="mt-2 text-sm text-gray-700">
              Kelola profil dan data perusahaan UMKM Anda
            </p>
          </div>
          <div className="mt-4 sm:mt-0">
            <button
              onClick={() => setShowCreateForm(true)}
              className="inline-flex items-center justify-center rounded-md border border-transparent bg-blue-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
            >
              Daftar Perusahaan Baru
            </button>
          </div>
        </div>

        {/* Companies List */}
        <div className="mt-8">
          {companies.length === 0 ? (
            <div className="text-center py-12">
              <div className="mx-auto h-12 w-12 text-gray-400">
                <svg
                  className="h-12 w-12"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"
                  />
                </svg>
              </div>
              <h3 className="mt-2 text-sm font-medium text-gray-900">
                Belum ada perusahaan
              </h3>
              <p className="mt-1 text-sm text-gray-500">
                Mulai dengan mendaftarkan perusahaan UMKM Anda.
              </p>
              <div className="mt-6">
                <button
                  onClick={() => setShowCreateForm(true)}
                  className="inline-flex items-center px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                >
                  Daftar Perusahaan Pertama
                </button>
              </div>
            </div>
          ) : (
            <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
              {companies.map((company) => {
                const verificationStatus = getVerificationStatus(
                  company.verification_status
                );
                return (
                  <div
                    key={company.id}
                    className="bg-white overflow-hidden shadow rounded-lg"
                  >
                    <div className="p-6">
                      <div className="flex items-center justify-between">
                        <h3 className="text-lg font-medium text-gray-900 truncate">
                          {company.company_name}
                        </h3>
                        <span
                          className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${verificationStatus.color}`}
                        >
                          {verificationStatus.text}
                        </span>
                      </div>

                      <div className="mt-4 space-y-2">
                        <div className="flex justify-between text-sm">
                          <span className="text-gray-500">Jenis Usaha:</span>
                          <span className="font-medium">
                            {company.business_type.toUpperCase()}
                          </span>
                        </div>

                        <div className="flex justify-between text-sm">
                          <span className="text-gray-500">Skala Usaha:</span>
                          <span className="font-medium">
                            {getBusinessScale(company.business_scale)}
                          </span>
                        </div>

                        <div className="flex justify-between text-sm">
                          <span className="text-gray-500">Industri:</span>
                          <span className="font-medium">
                            {company.industry}
                          </span>
                        </div>

                        {company.employee_count && (
                          <div className="flex justify-between text-sm">
                            <span className="text-gray-500">Karyawan:</span>
                            <span className="font-medium">
                              {company.employee_count} orang
                            </span>
                          </div>
                        )}

                        {company.annual_revenue && (
                          <div className="flex justify-between text-sm">
                            <span className="text-gray-500">
                              Omzet Tahunan:
                            </span>
                            <span className="font-medium">
                              {formatCurrency(company.annual_revenue)}
                            </span>
                          </div>
                        )}

                        <div className="pt-2 border-t border-gray-200">
                          <div className="flex justify-between text-xs text-gray-500">
                            <span>Lokasi:</span>
                            <span>
                              {company.address.city}, {company.address.province}
                            </span>
                          </div>
                        </div>
                      </div>

                      <div className="mt-6 flex space-x-3">
                        <button className="flex-1 bg-white py-2 px-3 border border-gray-300 rounded-md shadow-sm text-sm leading-4 font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500">
                          Detail
                        </button>
                        <button className="flex-1 bg-blue-600 py-2 px-3 border border-transparent rounded-md shadow-sm text-sm leading-4 font-medium text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500">
                          Edit
                        </button>
                      </div>
                    </div>
                  </div>
                );
              })}
            </div>
          )}
        </div>

        {/* Create Company Modal */}
        {showCreateForm && (
          <div className="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50">
            <div className="relative top-20 mx-auto p-5 border w-11/12 md:w-3/4 lg:w-2/3 xl:w-1/2 shadow-lg rounded-md bg-white">
              <div className="mt-3">
                <div className="flex items-center justify-between pb-3">
                  <h3 className="text-lg font-medium text-gray-900">
                    Daftar Perusahaan Baru
                  </h3>
                  <button
                    onClick={() => setShowCreateForm(false)}
                    className="text-gray-400 hover:text-gray-600"
                  >
                    <span className="sr-only">Close</span>
                    <svg
                      className="h-6 w-6"
                      fill="none"
                      viewBox="0 0 24 24"
                      stroke="currentColor"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M6 18L18 6M6 6l12 12"
                      />
                    </svg>
                  </button>
                </div>

                <form onSubmit={handleSubmit} className="space-y-6">
                  <div className="grid grid-cols-1 gap-6 sm:grid-cols-2">
                    {/* Company Name */}
                    <div className="sm:col-span-2">
                      <label
                        htmlFor="company_name"
                        className="block text-sm font-medium text-gray-700"
                      >
                        Nama Perusahaan *
                      </label>
                      <input
                        type="text"
                        name="company_name"
                        id="company_name"
                        required
                        value={formData.company_name}
                        onChange={handleInputChange}
                        className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
                      />
                    </div>

                    {/* Business Type */}
                    <div>
                      <label
                        htmlFor="business_type"
                        className="block text-sm font-medium text-gray-700"
                      >
                        Jenis Usaha *
                      </label>
                      <select
                        name="business_type"
                        id="business_type"
                        required
                        value={formData.business_type}
                        onChange={handleInputChange}
                        className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
                      >
                        <option value="">Pilih Jenis Usaha</option>
                        {businessTypes.map((type) => (
                          <option key={type.value} value={type.value}>
                            {type.label}
                          </option>
                        ))}
                      </select>
                    </div>

                    {/* Industry */}
                    <div>
                      <label
                        htmlFor="industry"
                        className="block text-sm font-medium text-gray-700"
                      >
                        Industri *
                      </label>
                      <select
                        name="industry"
                        id="industry"
                        required
                        value={formData.industry}
                        onChange={handleInputChange}
                        className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
                      >
                        <option value="">Pilih Industri</option>
                        {industries.map((industry) => (
                          <option key={industry} value={industry}>
                            {industry}
                          </option>
                        ))}
                      </select>
                    </div>

                    {/* Description */}
                    <div className="sm:col-span-2">
                      <label
                        htmlFor="description"
                        className="block text-sm font-medium text-gray-700"
                      >
                        Deskripsi Usaha
                      </label>
                      <textarea
                        name="description"
                        id="description"
                        rows={3}
                        value={formData.description}
                        onChange={handleInputChange}
                        className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
                        placeholder="Jelaskan tentang usaha Anda..."
                      />
                    </div>

                    {/* Address */}
                    <div className="sm:col-span-2">
                      <label
                        htmlFor="address_street"
                        className="block text-sm font-medium text-gray-700"
                      >
                        Alamat Lengkap *
                      </label>
                      <input
                        type="text"
                        name="address_street"
                        id="address_street"
                        required
                        value={formData.address_street}
                        onChange={handleInputChange}
                        className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
                        placeholder="Jalan, No. Rumah, RT/RW"
                      />
                    </div>

                    <div>
                      <label
                        htmlFor="address_city"
                        className="block text-sm font-medium text-gray-700"
                      >
                        Kota/Kabupaten *
                      </label>
                      <input
                        type="text"
                        name="address_city"
                        id="address_city"
                        required
                        value={formData.address_city}
                        onChange={handleInputChange}
                        className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
                      />
                    </div>

                    <div>
                      <label
                        htmlFor="address_province"
                        className="block text-sm font-medium text-gray-700"
                      >
                        Provinsi *
                      </label>
                      <select
                        name="address_province"
                        id="address_province"
                        required
                        value={formData.address_province}
                        onChange={handleInputChange}
                        className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
                      >
                        <option value="">Pilih Provinsi</option>
                        {provinces.map((province) => (
                          <option key={province} value={province}>
                            {province}
                          </option>
                        ))}
                      </select>
                    </div>

                    <div>
                      <label
                        htmlFor="address_postal_code"
                        className="block text-sm font-medium text-gray-700"
                      >
                        Kode Pos *
                      </label>
                      <input
                        type="text"
                        name="address_postal_code"
                        id="address_postal_code"
                        required
                        value={formData.address_postal_code}
                        onChange={handleInputChange}
                        className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
                      />
                    </div>

                    {/* Contact Info */}
                    <div>
                      <label
                        htmlFor="phone"
                        className="block text-sm font-medium text-gray-700"
                      >
                        Nomor Telepon
                      </label>
                      <input
                        type="tel"
                        name="phone"
                        id="phone"
                        value={formData.phone}
                        onChange={handleInputChange}
                        className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
                      />
                    </div>

                    <div>
                      <label
                        htmlFor="email"
                        className="block text-sm font-medium text-gray-700"
                      >
                        Email Perusahaan
                      </label>
                      <input
                        type="email"
                        name="email"
                        id="email"
                        value={formData.email}
                        onChange={handleInputChange}
                        className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
                      />
                    </div>

                    <div className="sm:col-span-2">
                      <label
                        htmlFor="website"
                        className="block text-sm font-medium text-gray-700"
                      >
                        Website
                      </label>
                      <input
                        type="url"
                        name="website"
                        id="website"
                        value={formData.website}
                        onChange={handleInputChange}
                        className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
                        placeholder="https://contoh.com"
                      />
                    </div>

                    {/* Business Documents */}
                    <div>
                      <label
                        htmlFor="nib"
                        className="block text-sm font-medium text-gray-700"
                      >
                        NIB (Nomor Induk Berusaha)
                      </label>
                      <input
                        type="text"
                        name="nib"
                        id="nib"
                        value={formData.nib}
                        onChange={handleInputChange}
                        className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
                      />
                    </div>

                    <div>
                      <label
                        htmlFor="siup"
                        className="block text-sm font-medium text-gray-700"
                      >
                        SIUP
                      </label>
                      <input
                        type="text"
                        name="siup"
                        id="siup"
                        value={formData.siup}
                        onChange={handleInputChange}
                        className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
                      />
                    </div>

                    <div>
                      <label
                        htmlFor="tdp"
                        className="block text-sm font-medium text-gray-700"
                      >
                        TDP
                      </label>
                      <input
                        type="text"
                        name="tdp"
                        id="tdp"
                        value={formData.tdp}
                        onChange={handleInputChange}
                        className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
                      />
                    </div>

                    <div>
                      <label
                        htmlFor="npwp"
                        className="block text-sm font-medium text-gray-700"
                      >
                        NPWP Perusahaan
                      </label>
                      <input
                        type="text"
                        name="npwp"
                        id="npwp"
                        value={formData.npwp}
                        onChange={handleInputChange}
                        className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
                      />
                    </div>

                    {/* Business Scale Data */}
                    <div>
                      <label
                        htmlFor="employee_count"
                        className="block text-sm font-medium text-gray-700"
                      >
                        Jumlah Karyawan
                      </label>
                      <input
                        type="number"
                        name="employee_count"
                        id="employee_count"
                        min="0"
                        value={formData.employee_count || ""}
                        onChange={handleInputChange}
                        className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
                      />
                    </div>

                    <div>
                      <label
                        htmlFor="annual_revenue"
                        className="block text-sm font-medium text-gray-700"
                      >
                        Omzet Tahunan (Rp)
                      </label>
                      <input
                        type="number"
                        name="annual_revenue"
                        id="annual_revenue"
                        min="0"
                        value={formData.annual_revenue || ""}
                        onChange={handleInputChange}
                        className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
                        placeholder="contoh: 300000000 (300 juta)"
                      />
                    </div>
                  </div>

                  <div className="flex justify-end space-x-3 pt-6 border-t border-gray-200">
                    <button
                      type="button"
                      onClick={() => setShowCreateForm(false)}
                      className="inline-flex justify-center py-2 px-4 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                    >
                      Batal
                    </button>
                    <button
                      type="submit"
                      disabled={submitting}
                      className="inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                      {submitting ? "Menyimpan..." : "Daftar Perusahaan"}
                    </button>
                  </div>
                </form>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
