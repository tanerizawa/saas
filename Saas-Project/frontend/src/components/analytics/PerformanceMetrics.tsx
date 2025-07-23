"use client";

import { useState, useEffect } from "react";

type MetricsData = {
  apiResponseTime: number[];
  errorRate: number;
  cacheHitRate: number;
  memoryUsage: number;
  cpuUsage: number;
  databaseQueryTime: number[];
  activeUsers: number[];
  timeLabels: string[];
};

const PerformanceMetrics = () => {
  const [data, setData] = useState<MetricsData | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchData = async () => {
      try {
        setLoading(true);

        // Simulate API call
        await new Promise((resolve) => setTimeout(resolve, 800));

        // Mocked data
        const timeLabels = Array.from({ length: 24 }, (_, i) => `${i}:00`);

        setData({
          apiResponseTime: Array.from(
            { length: 24 },
            () => Math.floor(Math.random() * 80) + 40
          ),
          errorRate: Number((Math.random() * 1).toFixed(2)),
          cacheHitRate: Number((Math.random() * 40 + 60).toFixed(2)),
          memoryUsage: Number((Math.random() * 30 + 30).toFixed(2)),
          cpuUsage: Number((Math.random() * 40 + 20).toFixed(2)),
          databaseQueryTime: Array.from(
            { length: 24 },
            () => Math.floor(Math.random() * 40) + 20
          ),
          activeUsers: Array.from(
            { length: 24 },
            () => Math.floor(Math.random() * 50) + 10
          ),
          timeLabels,
        });
      } catch (err) {
        setError("Failed to load performance metrics");
        console.error(err);
      } finally {
        setLoading(false);
      }
    };

    fetchData();

    // Set up interval for updating metrics
    const interval = setInterval(fetchData, 300000); // Update every 5 minutes

    return () => clearInterval(interval);
  }, []);

  if (loading) {
    return (
      <div className="flex justify-center items-center h-64">
        <div className="animate-spin rounded-full h-10 w-10 border-b-2 border-blue-500"></div>
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

  const calculateAverage = (arr: number[]) => {
    return arr.reduce((sum, val) => sum + val, 0) / arr.length;
  };

  return (
    <div className="bg-white rounded-lg shadow-sm p-6">
      <h2 className="text-2xl font-bold mb-6">Performance Metrics</h2>

      <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-8">
        <div className="bg-blue-50 rounded-lg p-4">
          <h3 className="text-sm font-medium text-gray-500">
            Avg API Response
          </h3>
          <p className="text-2xl font-bold">
            {data
              ? `${calculateAverage(data.apiResponseTime).toFixed(1)}ms`
              : "0ms"}
          </p>
        </div>

        <div className="bg-red-50 rounded-lg p-4">
          <h3 className="text-sm font-medium text-gray-500">Error Rate</h3>
          <p className="text-2xl font-bold">
            {data ? `${data.errorRate}%` : "0%"}
          </p>
        </div>

        <div className="bg-green-50 rounded-lg p-4">
          <h3 className="text-sm font-medium text-gray-500">Cache Hit Rate</h3>
          <p className="text-2xl font-bold">
            {data ? `${data.cacheHitRate}%` : "0%"}
          </p>
        </div>

        <div className="bg-purple-50 rounded-lg p-4">
          <h3 className="text-sm font-medium text-gray-500">Active Users</h3>
          <p className="text-2xl font-bold">
            {data ? data.activeUsers[data.activeUsers.length - 1] : "0"}
          </p>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
        <div className="bg-gray-50 rounded-lg p-4">
          <h3 className="text-lg font-medium mb-4">System Resources</h3>

          <div className="space-y-4">
            <div>
              <div className="flex justify-between mb-1">
                <span className="text-sm font-medium">Memory Usage</span>
                <span className="text-sm font-medium">
                  {data?.memoryUsage}%
                </span>
              </div>
              <div className="w-full bg-gray-200 rounded-full h-2.5">
                <div
                  className="bg-blue-600 h-2.5 rounded-full"
                  style={{ width: `${data?.memoryUsage}%` }}
                ></div>
              </div>
            </div>

            <div>
              <div className="flex justify-between mb-1">
                <span className="text-sm font-medium">CPU Usage</span>
                <span className="text-sm font-medium">{data?.cpuUsage}%</span>
              </div>
              <div className="w-full bg-gray-200 rounded-full h-2.5">
                <div
                  className="bg-green-600 h-2.5 rounded-full"
                  style={{ width: `${data?.cpuUsage}%` }}
                ></div>
              </div>
            </div>
          </div>
        </div>

        <div className="bg-gray-50 rounded-lg p-4">
          <h3 className="text-lg font-medium mb-2">Database Performance</h3>
          <div className="flex flex-col space-y-2">
            <div>
              <p className="text-sm text-gray-600">Average Query Time</p>
              <p className="text-xl font-semibold">
                {data
                  ? `${calculateAverage(data.databaseQueryTime).toFixed(1)}ms`
                  : "0ms"}
              </p>
            </div>

            <div>
              <p className="text-sm text-gray-600">Queries Per Second</p>
              <p className="text-xl font-semibold">42</p>
            </div>

            <div>
              <p className="text-sm text-gray-600">Connection Pool Usage</p>
              <p className="text-xl font-semibold">18/25</p>
            </div>
          </div>
        </div>
      </div>

      <div className="bg-gray-50 rounded-lg p-4">
        <h3 className="text-lg font-medium mb-4">Response Time Trends</h3>
        <div className="relative h-60">
          {data && (
            <div className="absolute inset-0 flex items-end">
              {data.apiResponseTime.map((time, index) => (
                <div
                  key={index}
                  className="flex-1 bg-blue-500 hover:bg-blue-600 transition-all"
                  style={{ height: `${(time / 200) * 100}%` }}
                  title={`${data.timeLabels[index]}: ${time}ms`}
                ></div>
              ))}
            </div>
          )}
        </div>
        <div className="flex justify-between mt-2 text-xs text-gray-500">
          {data?.timeLabels
            .filter((_, i) => i % 4 === 0)
            .map((label) => (
              <span key={label}>{label}</span>
            ))}
        </div>
      </div>
    </div>
  );
};

export default PerformanceMetrics;
