"use client";

import { useAuth } from "@/contexts/AuthContext";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import Link from "next/link";
import {
  UserIcon,
  PencilIcon,
  TrashIcon,
  PlusIcon,
  MagnifyingGlassIcon,
} from "@heroicons/react/24/outline";

interface User {
  id: string;
  email: string;
  full_name: string;
  role: string;
  status: string;
  created_at: string;
  last_login?: string;
}

export default function UsersPage() {
  const { user, isLoading } = useAuth();
  const router = useRouter();
  const [users, setUsers] = useState<User[]>([]);
  const [loading, setLoading] = useState(true);
  const [searchTerm, setSearchTerm] = useState("");

  useEffect(() => {
    if (!isLoading && !user) {
      router.push("/auth/login");
      return;
    }

    // Check if user has admin access
    if (user && user.role !== "super_admin" && user.role !== "admin_staff") {
      router.push("/dashboard");
      return;
    }
  }, [user, isLoading, router]);

  useEffect(() => {
    if (user) {
      loadUsers();
    }
  }, [user]);

  const loadUsers = async () => {
    try {
      setLoading(true);
      
      // Mock users data for demo
      const mockUsers: User[] = [
        {
          id: "5ebe8671-bd7f-45e4-aff6-d69f2ecf1df3",
          email: "admin@saas-umkm.local",
          full_name: "System Administrator",
          role: "super_admin",
          status: "active",
          created_at: "2025-01-01T00:00:00Z",
          last_login: "2025-07-28T12:00:00Z"
        },
        {
          id: "user-001",
          email: "staff@saas-umkm.local",
          full_name: "Staff Admin",
          role: "admin_staff",
          status: "active",
          created_at: "2025-01-15T00:00:00Z",
          last_login: "2025-07-27T10:30:00Z"
        },
        {
          id: "user-002",
          email: "umkm1@example.com",
          full_name: "UMKM Owner 1",
          role: "umkm_owner",
          status: "active",
          created_at: "2025-02-01T00:00:00Z",
          last_login: "2025-07-26T15:20:00Z"
        },
        {
          id: "user-003",
          email: "umkm2@example.com",
          full_name: "UMKM Owner 2",
          role: "umkm_owner",
          status: "inactive",
          created_at: "2025-02-15T00:00:00Z",
        }
      ];

      // Simulate API delay
      await new Promise(resolve => setTimeout(resolve, 800));
      
      setUsers(mockUsers);
      console.log("👥 Users loaded:", mockUsers);
    } catch (error) {
      console.error("Failed to load users:", error);
    } finally {
      setLoading(false);
    }
  };

  const filteredUsers = users.filter(u =>
    u.full_name.toLowerCase().includes(searchTerm.toLowerCase()) ||
    u.email.toLowerCase().includes(searchTerm.toLowerCase()) ||
    u.role.toLowerCase().includes(searchTerm.toLowerCase())
  );

  const getRoleDisplay = (role: string) => {
    switch (role) {
      case "super_admin": return "Super Administrator";
      case "admin_staff": return "Admin Staff";
      case "umkm_owner": return "UMKM Owner";
      default: return role;
    }
  };

  const getStatusBadge = (status: string) => {
    const baseClass = "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium";
    if (status === "active") {
      return `${baseClass} bg-green-100 text-green-800`;
    }
    return `${baseClass} bg-red-100 text-red-800`;
  };

  if (isLoading || loading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="animate-spin rounded-full h-32 w-32 border-b-2 border-blue-600"></div>
      </div>
    );
  }

  if (!user || (user.role !== "super_admin" && user.role !== "admin_staff")) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="text-center">
          <h1 className="text-2xl font-bold text-gray-900">Access Denied</h1>
          <p className="mt-2 text-gray-600">You don&apos;t have permission to access this page.</p>
          <Link href="/dashboard" className="mt-4 inline-block text-blue-600 hover:text-blue-500">
            Return to Dashboard
          </Link>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Header */}
      <div className="bg-white shadow">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center py-6">
            <div>
              <nav className="flex" aria-label="Breadcrumb">
                <ol className="flex items-center space-x-4">
                  <li>
                    <Link href="/dashboard" className="text-gray-400 hover:text-gray-500">
                      Dashboard
                    </Link>
                  </li>
                  <li>
                    <span className="text-gray-400">/</span>
                  </li>
                  <li>
                    <span className="text-gray-900 font-medium">Manajemen User</span>
                  </li>
                </ol>
              </nav>
              <h1 className="mt-2 text-3xl font-bold text-gray-900">
                Manajemen User
              </h1>
            </div>
            <button className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700">
              <PlusIcon className="h-4 w-4 mr-2" />
              Tambah User
            </button>
          </div>
        </div>
      </div>

      {/* Main Content */}
      <div className="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
        <div className="px-4 py-6 sm:px-0">
          {/* Search and Filters */}
          <div className="mb-6 flex flex-col sm:flex-row gap-4">
            <div className="flex-1">
              <div className="relative">
                <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                  <MagnifyingGlassIcon className="h-5 w-5 text-gray-400" />
                </div>
                <input
                  type="text"
                  value={searchTerm}
                  onChange={(e) => setSearchTerm(e.target.value)}
                  className="block w-full pl-10 pr-3 py-2 border border-gray-300 rounded-md leading-5 bg-white placeholder-gray-500 focus:outline-none focus:placeholder-gray-400 focus:ring-1 focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
                  placeholder="Cari user berdasarkan nama, email, atau role..."
                />
              </div>
            </div>
          </div>

          {/* Users Table */}
          <div className="bg-white shadow overflow-hidden sm:rounded-md">
            <div className="px-4 py-5 sm:p-6">
              <div className="flex justify-between items-center mb-4">
                <h3 className="text-lg leading-6 font-medium text-gray-900">
                  Daftar User ({filteredUsers.length})
                </h3>
              </div>
              
              <div className="overflow-x-auto">
                <table className="min-w-full divide-y divide-gray-200">
                  <thead className="bg-gray-50">
                    <tr>
                      <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                        User
                      </th>
                      <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                        Role
                      </th>
                      <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                        Status
                      </th>
                      <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                        Last Login
                      </th>
                      <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                        Actions
                      </th>
                    </tr>
                  </thead>
                  <tbody className="bg-white divide-y divide-gray-200">
                    {filteredUsers.map((u) => (
                      <tr key={u.id} className="hover:bg-gray-50">
                        <td className="px-6 py-4 whitespace-nowrap">
                          <div className="flex items-center">
                            <div className="flex-shrink-0 h-10 w-10">
                              <div className="h-10 w-10 rounded-full bg-blue-500 flex items-center justify-center">
                                <UserIcon className="h-6 w-6 text-white" />
                              </div>
                            </div>
                            <div className="ml-4">
                              <div className="text-sm font-medium text-gray-900">
                                {u.full_name}
                              </div>
                              <div className="text-sm text-gray-500">
                                {u.email}
                              </div>
                            </div>
                          </div>
                        </td>
                        <td className="px-6 py-4 whitespace-nowrap">
                          <span className="text-sm text-gray-900">
                            {getRoleDisplay(u.role)}
                          </span>
                        </td>
                        <td className="px-6 py-4 whitespace-nowrap">
                          <span className={getStatusBadge(u.status)}>
                            {u.status === "active" ? "Aktif" : "Tidak Aktif"}
                          </span>
                        </td>
                        <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                          {u.last_login 
                            ? new Date(u.last_login).toLocaleDateString("id-ID", {
                                year: "numeric",
                                month: "short",
                                day: "numeric",
                                hour: "2-digit",
                                minute: "2-digit"
                              })
                            : "Belum pernah login"
                          }
                        </td>
                        <td className="px-6 py-4 whitespace-nowrap text-sm font-medium space-x-3">
                          <button className="text-blue-600 hover:text-blue-900">
                            <PencilIcon className="h-4 w-4" />
                          </button>
                          {u.id !== user.id && (
                            <button className="text-red-600 hover:text-red-900">
                              <TrashIcon className="h-4 w-4" />
                            </button>
                          )}
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>

              {filteredUsers.length === 0 && (
                <div className="text-center py-8">
                  <UserIcon className="mx-auto h-12 w-12 text-gray-400" />
                  <h3 className="mt-2 text-sm font-medium text-gray-900">Tidak ada user ditemukan</h3>
                  <p className="mt-1 text-sm text-gray-500">
                    {searchTerm ? "Coba ubah kata kunci pencarian" : "Belum ada user yang terdaftar"}
                  </p>
                </div>
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
