"use client";

import { useState, useEffect } from "react";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { useToast } from "@/components/ui/use-toast";

// Akan diganti dengan API call yang sebenarnya nanti
const mockAccounts = [
  { id: "1", name: "Kas", type: "Cash", currency: "IDR", balance: 5000000 },
  {
    id: "2",
    name: "Bank BCA",
    type: "Bank",
    currency: "IDR",
    balance: 15000000,
  },
  {
    id: "3",
    name: "Bank Mandiri",
    type: "Bank",
    currency: "IDR",
    balance: 22567000,
  },
];

// Types
type TransactionType = "Income" | "Expense" | "Transfer";

interface CreateTransactionDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onSuccess: () => void;
}

export default function CreateTransactionDialog({
  open,
  onOpenChange,
  onSuccess,
}: CreateTransactionDialogProps) {
  const { toast } = useToast();

  const [loading, setLoading] = useState(false);
  const [transactionType, setTransactionType] =
    useState<TransactionType>("Income");
  const [amount, setAmount] = useState("");
  const [description, setDescription] = useState("");
  const [accountId, setAccountId] = useState("");
  const [reference, setReference] = useState("");
  const [accounts, setAccounts] = useState(mockAccounts);

  // Fetch accounts on initial load
  useEffect(() => {
    // In the real implementation, we would fetch accounts from API
    // For now, we'll use the mock data
    setAccounts(mockAccounts);
  }, []);

  const resetForm = () => {
    setTransactionType("Income");
    setAmount("");
    setDescription("");
    setAccountId("");
    setReference("");
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!amount || !description || !accountId) {
      toast({
        variant: "destructive",
        title: "Input tidak lengkap",
        description: "Mohon isi semua field yang diperlukan",
      });
      return;
    }

    try {
      setLoading(true);

      // Simulate API call
      await new Promise((resolve) => setTimeout(resolve, 1000));

      toast({
        title: "Transaksi berhasil dibuat",
        description: `${transactionType} sebesar Rp${parseInt(
          amount
        ).toLocaleString("id-ID")} telah ditambahkan`,
      });

      resetForm();
      onSuccess();
    } catch (error) {
      console.error("Error creating transaction:", error);
      toast({
        variant: "destructive",
        title: "Gagal membuat transaksi",
        description: "Terjadi kesalahan saat membuat transaksi",
      });
    } finally {
      setLoading(false);
    }
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[425px]">
        <form onSubmit={handleSubmit}>
          <DialogHeader>
            <DialogTitle>Transaksi Baru</DialogTitle>
            <DialogDescription>
              Buat transaksi keuangan baru. Klik simpan setelah selesai.
            </DialogDescription>
          </DialogHeader>
          <div className="grid gap-4 py-4">
            <div className="grid grid-cols-4 items-center gap-4">
              <Label htmlFor="transaction-type" className="text-right">
                Jenis
              </Label>
              <Select
                value={transactionType}
                onValueChange={(value: TransactionType) =>
                  setTransactionType(value)
                }
              >
                <SelectTrigger className="col-span-3">
                  <SelectValue placeholder="Pilih jenis transaksi" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="Income">Pemasukan</SelectItem>
                  <SelectItem value="Expense">Pengeluaran</SelectItem>
                  <SelectItem value="Transfer">Transfer</SelectItem>
                </SelectContent>
              </Select>
            </div>
            <div className="grid grid-cols-4 items-center gap-4">
              <Label htmlFor="amount" className="text-right">
                Jumlah
              </Label>
              <Input
                id="amount"
                type="number"
                min="0"
                value={amount}
                onChange={(e) => setAmount(e.target.value)}
                placeholder="0"
                className="col-span-3"
              />
            </div>
            <div className="grid grid-cols-4 items-center gap-4">
              <Label htmlFor="description" className="text-right">
                Deskripsi
              </Label>
              <Input
                id="description"
                value={description}
                onChange={(e) => setDescription(e.target.value)}
                placeholder="Deskripsi transaksi"
                className="col-span-3"
              />
            </div>
            <div className="grid grid-cols-4 items-center gap-4">
              <Label htmlFor="account" className="text-right">
                Rekening
              </Label>
              <Select value={accountId} onValueChange={setAccountId}>
                <SelectTrigger className="col-span-3">
                  <SelectValue placeholder="Pilih rekening" />
                </SelectTrigger>
                <SelectContent>
                  {accounts.map((account) => (
                    <SelectItem key={account.id} value={account.id}>
                      {account.name} - Rp
                      {account.balance.toLocaleString("id-ID")}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            <div className="grid grid-cols-4 items-center gap-4">
              <Label htmlFor="reference" className="text-right">
                Referensi
              </Label>
              <Input
                id="reference"
                value={reference}
                onChange={(e) => setReference(e.target.value)}
                placeholder="No. referensi (opsional)"
                className="col-span-3"
              />
            </div>
          </div>
          <DialogFooter>
            <Button
              type="button"
              variant="outline"
              onClick={() => onOpenChange(false)}
            >
              Batal
            </Button>
            <Button type="submit" disabled={loading}>
              {loading ? "Menyimpan..." : "Simpan"}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
