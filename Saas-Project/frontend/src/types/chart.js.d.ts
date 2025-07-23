declare module "chart.js" {
  import {
    Chart as ChartJS,
    ChartType,
    ChartData,
    ChartOptions,
  } from "chart.js";

  export const Chart: typeof ChartJS;
  export const registerables: unknown[];

  namespace Chart {
    interface ChartHelpers {
      each: (
        instances: Record<string, unknown>,
        callback: (instance: Chart) => void
      ) => void;
    }

    const helpers: ChartHelpers;
    const instances: Chart[];
  }

  export { ChartType, ChartData, ChartOptions };
}
