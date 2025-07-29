"use client";

import { useAuth } from "@/contexts/AuthContext";
import { useState, useEffect } from "react";
import { 
  CreditCardIcon,
  ClockIcon,
  CheckCircleIcon,
  ExclamationTriangleIcon,
  BanknotesIcon,
  CalendarIcon,
  DocumentTextIcon,
  ArrowPathIcon
} from "@heroicons/react/24/outline";

interface Payment {
  id: string;
  license_type: string;
  amount: number;
  status: "pending" | "paid" | "failed" | "refunded";
  payment_method: string;
  created_at: string;
  paid_at?: string;
  due_date: string;
  transaction_id?: string;
  invoice_number: string;
}

interface PaymentSummary {
  total_paid: number;
  total_pending: number;
  total_failed: number;
  next_due_amount: number;
  next_due_date: string;
}

export default function UmkmPaymentsPage() {
  const { isUmkmOwner } = useAuth();
  const [payments, setPayments] = useState<Payment[]>([]);
  const [summary, setSummary] = useState<PaymentSummary | null>(null);
  const [loading, setLoading] = useState(true);
  const [selectedStatus, setSelectedStatus] = useState("all");

  useEffect(() => {
    // Mock data - replace with actual API call
    setTimeout(() => {
      setPayments([
        {
          id: "1",
          license_type: "SIUP (Surat Izin Usaha Perdagangan)",
          amount: 500000,
          status: "paid",
          payment_method: "Bank Transfer",
          created_at: "2024-01-15T10:30:00Z",
          paid_at: "2024-01-16T09:15:00Z",
          due_date: "2024-01-25T23:59:59Z",
          transaction_id: "TXN001234567890",
          invoice_number: "INV-2024-001"
        },
        {
          id: "2",
          license_type: "TDP (Tanda Daftar Perusahaan)",
          amount: 300000,
          status: "paid",
          payment_method: "E-Wallet",
          created_at: "2024-02-10T14:20:00Z",
          paid_at: "2024-02-10T14:25:00Z",
          due_date: "2024-02-20T23:59:59Z",
          transaction_id: "TXN001234567891",
          invoice_number: "INV-2024-002"
        },
        {
          id: "3",
          license_type: "NPWP Badan",
          amount: 0,
          status: "paid",
          payment_method: "Free",
          created_at: "2024-03-05T11:10:00Z",
          paid_at: "2024-03-05T11:10:00Z",
          due_date: "2024-03-15T23:59:59Z",
          invoice_number: "INV-2024-003"
        },
        {
          id: "4",
          license_type: "IUMK (Izin Usaha Mikro Kecil)",
          amount: 0,
          status: "paid",
          payment_method: "Free",
          created_at: "2024-04-12T16:45:00Z",
          paid_at: "2024-04-12T16:45:00Z",
          due_date: "2024-04-22T23:59:59Z",
          invoice_number: "INV-2024-004"
        },
        {
          id: "5",
          license_type: "Perpanjangan SIUP",
          amount: 250000,
          status: "pending",
          payment_method: "Bank Transfer",
          created_at: "2024-07-20T08:30:00Z",
          due_date: "2024-08-05T23:59:59Z",
          invoice_number: "INV-2024-005"
        }
      ]);

      setSummary({
        total_paid: 800000,
        total_pending: 250000,
        total_failed: 0,
        next_due_amount: 250000,
        next_due_date: "2024-08-05T23:59:59Z"
      });

      setLoading(false);
    }, 1000);
  }, []);

  const filteredPayments = payments.filter(payment => {
    if (selectedStatus === "all") return true;
    return payment.status === selectedStatus;
  });

  const formatCurrency = (amount: number) => {
    if (amount === 0) return "Gratis";
    return new Intl.NumberFormat('id-ID', {
      style: 'currency',
      currency: 'IDR',
      minimumFractionDigits: 0
    }).format(amount);
  };

  const getStatusBadge = (status: string) => {
    const badges = {
      pending: "bg-yellow-100 text-yellow-800 border-yellow-300",
      paid: "bg-green-100 text-green-800 border-green-300",
      failed: "bg-red-100 text-red-800 border-red-300",
      refunded: "bg-blue-100 text-blue-800 border-blue-300"
    };
    
    const labels = {
      pending: "Menunggu Pembayaran",
      paid: "Lunas",
      failed: "Gagal",
      refunded: "Dikembalikan"
    };

    return (
      <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border ${badges[status as keyof typeof badges]}`}>
        {labels[status as keyof typeof labels]}
      </span>
    );
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case "pending":
        return <ClockIcon className="h-5 w-5 text-yellow-500" />;
      case "paid":
        return <CheckCircleIcon className="h-5 w-5 text-green-500" />;
      case "failed":
        return <ExclamationTriangleIcon className="h-5 w-5 text-red-500" />;
      case "refunded":
        return <ArrowPathIcon className="h-5 w-5 text-blue-500" />;
      default:
        return <CreditCardIcon className="h-5 w-5 text-gray-500" />;
    }
  };

  const getDaysUntilDue = (dueDate: string) => {
    const due = new Date(dueDate);
    const now = new Date();
    const diffTime = due.getTime() - now.getTime();
    const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24));
    return diffDays;
  };

  const handlePayNow = (paymentId: string) => {
    // Simulate payment process
    alert(`Memproses pembayaran untuk ID: ${paymentId}`);
  };

  const downloadInvoice = (invoiceNumber: string) => {
    // Simulate invoice download
    alert(`Mengunduh invoice: ${invoiceNumber}`);
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
            <div className="flex items-center">
              <CreditCardIcon className="h-8 w-8 text-blue-600 mr-3" />
              <div>
                <h1 className="text-2xl font-bold text-gray-900">Status Pembayaran</h1>
                <p className="text-sm text-gray-500">Kelola pembayaran perizinan dan tagihan usaha Anda</p>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Payment Summary */}
        {summary && (
          <div className="grid grid-cols-1 md:grid-cols-4 gap-6 mb-8">
            <div className="bg-white rounded-lg shadow p-6">
              <div className="flex items-center">
                <div className="flex-shrink-0">
                  <CheckCircleIcon className="h-8 w-8 text-green-600" />
                </div>
                <div className="ml-4">
                  <p className="text-sm font-medium text-gray-500">Total Terbayar</p>
                  <p className="text-2xl font-semibold text-gray-900">
                    {formatCurrency(summary.total_paid)}
                  </p>
                </div>
              </div>
            </div>

            <div className="bg-white rounded-lg shadow p-6">
              <div className="flex items-center">
                <div className="flex-shrink-0">
                  <ClockIcon className="h-8 w-8 text-yellow-600" />
                </div>
                <div className="ml-4">
                  <p className="text-sm font-medium text-gray-500">Menunggu Pembayaran</p>
                  <p className="text-2xl font-semibold text-gray-900">
                    {formatCurrency(summary.total_pending)}
                  </p>
                </div>
              </div>
            </div>

            <div className="bg-white rounded-lg shadow p-6">
              <div className="flex items-center">
                <div className="flex-shrink-0">
                  <ExclamationTriangleIcon className="h-8 w-8 text-red-600" />
                </div>
                <div className="ml-4">
                  <p className="text-sm font-medium text-gray-500">Pembayaran Gagal</p>
                  <p className="text-2xl font-semibold text-gray-900">
                    {formatCurrency(summary.total_failed)}
                  </p>
                </div>
              </div>
            </div>

            <div className="bg-white rounded-lg shadow p-6">
              <div className="flex items-center">
                <div className="flex-shrink-0">
                  <CalendarIcon className="h-8 w-8 text-purple-600" />
                </div>
                <div className="ml-4">
                  <p className="text-sm font-medium text-gray-500">Jatuh Tempo Terdekat</p>
                  <p className="text-lg font-semibold text-gray-900">
                    {getDaysUntilDue(summary.next_due_date)} hari
                  </p>
                  <p className="text-sm text-gray-500">
                    {formatCurrency(summary.next_due_amount)}
                  </p>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* Filter */}
        <div className="bg-white rounded-lg shadow mb-6">
          <div className="p-6">
            <div className="flex items-center space-x-4">
              <label className="text-sm font-medium text-gray-700">Filter Status:</label>
              <select
                value={selectedStatus}
                onChange={(e) => setSelectedStatus(e.target.value)}
                className="px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-blue-500 focus:border-blue-500"
              >
                <option value="all">Semua Status</option>
                <option value="pending">Menunggu Pembayaran</option>
                <option value="paid">Lunas</option>
                <option value="failed">Gagal</option>
                <option value="refunded">Dikembalikan</option>
              </select>
            </div>
          </div>
        </div>

        {/* Payments List */}
        <div className="bg-white rounded-lg shadow">
          <div className="px-6 py-4 border-b border-gray-200">
            <h2 className="text-lg font-medium text-gray-900">
              Riwayat Pembayaran ({filteredPayments.length})
            </h2>
          </div>
          
          {filteredPayments.length > 0 ? (
            <div className="divide-y divide-gray-200">
              {filteredPayments.map((payment) => (
                <div key={payment.id} className="px-6 py-4 hover:bg-gray-50">
                  <div className="flex items-center justify-between">
                    <div className="flex items-center space-x-4">
                      {getStatusIcon(payment.status)}
                      <div className="flex-1">
                        <div className="flex items-center space-x-2 mb-1">
                          <h3 className="text-sm font-medium text-gray-900">
                            {payment.license_type}
                          </h3>
                          {payment.status === "pending" && getDaysUntilDue(payment.due_date) <= 3 && (
                            <span className="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-red-100 text-red-800">
                              Segera Jatuh Tempo
                            </span>
                          )}
                        </div>
                        <div className="flex items-center space-x-4 text-sm text-gray-500">
                          <span>Invoice: {payment.invoice_number}</span>
                          <span>Jumlah: {formatCurrency(payment.amount)}</span>
                          <span>Metode: {payment.payment_method}</span>
                          {payment.transaction_id && (
                            <span>TXN: {payment.transaction_id}</span>
                          )}
                        </div>
                        <div className="flex items-center space-x-4 text-sm text-gray-500 mt-1">
                          <span>
                            Dibuat: {new Date(payment.created_at).toLocaleDateString('id-ID')}
                          </span>
                          {payment.paid_at && (
                            <span>
                              Dibayar: {new Date(payment.paid_at).toLocaleDateString('id-ID')}
                            </span>
                          )}
                          <span>
                            Jatuh Tempo: {new Date(payment.due_date).toLocaleDateString('id-ID')}
                            {payment.status === "pending" && (
                              <span className="ml-1">
                                ({getDaysUntilDue(payment.due_date)} hari lagi)
                              </span>
                            )}
                          </span>
                        </div>
                      </div>
                    </div>
                    
                    <div className="flex items-center space-x-3">
                      {getStatusBadge(payment.status)}
                      
                      <div className="flex space-x-2">
                        <button
                          onClick={() => downloadInvoice(payment.invoice_number)}
                          className="p-2 text-gray-400 hover:text-blue-600 rounded-full hover:bg-blue-50"
                          title="Download Invoice"
                        >
                          <DocumentTextIcon className="h-5 w-5" />
                        </button>
                        
                        {payment.status === "pending" && payment.amount > 0 && (
                          <button
                            onClick={() => handlePayNow(payment.id)}
                            className="inline-flex items-center px-3 py-1 border border-transparent text-sm font-medium rounded text-white bg-blue-600 hover:bg-blue-700"
                          >
                            <BanknotesIcon className="h-4 w-4 mr-1" />
                            Bayar Sekarang
                          </button>
                        )}
                      </div>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          ) : (
            <div className="px-6 py-12 text-center">
              <CreditCardIcon className="mx-auto h-12 w-12 text-gray-400" />
              <h3 className="mt-2 text-sm font-medium text-gray-900">
                {selectedStatus === "all" ? "Belum ada riwayat pembayaran" : `Tidak ada pembayaran dengan status "${selectedStatus}"`}
              </h3>
              <p className="mt-1 text-sm text-gray-500">
                {selectedStatus === "all" 
                  ? "Pembayaran akan muncul setelah Anda mengajukan perizinan."
                  : "Coba ubah filter status untuk melihat pembayaran lainnya."
                }
              </p>
            </div>
          )}
        </div>

        {/* Payment Methods Info */}
        <div className="mt-8 bg-blue-50 border border-blue-200 rounded-lg p-6">
          <h3 className="text-lg font-medium text-blue-900 mb-4">💳 Metode Pembayaran Tersedia</h3>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm text-blue-700">
            <div>
              <h4 className="font-medium mb-2">Bank Transfer:</h4>
              <ul className="space-y-1">
                <li>• BCA, BNI, BRI, Mandiri</li>
                <li>• Proses 1-2 hari kerja</li>
                <li>• Gratis biaya admin</li>
              </ul>
            </div>
            <div>
              <h4 className="font-medium mb-2">E-Wallet:</h4>
              <ul className="space-y-1">
                <li>• GoPay, OVO, DANA</li>
                <li>• Proses instan</li>
                <li>• Biaya admin 2,5%</li>
              </ul>
            </div>
            <div>
              <h4 className="font-medium mb-2">Virtual Account:</h4>
              <ul className="space-y-1">
                <li>• Semua bank ATM</li>
                <li>• Proses 1 hari kerja</li>
                <li>• Biaya admin Rp 4.000</li>
              </ul>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
