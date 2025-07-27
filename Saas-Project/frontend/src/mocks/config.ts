// Mock API Configuration
// This file provides configuration for the mock API service
// It allows toggling between real API and mock data

interface MockConfig {
  enabled: boolean;
  delay: number; // Simulated network delay in ms
}

// Mock API configuration
export const mockConfig: MockConfig = {
  enabled: process.env.NEXT_PUBLIC_USE_MOCK_API === 'true',
  delay: 500,
};

// Helper for simulating API latency
export const delay = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

// Helper for getting mock data with simulated latency
export async function getMockData<T>(data: T): Promise<T> {
  if (mockConfig.delay > 0) {
    await delay(mockConfig.delay);
  }
  return data;
}
