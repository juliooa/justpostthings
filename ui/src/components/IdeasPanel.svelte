<script lang="ts">
  import { onMount } from "svelte";
  import { postStore } from "../lib/stores/post.svelte";
  import { listIdeas, createIdea, updateIdea, deleteIdea } from "../lib/api";
  import type { Idea } from "../lib/types";

  let ideas = $state<Idea[]>([]);
  let collapsed = $state(false);
  let editingId = $state<string | null>(null);
  let editText = $state("");
  let adding = $state(false);
  let newText = $state("");

  onMount(async () => {
    try {
      ideas = await listIdeas();
    } catch (e) {
      console.error("Failed to load ideas:", e);
    }
  });

  async function handleAdd() {
    const content = newText.trim();
    if (!content) return;
    try {
      const idea = await createIdea(content);
      ideas = [idea, ...ideas];
      newText = "";
      adding = false;
    } catch (e) {
      console.error("Failed to create idea:", e);
    }
  }

  async function handleSave(id: string) {
    const content = editText.trim();
    if (!content) return;
    try {
      await updateIdea(id, content);
      ideas = ideas.map((i) => (i.id === id ? { ...i, content } : i));
    } catch (e) {
      console.error("Failed to update idea:", e);
    }
    editingId = null;
  }

  async function handleDelete(id: string) {
    try {
      await deleteIdea(id);
      ideas = ideas.filter((i) => i.id !== id);
    } catch (e) {
      console.error("Failed to delete idea:", e);
    }
  }

  function handleUse(idea: Idea) {
    postStore.text = idea.content;
  }

  function startEdit(idea: Idea) {
    editingId = idea.id;
    editText = idea.content;
  }

  function handleEditKeydown(e: KeyboardEvent, id: string) {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      handleSave(id);
    } else if (e.key === "Escape") {
      editingId = null;
    }
  }

  function handleNewKeydown(e: KeyboardEvent) {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      handleAdd();
    } else if (e.key === "Escape") {
      adding = false;
      newText = "";
    }
  }
</script>

{#if collapsed}
  <button class="collapsed-strip" onclick={() => (collapsed = false)}>
    <span class="collapsed-label">IDEAS</span>
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <polyline points="9 18 15 12 9 6"></polyline>
    </svg>
  </button>
{:else}
  <div class="ideas-panel">
    <div class="panel-header">
      <span class="section-label">Ideas</span>
      <div class="header-actions">
        <button class="icon-btn" title="Add idea" onclick={() => { adding = true; newText = ""; }}>
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="12" y1="5" x2="12" y2="19"></line>
            <line x1="5" y1="12" x2="19" y2="12"></line>
          </svg>
        </button>
        <button class="icon-btn" title="Collapse" onclick={() => (collapsed = true)}>
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="15 18 9 12 15 6"></polyline>
          </svg>
        </button>
      </div>
    </div>

    <div class="ideas-list">
      {#if adding}
        <div class="idea-card editing">
          <textarea
            class="idea-textarea"
            bind:value={newText}
            onkeydown={handleNewKeydown}
            placeholder="Write your idea..."
            rows="3"
          ></textarea>
          <div class="edit-actions">
            <button class="btn-save" onclick={handleAdd}>Save</button>
            <button class="btn-cancel" onclick={() => { adding = false; newText = ""; }}>Cancel</button>
          </div>
        </div>
      {/if}

      {#each ideas as idea (idea.id)}
        <div class="idea-card">
          {#if editingId === idea.id}
            <textarea
              class="idea-textarea"
              bind:value={editText}
              onkeydown={(e) => handleEditKeydown(e, idea.id)}
              rows="3"
            ></textarea>
            <div class="edit-actions">
              <button class="btn-save" onclick={() => handleSave(idea.id)}>Save</button>
              <button class="btn-cancel" onclick={() => (editingId = null)}>Cancel</button>
            </div>
          {:else}
            <button class="idea-content" onclick={() => startEdit(idea)}>
              {idea.content.length > 100 ? idea.content.slice(0, 100) + "..." : idea.content}
            </button>
            <div class="card-actions">
              <button class="icon-btn use-btn" title="Use this idea" onclick={() => handleUse(idea)}>
                <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                  <line x1="5" y1="12" x2="19" y2="12"></line>
                  <polyline points="12 5 19 12 12 19"></polyline>
                </svg>
              </button>
              <button class="icon-btn delete-btn" title="Delete idea" onclick={() => handleDelete(idea.id)}>
                <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                  <line x1="18" y1="6" x2="6" y2="18"></line>
                  <line x1="6" y1="6" x2="18" y2="18"></line>
                </svg>
              </button>
            </div>
          {/if}
        </div>
      {/each}

      {#if ideas.length === 0 && !adding}
        <div class="empty-state">No ideas yet</div>
      {/if}
    </div>
  </div>
{/if}

<style>
  .ideas-panel {
    width: 240px;
    flex-shrink: 0;
    display: flex;
    flex-direction: column;
    gap: 12px;
    height: 100%;
    overflow: hidden;
  }

  .panel-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .header-actions {
    display: flex;
    gap: 4px;
  }

  .ideas-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
    overflow-y: auto;
    flex: 1;
    padding-right: 4px;
  }

  .ideas-list::-webkit-scrollbar {
    width: 4px;
  }

  .ideas-list::-webkit-scrollbar-track {
    background: transparent;
  }

  .ideas-list::-webkit-scrollbar-thumb {
    background: var(--border-strong);
    border-radius: 2px;
  }

  .idea-card {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    padding: 10px;
    display: flex;
    flex-direction: column;
    gap: 8px;
    transition: border-color 0.15s ease;
  }

  .idea-card:hover {
    border-color: var(--border-strong);
  }

  .idea-card.editing {
    border-color: var(--accent);
    box-shadow: 0 0 0 2px var(--accent-glow);
  }

  .idea-content {
    all: unset;
    font-size: 12.5px;
    line-height: 1.5;
    color: var(--text-secondary);
    cursor: text;
    word-break: break-word;
    text-align: left;
  }

  .idea-content:hover {
    color: var(--text);
  }

  .card-actions {
    display: flex;
    gap: 4px;
    opacity: 0;
    transition: opacity 0.15s ease;
  }

  .idea-card:hover .card-actions {
    opacity: 1;
  }

  .icon-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 26px;
    height: 26px;
    border: 1px solid var(--border);
    border-radius: 6px;
    background: transparent;
    color: var(--text-tertiary);
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .icon-btn:hover {
    border-color: var(--border-strong);
    color: var(--text-secondary);
    background: var(--surface-raised);
  }

  .use-btn:hover {
    border-color: var(--accent);
    color: var(--accent);
    background: var(--accent-subtle);
  }

  .delete-btn:hover {
    border-color: var(--danger-border);
    color: var(--danger);
    background: var(--danger-bg);
  }

  .idea-textarea {
    width: 100%;
    padding: 8px;
    border: 1px solid var(--border-strong);
    border-radius: 6px;
    background: var(--surface-raised);
    color: var(--text);
    font-size: 12.5px;
    font-family: "DM Sans", system-ui, sans-serif;
    line-height: 1.5;
    resize: vertical;
    min-height: 60px;
  }

  .idea-textarea:focus {
    outline: none;
    border-color: var(--accent);
  }

  .idea-textarea::placeholder {
    color: var(--text-tertiary);
  }

  .edit-actions {
    display: flex;
    gap: 6px;
  }

  .btn-save,
  .btn-cancel {
    padding: 4px 10px;
    border-radius: 6px;
    border: 1px solid var(--border-strong);
    font-size: 11px;
    font-weight: 500;
    cursor: pointer;
    font-family: "DM Sans", system-ui, sans-serif;
    transition: all 0.15s ease;
  }

  .btn-save {
    background: var(--accent);
    border-color: var(--accent);
    color: white;
  }

  .btn-save:hover {
    background: var(--accent-hover);
  }

  .btn-cancel {
    background: transparent;
    color: var(--text-tertiary);
  }

  .btn-cancel:hover {
    color: var(--text-secondary);
    background: var(--surface-raised);
  }

  .empty-state {
    font-size: 12px;
    color: var(--text-tertiary);
    text-align: center;
    padding: 24px 0;
    font-family: "DM Mono", monospace;
    letter-spacing: 0.3px;
  }

  /* Collapsed strip */
  .collapsed-strip {
    all: unset;
    width: 36px;
    flex-shrink: 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 10px;
    cursor: pointer;
    color: var(--text-tertiary);
    transition: color 0.15s ease;
    height: 100%;
  }

  .collapsed-strip:hover {
    color: var(--text-secondary);
  }

  .collapsed-label {
    writing-mode: vertical-rl;
    text-orientation: mixed;
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 1.2px;
    font-family: "DM Mono", monospace;
  }
</style>
