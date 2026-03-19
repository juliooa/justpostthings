<script lang="ts">
  import { listSentPosts, deleteSentPost, openPostsFolder } from "../lib/api";
  import { postStore } from "../lib/stores/post.svelte";
  import type { SentPost } from "../lib/types";

  let { onclose }: { onclose: () => void } = $props();

  let posts = $state<SentPost[]>([]);
  let loading = $state(true);
  let expandedId = $state<string | null>(null);

  $effect(() => {
    loadPosts();
  });

  async function loadPosts() {
    loading = true;
    try {
      posts = await listSentPosts();
    } catch (e) {
      console.error("Failed to load sent posts:", e);
    } finally {
      loading = false;
    }
  }

  async function handleDelete(id: string) {
    try {
      await deleteSentPost(id);
      posts = posts.filter((p) => p.id !== id);
      if (expandedId === id) expandedId = null;
    } catch (e) {
      console.error("Failed to delete post:", e);
    }
  }

  function handleReuse(post: SentPost) {
    const lines = post.content.split("\n");
    const firstChannelText: string[] = [];
    let pastFirstLine = false;
    for (const line of lines) {
      if (line.startsWith("channel: ") && !pastFirstLine) {
        pastFirstLine = true;
        continue;
      }
      if (line === "-------") break;
      if (pastFirstLine) firstChannelText.push(line);
    }
    postStore.text = firstChannelText.join("\n");
    onclose();
  }

  function formatDate(id: string): string {
    const ms = parseInt(id, 10);
    if (isNaN(ms)) return id;
    const d = new Date(ms);
    return d.toLocaleDateString(undefined, {
      month: "short",
      day: "numeric",
      year: "numeric",
    }) + " " + d.toLocaleTimeString(undefined, {
      hour: "2-digit",
      minute: "2-digit",
    });
  }

  function parseChannels(content: string): { channel: string; text: string }[] {
    const blocks = content.split("\n-------\n");
    return blocks.map((block) => {
      const firstNewline = block.indexOf("\n");
      if (firstNewline === -1) {
        return { channel: block.replace("channel: ", ""), text: "" };
      }
      const channel = block.slice(0, firstNewline).replace("channel: ", "");
      const text = block.slice(firstNewline + 1);
      return { channel, text };
    });
  }

  function previewText(content: string): string {
    const lines = content.split("\n");
    const textLines: string[] = [];
    for (const line of lines) {
      if (line.startsWith("channel: ")) continue;
      if (line === "-------") break;
      textLines.push(line);
    }
    const joined = textLines.join(" ").trim();
    return joined.length > 120 ? joined.slice(0, 120) + "..." : joined;
  }
</script>

<div class="sent-posts">
  <div class="sent-posts-header">
    <span class="sent-posts-title">Sent Posts</span>
    <div class="header-actions">
      <button class="btn-folder" title="Open folder" onclick={() => openPostsFolder()}>
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path>
        </svg>
      </button>
      <button class="btn-close" title="Close" onclick={onclose}>
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <line x1="18" y1="6" x2="6" y2="18"></line>
          <line x1="6" y1="6" x2="18" y2="18"></line>
        </svg>
      </button>
    </div>
  </div>

  <div class="posts-list">
    {#if loading}
      <div class="empty-state">Loading...</div>
    {:else if posts.length === 0}
      <div class="empty-state">No sent posts yet</div>
    {:else}
      {#each posts as post (post.id)}
        <div class="post-card" class:expanded={expandedId === post.id}>
          <button class="post-summary" onclick={() => (expandedId = expandedId === post.id ? null : post.id)}>
            <span class="post-date">{formatDate(post.id)}</span>
            <span class="post-preview">{previewText(post.content)}</span>
          </button>

          {#if expandedId === post.id}
            <div class="post-detail">
              {#each parseChannels(post.content) as block}
                <div class="channel-block">
                  <span class="channel-badge">{block.channel}</span>
                  <p class="channel-text">{block.text}</p>
                </div>
              {/each}
              <div class="post-actions">
                <button class="btn-reuse" onclick={() => handleReuse(post)}>
                  <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                    <line x1="5" y1="12" x2="19" y2="12"></line>
                    <polyline points="12 5 19 12 12 19"></polyline>
                  </svg>
                  Reuse text
                </button>
                <button class="btn-delete" onclick={() => handleDelete(post.id)}>
                  <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                    <polyline points="3 6 5 6 21 6"></polyline>
                    <path d="M19 6l-1 14a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2L5 6"></path>
                    <path d="M10 11v6"></path>
                    <path d="M14 11v6"></path>
                  </svg>
                  Delete
                </button>
              </div>
            </div>
          {/if}
        </div>
      {/each}
    {/if}
  </div>
</div>

<style>
  .sent-posts {
    display: flex;
    flex-direction: column;
    height: 100%;
    max-width: 640px;
    margin: 0 auto;
  }

  .sent-posts-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 20px;
    flex-shrink: 0;
  }

  .sent-posts-title {
    font-size: 15px;
    font-weight: 700;
    color: var(--text);
    letter-spacing: -0.3px;
  }

  .header-actions {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .btn-folder {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 30px;
    height: 30px;
    border: 1px solid var(--border);
    border-radius: 8px;
    background: transparent;
    color: var(--text-tertiary);
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .btn-folder:hover {
    border-color: var(--border-strong);
    color: var(--text-secondary);
    background: var(--surface-raised);
  }

  .btn-close {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 30px;
    height: 30px;
    border: 1px solid var(--border);
    border-radius: 8px;
    background: transparent;
    color: var(--text-tertiary);
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .btn-close:hover {
    border-color: var(--border-strong);
    color: var(--text-secondary);
    background: var(--surface-raised);
  }

  .posts-list {
    flex: 1;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding-right: 4px;
  }

  .posts-list::-webkit-scrollbar {
    width: 4px;
  }

  .posts-list::-webkit-scrollbar-track {
    background: transparent;
  }

  .posts-list::-webkit-scrollbar-thumb {
    background: var(--border-strong);
    border-radius: 2px;
  }

  .post-card {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    transition: border-color 0.15s ease;
    overflow: hidden;
  }

  .post-card:hover {
    border-color: var(--border-strong);
  }

  .post-card.expanded {
    border-color: var(--accent);
    box-shadow: 0 0 0 1px var(--accent-glow);
  }

  .post-summary {
    all: unset;
    display: flex;
    flex-direction: column;
    gap: 4px;
    padding: 12px;
    cursor: pointer;
    width: 100%;
    box-sizing: border-box;
  }

  .post-summary:hover {
    background: var(--surface-raised);
  }

  .post-date {
    font-size: 11px;
    font-weight: 600;
    color: var(--text-tertiary);
    font-family: "DM Mono", monospace;
    letter-spacing: 0.3px;
  }

  .post-preview {
    font-size: 12.5px;
    line-height: 1.5;
    color: var(--text-secondary);
    word-break: break-word;
  }

  .post-detail {
    padding: 0 12px 12px;
    display: flex;
    flex-direction: column;
    gap: 10px;
    border-top: 1px solid var(--border);
    padding-top: 10px;
  }

  .channel-block {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .channel-badge {
    font-size: 10.5px;
    font-weight: 600;
    color: var(--accent);
    text-transform: uppercase;
    letter-spacing: 0.5px;
    font-family: "DM Mono", monospace;
  }

  .channel-text {
    font-size: 12.5px;
    line-height: 1.6;
    color: var(--text);
    margin: 0;
    white-space: pre-wrap;
    word-break: break-word;
  }

  .post-actions {
    display: flex;
    gap: 8px;
    padding-top: 6px;
  }

  .btn-reuse,
  .btn-delete {
    display: flex;
    align-items: center;
    gap: 5px;
    padding: 5px 10px;
    border-radius: 6px;
    border: 1px solid var(--border-strong);
    font-size: 11px;
    font-weight: 500;
    cursor: pointer;
    font-family: "DM Sans", system-ui, sans-serif;
    transition: all 0.15s ease;
    background: transparent;
    color: var(--text-tertiary);
  }

  .btn-reuse:hover {
    border-color: var(--accent);
    color: var(--accent);
    background: var(--accent-subtle);
  }

  .btn-delete:hover {
    border-color: var(--danger-border);
    color: var(--danger);
    background: var(--danger-bg);
  }

  .empty-state {
    font-size: 12px;
    color: var(--text-tertiary);
    text-align: center;
    padding: 48px 0;
    font-family: "DM Mono", monospace;
    letter-spacing: 0.3px;
  }
</style>
