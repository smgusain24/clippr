<script lang="ts">
  import { addCategory } from "../ipc";
  import { refreshCategories } from "../stores/categories";

  interface Props { visible: boolean; onClose: () => void; }
  let { visible, onClose }: Props = $props();
  let name = $state("");
  let icon = $state("📁");
  const icons = ["📁", "💼", "🏠", "💻", "📝", "🔗", "⭐", "🎨", "📊", "🔧"];

  async function handleSubmit() {
    if (!name.trim()) return;
    await addCategory(name.trim(), icon);
    await refreshCategories();
    name = "";
    icon = "📁";
    onClose();
  }
</script>

{#if visible}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="overlay" onclick={onClose} onkeydown={() => {}}>
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="modal" onclick={(e) => e.stopPropagation()} onkeydown={() => {}}>
      <h3>New Category</h3>
      <div class="icons">
        {#each icons as ic}
          <button class="ic" class:sel={icon === ic} onclick={() => (icon = ic)}>{ic}</button>
        {/each}
      </div>
      <input
        type="text"
        placeholder="Category name"
        bind:value={name}
        onkeydown={(e) => { if (e.key === "Enter") handleSubmit(); if (e.key === "Escape") onClose(); }}
      />
      <div class="actions">
        <button class="btn" onclick={onClose}>Cancel</button>
        <button class="btn primary" onclick={handleSubmit} disabled={!name.trim()}>Create</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.55);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 200;
    backdrop-filter: blur(4px);
    -webkit-backdrop-filter: blur(4px);
  }
  .modal {
    background: var(--bg-elevated);
    border: 0.5px solid var(--border-strong);
    border-radius: var(--r-lg);
    padding: 16px;
    width: 260px;
    box-shadow: 0 8px 40px rgba(0, 0, 0, 0.6);
    animation: pop 150ms ease;
  }
  @keyframes pop {
    from { opacity: 0; transform: scale(0.95) translateY(6px); }
    to { opacity: 1; transform: scale(1) translateY(0); }
  }
  h3 { font-size: 14px; font-weight: 600; color: var(--text-primary); margin-bottom: 12px; }
  .icons { display: flex; flex-wrap: wrap; gap: 4px; margin-bottom: 12px; }
  .ic {
    width: 34px;
    height: 34px;
    border: 1px solid var(--border-subtle);
    border-radius: var(--r-md);
    background: var(--bg-surface);
    cursor: pointer;
    font-size: 16px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: border-color var(--t-fast);
  }
  .ic:hover { border-color: var(--border-strong); }
  .ic.sel { border-color: var(--accent); background: var(--accent-dim); }
  input {
    width: 100%;
    padding: 8px 10px;
    border: 1px solid var(--border-subtle);
    border-radius: var(--r-md);
    background: var(--bg-surface);
    color: var(--text-primary);
    font-size: 12.5px;
    font-family: var(--font-ui);
    outline: none;
    margin-bottom: 14px;
    transition: border-color var(--t-norm);
  }
  input:focus { border-color: var(--accent); box-shadow: 0 0 0 2px var(--accent-dim); }
  input::placeholder { color: var(--text-ghost); }
  .actions { display: flex; gap: 6px; justify-content: flex-end; }
  .btn {
    padding: 6px 14px;
    border-radius: var(--r-md);
    font-size: 12px;
    font-family: var(--font-ui);
    font-weight: 500;
    cursor: pointer;
    border: 1px solid var(--border-subtle);
    background: transparent;
    color: var(--text-tertiary);
    transition: all var(--t-fast);
  }
  .btn:hover { color: var(--text-primary); border-color: var(--border-strong); background: var(--bg-hover); }
  .btn.primary { color: #fff; background: var(--accent); border-color: transparent; }
  .btn.primary:hover:not(:disabled) { filter: brightness(1.1); }
  .btn.primary:disabled { opacity: 0.3; cursor: not-allowed; }
</style>
