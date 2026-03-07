<script lang="ts">
  import { postStore } from "../lib/stores/post.svelte";
  import DateTimePicker from "./DateTimePicker.svelte";
</script>

<div class="scheduler">
  <label class="toggle-label">
    <div class="toggle-track" class:active={postStore.scheduleEnabled}>
      <input type="checkbox" bind:checked={postStore.scheduleEnabled} />
      <div class="toggle-thumb"></div>
    </div>
    <span>Schedule for later</span>
  </label>

  {#if postStore.scheduleEnabled}
    <DateTimePicker
      value={postStore.scheduleDatetime}
      onchange={(v) => (postStore.scheduleDatetime = v)}
    />
  {/if}
</div>

<style>
  .scheduler {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .toggle-label {
    display: flex;
    align-items: center;
    gap: 10px;
    font-size: 13.5px;
    color: var(--text-secondary);
    cursor: pointer;
    user-select: none;
    font-weight: 500;
  }

  .toggle-track {
    position: relative;
    width: 36px;
    height: 20px;
    border-radius: 10px;
    background: var(--surface-raised);
    border: 1px solid var(--border-strong);
    transition: all 0.2s ease;
    flex-shrink: 0;
  }

  .toggle-track.active {
    background: var(--accent);
    border-color: var(--accent);
  }

  .toggle-track input {
    position: absolute;
    opacity: 0;
    width: 0;
    height: 0;
  }

  .toggle-thumb {
    position: absolute;
    top: 2px;
    left: 2px;
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background: var(--text-tertiary);
    transition: all 0.2s ease;
  }

  .toggle-track.active .toggle-thumb {
    left: 18px;
    background: white;
  }
</style>
