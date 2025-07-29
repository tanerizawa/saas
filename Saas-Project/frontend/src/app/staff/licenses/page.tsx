"use client";

import { useAuth } from "@/contexts/AuthContext";
import { useState, useEffect } from "react";
import {
  ShieldCheckIcon,
  ClockIcon,
  CheckCircleIcon,
  XCircleIcon,
  ExclamationTriangleIcon,
  MagnifyingGlassIcon,
  EyeIcon,
  DocumentTextIcon,
  UserIcon,
  CalendarIcon,
  BuildingOfficeIcon
} from "@heroicons/react/24/outline";

interface LicenseApplication {
  id: string;
  licenseType: "SIUP" | "TDP" | "HO" | "NPWP" | "NIB";
  company: {
    id: string;
    name: string;
    owner: string;
    email: string;
  };
  applicant: {
    name: string;
    phone: string;
    position: string;
  };
  status: "pending_review" | "under_review" | "approved" | "rejected" | "needs_revision";
  priority: "low" | "medium" | "high" | "urgent";
  submittedAt: string;
  dueDate: string;
  assignedStaff?: string;
  documents: {
    id: string;
    name: string;
    type: string;
    status: "pending" | "verified" | "rejected";
  }[];
  notes?: string;
  reviewHistory: {
    date: string;
    action: string;
    staff: string;
    comment: string;
  }[];
}

export default function StaffLicensesPage() {
  const { user, isAdminStaff } = useAuth();
  const [applications, setApplications] = useState<LicenseApplication[]>([]);
  const [filteredApplications, setFilteredApplications] = useState<LicenseApplication[]>([]);
  const [loading, setLoading] = useState(true);
  const [searchTerm, setSearchTerm] = useState("");
  const [statusFilter, setStatusFilter] = useState("all");
  const [priorityFilter, setPriorityFilter] = useState("all");
  const [selectedApplication, setSelectedApplication] = useState<LicenseApplication | null>(null);
  const [actionType, setActionType] = useState<"approve" | "reject" | "revision" | null>(null);
  const [actionComment, setActionComment] = useState("");

  useEffect(() => {
    // Mock data - replace with actual API calls
    setTimeout(() => {
      const mockApplications: LicenseApplication[] = [
        {
          id: "LIC-001",
          licenseType: "SIUP",
          company: {
            id: "COMP-001",
            name: "PT. Teknologi Maju",
            owner: "Budi Santoso",
            email: "budi@tekmaju.com"
          },
          applicant: {
            name: "Budi Santoso",
            phone: "081234567890",
            position: "Direktur"
          },
          status: "pending_review",
          priority: "high",
          submittedAt: "2024-01-15T08:30:00Z",
          dueDate: "2024-01-18T17:00:00Z",
          assignedStaff: user?.name,
          documents: [
            { id: "DOC-001", name: "KTP Direktur", type: "identity", status: "verified" },
            { id: "DOC-002", name: "Akta Pendirian", type: "legal", status: "pending" },
            { id: "DOC-003", name: "NPWP Perusahaan", type: "tax", status: "verified" }
          ],
          reviewHistory: [
            {
              date: "2024-01-15T09:00:00Z",
              action: "ASSIGNED",
              staff: "System",
              comment: "Ditugaskan ke staff untuk review"
            }
          ]
        },
        {
          id: "LIC-002",
          licenseType: "TDP",
          company: {
            id: "COMP-002",
            name: "CV. Berkah Mandiri",
            owner: "Siti Nurhaliza",
            email: "siti@berkah.com"
          },
          applicant: {
            name: "Siti Nurhaliza",
            phone: "081987654321",
            position: "Pemilik"
          },
          status: "under_review",
          priority: "medium",
          submittedAt: "2024-01-14T14:20:00Z",
          dueDate: "2024-01-17T17:00:00Z",
          assignedStaff: user?.name,
          documents: [
            { id: "DOC-004", name: "KTP Pemilik", type: "identity", status: "verified" },
            { id: "DOC-005", name: "Surat Domisili", type: "address", status: "rejected" },
            { id: "DOC-006", name: "SIUP", type: "business", status: "verified" }
          ],
          notes: "Surat domisili perlu diperbaiki - alamat tidak sesuai dengan data pendaftaran",
          reviewHistory: [
            {
              date: "2024-01-14T15:00:00Z",
              action: "STARTED_REVIEW",
              staff: user?.name || "Staff",
              comment: "Mulai review dokumen"
            },
            {
              date: "2024-01-15T10:30:00Z",
              action: "DOCUMENT_REJECTED",
              staff: user?.name || "Staff",
              comment: "Surat domisili ditolak - alamat tidak sesuai"
            }
          ]
        },
        {
          id: "LIC-003",
          licenseType: "HO",
          company: {
            id: "COMP-003",
            name: "UD. Sukses Bersama",
            owner: "Andi Pratama",
            email: "andi@suksesbersama.com"
          },
          applicant: {
            name: "Rahman Hidayat",
            phone: "081555666777",
            position: "Manager Operasional"
          },
          status: "needs_revision",
          priority: "low",
          submittedAt: "2024-01-13T11:45:00Z",
          dueDate: "2024-01-19T17:00:00Z",
          assignedStaff: user?.name,
          documents: [
            { id: "DOC-007", name: "KTP Manager", type: "identity", status: "verified" },
            { id: "DOC-008", name: "Surat Kuasa", type: "authorization", status: "pending" },
            { id: "DOC-009", name: "Denah Lokasi", type: "location", status: "rejected" }
          ],
          notes: "Denah lokasi kurang detail, perlu menambahkan titik koordinat GPS",
          reviewHistory: [
            {
              date: "2024-01-13T12:00:00Z",
              action: "STARTED_REVIEW", 
              staff: user?.name || "Staff",
              comment: "Mulai review aplikasi HO"
            },
            {
              date: "2024-01-14T16:20:00Z",
              action: "REQUESTED_REVISION",
              staff: user?.name || "Staff",
              comment: "Meminta revisi untuk denah lokasi"
            }
          ]
        },
        {
          id: "LIC-004",
          licenseType: "NIB",
          company: {
            id: "COMP-004",
            name: "PT. Digital Innovation",
            owner: "Lisa Permata",
            email: "lisa@diginov.com"
          },
          applicant: {
            name: "Lisa Permata",
            phone: "081222333444",
            position: "CEO"
          },
          status: "pending_review",
          priority: "urgent",
          submittedAt: "2024-01-15T13:15:00Z",
          dueDate: "2024-01-16T17:00:00Z",
          assignedStaff: user?.name,
          documents: [
            { id: "DOC-010", name: "KTP CEO", type: "identity", status: "pending" },
            { id: "DOC-011", name: "Akta Pendirian PT", type: "legal", status: "pending" },
            { id: "DOC-012", name: "SK Menkumham", type: "legal", status: "pending" },
            { id: "DOC-013", name: "NPWP Perusahaan", type: "tax", status: "pending" }
          ],
          reviewHistory: [
            {
              date: "2024-01-15T13:30:00Z",
              action: "ASSIGNED",
              staff: "System",
              comment: "Aplikasi NIB urgent - ditugaskan untuk review prioritas"
            }
          ]
        }
      ];

      setApplications(mockApplications);
      setFilteredApplications(mockApplications);
      setLoading(false);
    }, 1000);
  }, [user?.name]);

  useEffect(() => {
    let filtered = applications;

    // Search filter
    if (searchTerm) {
      filtered = filtered.filter(app => 
        app.company.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
        app.licenseType.toLowerCase().includes(searchTerm.toLowerCase()) ||
        app.id.toLowerCase().includes(searchTerm.toLowerCase())
      );
    }

    // Status filter
    if (statusFilter !== "all") {
      filtered = filtered.filter(app => app.status === statusFilter);
    }

    // Priority filter
    if (priorityFilter !== "all") {
      filtered = filtered.filter(app => app.priority === priorityFilter);
    }

    setFilteredApplications(filtered);
  }, [applications, searchTerm, statusFilter, priorityFilter]);

  const getPriorityColor = (priority: string) => {
    switch (priority) {
      case "urgent": return "bg-red-100 text-red-800 border-red-200";
      case "high": return "bg-orange-100 text-orange-800 border-orange-200";
      case "medium": return "bg-yellow-100 text-yellow-800 border-yellow-200";
      case "low": return "bg-green-100 text-green-800 border-green-200";
      default: return "bg-gray-100 text-gray-800 border-gray-200";
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case "pending_review": return "bg-blue-100 text-blue-800";
      case "under_review": return "bg-yellow-100 text-yellow-800";
      case "approved": return "bg-green-100 text-green-800";
      case "rejected": return "bg-red-100 text-red-800";
      case "needs_revision": return "bg-orange-100 text-orange-800";
      default: return "bg-gray-100 text-gray-800";
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case "pending_review": return <ClockIcon className="h-4 w-4" />;
      case "under_review": return <EyeIcon className="h-4 w-4" />;
      case "approved": return <CheckCircleIcon className="h-4 w-4" />;
      case "rejected": return <XCircleIcon className="h-4 w-4" />;
      case "needs_revision": return <ExclamationTriangleIcon className="h-4 w-4" />;
      default: return <ClockIcon className="h-4 w-4" />;
    }
  };

  const getDocumentStatusColor = (status: string) => {
    switch (status) {
      case "verified": return "text-green-600";
      case "rejected": return "text-red-600";
      case "pending": return "text-yellow-600";
      default: return "text-gray-600";
    }
  };

  const handleAction = (application: LicenseApplication, action: "approve" | "reject" | "revision") => {
    setSelectedApplication(application);
    setActionType(action);
    setActionComment("");
  };

  const submitAction = () => {
    if (!selectedApplication || !actionType || !actionComment.trim()) return;

    // TODO: Replace with actual API call
    const updatedApplications = applications.map(app => {
      if (app.id === selectedApplication.id) {
        const newStatus = actionType === "approve" ? "approved" : 
                         actionType === "reject" ? "rejected" : "needs_revision";
        
        return {
          ...app,
          status: newStatus as typeof app.status,
          reviewHistory: [
            ...app.reviewHistory,
            {
              date: new Date().toISOString(),
              action: actionType.toUpperCase(),
              staff: user?.name || "Staff",
              comment: actionComment
            }
          ],
          notes: actionComment
        };
      }
      return app;
    });

    setApplications(updatedApplications);
    setSelectedApplication(null);
    setActionType(null);
    setActionComment("");
    
    alert(`Aplikasi ${selectedApplication.id} berhasil ${actionType === "approve" ? "disetujui" : actionType === "reject" ? "ditolak" : "diminta revisi"}!`);
  };

  if (!isAdminStaff()) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-center">
          <ExclamationTriangleIcon className="mx-auto h-16 w-16 text-red-500 mb-4" />
          <h2 className="text-xl font-semibold text-gray-900 mb-2">Akses Ditolak</h2>
          <p className="text-gray-600">Halaman ini hanya dapat diakses oleh Staff Administrator.</p>
        </div>
      </div>
    );
  }

  if (loading) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="animate-spin rounded-full h-32 w-32 border-b-2 border-green-600"></div>
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
                <ShieldCheckIcon className="h-8 w-8 text-green-600 mr-3" />
                <div>
                  <h1 className="text-2xl font-bold text-gray-900">Review Perizinan</h1>
                  <p className="text-sm text-gray-500">Proses dan review aplikasi perizinan UMKM</p>
                </div>
              </div>
              <div className="flex items-center space-x-4">
                <div className="text-sm text-gray-500">
                  {filteredApplications.length} dari {applications.length} aplikasi
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Stats Cards */}
        <div className="grid grid-cols-1 md:grid-cols-4 gap-6 mb-8">
          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center">
              <div className="p-2 bg-blue-100 rounded-full">
                <ClockIcon className="h-6 w-6 text-blue-600" />
              </div>
              <div className="ml-3">
                <p className="text-sm font-medium text-gray-500">Pending Review</p>
                <p className="text-2xl font-semibold text-gray-900">
                  {applications.filter(app => app.status === "pending_review").length}
                </p>
              </div>
            </div>
          </div>

          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center">
              <div className="p-2 bg-yellow-100 rounded-full">
                <EyeIcon className="h-6 w-6 text-yellow-600" />
              </div>
              <div className="ml-3">
                <p className="text-sm font-medium text-gray-500">Under Review</p>
                <p className="text-2xl font-semibold text-gray-900">
                  {applications.filter(app => app.status === "under_review").length}
                </p>
              </div>
            </div>
          </div>

          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center">
              <div className="p-2 bg-orange-100 rounded-full">
                <ExclamationTriangleIcon className="h-6 w-6 text-orange-600" />
              </div>
              <div className="ml-3">
                <p className="text-sm font-medium text-gray-500">Needs Revision</p>
                <p className="text-2xl font-semibold text-gray-900">
                  {applications.filter(app => app.status === "needs_revision").length}
                </p>
              </div>
            </div>
          </div>

          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center">
              <div className="p-2 bg-red-100 rounded-full">
                <ExclamationTriangleIcon className="h-6 w-6 text-red-600" />
              </div>
              <div className="ml-3">
                <p className="text-sm font-medium text-gray-500">Urgent Priority</p>
                <p className="text-2xl font-semibold text-gray-900">
                  {applications.filter(app => app.priority === "urgent").length}
                </p>
              </div>
            </div>
          </div>
        </div>

        {/* Filters */}
        <div className="bg-white rounded-lg shadow mb-8">
          <div className="px-6 py-4">
            <div className="flex flex-col lg:flex-row lg:items-center lg:justify-between space-y-4 lg:space-y-0">
              {/* Search */}
              <div className="relative flex-1 max-w-lg">
                <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                  <MagnifyingGlassIcon className="h-5 w-5 text-gray-400" />
                </div>
                <input
                  type="text"
                  value={searchTerm}
                  onChange={(e) => setSearchTerm(e.target.value)}
                  className="block w-full pl-10 pr-3 py-2 border border-gray-300 rounded-md leading-5 placeholder-gray-500 focus:outline-none focus:placeholder-gray-400 focus:ring-1 focus:ring-green-500 focus:border-green-500"
                  placeholder="Cari berdasarkan perusahaan, jenis perizinan, atau ID..."
                />
              </div>

              {/* Filters */}
              <div className="flex space-x-4">
                <select
                  value={statusFilter}
                  onChange={(e) => setStatusFilter(e.target.value)}
                  className="px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-green-500 focus:border-green-500"
                >
                  <option value="all">Semua Status</option>
                  <option value="pending_review">Pending Review</option>
                  <option value="under_review">Under Review</option>
                  <option value="needs_revision">Needs Revision</option>
                  <option value="approved">Approved</option>
                  <option value="rejected">Rejected</option>
                </select>

                <select
                  value={priorityFilter}
                  onChange={(e) => setPriorityFilter(e.target.value)}
                  className="px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-green-500 focus:border-green-500"
                >
                  <option value="all">Semua Prioritas</option>
                  <option value="urgent">Urgent</option>
                  <option value="high">High</option>
                  <option value="medium">Medium</option>
                  <option value="low">Low</option>
                </select>
              </div>
            </div>
          </div>
        </div>

        {/* Applications List */}
        <div className="space-y-6">
          {filteredApplications.map((application) => (
            <div key={application.id} className="bg-white rounded-lg shadow">
              <div className="px-6 py-4 border-b border-gray-200">
                <div className="flex items-center justify-between">
                  <div className="flex items-center space-x-4">
                    <div className="flex items-center space-x-2">
                      <div className={`p-1 rounded-full ${getStatusColor(application.status)}`}>
                        {getStatusIcon(application.status)}
                      </div>
                      <div>
                        <h3 className="text-lg font-medium text-gray-900">
                          {application.licenseType} - {application.company.name}
                        </h3>
                        <p className="text-sm text-gray-500">ID: {application.id}</p>
                      </div>
                    </div>
                    <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border ${getPriorityColor(application.priority)}`}>
                      {application.priority.toUpperCase()}
                    </span>
                  </div>
                  <div className="flex items-center space-x-2">
                    <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${getStatusColor(application.status)}`}>
                      {application.status.replace('_', ' ').toUpperCase()}
                    </span>
                  </div>
                </div>
              </div>

              <div className="px-6 py-4">
                <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
                  {/* Company Info */}
                  <div>
                    <h4 className="text-sm font-medium text-gray-700 mb-3 flex items-center">
                      <BuildingOfficeIcon className="h-4 w-4 mr-2" />
                      Informasi Perusahaan
                    </h4>
                    <div className="space-y-2 text-sm">
                      <div><span className="text-gray-500">Perusahaan:</span> {application.company.name}</div>
                      <div><span className="text-gray-500">Pemilik:</span> {application.company.owner}</div>
                      <div><span className="text-gray-500">Email:</span> {application.company.email}</div>
                    </div>
                  </div>

                  {/* Applicant Info */}
                  <div>
                    <h4 className="text-sm font-medium text-gray-700 mb-3 flex items-center">
                      <UserIcon className="h-4 w-4 mr-2" />
                      Pemohon
                    </h4>
                    <div className="space-y-2 text-sm">
                      <div><span className="text-gray-500">Nama:</span> {application.applicant.name}</div>
                      <div><span className="text-gray-500">Jabatan:</span> {application.applicant.position}</div>
                      <div><span className="text-gray-500">Telepon:</span> {application.applicant.phone}</div>
                    </div>
                  </div>

                  {/* Timeline */}
                  <div>
                    <h4 className="text-sm font-medium text-gray-700 mb-3 flex items-center">
                      <CalendarIcon className="h-4 w-4 mr-2" />
                      Timeline
                    </h4>
                    <div className="space-y-2 text-sm">
                      <div><span className="text-gray-500">Submitted:</span> {new Date(application.submittedAt).toLocaleDateString("id-ID")}</div>
                      <div><span className="text-gray-500">Due Date:</span> <span className="text-red-600">{new Date(application.dueDate).toLocaleDateString("id-ID")}</span></div>
                      <div><span className="text-gray-500">Assigned:</span> {application.assignedStaff}</div>
                    </div>
                  </div>
                </div>

                {/* Documents */}
                <div className="mt-6">
                  <h4 className="text-sm font-medium text-gray-700 mb-3 flex items-center">
                    <DocumentTextIcon className="h-4 w-4 mr-2" />
                    Dokumen ({application.documents.length})
                  </h4>
                  <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
                    {application.documents.map((doc) => (
                      <div key={doc.id} className="flex items-center justify-between p-3 border border-gray-200 rounded">
                        <div className="flex-1">
                          <p className="text-sm font-medium text-gray-900">{doc.name}</p>
                          <p className="text-xs text-gray-500 capitalize">{doc.type}</p>
                        </div>
                        <span className={`text-xs font-medium ${getDocumentStatusColor(doc.status)}`}>
                          {doc.status}
                        </span>
                      </div>
                    ))}
                  </div>
                </div>

                {/* Notes */}
                {application.notes && (
                  <div className="mt-4 p-3 bg-yellow-50 rounded-lg">
                    <p className="text-sm text-gray-700"><strong>Catatan:</strong> {application.notes}</p>
                  </div>
                )}

                {/* Actions */}
                {(application.status === "pending_review" || application.status === "under_review") && (
                  <div className="mt-6 flex items-center space-x-3">
                    <button
                      onClick={() => handleAction(application, "approve")}
                      className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-green-600 hover:bg-green-700"
                    >
                      <CheckCircleIcon className="h-4 w-4 mr-2" />
                      Setujui
                    </button>
                    <button
                      onClick={() => handleAction(application, "revision")}
                      className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-yellow-600 hover:bg-yellow-700"
                    >
                      <ExclamationTriangleIcon className="h-4 w-4 mr-2" />
                      Minta Revisi
                    </button>
                    <button
                      onClick={() => handleAction(application, "reject")}
                      className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-red-600 hover:bg-red-700"
                    >
                      <XCircleIcon className="h-4 w-4 mr-2" />
                      Tolak
                    </button>
                  </div>
                )}
              </div>
            </div>
          ))}
        </div>

        {filteredApplications.length === 0 && (
          <div className="text-center py-12 bg-white rounded-lg shadow">
            <ShieldCheckIcon className="mx-auto h-12 w-12 text-gray-400" />
            <h3 className="mt-2 text-sm font-medium text-gray-900">Tidak ada aplikasi perizinan</h3>
            <p className="mt-1 text-sm text-gray-500">
              Tidak ada aplikasi yang sesuai dengan filter yang dipilih.
            </p>
          </div>
        )}
      </div>

      {/* Action Modal */}
      {selectedApplication && actionType && (
        <div className="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50">
          <div className="relative top-20 mx-auto p-5 border w-11/12 max-w-2xl shadow-lg rounded-md bg-white">
            <div className="mt-3">
              <div className="flex items-center justify-between mb-4">
                <h3 className="text-lg font-medium text-gray-900">
                  {actionType === "approve" ? "Setujui" : actionType === "reject" ? "Tolak" : "Minta Revisi"} Aplikasi
                </h3>
                <button
                  onClick={() => {
                    setSelectedApplication(null);
                    setActionType(null);
                  }}
                  className="text-gray-400 hover:text-gray-600"
                >
                  <XCircleIcon className="h-6 w-6" />
                </button>
              </div>

              <div className="mb-4">
                <p className="text-sm text-gray-600">
                  <strong>Aplikasi:</strong> {selectedApplication.licenseType} - {selectedApplication.company.name}
                </p>
                <p className="text-sm text-gray-600">
                  <strong>ID:</strong> {selectedApplication.id}
                </p>
              </div>

              <div className="mb-4">
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  {actionType === "approve" ? "Catatan Persetujuan" : 
                   actionType === "reject" ? "Alasan Penolakan" : 
                   "Alasan Revisi"} *
                </label>
                <textarea
                  value={actionComment}
                  onChange={(e) => setActionComment(e.target.value)}
                  rows={4}
                  className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-green-500 focus:border-green-500"
                  placeholder={
                    actionType === "approve" ? "Tuliskan catatan persetujuan..." :
                    actionType === "reject" ? "Jelaskan alasan penolakan..." :
                    "Jelaskan apa yang perlu direvisi..."
                  }
                />
              </div>

              <div className="flex justify-end space-x-3">
                <button
                  onClick={() => {
                    setSelectedApplication(null);
                    setActionType(null);
                  }}
                  className="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-md hover:bg-gray-200"
                >
                  Batal
                </button>
                <button
                  onClick={submitAction}
                  disabled={!actionComment.trim()}
                  className={`px-4 py-2 text-sm font-medium text-white rounded-md ${
                    !actionComment.trim() 
                      ? "bg-gray-400 cursor-not-allowed" 
                      : actionType === "approve" 
                        ? "bg-green-600 hover:bg-green-700"
                        : actionType === "reject"
                          ? "bg-red-600 hover:bg-red-700"
                          : "bg-yellow-600 hover:bg-yellow-700"
                  }`}
                >
                  {actionType === "approve" ? "Setujui" : actionType === "reject" ? "Tolak" : "Minta Revisi"}
                </button>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
