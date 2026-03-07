<script lang="ts">
  import { postStore } from "../lib/stores/post.svelte";
  import { translatePreview } from "../lib/api";

  let activeTab = $state<string>("");

  // Keep activeTab in sync with selected channels
  $effect(() => {
    const channels = postStore.selectedChannels;
    if (channels.length > 0 && !channels.includes(activeTab)) {
      activeTab = channels[0];
    }
    if (channels.length === 0) {
      activeTab = "";
    }
  });

  let translatingChannel = $state<string | null>(null);

  function getChannel(name: string) {
    return postStore.channels.find((c) => c.name === name);
  }

  async function translateForChannel(channelName: string) {
    if (!postStore.text.trim()) return;
    translatingChannel = channelName;
    try {
      const results = await translatePreview(postStore.text, [channelName]);
      // Merge into existing translations
      for (const r of results) {
        const existing = postStore.translations.find((t) => t.channel === r.channel);
        if (existing) {
          postStore.updateTranslation(r.channel, r.translated_text);
        } else {
          postStore.translations = [...postStore.translations, r];
        }
      }
    } catch (e) {
      postStore.error = `Translation failed: ${e}`;
    } finally {
      translatingChannel = null;
    }
  }
</script>

<div class="post-preview">
  <span class="section-label">Preview</span>

  {#if postStore.selectedChannels.length > 0}
    <div class="tab-bar">
      {#each postStore.selectedChannels as channelName}
        {@const channel = getChannel(channelName)}
        <button
          class="tab"
          class:active={activeTab === channelName}
          onclick={() => (activeTab = channelName)}
        >
          <span class="tab-name">{channelName}</span>
          {#if channel?.should_translate}
            <span class="tab-badge">TR</span>
          {/if}
        </button>
      {/each}
    </div>

    {#if postStore.text}
      {@const channel = getChannel(activeTab)}
      {@const translation = postStore.translations.find((t) => t.channel === activeTab)}
      {@const isTranslating = translatingChannel === activeTab}

      <div class="preview-card">
        {#if channel?.should_translate && translation}
          <div class="translation-section">
            <div class="translation-header">
              <div class="lang-info">
                {#if channel.translate}
                  <span class="lang-pair">{channel.translate.from} &rarr; {channel.translate.to}</span>
                {/if}
              </div>
              <button
                class="btn-secondary btn-sm"
                onclick={() => translateForChannel(activeTab)}
                disabled={isTranslating}
              >
                {#if isTranslating}
                  <span class="refresh-spinner"></span>
                {/if}
                {isTranslating ? "Translating..." : "Refresh"}
              </button>
            </div>
            <textarea
              class="translated-text"
              value={translation.translated_text}
              oninput={(e) => postStore.updateTranslation(activeTab, e.currentTarget.value)}
              rows="4"
            ></textarea>
            <div class="char-info">
              <div class="char-bar">
                <div
                  class="char-fill"
                  class:over-limit={translation.translated_text.length > 280}
                  style="width: {Math.min((translation.translated_text.length / 280) * 100, 100)}%"
                ></div>
              </div>
              <span class="char-counter" class:over-limit={translation.translated_text.length > 280}>
                {translation.translated_text.length}/280
              </span>
            </div>
          </div>
        {:else if channel?.should_translate}
          <div class="translation-section">
            <div class="translation-header">
              <div class="lang-info">
                {#if channel.translate}
                  <span class="lang-pair">{channel.translate.from} &rarr; {channel.translate.to}</span>
                {/if}
              </div>
              <button
                class="btn-secondary btn-sm"
                onclick={() => translateForChannel(activeTab)}
                disabled={isTranslating}
              >
                {#if isTranslating}
                  <span class="refresh-spinner"></span>
                {/if}
                {isTranslating ? "Translating..." : "Translate"}
              </button>
            </div>
            <p class="no-translation">Click "Translate" to preview translation</p>
          </div>
        {:else}
          <p class="preview-text">{postStore.text}</p>
        {/if}

        {#if postStore.images.length > 0}
          <div class="preview-images">
            {#each postStore.images as image, i}
              <img src={image.preview} alt="Preview {i + 1}" />
            {/each}
          </div>
        {/if}

        {#if postStore.scheduleEnabled && postStore.scheduleDatetime}
          <div class="schedule-badge">
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/>
            </svg>
            {new Date(postStore.scheduleDatetime).toLocaleString()}
          </div>
        {/if}
      </div>
    {:else}
      <div class="empty-state">
        <div class="empty-icon">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
            <path d="M12 20h9"/><path d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z"/>
          </svg>
        </div>
        <span>Start typing to see a preview</span>
      </div>
    {/if}
  {:else}
    <div class="empty-state">
      <div class="empty-icon">
        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
          <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/><line x1="3" y1="9" x2="21" y2="9"/><line x1="9" y1="21" x2="9" y2="9"/>
        </svg>
      </div>
      <span>Select channels to preview</span>
    </div>
  {/if}
</div>

<style>
  .post-preview {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .tab-bar {
    display: flex;
    gap: 2px;
    background: var(--surface);
    border-radius: var(--radius);
    padding: 3px;
    border: 1px solid var(--border);
  }

  .tab {
    flex: 1;
    padding: 8px 12px;
    border: none;
    background: transparent;
    color: var(--text-tertiary);
    font-size: 12px;
    font-weight: 600;
    font-family: "DM Mono", monospace;
    text-transform: uppercase;
    letter-spacing: 0.8px;
    cursor: pointer;
    border-radius: 7px;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
  }

  .tab:hover:not(.active) {
    color: var(--text-secondary);
    background: var(--surface-raised);
  }

  .tab.active {
    background: var(--surface-raised);
    color: var(--text);
    box-shadow: var(--shadow-sm);
  }

  .tab-badge {
    font-size: 8px;
    padding: 1px 4px;
    border-radius: 3px;
    background: var(--accent-subtle);
    color: var(--accent);
    letter-spacing: 0.5px;
    font-weight: 700;
  }

  .tab-name {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .preview-card {
    padding: 16px;
    border-radius: var(--radius-lg);
    background: var(--surface);
    border: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    gap: 12px;
    box-shadow: var(--shadow-sm);
  }

  .preview-text {
    margin: 0;
    font-size: 14.5px;
    line-height: 1.65;
    color: var(--text);
    white-space: pre-wrap;
    word-break: break-word;
  }

  .preview-images {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
  }

  .preview-images img {
    width: 120px;
    height: 80px;
    object-fit: cover;
    border-radius: 8px;
    border: 1px solid var(--border);
  }

  .schedule-badge {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 12px;
    font-family: "DM Mono", monospace;
    color: var(--accent);
    padding: 6px 10px;
    background: var(--accent-subtle);
    border-radius: 6px;
    width: fit-content;
    letter-spacing: 0.3px;
  }

  .empty-state {
    padding: 40px 20px;
    text-align: center;
    color: var(--text-tertiary);
    font-size: 13px;
    border: 1px dashed var(--border);
    border-radius: var(--radius-lg);
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 10px;
  }

  .empty-icon {
    opacity: 0.3;
  }

  /* Translation section */
  .translation-section {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .translation-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .lang-info {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .lang-pair {
    font-size: 11px;
    color: var(--text-tertiary);
    font-family: "DM Mono", monospace;
    letter-spacing: 0.3px;
  }

  .translated-text {
    width: 100%;
    margin: 0;
    padding: 10px 12px;
    font-size: 14px;
    line-height: 1.55;
    color: var(--text);
    white-space: pre-wrap;
    background: var(--bg);
    border: 1px solid var(--border);
    border-radius: 8px;
    font-family: "DM Sans", system-ui, sans-serif;
    resize: vertical;
    box-sizing: border-box;
    transition: border-color 0.2s ease;
  }

  .translated-text:focus {
    outline: none;
    border-color: var(--accent);
    box-shadow: 0 0 0 2px var(--accent-glow);
  }

  .char-info {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .char-bar {
    flex: 1;
    height: 2px;
    background: var(--border);
    border-radius: 1px;
    overflow: hidden;
  }

  .char-fill {
    height: 100%;
    background: var(--text-tertiary);
    border-radius: 1px;
    transition: width 0.2s ease, background 0.2s ease;
  }

  .char-fill.over-limit {
    background: var(--danger);
  }

  .char-counter {
    font-size: 11px;
    font-family: "DM Mono", monospace;
    color: var(--text-tertiary);
    letter-spacing: 0.5px;
    flex-shrink: 0;
  }

  .char-counter.over-limit {
    color: var(--danger);
    font-weight: 500;
  }

  .no-translation {
    margin: 0;
    font-size: 12.5px;
    color: var(--text-tertiary);
    font-style: italic;
    padding: 8px 0;
  }

  .refresh-spinner {
    display: inline-block;
    width: 10px;
    height: 10px;
    border: 1.5px solid var(--border);
    border-top-color: var(--text-secondary);
    border-radius: 50%;
    animation: spin 0.6s linear infinite;
    margin-right: 4px;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
