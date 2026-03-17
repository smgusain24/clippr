<script lang="ts">
  import type { Clip, Category } from "../ipc";
  import { copyClipToClipboard, deleteClip, togglePin, setClipCategory, hideWindow } from "../ipc";
  import { refreshClips } from "../stores/clips";
  import { categories } from "../stores/categories";

  interface Props { clip: Clip; }
  let { clip }: Props = $props();
  let showMenu = $state(false);
  let showCatMenu = $state(false);
  let hovered = $state(false);
  let categoryList: Category[] = $state([]);
  categories.subscribe((v) => (categoryList = v));

  function timeAgo(d: string): string {
    const s = Math.floor((Date.now() - new Date(d + "Z").getTime()) / 1000);
    if (s < 60) return "now";
    const m = Math.floor(s / 60);
    if (m < 60) return `${m}m`;
    const h = Math.floor(m / 60);
    if (h < 24) return `${h}h`;
    return `${Math.floor(h / 24)}d`;
  }

  function isUrl(t: string): boolean { return /^https?:\/\//i.test(t.trim()); }

  async function handleCopy() {
    await copyClipToClipboard(clip.id);
    await hideWindow();
  }

  async function handlePin(e: MouseEvent) {
    e.stopPropagation();
    await togglePin(clip.id);
    await refreshClips();
    showMenu = false;
  }

  async function handleDelete() {
    await deleteClip(clip.id);
    await refreshClips();
    showMenu = false;
  }

  async function handleSetCat(catId: number | null) {
    await setClipCategory(clip.id, catId);
    await refreshClips();
    showCatMenu = false;
    showMenu = false;
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="clip"
  onclick={handleCopy}
  oncontextmenu={(e) => { e.preventDefault(); showMenu = !showMenu; }}
  onkeydown={(e) => e.key === "Enter" && handleCopy()}
  onmouseenter={() => (hovered = true)}
  onmouseleave={() => (hovered = false)}
  tabindex="0"
  role="button"
>
  {#if clip.pinned}<div class="pin-bar"></div>{/if}
  <div class="body">
    <div class="text" class:url={isUrl(clip.content)}>{clip.preview}</div>
    <div class="meta">
      <span>{timeAgo(clip.created_at)}</span>
      {#if clip.category_name}<span class="tag">{clip.category_name}</span>{/if}
    </div>
  </div>
  {#if hovered || clip.pinned}
    <button class="pin" class:pinned={clip.pinned} onclick={handlePin} title={clip.pinned ? "Unpin" : "Pin"}>
      <svg viewBox="0 0 12 12" fill={clip.pinned ? "currentColor" : "none"} stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round" width="11" height="11">
        <path d="M6 1L8 4.5V7L9.5 8H2.5L4 7V4.5Z" /><line x1="6" y1="8" x2="6" y2="11.5" />
      </svg>
    </button>
  {/if}
</div>

{#if showMenu}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="overlay" onclick={() => { showMenu = false; showCatMenu = false; }} onkeydown={() => {}}></div>
  <div class="ctx">
    <button onclick={(e) => handlePin(e)}>{clip.pinned ? "Unpin" : "Pin"}</button>
    <button onclick={() => (showCatMenu = !showCatMenu)}>Category ›</button>
    {#if showCatMenu}
      <div class="sub">
        <button onclick={() => handleSetCat(null)}>None</button>
        {#each categoryList as cat (cat.id)}
          <button onclick={() => handleSetCat(cat.id)}>{cat.icon} {cat.name}</button>
        {/each}
      </div>
    {/if}
    <div class="sep"></div>
    <button class="danger" onclick={handleDelete}>Delete</button>
  </div>
{/if}

<style>
  .clip {
    position: relative;
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    margin: 0 6px;
    border-radius: var(--r-md);
    cursor: pointer;
    transition: background var(--t-fast);
  }
  .clip:hover { background: var(--bg-elevated); }
  .pin-bar {
    position: absolute;
    left: 4px;
    top: 50%;
    transform: translateY(-50%);
    width: 2px;
    height: 14px;
    background: var(--accent);
    border-radius: 1px;
    opacity: 0.7;
  }
  .body { flex: 1; min-width: 0; }
  .text {
    font-family: var(--font-mono);
    font-size: 11.5px;
    line-height: 1.45;
    color: var(--text-secondary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .text.url { color: var(--accent-text); }
  .meta {
    display: flex;
    align-items: center;
    gap: 6px;
    margin-top: 2px;
    font-family: var(--font-mono);
    font-size: 9.5px;
    color: var(--text-ghost);
  }
  .tag {
    background: var(--bg-surface);
    padding: 0 5px;
    border-radius: 3px;
  }
  .pin {
    background: none;
    border: none;
    cursor: pointer;
    color: var(--text-ghost);
    padding: 4px;
    border-radius: var(--r-sm);
    line-height: 0;
    transition: color var(--t-fast), background var(--t-fast);
  }
  .pin:hover { color: var(--text-secondary); background: var(--bg-hover); }
  .pin.pinned { color: var(--accent); }

  .overlay { position: fixed; inset: 0; z-index: 99; }
  .ctx {
    position: absolute;
    right: 20px;
    margin-top: -2px;
    background: var(--bg-elevated);
    border: 0.5px solid var(--border-strong);
    border-radius: var(--r-md);
    padding: 4px;
    z-index: 100;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.5);
    min-width: 130px;
    animation: pop 100ms ease;
  }
  @keyframes pop {
    from { opacity: 0; transform: scale(0.96) translateY(-4px); }
    to { opacity: 1; transform: scale(1) translateY(0); }
  }
  .ctx button {
    display: block;
    width: 100%;
    text-align: left;
    padding: 5px 8px;
    border: none;
    background: transparent;
    color: var(--text-secondary);
    font-size: 12px;
    font-family: var(--font-ui);
    cursor: pointer;
    border-radius: var(--r-sm);
    transition: background var(--t-fast);
  }
  .ctx button:hover { background: var(--bg-hover); }
  .ctx .danger { color: var(--red); }
  .ctx .danger:hover { background: var(--red-dim); }
  .sep { height: 1px; background: var(--border-subtle); margin: 3px 4px; }
  .sub { padding-left: 4px; }
  .sub button { font-size: 11px; padding: 4px 8px; }
</style>
