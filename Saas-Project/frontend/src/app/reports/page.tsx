"use client";

import { useAuth } from "@/contexts/AuthContext";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import Link from "next/link";
import {
  ChartBarIcon,
  DocumentTextIcon,
  ArrowDownTrayIcon,
  CalendarIcon,
  PrinterIcon,
  ShareIcon,
} from "@heroicons/react/24/outline";

interface ReportData {
  id: string;
  title: string;
  description: string;
  type: "financial" | "licensing" | "business" | "tax";
  generated_at: string;
  period: string;
  status: "ready" | "generating" | "error";
  file_size?: string;
}

export default function ReportsPage() {
  const { user, isLoading } = useAuth();
  const router = useRouter();
  const [reports, setReports] = useState<ReportData[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (!isLoading && !user) {
      router.push("/auth/login");
      return;
    }
  }, [user, isLoading, router]);

  useEffect(() => {
    if (user) {
      loadReports();
    }
  }, [user]); // eslint-disable-line react-hooks/exhaustive-deps

  const loadReports = async () => {
    try {
      setLoading(true);
      
      // Mock reports data
      const mockReports: ReportData[] = user?.role === "super_admin" ? [
        {
          id: "rpt-001",
          title: "Laporan Keuangan Bulanan",
          description: "Ringkasan pendapatan, pengeluaran, dan laba rugi bulan Juli 2025",
          type: "financial",
          generated_at: "2025-07-25T10:30:00Z",
          period: "Juli 2025",
          status: "ready",
          file_size: "2.4 MB"
        },
        {
          id: "rpt-002",
          title: "Status Perizinan Perusahaan",
          description: "Daftar semua izin yang telah disetujui, pending, dan ditolak",
          type: "licensing",
          generated_at: "2025-07-24T14:20:00Z",
          period: "Q2 2025",
          status: "ready",
          file_size: "1.8 MB"
        },
        {
          id: "rpt-003",
          title: "Analisis Bisnis Triwulan",
          description: "Performa bisnis dan trend pertumbuhan UMKM",
          type: "business",
          generated_at: "2025-07-23T09:15:00Z",
          period: "Q2 2025",
          status: "ready",
          file_size: "3.1 MB"
        },
        {
          id: "rpt-004",
          title: "Laporan Pajak",
          description: "Perhitungan dan kewajiban pajak untuk periode pelaporan",
          type: "tax",
          generated_at: "2025-07-25T16:45:00Z",
          period: "Juli 2025",
          status: "generating",
        },
        {
          id: "rpt-005",
          title: "Laporan User Activity",
          description: "Aktivitas pengguna dan penggunaan platform",
          type: "business",
          generated_at: "2025-07-22T11:00:00Z",
          period: "Juli 2025",
          status: "ready",
          file_size: "950 KB"
        }
      ] : [
        {
          id: "rpt-001",
          title: "Laporan Keuangan Bulanan",
          description: "Ringkasan keuangan usaha untuk bulan Juli 2025",
          type: "financial",
          generated_at: "2025-07-25T10:30:00Z",
          period: "Juli 2025",
          status: "ready",
          file_size: "1.2 MB"
        },
        {
          id: "rpt-002",
          title: "Status Perizinan",
          description: "Status izin usaha yang dimiliki",
          type: "licensing",
          generated_at: "2025-07-20T14:20:00Z",
          period: "2025",
          status: "ready",
          file_size: "650 KB"
        },
        {
          id: "rpt-003",
          title: "Laporan Pajak Bulanan",
          description: "Perhitungan pajak bulanan",
          type: "tax",
          generated_at: "2025-07-25T16:45:00Z",
          period: "Juli 2025",
          status: "generating",
        }
      ];

      await new Promise(resolve => setTimeout(resolve, 800));
      setReports(mockReports);
      console.log("📊 Reports loaded:", mockReports);
    } catch (error) {
      console.error("Failed to load reports:", error);
    } finally {
      setLoading(false);
    }
  };

  const getReportTypeIcon = (type: string) => {
    switch (type) {
      case "financial":
        return <ChartBarIcon className="h-6 w-6 text-green-500" />;
      case "licensing":
        return <DocumentTextIcon className="h-6 w-6 text-blue-500" />;
      case "business":
        return <ChartBarIcon className="h-6 w-6 text-purple-500" />;
      case "tax":
        return <DocumentTextIcon className="h-6 w-6 text-yellow-500" />;
      default:
        return <DocumentTextIcon className="h-6 w-6 text-gray-500" />;
    }
  };

  const getReportTypeBadge = (type: string) => {
    const baseClass = "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium";
    switch (type) {
      case "financial":
        return `${baseClass} bg-green-100 text-green-800`;
      case "licensing":
        return `${baseClass} bg-blue-100 text-blue-800`;
      case "business":
        return `${baseClass} bg-purple-100 text-purple-800`;
      case "tax":
        return `${baseClass} bg-yellow-100 text-yellow-800`;
      default:
        return `${baseClass} bg-gray-100 text-gray-800`;
    }
  };

  const getReportTypeText = (type: string) => {
    switch (type) {
      case "financial": return "Keuangan";
      case "licensing": return "Perizinan";
      case "business": return "Bisnis";
      case "tax": return "Pajak";
      default: return type;
    }
  };

  const getStatusBadge = (status: string) => {
    const baseClass = "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium";
    switch (status) {
      case "ready":
        return `${baseClass} bg-green-100 text-green-800`;
      case "generating":
        return `${baseClass} bg-yellow-100 text-yellow-800`;
      case "error":
        return `${baseClass} bg-red-100 text-red-800`;
      default:
        return `${baseClass} bg-gray-100 text-gray-800`;
    }
  };

  const getStatusText = (status: string) => {
    switch (status) {
      case "ready": return "Siap";
      case "generating": return "Generating...";
      case "error": return "Error";
      default: return status;
    }
  };

  if (isLoading || loading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="animate-spin rounded-full h-32 w-32 border-b-2 border-blue-600"></div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Header */}
      <div className="bg-white shadow">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center py-6">
            <div>
              <nav className="flex" aria-label="Breadcrumb">
                <ol className="flex items-center space-x-4">
                  <li>
                    <Link href="/dashboard" className="text-gray-400 hover:text-gray-500">
                      Dashboard
                    </Link>
                  </li>
                  <li>
                    <span className="text-gray-400">/</span>
                  </li>
                  <li>
                    <span className="text-gray-900 font-medium">Laporan</span>
                  </li>
                </ol>
              </nav>
              <h1 className="mt-2 text-3xl font-bold text-gray-900">
                Laporan & Analisis
              </h1>
            </div>
            <button className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700">
              <ChartBarIcon className="h-4 w-4 mr-2" />
              Generate Laporan Baru
            </button>
          </div>
        </div>
      </div>

      {/* Main Content */}
      <div className="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
        <div className="px-4 py-6 sm:px-0">
          {/* Stats Cards */}
          <div className="grid grid-cols-1 md:grid-cols-4 gap-6 mb-8">
            <div className="bg-white overflow-hidden shadow rounded-lg">
              <div className="p-5">
                <div className="flex items-center">
                  <div className="flex-shrink-0">
                    <DocumentTextIcon className="h-6 w-6 text-blue-500" />
                  </div>
                  <div className="ml-5 w-0 flex-1">
                    <dl>
                      <dt className="text-sm font-medium text-gray-500 truncate">
                        Total Laporan
                      </dt>
                      <dd className="text-lg font-medium text-gray-900">
                        {reports.length}
                      </dd>
                    </dl>
                  </div>
                </div>
              </div>
            </div>

            <div className="bg-white overflow-hidden shadow rounded-lg">
              <div className="p-5">
                <div className="flex items-center">
                  <div className="flex-shrink-0">
                    <ChartBarIcon className="h-6 w-6 text-green-500" />
                  </div>
                  <div className="ml-5 w-0 flex-1">
                    <dl>
                      <dt className="text-sm font-medium text-gray-500 truncate">
                        Laporan Siap
                      </dt>
                      <dd className="text-lg font-medium text-gray-900">
                        {reports.filter(r => r.status === "ready").length}
                      </dd>
                    </dl>
                  </div>
                </div>
              </div>
            </div>

            <div className="bg-white overflow-hidden shadow rounded-lg">
              <div className="p-5">
                <div className="flex items-center">
                  <div className="flex-shrink-0">
                    <CalendarIcon className="h-6 w-6 text-yellow-500" />
                  </div>
                  <div className="ml-5 w-0 flex-1">
                    <dl>
                      <dt className="text-sm font-medium text-gray-500 truncate">
                        Generating
                      </dt>
                      <dd className="text-lg font-medium text-gray-900">
                        {reports.filter(r => r.status === "generating").length}
                      </dd>
                    </dl>
                  </div>
                </div>
              </div>
            </div>

            <div className="bg-white overflow-hidden shadow rounded-lg">
              <div className="p-5">
                <div className="flex items-center">
                  <div className="flex-shrink-0">
                    <ArrowDownTrayIcon className="h-6 w-6 text-purple-500" />
                  </div>
                  <div className="ml-5 w-0 flex-1">
                    <dl>
                      <dt className="text-sm font-medium text-gray-500 truncate">
                        Total Download
                      </dt>
                      <dd className="text-lg font-medium text-gray-900">
                        {reports.filter(r => r.status === "ready").length * 3}
                      </dd>
                    </dl>
                  </div>
                </div>
              </div>
            </div>
          </div>

          {/* Reports List */}
          <div className="bg-white shadow overflow-hidden sm:rounded-md">
            <div className="px-4 py-5 sm:p-6">
              <h3 className="text-lg leading-6 font-medium text-gray-900 mb-4">
                Daftar Laporan
              </h3>
              
              <div className="space-y-4">
                {reports.map((report) => (
                  <div key={report.id} className="border border-gray-200 rounded-lg p-4 hover:bg-gray-50">
                    <div className="flex items-center justify-between">
                      <div className="flex items-center space-x-4">
                        <div className="flex-shrink-0">
                          {getReportTypeIcon(report.type)}
                        </div>
                        <div className="min-w-0 flex-1">
                          <h4 className="text-lg font-medium text-gray-900">
                            {report.title}
                          </h4>
                          <p className="text-sm text-gray-500 mt-1">
                            {report.description}
                          </p>
                          <div className="flex items-center space-x-4 mt-2 text-sm text-gray-500">
                            <span>
                              Period: {report.period}
                            </span>
                            <span>
                              Generated: {new Date(report.generated_at).toLocaleDateString("id-ID", { 
                                day: "numeric", 
                                month: "short", 
                                year: "numeric",
                                hour: "2-digit",
                                minute: "2-digit"
                              })}
                            </span>
                            {report.file_size && (
                              <span>
                                Size: {report.file_size}
                              </span>
                            )}
                          </div>
                        </div>
                      </div>
                      <div className="flex items-center space-x-3">
                        <span className={getReportTypeBadge(report.type)}>
                          {getReportTypeText(report.type)}
                        </span>
                        <span className={getStatusBadge(report.status)}>
                          {getStatusText(report.status)}
                        </span>
                        {report.status === "ready" && (
                          <div className="flex space-x-2">
                            <button className="text-blue-600 hover:text-blue-900" title="Download">
                              <ArrowDownTrayIcon className="h-5 w-5" />
                            </button>
                            <button className="text-green-600 hover:text-green-900" title="Print">
                              <PrinterIcon className="h-5 w-5" />
                            </button>
                            <button className="text-purple-600 hover:text-purple-900" title="Share">
                              <ShareIcon className="h-5 w-5" />
                            </button>
                          </div>
                        )}
                      </div>
                    </div>
                  </div>
                ))}
              </div>

              {reports.length === 0 && (
                <div className="text-center py-8">
                  <ChartBarIcon className="mx-auto h-12 w-12 text-gray-400" />
                  <h3 className="mt-2 text-sm font-medium text-gray-900">Belum ada laporan</h3>
                  <p className="mt-1 text-sm text-gray-500">
                    Generate laporan pertama untuk analisis bisnis
                  </p>
                  <div className="mt-6">
                    <button className="inline-flex items-center px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700">
                      <ChartBarIcon className="h-4 w-4 mr-2" />
                      Generate Laporan Pertama
                    </button>
                  </div>
                </div>
              )}
            </div>
          </div>

          {/* Quick Report Generation */}
          <div className="mt-8 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
            <div className="bg-white shadow rounded-lg p-6">
              <div className="flex items-center">
                <div className="flex-shrink-0">
                  <ChartBarIcon className="h-8 w-8 text-green-500" />
                </div>
                <div className="ml-4">
                  <h3 className="text-lg font-medium text-gray-900">Laporan Keuangan</h3>
                  <p className="text-sm text-gray-500">Generate laporan profit & loss</p>
                </div>
              </div>
              <div className="mt-4">
                <button className="w-full inline-flex justify-center items-center px-4 py-2 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50">
                  Generate
                </button>
              </div>
            </div>

            <div className="bg-white shadow rounded-lg p-6">
              <div className="flex items-center">
                <div className="flex-shrink-0">
                  <DocumentTextIcon className="h-8 w-8 text-blue-500" />
                </div>
                <div className="ml-4">
                  <h3 className="text-lg font-medium text-gray-900">Status Perizinan</h3>
                  <p className="text-sm text-gray-500">Laporan izin yang dimiliki</p>
                </div>
              </div>
              <div className="mt-4">
                <button className="w-full inline-flex justify-center items-center px-4 py-2 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50">
                  Generate
                </button>
              </div>
            </div>

            <div className="bg-white shadow rounded-lg p-6">
              <div className="flex items-center">
                <div className="flex-shrink-0">
                  <ChartBarIcon className="h-8 w-8 text-purple-500" />
                </div>
                <div className="ml-4">
                  <h3 className="text-lg font-medium text-gray-900">Analisis Bisnis</h3>
                  <p className="text-sm text-gray-500">Trend dan performance review</p>
                </div>
              </div>
              <div className="mt-4">
                <button className="w-full inline-flex justify-center items-center px-4 py-2 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50">
                  Generate
                </button>
              </div>
            </div>

            <div className="bg-white shadow rounded-lg p-6">
              <div className="flex items-center">
                <div className="flex-shrink-0">
                  <DocumentTextIcon className="h-8 w-8 text-yellow-500" />
                </div>
                <div className="ml-4">
                  <h3 className="text-lg font-medium text-gray-900">Laporan Pajak</h3>
                  <p className="text-sm text-gray-500">Perhitungan untuk SPT</p>
                </div>
              </div>
              <div className="mt-4">
                <button className="w-full inline-flex justify-center items-center px-4 py-2 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50">
                  Generate
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
