"use client";

import { useState, useEffect } from "react";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Button } from "@/components/ui/button";
import { PlusIcon, FileTextIcon, BuildingIcon } from "lucide-react";
import TransactionList from "./components/TransactionList";
import AccountsList from "./components/AccountsList";
import FinancialSummary from "./components/FinancialSummary";
import CreateTransactionDialog from "./components/CreateTransactionDialog";
import CreateAccountDialog from "./components/CreateAccountDialog";

export default function FinanceDashboard() {
  const [activeTab, setActiveTab] = useState("overview");
  const [createTransactionOpen, setCreateTransactionOpen] = useState(false);
  const [createAccountOpen, setCreateAccountOpen] = useState(false);
  const [refreshTrigger, setRefreshTrigger] = useState(0);

  const refreshData = () => {
    setRefreshTrigger((prev) => prev + 1);
  };

  return (
    <div className="container mx-auto py-6">
      <div className="flex justify-between items-center mb-6">
        <div>
          <h1 className="text-3xl font-bold">Keuangan</h1>
          <p className="text-muted-foreground">
            Kelola transaksi, rekening, dan laporan keuangan
          </p>
        </div>
        <div className="flex gap-2">
          <Button variant="outline" onClick={() => setCreateAccountOpen(true)}>
            <BuildingIcon className="mr-2 h-4 w-4" /> Akun Baru
          </Button>
          <Button onClick={() => setCreateTransactionOpen(true)}>
            <PlusIcon className="mr-2 h-4 w-4" /> Transaksi Baru
          </Button>
        </div>
      </div>

      <Tabs
        defaultValue="overview"
        value={activeTab}
        onValueChange={setActiveTab}
        className="w-full"
      >
        <TabsList className="grid grid-cols-4 mb-8">
          <TabsTrigger value="overview">Ringkasan</TabsTrigger>
          <TabsTrigger value="transactions">Transaksi</TabsTrigger>
          <TabsTrigger value="accounts">Rekening</TabsTrigger>
          <TabsTrigger value="reports">Laporan</TabsTrigger>
        </TabsList>

        <TabsContent value="overview">
          <FinancialSummary companyId="1" />

          <div className="mt-8 grid gap-4 md:grid-cols-2 lg:grid-cols-7">
            <div className="lg:col-span-4">
              <Card>
                <CardHeader>
                  <CardTitle>Transaksi Terbaru</CardTitle>
                  <CardDescription>
                    5 transaksi terakhir di semua rekening
                  </CardDescription>
                </CardHeader>
                <CardContent>
                  <TransactionList />
                </CardContent>
              </Card>
            </div>
            <div className="lg:col-span-3">
              <AccountsList />
            </div>
          </div>
        </TabsContent>

        <TabsContent value="transactions">
          <Card>
            <CardHeader>
              <CardTitle>Daftar Transaksi</CardTitle>
              <CardDescription>
                Lihat dan kelola semua transaksi keuangan
              </CardDescription>
            </CardHeader>
            <CardContent>
              <TransactionList />
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="accounts">
          <Card>
            <CardHeader>
              <CardTitle>Rekening Keuangan</CardTitle>
              <CardDescription>
                Kelola rekening bank, kas, dan akun keuangan lainnya
              </CardDescription>
            </CardHeader>
            <CardContent>
              <AccountsList />
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="reports">
          <Card>
            <CardHeader>
              <CardTitle>Laporan Keuangan</CardTitle>
              <CardDescription>
                Lihat dan unduh laporan keuangan
              </CardDescription>
            </CardHeader>
            <CardContent>
              <div className="py-12 text-center">
                <FileTextIcon className="mx-auto h-12 w-12 text-muted-foreground" />
                <h3 className="mt-4 text-lg font-medium">Laporan Keuangan</h3>
                <p className="mt-2 text-sm text-muted-foreground">
                  Fitur laporan keuangan akan tersedia pada pembaruan berikutnya
                </p>
                <Button variant="outline" className="mt-4" disabled>
                  Akan Datang
                </Button>
              </div>
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>

      <CreateTransactionDialog
        open={createTransactionOpen}
        onOpenChange={setCreateTransactionOpen}
        onSuccess={() => {
          refreshData();
          setCreateTransactionOpen(false);
        }}
      />

      <CreateAccountDialog
        open={createAccountOpen}
        onOpenChange={setCreateAccountOpen}
        onSuccess={() => {
          refreshData();
          setCreateAccountOpen(false);
        }}
      />
    </div>
  );
}
