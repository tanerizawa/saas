"use client";

import { useAuth } from "@/contexts/AuthContext";
import { useState } from "react";
import Link from "next/link";
import { usePathname } from "next/navigation";
import {
  HomeIcon,
  UsersIcon,
  DocumentTextIcon,
  CogIcon,
  BellIcon,
  ClipboardDocumentListIcon,
  ChartBarIcon,
  CurrencyDollarIcon,
  ShieldCheckIcon,
  Bars3Icon,
  XMarkIcon,
  ArrowRightOnRectangleIcon
} from "@heroicons/react/24/outline";

interface NavigationItem {
  name: string;
  href: string;
  icon: React.ComponentType<React.SVGProps<SVGSVGElement>>;
  description: string;
}

const navigation: NavigationItem[] = [
  {
    name: "Dashboard",
    href: "/admin",
    icon: HomeIcon,
    description: "Overview dan statistik sistem"
  },
  {
    name: "Manajemen User",
    href: "/admin/users",
    icon: UsersIcon,
    description: "Kelola user UMKM dan staff"
  },
  {
    name: "Perizinan",
    href: "/admin/licenses",
    icon: ShieldCheckIcon,
    description: "Review dan approve perizinan"
  },
  {
    name: "Dokumen",
    href: "/admin/documents",
    icon: DocumentTextIcon,
    description: "Manajemen dokumen sistem"
  },
  {
    name: "Pembayaran",
    href: "/admin/payments",
    icon: CurrencyDollarIcon,
    description: "Monitor transaksi dan pembayaran"
  },
  {
    name: "Laporan",
    href: "/admin/reports",
    icon: ChartBarIcon,
    description: "Analytics dan business intelligence"
  },
  {
    name: "Notifikasi Email",
    href: "/admin/notifications",
    icon: BellIcon,
    description: "Template dan pengaturan email"
  },
  {
    name: "Audit Logs",
    href: "/admin/audit-logs",
    icon: ClipboardDocumentListIcon,
    description: "Track aktivitas dan keamanan sistem"
  },
  {
    name: "Pengaturan",
    href: "/admin/settings",
    icon: CogIcon,
    description: "Konfigurasi sistem global"
  }
];

export default function AdminLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const { user, logout, isSuperAdmin } = useAuth();
  const pathname = usePathname();
  const [sidebarOpen, setSidebarOpen] = useState(false);

  // Filter menu berdasarkan role user
  const filteredNavigation = navigation.filter(item => {
    // Beberapa menu hanya untuk Super Admin
    const superAdminOnly = ["/admin/notifications", "/admin/audit-logs", "/admin/settings"];
    if (superAdminOnly.includes(item.href) && !isSuperAdmin()) {
      return false;
    }
    return true;
  });

  const handleLogout = () => {
    logout();
  };

  return (
    <div className="h-screen flex overflow-hidden bg-gray-100">
      {/* Mobile sidebar backdrop */}
      {sidebarOpen && (
        <div 
          className="fixed inset-0 flex z-40 md:hidden"
          onClick={() => setSidebarOpen(false)}
        >
          <div className="fixed inset-0 bg-gray-600 bg-opacity-75" />
        </div>
      )}

      {/* Sidebar */}
      <div className={`${
        sidebarOpen ? "translate-x-0" : "-translate-x-full"
      } fixed inset-y-0 left-0 z-50 w-64 bg-white shadow-lg transform transition-transform duration-300 ease-in-out md:translate-x-0 md:static md:inset-0`}>
        
        {/* Sidebar Header */}
        <div className="flex items-center justify-between h-16 px-4 bg-blue-600">
          <div className="flex items-center">
            <div className="flex-shrink-0">
              <div className="h-8 w-8 bg-white rounded flex items-center justify-center">
                <span className="text-blue-600 font-bold text-sm">SA</span>
              </div>
            </div>
            <div className="ml-2">
              <div className="text-white font-semibold text-sm">SaaS Admin</div>
              <div className="text-blue-200 text-xs">Control Panel</div>
            </div>
          </div>
          <button
            onClick={() => setSidebarOpen(false)}
            className="md:hidden text-white hover:text-blue-200"
          >
            <XMarkIcon className="h-6 w-6" />
          </button>
        </div>

        {/* User Info */}
        <div className="px-4 py-4 border-b border-gray-200">
          <div className="flex items-center">
            <div className="flex-shrink-0">
              <div className="h-10 w-10 bg-gray-300 rounded-full flex items-center justify-center">
                <span className="text-gray-600 font-medium text-sm">
                  {user?.name?.charAt(0).toUpperCase() || 'A'}
                </span>
              </div>
            </div>
            <div className="ml-3">
              <div className="text-sm font-medium text-gray-900">{user?.name || 'Admin'}</div>
              <div className="text-xs text-gray-500 capitalize">
                {user?.role?.replace('_', ' ') || 'Administrator'}
              </div>
            </div>
          </div>
        </div>

        {/* Navigation */}
        <nav className="mt-2 flex-1 px-2 space-y-1 overflow-y-auto">
          {filteredNavigation.map((item) => {
            const isActive = pathname === item.href;
            return (
              <Link
                key={item.name}
                href={item.href}
                className={`${
                  isActive
                    ? "bg-blue-50 border-r-4 border-blue-600 text-blue-700"
                    : "text-gray-600 hover:bg-gray-50 hover:text-gray-900"
                } group flex items-center px-3 py-2 text-sm font-medium rounded-l-md transition-colors duration-150`}
                onClick={() => setSidebarOpen(false)}
              >
                <item.icon
                  className={`${
                    isActive ? "text-blue-600" : "text-gray-400 group-hover:text-gray-500"
                  } flex-shrink-0 -ml-1 mr-3 h-5 w-5 transition-colors duration-150`}
                />
                <div className="flex-1">
                  <div className="text-sm font-medium">{item.name}</div>
                  <div className="text-xs text-gray-500 group-hover:text-gray-600 mt-0.5">
                    {item.description}
                  </div>
                </div>
              </Link>
            );
          })}
        </nav>

        {/* Logout Button */}
        <div className="flex-shrink-0 p-4 border-t border-gray-200">
          <button
            onClick={handleLogout}
            className="w-full flex items-center px-3 py-2 text-sm font-medium text-gray-600 rounded-md hover:bg-gray-50 hover:text-gray-900 transition-colors duration-150"
          >
            <ArrowRightOnRectangleIcon className="flex-shrink-0 -ml-1 mr-3 h-5 w-5 text-gray-400" />
            <span>Logout</span>
          </button>
        </div>
      </div>

      {/* Main content */}
      <div className="flex-1 overflow-hidden">
        {/* Top bar for mobile */}
        <div className="md:hidden">
          <div className="bg-white shadow-sm border-b border-gray-200 px-4 py-2">
            <div className="flex items-center justify-between">
              <button
                onClick={() => setSidebarOpen(true)}
                className="text-gray-500 hover:text-gray-700"
              >
                <Bars3Icon className="h-6 w-6" />
              </button>
              <div className="text-lg font-semibold text-gray-900">
                SaaS Admin Panel
              </div>
              <div></div>
            </div>
          </div>
        </div>

        {/* Page content */}
        <main className="flex-1 relative overflow-y-auto focus:outline-none">
          {children}
        </main>
      </div>
    </div>
  );
}
