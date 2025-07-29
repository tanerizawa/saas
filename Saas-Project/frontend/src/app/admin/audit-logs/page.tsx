"use client";

import { useAuth } from "@/contexts/AuthContext";
import { useState, useEffect } from "react";
import { 
  ClipboardDocumentListIcon,
  EyeIcon,
  MagnifyingGlassIcon,
  UserIcon,
  DocumentTextIcon,
  ShieldCheckIcon,
  CurrencyDollarIcon,
  ExclamationTriangleIcon,
  ChevronDownIcon,
  FunnelIcon,
  ArrowDownTrayIcon,
  XMarkIcon
} from "@heroicons/react/24/outline";

interface AuditLog {
  id: string;
  timestamp: string;
  user_id: string;
  user_name: string;
  user_role: "super_admin" | "admin_staff" | "umkm_owner";
  action: string;
  resource_type: "user" | "license" | "document" | "payment" | "system";
  resource_id?: string;
  details: string;
  ip_address: string;
  user_agent: string;
  success: boolean;
  severity: "low" | "medium" | "high" | "critical";
}

interface FilterOptions {
  dateRange: "today" | "week" | "month" | "quarter" | "custom";
  customStartDate: string;
  customEndDate: string;
  userRole: string;
  action: string;
  resourceType: string;
  severity: string;
  success: string;
}

export default function AuditLogsPage() {
  const { isSuperAdmin } = useAuth();
  const [logs, setLogs] = useState<AuditLog[]>([]);
  const [filteredLogs, setFilteredLogs] = useState<AuditLog[]>([]);
  const [loading, setLoading] = useState(true);
  const [searchTerm, setSearchTerm] = useState("");
  const [selectedLog, setSelectedLog] = useState<AuditLog | null>(null);
  const [showFilters, setShowFilters] = useState(false);
  const [filters, setFilters] = useState<FilterOptions>({
    dateRange: "week",
    customStartDate: "",
    customEndDate: "",
    userRole: "",
    action: "",
    resourceType: "",
    severity: "",
    success: ""
  });

  useEffect(() => {
    // Mock data - replace with actual API call
    setTimeout(() => {
      const mockLogs: AuditLog[] = [
        {
          id: "1",
          timestamp: "2024-01-15T10:30:00Z",
          user_id: "admin_001",
          user_name: "Admin Dina",
          user_role: "super_admin",
          action: "CREATE_LICENSE",
          resource_type: "license",
          resource_id: "LIC_001",
          details: "Menyetujui perizinan SIUP untuk PT. Maju Jaya",
          ip_address: "192.168.1.100",
          user_agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)",
          success: true,
          severity: "medium"
        },
        {
          id: "2",
          timestamp: "2024-01-15T10:25:00Z",
          user_id: "umkm_001",
          user_name: "Budi Santoso",
          user_role: "umkm_owner",
          action: "UPLOAD_DOCUMENT",
          resource_type: "document",
          resource_id: "DOC_001",
          details: "Upload dokumen KTP untuk verifikasi identitas",
          ip_address: "203.194.112.45",
          user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64)",
          success: true,
          severity: "low"
        },
        {
          id: "3",
          timestamp: "2024-01-15T10:20:00Z",
          user_id: "staff_001",
          user_name: "Siti Nurhaliza",
          user_role: "admin_staff",
          action: "REVIEW_DOCUMENT",
          resource_type: "document",
          resource_id: "DOC_001",
          details: "Review dan verifikasi dokumen KTP - Status: Approved",
          ip_address: "192.168.1.101",
          user_agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)",
          success: true,
          severity: "low"
        },
        {
          id: "4",
          timestamp: "2024-01-15T10:15:00Z",
          user_id: "umkm_002",
          user_name: "Andi Pratama",
          user_role: "umkm_owner",
          action: "PAYMENT_ATTEMPT",
          resource_type: "payment",
          resource_id: "PAY_001",
          details: "Percobaan pembayaran gagal - Insufficient balance",
          ip_address: "114.79.41.123",
          user_agent: "Mozilla/5.0 (Linux; Android 11; SM-G975F)",
          success: false,
          severity: "medium"
        },
        {
          id: "5",
          timestamp: "2024-01-15T10:10:00Z",
          user_id: "admin_001",
          user_name: "Admin Dina",
          user_role: "super_admin",
          action: "USER_LOGIN",
          resource_type: "user",
          resource_id: "admin_001",
          details: "Login berhasil ke dashboard admin",
          ip_address: "192.168.1.100",
          user_agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)",
          success: true,
          severity: "low"
        },
        {
          id: "6",
          timestamp: "2024-01-15T09:55:00Z",
          user_id: "system",
          user_name: "System",
          user_role: "super_admin",
          action: "DATABASE_BACKUP",
          resource_type: "system",
          details: "Scheduled database backup completed successfully",
          ip_address: "127.0.0.1",
          user_agent: "System/1.0",
          success: true,
          severity: "low"
        },
        {
          id: "7",
          timestamp: "2024-01-15T09:30:00Z",
          user_id: "umkm_003",
          user_name: "Lisa Permata",
          user_role: "umkm_owner",
          action: "FAILED_LOGIN",
          resource_type: "user",
          resource_id: "umkm_003",
          details: "Login gagal - Password salah (Percobaan ke-3)",
          ip_address: "180.241.89.76",
          user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64)",
          success: false,
          severity: "high"
        },
        {
          id: "8",
          timestamp: "2024-01-15T09:15:00Z",
          user_id: "admin_001",
          user_name: "Admin Dina",
          user_role: "super_admin",
          action: "DELETE_USER",
          resource_type: "user",
          resource_id: "umkm_004",
          details: "Menghapus akun UMKM yang tidak aktif - CV. Mandiri Sejahtera",
          ip_address: "192.168.1.100",
          user_agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)",
          success: true,
          severity: "critical"
        }
      ];

      setLogs(mockLogs);
      setFilteredLogs(mockLogs);
      setLoading(false);
    }, 1000);
  }, []);

  useEffect(() => {
    // Apply filters
    let filtered = logs;

    // Search filter
    if (searchTerm) {
      filtered = filtered.filter(log => 
        log.user_name.toLowerCase().includes(searchTerm.toLowerCase()) ||
        log.action.toLowerCase().includes(searchTerm.toLowerCase()) ||
        log.details.toLowerCase().includes(searchTerm.toLowerCase()) ||
        log.ip_address.includes(searchTerm)
      );
    }

    // Role filter
    if (filters.userRole) {
      filtered = filtered.filter(log => log.user_role === filters.userRole);
    }

    // Action filter
    if (filters.action) {
      filtered = filtered.filter(log => log.action === filters.action);
    }

    // Resource type filter
    if (filters.resourceType) {
      filtered = filtered.filter(log => log.resource_type === filters.resourceType);
    }

    // Severity filter
    if (filters.severity) {
      filtered = filtered.filter(log => log.severity === filters.severity);
    }

    // Success filter
    if (filters.success) {
      filtered = filtered.filter(log => 
        filters.success === "true" ? log.success : !log.success
      );
    }

    // Date range filter
    const now = new Date();
    let startDate: Date;

    switch (filters.dateRange) {
      case "today":
        startDate = new Date(now.getFullYear(), now.getMonth(), now.getDate());
        break;
      case "week":
        startDate = new Date(now.getTime() - 7 * 24 * 60 * 60 * 1000);
        break;
      case "month":
        startDate = new Date(now.getFullYear(), now.getMonth() - 1, now.getDate());
        break;
      case "quarter":
        startDate = new Date(now.getFullYear(), now.getMonth() - 3, now.getDate());
        break;
      case "custom":
        if (filters.customStartDate && filters.customEndDate) {
          const start = new Date(filters.customStartDate);
          const end = new Date(filters.customEndDate);
          filtered = filtered.filter(log => {
            const logDate = new Date(log.timestamp);
            return logDate >= start && logDate <= end;
          });
        }
        break;
      default:
        startDate = new Date(0); // No filter
    }

    if (filters.dateRange !== "custom") {
      filtered = filtered.filter(log => new Date(log.timestamp) >= startDate);
    }

    setFilteredLogs(filtered);
  }, [logs, searchTerm, filters]);

  const getSeverityColor = (severity: string) => {
    switch (severity) {
      case "critical": return "bg-red-100 text-red-800";
      case "high": return "bg-orange-100 text-orange-800";
      case "medium": return "bg-yellow-100 text-yellow-800";
      case "low": return "bg-green-100 text-green-800";
      default: return "bg-gray-100 text-gray-800";
    }
  };

  const getResourceIcon = (resourceType: string) => {
    switch (resourceType) {
      case "user": return <UserIcon className="h-4 w-4" />;
      case "license": return <ShieldCheckIcon className="h-4 w-4" />;
      case "document": return <DocumentTextIcon className="h-4 w-4" />;
      case "payment": return <CurrencyDollarIcon className="h-4 w-4" />;
      case "system": return <ClipboardDocumentListIcon className="h-4 w-4" />;
      default: return <ClipboardDocumentListIcon className="h-4 w-4" />;
    }
  };

  const exportLogs = () => {
    // TODO: Replace with actual export functionality
    const csvContent = [
      "Timestamp,User,Role,Action,Resource,Details,IP Address,Success,Severity",
      ...filteredLogs.map(log => 
        `${log.timestamp},${log.user_name},${log.user_role},${log.action},${log.resource_type},${log.details},${log.ip_address},${log.success},${log.severity}`
      )
    ].join("\n");

    const blob = new Blob([csvContent], { type: "text/csv" });
    const url = window.URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `audit-logs-${new Date().toISOString().split("T")[0]}.csv`;
    a.click();
    window.URL.revokeObjectURL(url);
  };

  if (!isSuperAdmin()) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-center">
          <ExclamationTriangleIcon className="mx-auto h-16 w-16 text-red-500 mb-4" />
          <h2 className="text-xl font-semibold text-gray-900 mb-2">Akses Ditolak</h2>
          <p className="text-gray-600">Halaman ini hanya dapat diakses oleh Super Admin.</p>
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
                <ClipboardDocumentListIcon className="h-8 w-8 text-blue-600 mr-3" />
                <div>
                  <h1 className="text-2xl font-bold text-gray-900">Audit Logs & Activity Tracking</h1>
                  <p className="text-sm text-gray-500">Monitor dan track semua aktivitas sistem</p>
                </div>
              </div>
              <button
                onClick={exportLogs}
                className="inline-flex items-center px-4 py-2 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50"
              >
                <ArrowDownTrayIcon className="h-4 w-4 mr-2" />
                Export CSV
              </button>
            </div>
          </div>
        </div>
      </div>

      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Statistics Cards */}
        <div className="grid grid-cols-1 md:grid-cols-4 gap-6 mb-8">
          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center">
              <div className="p-2 bg-blue-100 rounded-full">
                <ClipboardDocumentListIcon className="h-6 w-6 text-blue-600" />
              </div>
              <div className="ml-3">
                <p className="text-sm font-medium text-gray-500">Total Aktivitas</p>
                <p className="text-2xl font-semibold text-gray-900">{filteredLogs.length}</p>
              </div>
            </div>
          </div>

          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center">
              <div className="p-2 bg-green-100 rounded-full">
                <ShieldCheckIcon className="h-6 w-6 text-green-600" />
              </div>
              <div className="ml-3">
                <p className="text-sm font-medium text-gray-500">Berhasil</p>
                <p className="text-2xl font-semibold text-gray-900">
                  {filteredLogs.filter(log => log.success).length}
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
                <p className="text-sm font-medium text-gray-500">Gagal</p>
                <p className="text-2xl font-semibold text-gray-900">
                  {filteredLogs.filter(log => !log.success).length}
                </p>
              </div>
            </div>
          </div>

          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center">
              <div className="p-2 bg-orange-100 rounded-full">
                <UserIcon className="h-6 w-6 text-orange-600" />
              </div>
              <div className="ml-3">
                <p className="text-sm font-medium text-gray-500">User Aktif</p>
                <p className="text-2xl font-semibold text-gray-900">
                  {new Set(filteredLogs.map(log => log.user_id)).size}
                </p>
              </div>
            </div>
          </div>
        </div>

        {/* Search and Filters */}
        <div className="bg-white rounded-lg shadow mb-8">
          <div className="px-6 py-4 border-b border-gray-200">
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
                  className="block w-full pl-10 pr-3 py-2 border border-gray-300 rounded-md leading-5 placeholder-gray-500 focus:outline-none focus:placeholder-gray-400 focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
                  placeholder="Cari berdasarkan user, action, atau IP address..."
                />
              </div>

              {/* Filter Toggle */}
              <button
                onClick={() => setShowFilters(!showFilters)}
                className="inline-flex items-center px-4 py-2 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50"
              >
                <FunnelIcon className="h-4 w-4 mr-2" />
                Filter
                <ChevronDownIcon className={`h-4 w-4 ml-2 transform transition-transform ${showFilters ? 'rotate-180' : ''}`} />
              </button>
            </div>

            {/* Filters Panel */}
            {showFilters && (
              <div className="mt-4 p-4 bg-gray-50 rounded-lg">
                <div className="grid grid-cols-1 md:grid-cols-3 lg:grid-cols-6 gap-4">
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">Periode</label>
                    <select
                      value={filters.dateRange}
                      onChange={(e) => setFilters(prev => ({ ...prev, dateRange: e.target.value as FilterOptions['dateRange'] }))}
                      className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 text-sm"
                    >
                      <option value="today">Hari Ini</option>
                      <option value="week">7 Hari</option>
                      <option value="month">30 Hari</option>
                      <option value="quarter">90 Hari</option>
                      <option value="custom">Custom</option>
                    </select>
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">Role</label>
                    <select
                      value={filters.userRole}
                      onChange={(e) => setFilters(prev => ({ ...prev, userRole: e.target.value }))}
                      className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 text-sm"
                    >
                      <option value="">Semua Role</option>
                      <option value="super_admin">Super Admin</option>
                      <option value="admin_staff">Admin Staff</option>
                      <option value="umkm_owner">UMKM Owner</option>
                    </select>
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">Resource</label>
                    <select
                      value={filters.resourceType}
                      onChange={(e) => setFilters(prev => ({ ...prev, resourceType: e.target.value }))}
                      className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 text-sm"
                    >
                      <option value="">Semua Resource</option>
                      <option value="user">User</option>
                      <option value="license">License</option>
                      <option value="document">Document</option>
                      <option value="payment">Payment</option>
                      <option value="system">System</option>
                    </select>
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">Severity</label>
                    <select
                      value={filters.severity}
                      onChange={(e) => setFilters(prev => ({ ...prev, severity: e.target.value }))}
                      className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 text-sm"
                    >
                      <option value="">Semua Level</option>
                      <option value="low">Low</option>
                      <option value="medium">Medium</option>
                      <option value="high">High</option>
                      <option value="critical">Critical</option>
                    </select>
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">Status</label>
                    <select
                      value={filters.success}
                      onChange={(e) => setFilters(prev => ({ ...prev, success: e.target.value }))}
                      className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 text-sm"
                    >
                      <option value="">Semua Status</option>
                      <option value="true">Berhasil</option>
                      <option value="false">Gagal</option>
                    </select>
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">Action</label>
                    <button
                      onClick={() => setFilters({
                        dateRange: "week",
                        customStartDate: "",
                        customEndDate: "",
                        userRole: "",
                        action: "",
                        resourceType: "",
                        severity: "",
                        success: ""
                      })}
                      className="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50"
                    >
                      Reset Filter
                    </button>
                  </div>
                </div>
              </div>
            )}
          </div>
        </div>

        {/* Audit Logs Table */}
        <div className="bg-white rounded-lg shadow overflow-hidden">
          <div className="px-6 py-4 border-b border-gray-200">
            <h3 className="text-lg font-medium text-gray-900">
              Log Aktivitas ({filteredLogs.length} entries)
            </h3>
          </div>

          <div className="overflow-x-auto">
            <table className="min-w-full divide-y divide-gray-200">
              <thead className="bg-gray-50">
                <tr>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Timestamp
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    User
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Action
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Resource
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Status
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Severity
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    IP Address
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Action
                  </th>
                </tr>
              </thead>
              <tbody className="bg-white divide-y divide-gray-200">
                {filteredLogs.map((log) => (
                  <tr key={log.id} className="hover:bg-gray-50">
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                      {new Date(log.timestamp).toLocaleString("id-ID")}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="flex items-center">
                        <div className="flex-shrink-0 h-8 w-8">
                          <div className="h-8 w-8 rounded-full bg-gray-200 flex items-center justify-center">
                            <UserIcon className="h-4 w-4 text-gray-600" />
                          </div>
                        </div>
                        <div className="ml-3">
                          <div className="text-sm font-medium text-gray-900">{log.user_name}</div>
                          <div className="text-sm text-gray-500">{log.user_role}</div>
                        </div>
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                      {log.action.replace(/_/g, ' ')}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="flex items-center">
                        <div className="flex-shrink-0 mr-2">
                          {getResourceIcon(log.resource_type)}
                        </div>
                        <span className="text-sm text-gray-900 capitalize">
                          {log.resource_type}
                        </span>
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${
                        log.success ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800'
                      }`}>
                        {log.success ? 'Berhasil' : 'Gagal'}
                      </span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${getSeverityColor(log.severity)}`}>
                        {log.severity.toUpperCase()}
                      </span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                      {log.ip_address}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm font-medium">
                      <button
                        onClick={() => setSelectedLog(log)}
                        className="text-blue-600 hover:text-blue-500 flex items-center"
                      >
                        <EyeIcon className="h-4 w-4 mr-1" />
                        Detail
                      </button>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>

          {filteredLogs.length === 0 && (
            <div className="text-center py-12">
              <ClipboardDocumentListIcon className="mx-auto h-12 w-12 text-gray-400" />
              <h3 className="mt-2 text-sm font-medium text-gray-900">Tidak ada log aktivitas</h3>
              <p className="mt-1 text-sm text-gray-500">
                Tidak ada log yang sesuai dengan kriteria filter yang dipilih.
              </p>
            </div>
          )}
        </div>
      </div>

      {/* Detail Modal */}
      {selectedLog && (
        <div className="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50">
          <div className="relative top-20 mx-auto p-5 border w-11/12 max-w-2xl shadow-lg rounded-md bg-white">
            <div className="mt-3">
              <div className="flex items-center justify-between mb-4">
                <h3 className="text-lg font-medium text-gray-900">Detail Log Aktivitas</h3>
                <button
                  onClick={() => setSelectedLog(null)}
                  className="text-gray-400 hover:text-gray-600"
                >
                  <XMarkIcon className="h-6 w-6" />
                </button>
              </div>

              <div className="space-y-4">
                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <label className="text-sm font-medium text-gray-500">Timestamp</label>
                    <p className="text-sm text-gray-900">{new Date(selectedLog.timestamp).toLocaleString("id-ID")}</p>
                  </div>
                  <div>
                    <label className="text-sm font-medium text-gray-500">User</label>
                    <p className="text-sm text-gray-900">{selectedLog.user_name} ({selectedLog.user_role})</p>
                  </div>
                  <div>
                    <label className="text-sm font-medium text-gray-500">Action</label>
                    <p className="text-sm text-gray-900">{selectedLog.action.replace(/_/g, ' ')}</p>
                  </div>
                  <div>
                    <label className="text-sm font-medium text-gray-500">Resource</label>
                    <p className="text-sm text-gray-900 capitalize">{selectedLog.resource_type}</p>
                  </div>
                  <div>
                    <label className="text-sm font-medium text-gray-500">Status</label>
                    <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${
                      selectedLog.success ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800'
                    }`}>
                      {selectedLog.success ? 'Berhasil' : 'Gagal'}
                    </span>
                  </div>
                  <div>
                    <label className="text-sm font-medium text-gray-500">Severity</label>
                    <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${getSeverityColor(selectedLog.severity)}`}>
                      {selectedLog.severity.toUpperCase()}
                    </span>
                  </div>
                </div>

                <div>
                  <label className="text-sm font-medium text-gray-500">Details</label>
                  <p className="text-sm text-gray-900 mt-1 p-3 bg-gray-50 rounded">{selectedLog.details}</p>
                </div>

                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <label className="text-sm font-medium text-gray-500">IP Address</label>
                    <p className="text-sm text-gray-900">{selectedLog.ip_address}</p>
                  </div>
                  <div>
                    <label className="text-sm font-medium text-gray-500">User Agent</label>
                    <p className="text-sm text-gray-900 truncate" title={selectedLog.user_agent}>
                      {selectedLog.user_agent}
                    </p>
                  </div>
                </div>
              </div>

              <div className="mt-6 flex justify-end">
                <button
                  onClick={() => setSelectedLog(null)}
                  className="px-4 py-2 bg-gray-300 text-gray-700 rounded-md hover:bg-gray-400"
                >
                  Tutup
                </button>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
