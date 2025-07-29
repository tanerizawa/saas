"use client";

import { useAuth } from "@/contexts/AuthContext";
import { useState, useEffect } from "react";
import { 
  DocumentTextIcon,
  CloudArrowUpIcon,
  TrashIcon,
  EyeIcon,
  ArrowDownTrayIcon,
  FolderIcon,
  MagnifyingGlassIcon,
  FunnelIcon,
  PlusIcon
} from "@heroicons/react/24/outline";

interface Document {
  id: string;
  name: string;
  type: string;
  size: number;
  category: string;
  upload_date: string;
  license_application_id?: string;
  url?: string;
  status: "active" | "archived";
}

const DOCUMENT_CATEGORIES = [
  { id: "identity", name: "Dokumen Identitas", color: "blue" },
  { id: "business", name: "Dokumen Usaha", color: "green" },
  { id: "financial", name: "Dokumen Keuangan", color: "yellow" },
  { id: "legal", name: "Dokumen Legal", color: "purple" },
  { id: "other", name: "Lainnya", color: "gray" }
];

export default function UmkmDocumentsPage() {
  const { isUmkmOwner } = useAuth();
  const [documents, setDocuments] = useState<Document[]>([]);
  const [loading, setLoading] = useState(true);
  const [searchTerm, setSearchTerm] = useState("");
  const [selectedCategory, setSelectedCategory] = useState("all");
  const [showUploadModal, setShowUploadModal] = useState(false);
  const [uploadForm, setUploadForm] = useState({
    category: "identity",
    description: ""
  });

  useEffect(() => {
    // Mock data - replace with actual API call
    setTimeout(() => {
      setDocuments([
        {
          id: "1",
          name: "KTP-Pemilik.pdf",
          type: "application/pdf",
          size: 2048000,
          category: "identity",
          upload_date: "2024-01-15T10:30:00Z",
          license_application_id: "1",
          status: "active"
        },
        {
          id: "2",
          name: "Akta-Pendirian.pdf",
          type: "application/pdf",
          size: 5120000,
          category: "business",
          upload_date: "2024-01-14T09:15:00Z",
          license_application_id: "1",
          status: "active"
        },
        {
          id: "3",
          name: "NPWP-Perusahaan.pdf",
          type: "application/pdf",
          size: 1024000,
          category: "business",
          upload_date: "2024-01-13T16:45:00Z",
          status: "active"
        },
        {
          id: "4",
          name: "Laporan-Keuangan-2023.pdf",
          type: "application/pdf",
          size: 3072000,
          category: "financial",
          upload_date: "2024-01-12T11:20:00Z",
          status: "active"
        },
        {
          id: "5",
          name: "Surat-Domisili.jpg",
          type: "image/jpeg",
          size: 1536000,
          category: "legal",
          upload_date: "2024-01-11T14:10:00Z",
          status: "active"
        }
      ]);
      setLoading(false);
    }, 1000);
  }, []);

  const filteredDocuments = documents.filter(doc => {
    const matchesSearch = doc.name.toLowerCase().includes(searchTerm.toLowerCase());
    const matchesCategory = selectedCategory === "all" || doc.category === selectedCategory;
    return matchesSearch && matchesCategory && doc.status === "active";
  });

  const formatFileSize = (bytes: number) => {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  };

  const getFileIcon = (type: string) => {
    if (type.includes('pdf')) return <DocumentTextIcon className="h-8 w-8 text-red-500" />;
    if (type.includes('image')) return <DocumentTextIcon className="h-8 w-8 text-blue-500" />;
    return <DocumentTextIcon className="h-8 w-8 text-gray-500" />;
  };

  const getCategoryColor = (category: string) => {
    const cat = DOCUMENT_CATEGORIES.find(c => c.id === category);
    const colors = {
      blue: "bg-blue-100 text-blue-800 border-blue-300",
      green: "bg-green-100 text-green-800 border-green-300",
      yellow: "bg-yellow-100 text-yellow-800 border-yellow-300",
      purple: "bg-purple-100 text-purple-800 border-purple-300",
      gray: "bg-gray-100 text-gray-800 border-gray-300"
    };
    return colors[cat?.color as keyof typeof colors] || colors.gray;
  };

  const getCategoryName = (category: string) => {
    return DOCUMENT_CATEGORIES.find(c => c.id === category)?.name || "Lainnya";
  };

  const handleFileUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files;
    if (!files) return;

    Array.from(files).forEach(file => {
      const newDoc: Document = {
        id: Date.now().toString() + Math.random().toString(36).substr(2, 9),
        name: file.name,
        type: file.type,
        size: file.size,
        category: uploadForm.category,
        upload_date: new Date().toISOString(),
        status: "active"
      };

      setDocuments(prev => [newDoc, ...prev]);
    });

    setShowUploadModal(false);
    setUploadForm({ category: "identity", description: "" });
  };

  const handleDeleteDocument = (docId: string) => {
    if (confirm("Apakah Anda yakin ingin menghapus dokumen ini?")) {
      setDocuments(prev => prev.map(doc => 
        doc.id === docId ? { ...doc, status: "archived" as const } : doc
      ));
    }
  };

  const handleDownload = (doc: Document) => {
    // Simulate download
    console.log(`Downloading: ${doc.name}`);
    alert(`Download dimulai: ${doc.name}`);
  };

  const handleView = (doc: Document) => {
    // Simulate view
    console.log(`Viewing: ${doc.name}`);
    alert(`Membuka dokumen: ${doc.name}`);
  };

  if (!isUmkmOwner()) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-center">
          <DocumentTextIcon className="mx-auto h-16 w-16 text-red-500 mb-4" />
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
          <div className="py-6">
            <div className="flex items-center justify-between">
              <div className="flex items-center">
                <FolderIcon className="h-8 w-8 text-blue-600 mr-3" />
                <div>
                  <h1 className="text-2xl font-bold text-gray-900">Manajemen Dokumen</h1>
                  <p className="text-sm text-gray-500">Kelola semua dokumen perizinan dan usaha Anda</p>
                </div>
              </div>
              <button
                onClick={() => setShowUploadModal(true)}
                className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700"
              >
                <PlusIcon className="h-4 w-4 mr-2" />
                Upload Dokumen
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
                <p className="text-sm font-medium text-gray-500">Total Dokumen</p>
                <p className="text-2xl font-semibold text-gray-900">{documents.filter(d => d.status === 'active').length}</p>
              </div>
            </div>
          </div>

          {DOCUMENT_CATEGORIES.slice(0, 3).map((category) => (
            <div key={category.id} className="bg-white rounded-lg shadow p-6">
              <div className="flex items-center">
                <FolderIcon className="h-8 w-8 text-blue-600" />
                <div className="ml-4">
                  <p className="text-sm font-medium text-gray-500">{category.name}</p>
                  <p className="text-2xl font-semibold text-gray-900">
                    {documents.filter(d => d.category === category.id && d.status === 'active').length}
                  </p>
                </div>
              </div>
            </div>
          ))}
        </div>

        {/* Search and Filter */}
        <div className="bg-white rounded-lg shadow mb-6">
          <div className="p-6">
            <div className="flex flex-col sm:flex-row gap-4">
              <div className="flex-1">
                <div className="relative">
                  <MagnifyingGlassIcon className="absolute left-3 top-1/2 transform -translate-y-1/2 h-5 w-5 text-gray-400" />
                  <input
                    type="text"
                    placeholder="Cari dokumen..."
                    value={searchTerm}
                    onChange={(e) => setSearchTerm(e.target.value)}
                    className="block w-full pl-10 pr-3 py-2 border border-gray-300 rounded-md leading-5 bg-white placeholder-gray-500 focus:outline-none focus:placeholder-gray-400 focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
                  />
                </div>
              </div>
              <div className="sm:w-64">
                <div className="relative">
                  <FunnelIcon className="absolute left-3 top-1/2 transform -translate-y-1/2 h-5 w-5 text-gray-400" />
                  <select
                    value={selectedCategory}
                    onChange={(e) => setSelectedCategory(e.target.value)}
                    className="block w-full pl-10 pr-3 py-2 border border-gray-300 rounded-md leading-5 bg-white focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
                  >
                    <option value="all">Semua Kategori</option>
                    {DOCUMENT_CATEGORIES.map((category) => (
                      <option key={category.id} value={category.id}>{category.name}</option>
                    ))}
                  </select>
                </div>
              </div>
            </div>
          </div>
        </div>

        {/* Documents List */}
        <div className="bg-white rounded-lg shadow">
          <div className="px-6 py-4 border-b border-gray-200">
            <h2 className="text-lg font-medium text-gray-900">
              Dokumen ({filteredDocuments.length})
            </h2>
          </div>
          
          {filteredDocuments.length > 0 ? (
            <div className="divide-y divide-gray-200">
              {filteredDocuments.map((doc) => (
                <div key={doc.id} className="px-6 py-4 hover:bg-gray-50">
                  <div className="flex items-center justify-between">
                    <div className="flex items-center space-x-4">
                      {getFileIcon(doc.type)}
                      <div className="flex-1">
                        <h3 className="text-sm font-medium text-gray-900">{doc.name}</h3>
                        <div className="flex items-center space-x-4 mt-1">
                          <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border ${getCategoryColor(doc.category)}`}>
                            {getCategoryName(doc.category)}
                          </span>
                          <p className="text-sm text-gray-500">
                            {formatFileSize(doc.size)}
                          </p>
                          <p className="text-sm text-gray-500">
                            {new Date(doc.upload_date).toLocaleDateString('id-ID')}
                          </p>
                          {doc.license_application_id && (
                            <p className="text-sm text-blue-600">
                              Terkait pengajuan perizinan
                            </p>
                          )}
                        </div>
                      </div>
                    </div>
                    
                    <div className="flex items-center space-x-2">
                      <button
                        onClick={() => handleView(doc)}
                        className="p-2 text-gray-400 hover:text-blue-600 rounded-full hover:bg-blue-50"
                        title="Lihat dokumen"
                      >
                        <EyeIcon className="h-5 w-5" />
                      </button>
                      <button
                        onClick={() => handleDownload(doc)}
                        className="p-2 text-gray-400 hover:text-green-600 rounded-full hover:bg-green-50"
                        title="Download dokumen"
                      >
                        <ArrowDownTrayIcon className="h-5 w-5" />
                      </button>
                      <button
                        onClick={() => handleDeleteDocument(doc.id)}
                        className="p-2 text-gray-400 hover:text-red-600 rounded-full hover:bg-red-50"
                        title="Hapus dokumen"
                      >
                        <TrashIcon className="h-5 w-5" />
                      </button>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          ) : (
            <div className="px-6 py-12 text-center">
              <FolderIcon className="mx-auto h-12 w-12 text-gray-400" />
              <h3 className="mt-2 text-sm font-medium text-gray-900">
                {searchTerm || selectedCategory !== "all" ? "Tidak ada dokumen yang sesuai" : "Belum ada dokumen"}
              </h3>
              <p className="mt-1 text-sm text-gray-500">
                {searchTerm || selectedCategory !== "all" 
                  ? "Coba ubah kata kunci pencarian atau filter kategori."
                  : "Mulai dengan mengupload dokumen pertama Anda."
                }
              </p>
              {(!searchTerm && selectedCategory === "all") && (
                <div className="mt-6">
                  <button
                    onClick={() => setShowUploadModal(true)}
                    className="inline-flex items-center px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700"
                  >
                    <PlusIcon className="h-4 w-4 mr-2" />
                    Upload Dokumen Pertama
                  </button>
                </div>
              )}
            </div>
          )}
        </div>
      </div>

      {/* Upload Modal */}
      {showUploadModal && (
        <div className="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50">
          <div className="relative top-20 mx-auto p-5 border w-96 shadow-lg rounded-md bg-white">
            <div className="mt-3">
              <h3 className="text-lg font-medium text-gray-900 mb-4">Upload Dokumen Baru</h3>
              
              <div className="space-y-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Kategori Dokumen
                  </label>
                  <select
                    value={uploadForm.category}
                    onChange={(e) => setUploadForm(prev => ({ ...prev, category: e.target.value }))}
                    className="block w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                  >
                    {DOCUMENT_CATEGORIES.map((category) => (
                      <option key={category.id} value={category.id}>{category.name}</option>
                    ))}
                  </select>
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Deskripsi (Optional)
                  </label>
                  <textarea
                    value={uploadForm.description}
                    onChange={(e) => setUploadForm(prev => ({ ...prev, description: e.target.value }))}
                    rows={3}
                    className="block w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                    placeholder="Deskripsi dokumen..."
                  />
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Pilih File
                  </label>
                  <div className="border-2 border-dashed border-gray-300 rounded-lg p-6 text-center hover:border-gray-400 transition-colors">
                    <input
                      type="file"
                      id="file-upload"
                      className="hidden"
                      multiple
                      accept=".pdf,.jpg,.jpeg,.png"
                      onChange={handleFileUpload}
                    />
                    <label htmlFor="file-upload" className="cursor-pointer">
                      <CloudArrowUpIcon className="mx-auto h-12 w-12 text-gray-400" />
                      <p className="mt-2 text-sm text-gray-600">Klik untuk pilih file</p>
                      <p className="text-xs text-gray-500 mt-1">PDF, JPG, PNG (max 10MB)</p>
                    </label>
                  </div>
                </div>
              </div>

              <div className="flex justify-end space-x-3 mt-6">
                <button
                  onClick={() => setShowUploadModal(false)}
                  className="px-4 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 hover:bg-gray-50"
                >
                  Batal
                </button>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
