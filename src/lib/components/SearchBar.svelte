<script lang="ts">
  import { searchQuery, refreshClips } from "../stores/clips";

  let value = $state("");
  let focused = $state(false);
  let debounceTimer: ReturnType<typeof setTimeout>;

  function handleInput(e: Event) {
    value = (e.target as HTMLInputElement).value;
    clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      searchQuery.set(value);
      refreshClips();
    }, 200);
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape" && value) {
      e.stopPropagation();
      value = "";
      searchQuery.set("");
      refreshClips();
    }
  }

  function clear() {
    value = "";
    searchQuery.set("");
    refreshClips();
  }
</script>

<div class="wrap">
  <div class="bar" class:focused>
    <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" width="13" height="13" stroke-width="1.8" stroke-linecap="round">
      <circle cx="6.5" cy="6.5" r="4" /><line x1="9.5" y1="9.5" x2="13.5" y2="13.5" />
    </svg>
    <input
      type="text"
      placeholder="Search clips..."
      {value}
      oninput={handleInput}
      onkeydown={handleKeydown}
      onfocus={() => (focused = true)}
      onblur={() => (focused = false)}
    />
    {#if value}
      <button class="x" onclick={clear} aria-label="Clear search">×</button>
    {/if}
  </div>
</div>

<style>
  .wrap { padding: 0 10px 8px; flex-shrink: 0; }
  .bar {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 7px 10px;
    border: 1px solid var(--border-subtle);
    border-radius: var(--r-md);
    background: var(--bg-elevated);
    transition: border-color var(--t-norm), box-shadow var(--t-norm);
  }
  .bar:hover { border-color: var(--border-default); }
  .bar.focused {
    border-color: var(--accent);
    box-shadow: 0 0 0 2px var(--accent-dim);
  }
  .bar svg {
    color: var(--text-ghost);
    flex-shrink: 0;
    transition: color var(--t-norm);
  }
  .focused svg { color: var(--accent); }
  input {
    flex: 1;
    border: none;
    background: transparent;
    color: var(--text-primary);
    font-size: 12.5px;
    font-family: var(--font-ui);
    outline: none;
  }
  input::placeholder { color: var(--text-ghost); }
  .x {
    background: var(--bg-hover);
    border: none;
    color: var(--text-tertiary);
    cursor: pointer;
    width: 16px;
    height: 16px;
    border-radius: 50%;
    font-size: 12px;
    line-height: 16px;
    text-align: center;
    padding: 0;
    transition: background var(--t-fast), color var(--t-fast);
  }
  .x:hover { background: var(--bg-active); color: var(--text-primary); }
</style>
