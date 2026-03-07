<script lang="ts">
  import { postStore } from "../lib/stores/post.svelte";

  let charCount = $derived(postStore.text.length);
</script>

<div class="post-editor">
  <label for="post-text" class="section-label">Compose</label>
  <div class="editor-wrap">
    <textarea
      id="post-text"
      bind:value={postStore.text}
      placeholder="What's on your mind?"
      rows="6"
    ></textarea>
    <div class="bottom-bar">
      <div></div>
      <span class="char-count">{charCount}</span>
    </div>
  </div>
</div>

<style>
  .post-editor {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .editor-wrap {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    overflow: hidden;
    transition: border-color 0.2s ease, box-shadow 0.2s ease;
  }

  .editor-wrap:focus-within {
    border-color: var(--accent);
    box-shadow: 0 0 0 2px var(--accent-glow);
  }

  textarea {
    width: 100%;
    padding: 14px 16px;
    padding-bottom: 0;
    border: none;
    background: transparent;
    color: var(--text);
    font-size: 15px;
    font-family: "DM Sans", system-ui, sans-serif;
    line-height: 1.6;
    resize: none;
    min-height: 120px;
    letter-spacing: 0.1px;
  }

  textarea:focus {
    outline: none;
  }

  textarea::placeholder {
    color: var(--text-tertiary);
    font-style: italic;
  }

  .bottom-bar {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    padding: 8px 14px;
  }

  .char-count {
    font-size: 12px;
    font-family: "DM Mono", monospace;
    color: var(--text-tertiary);
    letter-spacing: 0.5px;
  }
</style>
