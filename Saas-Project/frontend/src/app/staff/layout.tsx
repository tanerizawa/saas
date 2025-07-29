"use client";

import { useAuth } from "@/contexts/AuthContext";
import { useState } from "react";
import Link from "next/link";
import { usePathname } from "next/navigation";
import {
  HomeIcon,
  ShieldCheckIcon,
  DocumentTextIcon,
  ChartBarIcon,
  TicketIcon,
  UserGroupIcon,
  ClipboardDocumentListIcon,
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
    href: "/staff",
    icon: HomeIcon,
    description: "Overview tugas dan performance"
  },
  {
    name: "Perusahaan Assigned",
    href: "/staff/companies",
    icon: UserGroupIcon,
    description: "Kelola perusahaan yang ditugaskan"
  },
  {
    name: "Review Perizinan",
    href: "/staff/licenses",
    icon: ShieldCheckIcon,
    description: "Proses aplikasi perizinan"
  },
  {
    name: "Verifikasi Dokumen",
    href: "/staff/documents",
    icon: DocumentTextIcon,
    description: "Review dan verifikasi dokumen"
  },
  {
    name: "Customer Support",
    href: "/staff/support",
    icon: TicketIcon,
    description: "Handle tickets dan support"
  },
  {
    name: "Task Management",
    href: "/staff/tasks",
    icon: ClipboardDocumentListIcon,
    description: "Kelola semua tugas pending"
  },
  {
    name: "Laporan",
    href: "/staff/reports",
    icon: ChartBarIcon,
    description: "Performance dan aktivitas reports"
  }
];

export default function StaffLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const { user, logout } = useAuth();
  const pathname = usePathname();
  const [sidebarOpen, setSidebarOpen] = useState(false);

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
        <div className="flex items-center justify-between h-16 px-4 bg-green-600">
          <div className="flex items-center">
            <div className="flex-shrink-0">
              <div className="h-8 w-8 bg-white rounded flex items-center justify-center">
                <span className="text-green-600 font-bold text-sm">ST</span>
              </div>
            </div>
            <div className="ml-2">
              <div className="text-white font-semibold text-sm">Staff Panel</div>
              <div className="text-green-200 text-xs">Task Management</div>
            </div>
          </div>
          <button
            onClick={() => setSidebarOpen(false)}
            className="md:hidden text-white hover:text-green-200"
          >
            <XMarkIcon className="h-6 w-6" />
          </button>
        </div>

        {/* User Info */}
        <div className="px-4 py-4 border-b border-gray-200">
          <div className="flex items-center">
            <div className="flex-shrink-0">
              <div className="h-10 w-10 bg-green-100 rounded-full flex items-center justify-center">
                <span className="text-green-600 font-medium text-sm">
                  {user?.name?.charAt(0).toUpperCase() || 'S'}
                </span>
              </div>
            </div>
            <div className="ml-3">
              <div className="text-sm font-medium text-gray-900">{user?.name || 'Staff'}</div>
              <div className="text-xs text-gray-500">
                Admin Staff • Processing Team
              </div>
            </div>
          </div>
        </div>

        {/* Navigation */}
        <nav className="mt-2 flex-1 px-2 space-y-1 overflow-y-auto">
          {navigation.map((item) => {
            const isActive = pathname === item.href;
            return (
              <Link
                key={item.name}
                href={item.href}
                className={`${
                  isActive
                    ? "bg-green-50 border-r-4 border-green-600 text-green-700"
                    : "text-gray-600 hover:bg-gray-50 hover:text-gray-900"
                } group flex items-center px-3 py-2 text-sm font-medium rounded-l-md transition-colors duration-150`}
                onClick={() => setSidebarOpen(false)}
              >
                <item.icon
                  className={`${
                    isActive ? "text-green-600" : "text-gray-400 group-hover:text-gray-500"
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

        {/* Performance Summary */}
        <div className="flex-shrink-0 p-4 border-t border-gray-200 bg-gray-50">
          <div className="text-xs font-medium text-gray-500 mb-2">Quick Stats</div>
          <div className="grid grid-cols-2 gap-2 text-xs">
            <div className="text-center">
              <div className="font-semibold text-gray-900">8</div>
              <div className="text-gray-500">Pending</div>
            </div>
            <div className="text-center">
              <div className="font-semibold text-green-600">23</div>
              <div className="text-gray-500">Completed</div>
            </div>
          </div>
        </div>

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
                Staff Control Panel
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
