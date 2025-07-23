"use client";

import { useState } from "react";
import { formatRupiah } from "@/lib/utils";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import {
  CreditCardIcon,
  PiggyBankIcon,
  CoinsIcon,
  BuildingIcon,
} from "lucide-react";

// Types
export interface FinancialAccount {
  id: string;
  name: string;
  type: "Bank" | "Cash" | "Investment" | "Credit" | "Other";
  balance: number;
  currency: string;
  isActive: boolean;
}

// Mock data - akan digantikan dengan API call
const mockAccounts: FinancialAccount[] = [
  {
    id: "1",
    name: "Kas Operasional",
    type: "Cash",
    balance: 5000000,
    currency: "IDR",
    isActive: true,
  },
  {
    id: "2",
    name: "Bank BCA",
    type: "Bank",
    balance: 15000000,
    currency: "IDR",
    isActive: true,
  },
  {
    id: "3",
    name: "Bank Mandiri",
    type: "Bank",
    balance: 22567000,
    currency: "IDR",
    isActive: true,
  },
  {
    id: "4",
    name: "Deposito",
    type: "Investment",
    balance: 50000000,
    currency: "IDR",
    isActive: true,
  },
];

export default function AccountsList() {
  const [accounts, setAccounts] = useState<FinancialAccount[]>(mockAccounts);

  const getAccountIcon = (type: FinancialAccount["type"]) => {
    switch (type) {
      case "Bank":
        return <BuildingIcon className="h-5 w-5 text-blue-500" />;
      case "Cash":
        return <CoinsIcon className="h-5 w-5 text-green-500" />;
      case "Investment":
        return <PiggyBankIcon className="h-5 w-5 text-purple-500" />;
      case "Credit":
        return <CreditCardIcon className="h-5 w-5 text-red-500" />;
      default:
        return <CreditCardIcon className="h-5 w-5 text-gray-500" />;
    }
  };

  const getTotalBalance = () => {
    return accounts.reduce((total, account) => total + account.balance, 0);
  };

  return (
    <div className="space-y-4">
      <Card>
        <CardHeader className="pb-2">
          <CardTitle className="text-lg">Rekening & Saldo</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="space-y-3">
            {accounts.map((account) => (
              <div
                key={account.id}
                className="flex items-center justify-between p-3 rounded-lg border hover:bg-muted/50 cursor-pointer"
              >
                <div className="flex items-center gap-3">
                  {getAccountIcon(account.type)}
                  <div>
                    <p className="font-medium">{account.name}</p>
                    <p className="text-xs text-muted-foreground">
                      {account.type}
                    </p>
                  </div>
                </div>
                <p className="font-semibold">{formatRupiah(account.balance)}</p>
              </div>
            ))}
          </div>

          <div className="mt-4 pt-4 border-t flex items-center justify-between">
            <p className="font-medium">Total</p>
            <p className="font-bold text-lg">
              {formatRupiah(getTotalBalance())}
            </p>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
