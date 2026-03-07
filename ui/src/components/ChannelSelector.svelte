<script lang="ts">
  import { postStore } from "../lib/stores/post.svelte";
</script>

<div class="channel-selector">
  <span class="section-label">Channels</span>
  <div class="channel-list">
    {#each postStore.channels as channel}
      {@const isSelected = postStore.selectedChannels.includes(channel.name)}
      <button
        class="channel-toggle"
        class:active={isSelected}
        onclick={() => postStore.toggleChannel(channel.name)}
      >
        <span class="channel-dot" class:active={isSelected}></span>
        <span class="channel-name">{channel.name}</span>
        {#if channel.should_translate}
          <span class="badge">TR</span>
        {/if}
      </button>
    {/each}
  </div>
</div>

<style>
  .channel-selector {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .channel-list {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
  }

  .channel-toggle {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 16px;
    border-radius: 24px;
    border: 1px solid var(--border);
    background: var(--surface);
    color: var(--text-secondary);
    font-size: 13.5px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
    font-family: "DM Sans", system-ui, sans-serif;
    letter-spacing: 0.2px;
  }

  .channel-toggle:hover {
    border-color: var(--border-strong);
    color: var(--text);
    background: var(--surface-raised);
  }

  .channel-toggle.active {
    background: var(--accent-subtle);
    border-color: var(--accent);
    color: var(--accent);
  }

  .channel-dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: var(--text-tertiary);
    transition: all 0.2s ease;
  }

  .channel-dot.active {
    background: var(--accent);
    box-shadow: 0 0 6px var(--accent-glow);
  }

  .badge {
    font-size: 9px;
    font-weight: 600;
    font-family: "DM Mono", monospace;
    padding: 2px 5px;
    border-radius: 4px;
    background: rgba(255, 255, 255, 0.06);
    color: var(--text-tertiary);
    letter-spacing: 0.8px;
  }

  .channel-toggle.active .badge {
    background: var(--accent-glow);
    color: var(--accent);
  }
</style>
