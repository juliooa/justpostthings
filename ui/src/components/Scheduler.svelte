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
    <label class="toggle-label sub-toggle">
      <div class="toggle-track small" class:active={postStore.perChannelSchedule}>
        <input type="checkbox" bind:checked={postStore.perChannelSchedule} />
        <div class="toggle-thumb"></div>
      </div>
      <span>Schedule per channel</span>
    </label>

    {#if postStore.perChannelSchedule}
      <div class="channel-schedules">
        {#each postStore.selectedChannels as channelName}
          <div class="channel-schedule-row">
            <span class="channel-name">{channelName}</span>
            <DateTimePicker
              value={postStore.channelSchedules[channelName] ?? ""}
              onchange={(v) => (postStore.channelSchedules = { ...postStore.channelSchedules, [channelName]: v })}
            />
          </div>
        {/each}
        {#if postStore.selectedChannels.length === 0}
          <p class="no-channels-hint">Select channels first</p>
        {/if}
      </div>
    {:else}
      <DateTimePicker
        value={postStore.scheduleDatetime}
        onchange={(v) => (postStore.scheduleDatetime = v)}
      />
    {/if}
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

  .sub-toggle {
    font-size: 12.5px;
    padding-left: 4px;
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

  .toggle-track.small {
    width: 30px;
    height: 16px;
    border-radius: 8px;
  }

  .toggle-track.small .toggle-thumb {
    width: 10px;
    height: 10px;
    top: 2px;
    left: 2px;
  }

  .toggle-track.small.active .toggle-thumb {
    left: 15px;
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

  .channel-schedules {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .channel-schedule-row {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .channel-name {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-secondary);
    text-transform: capitalize;
    letter-spacing: 0.3px;
  }

  .no-channels-hint {
    font-size: 12px;
    color: var(--text-tertiary);
    margin: 0;
    font-style: italic;
  }
</style>
