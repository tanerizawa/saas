"use client";

import { useState, useEffect } from "react";
import { Chart, registerables } from "chart.js";

// Register Chart.js components
Chart.register(...registerables);

type AnalyticsData = {
  licensesByType: Record<string, number>;
  licensesByStatus: Record<string, number>;
  licensesByMonth: Record<string, number>;
  processingTimes: Record<string, number>;
};

const AnalyticsDashboard = () => {
  const [data, setData] = useState<AnalyticsData | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchData = async () => {
      try {
        setLoading(true);

        // Simulate API call
        await new Promise((resolve) => setTimeout(resolve, 1000));

        // Mocked data
        setData({
          licensesByType: {
            NIB: 24,
            SIUP: 18,
            TDP: 15,
            NPWP: 21,
            HALAL: 7,
          },
          licensesByStatus: {
            approved: 45,
            pending: 23,
            rejected: 12,
            draft: 5,
          },
          licensesByMonth: {
            Jan: 12,
            Feb: 15,
            Mar: 8,
            Apr: 14,
            May: 18,
            Jun: 24,
            Jul: 20,
          },
          processingTimes: {
            NIB: 7.2,
            SIUP: 12.5,
            TDP: 8.3,
            NPWP: 5.1,
            HALAL: 14.8,
          },
        });
      } catch (err) {
        setError("Failed to load analytics data");
        console.error(err);
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, []);

  useEffect(() => {
    if (!data || !document) return;

    // Create charts when data is available
    const createCharts = () => {
      // License by type chart
      const typeCtx = document.getElementById(
        "licenseTypeChart"
      ) as HTMLCanvasElement;
      if (typeCtx) {
        new Chart(typeCtx, {
          type: "pie",
          data: {
            labels: Object.keys(data.licensesByType),
            datasets: [
              {
                data: Object.values(data.licensesByType),
                backgroundColor: [
                  "#4299E1",
                  "#48BB78",
                  "#F6AD55",
                  "#F56565",
                  "#9F7AEA",
                ],
                borderWidth: 1,
              },
            ],
          },
          options: {
            responsive: true,
            plugins: {
              legend: {
                position: "bottom",
              },
              title: {
                display: true,
                text: "Perizinan Berdasarkan Jenis",
              },
            },
          },
        });
      }

      // License by status chart
      const statusCtx = document.getElementById(
        "licenseStatusChart"
      ) as HTMLCanvasElement;
      if (statusCtx) {
        new Chart(statusCtx, {
          type: "doughnut",
          data: {
            labels: Object.keys(data.licensesByStatus).map((status) =>
              status === "approved"
                ? "Disetujui"
                : status === "pending"
                ? "Diproses"
                : status === "rejected"
                ? "Ditolak"
                : "Draft"
            ),
            datasets: [
              {
                data: Object.values(data.licensesByStatus),
                backgroundColor: ["#48BB78", "#F6AD55", "#F56565", "#A0AEC0"],
                borderWidth: 1,
              },
            ],
          },
          options: {
            responsive: true,
            plugins: {
              legend: {
                position: "bottom",
              },
              title: {
                display: true,
                text: "Status Perizinan",
              },
            },
          },
        });
      }

      // License by month chart
      const monthCtx = document.getElementById(
        "licenseMonthChart"
      ) as HTMLCanvasElement;
      if (monthCtx) {
        new Chart(monthCtx, {
          type: "bar",
          data: {
            labels: Object.keys(data.licensesByMonth),
            datasets: [
              {
                label: "Perizinan",
                data: Object.values(data.licensesByMonth),
                backgroundColor: "#4299E1",
                borderColor: "#2B6CB0",
                borderWidth: 1,
              },
            ],
          },
          options: {
            responsive: true,
            scales: {
              y: {
                beginAtZero: true,
                title: {
                  display: true,
                  text: "Jumlah Perizinan",
                },
              },
              x: {
                title: {
                  display: true,
                  text: "Bulan",
                },
              },
            },
            plugins: {
              title: {
                display: true,
                text: "Perizinan per Bulan",
              },
            },
          },
        });
      }

      // Processing times chart
      const timeCtx = document.getElementById(
        "processingTimeChart"
      ) as HTMLCanvasElement;
      if (timeCtx) {
        new Chart(timeCtx, {
          type: "bar",
          data: {
            labels: Object.keys(data.processingTimes),
            datasets: [
              {
                label: "Waktu Proses (Hari)",
                data: Object.values(data.processingTimes),
                backgroundColor: "#9F7AEA",
                borderColor: "#6B46C1",
                borderWidth: 1,
              },
            ],
          },
          options: {
            responsive: true,
            scales: {
              y: {
                beginAtZero: true,
                title: {
                  display: true,
                  text: "Rata-rata Waktu (Hari)",
                },
              },
            },
            plugins: {
              title: {
                display: true,
                text: "Rata-rata Waktu Pemrosesan",
              },
            },
          },
        });
      }
    };

    createCharts();

    // Cleanup function to prevent memory leaks
    return () => {
      Chart.helpers.each(Chart.instances, (instance) => {
        instance.destroy();
      });
    };
  }, [data]);

  if (loading) {
    return (
      <div className="flex justify-center items-center h-96">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500"></div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded-md">
        <p>{error}</p>
      </div>
    );
  }

  return (
    <div className="bg-white rounded-lg shadow-sm p-6">
      <h2 className="text-2xl font-bold mb-6">Dashboard Analitik</h2>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
        <div className="bg-blue-50 rounded-lg p-4">
          <h3 className="text-lg font-semibold mb-2">Total Perizinan</h3>
          <p className="text-4xl font-bold">
            {data
              ? Object.values(data.licensesByType).reduce(
                  (sum, val) => sum + val,
                  0
                )
              : 0}
          </p>
        </div>

        <div className="bg-green-50 rounded-lg p-4">
          <h3 className="text-lg font-semibold mb-2">Perizinan Disetujui</h3>
          <p className="text-4xl font-bold text-green-600">
            {data?.licensesByStatus?.approved || 0}
          </p>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
        <div>
          <canvas id="licenseTypeChart" width="400" height="300"></canvas>
        </div>
        <div>
          <canvas id="licenseStatusChart" width="400" height="300"></canvas>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div>
          <canvas id="licenseMonthChart" width="400" height="300"></canvas>
        </div>
        <div>
          <canvas id="processingTimeChart" width="400" height="300"></canvas>
        </div>
      </div>
    </div>
  );
};

export default AnalyticsDashboard;
