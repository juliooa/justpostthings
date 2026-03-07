<script lang="ts">
  import { postStore } from "../lib/stores/post.svelte";
  import { submitPost } from "../lib/api";

  async function handlePost() {
    if (!postStore.canPost) return;

    postStore.isPosting = true;
    postStore.error = null;
    postStore.results = [];

    try {
      const overrides: Record<string, string> = {};
      for (const t of postStore.translations) {
        overrides[t.channel] = t.translated_text;
      }
      postStore.results = await submitPost(
        postStore.text,
        postStore.imagePaths,
        postStore.schedule,
        postStore.selectedChannels,
        overrides
      );
    } catch (e) {
      postStore.error = `Post failed: ${e}`;
    } finally {
      postStore.isPosting = false;
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if ((e.metaKey || e.ctrlKey) && e.key === "Enter") {
      handlePost();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<button
  class="post-button"
  onclick={handlePost}
  disabled={!postStore.canPost}
>
  {#if postStore.isPosting}
    <span class="spinner"></span>
    Posting...
  {:else}
    <span class="post-label">
      Post {postStore.selectedChannels.length > 0
        ? `to ${postStore.selectedChannels.join(", ")}`
        : ""}
    </span>
    <span class="kbd-hint">Cmd+Enter</span>
  {/if}
</button>

<style>
  .post-button {
    width: 100%;
    padding: 13px 24px;
    border-radius: var(--radius);
    border: none;
    background: var(--accent);
    color: white;
    font-size: 14px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    font-family: "DM Sans", system-ui, sans-serif;
    letter-spacing: 0.2px;
    position: relative;
    box-shadow: 0 2px 8px var(--accent-glow);
  }

  .post-button:hover:not(:disabled) {
    background: var(--accent-hover);
    box-shadow: 0 4px 20px var(--accent-glow);
    transform: translateY(-1px);
  }

  .post-button:active:not(:disabled) {
    transform: translateY(0);
    box-shadow: 0 1px 4px var(--accent-glow);
  }

  .post-button:disabled {
    opacity: 0.35;
    cursor: not-allowed;
    box-shadow: none;
  }

  .post-label {
    flex: 1;
    text-align: center;
  }

  .kbd-hint {
    font-size: 10px;
    font-family: "DM Mono", monospace;
    opacity: 0.5;
    letter-spacing: 0.5px;
    padding: 2px 6px;
    background: rgba(0, 0, 0, 0.15);
    border-radius: 4px;
  }

  .spinner {
    width: 16px;
    height: 16px;
    border: 2px solid rgba(255, 255, 255, 0.25);
    border-top-color: white;
    border-radius: 50%;
    animation: spin 0.6s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
</style>
