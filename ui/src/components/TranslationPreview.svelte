<script lang="ts">
  import { postStore } from "../lib/stores/post.svelte";
  import { translatePreview } from "../lib/api";

  async function refreshTranslations() {
    if (!postStore.text.trim()) return;

    const channelNames = postStore.translatingChannels.map((c) => c.name);
    if (channelNames.length === 0) return;

    postStore.isTranslating = true;
    try {
      postStore.translations = await translatePreview(
        postStore.text,
        channelNames
      );
    } catch (e) {
      postStore.error = `Translation failed: ${e}`;
    } finally {
      postStore.isTranslating = false;
    }
  }
</script>

<div class="translation-preview">
  <div class="header">
    <span class="section-label">Translations</span>
    <button
      class="btn-secondary btn-sm"
      onclick={refreshTranslations}
      disabled={postStore.isTranslating}
    >
      {#if postStore.isTranslating}
        <span class="refresh-spinner"></span>
      {/if}
      {postStore.isTranslating ? "Translating..." : "Refresh"}
    </button>
  </div>

  {#each postStore.translatingChannels as channel}
    {@const translation = postStore.translations.find(
      (t) => t.channel === channel.name
    )}
    <div class="translation-card">
      <div class="channel-label">
        <span class="channel-name">{channel.name}</span>
        {#if channel.translate}
          <span class="lang-pair">{channel.translate.from} &rarr; {channel.translate.to}</span>
        {/if}
      </div>
      {#if translation}
        <textarea
          class="translated-text"
          value={translation.translated_text}
          oninput={(e) => postStore.updateTranslation(channel.name, e.currentTarget.value)}
          rows="3"
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
      {:else}
        <p class="no-translation">
          Click "Refresh" to preview translation
        </p>
      {/if}
    </div>
  {/each}
</div>

<style>
  .translation-preview {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .header {
    display: flex;
    align-items: center;
    justify-content: space-between;
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

  .translation-card {
    padding: 14px;
    border-radius: var(--radius);
    background: var(--surface);
    border: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    gap: 8px;
    box-shadow: var(--shadow-sm);
  }

  .channel-label {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .channel-name {
    font-size: 12px;
    font-weight: 600;
    font-family: "DM Mono", monospace;
    color: var(--accent);
    text-transform: uppercase;
    letter-spacing: 0.8px;
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
</style>
