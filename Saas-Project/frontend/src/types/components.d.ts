// Deklarasi tipe untuk komponen-komponen yang diimpor di LazyComponents.tsx

declare module "../licenses/LicenseApplicationForm" {
  const LicenseApplicationForm: React.ComponentType;
  export default LicenseApplicationForm;
}

declare module "../licenses/LicenseDetail" {
  const LicenseDetail: React.ComponentType;
  export default LicenseDetail;
}

declare module "../licenses/DocumentUpload" {
  const DocumentUpload: React.ComponentType;
  export default DocumentUpload;
}

declare module "../licenses/LicenseList" {
  const LicenseList: React.ComponentType;
  export default LicenseList;
}

declare module "../analytics/AnalyticsDashboard" {
  const AnalyticsDashboard: React.ComponentType;
  export default AnalyticsDashboard;
}

declare module "../analytics/PerformanceMetrics" {
  const PerformanceMetrics: React.ComponentType;
  export default PerformanceMetrics;
}

declare module "../security/TwoFactorSetup" {
  const TwoFactorSetup: React.ComponentType;
  export default TwoFactorSetup;
}
