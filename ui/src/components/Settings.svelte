<script lang="ts">
  import { settingsStore } from "../lib/stores/settings.svelte";
  import { openPostsFolder } from "../lib/api";
  import type { Channel, LlmService, Config, Prompts } from "../lib/types";

  let { onclose }: { onclose: () => void } = $props();

  let saving = $state(false);
  let error = $state<string | null>(null);
  let rawMode = $state(false);
  let rawJson = $state("");
  let rawError = $state<string | null>(null);

  const DEFAULT_TRANSLATION_PROMPT = `Translate the following text from {from} to {to}. Reply with ONLY the translated text, nothing else. No explanations, no quotes, no extra words. Keep the same tone and meaning. Preserve all line breaks and paragraph spacing exactly as in the original. If something from {from} sounds weird in {to}, find a natural way to say it.

{text}`;

  const DEFAULT_SHRINK_PROMPT = `Shorten the following text to fit within {max_chars} characters. Reply with ONLY the shortened text, nothing else. No explanations, no quotes, no extra words. Keep the most important information and the same tone. Preserve line breaks where possible. The result MUST be {max_chars} characters or fewer.

{text}`;

  function getTranslationPrompt(): string {
    return settingsStore.prompts?.translation_prompt ?? DEFAULT_TRANSLATION_PROMPT;
  }

  function getShrinkPrompt(): string {
    return settingsStore.prompts?.shrink_prompt ?? DEFAULT_SHRINK_PROMPT;
  }

  function updatePrompt(field: keyof Prompts, value: string) {
    const defaults: Record<keyof Prompts, string> = {
      translation_prompt: DEFAULT_TRANSLATION_PROMPT,
      shrink_prompt: DEFAULT_SHRINK_PROMPT,
    };
    const current = settingsStore.prompts ?? {};
    const newVal = value === defaults[field] ? undefined : value;
    const updated = { ...current, [field]: newVal };
    // If both are undefined/default, set prompts to undefined
    if (!updated.translation_prompt && !updated.shrink_prompt) {
      settingsStore.prompts = undefined;
    } else {
      settingsStore.prompts = updated;
    }
  }

  function enterRawMode() {
    const config: Config = {
      channels: settingsStore.channels,
      default_post_channels: settingsStore.default_post_channels,
      llm_service: settingsStore.llm_service,
      save_sent_posts: settingsStore.save_sent_posts,
      prompts: settingsStore.prompts,
    };
    rawJson = JSON.stringify(config, null, 2);
    rawError = null;
    rawMode = true;
  }

  function applyRawJson() {
    try {
      const parsed: Config = JSON.parse(rawJson);
      settingsStore.channels = parsed.channels ?? [];
      settingsStore.default_post_channels = parsed.default_post_channels ?? [];
      settingsStore.llm_service = parsed.llm_service;
      settingsStore.save_sent_posts = parsed.save_sent_posts ?? false;
      settingsStore.prompts = parsed.prompts;
      rawError = null;
      rawMode = false;
    } catch (e) {
      rawError = `Invalid JSON: ${e}`;
    }
  }

  async function handleSave() {
    if (rawMode) {
      try {
        const parsed: Config = JSON.parse(rawJson);
        settingsStore.channels = parsed.channels ?? [];
        settingsStore.default_post_channels = parsed.default_post_channels ?? [];
        settingsStore.llm_service = parsed.llm_service;
        settingsStore.save_sent_posts = parsed.save_sent_posts ?? false;
        settingsStore.prompts = parsed.prompts;
      } catch (e) {
        rawError = `Invalid JSON: ${e}`;
        return;
      }
    }
    saving = true;
    error = null;
    try {
      await settingsStore.save();
      onclose();
    } catch (e) {
      error = `Failed to save: ${e}`;
    } finally {
      saving = false;
    }
  }

  function addChannel() {
    settingsStore.channels = [
      ...settingsStore.channels,
      { name: "", id: "", should_translate: false },
    ];
  }

  function removeChannel(index: number) {
    const channels = settingsStore.channels.filter((_, i) => i !== index);
    const names = new Set(channels.map((c) => c.name));
    settingsStore.channels = channels;
    settingsStore.default_post_channels = settingsStore.default_post_channels.filter(
      (n) => names.has(n)
    );
  }

  function updateChannel(index: number, partial: Partial<Channel>) {
    settingsStore.channels = settingsStore.channels.map((c, i) =>
      i === index ? { ...c, ...partial } : c
    );
  }

  function toggleTranslation(index: number) {
    const ch = settingsStore.channels[index];
    const newVal = !ch.should_translate;
    updateChannel(index, {
      should_translate: newVal,
      translate: newVal ? ch.translate ?? { from: "en", to: "es" } : ch.translate,
    });
  }

  function toggleDefault(name: string) {
    const defaults = settingsStore.default_post_channels;
    settingsStore.default_post_channels = defaults.includes(name)
      ? defaults.filter((n) => n !== name)
      : [...defaults, name];
  }

  function updateLlm(partial: Partial<LlmService>) {
    const current = settingsStore.llm_service ?? { provider: "", model: undefined };
    settingsStore.llm_service = { ...current, ...partial };
  }
</script>

<div class="settings">
  <div class="settings-header">
    <span class="settings-title">Settings</span>
    <div class="header-actions">
      <button class="btn-mode" onclick={() => rawMode ? applyRawJson() : enterRawMode()}>
        {rawMode ? "Form view" : "Edit JSON"}
      </button>
      <button class="btn-close" title="Close" onclick={onclose}>
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <line x1="18" y1="6" x2="6" y2="18"></line>
          <line x1="6" y1="6" x2="18" y2="18"></line>
        </svg>
      </button>
    </div>
  </div>

  {#if rawMode}
    <div class="raw-editor">
      <textarea class="raw-textarea" bind:value={rawJson} spellcheck="false"></textarea>
      {#if rawError}
        <div class="settings-error">{rawError}</div>
      {/if}
    </div>
  {:else}
    <div class="settings-body">
      <!-- API Keys -->
      <section class="settings-section">
        <span class="section-label">API Keys</span>
        <div class="field-group">
          <label class="field">
            <span class="field-label">Buffer</span>
            <input type="password" class="field-input" bind:value={settingsStore.bufferApiKey} placeholder="Buffer API key" />
          </label>
          <label class="field">
            <span class="field-label">OpenAI</span>
            <input type="password" class="field-input" bind:value={settingsStore.openaiApiKey} placeholder="OpenAI API key" />
          </label>
          <label class="field">
            <span class="field-label">Gemini</span>
            <input type="password" class="field-input" bind:value={settingsStore.geminiApiKey} placeholder="Gemini API key" />
          </label>
          <label class="field">
            <span class="field-label">ImgBB</span>
            <input type="password" class="field-input" bind:value={settingsStore.imgbbApiKey} placeholder="ImgBB API key" />
          </label>
        </div>
      </section>

      <!-- LLM Service -->
      <section class="settings-section">
        <span class="section-label">LLM Service</span>
        <div class="field-row">
          <label class="field" style="flex:1">
            <span class="field-label">Provider</span>
            <select class="field-input" value={settingsStore.llm_service?.provider ?? ""} onchange={(e) => updateLlm({ provider: e.currentTarget.value })}>
              <option value="">None</option>
              <option value="openai">OpenAI</option>
              <option value="gemini">Gemini</option>
              <option value="anthropic">Anthropic</option>
              <option value="claude-cli">Claude CLI</option>
            </select>
          </label>
          <label class="field" style="flex:1">
            <span class="field-label">Model</span>
            <input class="field-input" value={settingsStore.llm_service?.model ?? ""} oninput={(e) => updateLlm({ model: e.currentTarget.value || undefined })} placeholder="e.g. gpt-4o-mini" />
          </label>
        </div>
      </section>

      <!-- Prompts -->
      <section class="settings-section">
        <span class="section-label">LLM Prompts</span>
        <div class="field-group">
          <label class="field">
            <span class="field-label">Translation Prompt</span>
            <span class="field-hint">Use {"{from}"}, {"{to}"}, {"{text}"} as placeholders</span>
            <textarea
              class="prompt-textarea"
              value={getTranslationPrompt()}
              oninput={(e) => updatePrompt("translation_prompt", e.currentTarget.value)}
              spellcheck="false"
            ></textarea>
          </label>
          <label class="field">
            <span class="field-label">Shrink Prompt</span>
            <span class="field-hint">Use {"{max_chars}"}, {"{text}"} as placeholders</span>
            <textarea
              class="prompt-textarea"
              value={getShrinkPrompt()}
              oninput={(e) => updatePrompt("shrink_prompt", e.currentTarget.value)}
              spellcheck="false"
            ></textarea>
          </label>
        </div>
      </section>

      <!-- Channels -->
      <section class="settings-section">
        <div class="section-header">
          <span class="section-label">Channels</span>
          <button class="btn-add" onclick={addChannel}>+ Add</button>
        </div>
        <div class="channels-list">
          {#each settingsStore.channels as channel, i (i)}
            <div class="channel-card">
              <div class="channel-row">
                <label class="field" style="flex:1">
                  <span class="field-label">Name</span>
                  <input class="field-input" value={channel.name} oninput={(e) => updateChannel(i, { name: e.currentTarget.value })} placeholder="e.g. x" />
                </label>
                <label class="field" style="flex:2">
                  <span class="field-label">Buffer ID</span>
                  <input class="field-input" value={channel.id} oninput={(e) => updateChannel(i, { id: e.currentTarget.value })} placeholder="Buffer channel ID" />
                </label>
                <button class="btn-remove" onclick={() => removeChannel(i)} title="Remove channel">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <line x1="18" y1="6" x2="6" y2="18"></line>
                    <line x1="6" y1="6" x2="18" y2="18"></line>
                  </svg>
                </button>
              </div>
              <div class="channel-row">
                <label class="toggle-label">
                  <div class="toggle-track" class:active={channel.should_translate}>
                    <input type="checkbox" checked={channel.should_translate} onchange={() => toggleTranslation(i)} />
                    <div class="toggle-thumb"></div>
                  </div>
                  <span>Translate</span>
                </label>
                {#if channel.should_translate}
                  <label class="field" style="flex:1">
                    <span class="field-label">From</span>
                    <input class="field-input" value={channel.translate?.from ?? ""} oninput={(e) => updateChannel(i, { translate: { from: e.currentTarget.value, to: channel.translate?.to ?? "" } })} placeholder="en" />
                  </label>
                  <label class="field" style="flex:1">
                    <span class="field-label">To</span>
                    <input class="field-input" value={channel.translate?.to ?? ""} oninput={(e) => updateChannel(i, { translate: { from: channel.translate?.from ?? "", to: e.currentTarget.value } })} placeholder="es" />
                  </label>
                {/if}
              </div>
            </div>
          {/each}
        </div>
      </section>

      <!-- Default Channels -->
      {#if settingsStore.channels.length > 0}
        <section class="settings-section">
          <span class="section-label">Default Post Channels</span>
          <div class="defaults-list">
            {#each settingsStore.channels as channel}
              {#if channel.name}
                <label class="default-toggle">
                  <input
                    type="checkbox"
                    checked={settingsStore.default_post_channels.includes(channel.name)}
                    onchange={() => toggleDefault(channel.name)}
                  />
                  <span>{channel.name}</span>
                </label>
              {/if}
            {/each}
          </div>
        </section>
      {/if}

      <!-- Sent Posts -->
      <section class="settings-section">
        <span class="section-label">Sent Posts</span>
        <div class="sent-posts-row">
          <label class="toggle-label">
            <div class="toggle-track" class:active={settingsStore.save_sent_posts}>
              <input type="checkbox" bind:checked={settingsStore.save_sent_posts} />
              <div class="toggle-thumb"></div>
            </div>
            <span>Save sent posts to disk</span>
          </label>
          <button class="btn-open-folder" onclick={() => openPostsFolder()}>
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path>
            </svg>
            Open saved posts
          </button>
        </div>
      </section>
    </div>
  {/if}

  {#if error}
    <div class="settings-error">{error}</div>
  {/if}

  <div class="settings-footer">
    <button class="btn-secondary" onclick={onclose}>Cancel</button>
    <button class="btn-primary" onclick={handleSave} disabled={saving}>
      {saving ? "Saving..." : "Save"}
    </button>
  </div>
</div>

<style>
  .settings {
    display: flex;
    flex-direction: column;
    height: 100%;
    max-width: 640px;
    margin: 0 auto;
  }

  .settings-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 24px;
    flex-shrink: 0;
  }

  .settings-title {
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

  .btn-mode {
    padding: 5px 12px;
    border-radius: 7px;
    border: 1px solid var(--border-strong);
    background: var(--surface-raised);
    color: var(--text-secondary);
    font-size: 11.5px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
    font-family: "DM Mono", monospace;
    letter-spacing: 0.3px;
  }

  .btn-mode:hover {
    border-color: var(--accent);
    color: var(--text);
    background: var(--accent-subtle);
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

  .settings-body {
    flex: 1;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 28px;
    padding-right: 4px;
  }

  .settings-body::-webkit-scrollbar {
    width: 4px;
  }

  .settings-body::-webkit-scrollbar-track {
    background: transparent;
  }

  .settings-body::-webkit-scrollbar-thumb {
    background: var(--border-strong);
    border-radius: 2px;
  }

  .raw-editor {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 8px;
    min-height: 0;
  }

  .raw-textarea {
    flex: 1;
    width: 100%;
    padding: 14px;
    border: 1px solid var(--border-strong);
    border-radius: var(--radius);
    background: var(--surface);
    color: var(--text);
    font-size: 12.5px;
    font-family: "DM Mono", monospace;
    line-height: 1.6;
    resize: none;
    tab-size: 2;
  }

  .raw-textarea:focus {
    outline: none;
    border-color: var(--accent);
  }

  .settings-section {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .field-group {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .field-row {
    display: flex;
    gap: 12px;
    align-items: flex-end;
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .field-label {
    font-size: 11px;
    font-weight: 500;
    color: var(--text-tertiary);
    letter-spacing: 0.3px;
  }

  .field-input {
    padding: 8px 12px;
    border: 1px solid var(--border-strong);
    border-radius: 8px;
    background: var(--surface);
    color: var(--text);
    font-size: 13px;
    font-family: "DM Sans", system-ui, sans-serif;
    transition: border-color 0.15s ease;
    width: 100%;
  }

  .field-input:focus {
    outline: none;
    border-color: var(--accent);
  }

  .field-input::placeholder {
    color: var(--text-tertiary);
  }

  select.field-input {
    appearance: none;
    background-image: url("data:image/svg+xml,%3Csvg width='12' height='8' viewBox='0 0 12 8' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M1 1.5L6 6.5L11 1.5' stroke='%234a4f62' stroke-width='1.5' stroke-linecap='round' stroke-linejoin='round'/%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 12px center;
    padding-right: 32px;
  }

  .field-hint {
    font-size: 10.5px;
    color: var(--text-tertiary);
    font-family: "DM Mono", monospace;
    letter-spacing: 0.2px;
  }

  .prompt-textarea {
    width: 100%;
    min-height: 100px;
    padding: 10px 12px;
    border: 1px solid var(--border-strong);
    border-radius: 8px;
    background: var(--surface);
    color: var(--text);
    font-size: 12.5px;
    font-family: "DM Mono", monospace;
    line-height: 1.5;
    resize: vertical;
    transition: border-color 0.15s ease;
  }

  .prompt-textarea:focus {
    outline: none;
    border-color: var(--accent);
  }

  .channels-list {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .channel-card {
    border: 1px solid var(--border);
    border-radius: var(--radius);
    padding: 12px;
    display: flex;
    flex-direction: column;
    gap: 10px;
    background: var(--surface);
    transition: border-color 0.15s ease;
  }

  .channel-card:hover {
    border-color: var(--border-strong);
  }

  .channel-row {
    display: flex;
    gap: 10px;
    align-items: flex-end;
  }

  .btn-add {
    padding: 4px 10px;
    border-radius: 6px;
    border: 1px solid var(--border-strong);
    background: transparent;
    color: var(--text-tertiary);
    font-size: 11px;
    font-weight: 500;
    cursor: pointer;
    font-family: "DM Sans", system-ui, sans-serif;
    transition: all 0.15s ease;
  }

  .btn-add:hover {
    border-color: var(--accent);
    color: var(--accent);
    background: var(--accent-subtle);
  }

  .btn-remove {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    border: 1px solid var(--border);
    border-radius: 8px;
    background: transparent;
    color: var(--text-tertiary);
    cursor: pointer;
    transition: all 0.15s ease;
    flex-shrink: 0;
    margin-bottom: 1px;
  }

  .btn-remove:hover {
    border-color: var(--danger-border);
    color: var(--danger);
    background: var(--danger-bg);
  }

  .toggle-label {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 12.5px;
    color: var(--text-secondary);
    cursor: pointer;
    user-select: none;
    font-weight: 500;
    white-space: nowrap;
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

  .defaults-list {
    display: flex;
    flex-wrap: wrap;
    gap: 12px;
  }

  .default-toggle {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 13px;
    color: var(--text-secondary);
    cursor: pointer;
    user-select: none;
  }

  .default-toggle input[type="checkbox"] {
    appearance: none;
    width: 16px;
    height: 16px;
    border: 1px solid var(--border-strong);
    border-radius: 4px;
    background: var(--surface);
    cursor: pointer;
    transition: all 0.15s ease;
    position: relative;
  }

  .default-toggle input[type="checkbox"]:checked {
    background: var(--accent);
    border-color: var(--accent);
  }

  .default-toggle input[type="checkbox"]:checked::after {
    content: "";
    position: absolute;
    top: 2px;
    left: 5px;
    width: 4px;
    height: 8px;
    border: solid white;
    border-width: 0 2px 2px 0;
    transform: rotate(45deg);
  }

  .settings-error {
    padding: 10px 14px;
    border-radius: 8px;
    background: var(--danger-bg);
    border: 1px solid var(--danger-border);
    color: var(--danger);
    font-size: 12.5px;
    margin-top: 12px;
    flex-shrink: 0;
  }

  .settings-footer {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    padding-top: 20px;
    border-top: 1px solid var(--border);
    margin-top: 20px;
    flex-shrink: 0;
  }

  .btn-primary {
    padding: 8px 20px;
    border-radius: 8px;
    border: none;
    background: var(--accent);
    color: white;
    font-size: 13px;
    font-weight: 600;
    cursor: pointer;
    font-family: "DM Sans", system-ui, sans-serif;
    transition: all 0.15s ease;
  }

  .btn-primary:hover:not(:disabled) {
    background: var(--accent-hover);
  }

  .btn-primary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .sent-posts-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
  }

  .btn-open-folder {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    border-radius: 7px;
    border: 1px solid var(--border-strong);
    background: var(--surface-raised);
    color: var(--text-secondary);
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
    font-family: "DM Sans", system-ui, sans-serif;
    transition: all 0.15s ease;
    white-space: nowrap;
  }

  .btn-open-folder:hover {
    border-color: var(--accent);
    color: var(--text);
    background: var(--accent-subtle);
  }
</style>
