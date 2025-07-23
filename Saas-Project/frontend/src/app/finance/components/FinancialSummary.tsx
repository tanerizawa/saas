"use client";

import { useState, useEffect } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { formatRupiah } from "@/lib/utils";
import {
  ArrowUpIcon,
  ArrowDownIcon,
  RefreshCwIcon,
  WalletIcon,
} from "lucide-react";

// Types
interface FinancialSummaryProps {
  companyId: string;
}

interface SummaryData {
  totalBalance: number;
  monthlyIncome: number;
  monthlyExpense: number;
  pendingTransactions: number;
}

export default function FinancialSummary({ companyId }: FinancialSummaryProps) {
  const [isLoading, setIsLoading] = useState(true);
  const [summaryData, setSummaryData] = useState<SummaryData>({
    totalBalance: 0,
    monthlyIncome: 0,
    monthlyExpense: 0,
    pendingTransactions: 0,
  });

  // Fetch summary data
  useEffect(() => {
    const fetchSummaryData = async () => {
      try {
        setIsLoading(true);
        // Simulate API call
        await new Promise((resolve) => setTimeout(resolve, 1000));

        // Mock data - will be replaced with actual API call
        setSummaryData({
          totalBalance: 42567000,
          monthlyIncome: 8750000,
          monthlyExpense: -2950000,
          pendingTransactions: 2,
        });
      } catch (error) {
        console.error("Error fetching financial summary:", error);
      } finally {
        setIsLoading(false);
      }
    };

    fetchSummaryData();
  }, [companyId]);

  return (
    <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
      <Card>
        <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle className="text-sm font-medium">Saldo Total</CardTitle>
          <WalletIcon className="h-4 w-4 text-muted-foreground" />
        </CardHeader>
        <CardContent>
          {isLoading ? (
            <div className="h-6 w-3/4 animate-pulse rounded bg-muted"></div>
          ) : (
            <div className="text-2xl font-bold">
              {formatRupiah(summaryData.totalBalance)}
            </div>
          )}
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle className="text-sm font-medium">
            Pemasukan Bulan Ini
          </CardTitle>
          <ArrowUpIcon className="h-4 w-4 text-green-500" />
        </CardHeader>
        <CardContent>
          {isLoading ? (
            <div className="h-6 w-3/4 animate-pulse rounded bg-muted"></div>
          ) : (
            <div className="text-2xl font-bold text-green-600">
              {formatRupiah(summaryData.monthlyIncome)}
            </div>
          )}
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle className="text-sm font-medium">
            Pengeluaran Bulan Ini
          </CardTitle>
          <ArrowDownIcon className="h-4 w-4 text-red-500" />
        </CardHeader>
        <CardContent>
          {isLoading ? (
            <div className="h-6 w-3/4 animate-pulse rounded bg-muted"></div>
          ) : (
            <div className="text-2xl font-bold text-red-600">
              {formatRupiah(Math.abs(summaryData.monthlyExpense))}
            </div>
          )}
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle className="text-sm font-medium">
            Transaksi Pending
          </CardTitle>
          <RefreshCwIcon className="h-4 w-4 text-yellow-500" />
        </CardHeader>
        <CardContent>
          {isLoading ? (
            <div className="h-6 w-3/4 animate-pulse rounded bg-muted"></div>
          ) : (
            <div className="text-2xl font-bold">
              {summaryData.pendingTransactions}
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
