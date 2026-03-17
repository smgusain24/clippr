import { writable } from "svelte/store";
import type { Category } from "../ipc";
import { getCategories } from "../ipc";

export const categories = writable<Category[]>([]);

export async function refreshCategories() {
  const result = await getCategories();
  categories.set(result);
}
