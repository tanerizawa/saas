"use client";

import { useAuth } from "@/contexts/AuthContext";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import Link from "next/link";
import {
  BuildingOffice2Icon,
  DocumentTextIcon,
  CreditCardIcon,
  UsersIcon,
  BellIcon,
  Cog6ToothIcon,
  ChartBarIcon,
  BuildingOfficeIcon,
} from "@heroicons/react/24/outline";

interface DashboardStats {
  totalLicenses: number;
  activeCompanies: number;
  pendingPayments: number;
  activeUsers: number;
}

export default function DashboardPage() {
  const { user, logout, isLoading } = useAuth();
  const router = useRouter();
  const [stats, setStats] = useState<DashboardStats>({
    totalLicenses: 0,
    activeCompanies: 0,
    pendingPayments: 0,
    activeUsers: 0,
  });
  const [statsLoading, setStatsLoading] = useState(true);

  useEffect(() => {
    if (!isLoading && !user) {
      router.push("/auth/login");
    }
  }, [user, isLoading, router]);

  // Load dashboard stats
  useEffect(() => {
    const loadDashboardStats = async () => {
      if (!user) return;
      
      try {
        setStatsLoading(true);
        
        // Mock stats for demo - in real app, fetch from backend
        const mockStats: DashboardStats = {
          totalLicenses: user.role === "super_admin" ? 15 : 3,
          activeCompanies: user.role === "super_admin" ? 8 : 1,
          pendingPayments: user.role === "super_admin" ? 5 : 2,
          activeUsers: user.role === "super_admin" ? 24 : 1,
        };
        
        // Simulate API call delay
        await new Promise(resolve => setTimeout(resolve, 1000));
        
        setStats(mockStats);
        console.log("📊 Dashboard stats loaded:", mockStats);
      } catch (error) {
        console.error("Failed to load dashboard stats:", error);
        // Set default stats on error
        setStats({
          totalLicenses: 0,
          activeCompanies: 0,
          pendingPayments: 0,
          activeUsers: 0,
        });
      } finally {
        setStatsLoading(false);
      }
    };

    loadDashboardStats();
  }, [user]);

  const handleLogout = async () => {
    try {
      await logout();
      router.push("/auth/login");
    } catch (error) {
      console.error("Logout error:", error);
    }
  };

  if (isLoading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="animate-spin rounded-full h-32 w-32 border-b-2 border-blue-600"></div>
      </div>
    );
  }

  if (!user) {
    return null; // Will redirect to login
  }

  const dashboardCards = [
    {
      title: "Perizinan Usaha",
      description: "Kelola NIB, SIUP, TDP, dan izin lainnya",
      icon: DocumentTextIcon,
      href: "/licensing",
      color: "bg-blue-500",
    },
    {
      title: "Profil Perusahaan",
      description: "Informasi dan data perusahaan",
      icon: BuildingOfficeIcon,
      href: "/companies",
      color: "bg-green-500",
    },
    {
      title: "Keuangan",
      description: "Pajak dan laporan keuangan",
      icon: CreditCardIcon,
      href: "/finance",
      color: "bg-yellow-500",
    },
    {
      title: "Laporan",
      description: "Laporan dan analisis bisnis",
      icon: ChartBarIcon,
      href: "/reports",
      color: "bg-indigo-500",
    },
    {
      title: "Manajemen User",
      description: "Kelola pengguna dan hak akses",
      icon: UsersIcon,
      href: "/users",
      color: "bg-purple-500",
      adminOnly: true,
    },
  ];

  // Fix role-based filtering - include super_admin role
  const isAdmin = user.role === "super_admin" || user.role === "admin_staff";
  const filteredCards = dashboardCards.filter(
    (card) => !card.adminOnly || isAdmin
  );

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Header */}
      <header className="bg-white shadow">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center py-6">
            <div>
              <h1 className="text-3xl font-bold text-gray-900">
                Dashboard SaaS UMKM
              </h1>
              <p className="mt-1 text-sm text-gray-600">
                Selamat datang, {user.full_name}
              </p>
            </div>
            <div className="flex items-center space-x-4">
              <button className="p-2 text-gray-400 hover:text-gray-500">
                <BellIcon className="h-6 w-6" />
              </button>
              <button className="p-2 text-gray-400 hover:text-gray-500">
                <Cog6ToothIcon className="h-6 w-6" />
              </button>
              <div className="flex items-center space-x-3">
                <div className="flex-shrink-0">
                  <div className="h-10 w-10 rounded-full bg-blue-500 flex items-center justify-center">
                    <span className="text-white font-medium">
                      {user.full_name.charAt(0).toUpperCase()}
                    </span>
                  </div>
                </div>
                <div className="hidden md:block">
                  <div className="text-sm font-medium text-gray-900">
                    {user.full_name}
                  </div>
                  <div className="text-sm text-gray-500">
                    {user.role === "super_admin" ? "Super Administrator" : 
                     user.role === "admin_staff" ? "Admin Staff" : "UMKM Owner"}
                  </div>
                </div>
                <button
                  onClick={handleLogout}
                  className="text-sm text-red-600 hover:text-red-500 font-medium"
                >
                  Logout
                </button>
              </div>
            </div>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
        <div className="px-4 py-6 sm:px-0">
          {/* Stats Overview */}
          <div className="grid grid-cols-1 md:grid-cols-4 gap-6 mb-8">
            <div className="bg-white overflow-hidden shadow rounded-lg">
              <div className="p-5">
                <div className="flex items-center">
                  <div className="flex-shrink-0">
                    <DocumentTextIcon className="h-6 w-6 text-gray-400" />
                  </div>
                  <div className="ml-5 w-0 flex-1">
                    <dl>
                      <dt className="text-sm font-medium text-gray-500 truncate">
                        Total Izin
                      </dt>
                      <dd className="text-lg font-medium text-gray-900">
                        {statsLoading ? "..." : stats.totalLicenses}
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
                    <BuildingOffice2Icon className="h-6 w-6 text-gray-400" />
                  </div>
                  <div className="ml-5 w-0 flex-1">
                    <dl>
                      <dt className="text-sm font-medium text-gray-500 truncate">
                        Perusahaan Aktif
                      </dt>
                      <dd className="text-lg font-medium text-gray-900">
                        {statsLoading ? "..." : stats.activeCompanies}
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
                    <CreditCardIcon className="h-6 w-6 text-gray-400" />
                  </div>
                  <div className="ml-5 w-0 flex-1">
                    <dl>
                      <dt className="text-sm font-medium text-gray-500 truncate">
                        Pembayaran Pending
                      </dt>
                      <dd className="text-lg font-medium text-gray-900">
                        {statsLoading ? "..." : stats.pendingPayments}
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
                    <UsersIcon className="h-6 w-6 text-gray-400" />
                  </div>
                  <div className="ml-5 w-0 flex-1">
                    <dl>
                      <dt className="text-sm font-medium text-gray-500 truncate">
                        User Aktif
                      </dt>
                      <dd className="text-lg font-medium text-gray-900">
                        {statsLoading ? "..." : stats.activeUsers}
                      </dd>
                    </dl>
                  </div>
                </div>
              </div>
            </div>
          </div>

          {/* Quick Actions */}
          <div className="mb-8">
            <h2 className="text-lg leading-6 font-medium text-gray-900 mb-4">
              Aksi Cepat
            </h2>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
              {filteredCards.map((card) => {
                const IconComponent = card.icon;
                return (
                  <Link
                    key={card.title}
                    href={card.href}
                    className="bg-white overflow-hidden shadow rounded-lg hover:shadow-md transition-shadow cursor-pointer block"
                  >
                    <div className="p-6">
                      <div className="flex items-center">
                        <div
                          className={`flex-shrink-0 p-3 rounded-lg ${card.color}`}
                        >
                          <IconComponent className="h-6 w-6 text-white" />
                        </div>
                        <div className="ml-4">
                          <h3 className="text-lg font-medium text-gray-900">
                            {card.title}
                          </h3>
                          <p className="text-sm text-gray-500">
                            {card.description}
                          </p>
                        </div>
                      </div>
                    </div>
                  </Link>
                );
              })}
            </div>
          </div>

          {/* Recent Activity */}
          <div className="bg-white shadow rounded-lg">
            <div className="px-6 py-4 border-b border-gray-200">
              <h3 className="text-lg leading-6 font-medium text-gray-900">
                Aktivitas Terbaru
              </h3>
            </div>
            <div className="px-6 py-4">
              <div className="flow-root">
                <ul className="-mb-8">
                  <li>
                    <div className="relative pb-8">
                      <div className="relative flex space-x-3">
                        <div>
                          <span className="h-8 w-8 rounded-full bg-blue-500 flex items-center justify-center ring-8 ring-white">
                            <DocumentTextIcon className="h-5 w-5 text-white" />
                          </span>
                        </div>
                        <div className="min-w-0 flex-1 pt-1.5 flex justify-between space-x-4">
                          <div>
                            <p className="text-sm text-gray-500">
                              Pendaftaran NIB berhasil disubmit{" "}
                              <span className="font-medium text-gray-900">
                                #NIB202412001
                              </span>
                            </p>
                          </div>
                          <div className="text-right text-sm whitespace-nowrap text-gray-500">
                            <time dateTime="2024-12-09">2 jam yang lalu</time>
                          </div>
                        </div>
                      </div>
                    </div>
                  </li>
                  <li>
                    <div className="relative pb-8">
                      <div className="relative flex space-x-3">
                        <div>
                          <span className="h-8 w-8 rounded-full bg-green-500 flex items-center justify-center ring-8 ring-white">
                            <CreditCardIcon className="h-5 w-5 text-white" />
                          </span>
                        </div>
                        <div className="min-w-0 flex-1 pt-1.5 flex justify-between space-x-4">
                          <div>
                            <p className="text-sm text-gray-500">
                              Pembayaran biaya perizinan{" "}
                              <span className="font-medium text-gray-900">
                                Rp 500.000
                              </span>{" "}
                              berhasil dikonfirmasi
                            </p>
                          </div>
                          <div className="text-right text-sm whitespace-nowrap text-gray-500">
                            <time dateTime="2024-12-08">1 hari yang lalu</time>
                          </div>
                        </div>
                      </div>
                    </div>
                  </li>
                  <li>
                    <div className="relative">
                      <div className="relative flex space-x-3">
                        <div>
                          <span className="h-8 w-8 rounded-full bg-yellow-500 flex items-center justify-center ring-8 ring-white">
                            <UsersIcon className="h-5 w-5 text-white" />
                          </span>
                        </div>
                        <div className="min-w-0 flex-1 pt-1.5 flex justify-between space-x-4">
                          <div>
                            <p className="text-sm text-gray-500">
                              Akun Anda telah diverifikasi dan diaktifkan
                            </p>
                          </div>
                          <div className="text-right text-sm whitespace-nowrap text-gray-500">
                            <time dateTime="2024-12-07">2 hari yang lalu</time>
                          </div>
                        </div>
                      </div>
                    </div>
                  </li>
                </ul>
              </div>
            </div>
          </div>
        </div>
      </main>
    </div>
  );
}
