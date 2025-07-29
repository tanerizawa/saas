"use client";

import { useAuth } from "@/contexts/AuthContext";
import { useState, useEffect } from "react";
import { 
  ChartBarIcon,
  CurrencyDollarIcon,
  DocumentChartBarIcon,
  CalendarIcon,
  ArrowDownTrayIcon,
  ExclamationTriangleIcon
} from "@heroicons/react/24/outline";

interface FinancialData {
  month: string;
  revenue: number;
  expenses: number;
  profit: number;
}

interface LicenseCost {
  license_type: string;
  cost: number;
  date: string;
  status: "paid" | "pending" | "overdue";
}

export default function UmkmReportsPage() {
  const { isUmkmOwner } = useAuth();
  const [loading, setLoading] = useState(true);
  const [selectedPeriod, setSelectedPeriod] = useState("2024");
  const [reportType, setReportType] = useState("financial");
  
  const [financialData, setFinancialData] = useState<FinancialData[]>([]);
  const [licenseCosts, setLicenseCosts] = useState<LicenseCost[]>([]);
  const [summary, setSummary] = useState({
    totalRevenue: 0,
    totalExpenses: 0,
    totalProfit: 0,
    licensingCosts: 0
  });

  useEffect(() => {
    // Mock data - replace with actual API call
    setTimeout(() => {
      setFinancialData([
        { month: "Jan 2024", revenue: 25000000, expenses: 18000000, profit: 7000000 },
        { month: "Feb 2024", revenue: 28000000, expenses: 19000000, profit: 9000000 },
        { month: "Mar 2024", revenue: 32000000, expenses: 21000000, profit: 11000000 },
        { month: "Apr 2024", revenue: 30000000, expenses: 20000000, profit: 10000000 },
        { month: "May 2024", revenue: 35000000, expenses: 22000000, profit: 13000000 },
        { month: "Jun 2024", revenue: 38000000, expenses: 24000000, profit: 14000000 }
      ]);

      setLicenseCosts([
        { license_type: "SIUP", cost: 500000, date: "2024-01-15", status: "paid" },
        { license_type: "TDP", cost: 300000, date: "2024-02-10", status: "paid" },
        { license_type: "NPWP", cost: 0, date: "2024-03-05", status: "paid" },
        { license_type: "IUMK", cost: 0, date: "2024-04-12", status: "paid" }
      ]);

      setSummary({
        totalRevenue: 188000000,
        totalExpenses: 124000000,
        totalProfit: 64000000,
        licensingCosts: 800000
      });

      setLoading(false);
    }, 1000);
  }, [selectedPeriod]);

  const formatCurrency = (amount: number) => {
    return new Intl.NumberFormat('id-ID', {
      style: 'currency',
      currency: 'IDR',
      minimumFractionDigits: 0
    }).format(amount);
  };

  const getStatusBadge = (status: string) => {
    const badges = {
      paid: "bg-green-100 text-green-800 border-green-300",
      pending: "bg-yellow-100 text-yellow-800 border-yellow-300",
      overdue: "bg-red-100 text-red-800 border-red-300"
    };
    
    const labels = {
      paid: "Lunas",
      pending: "Pending",
      overdue: "Terlambat"
    };

    return (
      <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border ${badges[status as keyof typeof badges]}`}>
        {labels[status as keyof typeof labels]}
      </span>
    );
  };

  const downloadReport = () => {
    // Simulate report download
    alert("Laporan sedang diunduh...");
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
                <DocumentChartBarIcon className="h-8 w-8 text-blue-600 mr-3" />
                <div>
                  <h1 className="text-2xl font-bold text-gray-900">Laporan Keuangan</h1>
                  <p className="text-sm text-gray-500">Analisis keuangan dan biaya perizinan usaha Anda</p>
                </div>
              </div>
              <div className="flex items-center space-x-4">
                <select
                  value={selectedPeriod}
                  onChange={(e) => setSelectedPeriod(e.target.value)}
                  className="px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                >
                  <option value="2024">2024</option>
                  <option value="2023">2023</option>
                  <option value="2022">2022</option>
                </select>
                <button
                  onClick={downloadReport}
                  className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700"
                >
                  <ArrowDownTrayIcon className="h-4 w-4 mr-2" />
                  Download Laporan
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Report Type Tabs */}
        <div className="mb-8">
          <nav className="flex space-x-8">
            <button
              onClick={() => setReportType("financial")}
              className={`py-2 px-1 border-b-2 font-medium text-sm ${
                reportType === "financial"
                  ? "border-blue-500 text-blue-600"
                  : "border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300"
              }`}
            >
              Laporan Keuangan
            </button>
            <button
              onClick={() => setReportType("licensing")}
              className={`py-2 px-1 border-b-2 font-medium text-sm ${
                reportType === "licensing"
                  ? "border-blue-500 text-blue-600"
                  : "border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300"
              }`}
            >
              Biaya Perizinan
            </button>
          </nav>
        </div>

        {reportType === "financial" && (
          <>
            {/* Financial Summary */}
            <div className="grid grid-cols-1 md:grid-cols-4 gap-6 mb-8">
              <div className="bg-white rounded-lg shadow p-6">
                <div className="flex items-center">
                  <div className="flex-shrink-0">
                    <CurrencyDollarIcon className="h-8 w-8 text-green-600" />
                  </div>
                  <div className="ml-4">
                    <p className="text-sm font-medium text-gray-500">Total Pendapatan</p>
                    <p className="text-2xl font-semibold text-gray-900">
                      {formatCurrency(summary.totalRevenue)}
                    </p>
                  </div>
                </div>
              </div>

              <div className="bg-white rounded-lg shadow p-6">
                <div className="flex items-center">
                  <div className="flex-shrink-0">
                    <CurrencyDollarIcon className="h-8 w-8 text-red-600" />
                  </div>
                  <div className="ml-4">
                    <p className="text-sm font-medium text-gray-500">Total Pengeluaran</p>
                    <p className="text-2xl font-semibold text-gray-900">
                      {formatCurrency(summary.totalExpenses)}
                    </p>
                  </div>
                </div>
              </div>

              <div className="bg-white rounded-lg shadow p-6">
                <div className="flex items-center">
                  <div className="flex-shrink-0">
                    <ChartBarIcon className="h-8 w-8 text-blue-600" />
                  </div>
                  <div className="ml-4">
                    <p className="text-sm font-medium text-gray-500">Total Keuntungan</p>
                    <p className="text-2xl font-semibold text-gray-900">
                      {formatCurrency(summary.totalProfit)}
                    </p>
                  </div>
                </div>
              </div>

              <div className="bg-white rounded-lg shadow p-6">
                <div className="flex items-center">
                  <div className="flex-shrink-0">
                    <DocumentChartBarIcon className="h-8 w-8 text-purple-600" />
                  </div>
                  <div className="ml-4">
                    <p className="text-sm font-medium text-gray-500">Biaya Perizinan</p>
                    <p className="text-2xl font-semibold text-gray-900">
                      {formatCurrency(summary.licensingCosts)}
                    </p>
                  </div>
                </div>
              </div>
            </div>

            {/* Monthly Financial Chart */}
            <div className="bg-white rounded-lg shadow mb-8">
              <div className="px-6 py-4 border-b border-gray-200">
                <h2 className="text-lg font-medium text-gray-900">Tren Keuangan Bulanan</h2>
              </div>
              <div className="p-6">
                <div className="space-y-4">
                  {financialData.map((data, index) => (
                    <div key={index} className="border border-gray-200 rounded-lg p-4">
                      <div className="flex justify-between items-center mb-2">
                        <h4 className="font-medium text-gray-900">{data.month}</h4>
                        <span className={`text-sm font-medium ${
                          data.profit > 0 ? 'text-green-600' : 'text-red-600'
                        }`}>
                          {data.profit > 0 ? 'Profit' : 'Rugi'}: {formatCurrency(Math.abs(data.profit))}
                        </span>
                      </div>
                      <div className="grid grid-cols-3 gap-4 text-sm">
                        <div>
                          <span className="text-gray-500">Pendapatan:</span>
                          <div className="font-medium text-green-600">{formatCurrency(data.revenue)}</div>
                        </div>
                        <div>
                          <span className="text-gray-500">Pengeluaran:</span>
                          <div className="font-medium text-red-600">{formatCurrency(data.expenses)}</div>
                        </div>
                        <div>
                          <span className="text-gray-500">Margin:</span>
                          <div className="font-medium text-blue-600">
                            {((data.profit / data.revenue) * 100).toFixed(1)}%
                          </div>
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            </div>
          </>
        )}

        {reportType === "licensing" && (
          <>
            {/* Licensing Costs Summary */}
            <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
              <div className="bg-white rounded-lg shadow p-6">
                <div className="flex items-center">
                  <div className="flex-shrink-0">
                    <DocumentChartBarIcon className="h-8 w-8 text-blue-600" />
                  </div>
                  <div className="ml-4">
                    <p className="text-sm font-medium text-gray-500">Total Biaya Perizinan</p>
                    <p className="text-2xl font-semibold text-gray-900">
                      {formatCurrency(summary.licensingCosts)}
                    </p>
                  </div>
                </div>
              </div>

              <div className="bg-white rounded-lg shadow p-6">
                <div className="flex items-center">
                  <div className="flex-shrink-0">
                    <CalendarIcon className="h-8 w-8 text-green-600" />
                  </div>
                  <div className="ml-4">
                    <p className="text-sm font-medium text-gray-500">Perizinan Selesai</p>
                    <p className="text-2xl font-semibold text-gray-900">
                      {licenseCosts.filter(l => l.status === 'paid').length}
                    </p>
                  </div>
                </div>
              </div>

              <div className="bg-white rounded-lg shadow p-6">
                <div className="flex items-center">
                  <div className="flex-shrink-0">
                    <CurrencyDollarIcon className="h-8 w-8 text-yellow-600" />
                  </div>
                  <div className="ml-4">
                    <p className="text-sm font-medium text-gray-500">Rata-rata Biaya</p>
                    <p className="text-2xl font-semibold text-gray-900">
                      {formatCurrency(summary.licensingCosts / licenseCosts.length)}
                    </p>
                  </div>
                </div>
              </div>
            </div>

            {/* Licensing Costs Detail */}
            <div className="bg-white rounded-lg shadow">
              <div className="px-6 py-4 border-b border-gray-200">
                <h2 className="text-lg font-medium text-gray-900">Detail Biaya Perizinan</h2>
              </div>
              <div className="divide-y divide-gray-200">
                {licenseCosts.map((license, index) => (
                  <div key={index} className="px-6 py-4">
                    <div className="flex items-center justify-between">
                      <div className="flex items-center space-x-4">
                        <DocumentChartBarIcon className="h-6 w-6 text-blue-600" />
                        <div>
                          <h3 className="text-sm font-medium text-gray-900">{license.license_type}</h3>
                          <p className="text-sm text-gray-500">
                            Tanggal: {new Date(license.date).toLocaleDateString('id-ID')}
                          </p>
                        </div>
                      </div>
                      <div className="flex items-center space-x-4">
                        <div className="text-right">
                          <p className="text-sm font-medium text-gray-900">
                            {license.cost === 0 ? 'Gratis' : formatCurrency(license.cost)}
                          </p>
                        </div>
                        {getStatusBadge(license.status)}
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </>
        )}

        {/* Financial Tips */}
        <div className="mt-8 bg-blue-50 border border-blue-200 rounded-lg p-6">
          <h3 className="text-lg font-medium text-blue-900 mb-4">💡 Tips Finansial</h3>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4 text-sm text-blue-700">
            <div>
              <h4 className="font-medium mb-2">Optimasi Keuntungan:</h4>
              <ul className="space-y-1">
                <li>• Monitor margin keuntungan bulanan</li>
                <li>• Identifikasi bulan dengan performa terbaik</li>
                <li>• Analisis pola pengeluaran untuk efisiensi</li>
              </ul>
            </div>
            <div>
              <h4 className="font-medium mb-2">Manajemen Perizinan:</h4>
              <ul className="space-y-1">
                <li>• Rencanakan perpanjangan perizinan</li>
                <li>• Manfaatkan perizinan gratis (NPWP, IUMK)</li>
                <li>• Simpan dokumen untuk audit keuangan</li>
              </ul>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
