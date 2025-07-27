// Mock API Toggle Component
// Provides a simple UI to toggle between mock API and real API

"use client";

import React, { useState, useEffect } from "react";
import { mockConfig } from "../../mocks";

export default function MockApiToggle() {
  const [isMockEnabled, setIsMockEnabled] = useState(false);
  const [showToggle, setShowToggle] = useState(false);

  // Check if we're in development mode
  useEffect(() => {
    setShowToggle(process.env.NODE_ENV === "development");
    
    // Safely access mockConfig if available
    try {
      setIsMockEnabled(mockConfig?.enabled || false);
    } catch (error) {
      console.warn("Mock API config not available:", error);
      setIsMockEnabled(false);
    }
  }, []);

  // Toggle mock API
  const toggleMockApi = () => {
    const newState = !isMockEnabled;
    // In a real implementation, we would need to update the runtime config
    // For now, we'll just show this UI indicator and require a reload
    setIsMockEnabled(newState);
    
    // Store the preference in localStorage so it persists across reloads
    localStorage.setItem("useMockApi", newState ? "true" : "false");
    
    // Alert the user that they need to reload
    window.alert(`Mock API ${newState ? "enabled" : "disabled"}. Please reload the page for the changes to take effect.`);
  };

  if (!showToggle) return null;

  return (
    <div className="fixed bottom-4 right-4 z-50 bg-gray-800 text-white px-4 py-2 rounded-lg shadow-lg flex items-center space-x-2">
      <span className="text-xs">Mock API:</span>
      <button
        onClick={toggleMockApi}
        className={`px-2 py-1 text-xs rounded ${
          isMockEnabled ? "bg-green-500" : "bg-red-500"
        }`}
      >
        {isMockEnabled ? "ON" : "OFF"}
      </button>
    </div>
  );
}
