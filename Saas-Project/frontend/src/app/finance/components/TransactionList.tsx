"use client";

import { useState } from "react";
import { formatRupiah } from "@/lib/utils";
import { Button } from "@/components/ui/button";
import {
  ArrowUpIcon,
  ArrowDownIcon,
  ArrowRightIcon,
  PlusIcon,
} from "lucide-react";
import CreateTransactionDialog from "./CreateTransactionDialog";

// Types
export interface Transaction {
  id: string;
  date: string;
  type: "Income" | "Expense" | "Transfer";
  description: string;
  amount: number;
  account: string;
  category?: string;
  status: "Pending" | "Completed" | "Failed";
}

// Mock data - akan digantikan dengan API call
const mockTransactions: Transaction[] = [
  {
    id: "1",
    date: "2025-07-20",
    type: "Income",
    description: "Pembayaran dari PT ABC",
    amount: 5000000,
    account: "Bank BCA",
    category: "Penjualan",
    status: "Completed",
  },
  {
    id: "2",
    date: "2025-07-19",
    type: "Expense",
    description: "Pembayaran sewa kantor",
    amount: -2500000,
    account: "Bank Mandiri",
    category: "Sewa",
    status: "Completed",
  },
  {
    id: "3",
    date: "2025-07-18",
    type: "Transfer",
    description: "Transfer ke rekening operasional",
    amount: -1000000,
    account: "Bank BCA",
    status: "Pending",
  },
  {
    id: "4",
    date: "2025-07-15",
    type: "Income",
    description: "Pembayaran dari PT XYZ",
    amount: 3750000,
    account: "Kas",
    category: "Penjualan",
    status: "Completed",
  },
  {
    id: "5",
    date: "2025-07-12",
    type: "Expense",
    description: "Pembelian perlengkapan kantor",
    amount: -450000,
    account: "Bank BCA",
    category: "Perlengkapan",
    status: "Completed",
  },
];

export default function TransactionList() {
  const [transactions, setTransactions] =
    useState<Transaction[]>(mockTransactions);
  const [createDialogOpen, setCreateDialogOpen] = useState(false);

  const getTypeIcon = (type: Transaction["type"]) => {
    switch (type) {
      case "Income":
        return <ArrowUpIcon className="h-4 w-4 text-green-500" />;
      case "Expense":
        return <ArrowDownIcon className="h-4 w-4 text-red-500" />;
      case "Transfer":
        return <ArrowRightIcon className="h-4 w-4 text-blue-500" />;
    }
  };

  const getAmountColor = (amount: number) => {
    if (amount > 0) return "text-green-600";
    if (amount < 0) return "text-red-600";
    return "";
  };

  const getStatusBadge = (status: Transaction["status"]) => {
    switch (status) {
      case "Completed":
        return (
          <span className="inline-flex items-center rounded-full bg-green-100 px-2 py-1 text-xs font-medium text-green-700">
            Selesai
          </span>
        );
      case "Pending":
        return (
          <span className="inline-flex items-center rounded-full bg-yellow-100 px-2 py-1 text-xs font-medium text-yellow-700">
            Menunggu
          </span>
        );
      case "Failed":
        return (
          <span className="inline-flex items-center rounded-full bg-red-100 px-2 py-1 text-xs font-medium text-red-700">
            Gagal
          </span>
        );
    }
  };

  const handleTransactionCreated = () => {
    setCreateDialogOpen(false);
    // In a real app, we would fetch the updated list of transactions here
    // For now, we'll just close the dialog
  };

  return (
    <div className="rounded-md border">
      <div className="flex items-center justify-between p-4 border-b">
        <h2 className="text-lg font-medium">Transaksi Terbaru</h2>
        <Button onClick={() => setCreateDialogOpen(true)} size="sm">
          <PlusIcon className="h-4 w-4 mr-1" />
          Transaksi Baru
        </Button>
      </div>

      <div className="overflow-x-auto">
        <table className="min-w-full divide-y divide-gray-200">
          <thead className="bg-gray-50">
            <tr>
              <th
                scope="col"
                className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                Tanggal
              </th>
              <th
                scope="col"
                className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                Jenis
              </th>
              <th
                scope="col"
                className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                Deskripsi
              </th>
              <th
                scope="col"
                className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                Jumlah
              </th>
              <th
                scope="col"
                className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                Rekening
              </th>
              <th
                scope="col"
                className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                Status
              </th>
            </tr>
          </thead>
          <tbody className="bg-white divide-y divide-gray-200">
            {transactions.map((transaction) => (
              <tr key={transaction.id} className="hover:bg-gray-50">
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                  {new Date(transaction.date).toLocaleDateString("id-ID")}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                  <div className="flex items-center">
                    {getTypeIcon(transaction.type)}
                    <span className="ml-1">
                      {transaction.type === "Income"
                        ? "Pemasukan"
                        : transaction.type === "Expense"
                        ? "Pengeluaran"
                        : "Transfer"}
                    </span>
                  </div>
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                  {transaction.description}
                  {transaction.category && (
                    <span className="ml-2 text-xs text-gray-500">
                      ({transaction.category})
                    </span>
                  )}
                </td>
                <td
                  className={`px-6 py-4 whitespace-nowrap text-sm font-medium ${getAmountColor(
                    transaction.amount
                  )}`}
                >
                  {formatRupiah(Math.abs(transaction.amount))}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                  {transaction.account}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                  {getStatusBadge(transaction.status)}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      <CreateTransactionDialog
        open={createDialogOpen}
        onOpenChange={setCreateDialogOpen}
        onSuccess={handleTransactionCreated}
      />
    </div>
  );
}
