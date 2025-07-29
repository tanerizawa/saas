"use client";

import { useAuth } from "@/contexts/AuthContext";
import { useState, useEffect } from "react";
import { useRouter, useSearchParams } from "next/navigation";
import { 
  DocumentTextIcon,
  ArrowLeftIcon,
  CloudArrowUpIcon,
  CheckIcon,
  XMarkIcon,
  ExclamationTriangleIcon
} from "@heroicons/react/24/outline";

const LICENSE_TYPES = {
  siup: {
    name: "SIUP (Surat Izin Usaha Perdagangan)",
    description: "Izin usaha untuk kegiatan perdagangan barang dan jasa",
    requirements: [
      { id: "ktp", name: "KTP Pemilik", required: true },
      { id: "akta", name: "Akta Pendirian Perusahaan", required: true },
      { id: "npwp", name: "NPWP Perusahaan", required: true },
      { id: "domisili", name: "Surat Keterangan Domisili", required: true },
      { id: "foto", name: "Foto Perusahaan", required: true }
    ],
    processing_time: "7-14 hari kerja",
    fee: "Rp 500.000"
  },
  tdp: {
    name: "TDP (Tanda Daftar Perusahaan)",
    description: "Tanda daftar perusahaan di dinas perdagangan setempat",
    requirements: [
      { id: "siup", name: "SIUP yang berlaku", required: true },
      { id: "akta", name: "Akta Pendirian Perusahaan", required: true },
      { id: "npwp", name: "NPWP Perusahaan", required: true },
      { id: "domisili", name: "Surat Keterangan Domisili", required: true }
    ],
    processing_time: "5-10 hari kerja",
    fee: "Rp 300.000"
  },
  npwp: {
    name: "NPWP Badan",
    description: "Nomor Pokok Wajib Pajak untuk badan usaha",
    requirements: [
      { id: "akta", name: "Akta Pendirian Perusahaan", required: true },
      { id: "ktp", name: "KTP Pengurus", required: true },
      { id: "domisili", name: "Surat Keterangan Domisili", required: true },
      { id: "pernyataan", name: "Surat Pernyataan Bermaterai", required: true }
    ],
    processing_time: "3-7 hari kerja",
    fee: "Gratis"
  },
  iumk: {
    name: "IUMK (Izin Usaha Mikro Kecil)",
    description: "Izin usaha untuk UMKM dengan omzet di bawah Rp 300 juta",
    requirements: [
      { id: "ktp", name: "KTP Pemilik", required: true },
      { id: "domisili", name: "Surat Keterangan Domisili", required: true },
      { id: "foto", name: "Pas Foto 3x4", required: true },
      { id: "pernyataan", name: "Surat Pernyataan tidak memiliki SIUP", required: true }
    ],
    processing_time: "1-3 hari kerja",
    fee: "Gratis"
  }
};

interface UploadedFile {
  id: string;
  name: string;
  size: number;
  type: string;
  url?: string;
}

export default function UmkmLicenseApplyPage() {
  const { isUmkmOwner } = useAuth();
  const router = useRouter();
  const searchParams = useSearchParams();
  const licenseType = searchParams.get('type') as keyof typeof LICENSE_TYPES;
  
  const [formData, setFormData] = useState({
    business_description: "",
    capital_amount: "",
    employee_count: "",
    business_location: "",
    notes: ""
  });
  
  const [uploadedFiles, setUploadedFiles] = useState<{[key: string]: UploadedFile}>({});
  const [loading, setLoading] = useState(false);
  const [step, setStep] = useState(1); // 1: Form, 2: Documents, 3: Review
  const [errors, setErrors] = useState<{[key: string]: string}>({});

  useEffect(() => {
    if (!licenseType || !LICENSE_TYPES[licenseType]) {
      router.push('/umkm/licenses');
    }
  }, [licenseType, router]);

  const license = licenseType ? LICENSE_TYPES[licenseType] : null;

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    const { name, value } = e.target;
    setFormData(prev => ({ ...prev, [name]: value }));
    if (errors[name]) {
      setErrors(prev => ({ ...prev, [name]: "" }));
    }
  };

  const handleFileUpload = (requirementId: string, file: File) => {
    // Simulate file upload
    const uploadedFile: UploadedFile = {
      id: Date.now().toString(),
      name: file.name,
      size: file.size,
      type: file.type,
      url: URL.createObjectURL(file)
    };
    
    setUploadedFiles(prev => ({
      ...prev,
      [requirementId]: uploadedFile
    }));
  };

  const removeFile = (requirementId: string) => {
    setUploadedFiles(prev => {
      const updated = { ...prev };
      delete updated[requirementId];
      return updated;
    });
  };

  const validateStep1 = () => {
    const newErrors: {[key: string]: string} = {};
    
    if (!formData.business_description.trim()) {
      newErrors.business_description = "Deskripsi usaha wajib diisi";
    }
    
    if (!formData.capital_amount.trim()) {
      newErrors.capital_amount = "Modal usaha wajib diisi";
    }
    
    if (!formData.employee_count.trim()) {
      newErrors.employee_count = "Jumlah karyawan wajib diisi";
    }
    
    if (!formData.business_location.trim()) {
      newErrors.business_location = "Lokasi usaha wajib diisi";
    }
    
    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const validateStep2 = () => {
    if (!license) return false;
    
    const newErrors: {[key: string]: string} = {};
    
    license.requirements.forEach(req => {
      if (req.required && !uploadedFiles[req.id]) {
        newErrors[req.id] = `${req.name} wajib diunggah`;
      }
    });
    
    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleNext = () => {
    if (step === 1 && validateStep1()) {
      setStep(2);
    } else if (step === 2 && validateStep2()) {
      setStep(3);
    }
  };

  const handleSubmit = async () => {
    setLoading(true);
    try {
      // TODO: Replace with actual API call
      const applicationData = {
        license_type: license?.name,
        ...formData,
        documents: Object.keys(uploadedFiles).map(key => ({
          requirement_id: key,
          file: uploadedFiles[key]
        }))
      };
      
      console.log('Submitting application:', applicationData);
      
      // Simulate API call
      await new Promise(resolve => setTimeout(resolve, 2000));
      
      // Redirect to licenses page with success message
      router.push('/umkm/licenses?success=application-submitted');
    } catch (error) {
      console.error('Error submitting application:', error);
    } finally {
      setLoading(false);
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

  if (!license) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="animate-spin rounded-full h-32 w-32 border-b-2 border-blue-600"></div>
      </div>
    );
  }

  const FileUploadComponent = ({ 
    requirement, 
    uploadedFile, 
    error 
  }: { 
    requirement: { id: string; name: string; required: boolean };
    uploadedFile?: UploadedFile;
    error?: string;
  }) => (
    <div className="border border-gray-300 rounded-lg p-4">
      <div className="flex items-center justify-between mb-2">
        <h4 className="font-medium text-gray-900">{requirement.name}</h4>
        {requirement.required && (
          <span className="text-red-500 text-sm">*wajib</span>
        )}
      </div>
      
      {uploadedFile ? (
        <div className="flex items-center justify-between bg-green-50 border border-green-200 rounded p-3">
          <div className="flex items-center">
            <CheckIcon className="h-5 w-5 text-green-600 mr-2" />
            <div>
              <p className="text-sm font-medium text-green-900">{uploadedFile.name}</p>
              <p className="text-xs text-green-600">
                {(uploadedFile.size / 1024 / 1024).toFixed(2)} MB
              </p>
            </div>
          </div>
          <button
            onClick={() => removeFile(requirement.id)}
            className="text-red-600 hover:text-red-800"
          >
            <XMarkIcon className="h-5 w-5" />
          </button>
        </div>
      ) : (
        <div className="border-2 border-dashed border-gray-300 rounded-lg p-4 text-center hover:border-gray-400 transition-colors">
          <input
            type="file"
            id={`file-${requirement.id}`}
            className="hidden"
            accept=".pdf,.jpg,.jpeg,.png"
            onChange={(e) => {
              const file = e.target.files?.[0];
              if (file) {
                handleFileUpload(requirement.id, file);
              }
            }}
          />
          <label
            htmlFor={`file-${requirement.id}`}
            className="cursor-pointer flex flex-col items-center"
          >
            <CloudArrowUpIcon className="h-8 w-8 text-gray-400 mb-2" />
            <p className="text-sm text-gray-600">Klik untuk upload file</p>
            <p className="text-xs text-gray-500 mt-1">PDF, JPG, PNG (max 5MB)</p>
          </label>
        </div>
      )}
      
      {error && (
        <p className="text-red-600 text-sm mt-2">{error}</p>
      )}
    </div>
  );

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Header */}
      <div className="bg-white shadow-sm border-b">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="py-6">
            <div className="flex items-center">
              <button
                onClick={() => router.back()}
                className="mr-4 p-2 text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100"
              >
                <ArrowLeftIcon className="h-6 w-6" />
              </button>
              <DocumentTextIcon className="h-8 w-8 text-blue-600 mr-3" />
              <div>
                <h1 className="text-2xl font-bold text-gray-900">Pengajuan {license.name}</h1>
                <p className="text-sm text-gray-500">{license.description}</p>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Progress Indicator */}
      <div className="bg-white border-b">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-4">
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
              <span className={step === 1 ? 'font-medium text-blue-600' : ''}>Data Usaha</span>
              <span className={step === 2 ? 'font-medium text-blue-600' : ''}>Upload Dokumen</span>
              <span className={step === 3 ? 'font-medium text-blue-600' : ''}>Review & Submit</span>
            </div>
          </div>
        </div>
      </div>

      <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="bg-white rounded-lg shadow">
          <div className="p-6">
            
            {/* Step 1: Business Information */}
            {step === 1 && (
              <div className="space-y-6">
                <h3 className="text-lg font-medium text-gray-900 mb-6">Informasi Data Usaha</h3>
                
                <div>
                  <label htmlFor="business_description" className="block text-sm font-medium text-gray-700 mb-2">
                    Deskripsi Usaha *
                  </label>
                  <textarea
                    id="business_description"
                    name="business_description"
                    rows={4}
                    value={formData.business_description}
                    onChange={handleInputChange}
                    className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                    placeholder="Jelaskan jenis usaha dan kegiatan yang dilakukan..."
                  />
                  {errors.business_description && (
                    <p className="text-red-600 text-sm mt-1">{errors.business_description}</p>
                  )}
                </div>

                <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                  <div>
                    <label htmlFor="capital_amount" className="block text-sm font-medium text-gray-700 mb-2">
                      Modal Usaha *
                    </label>
                    <input
                      type="text"
                      id="capital_amount"
                      name="capital_amount"
                      value={formData.capital_amount}
                      onChange={handleInputChange}
                      className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                      placeholder="Contoh: Rp 50.000.000"
                    />
                    {errors.capital_amount && (
                      <p className="text-red-600 text-sm mt-1">{errors.capital_amount}</p>
                    )}
                  </div>

                  <div>
                    <label htmlFor="employee_count" className="block text-sm font-medium text-gray-700 mb-2">
                      Jumlah Karyawan *
                    </label>
                    <input
                      type="number"
                      id="employee_count"
                      name="employee_count"
                      value={formData.employee_count}
                      onChange={handleInputChange}
                      className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                      placeholder="Contoh: 5"
                    />
                    {errors.employee_count && (
                      <p className="text-red-600 text-sm mt-1">{errors.employee_count}</p>
                    )}
                  </div>
                </div>

                <div>
                  <label htmlFor="business_location" className="block text-sm font-medium text-gray-700 mb-2">
                    Lokasi Usaha *
                  </label>
                  <textarea
                    id="business_location"
                    name="business_location"
                    rows={3}
                    value={formData.business_location}
                    onChange={handleInputChange}
                    className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                    placeholder="Alamat lengkap lokasi usaha..."
                  />
                  {errors.business_location && (
                    <p className="text-red-600 text-sm mt-1">{errors.business_location}</p>
                  )}
                </div>

                <div>
                  <label htmlFor="notes" className="block text-sm font-medium text-gray-700 mb-2">
                    Catatan Tambahan
                  </label>
                  <textarea
                    id="notes"
                    name="notes"
                    rows={3}
                    value={formData.notes}
                    onChange={handleInputChange}
                    className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                    placeholder="Informasi tambahan yang perlu disampaikan..."
                  />
                </div>
              </div>
            )}

            {/* Step 2: Document Upload */}
            {step === 2 && (
              <div className="space-y-6">
                <h3 className="text-lg font-medium text-gray-900 mb-6">Upload Dokumen Persyaratan</h3>
                
                <div className="space-y-4">
                  {license.requirements.map((requirement) => (
                    <FileUploadComponent
                      key={requirement.id}
                      requirement={requirement}
                      uploadedFile={uploadedFiles[requirement.id]}
                      error={errors[requirement.id]}
                    />
                  ))}
                </div>

                <div className="bg-blue-50 border border-blue-200 rounded p-4">
                  <h4 className="font-medium text-blue-900 mb-2">Persyaratan File:</h4>
                  <ul className="text-sm text-blue-700 space-y-1">
                    <li>• Format file: PDF, JPG, JPEG, PNG</li>
                    <li>• Ukuran maksimal: 5MB per file</li>
                    <li>• Pastikan dokumen jelas dan mudah dibaca</li>
                    <li>• File yang diunggah harus asli atau fotokopi yang jelas</li>
                  </ul>
                </div>
              </div>
            )}

            {/* Step 3: Review */}
            {step === 3 && (
              <div className="space-y-6">
                <h3 className="text-lg font-medium text-gray-900 mb-6">Review Pengajuan</h3>
                
                {/* License Info */}
                <div className="bg-gray-50 rounded-lg p-4">
                  <h4 className="font-medium text-gray-900 mb-2">Jenis Perizinan</h4>
                  <p className="text-gray-700">{license.name}</p>
                  <p className="text-sm text-gray-600 mt-1">{license.description}</p>
                  <div className="flex justify-between items-center mt-3 text-sm">
                    <span>Waktu proses: <strong>{license.processing_time}</strong></span>
                    <span>Biaya: <strong>{license.fee}</strong></span>
                  </div>
                </div>

                {/* Business Data */}
                <div className="bg-gray-50 rounded-lg p-4">
                  <h4 className="font-medium text-gray-900 mb-3">Data Usaha</h4>
                  <div className="space-y-2 text-sm">
                    <div><strong>Deskripsi Usaha:</strong> {formData.business_description}</div>
                    <div><strong>Modal Usaha:</strong> {formData.capital_amount}</div>
                    <div><strong>Jumlah Karyawan:</strong> {formData.employee_count}</div>
                    <div><strong>Lokasi Usaha:</strong> {formData.business_location}</div>
                    {formData.notes && <div><strong>Catatan:</strong> {formData.notes}</div>}
                  </div>
                </div>

                {/* Documents */}
                <div className="bg-gray-50 rounded-lg p-4">
                  <h4 className="font-medium text-gray-900 mb-3">Dokumen yang Diunggah</h4>
                  <div className="space-y-2">
                    {license.requirements.map((req) => (
                      <div key={req.id} className="flex items-center justify-between text-sm">
                        <span>{req.name}</span>
                        {uploadedFiles[req.id] ? (
                          <span className="text-green-600 flex items-center">
                            <CheckIcon className="h-4 w-4 mr-1" />
                            {uploadedFiles[req.id].name}
                          </span>
                        ) : (
                          <span className="text-red-600">Belum diunggah</span>
                        )}
                      </div>
                    ))}
                  </div>
                </div>

                <div className="bg-yellow-50 border border-yellow-200 rounded p-4">
                  <p className="text-yellow-800 text-sm">
                    <strong>Perhatian:</strong> Setelah pengajuan disubmit, Anda tidak dapat mengubah data. 
                    Pastikan semua informasi sudah benar sebelum melanjutkan.
                  </p>
                </div>
              </div>
            )}

            {/* Navigation Buttons */}
            <div className="flex justify-between mt-8 pt-6 border-t border-gray-200">
              {step > 1 && (
                <button
                  onClick={() => setStep(step - 1)}
                  className="px-6 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 hover:bg-gray-50"
                >
                  Kembali
                </button>
              )}
              
              <div className="flex-1 flex justify-end">
                {step < 3 ? (
                  <button
                    onClick={handleNext}
                    className="px-6 py-2 bg-blue-600 text-white rounded-md text-sm font-medium hover:bg-blue-700"
                  >
                    Lanjutkan
                  </button>
                ) : (
                  <button
                    onClick={handleSubmit}
                    disabled={loading}
                    className="px-6 py-2 bg-green-600 text-white rounded-md text-sm font-medium hover:bg-green-700 disabled:opacity-50 disabled:cursor-not-allowed"
                  >
                    {loading ? (
                      <div className="flex items-center">
                        <svg className="animate-spin -ml-1 mr-2 h-4 w-4 text-white" fill="none" viewBox="0 0 24 24">
                          <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                          <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                        </svg>
                        Memproses...
                      </div>
                    ) : (
                      "Submit Pengajuan"
                    )}
                  </button>
                )}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
