<script lang="ts">
  import { onMount } from "svelte";
  import { postStore } from "./lib/stores/post.svelte";
  import { getConfig } from "./lib/api";
  import PostEditor from "./components/PostEditor.svelte";
  import ImageUploader from "./components/ImageUploader.svelte";
  import ChannelSelector from "./components/ChannelSelector.svelte";
  import Scheduler from "./components/Scheduler.svelte";
  import PostPreview from "./components/PostPreview.svelte";
  import PostButton from "./components/PostButton.svelte";
  import StatusFeedback from "./components/StatusFeedback.svelte";

  let loading = $state(true);
  let loadError = $state<string | null>(null);

  onMount(async () => {
    try {
      const config = await getConfig();
      postStore.config = config;
      postStore.selectedChannels = [...config.default_post_channels];
    } catch (e) {
      loadError = `Failed to load config: ${e}`;
    } finally {
      loading = false;
    }
  });
</script>

<main>
  {#if loading}
    <div class="loading">
      <div class="loading-dot"></div>
      <span>Loading configuration...</span>
    </div>
  {:else if loadError}
    <div class="load-error">{loadError}</div>
  {:else}
    <div class="layout">
      <div class="left-column">
        <div class="column-header">
          <span class="app-title">justpostthings</span>
          <button class="new-post-btn" onclick={() => postStore.newPost()}>+ New post</button>
        </div>
        <PostEditor />
        <ImageUploader />
        <ChannelSelector />
        <Scheduler />
        <PostButton />
      </div>
      <div class="divider"></div>
      <div class="right-column">
        <PostPreview />
        <StatusFeedback />
      </div>
    </div>
  {/if}
</main>

<style>
  main {
    height: 100vh;
    padding: 20px;
    box-sizing: border-box;
    overflow-y: auto;
  }

  .loading,
  .load-error {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 10px;
    height: 100%;
    font-size: 13px;
    color: var(--text-tertiary);
    font-family: "DM Mono", monospace;
    letter-spacing: 0.5px;
  }

  .loading-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: var(--accent);
    animation: pulse 1.2s ease-in-out infinite;
  }

  @keyframes pulse {
    0%, 100% { opacity: 0.3; transform: scale(0.8); }
    50% { opacity: 1; transform: scale(1.2); }
  }

  .load-error {
    color: var(--danger);
  }

  .layout {
    display: flex;
    gap: 0;
    height: 100%;
  }

  .column-header {
    margin-bottom: 4px;
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .new-post-btn {
    padding: 5px 12px;
    border-radius: 7px;
    border: 1px solid var(--border-strong);
    background: var(--surface-raised);
    color: var(--text-secondary);
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
    font-family: "DM Sans", system-ui, sans-serif;
  }

  .new-post-btn:hover {
    border-color: var(--accent);
    color: var(--text);
    background: var(--accent-subtle);
  }

  .app-title {
    font-size: 15px;
    font-weight: 700;
    color: var(--text);
    letter-spacing: -0.3px;
  }

  .left-column {
    flex: 0 0 54%;
    display: flex;
    flex-direction: column;
    gap: 18px;
    padding-right: 24px;
  }

  .divider {
    width: 1px;
    background: linear-gradient(
      to bottom,
      transparent,
      var(--border-strong) 15%,
      var(--border-strong) 85%,
      transparent
    );
    flex-shrink: 0;
  }

  .right-column {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 16px;
    padding-left: 24px;
    overflow-y: auto;
  }
</style>
