declare module "tailwind-merge" {
  /**
   * Merges multiple Tailwind CSS class strings together, resolving conflicts between them.
   * @param classLists - The class lists to merge.
   */
  export function twMerge(...classLists: string[]): string;
}
