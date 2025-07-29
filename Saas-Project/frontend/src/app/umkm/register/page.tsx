"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import Link from "next/link";
import { EyeIcon, EyeSlashIcon, BuildingOfficeIcon } from "@heroicons/react/24/outline";

export default function UmkmRegisterPage() {
  const [formData, setFormData] = useState({
    companyName: "",
    businessType: "",
    address: "",
    phone: "",
    email: "",
    password: "",
    confirmPassword: "",
    ownerName: "",
    ownerPhone: "",
    ownerEmail: "",
    agreeToTerms: false
  });
  
  const [showPassword, setShowPassword] = useState(false);
  const [showConfirmPassword, setShowConfirmPassword] = useState(false);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");
  const [step, setStep] = useState(1); // 1: Company Info, 2: Owner Info, 3: Account Setup
  
  const router = useRouter();

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
    const { name, value, type } = e.target;
    setFormData(prev => ({
      ...prev,
      [name]: type === "checkbox" ? (e.target as HTMLInputElement).checked : value
    }));
  };

  const validateStep1 = () => {
    return formData.companyName && formData.businessType && formData.address && formData.phone && formData.email;
  };

  const validateStep2 = () => {
    return formData.ownerName && formData.ownerPhone && formData.ownerEmail;
  };

  const validateStep3 = () => {
    return formData.password && formData.confirmPassword && formData.password === formData.confirmPassword && formData.agreeToTerms;
  };

  const handleNext = () => {
    if (step === 1 && validateStep1()) {
      setStep(2);
    } else if (step === 2 && validateStep2()) {
      setStep(3);
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!validateStep3()) {
      setError("Mohon lengkapi semua field dan pastikan password cocok");
      return;
    }

    if (formData.password !== formData.confirmPassword) {
      setError("Password dan konfirmasi password tidak sama");
      return;
    }

    setError("");
    setLoading(true);

    try {
      // TODO: Replace with actual API call
      console.log("Registering UMKM:", formData);
      
      // Simulate API call
      await new Promise(resolve => setTimeout(resolve, 2000));
      
      // On success, redirect to login with success message
      router.push("/umkm/login?message=registration-success");
    } catch {
      setError("Registrasi gagal. Silakan coba lagi.");
    } finally {
      setLoading(false);
    }
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

  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 py-12 px-4 sm:px-6 lg:px-8">
      <div className="max-w-2xl mx-auto">
        {/* Header */}
        <div className="text-center mb-8">
          <div className="mx-auto h-20 w-20 bg-blue-600 rounded-full flex items-center justify-center mb-6">
            <BuildingOfficeIcon className="h-10 w-10 text-white" />
          </div>
          <h2 className="text-3xl font-bold text-gray-900">Daftar UMKM Baru</h2>
          <p className="mt-2 text-gray-600">
            Daftarkan UMKM Anda untuk mengakses layanan perizinan digital
          </p>
        </div>

        {/* Progress Indicator */}
        <div className="mb-8">
          <div className="flex items-center justify-center">
            <div className="flex items-center space-x-4">
              <div className={`flex items-center justify-center w-8 h-8 rounded-full ${step >= 1 ? 'bg-blue-600 text-white' : 'bg-gray-300 text-gray-600'}`}>
                1
              </div>
              <div className={`w-16 h-1 ${step >= 2 ? 'bg-blue-600' : 'bg-gray-300'}`}></div>
              <div className={`flex items-center justify-center w-8 h-8 rounded-full ${step >= 2 ? 'bg-blue-600 text-white' : 'bg-gray-300 text-gray-600'}`}>
                2
              </div>
              <div className={`w-16 h-1 ${step >= 3 ? 'bg-blue-600' : 'bg-gray-300'}`}></div>
              <div className={`flex items-center justify-center w-8 h-8 rounded-full ${step >= 3 ? 'bg-blue-600 text-white' : 'bg-gray-300 text-gray-600'}`}>
                3
              </div>
            </div>
          </div>
          <div className="flex justify-center mt-2">
            <div className="flex space-x-12 text-sm text-gray-600">
              <span className={step === 1 ? 'font-medium text-blue-600' : ''}>Info Perusahaan</span>
              <span className={step === 2 ? 'font-medium text-blue-600' : ''}>Info Pemilik</span>
              <span className={step === 3 ? 'font-medium text-blue-600' : ''}>Setup Akun</span>
            </div>
          </div>
        </div>

        {/* Registration Form */}
        <div className="bg-white p-8 rounded-xl shadow-lg">
          <form onSubmit={handleSubmit}>
            
            {/* Step 1: Company Information */}
            {step === 1 && (
              <div className="space-y-6">
                <h3 className="text-lg font-medium text-gray-900 mb-6">Informasi Perusahaan</h3>
                
                <div>
                  <label htmlFor="companyName" className="block text-sm font-medium text-gray-700">
                    Nama Perusahaan/UMKM *
                  </label>
                  <input
                    type="text"
                    id="companyName"
                    name="companyName"
                    required
                    value={formData.companyName}
                    onChange={handleInputChange}
                    className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                    placeholder="Contoh: Toko Berkah Jaya"
                  />
                </div>

                <div>
                  <label htmlFor="businessType" className="block text-sm font-medium text-gray-700">
                    Jenis Usaha *
                  </label>
                  <select
                    id="businessType"
                    name="businessType"
                    required
                    value={formData.businessType}
                    onChange={handleInputChange}
                    className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                  >
                    <option value="">Pilih jenis usaha</option>
                    {businessTypes.map(type => (
                      <option key={type} value={type}>{type}</option>
                    ))}
                  </select>
                </div>

                <div>
                  <label htmlFor="address" className="block text-sm font-medium text-gray-700">
                    Alamat Usaha *
                  </label>
                  <input
                    type="text"
                    id="address"
                    name="address"
                    required
                    value={formData.address}
                    onChange={handleInputChange}
                    className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                    placeholder="Alamat lengkap usaha"
                  />
                </div>

                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div>
                    <label htmlFor="phone" className="block text-sm font-medium text-gray-700">
                      Telepon Perusahaan *
                    </label>
                    <input
                      type="tel"
                      id="phone"
                      name="phone"
                      required
                      value={formData.phone}
                      onChange={handleInputChange}
                      className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                      placeholder="021-12345678"
                    />
                  </div>

                  <div>
                    <label htmlFor="email" className="block text-sm font-medium text-gray-700">
                      Email Perusahaan *
                    </label>
                    <input
                      type="email"
                      id="email"
                      name="email"
                      required
                      value={formData.email}
                      onChange={handleInputChange}
                      className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                      placeholder="info@perusahaan.com"
                    />
                  </div>
                </div>
              </div>
            )}

            {/* Step 2: Owner Information */}
            {step === 2 && (
              <div className="space-y-6">
                <h3 className="text-lg font-medium text-gray-900 mb-6">Informasi Pemilik</h3>
                
                <div>
                  <label htmlFor="ownerName" className="block text-sm font-medium text-gray-700">
                    Nama Lengkap Pemilik *
                  </label>
                  <input
                    type="text"
                    id="ownerName"
                    name="ownerName"
                    required
                    value={formData.ownerName}
                    onChange={handleInputChange}
                    className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                    placeholder="Nama sesuai KTP"
                  />
                </div>

                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div>
                    <label htmlFor="ownerPhone" className="block text-sm font-medium text-gray-700">
                      No. HP Pemilik *
                    </label>
                    <input
                      type="tel"
                      id="ownerPhone"
                      name="ownerPhone"
                      required
                      value={formData.ownerPhone}
                      onChange={handleInputChange}
                      className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                      placeholder="08123456789"
                    />
                  </div>

                  <div>
                    <label htmlFor="ownerEmail" className="block text-sm font-medium text-gray-700">
                      Email Pribadi Pemilik *
                    </label>
                    <input
                      type="email"
                      id="ownerEmail"
                      name="ownerEmail"
                      required
                      value={formData.ownerEmail}
                      onChange={handleInputChange}
                      className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                      placeholder="pemilik@email.com"
                    />
                  </div>
                </div>
              </div>
            )}

            {/* Step 3: Account Setup */}
            {step === 3 && (
              <div className="space-y-6">
                <h3 className="text-lg font-medium text-gray-900 mb-6">Setup Akun</h3>
                
                <div>
                  <label htmlFor="password" className="block text-sm font-medium text-gray-700">
                    Password *
                  </label>
                  <div className="mt-1 relative">
                    <input
                      type={showPassword ? "text" : "password"}
                      id="password"
                      name="password"
                      required
                      value={formData.password}
                      onChange={handleInputChange}
                      className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 pr-10"
                      placeholder="Minimal 8 karakter"
                    />
                    <button
                      type="button"
                      className="absolute inset-y-0 right-0 pr-3 flex items-center"
                      onClick={() => setShowPassword(!showPassword)}
                    >
                      {showPassword ? (
                        <EyeSlashIcon className="h-5 w-5 text-gray-400" />
                      ) : (
                        <EyeIcon className="h-5 w-5 text-gray-400" />
                      )}
                    </button>
                  </div>
                </div>

                <div>
                  <label htmlFor="confirmPassword" className="block text-sm font-medium text-gray-700">
                    Konfirmasi Password *
                  </label>
                  <div className="mt-1 relative">
                    <input
                      type={showConfirmPassword ? "text" : "password"}
                      id="confirmPassword"
                      name="confirmPassword"
                      required
                      value={formData.confirmPassword}
                      onChange={handleInputChange}
                      className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 pr-10"
                      placeholder="Ulangi password"
                    />
                    <button
                      type="button"
                      className="absolute inset-y-0 right-0 pr-3 flex items-center"
                      onClick={() => setShowConfirmPassword(!showConfirmPassword)}
                    >
                      {showConfirmPassword ? (
                        <EyeSlashIcon className="h-5 w-5 text-gray-400" />
                      ) : (
                        <EyeIcon className="h-5 w-5 text-gray-400" />
                      )}
                    </button>
                  </div>
                </div>

                <div className="flex items-start">
                  <input
                    id="agreeToTerms"
                    name="agreeToTerms"
                    type="checkbox"
                    required
                    checked={formData.agreeToTerms}
                    onChange={handleInputChange}
                    className="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded mt-0.5"
                  />
                  <div className="ml-3 text-sm">
                    <label htmlFor="agreeToTerms" className="text-gray-700">
                      Saya setuju dengan{" "}
                      <Link href="/terms" className="text-blue-600 hover:text-blue-500 underline">
                        Syarat dan Ketentuan
                      </Link>{" "}
                      serta{" "}
                      <Link href="/privacy" className="text-blue-600 hover:text-blue-500 underline">
                        Kebijakan Privasi
                      </Link>
                    </label>
                  </div>
                </div>
              </div>
            )}

            {error && (
              <div className="bg-red-50 border border-red-300 text-red-700 px-4 py-3 rounded-md text-sm">
                {error}
              </div>
            )}

            {/* Navigation Buttons */}
            <div className="flex justify-between mt-8">
              {step > 1 && (
                <button
                  type="button"
                  onClick={() => setStep(step - 1)}
                  className="px-6 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 hover:bg-gray-50"
                >
                  Kembali
                </button>
              )}
              
              <div className="flex-1 flex justify-end">
                {step < 3 ? (
                  <button
                    type="button"
                    onClick={handleNext}
                    disabled={
                      (step === 1 && !validateStep1()) ||
                      (step === 2 && !validateStep2())
                    }
                    className="px-6 py-2 bg-blue-600 text-white rounded-md text-sm font-medium hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
                  >
                    Lanjutkan
                  </button>
                ) : (
                  <button
                    type="submit"
                    disabled={loading || !validateStep3()}
                    className="px-6 py-2 bg-blue-600 text-white rounded-md text-sm font-medium hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
                  >
                    {loading ? (
                      <div className="flex items-center">
                        <svg className="animate-spin -ml-1 mr-2 h-4 w-4 text-white" fill="none" viewBox="0 0 24 24">
                          <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                          <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                        </svg>
                        Mendaftar...
                      </div>
                    ) : (
                      "Daftar Sekarang"
                    )}
                  </button>
                )}
              </div>
            </div>
          </form>

          <div className="mt-6 text-center">
            <p className="text-sm text-gray-600">
              Sudah memiliki akun?{" "}
              <Link href="/umkm/login" className="font-medium text-blue-600 hover:text-blue-500">
                Masuk di sini
              </Link>
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}
