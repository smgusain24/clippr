import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";

export interface Clip {
  id: number;
  content: string;
  preview: string;
  created_at: string;
  pinned: boolean;
  category_id: number | null;
  category_name: string | null;
}

export interface Category {
  id: number;
  name: string;
  icon: string;
  sort_order: number;
}

export async function getClips(
  categoryId?: number | null,
  pinnedOnly: boolean = false,
  search?: string
): Promise<Clip[]> {
  return invoke("get_clips", {
    categoryId: categoryId ?? null,
    pinnedOnly,
    search: search ?? null,
  });
}

export async function deleteClip(id: number): Promise<void> {
  return invoke("delete_clip", { id });
}

export async function togglePin(id: number): Promise<boolean> {
  return invoke("toggle_pin", { id });
}

export async function setClipCategory(
  clipId: number,
  categoryId: number | null
): Promise<void> {
  return invoke("set_clip_category", { clipId, categoryId });
}

export async function getCategories(): Promise<Category[]> {
  return invoke("get_categories");
}

export async function addCategory(
  name: string,
  icon: string
): Promise<Category> {
  return invoke("add_category", { name, icon });
}

export async function deleteCategory(id: number): Promise<void> {
  return invoke("delete_category", { id });
}

export async function clearHistory(): Promise<void> {
  return invoke("clear_history");
}

export async function copyClipToClipboard(id: number): Promise<void> {
  const content: string = await invoke("copy_clip_to_clipboard", { id });
  await writeText(content);
}

export async function hideWindow(): Promise<void> {
  return invoke("hide_window");
}

export function onClipAdded(callback: () => void): Promise<() => void> {
  return listen("clip-added", () => callback()).then((unlisten) => unlisten);
}
