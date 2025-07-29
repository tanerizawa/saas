"use client";

import { useAuth } from "@/contexts/AuthContext";
import { useState, useEffect } from "react";
import { 
  EnvelopeIcon,
  BellIcon,
  CogIcon,
  CheckIcon,
  XMarkIcon,
  PlusIcon,
  ExclamationTriangleIcon
} from "@heroicons/react/24/outline";

interface EmailTemplate {
  id: string;
  name: string;
  subject: string;
  type: "license_approved" | "license_rejected" | "payment_reminder" | "welcome" | "custom";
  content: string;
  variables: string[];
  active: boolean;
  created_at: string;
}

interface NotificationSettings {
  email_notifications: boolean;
  license_status_updates: boolean;
  payment_reminders: boolean;
  system_alerts: boolean;
  weekly_reports: boolean;
  admin_email: string;
  notification_frequency: "immediate" | "daily" | "weekly";
}

export default function EmailNotificationsPage() {
  const { isSuperAdmin } = useAuth();
  const [templates, setTemplates] = useState<EmailTemplate[]>([]);
  const [settings, setSettings] = useState<NotificationSettings | null>(null);
  const [loading, setLoading] = useState(true);
  const [activeTab, setActiveTab] = useState("templates");

  useEffect(() => {
    // Mock data - replace with actual API call
    setTimeout(() => {
      setTemplates([
        {
          id: "1",
          name: "Perizinan Disetujui",
          subject: "Perizinan {{license_type}} Anda Telah Disetujui",
          type: "license_approved",
          content: `Halo {{company_name}},

Kami dengan senang hati memberitahukan bahwa pengajuan perizinan {{license_type}} Anda telah disetujui.

Detail Perizinan:
- Jenis: {{license_type}}
- Nomor: {{license_number}}
- Tanggal Persetujuan: {{approval_date}}
- Berlaku hingga: {{expiry_date}}

Dokumen perizinan dapat diunduh melalui dashboard UMKM Anda.

Terima kasih atas kepercayaan Anda menggunakan layanan kami.

Salam,
Tim SaaS UMKM`,
          variables: ["company_name", "license_type", "license_number", "approval_date", "expiry_date"],
          active: true,
          created_at: "2024-01-10T09:00:00Z"
        },
        {
          id: "2",
          name: "Perizinan Ditolak",
          subject: "Pengajuan {{license_type}} Perlu Perbaikan",
          type: "license_rejected",
          content: `Halo {{company_name}},

Setelah kami review, pengajuan perizinan {{license_type}} Anda memerlukan perbaikan.

Alasan penolakan:
{{rejection_reason}}

Langkah selanjutnya:
1. Perbaiki dokumen sesuai catatan di atas
2. Upload ulang dokumen yang sudah diperbaiki
3. Submit ulang pengajuan melalui dashboard

Jika ada pertanyaan, silakan hubungi customer service kami.

Salam,
Tim SaaS UMKM`,
          variables: ["company_name", "license_type", "rejection_reason"],
          active: true,
          created_at: "2024-01-10T09:15:00Z"
        },
        {
          id: "3",
          name: "Pengingat Pembayaran",
          subject: "Pengingat Pembayaran {{license_type}}",
          type: "payment_reminder",
          content: `Halo {{company_name}},

Ini adalah pengingat bahwa pembayaran untuk {{license_type}} akan jatuh tempo dalam {{days_remaining}} hari.

Detail Pembayaran:
- Invoice: {{invoice_number}}
- Jumlah: {{amount}}
- Jatuh Tempo: {{due_date}}

Silakan lakukan pembayaran melalui dashboard UMKM Anda untuk menghindari keterlambatan.

Terima kasih.

Salam,
Tim SaaS UMKM`,
          variables: ["company_name", "license_type", "days_remaining", "invoice_number", "amount", "due_date"],
          active: true,
          created_at: "2024-01-10T09:30:00Z"
        },
        {
          id: "4",
          name: "Selamat Datang",
          subject: "Selamat Datang di SaaS UMKM Platform",
          type: "welcome",
          content: `Halo {{company_name}},

Selamat datang di SaaS UMKM Platform!

Akun Anda telah berhasil dibuat dengan detail:
- Email: {{email}}
- Company: {{company_name}}
- Tanggal Registrasi: {{registration_date}}

Anda dapat mengakses dashboard UMKM melalui: {{dashboard_url}}

Fitur yang dapat Anda gunakan:
- Mengajukan berbagai perizinan
- Upload dan kelola dokumen
- Tracking status aplikasi
- Laporan keuangan

Jika membutuhkan bantuan, jangan ragu untuk menghubungi customer service kami.

Salam,
Tim SaaS UMKM`,
          variables: ["company_name", "email", "registration_date", "dashboard_url"],
          active: true,
          created_at: "2024-01-10T10:00:00Z"
        }
      ]);

      setSettings({
        email_notifications: true,
        license_status_updates: true,
        payment_reminders: true,
        system_alerts: true,
        weekly_reports: false,
        admin_email: "admin@saas-umkm.com",
        notification_frequency: "immediate"
      });

      setLoading(false);
    }, 1000);
  }, []);

  const handleToggleTemplate = (templateId: string) => {
    setTemplates(prev => prev.map(template => 
      template.id === templateId 
        ? { ...template, active: !template.active }
        : template
    ));
  };

  const handleSettingsChange = (key: keyof NotificationSettings, value: boolean | string) => {
    setSettings(prev => prev ? { ...prev, [key]: value } : null);
  };

  const saveSettings = () => {
    // TODO: Replace with actual API call
    console.log("Saving settings:", settings);
    alert("Pengaturan berhasil disimpan!");
  };

  const sendTestEmail = (templateId: string) => {
    // TODO: Replace with actual API call
    console.log("Sending test email for template:", templateId);
    alert("Email test berhasil dikirim!");
  };

  if (!isSuperAdmin()) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-center">
          <ExclamationTriangleIcon className="mx-auto h-16 w-16 text-red-500 mb-4" />
          <h2 className="text-xl font-semibold text-gray-900 mb-2">Akses Ditolak</h2>
          <p className="text-gray-600">Halaman ini hanya dapat diakses oleh Super Admin.</p>
        </div>
      </div>
    );
  }

  if (loading) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="animate-spin rounded-full h-32 w-32 border-b-2 border-blue-600"></div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Header */}
      <div className="bg-white shadow-sm border-b">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="py-6">
            <div className="flex items-center">
              <EnvelopeIcon className="h-8 w-8 text-blue-600 mr-3" />
              <div>
                <h1 className="text-2xl font-bold text-gray-900">Sistem Notifikasi Email</h1>
                <p className="text-sm text-gray-500">Kelola template email dan pengaturan notifikasi</p>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Tabs */}
        <div className="mb-8">
          <nav className="flex space-x-8">
            <button
              onClick={() => setActiveTab("templates")}
              className={`py-2 px-1 border-b-2 font-medium text-sm ${
                activeTab === "templates"
                  ? "border-blue-500 text-blue-600"
                  : "border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300"
              }`}
            >
              Template Email
            </button>
            <button
              onClick={() => setActiveTab("settings")}
              className={`py-2 px-1 border-b-2 font-medium text-sm ${
                activeTab === "settings"
                  ? "border-blue-500 text-blue-600"
                  : "border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300"
              }`}
            >
              Pengaturan Notifikasi
            </button>
          </nav>
        </div>

        {/* Email Templates Tab */}
        {activeTab === "templates" && (
          <div className="space-y-6">
            {/* Templates Header */}
            <div className="flex justify-between items-center">
              <h2 className="text-lg font-medium text-gray-900">Template Email</h2>
              <button
                onClick={() => alert("Fitur tambah template akan segera tersedia")}
                className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700"
              >
                <PlusIcon className="h-4 w-4 mr-2" />
                Buat Template Baru
              </button>
            </div>

            {/* Templates List */}
            <div className="grid grid-cols-1 gap-6">
              {templates.map((template) => (
                <div key={template.id} className="bg-white rounded-lg shadow">
                  <div className="px-6 py-4 border-b border-gray-200">
                    <div className="flex items-center justify-between">
                      <div className="flex items-center space-x-3">
                        <div className={`p-2 rounded-full ${template.active ? 'bg-green-100' : 'bg-gray-100'}`}>
                          <EnvelopeIcon className={`h-5 w-5 ${template.active ? 'text-green-600' : 'text-gray-400'}`} />
                        </div>
                        <div>
                          <h3 className="text-lg font-medium text-gray-900">{template.name}</h3>
                          <p className="text-sm text-gray-500">Subject: {template.subject}</p>
                        </div>
                      </div>
                      <div className="flex items-center space-x-2">
                        <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${
                          template.active 
                            ? 'bg-green-100 text-green-800' 
                            : 'bg-gray-100 text-gray-800'
                        }`}>
                          {template.active ? 'Aktif' : 'Nonaktif'}
                        </span>
                        <button
                          onClick={() => sendTestEmail(template.id)}
                          className="text-blue-600 hover:text-blue-500 text-sm font-medium"
                        >
                          Test Email
                        </button>
                        <button
                          onClick={() => alert("Fitur edit template akan segera tersedia")}
                          className="text-gray-600 hover:text-gray-500 text-sm font-medium"
                        >
                          Edit
                        </button>
                        <button
                          onClick={() => handleToggleTemplate(template.id)}
                          className={`p-1 rounded ${
                            template.active 
                              ? 'text-green-600 hover:text-green-500' 
                              : 'text-gray-400 hover:text-gray-500'
                          }`}
                        >
                          {template.active ? <CheckIcon className="h-5 w-5" /> : <XMarkIcon className="h-5 w-5" />}
                        </button>
                      </div>
                    </div>
                  </div>
                  <div className="px-6 py-4">
                    <div className="space-y-4">
                      <div>
                        <h4 className="text-sm font-medium text-gray-700 mb-2">Konten Email:</h4>
                        <div className="bg-gray-50 rounded p-3 text-sm text-gray-700 max-h-40 overflow-y-auto">
                          {template.content.split('\n').map((line, index) => (
                            <div key={index}>{line || <br />}</div>
                          ))}
                        </div>
                      </div>
                      <div>
                        <h4 className="text-sm font-medium text-gray-700 mb-2">Variabel Tersedia:</h4>
                        <div className="flex flex-wrap gap-2">
                          {template.variables.map((variable) => (
                            <span
                              key={variable}
                              className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800"
                            >
                              {`{{${variable}}}`}
                            </span>
                          ))}
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        {/* Settings Tab */}
        {activeTab === "settings" && settings && (
          <div className="space-y-6">
            <h2 className="text-lg font-medium text-gray-900">Pengaturan Notifikasi</h2>

            <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
              {/* General Settings */}
              <div className="bg-white rounded-lg shadow">
                <div className="px-6 py-4 border-b border-gray-200">
                  <h3 className="text-lg font-medium text-gray-900 flex items-center">
                    <CogIcon className="h-5 w-5 text-gray-400 mr-2" />
                    Pengaturan Umum
                  </h3>
                </div>
                <div className="px-6 py-4 space-y-4">
                  <div className="flex items-center justify-between">
                    <div>
                      <label className="text-sm font-medium text-gray-700">Notifikasi Email</label>
                      <p className="text-sm text-gray-500">Aktifkan sistem notifikasi email</p>
                    </div>
                    <button
                      onClick={() => handleSettingsChange('email_notifications', !settings.email_notifications)}
                      className={`relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 ${
                        settings.email_notifications ? 'bg-blue-600' : 'bg-gray-200'
                      }`}
                    >
                      <span
                        className={`pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out ${
                          settings.email_notifications ? 'translate-x-5' : 'translate-x-0'
                        }`}
                      />
                    </button>
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      Email Admin
                    </label>
                    <input
                      type="email"
                      value={settings.admin_email}
                      onChange={(e) => handleSettingsChange('admin_email', e.target.value)}
                      className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                    />
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      Frekuensi Notifikasi
                    </label>
                    <select
                      value={settings.notification_frequency}
                      onChange={(e) => handleSettingsChange('notification_frequency', e.target.value)}
                      className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                    >
                      <option value="immediate">Segera</option>
                      <option value="daily">Harian</option>
                      <option value="weekly">Mingguan</option>
                    </select>
                  </div>
                </div>
              </div>

              {/* Notification Types */}
              <div className="bg-white rounded-lg shadow">
                <div className="px-6 py-4 border-b border-gray-200">
                  <h3 className="text-lg font-medium text-gray-900 flex items-center">
                    <BellIcon className="h-5 w-5 text-gray-400 mr-2" />
                    Jenis Notifikasi
                  </h3>
                </div>
                <div className="px-6 py-4 space-y-4">
                  <div className="flex items-center justify-between">
                    <div>
                      <label className="text-sm font-medium text-gray-700">Update Status Perizinan</label>
                      <p className="text-sm text-gray-500">Notifikasi persetujuan/penolakan perizinan</p>
                    </div>
                    <button
                      onClick={() => handleSettingsChange('license_status_updates', !settings.license_status_updates)}
                      className={`relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 ${
                        settings.license_status_updates ? 'bg-blue-600' : 'bg-gray-200'
                      }`}
                    >
                      <span
                        className={`pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out ${
                          settings.license_status_updates ? 'translate-x-5' : 'translate-x-0'
                        }`}
                      />
                    </button>
                  </div>

                  <div className="flex items-center justify-between">
                    <div>
                      <label className="text-sm font-medium text-gray-700">Pengingat Pembayaran</label>
                      <p className="text-sm text-gray-500">Reminder jatuh tempo pembayaran</p>
                    </div>
                    <button
                      onClick={() => handleSettingsChange('payment_reminders', !settings.payment_reminders)}
                      className={`relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 ${
                        settings.payment_reminders ? 'bg-blue-600' : 'bg-gray-200'
                      }`}
                    >
                      <span
                        className={`pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out ${
                          settings.payment_reminders ? 'translate-x-5' : 'translate-x-0'
                        }`}
                      />
                    </button>
                  </div>

                  <div className="flex items-center justify-between">
                    <div>
                      <label className="text-sm font-medium text-gray-700">Alert Sistem</label>
                      <p className="text-sm text-gray-500">Notifikasi error dan maintenance</p>
                    </div>
                    <button
                      onClick={() => handleSettingsChange('system_alerts', !settings.system_alerts)}
                      className={`relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 ${
                        settings.system_alerts ? 'bg-blue-600' : 'bg-gray-200'
                      }`}
                    >
                      <span
                        className={`pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out ${
                          settings.system_alerts ? 'translate-x-5' : 'translate-x-0'
                        }`}
                      />
                    </button>
                  </div>

                  <div className="flex items-center justify-between">
                    <div>
                      <label className="text-sm font-medium text-gray-700">Laporan Mingguan</label>
                      <p className="text-sm text-gray-500">Ringkasan aktivitas mingguan</p>
                    </div>
                    <button
                      onClick={() => handleSettingsChange('weekly_reports', !settings.weekly_reports)}
                      className={`relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 ${
                        settings.weekly_reports ? 'bg-blue-600' : 'bg-gray-200'
                      }`}
                    >
                      <span
                        className={`pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out ${
                          settings.weekly_reports ? 'translate-x-5' : 'translate-x-0'
                        }`}
                      />
                    </button>
                  </div>
                </div>
              </div>
            </div>

            {/* Save Button */}
            <div className="flex justify-end">
              <button
                onClick={saveSettings}
                className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700"
              >
                Simpan Pengaturan
              </button>
            </div>
          </div>
        )}

        {/* Email Statistics */}
        <div className="mt-8 bg-white rounded-lg shadow">
          <div className="px-6 py-4 border-b border-gray-200">
            <h3 className="text-lg font-medium text-gray-900">Statistik Email (30 hari terakhir)</h3>
          </div>
          <div className="px-6 py-4">
            <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
              <div className="text-center">
                <div className="text-2xl font-semibold text-gray-900">1,234</div>
                <div className="text-sm text-gray-500">Email Terkirim</div>
              </div>
              <div className="text-center">
                <div className="text-2xl font-semibold text-green-600">89%</div>
                <div className="text-sm text-gray-500">Delivery Rate</div>
              </div>
              <div className="text-center">
                <div className="text-2xl font-semibold text-blue-600">67%</div>
                <div className="text-sm text-gray-500">Open Rate</div>
              </div>
              <div className="text-center">
                <div className="text-2xl font-semibold text-purple-600">23%</div>
                <div className="text-sm text-gray-500">Click Rate</div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
