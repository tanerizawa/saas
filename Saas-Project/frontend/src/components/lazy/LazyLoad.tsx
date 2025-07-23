import dynamic from "next/dynamic";
import { Suspense, ComponentType, ReactNode } from "react";

// Loading fallback component
interface LoadingProps {
  height?: string;
  message?: string;
}

export const LazyLoadingFallback: React.FC<LoadingProps> = ({
  height = "200px",
  message = "Loading...",
}) => (
  <div
    style={{
      display: "flex",
      justifyContent: "center",
      alignItems: "center",
      height,
      width: "100%",
    }}
  >
    <div className="flex flex-col items-center justify-center">
      <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-primary mb-2"></div>
      <p className="text-gray-500">{message}</p>
    </div>
  </div>
);

// Helper function to create lazily loaded components
export function createLazyComponent<T>(
  importFunc: () => Promise<{ default: ComponentType<T> }>,
  loadingProps: LoadingProps = {}
) {
  const LazyComponent = dynamic(importFunc, {
    loading: () => <LazyLoadingFallback {...loadingProps} />,
    ssr: false, // Disable SSR for improved performance
  });

  return function LazyComponentWrapper(props: T & { children?: ReactNode }) {
    return (
      <Suspense fallback={<LazyLoadingFallback {...loadingProps} />}>
        <LazyComponent {...props} />
      </Suspense>
    );
  };
}
