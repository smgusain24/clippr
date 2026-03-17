<script lang="ts">
  import { clips } from "../stores/clips";
  import type { Clip } from "../ipc";
  import ClipItem from "./ClipItem.svelte";

  let clipList: Clip[] = $state([]);
  clips.subscribe((v) => (clipList = v));
</script>

<div class="list">
  {#if clipList.length === 0}
    <div class="empty">
      <span class="empty-label">No clips yet</span>
      <span class="empty-hint">Copy something to get started</span>
    </div>
  {:else}
    {#each clipList as clip (clip.id)}
      <ClipItem {clip} />
    {/each}
  {/if}
</div>

<style>
  .list { flex: 1; overflow-y: auto; overflow-x: hidden; padding: 2px 0; }
  .empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    gap: 4px;
    padding-bottom: 40px;
  }
  .empty-label { font-size: 13px; font-weight: 500; color: var(--text-tertiary); }
  .empty-hint { font-size: 11px; color: var(--text-ghost); }
</style>
