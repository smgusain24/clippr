import { writable, derived } from "svelte/store";
import type { Clip } from "../ipc";
import { getClips } from "../ipc";

export const clips = writable<Clip[]>([]);
export const searchQuery = writable("");
export const activeCategory = writable<"all" | "pins" | number>("all");

export async function refreshClips() {
  let categoryId: number | null = null;
  let pinnedOnly = false;
  let search: string | undefined;

  const cat = getCategoryValue();
  if (cat === "pins") {
    pinnedOnly = true;
  } else if (typeof cat === "number") {
    categoryId = cat;
  }

  const q = getSearchValue();
  if (q) search = q;

  const result = await getClips(categoryId, pinnedOnly, search);
  clips.set(result);
}

// Helper to get current store values synchronously
let currentCategory: "all" | "pins" | number = "all";
let currentSearch = "";

activeCategory.subscribe((v) => (currentCategory = v));
searchQuery.subscribe((v) => (currentSearch = v));

function getCategoryValue() {
  return currentCategory;
}
function getSearchValue() {
  return currentSearch;
}
