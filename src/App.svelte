<script lang="ts">
  import "./app.css";
  import Sidebar from "./lib/components/Sidebar.svelte";
  import SearchBar from "./lib/components/SearchBar.svelte";
  import ClipList from "./lib/components/ClipList.svelte";
  import CategoryModal from "./lib/components/CategoryModal.svelte";
  import { refreshClips, clips } from "./lib/stores/clips";
  import { refreshCategories } from "./lib/stores/categories";
  import { onClipAdded, hideWindow, clearHistory } from "./lib/ipc";
  import { onMount } from "svelte";

  let showCategoryModal = $state(false);
  let clipCount = $state(0);
  clips.subscribe((v) => (clipCount = v.length));

  onMount(async () => {
    await refreshCategories();
    await refreshClips();
    const unlistenClip = await onClipAdded(() => refreshClips());
    window.addEventListener("keydown", (e: KeyboardEvent) => {
      if (e.key === "Escape") hideWindow();
    });
    return () => unlistenClip();
  });

  async function handleClearAll() {
    await clearHistory();
    await refreshClips();
  }
</script>

<div class="shell">
  <div class="popover">
    <div class="titlebar">
      <div class="titlebar-left">
        <span class="app-name">clippr</span>
        <span class="clip-count">{clipCount}</span>
      </div>
      <Sidebar onAddCategory={() => (showCategoryModal = true)} />
    </div>
    <SearchBar />
    <ClipList />
    <div class="footer">
      <span class="hint"><kbd>click</kbd> copy <kbd>esc</kbd> close</span>
      <button class="clear-btn" onclick={handleClearAll}>Clear all</button>
    </div>
  </div>
</div>

<CategoryModal visible={showCategoryModal} onClose={() => (showCategoryModal = false)} />

<style>
  .shell {
    height: 100%;
    padding: 1px;
    background: transparent;
  }
  .popover {
    display: flex;
    flex-direction: column;
    height: 100%;
    border-radius: var(--r-lg);
    overflow: hidden;
    background: var(--bg-base);
    box-shadow:
      0 0 0 0.5px rgba(255, 245, 230, 0.08),
      0 8px 40px rgba(0, 0, 0, 0.6);
  }
  .titlebar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 14px 8px;
    -webkit-app-region: drag;
    flex-shrink: 0;
  }
  .titlebar-left {
    display: flex;
    align-items: center;
    gap: 8px;
  }
  .app-name {
    font-family: var(--font-mono);
    font-size: 13px;
    font-weight: 500;
    color: var(--accent-text);
    letter-spacing: -0.3px;
  }
  .clip-count {
    font-family: var(--font-mono);
    font-size: 10px;
    color: var(--text-tertiary);
    background: var(--bg-surface);
    padding: 1px 6px;
    border-radius: 10px;
  }
  .footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 6px 14px;
    border-top: 1px solid var(--border-subtle);
    flex-shrink: 0;
  }
  .hint {
    font-size: 10px;
    color: var(--text-ghost);
    display: flex;
    align-items: center;
    gap: 3px;
  }
  .hint kbd {
    font-family: var(--font-mono);
    font-size: 9px;
    padding: 1px 4px;
    background: var(--bg-surface);
    border-radius: 3px;
    border: 0.5px solid var(--border-default);
    color: var(--text-tertiary);
  }
  .clear-btn {
    font-family: var(--font-ui);
    font-size: 10px;
    color: var(--text-ghost);
    background: none;
    border: none;
    cursor: pointer;
    padding: 2px 6px;
    border-radius: var(--r-sm);
    transition: color var(--t-fast), background var(--t-fast);
  }
  .clear-btn:hover {
    color: var(--red);
    background: var(--red-dim);
  }
</style>
