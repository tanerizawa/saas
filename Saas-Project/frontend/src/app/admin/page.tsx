"use client";

import { useAuth } from "@/contexts/AuthContext";
import { useState, useEffect } from "react";
import {
  UsersIcon,
  DocumentTextIcon,
  CurrencyDollarIcon,
  ShieldCheckIcon,
  ChartBarIcon,
  ExclamationTriangleIcon,
  ClockIcon,
  CheckCircleIcon,
  XCircleIcon,
  BellIcon
} from "@heroicons/react/24/outline";

interface DashboardStats {
  totalUsers: number;
  activeUsers: number;
  pendingLicenses: number;
  approvedLicenses: number;
  totalDocuments: number;
  pendingDocuments: number;
  totalRevenue: number;
  monthlyRevenue: number;
  systemAlerts: number;
}

interface RecentActivity {
  id: string;
  type: "license" | "document" | "payment" | "user";
  title: string;
  description: string;
  timestamp: string;
  status: "success" | "pending" | "failed";
  user?: string;
}

export default function AdminDashboard() {
  const { user, isSuperAdmin, isAdminStaff } = useAuth();
  const [stats, setStats] = useState<DashboardStats | null>(null);
  const [recentActivity, setRecentActivity] = useState<RecentActivity[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // Mock data - replace with actual API calls
    setTimeout(() => {
      setStats({
        totalUsers: 1847,
        activeUsers: 1234,
        pendingLicenses: 23,
        approvedLicenses: 156,
        totalDocuments: 3452,
        pendingDocuments: 45,
        totalRevenue: 1250000000, // Rp 1.25 Miliar
        monthlyRevenue: 125000000, // Rp 125 Juta
        systemAlerts: 3
      });

      setRecentActivity([
        {
          id: "1",
          type: "license",
          title: "Perizinan SIUP Disetujui",
          description: "PT. Maju Jaya - SIUP telah disetujui dan dokumen perizinan telah diterbitkan",
          timestamp: "2024-01-15T10:30:00Z",
          status: "success",
          user: "Admin Dina"
        },
        {
          id: "2",
          type: "document",
          title: "Dokumen Baru Diupload",
          description: "CV. Berkah Mandiri mengupload dokumen NPWP untuk verifikasi",
          timestamp: "2024-01-15T10:25:00Z",
          status: "pending",
          user: "Budi Santoso"
        },
        {
          id: "3",
          type: "payment",
          title: "Pembayaran Berhasil",
          description: "Pembayaran biaya perizinan TDP sebesar Rp 750.000",
          timestamp: "2024-01-15T10:20:00Z",
          status: "success",
          user: "Siti Nurhaliza"
        },
        {
          id: "4",
          type: "user",
          title: "Registrasi User Baru",
          description: "PT. Teknologi Nusantara mendaftar akun baru",
          timestamp: "2024-01-15T10:15:00Z",
          status: "success",
          user: "System"
        },
        {
          id: "5",
          type: "payment",
          title: "Pembayaran Gagal",
          description: "Pembayaran biaya perizinan HO gagal - Saldo tidak mencukupi",
          timestamp: "2024-01-15T10:10:00Z",
          status: "failed",
          user: "Andi Pratama"
        }
      ]);

      setLoading(false);
    }, 1000);
  }, []);

  const getActivityIcon = (type: string) => {
    switch (type) {
      case "license": return <ShieldCheckIcon className="h-5 w-5" />;
      case "document": return <DocumentTextIcon className="h-5 w-5" />;
      case "payment": return <CurrencyDollarIcon className="h-5 w-5" />;
      case "user": return <UsersIcon className="h-5 w-5" />;
      default: return <ClockIcon className="h-5 w-5" />;
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case "success": return <CheckCircleIcon className="h-4 w-4 text-green-500" />;
      case "pending": return <ClockIcon className="h-4 w-4 text-yellow-500" />;
      case "failed": return <XCircleIcon className="h-4 w-4 text-red-500" />;
      default: return <ClockIcon className="h-4 w-4 text-gray-500" />;
    }
  };

  const formatCurrency = (amount: number) => {
    return new Intl.NumberFormat('id-ID', {
      style: 'currency',
      currency: 'IDR',
      minimumFractionDigits: 0,
      maximumFractionDigits: 0,
    }).format(amount);
  };

  if (!isSuperAdmin() && !isAdminStaff()) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-center">
          <ExclamationTriangleIcon className="mx-auto h-16 w-16 text-red-500 mb-4" />
          <h2 className="text-xl font-semibold text-gray-900 mb-2">Akses Ditolak</h2>
          <p className="text-gray-600">Halaman ini hanya dapat diakses oleh Administrator.</p>
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
              <div>
                <h1 className="text-2xl font-bold text-gray-900">
                  Dashboard Admin {isSuperAdmin() ? "(Super Admin)" : "(Staff)"}
                </h1>
                <p className="text-sm text-gray-500">
                  Selamat datang, {user?.name}! Berikut ringkasan aktivitas sistem hari ini.
                </p>
              </div>
              <div className="flex items-center space-x-3">
                <div className="text-right">
                  <div className="text-sm text-gray-500">Terakhir login</div>
                  <div className="text-sm font-medium text-gray-900">
                    {new Date().toLocaleString("id-ID")}  
                  </div>
                </div>
                <div className="h-10 w-10 bg-blue-600 rounded-full flex items-center justify-center">
                  <span className="text-white font-medium text-sm">
                    {user?.name?.charAt(0).toUpperCase() || 'A'}
                  </span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Alert untuk Super Admin */}
        {isSuperAdmin() && stats && stats.systemAlerts > 0 && (
          <div className="mb-8 bg-yellow-50 border-l-4 border-yellow-400 p-4">
            <div className="flex">
              <div className="flex-shrink-0">
                <BellIcon className="h-5 w-5 text-yellow-400" />
              </div>
              <div className="ml-3">
                <p className="text-sm text-yellow-700">
                  Ada <strong>{stats.systemAlerts}</strong> alert sistem yang memerlukan perhatian.{" "}
                  <a href="/admin/audit-logs" className="font-medium underline text-yellow-700 hover:text-yellow-600">
                    Lihat detail audit logs
                  </a>
                </p>
              </div>
            </div>
          </div>
        )}

        {/* Stats Cards */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
          {/* Total Users */}
          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center">
              <div className="p-2 bg-blue-100 rounded-full">
                <UsersIcon className="h-6 w-6 text-blue-600" />
              </div>
              <div className="ml-3 flex-1">
                <p className="text-sm font-medium text-gray-500">Total Users</p>
                <div className="flex items-baseline">
                  <p className="text-2xl font-semibold text-gray-900">{stats?.totalUsers.toLocaleString()}</p>
                  <p className="ml-2 text-sm text-green-600">+12% dari bulan lalu</p>
                </div>
                <p className="text-xs text-gray-500 mt-1">
                  {stats?.activeUsers.toLocaleString()} aktif bulan ini
                </p>
              </div>
            </div>
          </div>

          {/* Perizinan */}
          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center">
              <div className="p-2 bg-green-100 rounded-full">
                <ShieldCheckIcon className="h-6 w-6 text-green-600" />
              </div>
              <div className="ml-3 flex-1">
                <p className="text-sm font-medium text-gray-500">Perizinan</p>
                <div className="flex items-baseline">
                  <p className="text-2xl font-semibold text-gray-900">{stats?.approvedLicenses}</p>
                  <p className="ml-2 text-sm text-orange-600">{stats?.pendingLicenses} pending</p>
                </div>
                <p className="text-xs text-gray-500 mt-1">
                  Disetujui bulan ini
                </p>
              </div>
            </div>
          </div>

          {/* Dokumen */}
          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center">
              <div className="p-2 bg-purple-100 rounded-full">
                <DocumentTextIcon className="h-6 w-6 text-purple-600" />
              </div>
              <div className="ml-3 flex-1">
                <p className="text-sm font-medium text-gray-500">Dokumen</p>
                <div className="flex items-baseline">
                  <p className="text-2xl font-semibold text-gray-900">{stats?.totalDocuments.toLocaleString()}</p>
                  <p className="ml-2 text-sm text-orange-600">{stats?.pendingDocuments} review</p>
                </div>
                <p className="text-xs text-gray-500 mt-1">
                  Total dokumen tersimpan
                </p>
              </div>
            </div>
          </div>

          {/* Revenue (hanya untuk Super Admin) */}
          {isSuperAdmin() && (
            <div className="bg-white rounded-lg shadow p-6">
              <div className="flex items-center">
                <div className="p-2 bg-yellow-100 rounded-full">
                  <CurrencyDollarIcon className="h-6 w-6 text-yellow-600" />
                </div>
                <div className="ml-3 flex-1">
                  <p className="text-sm font-medium text-gray-500">Revenue</p>
                  <div className="flex items-baseline">
                    <p className="text-lg font-semibold text-gray-900">
                      {formatCurrency(stats?.monthlyRevenue || 0)}
                    </p>
                  </div>
                  <p className="text-xs text-gray-500 mt-1">
                    Bulan ini • Total: {formatCurrency(stats?.totalRevenue || 0)}
                  </p>
                </div>
              </div>
            </div>
          )}
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
          {/* Recent Activity */}
          <div className="lg:col-span-2">
            <div className="bg-white rounded-lg shadow">
              <div className="px-6 py-4 border-b border-gray-200">
                <h3 className="text-lg font-medium text-gray-900">Aktivitas Terbaru</h3>
              </div>
              <div className="divide-y divide-gray-200">
                {recentActivity.map((activity) => (
                  <div key={activity.id} className="px-6 py-4 hover:bg-gray-50">
                    <div className="flex items-start space-x-3">
                      <div className={`flex-shrink-0 p-1 rounded-full ${
                        activity.type === "license" ? "bg-green-100 text-green-600" :
                        activity.type === "document" ? "bg-blue-100 text-blue-600" :
                        activity.type === "payment" ? "bg-yellow-100 text-yellow-600" :
                        "bg-purple-100 text-purple-600"
                      }`}>
                        {getActivityIcon(activity.type)}
                      </div>
                      <div className="flex-1 min-w-0">
                        <div className="flex items-center justify-between">
                          <p className="text-sm font-medium text-gray-900">{activity.title}</p>
                          <div className="flex items-center space-x-2">
                            {getStatusIcon(activity.status)}
                            <span className="text-xs text-gray-500">
                              {new Date(activity.timestamp).toLocaleTimeString("id-ID", {
                                hour: "2-digit",
                                minute: "2-digit"
                              })}
                            </span>
                          </div>
                        </div>
                        <p className="text-sm text-gray-500 mt-1">{activity.description}</p>
                        {activity.user && (
                          <p className="text-xs text-gray-400 mt-1">oleh {activity.user}</p>
                        )}
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </div>

          {/* Quick Actions & System Status */}
          <div className="space-y-6">
            {/* Quick Actions */}
            <div className="bg-white rounded-lg shadow">
              <div className="px-6 py-4 border-b border-gray-200">
                <h3 className="text-lg font-medium text-gray-900">Quick Actions</h3>
              </div>
              <div className="px-6 py-4 space-y-3">
                <a
                  href="/admin/licenses"
                  className="block w-full text-left px-4 py-2 text-sm font-medium text-gray-700 bg-gray-50 rounded-lg hover:bg-gray-100 transition-colors"
                >
                  Review Perizinan Pending ({stats?.pendingLicenses})
                </a>
                <a
                  href="/admin/documents"
                  className="block w-full text-left px-4 py-2 text-sm font-medium text-gray-700 bg-gray-50 rounded-lg hover:bg-gray-100 transition-colors"
                >
                  Verifikasi Dokumen ({stats?.pendingDocuments})
                </a>
                <a
                  href="/admin/users"
                  className="block w-full text-left px-4 py-2 text-sm font-medium text-gray-700 bg-gray-50 rounded-lg hover:bg-gray-100 transition-colors"
                >
                  Manajemen User
                </a>
                {isSuperAdmin() && (
                  <>
                    <a
                      href="/admin/notifications"
                      className="block w-full text-left px-4 py-2 text-sm font-medium text-gray-700 bg-gray-50 rounded-lg hover:bg-gray-100 transition-colors"
                    >
                      Kelola Email Template
                    </a>
                    <a
                      href="/admin/audit-logs"
                      className="block w-full text-left px-4 py-2 text-sm font-medium text-gray-700 bg-gray-50 rounded-lg hover:bg-gray-100 transition-colors"
                    >
                      Lihat Audit Logs
                    </a>
                  </>
                )}
              </div>
            </div>

            {/* System Status */}
            <div className="bg-white rounded-lg shadow">
              <div className="px-6 py-4 border-b border-gray-200">
                <h3 className="text-lg font-medium text-gray-900">Status Sistem</h3>
              </div>
              <div className="px-6 py-4 space-y-3">
                <div className="flex items-center justify-between">
                  <span className="text-sm text-gray-600">Database</span>
                  <div className="flex items-center">
                    <div className="w-2 h-2 bg-green-400 rounded-full mr-2"></div>
                    <span className="text-sm font-medium text-green-600">Online</span>
                  </div>
                </div>
                <div className="flex items-center justify-between">
                  <span className="text-sm text-gray-600">Server</span>
                  <div className="flex items-center">
                    <div className="w-2 h-2 bg-green-400 rounded-full mr-2"></div>
                    <span className="text-sm font-medium text-green-600">Healthy</span>
                  </div>
                </div>
                <div className="flex items-center justify-between">
                  <span className="text-sm text-gray-600">Storage</span>
                  <div className="flex items-center">
                    <div className="w-2 h-2 bg-yellow-400 rounded-full mr-2"></div>
                    <span className="text-sm font-medium text-yellow-600">78% Used</span>
                  </div>
                </div>
                {isSuperAdmin() && (
                  <div className="flex items-center justify-between">
                    <span className="text-sm text-gray-600">Security Alerts</span>
                    <div className="flex items-center">
                      <div className="w-2 h-2 bg-red-400 rounded-full mr-2"></div>
                      <span className="text-sm font-medium text-red-600">{stats?.systemAlerts} Issues</span>
                    </div>
                  </div>
                )}
              </div>
            </div>

            {/* Performance Chart (Placeholder) */}
            <div className="bg-white rounded-lg shadow">
              <div className="px-6 py-4 border-b border-gray-200">
                <h3 className="text-lg font-medium text-gray-900">Performance Overview</h3>
              </div>
              <div className="px-6 py-4">
                <div className="flex items-center justify-center h-32 bg-gray-50 rounded-lg">
                  <div className="text-center">
                    <ChartBarIcon className="h-8 w-8 text-gray-400 mx-auto mb-2" />
                    <p className="text-sm text-gray-500">Chart akan ditampilkan di sini</p>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
