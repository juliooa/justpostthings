<script lang="ts">
  import { postStore } from "../lib/stores/post.svelte";
</script>

{#if postStore.error}
  <div class="status-error">
    <div class="error-indicator"></div>
    <span>{postStore.error}</span>
  </div>
{/if}

{#if postStore.results.length > 0}
  <div class="results">
    <span class="section-label">Results</span>
    {#each postStore.results as result}
      <div class="result-item" class:success={result.success} class:failure={!result.success}>
        <div class="result-dot" class:success={result.success} class:failure={!result.success}></div>
        <div class="result-details">
          <div class="result-header">
            <span class="channel-name">{result.channel}</span>
            <span class="result-badge" class:success={result.success} class:failure={!result.success}>
              {result.success ? "sent" : "failed"}
            </span>
          </div>
          <span class="result-message">{result.message}</span>
          {#if result.post_id}
            <span class="post-id">{result.post_id}</span>
          {/if}
        </div>
      </div>
    {/each}
  </div>
{/if}

<style>
  .status-error {
    padding: 12px 14px;
    border-radius: var(--radius);
    background: var(--danger-bg);
    border: 1px solid var(--danger-border);
    color: var(--danger);
    font-size: 13px;
    display: flex;
    align-items: center;
    gap: 10px;
    line-height: 1.5;
  }

  .error-indicator {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: var(--danger);
    flex-shrink: 0;
    box-shadow: 0 0 8px rgba(212, 93, 93, 0.5);
  }

  .results {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .result-item {
    padding: 12px 14px;
    border-radius: var(--radius);
    display: flex;
    align-items: flex-start;
    gap: 12px;
    border: 1px solid var(--border);
    background: var(--surface);
    box-shadow: var(--shadow-sm);
  }

  .result-item.success {
    border-color: var(--success-border);
  }

  .result-item.failure {
    border-color: var(--danger-border);
  }

  .result-dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    margin-top: 5px;
    flex-shrink: 0;
  }

  .result-dot.success {
    background: var(--success);
    box-shadow: 0 0 6px rgba(93, 184, 122, 0.4);
  }

  .result-dot.failure {
    background: var(--danger);
    box-shadow: 0 0 6px rgba(212, 93, 93, 0.4);
  }

  .result-details {
    display: flex;
    flex-direction: column;
    gap: 3px;
    min-width: 0;
  }

  .result-header {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .channel-name {
    font-weight: 600;
    font-size: 13.5px;
    color: var(--text);
  }

  .result-badge {
    font-size: 9px;
    font-family: "DM Mono", monospace;
    font-weight: 500;
    padding: 1px 6px;
    border-radius: 4px;
    text-transform: uppercase;
    letter-spacing: 0.8px;
  }

  .result-badge.success {
    background: var(--success-bg);
    color: var(--success);
  }

  .result-badge.failure {
    background: var(--danger-bg);
    color: var(--danger);
  }

  .result-message {
    font-size: 12.5px;
    color: var(--text-secondary);
    line-height: 1.4;
  }

  .post-id {
    font-size: 11px;
    color: var(--text-tertiary);
    font-family: "DM Mono", monospace;
    letter-spacing: 0.3px;
  }
</style>
