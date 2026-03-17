<script lang="ts">
  import { categories } from "../stores/categories";
  import { activeCategory, refreshClips } from "../stores/clips";
  import type { Category } from "../ipc";

  interface Props { onAddCategory: () => void; }
  let { onAddCategory }: Props = $props();

  let categoryList: Category[] = $state([]);
  categories.subscribe((v) => (categoryList = v));
  let active: "all" | "pins" | number = $state("all");
  activeCategory.subscribe((v) => (active = v));

  function select(cat: "all" | "pins" | number) {
    activeCategory.set(cat);
    refreshClips();
  }
</script>

<nav class="tabs" style="-webkit-app-region: no-drag">
  <button class="tab" class:active={active === "all"} onclick={() => select("all")}>All</button>
  <button class="tab" class:active={active === "pins"} onclick={() => select("pins")} aria-label="Pinned">
    <svg viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round" width="10" height="10">
      <path d="M6 1L8 4.5V7L9.5 8H2.5L4 7V4.5Z" /><line x1="6" y1="8" x2="6" y2="11.5" />
    </svg>
  </button>
  {#each categoryList as cat (cat.id)}
    <button class="tab" class:active={active === cat.id} onclick={() => select(cat.id)} title={cat.name}>{cat.icon}</button>
  {/each}
  <button class="tab add" onclick={onAddCategory} title="New category">+</button>
</nav>

<style>
  .tabs { display: flex; align-items: center; gap: 1px; }
  .tab {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 3px 9px;
    border: none;
    background: transparent;
    color: var(--text-tertiary);
    cursor: pointer;
    border-radius: var(--r-sm);
    font-size: 11px;
    font-family: var(--font-ui);
    font-weight: 500;
    transition: color var(--t-fast), background var(--t-fast);
  }
  .tab:hover { color: var(--text-secondary); background: var(--bg-hover); }
  .tab.active { color: var(--accent-text); background: var(--accent-dim); }
  .tab.active:hover { background: var(--accent-hover); }
  .add { color: var(--text-ghost); padding: 3px 5px; }
  .add:hover { color: var(--text-tertiary); }
</style>
