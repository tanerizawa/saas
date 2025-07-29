"use client";

import { useAuth } from "@/contexts/AuthContext";
import { useState, useEffect } from "react";
import {
  TicketIcon,
  ClockIcon,
  CheckCircleIcon,
  ExclamationTriangleIcon,
  ChatBubbleLeftRightIcon,
  MagnifyingGlassIcon,
  EyeIcon,
  PaperAirplaneIcon,
  XMarkIcon,
  BuildingOfficeIcon,
  CalendarIcon
} from "@heroicons/react/24/outline";

interface SupportTicket {
  id: string;
  ticketNumber: string;
  subject: string;
  description: string;
  status: "open" | "in_progress" | "waiting_response" | "resolved" | "closed";
  priority: "low" | "medium" | "high" | "urgent";
  category: "technical" | "licensing" | "payment" | "document" | "general";
  company: {
    id: string;
    name: string;
    owner: string;
    email: string;
    phone: string;
  };
  createdAt: string;
  updatedAt: string;
  assignedStaff?: string;
  messages: {
    id: string;
    sender: "customer" | "staff";
    senderName: string;
    message: string;
    timestamp: string;
    attachments?: string[];
  }[];
}

export default function StaffSupportPage() {
  const { user, isAdminStaff } = useAuth();
  const [tickets, setTickets] = useState<SupportTicket[]>([]);
  const [filteredTickets, setFilteredTickets] = useState<SupportTicket[]>([]);
  const [loading, setLoading] = useState(true);
  const [searchTerm, setSearchTerm] = useState("");
  const [statusFilter, setStatusFilter] = useState("all");
  const [priorityFilter, setPriorityFilter] = useState("all");
  const [categoryFilter, setCategoryFilter] = useState("all");
  const [selectedTicket, setSelectedTicket] = useState<SupportTicket | null>(null);
  const [newMessage, setNewMessage] = useState("");
  const [sendingMessage, setSendingMessage] = useState(false);

  useEffect(() => {
    // Mock data - replace with actual API calls
    setTimeout(() => {
      const mockTickets: SupportTicket[] = [
        {
          id: "TIC-001",
          ticketNumber: "SUPP-2024-001",
          subject: "Tidak bisa upload dokumen KTP",
          description: "Saya mencoba upload dokumen KTP untuk aplikasi SIUP tapi selalu gagal. File sudah dalam format PDF dan ukurannya di bawah 5MB.",
          status: "open",
          priority: "high",
          category: "technical",
          company: {
            id: "COMP-001",
            name: "PT. Teknologi Maju",
            owner: "Budi Santoso",
            email: "budi@tekmaju.com",
            phone: "081234567890"
          },
          createdAt: "2024-01-15T08:30:00Z",
          updatedAt: "2024-01-15T08:30:00Z",
          assignedStaff: user?.name,
          messages: [
            {
              id: "MSG-001",
              sender: "customer",
              senderName: "Budi Santoso",
              message: "Saya mencoba upload dokumen KTP untuk aplikasi SIUP tapi selalu gagal. File sudah dalam format PDF dan ukurannya di bawah 5MB. Mohon bantuannya.",
              timestamp: "2024-01-15T08:30:00Z"
            }
          ]
        },
        {
          id: "TIC-002",
          ticketNumber: "SUPP-2024-002",
          subject: "Status aplikasi TDP belum update",
          description: "Sudah 5 hari submit aplikasi TDP tapi statusnya masih 'Under Review'. Kapan bisa diproses?",
          status: "in_progress",
          priority: "medium",
          category: "licensing",
          company: {
            id: "COMP-002",
            name: "CV. Berkah Mandiri",
            owner: "Siti Nurhaliza",
            email: "siti@berkah.com",
            phone: "081987654321"
          },
          createdAt: "2024-01-14T14:20:00Z",
          updatedAt: "2024-01-15T09:45:00Z",
          assignedStaff: user?.name,
          messages: [
            {
              id: "MSG-002",
              sender: "customer",
              senderName: "Siti Nurhaliza",
              message: "Sudah 5 hari submit aplikasi TDP tapi statusnya masih 'Under Review'. Kapan bisa diproses?",
              timestamp: "2024-01-14T14:20:00Z"
            },
            {
              id: "MSG-003",
              sender: "staff",
              senderName: user?.name || "Staff",
              message: "Terima kasih telah menghubungi kami. Aplikasi TDP Anda sedang dalam tahap review dokumen. Ada satu dokumen yang perlu diperbaiki - surat domisili. Silakan cek email untuk detailnya.",
              timestamp: "2024-01-15T09:45:00Z"
            },
            {
              id: "MSG-004",
              sender: "customer",
              senderName: "Siti Nurhaliza",
              message: "Baik, saya akan perbaiki dokumen surat domisili. Berapa lama lagi prosesnya setelah dokumen diperbaiki?",
              timestamp: "2024-01-15T10:30:00Z"
            }
          ]
        },
        {
          id: "TIC-003",
          ticketNumber: "SUPP-2024-003",
          subject: "Pembayaran berhasil tapi belum tercatat",
          description: "Saya sudah transfer biaya perizinan HO kemarin tapi di dashboard masih tertulis belum bayar.",
          status: "waiting_response",
          priority: "high",
          category: "payment",
          company: {
            id: "COMP-003",
            name: "UD. Sukses Bersama",
            owner: "Andi Pratama",
            email: "andi@suksesbersama.com",
            phone: "081555666777"
          },
          createdAt: "2024-01-13T11:45:00Z",
          updatedAt: "2024-01-15T11:20:00Z",
          assignedStaff: user?.name,
          messages: [
            {
              id: "MSG-005",
              sender: "customer",
              senderName: "Andi Pratama",
              message: "Saya sudah transfer biaya perizinan HO kemarin tapi di dashboard masih tertulis belum bayar. Nomor referensi: TRX123456789",
              timestamp: "2024-01-13T11:45:00Z"
            },
            {
              id: "MSG-006",
              sender: "staff",
              senderName: user?.name || "Staff",
              message: "Terima kasih informasinya. Saya akan cek dengan tim finance untuk verifikasi pembayaran Anda. Mohon tunggu maksimal 1x24 jam.",
              timestamp: "2024-01-15T11:20:00Z"
            }
          ]
        },
        {
          id: "TIC-004",
          ticketNumber: "SUPP-2024-004",
          subject: "Cara mengajukan perizinan NIB",
          description: "Saya baru daftar dan belum tahu cara mengajukan perizinan NIB. Bisa minta panduannya?",
          status: "resolved",
          priority: "low",
          category: "general",
          company: {
            id: "COMP-004",
            name: "PT. Digital Innovation",
            owner: "Lisa Permata",
            email: "lisa@diginov.com",
            phone: "081222333444"
          },
          createdAt: "2024-01-12T15:30:00Z",
          updatedAt: "2024-01-13T10:15:00Z",
          assignedStaff: user?.name,
          messages: [
            {
              id: "MSG-007",
              sender: "customer",
              senderName: "Lisa Permata",
              message: "Saya baru daftar dan belum tahu cara mengajukan perizinan NIB. Bisa minta panduannya?",
              timestamp: "2024-01-12T15:30:00Z"
            },
            {
              id: "MSG-008",
              sender: "staff",
              senderName: user?.name || "Staff",
              message: "Halo Bu Lisa, untuk mengajukan NIB silakan ikuti langkah berikut:\n1. Login ke dashboard UMKM\n2. Pilih menu 'Perizinan'\n3. Klik 'Ajukan Perizinan Baru'\n4. Pilih 'NIB'\n5. Isi form dan upload dokumen yang diperlukan\n\nDokumen yang perlu disiapkan: KTP, Akta Pendirian, SK Menkumham, NPWP Perusahaan.",
              timestamp: "2024-01-13T09:30:00Z"
            },
            {
              id: "MSG-009",
              sender: "customer",
              senderName: "Lisa Permata",
              message: "Terima kasih panduannya sangat jelas. Saya akan coba mengajukan sekarang.",
              timestamp: "2024-01-13T10:15:00Z"
            }
          ]
        }
      ];

      setTickets(mockTickets);
      setFilteredTickets(mockTickets);
      setLoading(false);
    }, 1000);
  }, [user?.name]);

  useEffect(() => {
    let filtered = tickets;

    // Search filter
    if (searchTerm) {
      filtered = filtered.filter(ticket => 
        ticket.subject.toLowerCase().includes(searchTerm.toLowerCase()) ||
        ticket.company.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
        ticket.ticketNumber.toLowerCase().includes(searchTerm.toLowerCase())
      );
    }

    // Status filter
    if (statusFilter !== "all") {
      filtered = filtered.filter(ticket => ticket.status === statusFilter);
    }

    // Priority filter
    if (priorityFilter !== "all") {
      filtered = filtered.filter(ticket => ticket.priority === priorityFilter);  
    }

    // Category filter
    if (categoryFilter !== "all") {
      filtered = filtered.filter(ticket => ticket.category === categoryFilter);
    }

    setFilteredTickets(filtered);
  }, [tickets, searchTerm, statusFilter, priorityFilter, categoryFilter]);

  const getPriorityColor = (priority: string) => {
    switch (priority) {
      case "urgent": return "bg-red-100 text-red-800 border-red-200";
      case "high": return "bg-orange-100 text-orange-800 border-orange-200";
      case "medium": return "bg-yellow-100 text-yellow-800 border-yellow-200";
      case "low": return "bg-green-100 text-green-800 border-green-200";
      default: return "bg-gray-100 text-gray-800 border-gray-200";
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case "open": return "bg-blue-100 text-blue-800";
      case "in_progress": return "bg-yellow-100 text-yellow-800";
      case "waiting_response": return "bg-purple-100 text-purple-800";
      case "resolved": return "bg-green-100 text-green-800";
      case "closed": return "bg-gray-100 text-gray-800";
      default: return "bg-gray-100 text-gray-800";
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case "open": return <TicketIcon className="h-4 w-4" />;
      case "in_progress": return <ClockIcon className="h-4 w-4" />;
      case "waiting_response": return <ChatBubbleLeftRightIcon className="h-4 w-4" />;
      case "resolved": return <CheckCircleIcon className="h-4 w-4" />;
      case "closed": return <XMarkIcon className="h-4 w-4" />;
      default: return <TicketIcon className="h-4 w-4" />;
    }
  };

  const getCategoryColor = (category: string) => {
    switch (category) {
      case "technical": return "bg-red-50 text-red-700";
      case "licensing": return "bg-blue-50 text-blue-700";
      case "payment": return "bg-green-50 text-green-700";
      case "document": return "bg-purple-50 text-purple-700";
      case "general": return "bg-gray-50 text-gray-700";
      default: return "bg-gray-50 text-gray-700";
    }
  };

  const sendMessage = async () => {
    if (!selectedTicket || !newMessage.trim()) return;

    setSendingMessage(true);
    
    // Simulate API call
    setTimeout(() => {
      const updatedTickets = tickets.map(ticket => {
        if (ticket.id === selectedTicket.id) {
          return {
            ...ticket,
            status: "in_progress" as typeof ticket.status,
            updatedAt: new Date().toISOString(),
            messages: [
              ...ticket.messages,
              {
                id: `MSG-${Date.now()}`,
                sender: "staff" as const,
                senderName: user?.name || "Staff",
                message: newMessage,
                timestamp: new Date().toISOString()
              }
            ]
          };
        }
        return ticket;
      });

      setTickets(updatedTickets);
      setSelectedTicket(prev => prev ? {
        ...prev,
        status: "in_progress",
        updatedAt: new Date().toISOString(),
        messages: [
          ...prev.messages,
          {
            id: `MSG-${Date.now()}`,
            sender: "staff",
            senderName: user?.name || "Staff",
            message: newMessage,
            timestamp: new Date().toISOString()
          }
        ]
      } : null);
      
      setNewMessage("");
      setSendingMessage(false);
    }, 1000);
  };

  const updateTicketStatus = (ticketId: string, newStatus: SupportTicket['status']) => {
    const updatedTickets = tickets.map(ticket => {
      if (ticket.id === ticketId) {
        return {
          ...ticket,
          status: newStatus,
          updatedAt: new Date().toISOString()
        };
      }
      return ticket;
    });

    setTickets(updatedTickets);
    if (selectedTicket && selectedTicket.id === ticketId) {
      setSelectedTicket(prev => prev ? { ...prev, status: newStatus, updatedAt: new Date().toISOString() } : null);
    }
  };

  if (!isAdminStaff()) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-center">
          <ExclamationTriangleIcon className="mx-auto h-16 w-16 text-red-500 mb-4" />
          <h2 className="text-xl font-semibold text-gray-900 mb-2">Akses Ditolak</h2>
          <p className="text-gray-600">Halaman ini hanya dapat diakses oleh Staff Administrator.</p>
        </div>
      </div>
    );
  }

  if (loading) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="animate-spin rounded-full h-32 w-32 border-b-2 border-green-600"></div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Header */}
      <div className="bg-white shadow-sm border-b">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="py-6">
            <div className="flex items-center justify-between">
              <div className="flex items-center">
                <TicketIcon className="h-8 w-8 text-green-600 mr-3" />
                <div>
                  <h1 className="text-2xl font-bold text-gray-900">Customer Support</h1>
                  <p className="text-sm text-gray-500">Handle support tickets dan komunikasi dengan UMKM</p>
                </div>
              </div>
              <div className="flex items-center space-x-4">
                <div className="text-sm text-gray-500">
                  {filteredTickets.length} dari {tickets.length} tickets
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Stats Cards */}
        <div className="grid grid-cols-1 md:grid-cols-5 gap-6 mb-8">
          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center">
              <div className="p-2 bg-blue-100 rounded-full">
                <TicketIcon className="h-6 w-6 text-blue-600" />
              </div>
              <div className="ml-3">
                <p className="text-sm font-medium text-gray-500">Open</p>
                <p className="text-2xl font-semibold text-gray-900">
                  {tickets.filter(t => t.status === "open").length}
                </p>
              </div>
            </div>
          </div>

          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center">
              <div className="p-2 bg-yellow-100 rounded-full">
                <ClockIcon className="h-6 w-6 text-yellow-600" />
              </div>
              <div className="ml-3">
                <p className="text-sm font-medium text-gray-500">In Progress</p>
                <p className="text-2xl font-semibold text-gray-900">
                  {tickets.filter(t => t.status === "in_progress").length}
                </p>
              </div>
            </div>
          </div>

          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center">
              <div className="p-2 bg-purple-100 rounded-full">
                <ChatBubbleLeftRightIcon className="h-6 w-6 text-purple-600" />
              </div>
              <div className="ml-3">
                <p className="text-sm font-medium text-gray-500">Waiting</p>
                <p className="text-2xl font-semibold text-gray-900">
                  {tickets.filter(t => t.status === "waiting_response").length}
                </p>
              </div>
            </div>
          </div>

          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center">
              <div className="p-2 bg-green-100 rounded-full">
                <CheckCircleIcon className="h-6 w-6 text-green-600" />
              </div>
              <div className="ml-3">
                <p className="text-sm font-medium text-gray-500">Resolved</p>
                <p className="text-2xl font-semibold text-gray-900">
                  {tickets.filter(t => t.status === "resolved").length}
                </p>
              </div>
            </div>
          </div>

          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center">
              <div className="p-2 bg-red-100 rounded-full">
                <ExclamationTriangleIcon className="h-6 w-6 text-red-600" />
              </div>
              <div className="ml-3">
                <p className="text-sm font-medium text-gray-500">Urgent</p>
                <p className="text-2xl font-semibold text-gray-900">
                  {tickets.filter(t => t.priority === "urgent").length}
                </p>
              </div>
            </div>
          </div>
        </div>

        {/* Filters */}
        <div className="bg-white rounded-lg shadow mb-8">
          <div className="px-6 py-4">
            <div className="flex flex-col lg:flex-row lg:items-center lg:justify-between space-y-4 lg:space-y-0">
              {/* Search */}
              <div className="relative flex-1 max-w-lg">
                <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                  <MagnifyingGlassIcon className="h-5 w-5 text-gray-400" />
                </div>
                <input
                  type="text"
                  value={searchTerm}
                  onChange={(e) => setSearchTerm(e.target.value)}
                  className="block w-full pl-10 pr-3 py-2 border border-gray-300 rounded-md leading-5 placeholder-gray-500 focus:outline-none focus:placeholder-gray-400 focus:ring-1 focus:ring-green-500 focus:border-green-500"
                  placeholder="Cari berdasarkan subject, perusahaan, atau ticket number..."
                />
              </div>

              {/* Filters */}
              <div className="flex space-x-4">
                <select
                  value={statusFilter}
                  onChange={(e) => setStatusFilter(e.target.value)}
                  className="px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-green-500 focus:border-green-500"
                >
                  <option value="all">Semua Status</option>
                  <option value="open">Open</option>
                  <option value="in_progress">In Progress</option>
                  <option value="waiting_response">Waiting Response</option>
                  <option value="resolved">Resolved</option>
                  <option value="closed">Closed</option>
                </select>

                <select
                  value={priorityFilter}
                  onChange={(e) => setPriorityFilter(e.target.value)}
                  className="px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-green-500 focus:border-green-500"
                >
                  <option value="all">Semua Prioritas</option>
                  <option value="urgent">Urgent</option>
                  <option value="high">High</option>
                  <option value="medium">Medium</option>
                  <option value="low">Low</option>
                </select>

                <select
                  value={categoryFilter}
                  onChange={(e) => setCategoryFilter(e.target.value)}
                  className="px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-green-500 focus:border-green-500"
                >
                  <option value="all">Semua Kategori</option>
                  <option value="technical">Technical</option>
                  <option value="licensing">Licensing</option>
                  <option value="payment">Payment</option>
                  <option value="document">Document</option>
                  <option value="general">General</option>
                </select>
              </div>
            </div>
          </div>
        </div>

        {/* Tickets List */}
        <div className="space-y-6">
          {filteredTickets.map((ticket) => (
            <div key={ticket.id} className="bg-white rounded-lg shadow">
              <div className="px-6 py-4 border-b border-gray-200">
                <div className="flex items-center justify-between">
                  <div className="flex items-center space-x-4">
                    <div className="flex items-center space-x-2">
                      <div className={`p-1 rounded-full ${getStatusColor(ticket.status)}`}>
                        {getStatusIcon(ticket.status)}
                      </div>
                      <div>
                        <h3 className="text-lg font-medium text-gray-900">{ticket.subject}</h3>
                        <p className="text-sm text-gray-500">#{ticket.ticketNumber}</p>
                      </div>
                    </div>
                    <div className="flex items-center space-x-2">
                      <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border ${getPriorityColor(ticket.priority)}`}>
                        {ticket.priority.toUpperCase()}
                      </span>
                      <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${getCategoryColor(ticket.category)}`}>
                        {ticket.category}
                      </span>
                    </div>
                  </div>
                  <div className="flex items-center space-x-2">
                    <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${getStatusColor(ticket.status)}`}>
                      {ticket.status.replace('_', ' ').toUpperCase()}
                    </span>
                    <button
                      onClick={() => setSelectedTicket(ticket)}
                      className="inline-flex items-center px-3 py-2 border border-transparent text-sm font-medium rounded-md text-green-700 bg-green-100 hover:bg-green-200"
                    >
                      <EyeIcon className="h-4 w-4 mr-1" />
                      View
                    </button>
                  </div>
                </div>
              </div>

              <div className="px-6 py-4">
                <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
                  {/* Company Info */}
                  <div>
                    <h4 className="text-sm font-medium text-gray-700 mb-3 flex items-center">
                      <BuildingOfficeIcon className="h-4 w-4 mr-2" />
                      Perusahaan
                    </h4>
                    <div className="space-y-2 text-sm">
                      <div><span className="text-gray-500">Nama:</span> {ticket.company.name}</div>
                      <div><span className="text-gray-500">Owner:</span> {ticket.company.owner}</div>
                      <div><span className="text-gray-500">Email:</span> {ticket.company.email}</div>
                      <div><span className="text-gray-500">Phone:</span> {ticket.company.phone}</div>
                    </div>
                  </div>

                  {/* Description */}
                  <div>
                    <h4 className="text-sm font-medium text-gray-700 mb-3">Deskripsi</h4>
                    <p className="text-sm text-gray-600">{ticket.description}</p>
                  </div>

                  {/* Timeline */}
                  <div>
                    <h4 className="text-sm font-medium text-gray-700 mb-3 flex items-center">
                      <CalendarIcon className="h-4 w-4 mr-2" />
                      Timeline
                    </h4>
                    <div className="space-y-2 text-sm">
                      <div><span className="text-gray-500">Created:</span> {new Date(ticket.createdAt).toLocaleDateString("id-ID")}</div>
                      <div><span className="text-gray-500">Last Update:</span> {new Date(ticket.updatedAt).toLocaleDateString("id-ID")}</div>
                      <div><span className="text-gray-500">Assigned:</span> {ticket.assignedStaff}</div>
                      <div><span className="text-gray-500">Messages:</span> {ticket.messages.length}</div>
                    </div>
                  </div>
                </div>

                {/* Quick Actions */}
                {ticket.status !== "closed" && ticket.status !== "resolved" && (
                  <div className="mt-6 flex items-center space-x-3">
                    <button
                      onClick={() => updateTicketStatus(ticket.id, "in_progress")}
                      disabled={ticket.status === "in_progress"}
                      className="inline-flex items-center px-3 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-yellow-600 hover:bg-yellow-700 disabled:bg-gray-400"
                    >
                      Start Progress
                    </button>
                    <button
                      onClick={() => updateTicketStatus(ticket.id, "waiting_response")}
                      disabled={ticket.status === "waiting_response"}
                      className="inline-flex items-center px-3 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-purple-600 hover:bg-purple-700 disabled:bg-gray-400"
                    >
                      Wait Response
                    </button>
                    <button
                      onClick={() => updateTicketStatus(ticket.id, "resolved")}
                      className="inline-flex items-center px-3 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-green-600 hover:bg-green-700"
                    >
                      Mark Resolved
                    </button>
                  </div>
                )}
              </div>
            </div>
          ))}
        </div>

        {filteredTickets.length === 0 && (
          <div className="text-center py-12 bg-white rounded-lg shadow">
            <TicketIcon className="mx-auto h-12 w-12 text-gray-400" />
            <h3 className="mt-2 text-sm font-medium text-gray-900">Tidak ada support tickets</h3>
            <p className="mt-1 text-sm text-gray-500">
              Tidak ada tickets yang sesuai dengan filter yang dipilih.
            </p>
          </div>
        )}
      </div>

      {/* Ticket Detail Modal */}
      {selectedTicket && (
        <div className="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50">
          <div className="relative top-10 mx-auto p-5 border w-11/12 max-w-4xl shadow-lg rounded-md bg-white">
            <div className="mt-3">
              <div className="flex items-center justify-between mb-4">
                <div>
                  <h3 className="text-lg font-medium text-gray-900">{selectedTicket.subject}</h3>
                  <p className="text-sm text-gray-500">#{selectedTicket.ticketNumber} • {selectedTicket.company.name}</p>
                </div>
                <button
                  onClick={() => setSelectedTicket(null)}
                  className="text-gray-400 hover:text-gray-600"
                >
                  <XMarkIcon className="h-6 w-6" />
                </button>
              </div>

              {/* Messages */}
              <div className="max-h-96 overflow-y-auto mb-4 border rounded-lg">
                <div className="p-4 space-y-4">
                  {selectedTicket.messages.map((message) => (
                    <div key={message.id} className={`flex ${message.sender === "staff" ? "justify-end" : "justify-start"}`}>
                      <div className={`max-w-md px-4 py-2 rounded-lg ${
                        message.sender === "staff" 
                          ? "bg-green-100 text-green-900" 
                          : "bg-gray-100 text-gray-900"
                      }`}>
                        <div className="flex items-center justify-between mb-1">
                          <span className="text-xs font-medium">
                            {message.senderName}
                          </span>
                          <span className="text-xs text-gray-500">
                            {new Date(message.timestamp).toLocaleString("id-ID")}
                          </span>
                        </div>
                        <p className="text-sm whitespace-pre-wrap">{message.message}</p>
                      </div>
                    </div>
                  ))}
                </div>
              </div>

              {/* Reply Box */}
              <div className="border-t pt-4">
                <div className="flex space-x-4">
                  <div className="flex-1">
                    <textarea
                      value={newMessage}
                      onChange={(e) => setNewMessage(e.target.value)}
                      rows={3}
                      className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-green-500 focus:border-green-500"
                      placeholder="Tulis balasan..."
                    />
                  </div>
                  <div className="flex flex-col justify-end">
                    <button
                      onClick={sendMessage}
                      disabled={!newMessage.trim() || sendingMessage}
                      className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-green-600 hover:bg-green-700 disabled:bg-gray-400"
                    >
                      {sendingMessage ? (
                        <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white mr-2"></div>
                      ) : (
                        <PaperAirplaneIcon className="h-4 w-4 mr-2" />
                      )}
                      Kirim
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
