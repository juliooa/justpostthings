<script lang="ts">
  import { onMount } from "svelte";
  import { postStore } from "./lib/stores/post.svelte";
  import { settingsStore } from "./lib/stores/settings.svelte";
  import PostEditor from "./components/PostEditor.svelte";
  import ImageUploader from "./components/ImageUploader.svelte";
  import ChannelSelector from "./components/ChannelSelector.svelte";
  import Scheduler from "./components/Scheduler.svelte";
  import PostPreview from "./components/PostPreview.svelte";
  import PostButton from "./components/PostButton.svelte";
  import StatusFeedback from "./components/StatusFeedback.svelte";
  import IdeasPanel from "./components/IdeasPanel.svelte";
  import ImageLightbox from "./components/ImageLightbox.svelte";
  import Settings from "./components/Settings.svelte";
  import SentPostsPanel from "./components/SentPostsPanel.svelte";

  let loading = $state(true);
  let showSettings = $state(false);
  let showSentPosts = $state(false);
  let missingConfig = $state(false);

  onMount(async () => {
    const hasSettings = await settingsStore.load();
    if (hasSettings) {
      postStore.config = settingsStore.config;
      postStore.selectedChannels = [...settingsStore.config.default_post_channels];
    } else {
      missingConfig = true;
      showSettings = true;
    }
    loading = false;
  });

  function handleSettingsClose() {
    showSettings = false;
    missingConfig = !settingsStore.hasSettings;
    postStore.config = settingsStore.config;
    postStore.selectedChannels = [...settingsStore.config.default_post_channels];
  }
</script>

<main>
  {#if loading}
    <div class="loading">
      <div class="loading-dot"></div>
      <span>Loading...</span>
    </div>
  {:else if showSettings}
    <Settings onclose={handleSettingsClose} />
  {:else if showSentPosts}
    <SentPostsPanel onclose={() => (showSentPosts = false)} />
  {:else}
    {#if missingConfig}
      <div class="config-warning">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"></path>
          <line x1="12" y1="9" x2="12" y2="13"></line>
          <line x1="12" y1="17" x2="12.01" y2="17"></line>
        </svg>
        <span>No config.json found. <button class="link-btn" onclick={() => (showSettings = true)}>Open Settings</button> to configure API keys and channels.</span>
      </div>
    {/if}
    <div class="layout">
      <IdeasPanel />
      <div class="ideas-divider"></div>
      <div class="left-column">
        <div class="column-header">
          <span class="app-title">JustPostThings</span>
          <div class="header-actions">
            <button class="icon-btn-header" title="Sent posts" onclick={() => (showSentPosts = true)}>
              <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="12" cy="12" r="10"></circle>
                <polyline points="12 6 12 12 16 14"></polyline>
              </svg>
            </button>
            <button class="icon-btn-header" title="Settings" onclick={() => (showSettings = true)}>
              <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="12" cy="12" r="3"></circle>
                <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path>
              </svg>
            </button>
            <button class="new-post-btn" onclick={() => postStore.newPost()}>+ New post</button>
          </div>
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

<ImageLightbox />

<style>
  main {
    height: 100vh;
    padding: 20px;
    box-sizing: border-box;
    overflow-y: auto;
  }

  .loading {
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

  .config-warning {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 10px 14px;
    margin-bottom: 16px;
    border-radius: 8px;
    background: rgba(251, 191, 36, 0.08);
    border: 1px solid rgba(251, 191, 36, 0.2);
    color: #fbbf24;
    font-size: 12.5px;
    flex-shrink: 0;
  }

  .config-warning svg {
    flex-shrink: 0;
  }

  .link-btn {
    all: unset;
    color: var(--accent);
    cursor: pointer;
    font-weight: 600;
    text-decoration: underline;
    text-underline-offset: 2px;
  }

  .link-btn:hover {
    color: var(--accent-hover);
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

  .header-actions {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .icon-btn-header {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 30px;
    height: 30px;
    border: 1px solid var(--border);
    border-radius: 7px;
    background: transparent;
    color: var(--text-tertiary);
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .icon-btn-header:hover {
    border-color: var(--border-strong);
    color: var(--text-secondary);
    background: var(--surface-raised);
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
    flex: 1 1 0;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 18px;
    padding-right: 24px;
  }

  .ideas-divider {
    width: 1px;
    background: linear-gradient(
      to bottom,
      transparent,
      var(--border-strong) 15%,
      var(--border-strong) 85%,
      transparent
    );
    flex-shrink: 0;
    margin-right: 20px;
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
