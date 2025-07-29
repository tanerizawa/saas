"use client";

import { useAuth } from "@/contexts/AuthContext";
import { useState, useEffect } from "react";
import {
  DocumentTextIcon,
  ShieldCheckIcon,
  ClockIcon,
  ExclamationTriangleIcon,
  ChartBarIcon,
  TicketIcon
} from "@heroicons/react/24/outline";

interface StaffStats {
  assignedCompanies: number;
  pendingLicenses: number;
  completedLicenses: number;
  documentsToReview: number;
  openTickets: number;
  resolvedTickets: number;
  thisWeekProcessed: number;
  averageProcessingTime: number; // in hours
}

interface AssignedCompany {
  id: string;
  name: string;
  owner: string;
  status: "active" | "pending" | "suspended";
  pendingLicenses: number;
  lastActivity: string;
}

interface PendingTask {
  id: string;
  type: "license_review" | "document_verification" | "support_ticket";
  title: string;
  company: string;
  priority: "low" | "medium" | "high" | "urgent";
  createdAt: string;
  dueDate?: string;
}

export default function StaffDashboard() {
  const { user, isAdminStaff } = useAuth();
  const [stats, setStats] = useState<StaffStats | null>(null);
  const [assignedCompanies, setAssignedCompanies] = useState<AssignedCompany[]>([]);
  const [pendingTasks, setPendingTasks] = useState<PendingTask[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // Mock data - replace with actual API calls
    setTimeout(() => {
      setStats({
        assignedCompanies: 15,
        pendingLicenses: 8,
        completedLicenses: 23,
        documentsToReview: 12,
        openTickets: 5,
        resolvedTickets: 18,
        thisWeekProcessed: 11,
        averageProcessingTime: 4.2
      });

      setAssignedCompanies([
        {
          id: "1",
          name: "PT. Teknologi Maju",
          owner: "Budi Santoso",
          status: "active",
          pendingLicenses: 2,
          lastActivity: "2024-01-15T10:30:00Z"
        },
        {
          id: "2", 
          name: "CV. Berkah Mandiri",
          owner: "Siti Nurhaliza",
          status: "pending",
          pendingLicenses: 1,
          lastActivity: "2024-01-15T09:45:00Z"
        },
        {
          id: "3",
          name: "UD. Sukses Bersama",
          owner: "Andi Pratama",
          status: "active",
          pendingLicenses: 0,
          lastActivity: "2024-01-14T16:20:00Z"
        },
        {
          id: "4",
          name: "PT. Digital Innovation",
          owner: "Lisa Permata",
          status: "active",
          pendingLicenses: 3,
          lastActivity: "2024-01-15T08:15:00Z"
        },
        {
          id: "5",
          name: "CV. Mandiri Sejahtera",
          owner: "Rahman Hidayat",
          status: "suspended",
          pendingLicenses: 0,
          lastActivity: "2024-01-10T14:30:00Z"
        }
      ]);

      setPendingTasks([
        {
          id: "1",
          type: "license_review",
          title: "Review Perizinan SIUP - PT. Teknologi Maju",
          company: "PT. Teknologi Maju",
          priority: "high",
          createdAt: "2024-01-15T09:00:00Z",
          dueDate: "2024-01-16T17:00:00Z"
        },
        {
          id: "2",
          type: "document_verification",
          title: "Verifikasi KTP - CV. Berkah Mandiri",
          company: "CV. Berkah Mandiri",
          priority: "medium",
          createdAt: "2024-01-15T10:15:00Z",
          dueDate: "2024-01-17T12:00:00Z"
        },
        {
          id: "3",
          type: "support_ticket",
          title: "Bantuan Upload Dokumen - UD. Sukses Bersama",
          company: "UD. Sukses Bersama",
          priority: "low",
          createdAt: "2024-01-15T11:30:00Z"
        },
        {
          id: "4",
          type: "license_review",
          title: "Review Perizinan TDP - PT. Digital Innovation",
          company: "PT. Digital Innovation", 
          priority: "urgent",
          createdAt: "2024-01-15T08:45:00Z",
          dueDate: "2024-01-15T18:00:00Z"
        },
        {
          id: "5",
          type: "document_verification",
          title: "Verifikasi NPWP - PT. Teknologi Maju",
          company: "PT. Teknologi Maju",
          priority: "medium",
          createdAt: "2024-01-15T12:00:00Z",
          dueDate: "2024-01-18T10:00:00Z"
        }
      ]);

      setLoading(false);
    }, 1000);
  }, []);

  const getPriorityColor = (priority: string) => {
    switch (priority) {
      case "urgent": return "bg-red-100 text-red-800 border-red-200";
      case "high": return "bg-orange-100 text-orange-800 border-orange-200";
      case "medium": return "bg-yellow-100 text-yellow-800 border-yellow-200";
      case "low": return "bg-green-100 text-green-800 border-green-200";
      default: return "bg-gray-100 text-gray-800 border-gray-200";
    }
  };

  const getTaskIcon = (type: string) => {
    switch (type) {
      case "license_review": return <ShieldCheckIcon className="h-4 w-4" />;
      case "document_verification": return <DocumentTextIcon className="h-4 w-4" />;
      case "support_ticket": return <TicketIcon className="h-4 w-4" />;
      default: return <ClockIcon className="h-4 w-4" />;
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case "active": return "bg-green-100 text-green-800";
      case "pending": return "bg-yellow-100 text-yellow-800";
      case "suspended": return "bg-red-100 text-red-800";
      default: return "bg-gray-100 text-gray-800";
    }
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
                <h1 className="text-2xl font-bold text-gray-900">Staff Dashboard</h1>
                <p className="text-sm text-gray-500">
                  Selamat datang, {user?.name}! Kelola tugas dan UMKM yang ditugaskan kepada Anda.
                </p>
              </div>
              <div className="flex items-center space-x-3">
                <div className="text-right">
                  <div className="text-sm text-gray-500">Companies Assigned</div>
                  <div className="text-lg font-semibold text-blue-600">
                    {stats?.assignedCompanies}
                  </div>
                </div>
                <div className="h-10 w-10 bg-green-600 rounded-full flex items-center justify-center">
                  <span className="text-white font-medium text-sm">
                    {user?.name?.charAt(0).toUpperCase() || 'S'}
                  </span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Quick Stats */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center">
              <div className="p-2 bg-blue-100 rounded-full">
                <ShieldCheckIcon className="h-6 w-6 text-blue-600" />
              </div>
              <div className="ml-3 flex-1">
                <p className="text-sm font-medium text-gray-500">Perizinan Pending</p>
                <div className="flex items-baseline">
                  <p className="text-2xl font-semibold text-gray-900">{stats?.pendingLicenses}</p>
                  <p className="ml-2 text-sm text-gray-500">dari {stats?.assignedCompanies} perusahaan</p>
                </div>
              </div>
            </div>
          </div>

          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center">
              <div className="p-2 bg-green-100 rounded-full">
                <DocumentTextIcon className="h-6 w-6 text-green-600" />
              </div>
              <div className="ml-3 flex-1">
                <p className="text-sm font-medium text-gray-500">Dokumen Review</p>
                <div className="flex items-baseline">
                  <p className="text-2xl font-semibold text-gray-900">{stats?.documentsToReview}</p>
                  <p className="ml-2 text-sm text-green-600">perlu verifikasi</p>
                </div>
              </div>
            </div>
          </div>

          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center">
              <div className="p-2 bg-purple-100 rounded-full">
                <TicketIcon className="h-6 w-6 text-purple-600" />
              </div>
              <div className="ml-3 flex-1">
                <p className="text-sm font-medium text-gray-500">Support Tickets</p>
                <div className="flex items-baseline">
                  <p className="text-2xl font-semibold text-gray-900">{stats?.openTickets}</p>
                  <p className="ml-2 text-sm text-purple-600">terbuka</p>
                </div>
              </div>
            </div>
          </div>

          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center">
              <div className="p-2 bg-yellow-100 rounded-full">
                <ChartBarIcon className="h-6 w-6 text-yellow-600" />
              </div>
              <div className="ml-3 flex-1">
                <p className="text-sm font-medium text-gray-500">Weekly Performance</p>
                <div className="flex items-baseline">
                  <p className="text-2xl font-semibold text-gray-900">{stats?.thisWeekProcessed}</p>
                  <p className="ml-2 text-sm text-yellow-600">selesai minggu ini</p>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
          {/* Pending Tasks */}
          <div className="lg:col-span-2">
            <div className="bg-white rounded-lg shadow">
              <div className="px-6 py-4 border-b border-gray-200">
                <div className="flex items-center justify-between">
                  <h3 className="text-lg font-medium text-gray-900">Tugas Pending</h3>
                  <span className="text-sm text-gray-500">{pendingTasks.length} tugas</span>
                </div>
              </div>
              <div className="divide-y divide-gray-200 max-h-96 overflow-y-auto">
                {pendingTasks.map((task) => (
                  <div key={task.id} className="px-6 py-4 hover:bg-gray-50">
                    <div className="flex items-start space-x-3">
                      <div className={`flex-shrink-0 p-1 rounded-full ${
                        task.type === "license_review" ? "bg-blue-100 text-blue-600" :
                        task.type === "document_verification" ? "bg-green-100 text-green-600" :
                        "bg-purple-100 text-purple-600"
                      }`}>
                        {getTaskIcon(task.type)}
                      </div>
                      <div className="flex-1 min-w-0">
                        <div className="flex items-center justify-between">
                          <p className="text-sm font-medium text-gray-900 truncate">{task.title}</p>
                          <span className={`inline-flex items-center px-2 py-1 rounded-full text-xs font-medium border ${getPriorityColor(task.priority)}`}>
                            {task.priority.toUpperCase()}
                          </span>
                        </div>
                        <p className="text-sm text-gray-500 mt-1">{task.company}</p>
                        <div className="flex items-center justify-between mt-2">
                          <span className="text-xs text-gray-400">
                            Dibuat: {new Date(task.createdAt).toLocaleDateString("id-ID")}
                          </span>
                          {task.dueDate && (
                            <span className="text-xs text-red-500">
                              Due: {new Date(task.dueDate).toLocaleDateString("id-ID")}
                            </span>
                          )}
                        </div>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
              <div className="px-6 py-3 bg-gray-50 text-center">
                <a
                  href="/staff/tasks"
                  className="text-sm font-medium text-blue-600 hover:text-blue-500"
                >
                  Lihat Semua Tugas →
                </a>
              </div>
            </div>
          </div>

          {/* Assigned Companies & Quick Actions */}
          <div className="space-y-6">
            {/* Assigned Companies */}
            <div className="bg-white rounded-lg shadow">
              <div className="px-6 py-4 border-b border-gray-200">
                <h3 className="text-lg font-medium text-gray-900">Perusahaan Ditugaskan</h3>
              </div>
              <div className="divide-y divide-gray-200 max-h-64 overflow-y-auto">
                {assignedCompanies.slice(0, 5).map((company) => (
                  <div key={company.id} className="px-6 py-3 hover:bg-gray-50">
                    <div className="flex items-center justify-between">
                      <div className="flex-1 min-w-0">
                        <p className="text-sm font-medium text-gray-900 truncate">{company.name}</p>
                        <p className="text-xs text-gray-500">{company.owner}</p>
                      </div>
                      <div className="flex items-center space-x-2">
                        {company.pendingLicenses > 0 && (
                          <span className="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-yellow-100 text-yellow-800">
                            {company.pendingLicenses} pending
                          </span>
                        )}
                        <span className={`inline-flex items-center px-2 py-1 rounded-full text-xs font-medium ${getStatusColor(company.status)}`}>
                          {company.status}
                        </span>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
              <div className="px-6 py-3 bg-gray-50 text-center">
                <a
                  href="/staff/companies"
                  className="text-sm font-medium text-blue-600 hover:text-blue-500"
                >
                  Lihat Semua Perusahaan →
                </a>
              </div>
            </div>

            {/* Quick Actions */}
            <div className="bg-white rounded-lg shadow">
              <div className="px-6 py-4 border-b border-gray-200">
                <h3 className="text-lg font-medium text-gray-900">Quick Actions</h3>
              </div>
              <div className="px-6 py-4 space-y-3">
                <a
                  href="/staff/licenses"
                  className="block w-full text-left px-4 py-2 text-sm font-medium text-gray-700 bg-gray-50 rounded-lg hover:bg-gray-100 transition-colors"
                >
                  Review Perizinan ({stats?.pendingLicenses})
                </a>
                <a
                  href="/staff/documents"
                  className="block w-full text-left px-4 py-2 text-sm font-medium text-gray-700 bg-gray-50 rounded-lg hover:bg-gray-100 transition-colors"
                >
                  Verifikasi Dokumen ({stats?.documentsToReview})
                </a>
                <a
                  href="/staff/support"
                  className="block w-full text-left px-4 py-2 text-sm font-medium text-gray-700 bg-gray-50 rounded-lg hover:bg-gray-100 transition-colors"
                >
                  Handle Support Tickets ({stats?.openTickets})
                </a>
                <a
                  href="/staff/reports"
                  className="block w-full text-left px-4 py-2 text-sm font-medium text-gray-700 bg-gray-50 rounded-lg hover:bg-gray-100 transition-colors"
                >
                  Lihat Laporan Performance
                </a>
              </div>
            </div>

            {/* Performance Summary */}
            <div className="bg-white rounded-lg shadow">
              <div className="px-6 py-4 border-b border-gray-200">
                <h3 className="text-lg font-medium text-gray-900">Performance Summary</h3>
              </div>
              <div className="px-6 py-4 space-y-4">
                <div className="flex items-center justify-between">
                  <span className="text-sm text-gray-600">Completed This Week</span>
                  <span className="text-sm font-medium text-gray-900">{stats?.thisWeekProcessed}</span>
                </div>
                <div className="flex items-center justify-between">
                  <span className="text-sm text-gray-600">Total Completed</span>
                  <span className="text-sm font-medium text-gray-900">{stats?.completedLicenses}</span>
                </div>
                <div className="flex items-center justify-between">
                  <span className="text-sm text-gray-600">Avg. Processing Time</span>
                  <span className="text-sm font-medium text-gray-900">{stats?.averageProcessingTime}h</span>
                </div>
                <div className="flex items-center justify-between">
                  <span className="text-sm text-gray-600">Tickets Resolved</span>
                  <span className="text-sm font-medium text-gray-900">{stats?.resolvedTickets}</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
